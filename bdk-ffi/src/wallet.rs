use crate::bitcoin::{Amount, FeeRate, OutPoint, Psbt, Script, Transaction};
use crate::descriptor::Descriptor;
use crate::error::{
    CalculateFeeError, CannotConnectError, CreateWithPersistError, DescriptorError,
    LoadWithPersistError, SignerError, SqliteError, TxidParseError,
};
use crate::store::Connection;
use crate::types::{
    AddressInfo, Balance, BlockId, CanonicalTx, FullScanRequestBuilder, KeychainAndIndex,
    LocalOutput, Policy, SentAndReceivedValues, SignOptions, SyncRequestBuilder, UnconfirmedTx,
    Update,
};

use bdk_wallet::bitcoin::{Network, Txid};
use bdk_wallet::rusqlite::Connection as BdkConnection;
use bdk_wallet::signer::SignOptions as BdkSignOptions;
use bdk_wallet::{KeychainKind, PersistedWallet, Wallet as BdkWallet};

use std::borrow::BorrowMut;
use std::str::FromStr;
use std::sync::{Arc, Mutex, MutexGuard};

/// A Bitcoin wallet.
///
/// The Wallet acts as a way of coherently interfacing with output descriptors and related transactions. Its main components are:
/// 1. output descriptors from which it can derive addresses.
/// 2. signers that can contribute signatures to addresses instantiated from the descriptors.
///
/// The user is responsible for loading and writing wallet changes which are represented as
/// ChangeSets (see take_staged). Also see individual functions and example for instructions on when
/// Wallet state needs to be persisted.
///
/// The Wallet descriptor (external) and change descriptor (internal) must not derive the same
/// script pubkeys. See KeychainTxOutIndex::insert_descriptor() for more details.
#[derive(uniffi::Object)]
pub struct Wallet {
    inner_mutex: Mutex<PersistedWallet<BdkConnection>>,
}

#[uniffi::export]
impl Wallet {
    /// Build a new Wallet.
    ///
    /// If you have previously created a wallet, use load instead.
    #[uniffi::constructor]
    pub fn new(
        descriptor: Arc<Descriptor>,
        change_descriptor: Arc<Descriptor>,
        network: Network,
        connection: Arc<Connection>,
    ) -> Result<Self, CreateWithPersistError> {
        let descriptor = descriptor.to_string_with_secret();
        let change_descriptor = change_descriptor.to_string_with_secret();
        let mut binding = connection.get_store();
        let db: &mut BdkConnection = binding.borrow_mut();

        let wallet: PersistedWallet<BdkConnection> =
            BdkWallet::create(descriptor, change_descriptor)
                .network(network)
                .create_wallet(db)?;

        Ok(Wallet {
            inner_mutex: Mutex::new(wallet),
        })
    }

    /// Build Wallet by loading from persistence.
    //
    // Note that the descriptor secret keys are not persisted to the db.
    #[uniffi::constructor]
    pub fn load(
        descriptor: Arc<Descriptor>,
        change_descriptor: Arc<Descriptor>,
        connection: Arc<Connection>,
    ) -> Result<Wallet, LoadWithPersistError> {
        let descriptor = descriptor.to_string_with_secret();
        let change_descriptor = change_descriptor.to_string_with_secret();
        let mut binding = connection.get_store();
        let db: &mut BdkConnection = binding.borrow_mut();

        let wallet: PersistedWallet<BdkConnection> = BdkWallet::load()
            .descriptor(KeychainKind::External, Some(descriptor))
            .descriptor(KeychainKind::Internal, Some(change_descriptor))
            .extract_keys()
            .load_wallet(db)?
            .ok_or(LoadWithPersistError::CouldNotLoad)?;

        Ok(Wallet {
            inner_mutex: Mutex::new(wallet),
        })
    }

    /// Finds how the wallet derived the script pubkey `spk`.
    ///
    /// Will only return `Some(_)` if the wallet has given out the spk.
    pub fn derivation_of_spk(&self, spk: Arc<Script>) -> Option<KeychainAndIndex> {
        self.get_wallet()
            .derivation_of_spk(spk.0.clone())
            .map(|(k, i)| KeychainAndIndex {
                keychain: k,
                index: i,
            })
    }

    /// Informs the wallet that you no longer intend to broadcast a tx that was built from it.
    ///
    /// This frees up the change address used when creating the tx for use in future transactions.
    pub fn cancel_tx(&self, tx: &Transaction) {
        self.get_wallet().cancel_tx(&tx.into())
    }

    /// Returns the utxo owned by this wallet corresponding to `outpoint` if it exists in the
    /// wallet's database.
    pub fn get_utxo(&self, op: OutPoint) -> Option<LocalOutput> {
        self.get_wallet()
            .get_utxo(op.into())
            .map(|local_output| local_output.into())
    }

