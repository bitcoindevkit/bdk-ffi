use bdk::bitcoin::Network;
use bdk::sled;
use bdk::sled::Tree;
use bdk::wallet::AddressIndex;
use bdk::Wallet;
use std::sync::Mutex;

uniffi_macros::include_scaffolding!("bdk");

struct OfflineWallet {
    wallet: Mutex<Wallet<(), Tree>>,
    //wallet: RwLock<Vec<String>>
}

impl OfflineWallet {
    fn new(descriptor: String) -> Self {
        let database = sled::open("testdb").unwrap();
        let tree = database.open_tree("test").unwrap();

        let wallet = Wallet::new_offline(&descriptor, None, Network::Regtest, tree).unwrap();

        OfflineWallet {
            wallet: Mutex::new(wallet),
        }

        //        OfflineWallet {
        //            wallet: RwLock::new(Vec::new())
        //        }
    }

    fn get_new_address(&self) -> String {
        self.wallet
            .lock()
            .unwrap()
            .get_address(AddressIndex::New)
            .unwrap()
            .address
            .to_string()
    }
}

uniffi::deps::static_assertions::assert_impl_all!(OfflineWallet: Sync, Send);
