use crate::bitcoin::{Address, OutPoint, Script, Transaction, TxOut};

use bdk::chain::tx_graph::CanonicalTx as BdkCanonicalTx;
use bdk::chain::{ChainPosition as BdkChainPosition, ConfirmationTimeHeightAnchor};
use bdk::wallet::AddressIndex as BdkAddressIndex;
use bdk::wallet::AddressInfo as BdkAddressInfo;
use bdk::wallet::Balance as BdkBalance;
use bdk::KeychainKind;
use bdk::LocalOutput as BdkLocalOutput;

use std::sync::Arc;

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
    pub amount: u64,
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

pub enum AddressIndex {
    New,
    LastUnused,
    Peek { index: u32 },
}

impl From<AddressIndex> for BdkAddressIndex {
    fn from(address_index: AddressIndex) -> Self {
        match address_index {
            AddressIndex::New => BdkAddressIndex::New,
            AddressIndex::LastUnused => BdkAddressIndex::LastUnused,
            AddressIndex::Peek { index } => BdkAddressIndex::Peek(index),
        }
    }
}

impl From<BdkAddressIndex> for AddressIndex {
    fn from(address_index: BdkAddressIndex) -> Self {
        match address_index {
            BdkAddressIndex::New => AddressIndex::New,
            BdkAddressIndex::LastUnused => AddressIndex::LastUnused,
            _ => panic!("Mmmm not working"),
        }
    }
}

// TODO 9: Peek is not correctly implemented
impl From<&AddressIndex> for BdkAddressIndex {
    fn from(address_index: &AddressIndex) -> Self {
        match address_index {
            AddressIndex::New => BdkAddressIndex::New,
            AddressIndex::LastUnused => BdkAddressIndex::LastUnused,
            AddressIndex::Peek { index } => BdkAddressIndex::Peek(*index),
        }
    }
}

impl From<&BdkAddressIndex> for AddressIndex {
    fn from(address_index: &BdkAddressIndex) -> Self {
        match address_index {
            BdkAddressIndex::New => AddressIndex::New,
            BdkAddressIndex::LastUnused => AddressIndex::LastUnused,
            _ => panic!("Mmmm not working"),
        }
    }
}

pub struct Balance {
    pub immature: u64,
    pub trusted_pending: u64,
    pub untrusted_pending: u64,
    pub confirmed: u64,
    pub trusted_spendable: u64,
    pub total: u64,
}

impl From<BdkBalance> for Balance {
    fn from(bdk_balance: BdkBalance) -> Self {
        Balance {
            immature: bdk_balance.immature,
            trusted_pending: bdk_balance.trusted_pending,
            untrusted_pending: bdk_balance.untrusted_pending,
            confirmed: bdk_balance.confirmed,
            trusted_spendable: bdk_balance.trusted_spendable(),
            total: bdk_balance.total(),
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
