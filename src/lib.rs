#![deny(unsafe_code)] /* No `unsafe` needed! */

use ::safer_ffi::prelude::*;
use bdk::bitcoin::network::constants::Network::Testnet;
use bdk::blockchain::{ElectrumBlockchain, log_progress};
use bdk::electrum_client::Client;
use bdk::sled;
use bdk::sled::Tree;
use bdk::Wallet;
use bdk::wallet::AddressIndex::New;
use safer_ffi::char_p::{char_p_ref, char_p_boxed};
use safer_ffi::boxed::Box;

#[ffi_export]
fn print_string (string: char_p_ref)
{
    println!("{}", string);
}

/// Concatenate two input UTF-8 (_e.g._, ASCII) strings.
///
/// The returned string must be freed with `rust_free_string`
#[ffi_export]
fn concat_string(fst: char_p_ref, snd: char_p_ref)
           -> char_p_boxed
{
    let fst = fst.to_str(); // : &'_ str
    let snd = snd.to_str(); // : &'_ str
    let ccat = format!("{}{}", fst, snd).try_into().unwrap(); 
    ccat
}

/// Frees a Rust-allocated string
#[ffi_export]
fn free_string (string: char_p_boxed)
{
    drop(string)
}

/// A `struct` usable from both Rust and C
#[derive_ReprC]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Config {
    name: char_p_boxed,
    count: i64
}

/// Debug print a Point
#[ffi_export]
fn print_config(config: &Config) {
    println!("{:?}", config);
}

/// Create a new Config
#[ffi_export]
fn new_config(name: char_p_ref, count: i64) -> Box<Config> {
    let name = name.to_string().try_into().unwrap();
    Box::new(Config { name, count })
}

#[ffi_export]
fn free_config(config: Box<Config>) {
    drop(config)
}

//#[derive_ReprC]
//#[ReprC::opaque]
//pub struct WalletPtr {
//    raw: Wallet<ElectrumBlockchain, Tree>,
//}

//impl From<Wallet<ElectrumBlockchain, Tree>> for WalletPtr {
//    fn from(wallet: Wallet<ElectrumBlockchain, Tree>) -> Self {
//        WalletPtr {
//            raw: wallet,
//        }
//    }
//}

//#[ffi_export]
//fn new_wallet<'a>(
//    name: char_p_ref<'a>,
//    descriptor: char_p_ref<'a>,
//    change_descriptor: Option<char_p_ref<'a>>,
//) -> Box<WalletPtr> {
//    let name = name.to_string();
//    let descriptor = descriptor.to_string();
//    let change_descriptor = change_descriptor.map(|s| s.to_string());
//
//    let database = sled::open("./wallet_db").unwrap();
//    let tree = database.open_tree(name.clone()).unwrap();
//
//    let descriptor: &str = descriptor.as_str();
//    let change_descriptor: Option<&str> = change_descriptor.as_deref();
//
//    let electrum_url = "ssl://electrum.blockstream.info:60002";
//    let client = Client::new(&electrum_url).unwrap();
//
//    let wallet = Wallet::new(
//        descriptor,
//        change_descriptor,
//        Testnet,
//        tree,
//        ElectrumBlockchain::from(client),
//    )
//    .unwrap();
//
//    Box::new(WalletPtr::from(wallet))
//}

//#[ffi_export]
//fn sync_wallet( wallet: &Box<WalletPtr>) {
//    println!("before sync");
//    let _r = wallet.raw.sync(log_progress(), Some(100));
//    println!("after sync");
//}

//#[ffi_export]
//fn new_address( wallet: &Box<WalletPtr>) -> char_p_boxed {
//    println!("before new_address");
//    let new_address = wallet.raw.get_address(New);
//    println!("after new_address: {:?}", new_address);
//    let new_address = new_address.unwrap();
//    let new_address = new_address.to_string();
//    println!("new address: ${}", new_address);
//    new_address.try_into().unwrap()
//}

//#[ffi_export]
//fn free_wallet( wallet: Box<WalletPtr>) {
//    drop(wallet)
//}

/// The following test function is necessary for the header generation.
#[::safer_ffi::cfg_headers]
#[test]
fn generate_headers() -> ::std::io::Result<()> {
    ::safer_ffi::headers::builder()
        .to_file("bdk_ffi.h")?
        .generate()
}
