use crate::bitcoin::{Address, Transaction, TxOut};
use crate::error::RequestBuilderError;

use bitcoin_ffi::Amount;
use bitcoin_ffi::OutPoint;
use bitcoin_ffi::Script;

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
use bdk_wallet::AddressInfo as BdkAddressInfo;
use bdk_wallet::Balance as BdkBalance;
use bdk_wallet::KeychainKind;
use bdk_wallet::LocalOutput as BdkLocalOutput;
use bdk_wallet::Update as BdkUpdate;

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
