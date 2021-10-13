use bdk::bitcoin::Network;
use bdk::database::MemoryDatabase;
use bdk::wallet::AddressIndex;
use bdk::Error;
use bdk::Wallet;

use std::sync::Mutex;

type BdkError = Error;

uniffi_macros::include_scaffolding!("bdk");
struct OfflineWallet {
    wallet: Mutex<Wallet<(), MemoryDatabase>>,
}

impl OfflineWallet {
    fn new(descriptor: String) -> Result<Self, BdkError> {
        let database = MemoryDatabase::default();
        let wallet = Mutex::new(Wallet::new_offline(
            &descriptor,
            None,
            Network::Regtest,
            database,
        )?);
        Ok(OfflineWallet { wallet })
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
