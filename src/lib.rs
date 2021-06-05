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

/// Concatenate two input UTF-8 (_e.g._, ASCII) strings.
///
/// \remark The returned string must be freed with `rust_free_string`
#[ffi_export]
fn concat_string<'a>(fst: char_p_ref<'a>, snd: char_p_ref<'a>)
           -> char_p_boxed
{
    let fst = fst.to_str(); // : &'_ str
    let snd = snd.to_str(); // : &'_ str
    let ccat = format!("{}{}", fst, snd).try_into().unwrap(); 
    ccat
}

#[ffi_export]
fn print_string (string: char_p_ref)
{
    println!("{}", string);
}

/// Frees a Rust-allocated string.
#[ffi_export]
fn free_string (string: char_p_boxed)
{
    drop(string)
}

/// A `struct` usable from both Rust and C
#[derive_ReprC]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Point {
    x: f64,
    y: f64,
}

/* Export a Rust function to the C world. */
/// Returns the middle point of `[a, b]`.
#[ffi_export]
fn mid_point(a: Point, b: Point) -> Point {
    Point {
        x: (a.x + b.x) / 2.,
        y: (a.y + b.y) / 2.,
    }
}

/// Pretty-prints a point using Rust's formatting logic.
#[ffi_export]
fn print_point(point: Point) {
    println!("{:?}", point);
}

#[ffi_export]
fn new_point(x: f64, y: f64) -> Point {
    Point { x, y }
}

//#[ffi_export]
//fn free_point(point: Point) {
//    drop(point)
//}

#[derive_ReprC]
#[ReprC::opaque]
pub struct WalletPtr {
    raw: Wallet<ElectrumBlockchain, Tree>,
}

impl From<Wallet<ElectrumBlockchain, Tree>> for WalletPtr {
    fn from(wallet: Wallet<ElectrumBlockchain, Tree>) -> Self {
        WalletPtr {
            raw: wallet,
        }
    }
}

#[ffi_export]
fn new_wallet<'a>(
    name: char_p_ref<'a>,
    descriptor: char_p_ref<'a>,
    change_descriptor: Option<char_p_ref<'a>>,
) -> Box<WalletPtr> {
    let name = name.to_string();
    let descriptor = descriptor.to_string();
    let change_descriptor = change_descriptor.map(|s| s.to_string());

    let database = sled::open("./wallet_db").unwrap();
    let tree = database.open_tree(name.clone()).unwrap();

    let descriptor: &str = descriptor.as_str();
    let change_descriptor: Option<&str> = change_descriptor.as_deref();

    let electrum_url = "ssl://electrum.blockstream.info:60002";
    let client = Client::new(&electrum_url).unwrap();

    let wallet = Wallet::new(
        descriptor,
        change_descriptor,
        Testnet,
        tree,
        ElectrumBlockchain::from(client),
    )
    .unwrap();

    Box::new(WalletPtr::from(wallet))
}

#[ffi_export]
fn sync_wallet( wallet: &Box<WalletPtr>) {
    println!("before sync");
    let _r = wallet.raw.sync(log_progress(), Some(100));
    println!("after sync");
}

#[ffi_export]
fn new_address( wallet: &Box<WalletPtr>) -> char_p_boxed {
    println!("before new_address");
    let new_address = wallet.raw.get_address(New);
    println!("after new_address: {:?}", new_address);
    let new_address = new_address.unwrap();
    let new_address = new_address.to_string();
    println!("new address: ${}", new_address);
    new_address.try_into().unwrap()
}

#[ffi_export]
fn free_wallet( wallet: Box<WalletPtr>) {
    drop(wallet)
}

/// The following test function is necessary for the header generation.
#[::safer_ffi::cfg_headers]
#[test]
fn generate_headers() -> ::std::io::Result<()> {
    ::safer_ffi::headers::builder()
        .to_file("bdk_ffi.h")?
        .generate()
}
