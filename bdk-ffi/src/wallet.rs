use bdk::bitcoin::Network;
use bdk::database::any::AnyDatabase;
use bdk::database::{AnyDatabaseConfig, ConfigurableDatabase};
use bdk::{Error as BdkError, SignOptions, SyncOptions as BdkSyncOptions, Wallet as BdkWallet};
use std::ops::Deref;
use std::sync::{Arc, Mutex, MutexGuard};

use crate::psbt::PartiallySignedTransaction;
use crate::{
    AddressIndex, AddressInfo, Balance, Blockchain, DatabaseConfig, Descriptor, LocalUtxo,
    NetworkLocalUtxo, Progress, ProgressHolder, TransactionDetails,
};

#[derive(Debug)]
pub(crate) struct Wallet {
    pub(crate) wallet_mutex: Mutex<BdkWallet<AnyDatabase>>,
}

/// A Bitcoin wallet.
/// The Wallet acts as a way of coherently interfacing with output descriptors and related transactions. Its main components are:
///     1. Output descriptors from which it can derive addresses.
///     2. A Database where it tracks transactions and utxos related to the descriptors.
///     3. Signers that can contribute signatures to addresses instantiated from the descriptors.
impl Wallet {
    pub(crate) fn new(
        descriptor: Arc<Descriptor>,
        change_descriptor: Option<Arc<Descriptor>>,
        network: Network,
        database_config: DatabaseConfig,
    ) -> Result<Self, BdkError> {
        let any_database_config = match database_config {
            DatabaseConfig::Memory => AnyDatabaseConfig::Memory(()),
            DatabaseConfig::Sled { config } => AnyDatabaseConfig::Sled(config),
            DatabaseConfig::Sqlite { config } => AnyDatabaseConfig::Sqlite(config),
        };
        let database = AnyDatabase::from_config(&any_database_config)?;
        let descriptor: String = descriptor.as_string_private();
        let change_descriptor: Option<String> = change_descriptor.map(|d| d.as_string_private());

        let wallet_mutex = Mutex::new(BdkWallet::new(
            &descriptor,
            change_descriptor.as_ref(),
            network,
            database,
        )?);
        Ok(Wallet { wallet_mutex })
    }

    pub(crate) fn get_wallet(&self) -> MutexGuard<BdkWallet<AnyDatabase>> {
        self.wallet_mutex.lock().expect("wallet")
    }

    /// Get the Bitcoin network the wallet is using.
    pub(crate) fn network(&self) -> Network {
        self.get_wallet().network()
    }

    /// Sync the internal database with the blockchain.
    pub(crate) fn sync(
        &self,
        blockchain: &Blockchain,
        progress: Option<Box<dyn Progress>>,
    ) -> Result<(), BdkError> {
        let bdk_sync_opts = BdkSyncOptions {
            progress: progress.map(|p| {
                Box::new(ProgressHolder { progress: p })
                    as Box<(dyn bdk::blockchain::Progress + 'static)>
            }),
        };

        let blockchain = blockchain.get_blockchain();
        self.get_wallet().sync(blockchain.deref(), bdk_sync_opts)
    }

    /// Return a derived address using the external descriptor, see AddressIndex for available address index selection
    /// strategies. If none of the keys in the descriptor are derivable (i.e. the descriptor does not end with a * character)
    /// then the same address will always be returned for any AddressIndex.
    pub(crate) fn get_address(&self, address_index: AddressIndex) -> Result<AddressInfo, BdkError> {
        self.get_wallet()
            .get_address(address_index.into())
            .map(AddressInfo::from)
    }

    /// Return the balance, meaning the sum of this wallet’s unspent outputs’ values. Note that this method only operates
    /// on the internal database, which first needs to be Wallet.sync manually.
    pub(crate) fn get_balance(&self) -> Result<Balance, BdkError> {
        self.get_wallet().get_balance().map(|b| b.into())
    }

    /// Sign a transaction with all the wallet’s signers.
    pub(crate) fn sign(&self, psbt: &PartiallySignedTransaction) -> Result<bool, BdkError> {
        let mut psbt = psbt.internal.lock().unwrap();
        self.get_wallet().sign(&mut psbt, SignOptions::default())
    }

    /// Return the list of transactions made and received by the wallet. Note that this method only operate on the internal database, which first needs to be [Wallet.sync] manually.
    pub(crate) fn list_transactions(&self) -> Result<Vec<TransactionDetails>, BdkError> {
        let transaction_details = self.get_wallet().list_transactions(true)?;
        Ok(transaction_details
            .iter()
            .map(TransactionDetails::from)
            .collect())
    }

    /// Return the list of unspent outputs of this wallet. Note that this method only operates on the internal database,
    /// which first needs to be Wallet.sync manually.
    pub(crate) fn list_unspent(&self) -> Result<Vec<LocalUtxo>, BdkError> {
        let unspents = self.get_wallet().list_unspent()?;
        Ok(unspents
            .iter()
            .map(|u| LocalUtxo::from_utxo(u, self.network()))
            .collect())
    }
}
