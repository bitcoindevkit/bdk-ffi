use bdk::Wallet;
use bdk::database::MemoryDatabase;
use bdk::bitcoin::Network;
// use crate::error::FfiError;
use std::sync::RwLock;
use std::vec::Vec;

//mod error;
//mod types;
//mod wallet;

uniffi_macros::include_scaffolding!("bdk");

struct OfflineWallet {
    wallet: Wallet<(), MemoryDatabase>,
    //wallet: RwLock<Vec<String>>
}

impl OfflineWallet {
    fn new(descriptor: String) -> Self {
        let wallet = Wallet::new_offline(
            &descriptor,
            None,
            Network::Regtest,
            MemoryDatabase::new(),
        ).unwrap();

        OfflineWallet {
            wallet
        }

//        OfflineWallet {
//            wallet: RwLock::new(Vec::new())
//        }
    }
}