    /// Attempt to reveal the next address of the given `keychain`.
    ///
    /// This will increment the keychain's derivation index. If the keychain's descriptor doesn't
    /// contain a wildcard or every address is already revealed up to the maximum derivation
    /// index defined in [BIP32](https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki),
    /// then the last revealed address will be returned.
    pub fn reveal_next_address(&self, keychain: KeychainKind) -> AddressInfo {
        self.get_wallet().reveal_next_address(keychain).into()
    }

    /// Peek an address of the given `keychain` at `index` without revealing it.
    ///
    /// For non-wildcard descriptors this returns the same address at every provided index.
    ///
    /// # Panics
    ///
    /// This panics when the caller requests for an address of derivation index greater than the
    /// [BIP32](https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki) max index.
    pub fn peek_address(&self, keychain: KeychainKind, index: u32) -> AddressInfo {
        self.get_wallet().peek_address(keychain, index).into()
    }

    /// The index of the next address that you would get if you were to ask the wallet for a new
    /// address.
    pub fn next_derivation_index(&self, keychain: KeychainKind) -> u32 {
        self.get_wallet().next_derivation_index(keychain)
    }

    /// Get the next unused address for the given `keychain`, i.e. the address with the lowest
    /// derivation index that hasn't been used in a transaction.
    ///
    /// This will attempt to reveal a new address if all previously revealed addresses have
    /// been used, in which case the returned address will be the same as calling [`Wallet::reveal_next_address`].
    ///
    /// **WARNING**: To avoid address reuse you must persist the changes resulting from one or more
    /// calls to this method before closing the wallet. See [`Wallet::reveal_next_address`].
    pub fn next_unused_address(&self, keychain: KeychainKind) -> AddressInfo {
        self.get_wallet().next_unused_address(keychain).into()
    }

    /// Marks an address used of the given `keychain` at `index`.
    ///
    /// Returns whether the given index was present and then removed from the unused set.
    pub fn mark_used(&self, keychain: KeychainKind, index: u32) -> bool {
        self.get_wallet().mark_used(keychain, index)
    }

    /// Reveal addresses up to and including the target `index` and return an iterator
    /// of newly revealed addresses.
    ///
    /// If the target `index` is unreachable, we make a best effort to reveal up to the last
    /// possible index. If all addresses up to the given `index` are already revealed, then
    /// no new addresses are returned.
    ///
    /// **WARNING**: To avoid address reuse you must persist the changes resulting from one or more
    /// calls to this method before closing the wallet. See [`Wallet::reveal_next_address`].
    pub fn reveal_addresses_to(&self, keychain: KeychainKind, index: u32) -> Vec<AddressInfo> {
        self.get_wallet()
            .reveal_addresses_to(keychain, index)
            .map(|address_info| address_info.into())
            .collect()
    }

    /// List addresses that are revealed but unused.
    ///
    /// Note if the returned iterator is empty you can reveal more addresses
    /// by using [`reveal_next_address`](Self::reveal_next_address) or
    /// [`reveal_addresses_to`](Self::reveal_addresses_to).
    pub fn list_unused_addresses(&self, keychain: KeychainKind) -> Vec<AddressInfo> {
        self.get_wallet()
            .list_unused_addresses(keychain)
            .map(|address_info| address_info.into())
            .collect()
    }

    /// Applies an update to the wallet and stages the changes (but does not persist them).
    ///
    /// Usually you create an `update` by interacting with some blockchain data source and inserting
    /// transactions related to your wallet into it.
    ///
    /// After applying updates you should persist the staged wallet changes. For an example of how
    /// to persist staged wallet changes see [`Wallet::reveal_next_address`].
    pub fn apply_update(&self, update: Arc<Update>) -> Result<(), CannotConnectError> {
        self.get_wallet()
            .apply_update(update.0.clone())
            .map_err(CannotConnectError::from)
    }

    /// Apply relevant unconfirmed transactions to the wallet.
    /// Transactions that are not relevant are filtered out.
    pub fn apply_unconfirmed_txs(&self, unconfirmed_txs: Vec<UnconfirmedTx>) {
        self.get_wallet().apply_unconfirmed_txs(
            unconfirmed_txs
                .into_iter()
                .map(|utx| (Arc::new(utx.tx.as_ref().into()), utx.last_seen)),
        )
    }

    /// The derivation index of this wallet. It will return `None` if it has not derived any addresses.
    /// Otherwise, it will return the index of the highest address it has derived.
    pub fn derivation_index(&self, keychain: KeychainKind) -> Option<u32> {
        self.get_wallet().derivation_index(keychain)
    }

