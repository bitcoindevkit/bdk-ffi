use bdk::Wallet;
use bdk::database::MemoryDatabase;
use bdk::bitcoin::Network;
// use crate::error::FfiError;
use std::sync::{RwLock, Mutex};
use std::vec::Vec;

use bdk::database::BatchDatabase;
use bdk::sled;
use bdk::sled::Tree;

//mod error;
//mod types;
//mod wallet;

uniffi_macros::include_scaffolding!("bdk");

struct OfflineWallet {
    wallet: Mutex<Wallet<(), Tree>>,
    //wallet: RwLock<Vec<String>>
}

impl OfflineWallet {
    fn new(descriptor: String) -> Self {
        let database = sled::open("testdb").unwrap();
        let tree = database.open_tree("test").unwrap();

        let wallet = Wallet::new_offline(
            &descriptor,
            None,
            Network::Regtest,
            tree,
        ).unwrap();

        OfflineWallet {
            wallet: Mutex::new(wallet)
        }

//        OfflineWallet {
//            wallet: RwLock::new(Vec::new())
//        }
    }
}

uniffi::deps::static_assertions::assert_impl_all!(OfflineWallet: Sync, Send);


