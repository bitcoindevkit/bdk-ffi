use ::safer_ffi::prelude::*;
use bdk::bitcoin::network::constants::Network::Testnet;
use bdk::blockchain::{ElectrumBlockchain, log_progress};
use bdk::electrum_client::Client;
use bdk::sled;
use bdk::sled::Tree;
use bdk::Wallet;

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
fn mid_point(a: Option<repr_c::Box<Point>>, b: Option<repr_c::Box<Point>>) -> repr_c::Box<Point> {
    let a = a.unwrap();
    let b = b.unwrap();
    repr_c::Box::new(Point {
        x: (a.x + b.x) / 2.,
        y: (a.y + b.y) / 2.,
    })
}

/// Pretty-prints a point using Rust's formatting logic.
#[ffi_export]
fn print_point(point: Option<repr_c::Box<Point>>) {
    println!("{:?}", point);
}

#[ffi_export]
fn new_point(x: f64, y: f64) -> repr_c::Box<Point> {
    repr_c::Box::new(Point { x, y })
}

#[ffi_export]
fn free_point(point: Option<repr_c::Box<Point>>) {
    drop(point)
}

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
fn new_wallet(
    name: char_p::Ref<'_>,
    descriptor: char_p::Ref<'_>,
    change_descriptor: Option<char_p::Ref<'_>>,
) -> repr_c::Box<WalletPtr> {
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

    repr_c::Box::new(WalletPtr::from(wallet))
}

#[ffi_export]
fn sync_wallet( wallet: repr_c::Box<WalletPtr>) {
    println!("before sync");
    wallet.raw.sync(log_progress(), Some(100));
    println!("after sync");
}

/// The following test function is necessary for the header generation.
#[::safer_ffi::cfg_headers]
#[test]
fn generate_headers() -> ::std::io::Result<()> {
    ::safer_ffi::headers::builder()
        .to_file("bdk_ffi.h")?
        .generate()
}
