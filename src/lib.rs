use bdk::bitcoin::hashes::hex::ToHex;
use bdk::bitcoin::secp256k1::Secp256k1;
use bdk::bitcoin::util::psbt::PartiallySignedTransaction;
use bdk::bitcoin::{Address, Network, Script, Txid};
use bdk::blockchain::any::{AnyBlockchain, AnyBlockchainConfig};
use bdk::blockchain::{
    electrum::ElectrumBlockchainConfig, esplora::EsploraBlockchainConfig, ConfigurableBlockchain,
};
use bdk::blockchain::{Blockchain as BdkBlockchain, Progress as BdkProgress};
use bdk::database::any::{AnyDatabase, SledDbConfiguration, SqliteDbConfiguration};
use bdk::database::{AnyDatabaseConfig, ConfigurableDatabase};
use bdk::keys::bip39::{Language, Mnemonic, WordCount};
use bdk::keys::{DerivableKey, ExtendedKey, GeneratableKey, GeneratedKey};
use bdk::miniscript::BareCtx;
use bdk::wallet::AddressIndex;
use bdk::{
    BlockTime, Error, FeeRate, SignOptions, SyncOptions as BdkSyncOptions, Wallet as BdkWallet,
};
use std::convert::TryFrom;
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;
use std::sync::{Arc, Mutex, MutexGuard};

uniffi_macros::include_scaffolding!("bdk");

type BdkError = Error;

pub enum DatabaseConfig {
    Memory,
    Sled { config: SledDbConfiguration },
    Sqlite { config: SqliteDbConfiguration },
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
    pub concurrency: Option<u8>,
    pub stop_gap: u64,
    pub timeout: Option<u64>,
}

pub enum BlockchainConfig {
    Electrum { config: ElectrumConfig },
    Esplora { config: EsploraConfig },
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TransactionDetails {
    pub fee: Option<u64>,
    pub received: u64,
    pub sent: u64,
    pub txid: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Transaction {
    Unconfirmed {
        details: TransactionDetails,
    },
    Confirmed {
        details: TransactionDetails,
        confirmation: BlockTime,
    },
}

impl From<&bdk::TransactionDetails> for TransactionDetails {
    fn from(x: &bdk::TransactionDetails) -> TransactionDetails {
        TransactionDetails {
            fee: x.fee,
            txid: x.txid.to_string(),
            received: x.received,
            sent: x.sent,
        }
    }
}

impl From<&bdk::TransactionDetails> for Transaction {
    fn from(x: &bdk::TransactionDetails) -> Transaction {
        match x.confirmation_time.clone() {
            Some(block_time) => Transaction::Confirmed {
                details: TransactionDetails::from(x),
                confirmation: block_time,
            },
            None => Transaction::Unconfirmed {
                details: TransactionDetails::from(x),
            },
        }
    }
}

struct Blockchain {
    blockchain_mutex: Mutex<AnyBlockchain>,
}

impl Blockchain {
    fn new(blockchain_config: BlockchainConfig) -> Result<Self, BdkError> {
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
                    concurrency: config.concurrency,
                    stop_gap: usize::try_from(config.stop_gap).unwrap(),
                    timeout: config.timeout,
                })
            }
        };
        let blockchain = AnyBlockchain::from_config(&any_blockchain_config)?;
        Ok(Self {
            blockchain_mutex: Mutex::new(blockchain),
        })
    }

    fn get_blockchain(&self) -> MutexGuard<AnyBlockchain> {
        self.blockchain_mutex.lock().expect("blockchain")
    }

    fn broadcast(&self, psbt: &PartiallySignedBitcoinTransaction) -> Result<(), Error> {
        let tx = psbt.internal.lock().unwrap().clone().extract_tx();
        self.get_blockchain().broadcast(&tx)
    }
}

struct Wallet {
    wallet_mutex: Mutex<BdkWallet<AnyDatabase>>,
}

pub trait Progress: Send + Sync + 'static {
    fn update(&self, progress: f32, message: Option<String>);
}

struct ProgressHolder {
    progress: Box<dyn Progress>,
}

impl BdkProgress for ProgressHolder {
    fn update(&self, progress: f32, message: Option<String>) -> Result<(), Error> {
        self.progress.update(progress, message);
        Ok(())
    }
}

impl fmt::Debug for ProgressHolder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ProgressHolder").finish_non_exhaustive()
    }
}

#[derive(Debug)]
struct PartiallySignedBitcoinTransaction {
    internal: Mutex<PartiallySignedTransaction>,
}

impl PartiallySignedBitcoinTransaction {
    fn new(psbt_base64: String) -> Result<Self, Error> {
        let psbt: PartiallySignedTransaction = PartiallySignedTransaction::from_str(&psbt_base64)?;
        Ok(PartiallySignedBitcoinTransaction {
            internal: Mutex::new(psbt),
        })
    }

