use bdk::bitcoin::Network;
use bdk::database::any::{AnyDatabase, SledDbConfiguration};
use bdk::database::{AnyDatabaseConfig, ConfigurableDatabase};
use bdk::wallet::AddressIndex;
use bdk::Error;
use bdk::Wallet;

use std::sync::Mutex;

uniffi_macros::include_scaffolding!("bdk");

type BdkError = Error;

pub enum DatabaseConfig {
    Memory { junk: String },
    Sled { configuration: SledDbConfiguration },
}

struct OfflineWallet {
    wallet: Mutex<Wallet<(), AnyDatabase>>,
}

impl OfflineWallet {
    fn new(
        descriptor: String,
        network: Network,
        database_config: DatabaseConfig,
    ) -> Result<Self, BdkError> {
        let any_database_config = match database_config {
            DatabaseConfig::Memory { .. } => AnyDatabaseConfig::Memory(()),
            DatabaseConfig::Sled { configuration } => AnyDatabaseConfig::Sled(configuration),
        };
        let database = AnyDatabase::from_config(&any_database_config)?;
        let wallet = Mutex::new(Wallet::new_offline(&descriptor, None, network, database)?);
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
