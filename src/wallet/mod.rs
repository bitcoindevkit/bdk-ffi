use std::convert::TryFrom;

use ::safer_ffi::prelude::*;
use bdk::bitcoin::network::constants::Network::Testnet;
use bdk::blockchain::{log_progress, AnyBlockchain, AnyBlockchainConfig, ConfigurableBlockchain};
use bdk::database::{AnyDatabase, AnyDatabaseConfig, ConfigurableDatabase};
use bdk::wallet::AddressIndex::New;
use bdk::{Error, Wallet};
use safer_ffi::boxed::Box;
use safer_ffi::char_p::{char_p_boxed, char_p_ref};

use blockchain::BlockchainConfig;
use database::DatabaseConfig;

use crate::error::get_name;
use crate::types::{FfiResult, FfiResultVec};

mod blockchain;
mod database;

// create a new wallet

#[derive_ReprC]
#[ReprC::opaque]
pub struct OpaqueWallet {
    raw: Wallet<AnyBlockchain, AnyDatabase>,
}

#[ffi_export]
fn new_wallet_result(
    descriptor: char_p_ref,
    change_descriptor: Option<char_p_ref>,
    blockchain_config: &BlockchainConfig,
    database_config: &DatabaseConfig,
) -> FfiResult<OpaqueWallet> {
    let descriptor = descriptor.to_string();
    let change_descriptor = change_descriptor.map(|s| s.to_string());
    let bc_config = &blockchain_config.raw;
    let db_config = &database_config.raw;
    let wallet_result = new_wallet(descriptor, change_descriptor, bc_config, db_config);

    match wallet_result {
        Ok(w) => FfiResult {
            ok: Some(Box::new(OpaqueWallet { raw: w })),
            err: None,
        },
        Err(e) => FfiResult {
            ok: None,
            err: Some(Box::new(char_p_boxed::try_from(get_name(&e)).unwrap())),
        },
    }
}

fn new_wallet(
    descriptor: String,
    change_descriptor: Option<String>,
    blockchain_config: &AnyBlockchainConfig,
    database_config: &AnyDatabaseConfig,
) -> Result<Wallet<AnyBlockchain, AnyDatabase>, Error> {
    let network = Testnet;

    let client = AnyBlockchain::from_config(blockchain_config)?;
    let database = AnyDatabase::from_config(database_config)?;

    let descriptor: &str = descriptor.as_str();
    let change_descriptor: Option<&str> = change_descriptor.as_deref();

    Wallet::new(descriptor, change_descriptor, network, database, client)
}

#[ffi_export]
fn free_wallet_result(wallet_result: FfiResult<OpaqueWallet>) {
    drop(wallet_result);
}

// wallet operations

#[ffi_export]
fn sync_wallet(opaque_wallet: &OpaqueWallet) -> FfiResult<()> {
    let void_result = opaque_wallet.raw.sync(log_progress(), Some(100));
    match void_result {
        Ok(v) => FfiResult {
            ok: Some(Box::new(v)),
            err: None,
        },
        Err(e) => FfiResult {
            ok: None,
            err: Some(Box::new(char_p_boxed::try_from(get_name(&e)).unwrap())),
        },
    }
}

#[ffi_export]
fn new_address(opaque_wallet: &OpaqueWallet) -> FfiResult<char_p_boxed> {
    let new_address = opaque_wallet.raw.get_address(New);
    let string_result = new_address.map(|a| a.to_string());
    match string_result {
        Ok(a) => FfiResult {
            ok: Some(Box::new(char_p_boxed::try_from(a).unwrap())),
            err: None,
        },
        Err(e) => FfiResult {
            ok: None,
            err: Some(Box::new(char_p_boxed::try_from(get_name(&e)).unwrap())),
        },
    }
}

#[ffi_export]
fn list_unspent(opaque_wallet: &OpaqueWallet) -> FfiResultVec<LocalUtxo> {
    let unspent_result = opaque_wallet.raw.list_unspent();

    match unspent_result {
        Ok(v) => FfiResultVec {
            ok: {
                let ve: Vec<LocalUtxo> = v.iter().map(|lu| LocalUtxo::from(lu)).collect();
                repr_c::Vec::from(ve)
            },
            err: None,
        },
        Err(e) => FfiResultVec {
            ok: repr_c::Vec::EMPTY,
            err: Some(Box::new(char_p_boxed::try_from(get_name(&e)).unwrap())),
        },
    }
}

#[ffi_export]
fn free_unspent_result(unspent_result: FfiResultVec<LocalUtxo>) {
    drop(unspent_result)
}

// Non-opaque returned values

#[derive_ReprC]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OutPoint {
    /// The referenced transaction's txid, as hex string
    pub txid: char_p_boxed,
    /// The index of the referenced output in its transaction's vout
    pub vout: u32,
}

impl From<&bdk::bitcoin::OutPoint> for OutPoint {
    fn from(op: &bdk::bitcoin::OutPoint) -> Self {
        OutPoint {
            txid: char_p_boxed::try_from(op.txid.to_string()).unwrap(),
            vout: op.vout,
        }
    }
}

#[derive_ReprC]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct TxOut {
    /// The value of the output, in satoshis
    pub value: u64,
    /// The script which must satisfy for the output to be spent, as hex string
    pub script_pubkey: char_p_boxed,
}

impl From<&bdk::bitcoin::TxOut> for TxOut {
    fn from(to: &bdk::bitcoin::TxOut) -> Self {
        TxOut {
            value: to.value,
            script_pubkey: char_p_boxed::try_from(to.script_pubkey.to_string()).unwrap(),
        }
    }
}

#[derive_ReprC]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct LocalUtxo {
    /// Reference to a transaction output
    pub outpoint: OutPoint,
    /// Transaction output
    pub txout: TxOut,
    /// Type of keychain, as short 0 for "external" or 1 for "internal"
    pub keychain: u16,
}

impl From<&bdk::LocalUtxo> for LocalUtxo {
    fn from(lu: &bdk::LocalUtxo) -> Self {
        LocalUtxo {
            outpoint: OutPoint::from(&lu.outpoint),
            txout: TxOut::from(&lu.txout),
            keychain: lu.keychain as u16,
        }
    }
}
