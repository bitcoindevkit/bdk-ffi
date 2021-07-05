use std::convert::TryFrom;
use std::ffi::CString;

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

use crate::error::FfiError;
use crate::types::{FfiResult, FfiResultVoid};
use crate::wallet::transaction::{LocalUtxo, TransactionDetails};

mod blockchain;
mod database;
mod transaction;

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
) -> FfiResult<Option<Box<OpaqueWallet>>> {
    let descriptor = descriptor.to_string();
    let change_descriptor = change_descriptor.map(|s| s.to_string());
    let bc_config = &blockchain_config.raw;
    let db_config = &database_config.raw;
    let wallet_result = new_wallet(descriptor, change_descriptor, bc_config, db_config);

    match wallet_result {
        Ok(w) => FfiResult {
            ok: Some(Box::new(OpaqueWallet { raw: w })),
            err: FfiError::None,
        },
        Err(e) => FfiResult {
            ok: None,
            err: FfiError::from(&e),
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
fn free_wallet_result(wallet_result: FfiResult<Option<Box<OpaqueWallet>>>) {
    drop(wallet_result);
}

// wallet operations

#[ffi_export]
fn sync_wallet(opaque_wallet: &OpaqueWallet) -> FfiResultVoid {
    let int_result = opaque_wallet.raw.sync(log_progress(), Some(100));
    match int_result {
        Ok(_v) => FfiResultVoid {
            err: FfiError::None,
        },
        Err(e) => FfiResultVoid {
            err: FfiError::from(&e),
        },
    }
}

#[ffi_export]
fn new_address(opaque_wallet: &OpaqueWallet) -> FfiResult<char_p_boxed> {
    let new_address = opaque_wallet.raw.get_address(New);
    let string_result = new_address.map(|a| a.to_string());
    match string_result {
        Ok(a) => FfiResult {
            ok: char_p_boxed::try_from(a).unwrap(),
            err: FfiError::None,
        },
        Err(e) => FfiResult {
            ok: char_p_boxed::from(CString::default()),
            err: FfiError::from(&e),
        },
    }
}

#[ffi_export]
fn list_unspent(opaque_wallet: &OpaqueWallet) -> FfiResult<repr_c::Vec<LocalUtxo>> {
    let unspent_result = opaque_wallet.raw.list_unspent();

    match unspent_result {
        Ok(v) => FfiResult {
            ok: {
                let ve: Vec<LocalUtxo> = v.iter().map(|lu| LocalUtxo::from(lu)).collect();
                repr_c::Vec::from(ve)
            },
            err: FfiError::None,
        },
        Err(e) => FfiResult {
            ok: repr_c::Vec::EMPTY,
            err: FfiError::from(&e),
        },
    }
}

#[ffi_export]
fn free_veclocalutxo_result(unspent_result: FfiResult<repr_c::Vec<LocalUtxo>>) {
    drop(unspent_result)
}

#[ffi_export]
fn balance(opaque_wallet: &OpaqueWallet) -> FfiResult<u64> {
    let balance_result = opaque_wallet.raw.get_balance();

    match balance_result {
        Ok(b) => FfiResult {
            ok: b,
            err: FfiError::None,
        },
        Err(e) => FfiResult {
            ok: u64::MIN,
            err: FfiError::from(&e),
        },
    }
}

#[ffi_export]
fn list_transactions(opaque_wallet: &OpaqueWallet) -> FfiResult<repr_c::Vec<TransactionDetails>> {
    let transactions_result = opaque_wallet.raw.list_transactions(false);

    match transactions_result {
        Ok(v) => FfiResult {
            ok: {
                let ve: Vec<TransactionDetails> =
                    v.iter().map(|t| TransactionDetails::from(t)).collect();
                repr_c::Vec::from(ve)
            },
            err: FfiError::None,
        },
        Err(e) => FfiResult {
            ok: repr_c::Vec::EMPTY,
            err: FfiError::from(&e),
        },
    }
}

#[ffi_export]
fn free_vectxdetails_result(txdetails_result: FfiResult<repr_c::Vec<TransactionDetails>>) {
    drop(txdetails_result)
}
