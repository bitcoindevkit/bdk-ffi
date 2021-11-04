use bdk::bitcoin::secp256k1::Secp256k1;
use bdk::bitcoin::util::psbt::PartiallySignedTransaction;
use bdk::bitcoin::{Address, Network};
use bdk::blockchain::any::{AnyBlockchain, AnyBlockchainConfig};
use bdk::blockchain::Progress;
use bdk::blockchain::{
    electrum::ElectrumBlockchainConfig, esplora::EsploraBlockchainConfig, ConfigurableBlockchain,
};
use bdk::database::any::{AnyDatabase, SledDbConfiguration};
use bdk::database::{AnyDatabaseConfig, ConfigurableDatabase};
use bdk::keys::bip39::{Language, Mnemonic, MnemonicType};
use bdk::keys::{DerivableKey, ExtendedKey, GeneratableKey, GeneratedKey};
use bdk::miniscript::BareCtx;
use bdk::wallet::AddressIndex;
use bdk::{ConfirmationTime, Error, FeeRate, SignOptions, Wallet};
use std::convert::TryFrom;
use std::str::FromStr;
use std::sync::{Mutex, MutexGuard};

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

trait WalletHolder<B> {
    fn get_wallet(&self) -> MutexGuard<Wallet<B, AnyDatabase>>;
}

struct OfflineWallet {
    wallet: Mutex<Wallet<(), AnyDatabase>>,
}

impl WalletHolder<()> for OfflineWallet {
    fn get_wallet(&self) -> MutexGuard<Wallet<(), AnyDatabase>> {
        self.wallet.lock().unwrap()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TransactionDetails {
    pub fees: Option<u64>,
    pub received: u64,
    pub sent: u64,
    pub id: String,
}

type Confirmation = ConfirmationTime;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Transaction {
    Unconfirmed {
        details: TransactionDetails,
    },
    Confirmed {
        details: TransactionDetails,
        confirmation: Confirmation,
    },
}

trait OfflineWalletOperations<B>: WalletHolder<B> {
    fn get_new_address(&self) -> String {
        self.get_wallet()
            .get_address(AddressIndex::New)
            .unwrap()
            .address
            .to_string()
    }

    fn get_last_unused_address(&self) -> String {
        self.get_wallet()
            .get_address(AddressIndex::LastUnused)
            .unwrap()
            .address
            .to_string()
    }

    fn get_balance(&self) -> Result<u64, Error> {
        self.get_wallet().get_balance()
    }

    fn sign<'a>(&self, psbt: &'a PartiallySignedBitcoinTransaction) -> Result<(), Error> {
        let mut psbt = psbt.internal.lock().unwrap();
        let finalized = self.get_wallet().sign(&mut psbt, SignOptions::default())?;
        match finalized {
            true => Ok(()),
            false => Err(BdkError::Generic(format!(
                "transaction signing not finalized {:?}",
                psbt
            ))),
        }
    }

    fn get_transactions(&self) -> Result<Vec<Transaction>, Error> {
        let transactions = self.get_wallet().list_transactions(true)?;
        Ok(transactions
            .iter()
            .map(|x| -> Transaction {
                let details = TransactionDetails {
                    fees: x.fee,
                    id: x.txid.to_string(),
                    received: x.received,
                    sent: x.sent,
                };
                match x.confirmation_time.clone() {
                    Some(confirmation) => Transaction::Confirmed {
                        details,
                        confirmation,
                    },
                    None => Transaction::Unconfirmed { details },
                }
            })
            .collect())
    }
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
}

impl OfflineWalletOperations<()> for OfflineWallet {}

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

struct PartiallySignedBitcoinTransaction {
    internal: Mutex<PartiallySignedTransaction>,
}

impl PartiallySignedBitcoinTransaction {
    fn new(
        online_wallet: &OnlineWallet,
        recipient: String,
        amount: u64,
        fee_rate: Option<f32>, // satoshis per vbyte
    ) -> Result<Self, Error> {
        let wallet = online_wallet.get_wallet();
        match Address::from_str(&recipient) {
            Ok(address) => {
                let (psbt, _) = {
                    let mut builder = wallet.build_tx();
                    builder.add_recipient(address.script_pubkey(), amount);
                    if let Some(sat_per_vb) = fee_rate {
                        builder.fee_rate(FeeRate::from_sat_per_vb(sat_per_vb));
                    }
                    builder.finish()?
                };
                Ok(PartiallySignedBitcoinTransaction {
                    internal: Mutex::new(psbt),
                })
            }
            Err(..) => Err(BdkError::Generic(
                "failed to read wallet address".to_string(),
            )),
        }
    }
}

impl OnlineWallet {
    fn new(
        descriptor: String,
        change_descriptor: Option<String>,
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
            change_descriptor.to_owned().as_ref(),
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

    fn broadcast<'a>(&self, psbt: &'a PartiallySignedBitcoinTransaction) -> Result<String, Error> {
        let tx = psbt.internal.lock().unwrap().clone().extract_tx();
        let tx_id = self.get_wallet().broadcast(tx)?;
        Ok(tx_id.to_string())
    }
}

impl WalletHolder<AnyBlockchain> for OnlineWallet {
    fn get_wallet(&self) -> MutexGuard<Wallet<AnyBlockchain, AnyDatabase>> {
        self.wallet.lock().unwrap()
    }
}

impl OfflineWalletOperations<AnyBlockchain> for OnlineWallet {}

pub struct ExtendedKeyInfo {
    mnemonic: String,
    xprv: String,
    fingerprint: String,
}

fn generate_extended_key(
    network: Network,
    mnemonic_type: MnemonicType,
    password: Option<String>,
) -> Result<ExtendedKeyInfo, Error> {
    let mnemonic: GeneratedKey<_, BareCtx> =
        Mnemonic::generate((mnemonic_type, Language::English)).unwrap();
    let mnemonic = mnemonic.into_key();
    let xkey: ExtendedKey = (mnemonic.clone(), password).into_extended_key()?;
    let xprv = xkey.into_xprv(network).unwrap();
    let fingerprint = xprv.fingerprint(&Secp256k1::new());
    Ok(ExtendedKeyInfo {
        mnemonic: mnemonic.to_string(),
        xprv: xprv.to_string(),
        fingerprint: fingerprint.to_string(),
    })
}

fn restore_extended_key(
    network: Network,
    mnemonic: String,
    password: Option<String>,
) -> Result<ExtendedKeyInfo, Error> {
    let mnemonic = Mnemonic::from_phrase(mnemonic.as_ref(), Language::English).unwrap();
    let xkey: ExtendedKey = (mnemonic.clone(), password).into_extended_key()?;
    let xprv = xkey.into_xprv(network).unwrap();
    let fingerprint = xprv.fingerprint(&Secp256k1::new());
    Ok(ExtendedKeyInfo {
        mnemonic: mnemonic.to_string(),
        xprv: xprv.to_string(),
        fingerprint: fingerprint.to_string(),
    })
}

uniffi::deps::static_assertions::assert_impl_all!(OfflineWallet: Sync, Send);
uniffi::deps::static_assertions::assert_impl_all!(OnlineWallet: Sync, Send);
