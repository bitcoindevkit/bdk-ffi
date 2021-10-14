use bdk::bitcoin::Network;
use bdk::blockchain::any::{AnyBlockchain, AnyBlockchainConfig};
use bdk::blockchain::Progress;
use bdk::blockchain::{
    electrum::ElectrumBlockchainConfig, esplora::EsploraBlockchainConfig, ConfigurableBlockchain,
};
use bdk::database::any::{AnyDatabase, SledDbConfiguration};
use bdk::database::{AnyDatabaseConfig, ConfigurableDatabase};
use bdk::wallet::AddressIndex;
use bdk::Error;
use bdk::Wallet;
use std::convert::TryFrom;
use std::sync::Mutex;

uniffi_macros::include_scaffolding!("bdk");

type BdkError = Error;

pub enum DatabaseConfig {
    Memory { junk: String },
    Sled { config: SledDbConfiguration },
}

pub struct ElectrumConfig {
    pub url: String,
    pub socks5: Option<String>,
    pub retry: u8,
    pub timeout: Option<u8>,
    pub stop_gap: u64,
}

pub struct EsploraConfig {
    pub base_url: String,
    pub proxy: Option<String>,
    pub timeout_read: u64,
    pub timeout_write: u64,
    pub stop_gap: u64,
}

pub enum BlockchainConfig {
    Electrum { config: ElectrumConfig },
    Esplora { config: EsploraConfig },
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
            DatabaseConfig::Sled { config } => AnyDatabaseConfig::Sled(config),
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

struct OnlineWallet {
    wallet: Mutex<Wallet<AnyBlockchain, AnyDatabase>>,
}

pub trait BdkProgress: Send + Sync {
    fn update(&self, progress: f32, message: Option<String>);
}

struct BdkProgressHolder {
    progress_update: Box<dyn BdkProgress>,
}

impl Progress for BdkProgressHolder {
    fn update(&self, progress: f32, message: Option<String>) -> Result<(), Error> {
        self.progress_update.update(progress, message);
        Ok(())
    }
}

impl OnlineWallet {
    fn new(
        descriptor: String,
        network: Network,
        database_config: DatabaseConfig,
        blockchain_config: BlockchainConfig,
    ) -> Result<Self, BdkError> {
        let any_database_config = match database_config {
            DatabaseConfig::Memory { .. } => AnyDatabaseConfig::Memory(()),
            DatabaseConfig::Sled { config } => AnyDatabaseConfig::Sled(config),
        };
        let any_blockchain_config = match blockchain_config {
            BlockchainConfig::Electrum { config } => {
                AnyBlockchainConfig::Electrum(ElectrumBlockchainConfig {
                    retry: config.retry,
                    socks5: config.socks5,
                    timeout: config.timeout,
                    url: config.url,
                    stop_gap: usize::try_from(config.stop_gap).unwrap(),
                })
            }
            BlockchainConfig::Esplora { config } => {
                AnyBlockchainConfig::Esplora(EsploraBlockchainConfig {
                    base_url: config.base_url,
                    proxy: config.proxy,
                    timeout_read: config.timeout_read,
                    timeout_write: config.timeout_write,
                    stop_gap: usize::try_from(config.stop_gap).unwrap(),
                })
            }
        };
        let database = AnyDatabase::from_config(&any_database_config)?;
        let blockchain = AnyBlockchain::from_config(&any_blockchain_config)?;
        let wallet = Mutex::new(Wallet::new(
            &descriptor,
            None,
            network,
            database,
            blockchain,
        )?);
        Ok(OnlineWallet { wallet })
    }

    fn get_network(&self) -> Network {
        self.wallet.lock().unwrap().network()
    }

    fn sync(
        &self,
        progress_update: Box<dyn BdkProgress>,
        max_address_param: Option<u32>,
    ) -> Result<(), BdkError> {
        progress_update.update(21.0, Some("message".to_string()));
        self.wallet
            .lock()
            .unwrap()
            .sync(BdkProgressHolder { progress_update }, max_address_param)
    }
}

uniffi::deps::static_assertions::assert_impl_all!(OfflineWallet: Sync, Send);
uniffi::deps::static_assertions::assert_impl_all!(OnlineWallet: Sync, Send);
