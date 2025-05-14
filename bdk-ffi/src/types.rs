use crate::bitcoin::{Address, Amount, BlockHash, OutPoint, Script, Transaction, TxOut, Txid};
use crate::error::{CreateTxError, RequestBuilderError};

use bdk_core::bitcoin::absolute::LockTime as BdkLockTime;
use bdk_core::spk_client::SyncItem;
use bdk_core::BlockId as BdkBlockId;

use bdk_wallet::bitcoin::Transaction as BdkTransaction;
use bdk_wallet::chain::spk_client::FullScanRequest as BdkFullScanRequest;
use bdk_wallet::chain::spk_client::FullScanRequestBuilder as BdkFullScanRequestBuilder;
use bdk_wallet::chain::spk_client::SyncRequest as BdkSyncRequest;
use bdk_wallet::chain::spk_client::SyncRequestBuilder as BdkSyncRequestBuilder;
use bdk_wallet::chain::tx_graph::CanonicalTx as BdkCanonicalTx;
use bdk_wallet::chain::{
    ChainPosition as BdkChainPosition, ConfirmationBlockTime as BdkConfirmationBlockTime,
};
use bdk_wallet::descriptor::policy::{
    Condition as BdkCondition, PkOrF as BdkPkOrF, Policy as BdkPolicy,
    Satisfaction as BdkSatisfaction, SatisfiableItem as BdkSatisfiableItem,
};
use bdk_wallet::signer::{SignOptions as BdkSignOptions, TapLeavesOptions};
use bdk_wallet::AddressInfo as BdkAddressInfo;
use bdk_wallet::Balance as BdkBalance;
use bdk_wallet::LocalOutput as BdkLocalOutput;
use bdk_wallet::Update as BdkUpdate;

use std::collections::HashMap;
use std::convert::TryFrom;
use std::sync::{Arc, Mutex};

use crate::{impl_from_core_type, impl_into_core_type};
use bdk_esplora::esplora_client::api::Tx as BdkTx;
use bdk_esplora::esplora_client::api::TxStatus as BdkTxStatus;

type KeychainKind = bdk_wallet::KeychainKind;

/// Types of keychains.
#[uniffi::remote(Enum)]
pub enum KeychainKind {
    /// External keychain, used for deriving recipient addresses.
    External = 0,
    /// Internal keychain, used for deriving change addresses.
    Internal = 1,
}

/// Represents the observed position of some chain data.
#[derive(Debug, uniffi::Enum)]
pub enum ChainPosition {
    /// The chain data is confirmed as it is anchored in the best chain by `A`.
    Confirmed {
        confirmation_block_time: ConfirmationBlockTime,
        /// A child transaction that has been confirmed. Due to incomplete information,
        /// it is only known that this transaction is confirmed at a chain height less than
        /// or equal to this child TXID.
        transitively: Option<Arc<Txid>>,
    },
    /// The transaction was last seen in the mempool at this timestamp.
    Unconfirmed { timestamp: Option<u64> },
}

impl From<BdkChainPosition<BdkConfirmationBlockTime>> for ChainPosition {
    fn from(chain_position: BdkChainPosition<BdkConfirmationBlockTime>) -> Self {
        match chain_position {
            BdkChainPosition::Confirmed {
                anchor,
                transitively,
            } => {
                let block_id = BlockId {
                    height: anchor.block_id.height,
                    hash: Arc::new(BlockHash(anchor.block_id.hash)),
                };
                ChainPosition::Confirmed {
                    confirmation_block_time: ConfirmationBlockTime {
                        block_id,
                        confirmation_time: anchor.confirmation_time,
                    },
                    transitively: transitively.map(|t| Arc::new(Txid(t))),
                }
            }
            BdkChainPosition::Unconfirmed { last_seen } => ChainPosition::Unconfirmed {
                timestamp: last_seen,
            },
        }
    }
}

/// Represents the confirmation block and time of a transaction.
#[derive(Debug, uniffi::Record)]
pub struct ConfirmationBlockTime {
    /// The anchor block.
    pub block_id: BlockId,
    /// The confirmation time of the transaction being anchored.
    pub confirmation_time: u64,
}