    fn serialize(&self) -> String {
        let psbt = self.internal.lock().unwrap().clone();
        psbt.to_string()
    }

    fn txid(&self) -> String {
        let tx = self.internal.lock().unwrap().clone().extract_tx();
        let txid = tx.txid();
        txid.to_hex()
    }
}

impl Wallet {
    fn new(
        descriptor: String,
        change_descriptor: Option<String>,
        network: Network,
        database_config: DatabaseConfig,
    ) -> Result<Self, BdkError> {
        let any_database_config = match database_config {
            DatabaseConfig::Memory => AnyDatabaseConfig::Memory(()),
            DatabaseConfig::Sled { config } => AnyDatabaseConfig::Sled(config),
            DatabaseConfig::Sqlite { config } => AnyDatabaseConfig::Sqlite(config),
        };
        let database = AnyDatabase::from_config(&any_database_config)?;
        let wallet_mutex = Mutex::new(BdkWallet::new(
            &descriptor,
            change_descriptor.as_ref(),
            network,
            database,
        )?);
        Ok(Wallet { wallet_mutex })
    }

    fn get_wallet(&self) -> MutexGuard<BdkWallet<AnyDatabase>> {
        self.wallet_mutex.lock().expect("wallet")
    }

    fn get_network(&self) -> Network {
        self.get_wallet().network()
    }

    fn sync(
        &self,
        blockchain: &Blockchain,
        progress: Option<Box<dyn Progress>>,
    ) -> Result<(), BdkError> {
        let bdk_sync_opts = BdkSyncOptions {
            progress: progress.map(|p| {
                Box::new(ProgressHolder { progress: p })
                    as Box<(dyn bdk::blockchain::Progress + 'static)>
            }),
        };

        let blockchain = blockchain.get_blockchain();
        self.get_wallet().sync(blockchain.deref(), bdk_sync_opts)
    }

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

    fn sign(&self, psbt: &PartiallySignedBitcoinTransaction) -> Result<(), Error> {
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
        Ok(transactions.iter().map(Transaction::from).collect())
    }
}

pub struct ExtendedKeyInfo {
    mnemonic: String,
    xprv: String,
    fingerprint: String,
}

fn generate_extended_key(
    network: Network,
    word_count: WordCount,
    password: Option<String>,
) -> Result<ExtendedKeyInfo, Error> {
    let mnemonic: GeneratedKey<_, BareCtx> =
        Mnemonic::generate((word_count, Language::English)).unwrap();
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
    let mnemonic = Mnemonic::parse_in(Language::English, mnemonic).unwrap();
    let xkey: ExtendedKey = (mnemonic.clone(), password).into_extended_key()?;
    let xprv = xkey.into_xprv(network).unwrap();
    let fingerprint = xprv.fingerprint(&Secp256k1::new());
    Ok(ExtendedKeyInfo {
        mnemonic: mnemonic.to_string(),
        xprv: xprv.to_string(),
        fingerprint: fingerprint.to_string(),
    })
}

fn to_script_pubkey(address: &str) -> Result<Script, BdkError> {
    Address::from_str(address)
        .map(|x| x.script_pubkey())
        .map_err(|e| BdkError::Generic(e.to_string()))
}

#[derive(Clone)]
enum RbfValue {
    Default,
    Value(u32),
}

struct TxBuilder {
    recipients: Vec<(String, u64)>,
    fee_rate: Option<f32>,
    drain_wallet: bool,
    drain_to: Option<String>,
    rbf: Option<RbfValue>,
}

impl TxBuilder {
    fn new() -> Self {
        TxBuilder {
            recipients: Vec::new(),
            fee_rate: None,
            drain_wallet: false,
            drain_to: None,
            rbf: None,
        }
    }

    fn add_recipient(&self, recipient: String, amount: u64) -> Arc<Self> {
        let mut recipients = self.recipients.to_vec();
        recipients.append(&mut vec![(recipient, amount)]);
        Arc::new(TxBuilder {
            recipients,
            fee_rate: self.fee_rate,
            drain_wallet: self.drain_wallet,
            drain_to: self.drain_to.clone(),
            rbf: self.rbf.clone(),
        })
    }

    fn fee_rate(&self, sat_per_vb: f32) -> Arc<Self> {
        Arc::new(TxBuilder {
            recipients: self.recipients.to_vec(),
            fee_rate: Some(sat_per_vb),
            drain_wallet: self.drain_wallet,
            drain_to: self.drain_to.clone(),
            rbf: self.rbf.clone(),
        })
    }

    fn drain_wallet(&self) -> Arc<Self> {
        Arc::new(TxBuilder {
            recipients: self.recipients.to_vec(),
            fee_rate: self.fee_rate,
            drain_wallet: true,
            drain_to: self.drain_to.clone(),
            rbf: self.rbf.clone(),
        })
    }

