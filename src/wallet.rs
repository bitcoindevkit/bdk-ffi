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

#[derive_ReprC]
#[ReprC::opaque]
pub struct VoidResult {
    raw: Result<(), String>,
}

#[ffi_export]
fn get_void_err(void_result: &VoidResult) -> Option<char_p_boxed> {
    void_result
        .raw
        .as_ref()
        .err()
        .map(|s| char_p_boxed::try_from(s.clone()).unwrap())
}

#[ffi_export]
fn free_void_result(void_result: Option<Box<VoidResult>>) {
    drop(void_result)
}

#[derive_ReprC]
#[ReprC::opaque]
pub struct StringResult {
    raw: Result<String, String>,
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
        .map(|s| char_p_boxed::try_from(s.clone()).unwrap())
}

#[ffi_export]
fn free_string_result(string_result: Option<Box<StringResult>>) {
    drop(string_result)
}

#[derive_ReprC]
#[ReprC::opaque]
pub struct WalletResult {
    raw: Result<Wallet<AnyBlockchain, AnyDatabase>, String>,
}

#[ffi_export]
fn new_wallet_result(
    name: char_p_ref,
    descriptor: char_p_ref,
    change_descriptor: Option<char_p_ref>,
) -> Box<WalletResult> {
    let name = name.to_string();
    let descriptor = descriptor.to_string();
    let change_descriptor = change_descriptor.map(|s| s.to_string());
    let wallet_result = new_wallet(name, descriptor, change_descriptor).map_err(|e| e.to_string());
    Box::new(WalletResult { raw: wallet_result })
}

fn new_wallet(
    name: String,
    descriptor: String,
    change_descriptor: Option<String>,
) -> Result<Wallet<AnyBlockchain, AnyDatabase>, Error> {
    let network = Testnet;
    let electrum_config = AnyBlockchainConfig::Electrum(ElectrumBlockchainConfig {
        url: "ssl://electrum.blockstream.info:60002".to_string(),
        socks5: None,
        retry: 5,
        timeout: None,
    });
    let blockchain_config = electrum_config;
    let client = AnyBlockchain::from_config(&blockchain_config)?;

    let database_config = AnyDatabaseConfig::Memory(());
    let database = AnyDatabase::from_config(&database_config)?;

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
        .map(|s| char_p_boxed::try_from(s.clone()).unwrap())
}

#[ffi_export]
fn sync_wallet(wallet_result: &WalletResult) -> Box<VoidResult> {
    let wallet_result_ref = wallet_result.raw.as_ref().map_err(|error| error.clone());
    let void_result = wallet_result_ref
        .and_then(|w| w.sync(log_progress(), Some(100)).map_err(|e| e.to_string()));
    Box::new(VoidResult { raw: void_result })
}

#[ffi_export]
fn new_address(wallet_result: &WalletResult) -> Box<StringResult> {
    let new_address = wallet_result
        .raw
        .as_ref()
        .map_err(|error| error.clone())
        .and_then(|w| w.get_address(New).map_err(|error| error.to_string()));
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