    /// Return the checksum of the public descriptor associated to `keychain`.
    ///
    /// Internally calls [`Self::public_descriptor`] to fetch the right descriptor.
    pub fn descriptor_checksum(&self, keychain: KeychainKind) -> String {
        self.get_wallet().descriptor_checksum(keychain)
    }

    /// Return the spending policies for the wallet’s descriptor.
    pub fn policies(&self, keychain: KeychainKind) -> Result<Option<Arc<Policy>>, DescriptorError> {
        self.get_wallet()
            .policies(keychain)
            .map_err(DescriptorError::from)
            .map(|e| e.map(|p| Arc::new(p.into())))
    }

    /// Get the Bitcoin network the wallet is using.
    pub fn network(&self) -> Network {
        self.get_wallet().network()
    }

    /// Return the balance, separated into available, trusted-pending, untrusted-pending and
    /// immature values.
    pub fn balance(&self) -> Balance {
        let bdk_balance = self.get_wallet().balance();
        Balance::from(bdk_balance)
    }

    /// Return whether or not a `script` is part of this wallet (either internal or external).
    pub fn is_mine(&self, script: Arc<Script>) -> bool {
        self.get_wallet().is_mine(script.0.clone())
    }

    /// Sign a transaction with all the wallet's signers, in the order specified by every signer's
    /// [`SignerOrdering`]. This function returns the `Result` type with an encapsulated `bool` that
    /// has the value true if the PSBT was finalized, or false otherwise.
    ///
    /// The [`SignOptions`] can be used to tweak the behavior of the software signers, and the way
    /// the transaction is finalized at the end. Note that it can't be guaranteed that *every*
    /// signers will follow the options, but the "software signers" (WIF keys and `xprv`) defined
    /// in this library will.
    #[uniffi::method(default(sign_options = None))]
    pub fn sign(
        &self,
        psbt: Arc<Psbt>,
        sign_options: Option<SignOptions>,
    ) -> Result<bool, SignerError> {
        let mut psbt = psbt.0.lock().unwrap();
        let bdk_sign_options: BdkSignOptions = match sign_options {
            Some(sign_options) => BdkSignOptions::from(sign_options),
            None => BdkSignOptions::default(),
        };

        self.get_wallet()
            .sign(&mut psbt, bdk_sign_options)
            .map_err(SignerError::from)
    }

    /// Finalize a PSBT, i.e., for each input determine if sufficient data is available to pass
    /// validation and construct the respective `scriptSig` or `scriptWitness`. Please refer to
    /// [BIP174](https://github.com/bitcoin/bips/blob/master/bip-0174.mediawiki#Input_Finalizer),
    /// and [BIP371](https://github.com/bitcoin/bips/blob/master/bip-0371.mediawiki)
    /// for further information.
    ///
    /// Returns `true` if the PSBT could be finalized, and `false` otherwise.
    ///
    /// The [`SignOptions`] can be used to tweak the behavior of the finalizer.
    #[uniffi::method(default(sign_options = None))]
    pub fn finalize_psbt(
        &self,
        psbt: Arc<Psbt>,
        sign_options: Option<SignOptions>,
    ) -> Result<bool, SignerError> {
        let mut psbt = psbt.0.lock().unwrap();
        let bdk_sign_options: BdkSignOptions = match sign_options {
            Some(sign_options) => BdkSignOptions::from(sign_options),
            None => BdkSignOptions::default(),
        };

        self.get_wallet()
            .finalize_psbt(&mut psbt, bdk_sign_options)
            .map_err(SignerError::from)
    }

    /// Compute the `tx`'s sent and received [`Amount`]s.
    ///
    /// This method returns a tuple `(sent, received)`. Sent is the sum of the txin amounts
    /// that spend from previous txouts tracked by this wallet. Received is the summation
    /// of this tx's outputs that send to script pubkeys tracked by this wallet.
    pub fn sent_and_received(&self, tx: &Transaction) -> SentAndReceivedValues {
        let (sent, received) = self.get_wallet().sent_and_received(&tx.into());
        SentAndReceivedValues {
            sent: Arc::new(sent.into()),
            received: Arc::new(received.into()),
        }
    }

    /// Iterate over the transactions in the wallet.
    pub fn transactions(&self) -> Vec<CanonicalTx> {
        self.get_wallet()
            .transactions_sort_by(|tx1, tx2| tx2.chain_position.cmp(&tx1.chain_position))
            .into_iter()
            .map(|tx| tx.into())
            .collect()
    }

