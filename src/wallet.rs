use ::safer_ffi::prelude::*;
use bdk::bitcoin::network::constants::Network::Testnet;
use bdk::blockchain::{
    log_progress, AnyBlockchain, AnyBlockchainConfig, ConfigurableBlockchain,
    ElectrumBlockchainConfig,
};
use bdk::database::{AnyDatabase, AnyDatabaseConfig, ConfigurableDatabase};
use bdk::wallet::AddressIndex::New;
use bdk::Wallet;
use safer_ffi::boxed::Box;
use safer_ffi::char_p::{char_p_boxed, char_p_ref};

#[derive_ReprC]
#[ReprC::opaque]
pub struct WalletPtr {
    raw: Wallet<AnyBlockchain, AnyDatabase>,
}

impl From<Wallet<AnyBlockchain, AnyDatabase>> for WalletPtr {
    fn from(wallet: Wallet<AnyBlockchain, AnyDatabase>) -> Self {
        WalletPtr { raw: wallet }
    }
}

#[ffi_export]
fn new_wallet(
    name: char_p_ref,
    descriptor: char_p_ref,
    change_descriptor: Option<char_p_ref>,
) -> Box<WalletPtr> {
    let network = Testnet;
    let _name = name.to_string();
    let descriptor = descriptor.to_string();
    let change_descriptor = change_descriptor.map(|s| s.to_string());

    let electrum_config = AnyBlockchainConfig::Electrum(ElectrumBlockchainConfig {
        url: "ssl://electrum.blockstream.info:60002".to_string(),
        socks5: None,
        retry: 5,
        timeout: None,
    });
    let blockchain_config = electrum_config;
    let client = AnyBlockchain::from_config(&blockchain_config).unwrap();

    let database_config = AnyDatabaseConfig::Memory(());
    let database = AnyDatabase::from_config(&database_config).unwrap();

    let descriptor: &str = descriptor.as_str();
    let change_descriptor: Option<&str> = change_descriptor.as_deref();

    let wallet = Wallet::new(descriptor, change_descriptor, network, database, client).unwrap();
    
    Box::new(WalletPtr::from(wallet))
}

#[ffi_export]
fn sync_wallet(wallet: &WalletPtr) {
    let _r = wallet.raw.sync(log_progress(), Some(100));
}

#[ffi_export]
fn new_address(wallet: &WalletPtr) -> char_p_boxed {
    let new_address = wallet.raw.get_address(New);
    let new_address = new_address.unwrap();
    let new_address = new_address.to_string();
    new_address.try_into().unwrap()
}

/// Frees a Rust-allocated string
#[ffi_export]
fn free_string(string: char_p_boxed) {
    drop(string)
}

#[ffi_export]
fn free_wallet(wallet: Option<Box<WalletPtr>>) {
    drop(wallet)
}