/// A reference to a block in the canonical chain.
#[derive(Debug, uniffi::Record)]
pub struct BlockId {
    /// The height of the block.
    pub height: u32,
    /// The hash of the block.
    pub hash: Arc<BlockHash>,
}

impl From<BdkBlockId> for BlockId {
    fn from(block_id: BdkBlockId) -> Self {
        BlockId {
            height: block_id.height,
            hash: Arc::new(BlockHash(block_id.hash)),
        }
    }
}

/// A transaction that is deemed to be part of the canonical history.
#[derive(uniffi::Record)]
pub struct CanonicalTx {
    /// The transaction.
    pub transaction: Arc<Transaction>,
    /// How the transaction is observed in the canonical chain (confirmed or unconfirmed).
    pub chain_position: ChainPosition,
}

impl From<BdkCanonicalTx<'_, Arc<BdkTransaction>, BdkConfirmationBlockTime>> for CanonicalTx {
    fn from(tx: BdkCanonicalTx<'_, Arc<BdkTransaction>, BdkConfirmationBlockTime>) -> Self {
        CanonicalTx {
            transaction: Arc::new(Transaction::from(tx.tx_node.tx.as_ref().clone())),
            chain_position: tx.chain_position.into(),
        }
    }
}

/// A bitcoin script and associated amount.
#[derive(uniffi::Record)]
pub struct ScriptAmount {
    /// The underlying script.
    pub script: Arc<Script>,
    /// The amount owned by the script.
    pub amount: Arc<Amount>,
}

/// A derived address and the index it was found at.
#[derive(uniffi::Record)]
pub struct AddressInfo {
    /// Child index of this address
    pub index: u32,
    /// The address
    pub address: Arc<Address>,
    /// Type of keychain
    pub keychain: KeychainKind,
}

impl From<BdkAddressInfo> for AddressInfo {
    fn from(address_info: BdkAddressInfo) -> Self {
        AddressInfo {
            index: address_info.index,
            address: Arc::new(address_info.address.into()),
            keychain: address_info.keychain,
        }
    }
}

/// Balance, differentiated into various categories.
#[derive(uniffi::Record)]
pub struct Balance {
    /// All coinbase outputs not yet matured
    pub immature: Arc<Amount>,
    /// Unconfirmed UTXOs generated by a wallet tx
    pub trusted_pending: Arc<Amount>,
    /// Unconfirmed UTXOs received from an external wallet
    pub untrusted_pending: Arc<Amount>,
    /// Confirmed and immediately spendable balance
    pub confirmed: Arc<Amount>,
    /// Get sum of trusted_pending and confirmed coins.
    ///
    /// This is the balance you can spend right now that shouldn't get cancelled via another party
    /// double spending it.
    pub trusted_spendable: Arc<Amount>,
    /// Get the whole balance visible to the wallet.
    pub total: Arc<Amount>,
}

impl From<BdkBalance> for Balance {
    fn from(bdk_balance: BdkBalance) -> Self {
        Balance {
            immature: Arc::new(bdk_balance.immature.into()),
            trusted_pending: Arc::new(bdk_balance.trusted_pending.into()),
            untrusted_pending: Arc::new(bdk_balance.untrusted_pending.into()),
            confirmed: Arc::new(bdk_balance.confirmed.into()),
            trusted_spendable: Arc::new(bdk_balance.trusted_spendable().into()),
            total: Arc::new(bdk_balance.total().into()),
        }
    }
}

/// An unspent output owned by a [`Wallet`].
#[derive(uniffi::Record)]
pub struct LocalOutput {
    /// Reference to a transaction output
    pub outpoint: OutPoint,
    /// Transaction output
    pub txout: TxOut,
    /// Type of keychain
    pub keychain: KeychainKind,
    /// Whether this UTXO is spent or not
    pub is_spent: bool,
    /// The derivation index for the script pubkey in the wallet
    pub derivation_index: u32,
    /// The position of the output in the blockchain.
    pub chain_position: ChainPosition,
}

impl From<BdkLocalOutput> for LocalOutput {
    fn from(local_utxo: BdkLocalOutput) -> Self {
        LocalOutput {
            outpoint: OutPoint {
                txid: Arc::new(Txid(local_utxo.outpoint.txid)),
                vout: local_utxo.outpoint.vout,
            },
            txout: TxOut {
                value: local_utxo.txout.value.to_sat(),
                script_pubkey: Arc::new(Script(local_utxo.txout.script_pubkey)),
            },
            keychain: local_utxo.keychain,
            is_spent: local_utxo.is_spent,
            derivation_index: local_utxo.derivation_index,
            chain_position: local_utxo.chain_position.into(),
        }
    }
}