    /// Get a single transaction from the wallet as a [`WalletTx`] (if the transaction exists).
    ///
    /// `WalletTx` contains the full transaction alongside meta-data such as:
    /// * Blocks that the transaction is [`Anchor`]ed in. These may or may not be blocks that exist
    ///   in the best chain.
    /// * The [`ChainPosition`] of the transaction in the best chain - whether the transaction is
    ///   confirmed or unconfirmed. If the transaction is confirmed, the anchor which proves the
    ///   confirmation is provided. If the transaction is unconfirmed, the unix timestamp of when
    ///   the transaction was last seen in the mempool is provided.
    pub fn get_tx(&self, txid: String) -> Result<Option<CanonicalTx>, TxidParseError> {
        let txid =
            Txid::from_str(txid.as_str()).map_err(|_| TxidParseError::InvalidTxid { txid })?;
        Ok(self.get_wallet().get_tx(txid).map(|tx| tx.into()))
    }

    /// Calculates the fee of a given transaction. Returns [`Amount::ZERO`] if `tx` is a coinbase transaction.
    ///
    /// To calculate the fee for a [`Transaction`] with inputs not owned by this wallet you must
    /// manually insert the TxOut(s) into the tx graph using the [`insert_txout`] function.
    ///
    /// Note `tx` does not have to be in the graph for this to work.
    pub fn calculate_fee(&self, tx: &Transaction) -> Result<Arc<Amount>, CalculateFeeError> {
        self.get_wallet()
            .calculate_fee(&tx.into())
            .map(Amount::from)
            .map(Arc::new)
            .map_err(|e| e.into())
    }

    /// Calculate the [`FeeRate`] for a given transaction.
    ///
    /// To calculate the fee rate for a [`Transaction`] with inputs not owned by this wallet you must
    /// manually insert the TxOut(s) into the tx graph using the [`insert_txout`] function.
    ///
    /// Note `tx` does not have to be in the graph for this to work.
    pub fn calculate_fee_rate(&self, tx: &Transaction) -> Result<Arc<FeeRate>, CalculateFeeError> {
        self.get_wallet()
            .calculate_fee_rate(&tx.into())
            .map(|bdk_fee_rate| Arc::new(FeeRate(bdk_fee_rate)))
            .map_err(|e| e.into())
    }

    /// Return the list of unspent outputs of this wallet.
    pub fn list_unspent(&self) -> Vec<LocalOutput> {
        self.get_wallet().list_unspent().map(|o| o.into()).collect()
    }

    /// List all relevant outputs (includes both spent and unspent, confirmed and unconfirmed).
    ///
    /// To list only unspent outputs (UTXOs), use [`Wallet::list_unspent`] instead.
    pub fn list_output(&self) -> Vec<LocalOutput> {
        self.get_wallet().list_output().map(|o| o.into()).collect()
    }

    /// Create a [`FullScanRequest] for this wallet.
    ///
    /// This is the first step when performing a spk-based wallet full scan, the returned
    /// [`FullScanRequest] collects iterators for the wallet's keychain script pub keys needed to
    /// start a blockchain full scan with a spk based blockchain client.
    ///
    /// This operation is generally only used when importing or restoring a previously used wallet
    /// in which the list of used scripts is not known.
    pub fn start_full_scan(&self) -> Arc<FullScanRequestBuilder> {
        let builder = self.get_wallet().start_full_scan();
        Arc::new(FullScanRequestBuilder(Mutex::new(Some(builder))))
    }

    /// Create a partial [`SyncRequest`] for this wallet for all revealed spks.
    ///
    /// This is the first step when performing a spk-based wallet partial sync, the returned
    /// [`SyncRequest`] collects all revealed script pubkeys from the wallet keychain needed to
    /// start a blockchain sync with a spk based blockchain client.
    pub fn start_sync_with_revealed_spks(&self) -> Arc<SyncRequestBuilder> {
        let builder = self.get_wallet().start_sync_with_revealed_spks();
        Arc::new(SyncRequestBuilder(Mutex::new(Some(builder))))
    }

    /// Persist staged changes of wallet into persister.
    ///
    /// Returns whether any new changes were persisted.
    ///
    /// If the persister errors, the staged changes will not be cleared.
    pub fn persist(&self, connection: Arc<Connection>) -> Result<bool, SqliteError> {
        let mut binding = connection.get_store();
        let db: &mut BdkConnection = binding.borrow_mut();
        self.get_wallet()
            .persist(db)
            .map_err(|e| SqliteError::Sqlite {
                rusqlite_error: e.to_string(),
            })
    }

    /// Returns the latest checkpoint.
    pub fn latest_checkpoint(&self) -> BlockId {
        self.get_wallet().latest_checkpoint().block_id().into()
    }
}

impl Wallet {
    pub(crate) fn get_wallet(&self) -> MutexGuard<PersistedWallet<BdkConnection>> {
        self.inner_mutex.lock().expect("wallet")
    }
}