    fn drain_to(&self, address: String) -> Arc<Self> {
        Arc::new(TxBuilder {
            recipients: self.recipients.to_vec(),
            fee_rate: self.fee_rate,
            drain_wallet: self.drain_wallet,
            drain_to: Some(address),
            rbf: self.rbf.clone(),
        })
    }

    fn enable_rbf(&self) -> Arc<Self> {
        Arc::new(TxBuilder {
            recipients: self.recipients.to_vec(),
            fee_rate: self.fee_rate,
            drain_wallet: self.drain_wallet,
            drain_to: self.drain_to.clone(),
            rbf: Some(RbfValue::Default),
        })
    }

    fn enable_rbf_with_sequence(&self, nsequence: u32) -> Arc<Self> {
        Arc::new(TxBuilder {
            recipients: self.recipients.to_vec(),
            fee_rate: self.fee_rate,
            drain_wallet: self.drain_wallet,
            drain_to: self.drain_to.clone(),
            rbf: Some(RbfValue::Value(nsequence)),
        })
    }

    fn build(&self, wallet: &Wallet) -> Result<Arc<PartiallySignedBitcoinTransaction>, Error> {
        let wallet = wallet.get_wallet();
        let mut tx_builder = wallet.build_tx();
        for (address, amount) in &self.recipients {
            tx_builder.add_recipient(to_script_pubkey(address)?, *amount);
        }
        if let Some(sat_per_vb) = self.fee_rate {
            tx_builder.fee_rate(FeeRate::from_sat_per_vb(sat_per_vb));
        }
        if self.drain_wallet {
            tx_builder.drain_wallet();
        }
        if let Some(address) = &self.drain_to {
            tx_builder.drain_to(to_script_pubkey(address)?);
        }
        if let Some(rbf) = &self.rbf {
            match *rbf {
                RbfValue::Default => {
                    tx_builder.enable_rbf();
                }
                RbfValue::Value(nsequence) => {
                    tx_builder.enable_rbf_with_sequence(nsequence);
                }
            }
        }
        tx_builder
            .finish()
            .map(|(psbt, _)| PartiallySignedBitcoinTransaction {
                internal: Mutex::new(psbt),
            })
            .map(Arc::new)
    }
}

struct BumpFeeTxBuilder {
    txid: String,
    fee_rate: f32,
    rbf: Option<RbfValue>,
    allow_shrinking: Option<String>,
}

impl BumpFeeTxBuilder {
    fn new(txid: String, fee_rate: f32) -> Self {
        Self {
            txid,
            fee_rate,
            allow_shrinking: None,
            rbf: None,
        }
    }

    fn allow_shrinking(&self, address: String) -> Arc<Self> {
        Arc::new(Self {
            txid: self.txid.clone(),
            fee_rate: self.fee_rate,
            allow_shrinking: Some(address),
            rbf: self.rbf.clone(),
        })
    }

    fn enable_rbf(&self) -> Arc<Self> {
        Arc::new(Self {
            txid: self.txid.clone(),
            fee_rate: self.fee_rate,
            rbf: Some(RbfValue::Default),
            allow_shrinking: self.allow_shrinking.clone(),
        })
    }

    fn enable_rbf_with_sequence(&self, nsequence: u32) -> Arc<Self> {
        Arc::new(Self {
            txid: self.txid.clone(),
            fee_rate: self.fee_rate,
            rbf: Some(RbfValue::Value(nsequence)),
            allow_shrinking: self.allow_shrinking.clone(),
        })
    }

    fn build(&self, wallet: &Wallet) -> Result<Arc<PartiallySignedBitcoinTransaction>, Error> {
        let wallet = wallet.get_wallet();
        let txid = Txid::from_str(self.txid.as_str())?;
        let mut tx_builder = wallet.build_fee_bump(txid)?;
        tx_builder.fee_rate(FeeRate::from_sat_per_vb(self.fee_rate));
        if let Some(rbf) = &self.rbf {
            match *rbf {
                RbfValue::Default => {
                    tx_builder.enable_rbf();
                }
                RbfValue::Value(nsequence) => {
                    tx_builder.enable_rbf_with_sequence(nsequence);
                }
            }
        }
        if let Some(allow_shrinking) = &self.allow_shrinking {
            let address =
                Address::from_str(allow_shrinking).map_err(|e| Error::Generic(e.to_string()))?;
            let script = address.script_pubkey();
            tx_builder.allow_shrinking(script)?;
        }
        tx_builder
            .finish()
            .map(|(psbt, _)| PartiallySignedBitcoinTransaction {
                internal: Mutex::new(psbt),
            })
            .map(Arc::new)
    }
}

uniffi::deps::static_assertions::assert_impl_all!(Wallet: Sync, Send);