// Callback for the FullScanRequest
pub trait FullScanScriptInspector: Sync + Send {
    fn inspect(&self, keychain: KeychainKind, index: u32, script: Arc<Script>);
}

// Callback for the SyncRequest
pub trait SyncScriptInspector: Sync + Send {
    fn inspect(&self, script: Arc<Script>, total: u64);
}

pub struct FullScanRequestBuilder(
    pub(crate) Mutex<Option<BdkFullScanRequestBuilder<KeychainKind>>>,
);

pub struct SyncRequestBuilder(pub(crate) Mutex<Option<BdkSyncRequestBuilder<(KeychainKind, u32)>>>);

pub struct FullScanRequest(pub(crate) Mutex<Option<BdkFullScanRequest<KeychainKind>>>);

pub struct SyncRequest(pub(crate) Mutex<Option<BdkSyncRequest<(KeychainKind, u32)>>>);

impl SyncRequestBuilder {
    pub fn inspect_spks(
        &self,
        inspector: Arc<dyn SyncScriptInspector>,
    ) -> Result<Arc<Self>, RequestBuilderError> {
        let guard = self
            .0
            .lock()
            .unwrap()
            .take()
            .ok_or(RequestBuilderError::RequestAlreadyConsumed)?;
        let sync_request_builder = guard.inspect({
            move |script, progress| {
                if let SyncItem::Spk(_, spk) = script {
                    inspector.inspect(Arc::new(Script(spk.to_owned())), progress.total() as u64)
                }
            }
        });
        Ok(Arc::new(SyncRequestBuilder(Mutex::new(Some(
            sync_request_builder,
        )))))
    }

    pub fn build(&self) -> Result<Arc<SyncRequest>, RequestBuilderError> {
        let guard = self
            .0
            .lock()
            .unwrap()
            .take()
            .ok_or(RequestBuilderError::RequestAlreadyConsumed)?;
        Ok(Arc::new(SyncRequest(Mutex::new(Some(guard.build())))))
    }
}

impl FullScanRequestBuilder {
    pub fn inspect_spks_for_all_keychains(
        &self,
        inspector: Arc<dyn FullScanScriptInspector>,
    ) -> Result<Arc<Self>, RequestBuilderError> {
        let guard = self
            .0
            .lock()
            .unwrap()
            .take()
            .ok_or(RequestBuilderError::RequestAlreadyConsumed)?;
        let full_scan_request_builder = guard.inspect(move |keychain, index, script| {
            inspector.inspect(keychain, index, Arc::new(Script(script.to_owned())))
        });
        Ok(Arc::new(FullScanRequestBuilder(Mutex::new(Some(
            full_scan_request_builder,
        )))))
    }

    pub fn build(&self) -> Result<Arc<FullScanRequest>, RequestBuilderError> {
        let guard = self
            .0
            .lock()
            .unwrap()
            .take()
            .ok_or(RequestBuilderError::RequestAlreadyConsumed)?;
        Ok(Arc::new(FullScanRequest(Mutex::new(Some(guard.build())))))
    }
}

/// An update for a wallet containing chain, descriptor index, and transaction data.
#[derive(uniffi::Object)]
pub struct Update(pub(crate) BdkUpdate);

/// The total value sent and received.
#[derive(uniffi::Record)]
pub struct SentAndReceivedValues {
    /// Amount sent in the transaction.
    pub sent: Arc<Amount>,
    /// The amount received in the transaction, possibly as a change output(s).
    pub received: Arc<Amount>,
}

/// The keychain kind and the index in that keychain.
#[derive(uniffi::Record)]
pub struct KeychainAndIndex {
    /// Type of keychains.
    pub keychain: KeychainKind,
    /// The index in the keychain.
    pub index: u32,
}

/// Descriptor spending policy
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Policy(BdkPolicy);

impl_from_core_type!(BdkPolicy, Policy);
impl_into_core_type!(Policy, BdkPolicy);

