use crate::bitcoin::{Address, OutPoint, Script, Transaction, TxOut};

use bdk::chain::spk_client::FullScanRequest as BdkFullScanRequest;
use bdk::chain::spk_client::SyncRequest as BdkSyncRequest;
use bdk::chain::tx_graph::CanonicalTx as BdkCanonicalTx;
use bdk::chain::{ChainPosition as BdkChainPosition, ConfirmationTimeHeightAnchor};
use bdk::wallet::AddressInfo as BdkAddressInfo;
use bdk::wallet::Balance as BdkBalance;
use bdk::KeychainKind;
use bdk::LocalOutput as BdkLocalOutput;

use std::sync::{Arc, Mutex};

use crate::bitcoin::Amount;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChainPosition {
    Confirmed { height: u32, timestamp: u64 },
    Unconfirmed { timestamp: u64 },
}

pub struct CanonicalTx {
    pub transaction: Arc<Transaction>,
    pub chain_position: ChainPosition,
}

impl From<BdkCanonicalTx<'_, Arc<bdk::bitcoin::Transaction>, ConfirmationTimeHeightAnchor>>
    for CanonicalTx
{
    fn from(
        tx: BdkCanonicalTx<'_, Arc<bdk::bitcoin::Transaction>, ConfirmationTimeHeightAnchor>,
    ) -> Self {
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

pub struct FullScanRequest(pub(crate) Mutex<Option<BdkFullScanRequest<KeychainKind>>>);

pub struct SyncRequest(pub(crate) Mutex<Option<BdkSyncRequest>>);
