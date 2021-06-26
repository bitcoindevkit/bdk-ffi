use crate::error::get_name;
use ::safer_ffi::prelude::*;
use bdk::bitcoin::network::constants::Network::Testnet;
use bdk::blockchain::{
    log_progress, AnyBlockchain, AnyBlockchainConfig, ConfigurableBlockchain,
    ElectrumBlockchainConfig,
};
use bdk::database::{AnyDatabase, AnyDatabaseConfig, ConfigurableDatabase};
use bdk::wallet::AddressIndex::New;
use bdk::{Error, Wallet};
use safer_ffi::boxed::Box;
use safer_ffi::char_p::{char_p_boxed, char_p_ref};
use crate::blockchain::BlockchainConfig;
use crate::database::DatabaseConfig;

#[derive_ReprC]
#[ReprC::opaque]
pub struct VoidResult {
    raw: Result<(), Error>,
}

#[ffi_export]
fn get_void_err(void_result: &VoidResult) -> Option<char_p_boxed> {
    void_result
        .raw
        .as_ref()
        .err()
        .map(|e| char_p_boxed::try_from(get_name(e)).unwrap())
}

#[ffi_export]
fn free_void_result(void_result: Option<Box<VoidResult>>) {
    drop(void_result)
}

#[derive_ReprC]
#[ReprC::opaque]
pub struct StringResult {
    raw: Result<String, Error>,
}

#[ffi_export]
fn get_string_ok(string_result: &StringResult) -> Option<char_p_boxed> {
    string_result
        .raw
        .as_ref()
        .ok()
        .map(|s| char_p_boxed::try_from(s.clone()).unwrap())
}

#[ffi_export]
fn get_string_err(string_result: &StringResult) -> Option<char_p_boxed> {
    string_result
        .raw
        .as_ref()
        .err()
        .map(|e| char_p_boxed::try_from(get_name(e)).unwrap())
}

#[ffi_export]
fn free_string_result(string_result: Option<Box<StringResult>>) {
    drop(string_result)
}

#[derive_ReprC]
#[ReprC::opaque]
pub struct WalletRef<'lt> {
    raw: &'lt Wallet<AnyBlockchain, AnyDatabase>,
}

#[ffi_export]
fn free_wallet_ref(wallet_ref: Option<Box<WalletRef>>) {
    drop(wallet_ref)
}

#[derive_ReprC]
#[ReprC::opaque]
pub struct WalletResult {
    raw: Result<Wallet<AnyBlockchain, AnyDatabase>, Error>,
}

#[ffi_export]
fn new_wallet_result(
    descriptor: char_p_ref,
    change_descriptor: Option<char_p_ref>,
    blockchain_config: &BlockchainConfig,
    database_config: &DatabaseConfig,
) -> Box<WalletResult> {
    
    let descriptor = descriptor.to_string();
    let change_descriptor = change_descriptor.map(|s| s.to_string());
    let bc_config = &blockchain_config.raw;
    let db_config  = &database_config.raw;
    let wallet_result = new_wallet(descriptor, change_descriptor, bc_config, db_config);
    Box::new(WalletResult { raw: wallet_result })
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
fn get_wallet_err(wallet_result: &WalletResult) -> Option<char_p_boxed> {
    wallet_result
        .raw
        .as_ref()
        .err()
        .map(|e| char_p_boxed::try_from(get_name(&e)).unwrap())
}

#[ffi_export]
fn get_wallet_ok<'lt>(wallet_result: &'lt WalletResult) -> Option<Box<WalletRef<'lt>>> {
    wallet_result
        .raw
        .as_ref()
        .ok()
        .map(|w| Box::new(WalletRef { raw: w }))
}

#[ffi_export]
fn sync_wallet<'lt>(wallet_ref: &'lt WalletRef<'lt>) -> Box<VoidResult> {
    let void_result = wallet_ref.raw.sync(log_progress(), Some(100));
    Box::new(VoidResult { raw: void_result })
}

#[ffi_export]
fn new_address<'lt>(wallet_ref: &'lt WalletRef<'lt>) -> Box<StringResult> {
    let new_address = wallet_ref.raw.get_address(New);
    let string_result = new_address.map(|a| a.to_string());
    Box::new(StringResult { raw: string_result })
}

#[ffi_export]
fn free_wallet_result(wallet_result: Option<Box<WalletResult>>) {
    drop(wallet_result)
}

/// Frees a Rust-allocated string
#[ffi_export]
fn free_string(string: Option<char_p_boxed>) {
    drop(string)
}