impl Policy {
    pub fn id(&self) -> String {
        self.0.id.clone()
    }

    pub fn as_string(&self) -> String {
        bdk_wallet::serde_json::to_string(&self.0).unwrap()
    }

    pub fn requires_path(&self) -> bool {
        self.0.requires_path()
    }

    pub fn item(&self) -> SatisfiableItem {
        self.0.item.clone().into()
    }

    pub fn satisfaction(&self) -> Satisfaction {
        self.0.satisfaction.clone().into()
    }

    pub fn contribution(&self) -> Satisfaction {
        self.0.contribution.clone().into()
    }
}

#[derive(Debug, Clone)]
pub enum SatisfiableItem {
    EcdsaSignature {
        key: PkOrF,
    },
    SchnorrSignature {
        key: PkOrF,
    },
    Sha256Preimage {
        hash: String,
    },
    Hash256Preimage {
        hash: String,
    },
    Ripemd160Preimage {
        hash: String,
    },
    Hash160Preimage {
        hash: String,
    },
    AbsoluteTimelock {
        value: LockTime,
    },
    RelativeTimelock {
        value: u32,
    },
    Multisig {
        keys: Vec<PkOrF>,
        threshold: u64,
    },
    Thresh {
        items: Vec<Arc<Policy>>,
        threshold: u64,
    },
}

impl From<BdkSatisfiableItem> for SatisfiableItem {
    fn from(value: BdkSatisfiableItem) -> Self {
        match value {
            BdkSatisfiableItem::EcdsaSignature(pk_or_f) => SatisfiableItem::EcdsaSignature {
                key: pk_or_f.into(),
            },
            BdkSatisfiableItem::SchnorrSignature(pk_or_f) => SatisfiableItem::SchnorrSignature {
                key: pk_or_f.into(),
            },
            BdkSatisfiableItem::Sha256Preimage { hash } => SatisfiableItem::Sha256Preimage {
                hash: hash.to_string(),
            },
            BdkSatisfiableItem::Hash256Preimage { hash } => SatisfiableItem::Hash256Preimage {
                hash: hash.to_string(),
            },
            BdkSatisfiableItem::Ripemd160Preimage { hash } => SatisfiableItem::Ripemd160Preimage {
                hash: hash.to_string(),
            },
            BdkSatisfiableItem::Hash160Preimage { hash } => SatisfiableItem::Hash160Preimage {
                hash: hash.to_string(),
            },
            BdkSatisfiableItem::AbsoluteTimelock { value } => SatisfiableItem::AbsoluteTimelock {
                value: value.into(),
            },
            BdkSatisfiableItem::RelativeTimelock { value } => SatisfiableItem::RelativeTimelock {
                value: value.to_consensus_u32(),
            },
            BdkSatisfiableItem::Multisig { keys, threshold } => SatisfiableItem::Multisig {
                keys: keys.iter().map(|e| e.to_owned().into()).collect(),
                threshold: threshold as u64,
            },
            BdkSatisfiableItem::Thresh { items, threshold } => SatisfiableItem::Thresh {
                items: items
                    .iter()
                    .map(|e| Arc::new(e.to_owned().into()))
                    .collect(),
                threshold: threshold as u64,
            },
        }
    }
}

#[derive(Debug, Clone)]
pub enum PkOrF {
    Pubkey { value: String },
    XOnlyPubkey { value: String },
    Fingerprint { value: String },
}

impl From<BdkPkOrF> for PkOrF {
    fn from(value: BdkPkOrF) -> Self {
        match value {
            BdkPkOrF::Pubkey(public_key) => PkOrF::Pubkey {
                value: public_key.to_string(),
            },
            BdkPkOrF::XOnlyPubkey(xonly_public_key) => PkOrF::XOnlyPubkey {
                value: xonly_public_key.to_string(),
            },
            BdkPkOrF::Fingerprint(fingerprint) => PkOrF::Fingerprint {
                value: fingerprint.to_string(),
            },
        }
    }
}

#[derive(Debug, Clone)]
pub enum LockTime {
    Blocks { height: u32 },
    Seconds { consensus_time: u32 },
}

