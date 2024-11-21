use crate::bitcoin::{Address, Transaction, TxOut};
use crate::error::{CreateTxError, RequestBuilderError};

use bitcoin_ffi::Amount;
use bitcoin_ffi::OutPoint;
use bitcoin_ffi::Script;

use bdk_core::bitcoin::absolute::LockTime as BdkLockTime;
use bdk_core::spk_client::SyncItem;

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
use bdk_wallet::AddressInfo as BdkAddressInfo;
use bdk_wallet::Balance as BdkBalance;
use bdk_wallet::KeychainKind;
use bdk_wallet::LocalOutput as BdkLocalOutput;
use bdk_wallet::Update as BdkUpdate;

use std::collections::HashMap;
use std::convert::TryFrom;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub enum ChainPosition {
    Confirmed {
        confirmation_block_time: ConfirmationBlockTime,
    },
    Unconfirmed {
        timestamp: u64,
    },
}

#[derive(Debug)]
pub struct ConfirmationBlockTime {
    pub block_id: BlockId,
    pub confirmation_time: u64,
}

#[derive(Debug)]
pub struct BlockId {
    pub height: u32,
    pub hash: String,
}

pub struct CanonicalTx {
    pub transaction: Arc<Transaction>,
    pub chain_position: ChainPosition,
}

impl From<BdkCanonicalTx<'_, Arc<BdkTransaction>, BdkConfirmationBlockTime>> for CanonicalTx {
    fn from(tx: BdkCanonicalTx<'_, Arc<BdkTransaction>, BdkConfirmationBlockTime>) -> Self {
        let chain_position = match tx.chain_position {
            BdkChainPosition::Confirmed(anchor) => {
                let block_id = BlockId {
                    height: anchor.block_id.height,
                    hash: anchor.block_id.hash.to_string(),
                };
                ChainPosition::Confirmed {
                    confirmation_block_time: ConfirmationBlockTime {
                        block_id,
                        confirmation_time: anchor.confirmation_time,
                    },
                }
            }
            BdkChainPosition::Unconfirmed(timestamp) => ChainPosition::Unconfirmed { timestamp },
        };

        CanonicalTx {
            transaction: Arc::new(Transaction::from(tx.tx_node.tx.as_ref().clone())),
            chain_position,
        }
    }
}

pub struct ScriptAmount {
    pub script: Arc<Script>,
    pub amount: Arc<Amount>,
}

pub struct AddressInfo {
    pub index: u32,
    pub address: Arc<Address>,
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

pub struct Balance {
    pub immature: Arc<Amount>,
    pub trusted_pending: Arc<Amount>,
    pub untrusted_pending: Arc<Amount>,
    pub confirmed: Arc<Amount>,
    pub trusted_spendable: Arc<Amount>,
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

pub struct LocalOutput {
    pub outpoint: OutPoint,
    pub txout: TxOut,
    pub keychain: KeychainKind,
    pub is_spent: bool,
}

impl From<BdkLocalOutput> for LocalOutput {
    fn from(local_utxo: BdkLocalOutput) -> Self {
        LocalOutput {
            outpoint: OutPoint {
                txid: local_utxo.outpoint.txid,
                vout: local_utxo.outpoint.vout,
            },
            txout: TxOut {
                value: local_utxo.txout.value.to_sat(),
                script_pubkey: Arc::new(Script(local_utxo.txout.script_pubkey)),
            },
            keychain: local_utxo.keychain,
            is_spent: local_utxo.is_spent,
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

pub struct Update(pub(crate) BdkUpdate);

pub struct SentAndReceivedValues {
    pub sent: Arc<Amount>,
    pub received: Arc<Amount>,
}

pub struct KeychainAndIndex {
    pub keychain: KeychainKind,
    pub index: u32,
}

/// Descriptor spending policy
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Policy(BdkPolicy);
impl From<BdkPolicy> for Policy {
    fn from(value: BdkPolicy) -> Self {
        Policy(value)
    }
}
impl From<Policy> for BdkPolicy {
    fn from(value: Policy) -> Self {
        value.0
    }
}
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

#[derive(Debug, Clone)]
pub struct Condition {
    pub csv: Option<u32>,
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
