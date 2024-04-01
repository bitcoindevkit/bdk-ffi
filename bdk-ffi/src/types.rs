use crate::bitcoin::{Address, OutPoint, Script, TxOut};

use bdk::wallet::AddressIndex as BdkAddressIndex;
use bdk::wallet::AddressInfo as BdkAddressInfo;
use bdk::wallet::Balance as BdkBalance;
use bdk::KeychainKind;
use bdk::LocalOutput as BdkLocalOutput;

use bdk::bitcoin::FeeRate as BdkFeeRate;

use crate::error::FeeRateError;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct FeeRate(pub BdkFeeRate);

impl FeeRate {
    pub fn from_sat_per_vb(sat_per_vb: u64) -> Result<Self, FeeRateError> {
        let fee_rate: Option<BdkFeeRate> = BdkFeeRate::from_sat_per_vb(sat_per_vb);
        match fee_rate {
            Some(fee_rate) => Ok(FeeRate(fee_rate)),
            None => Err(FeeRateError::ArithmeticOverflow),
        }
    }

    pub fn from_sat_per_kwu(sat_per_kwu: u64) -> Self {
        FeeRate(BdkFeeRate::from_sat_per_kwu(sat_per_kwu))
    }

    pub fn to_sat_per_vb_ceil(&self) -> u64 {
        self.0.to_sat_per_vb_ceil()
    }

    pub fn to_sat_per_vb_floor(&self) -> u64 {
        self.0.to_sat_per_vb_floor()
    }

    pub fn to_sat_per_kwu(&self) -> u64 {
        self.0.to_sat_per_kwu()
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
                value: local_utxo.txout.value,
                script_pubkey: Arc::new(Script(local_utxo.txout.script_pubkey)),
            },
            keychain: local_utxo.keychain,
            is_spent: local_utxo.is_spent,
        }
    }
}
