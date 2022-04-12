use bdk::bitcoin::hashes::hex::ToHex;
use bdk::bitcoin::secp256k1::{All, Message, Secp256k1, SecretKey};
use bdk::bitcoin::util::psbt::PartiallySignedTransaction;
use bdk::bitcoin::{Address, Network, PrivateKey, PublicKey, Script};
use bdk::blockchain::any::{AnyBlockchain, AnyBlockchainConfig};
use bdk::blockchain::Progress;
use bdk::blockchain::{
    electrum::ElectrumBlockchainConfig, esplora::EsploraBlockchainConfig, ConfigurableBlockchain,
};
use bdk::database::any::{AnyDatabase, SledDbConfiguration, SqliteDbConfiguration};
use bdk::database::{AnyDatabaseConfig, ConfigurableDatabase};
use bdk::keys::bip39::{Language, Mnemonic, WordCount};
use bdk::keys::{DerivableKey, ExtendedKey, GeneratableKey, GeneratedKey};
use bdk::miniscript::BareCtx;
use bdk::wallet::AddressIndex;
use bdk::{BlockTime, Error, FeeRate, SignOptions, Wallet as BdkWallet};
use std::convert::TryFrom;
use std::str::FromStr;
use std::sync::{Arc, Mutex, MutexGuard};
use bdk::bitcoin::hashes::Hash;
use bdk::bitcoin::util::bip32::{ExtendedPrivKey, ExtendedPubKey};
use bdk::bitcoin::util::misc::{MessageSignature, signed_msg_hash};

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
    pub timeout_read: u64,
    pub timeout_write: u64,
    pub stop_gap: u64,
}