impl From<BdkLockTime> for LockTime {
    fn from(value: BdkLockTime) -> Self {
        match value {
            BdkLockTime::Blocks(height) => LockTime::Blocks {
                height: height.to_consensus_u32(),
            },
            BdkLockTime::Seconds(time) => LockTime::Seconds {
                consensus_time: time.to_consensus_u32(),
            },
        }
    }
}

impl TryFrom<&LockTime> for BdkLockTime {
    type Error = CreateTxError;

    fn try_from(value: &LockTime) -> Result<Self, CreateTxError> {
        match value {
            LockTime::Blocks { height } => BdkLockTime::from_height(*height)
                .map_err(|_| CreateTxError::LockTimeConversionError),
            LockTime::Seconds { consensus_time } => BdkLockTime::from_time(*consensus_time)
                .map_err(|_| CreateTxError::LockTimeConversionError),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Satisfaction {
    Partial {
        n: u64,
        m: u64,
        items: Vec<u64>,
        sorted: Option<bool>,
        conditions: HashMap<u32, Vec<Condition>>,
    },
    PartialComplete {
        n: u64,
        m: u64,
        items: Vec<u64>,
        sorted: Option<bool>,
        conditions: HashMap<Vec<u32>, Vec<Condition>>,
    },
    Complete {
        condition: Condition,
    },
    None {
        msg: String,
    },
}

impl From<BdkSatisfaction> for Satisfaction {
    fn from(value: BdkSatisfaction) -> Self {
        match value {
            BdkSatisfaction::Partial {
                n,
                m,
                items,
                sorted,
                conditions,
            } => Satisfaction::Partial {
                n: n as u64,
                m: m as u64,
                items: items.iter().map(|e| e.to_owned() as u64).collect(),
                sorted,
                conditions: conditions
                    .into_iter()
                    .map(|(index, conditions)| {
                        (
                            index as u32,
                            conditions.into_iter().map(|e| e.into()).collect(),
                        )
                    })
                    .collect(),
            },
            BdkSatisfaction::PartialComplete {
                n,
                m,
                items,
                sorted,
                conditions,
            } => Satisfaction::PartialComplete {
                n: n as u64,
                m: m as u64,
                items: items.iter().map(|e| e.to_owned() as u64).collect(),
                sorted,
                conditions: conditions
                    .into_iter()
                    .map(|(index, conditions)| {
                        (
                            index.iter().map(|e| e.to_owned() as u32).collect(),
                            conditions.into_iter().map(|e| e.into()).collect(),
                        )
                    })
                    .collect(),
            },
            BdkSatisfaction::Complete { condition } => Satisfaction::Complete {
                condition: condition.into(),
            },
            BdkSatisfaction::None => Satisfaction::None {
                msg: "Cannot satisfy or contribute to the policy item".to_string(),
            },
        }
    }
}

/// An extra condition that must be satisfied but that is out of control of the user
#[derive(Debug, Clone, uniffi::Record)]
pub struct Condition {
    /// Optional CheckSequenceVerify condition
    pub csv: Option<u32>,
    /// Optional timelock condition
    pub timelock: Option<LockTime>,
}

impl From<BdkCondition> for Condition {
    fn from(value: BdkCondition) -> Self {
        Condition {
            csv: value.csv.map(|e| e.to_consensus_u32()),
            timelock: value.timelock.map(|e| e.into()),
        }
    }
}

// This is a wrapper type around the bdk type [SignOptions](https://docs.rs/bdk_wallet/1.0.0/bdk_wallet/signer/struct.SignOptions.html)
// because we did not want to expose the complexity behind the `TapLeavesOptions` type. When
// transforming from a SignOption to a BdkSignOptions, we simply use the default values for
// TapLeavesOptions.
/// Options for a software signer.
///
/// Adjust the behavior of our software signers and the way a transaction is finalized.
#[derive(uniffi::Record)]
pub struct SignOptions {
    /// Whether the signer should trust the `witness_utxo`, if the `non_witness_utxo` hasn't been
    /// provided
    ///
    /// Defaults to `false` to mitigate the "SegWit bug" which could trick the wallet into
    /// paying a fee larger than expected.
    ///
    /// Some wallets, especially if relatively old, might not provide the `non_witness_utxo` for
    /// SegWit transactions in the PSBT they generate: in those cases setting this to `true`
    /// should correctly produce a signature, at the expense of an increased trust in the creator
    /// of the PSBT.
    ///
    /// For more details see: <https://blog.trezor.io/details-of-firmware-updates-for-trezor-one-version-1-9-1-and-trezor-model-t-version-2-3-1-1eba8f60f2dd>
    pub trust_witness_utxo: bool,
    /// Whether the wallet should assume a specific height has been reached when trying to finalize
    /// a transaction
    ///
    /// The wallet will only "use" a timelock to satisfy the spending policy of an input if the
    /// timelock height has already been reached. This option allows overriding the "current height" to let the
    /// wallet use timelocks in the future to spend a coin.
    pub assume_height: Option<u32>,
    /// Whether the signer should use the `sighash_type` set in the PSBT when signing, no matter
    /// what its value is
    ///
    /// Defaults to `false` which will only allow signing using `SIGHASH_ALL`.
    pub allow_all_sighashes: bool,
    /// Whether to try finalizing the PSBT after the inputs are signed.
    ///
    /// Defaults to `true` which will try finalizing PSBT after inputs are signed.
    pub try_finalize: bool,
    /// Whether we should try to sign a taproot transaction with the taproot internal key
    /// or not. This option is ignored if we're signing a non-taproot PSBT.
    ///
    /// Defaults to `true`, i.e., we always try to sign with the taproot internal key.
    pub sign_with_tap_internal_key: bool,
    /// Whether we should grind ECDSA signature to ensure signing with low r
    /// or not.
    /// Defaults to `true`, i.e., we always grind ECDSA signature to sign with low r.
    pub allow_grinding: bool,
}

impl From<SignOptions> for BdkSignOptions {
    fn from(options: SignOptions) -> BdkSignOptions {
        BdkSignOptions {
            trust_witness_utxo: options.trust_witness_utxo,
            assume_height: options.assume_height,
            allow_all_sighashes: options.allow_all_sighashes,
            try_finalize: options.try_finalize,
            tap_leaves_options: TapLeavesOptions::default(),
            sign_with_tap_internal_key: options.sign_with_tap_internal_key,
            allow_grinding: options.allow_grinding,
        }
    }
}

/// Transaction confirmation metadata.
#[derive(uniffi::Record, Debug)]
pub struct TxStatus {
    /// Is the transaction in a block.
    pub confirmed: bool,
    /// Height of the block this transaction was included.
    pub block_height: Option<u32>,
    /// Hash of the block.
    pub block_hash: Option<Arc<BlockHash>>,
    /// The time shown in the block, not necessarily the same time as when the block was found.
    pub block_time: Option<u64>,
}

impl From<BdkTxStatus> for TxStatus {
    fn from(status: BdkTxStatus) -> Self {
        TxStatus {
            confirmed: status.confirmed,
            block_height: status.block_height,
            block_hash: status.block_hash.map(|h| Arc::new(BlockHash(h))),
            block_time: status.block_time,
        }
    }
}

/// Bitcoin transaction metadata.
#[derive(Debug, uniffi::Record)]
pub struct Tx {
    /// The transaction identifier.
    pub txid: Arc<Txid>,
    /// The transaction version, of which 0, 1, 2 are standard.
    pub version: i32,
    /// The block height or time restriction on the transaction.
    pub locktime: u32,
    /// The size of the transaction in bytes.
    pub size: u64,
    /// The weight units of this transaction.
    pub weight: u64,
    /// The fee of this transaction in satoshis.
    pub fee: u64,
    /// Confirmation status and data.
    pub status: TxStatus,
}

impl From<BdkTx> for Tx {
    fn from(tx: BdkTx) -> Self {
        Self {
            txid: Arc::new(Txid(tx.txid)),
            version: tx.version,
            locktime: tx.locktime,
            size: tx.size as u64,
            weight: tx.weight,
            fee: tx.fee,
            status: tx.status.into(),
        }
    }
}

/// This type replaces the Rust tuple `(tx, last_seen)` used in the Wallet::apply_unconfirmed_txs` method,
/// where `last_seen` is the timestamp of when the transaction `tx` was last seen in the mempool.
#[derive(uniffi::Record)]
pub struct UnconfirmedTx {
    pub tx: Arc<Transaction>,
    pub last_seen: u64,
}
