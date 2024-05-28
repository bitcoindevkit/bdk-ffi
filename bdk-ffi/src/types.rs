use crate::bitcoin::Amount;
use crate::bitcoin::{Address, OutPoint, Script, Transaction, TxOut};
use crate::InspectError;

use bdk_wallet::bitcoin::ScriptBuf as BdkScriptBuf;
use bdk_wallet::bitcoin::Transaction as BdkTransaction;
use bdk_wallet::chain::spk_client::FullScanRequest as BdkFullScanRequest;
use bdk_wallet::chain::spk_client::SyncRequest as BdkSyncRequest;
use bdk_wallet::chain::tx_graph::CanonicalTx as BdkCanonicalTx;
use bdk_wallet::chain::{ChainPosition as BdkChainPosition, ConfirmationTimeHeightAnchor};
use bdk_wallet::wallet::AddressInfo as BdkAddressInfo;
use bdk_wallet::wallet::Balance as BdkBalance;
use bdk_wallet::KeychainKind;
use bdk_wallet::LocalOutput as BdkLocalOutput;

use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChainPosition {
    Confirmed { height: u32, timestamp: u64 },
    Unconfirmed { timestamp: u64 },
}

pub struct CanonicalTx {
    pub transaction: Arc<Transaction>,
    pub chain_position: ChainPosition,
}

impl From<BdkCanonicalTx<'_, Arc<BdkTransaction>, ConfirmationTimeHeightAnchor>> for CanonicalTx {
    fn from(tx: BdkCanonicalTx<'_, Arc<BdkTransaction>, ConfirmationTimeHeightAnchor>) -> Self {
        let chain_position = match tx.chain_position {
            BdkChainPosition::Confirmed(anchor) => ChainPosition::Confirmed {
                height: anchor.confirmation_height,
                timestamp: anchor.confirmation_time,
            },
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
                txid: local_utxo.outpoint.txid.to_string(),
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

pub struct FullScanRequest(pub(crate) Mutex<Option<BdkFullScanRequest<KeychainKind>>>);

pub struct SyncRequest(pub(crate) Mutex<Option<BdkSyncRequest>>);

impl SyncRequest {
    pub fn inspect_spks(
        &self,
        inspector: Box<dyn SyncScriptInspector>,
    ) -> Result<Arc<Self>, InspectError> {
        let mut guard = self.0.lock().unwrap();
        if let Some(sync_request) = guard.take() {
            let total = sync_request.spks.len() as u64;
            let sync_request = sync_request.inspect_spks(move |spk| {
                inspector.inspect(Arc::new(BdkScriptBuf::from(spk).into()), total)
            });
            Ok(Arc::new(SyncRequest(Mutex::new(Some(sync_request)))))
        } else {
            Err(InspectError::RequestAlreadyConsumed)
        }
    }
}

impl FullScanRequest {
    pub fn inspect_spks_for_all_keychains(
        &self,
        inspector: Box<dyn FullScanScriptInspector>,
    ) -> Result<Arc<Self>, InspectError> {
        let mut guard = self.0.lock().unwrap();
        if let Some(full_scan_request) = guard.take() {
            let inspector = Arc::new(inspector);
            let full_scan_request =
                full_scan_request.inspect_spks_for_all_keychains(move |k, spk_i, script| {
                    inspector.inspect(k, spk_i, Arc::new(BdkScriptBuf::from(script).into()))
                });
            Ok(Arc::new(FullScanRequest(Mutex::new(Some(
                full_scan_request,
            )))))
        } else {
            Err(InspectError::RequestAlreadyConsumed)
        }
    }
}