pub enum BlockchainConfig {
    Electrum { config: ElectrumConfig },
    Esplora { config: EsploraConfig },
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TransactionDetails {
    pub fees: Option<u64>,
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
            fees: x.fee,
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

struct Wallet {
    wallet_mutex: Mutex<BdkWallet<AnyBlockchain, AnyDatabase>>,
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
}

impl Wallet {
    fn new(
        descriptor: String,
        change_descriptor: Option<String>,
        network: Network,
        database_config: DatabaseConfig,
        blockchain_config: BlockchainConfig,
    ) -> Result<Self, BdkError> {
        let any_database_config = match database_config {
            DatabaseConfig::Memory => AnyDatabaseConfig::Memory(()),
            DatabaseConfig::Sled { config } => AnyDatabaseConfig::Sled(config),
            DatabaseConfig::Sqlite { config } => AnyDatabaseConfig::Sqlite(config),
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
        let wallet_mutex = Mutex::new(BdkWallet::new(
            &descriptor,
            change_descriptor.as_ref(),
            network,
            database,
            blockchain,
        )?);
        Ok(Wallet { wallet_mutex })
    }

    fn get_wallet(&self) -> MutexGuard<BdkWallet<AnyBlockchain, AnyDatabase>> {
        self.wallet_mutex.lock().expect("wallet")
    }

    fn get_network(&self) -> Network {
        self.get_wallet().network()
    }

    fn sync(
        &self,
        progress_update: Box<dyn BdkProgress>,
        max_address_param: Option<u32>,
    ) -> Result<(), BdkError> {
        self.get_wallet()
            .sync(BdkProgressHolder { progress_update }, max_address_param)
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

    fn broadcast(&self, psbt: &PartiallySignedBitcoinTransaction) -> Result<String, Error> {
        let tx = psbt.internal.lock().unwrap().clone().extract_tx();
        let txid = self.get_wallet().broadcast(&tx)?;
        Ok(txid.to_hex())
    }
}

pub struct ExtendedKeyInfo {
    mnemonic: String,
    xprv: String,
    xpub: String,
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
    let secp = Secp256k1::new();
    let xpub = ExtendedPubKey::from_private(&secp, &xprv);
    let fingerprint = xprv.fingerprint(&secp);
    Ok(ExtendedKeyInfo {
        mnemonic: mnemonic.to_string(),
        xprv: xprv.to_string(),
        xpub: xpub.to_string(),
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
    let secp = Secp256k1::new();
    let xpub = ExtendedPubKey::from_private(&secp, &xprv);
    let fingerprint = xprv.fingerprint(&secp);
    Ok(ExtendedKeyInfo {
        mnemonic: mnemonic.to_string(),
        xprv: xprv.to_string(),
        xpub: xpub.to_string(),
        fingerprint: fingerprint.to_string(),
    })
}

fn to_script_pubkey(address: &str) -> Result<Script, BdkError> {
    Address::from_str(address)
        .map(|x| x.script_pubkey())
        .map_err(|e| BdkError::Generic(e.to_string()))
}

struct TxBuilder {
    recipients: Vec<(String, u64)>,
    fee_rate: Option<f32>,
    drain_wallet: bool,
    drain_to: Option<String>,
}

impl TxBuilder {
    fn new() -> Self {
        TxBuilder {
            recipients: Vec::new(),
            fee_rate: None,
            drain_wallet: false,
            drain_to: None,
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
        })
    }

    fn fee_rate(&self, sat_per_vb: f32) -> Arc<Self> {
        Arc::new(TxBuilder {
            recipients: self.recipients.to_vec(),
            fee_rate: Some(sat_per_vb),
            drain_wallet: self.drain_wallet,
            drain_to: self.drain_to.clone(),
        })
    }

    fn drain_wallet(&self) -> Arc<Self> {
        Arc::new(TxBuilder {
            recipients: self.recipients.to_vec(),
            fee_rate: self.fee_rate,
            drain_wallet: true,
            drain_to: self.drain_to.clone(),
        })
    }

    fn drain_to(&self, address: String) -> Arc<Self> {
        Arc::new(TxBuilder {
            recipients: self.recipients.to_vec(),
            fee_rate: self.fee_rate,
            drain_wallet: self.drain_wallet,
            drain_to: Some(address),
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
        tx_builder
            .finish()
            .map(|(psbt, _)| PartiallySignedBitcoinTransaction {
                internal: Mutex::new(psbt),
            })
            .map(Arc::new)
    }
}

pub trait MessageSigner {
    fn sign(&self, msg: String) -> String;
}

pub struct EcdsaMessageSigner {
    secp: Secp256k1<All>,
    secret_key: SecretKey
}

impl EcdsaMessageSigner {

    pub fn from_wif(wif: String) -> Result<Self, Error> {
        let prv = match PrivateKey::from_wif(wif.as_str()) {
            Ok(prv ) => prv,
            Err(e) => return Err(Error::Generic(e.to_string()))
        };

        EcdsaMessageSigner::from_secret_key(prv.key)
    }

    pub fn from_prv(str: String) -> Result<Self, Error> {
        let prv = SecretKey::from_str(str.as_str()).unwrap();
        EcdsaMessageSigner::from_secret_key(prv)
    }

    pub fn from_xprv(str: String) -> Result<Self, Error> {
        let xprv = ExtendedPrivKey::from_str(str.as_str()).unwrap();
        EcdsaMessageSigner::from_secret_key(xprv.private_key.key)
    }

    fn from_secret_key(secret_key: SecretKey) -> Result<Self, Error> {
        Ok(EcdsaMessageSigner {
            secret_key,
            secp: Secp256k1::new(),
        })
    }
}

impl MessageSigner for EcdsaMessageSigner {

    fn sign(&self, msg: String) -> String {
        let msg_hash = signed_msg_hash(msg.as_str());
        let sig = self.secp.sign_recoverable(
            &Message::from_slice(&msg_hash.into_inner()[..]).unwrap(),
            &self.secret_key,
        );
        MessageSignature::new(sig, false).to_base64()
    }
}

pub trait MessageSignatureVerifier {
    fn verify(&self, sig: String, msg: String) -> bool;
}

pub struct EcdsaMessageSignatureVerifier {
    secp: Secp256k1<All>,
    address: Address,
}

impl EcdsaMessageSignatureVerifier {

    pub fn from_address(address: String) -> Result<Self, Error> {
        let address = Address::from_str(address.as_str()).unwrap();
        EcdsaMessageSignatureVerifier::from_addr_struct(address)
    }

    pub fn from_pub(str: String) -> Result<Self, Error> {
        let public_key = PublicKey::from_str(str.as_str()).unwrap();
        let address = Address::p2pkh(&public_key, Network::Bitcoin);
        EcdsaMessageSignatureVerifier::from_addr_struct(address)
    }

    pub fn from_xpub(str: String) -> Result<Self, Error> {
        let public_key = ExtendedPubKey::from_str(str.as_str()).unwrap();
        EcdsaMessageSignatureVerifier::from_pub(public_key.public_key.to_string())
    }

    fn from_addr_struct(address: Address) -> Result<Self, Error> {
        Ok(EcdsaMessageSignatureVerifier {
            address,
            secp: Secp256k1::new(),
        })
    }
}

impl MessageSignatureVerifier for EcdsaMessageSignatureVerifier {

    fn verify(&self, sig: String, msg: String) -> bool {
        let msg_sig = MessageSignature::from_base64(sig.as_str());
        let pub_key = match msg_sig {
            Ok(sig) => sig.recover_pubkey(&self.secp, signed_msg_hash(msg.as_str())),
            Err(_) => return false
        };
        let compressed_pub_key = match pub_key {
            Ok(pub_key) => PublicKey::from_str(&pub_key.key.to_string()),
            Err(_) => return false
        };
        let address = match compressed_pub_key {
            Ok(pub_key) => Address::p2pkh(&pub_key, self.address.network),
            Err(_) => return false
        };
        address == self.address
    }
}

uniffi::deps::static_assertions::assert_impl_all!(Wallet: Sync, Send);
