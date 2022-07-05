use bdk::bitcoin::hashes::hex::ToHex;
use bdk::bitcoin::secp256k1::Secp256k1;
use bdk::bitcoin::util::bip32::{DerivationPath as BdkDerivationPath, KeySource};
use bdk::bitcoin::util::psbt::PartiallySignedTransaction;
use bdk::bitcoin::{Address, Network, Script, Txid};
use bdk::blockchain::any::{AnyBlockchain, AnyBlockchainConfig};
use bdk::blockchain::{
    electrum::ElectrumBlockchainConfig, esplora::EsploraBlockchainConfig, ConfigurableBlockchain,
};
use bdk::blockchain::{Blockchain as BdkBlockchain, Progress as BdkProgress};
use bdk::database::any::{AnyDatabase, SledDbConfiguration, SqliteDbConfiguration};
use bdk::database::{AnyDatabaseConfig, ConfigurableDatabase};
use bdk::descriptor::Legacy;
use bdk::keys::bip39::{Language, Mnemonic, WordCount};
use bdk::keys::{
    DerivableKey, DescriptorKey as BdkDescriptorKey, DescriptorPublicKey, DescriptorSecretKey,
    ExtendedKey, GeneratableKey, GeneratedKey,
};
use bdk::miniscript::BareCtx;
use bdk::wallet::AddressIndex as BdkAddressIndex;
use bdk::wallet::AddressInfo as BdkAddressInfo;
use bdk::{
    BlockTime, Error, FeeRate, KeychainKind, SignOptions, SyncOptions as BdkSyncOptions,
    Wallet as BdkWallet,
};
use std::convert::{From, TryFrom};
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;
use std::sync::{Arc, Mutex, MutexGuard};

uniffi_macros::include_scaffolding!("bdk");

type BdkError = Error;

pub struct AddressInfo {
    pub index: u32,
    pub address: String,
}

impl From<BdkAddressInfo> for AddressInfo {
    fn from(x: bdk::wallet::AddressInfo) -> AddressInfo {
        AddressInfo {
            index: x.index,
            address: x.address.to_string(),
        }
    }
}

pub enum AddressIndex {
    New,
    LastUnused,
}

impl From<AddressIndex> for BdkAddressIndex {
    fn from(x: AddressIndex) -> BdkAddressIndex {
        match x {
            AddressIndex::New => BdkAddressIndex::New,
            AddressIndex::LastUnused => BdkAddressIndex::LastUnused,
        }
    }
}

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

pub struct OutPoint {
    txid: String,
    vout: u32,
}

pub struct TxOut {
    value: u64,
    address: String,
}

pub struct LocalUtxo {
    outpoint: OutPoint,
    txout: TxOut,
    keychain: KeychainKind,
    is_spent: bool,
}

// This trait is used to convert the bdk TxOut type with field `script_pubkey: Script`
// into the bdk-ffi TxOut type which has a field `address: String` instead
trait NetworkLocalUtxo {
    fn from_utxo(x: &bdk::LocalUtxo, network: Network) -> LocalUtxo;
}

impl NetworkLocalUtxo for LocalUtxo {
    fn from_utxo(x: &bdk::LocalUtxo, network: Network) -> LocalUtxo {
        LocalUtxo {
            outpoint: OutPoint {
                txid: x.outpoint.txid.to_string(),
                vout: x.outpoint.vout,
            },
            txout: TxOut {
                value: x.txout.value,
                address: bdk::bitcoin::util::address::Address::from_script(
                    &x.txout.script_pubkey,
                    network,
                )
                .unwrap()
                .to_string(),
            },
            keychain: x.keychain,
            is_spent: x.is_spent,
        }
    }
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

    fn get_address(&self, address_index: AddressIndex) -> Result<AddressInfo, BdkError> {
        self.get_wallet()
            .get_address(address_index.into())
            .map(AddressInfo::from)
    }

    fn get_balance(&self) -> Result<u64, Error> {
        self.get_wallet().get_balance()
    }

    fn sign(&self, psbt: &PartiallySignedBitcoinTransaction) -> Result<bool, Error> {
        let mut psbt = psbt.internal.lock().unwrap();
        self.get_wallet().sign(&mut psbt, SignOptions::default())
    }

    fn get_transactions(&self) -> Result<Vec<Transaction>, Error> {
        let transactions = self.get_wallet().list_transactions(true)?;
        Ok(transactions.iter().map(Transaction::from).collect())
    }

    fn list_unspent(&self) -> Result<Vec<LocalUtxo>, Error> {
        let unspents = self.get_wallet().list_unspent()?;
        Ok(unspents
            .iter()
            .map(|u| LocalUtxo::from_utxo(u, self.get_network()))
            .collect())
    }
}

fn generate_mnemonic(word_count: WordCount) -> Result<String, BdkError> {
    let mnemonic: GeneratedKey<_, BareCtx> =
        Mnemonic::generate((word_count, Language::English)).unwrap();
    Ok(mnemonic.to_string())
}

struct DerivationPath {
    derivation_path_mutex: Mutex<BdkDerivationPath>,
}

impl DerivationPath {
    fn new(path: String) -> Result<Self, BdkError> {
        BdkDerivationPath::from_str(&path)
            .map(|x| DerivationPath {
                derivation_path_mutex: Mutex::new(x),
            })
            .map_err(|e| BdkError::Generic(e.to_string()))
    }
}

struct DescriptorKey {
    descriptor_key_mutex: Mutex<BdkDescriptorKey<Legacy>>,
}

impl DescriptorKey {
    fn new(network: Network, mnemonic: String, password: Option<String>) -> Result<Self, BdkError> {
        let mnemonic = Mnemonic::parse_in(Language::English, mnemonic)
            .map_err(|e| BdkError::Generic(e.to_string()))?;
        let xkey: ExtendedKey = (mnemonic.clone(), password).into_extended_key()?;
        let descriptor_key = xkey
            .into_xprv(network)
            .unwrap()
            .into_descriptor_key(None, BdkDerivationPath::master())?;
        Ok(Self {
            descriptor_key_mutex: Mutex::new(descriptor_key),
        })
    }

    fn derive(
        &self,
        origin_path: Arc<DerivationPath>,
        derivation_path: Option<Arc<DerivationPath>>,
    ) -> Arc<DescriptorKey> {
        let secp = Secp256k1::new();
        let root_key = self.descriptor_key_mutex.lock().unwrap();
        let root_path = origin_path
            .derivation_path_mutex
            .lock()
            .unwrap()
            .deref()
            .clone();
        let path = derivation_path
            .map(|dp| dp.derivation_path_mutex.lock().unwrap().deref().clone())
            .unwrap_or_default();
        match root_key.deref() {
            BdkDescriptorKey::Public(DescriptorPublicKey::XPub(xpub), _valid_networks, _) => {
                let key_source: KeySource = (xpub.xkey.fingerprint(), root_path.clone());
                let derived_xpub = xpub.xkey.derive_pub(&secp, &root_path).unwrap().clone();
                let derived_descriptor_key = derived_xpub
                    .into_descriptor_key(Some(key_source), path)
                    .unwrap();
                Arc::new(DescriptorKey {
                    descriptor_key_mutex: Mutex::new(derived_descriptor_key),
                })
            }
            BdkDescriptorKey::Secret(DescriptorSecretKey::XPrv(xprv), _valid_networks, _) => {
                let key_source: KeySource = (xprv.xkey.fingerprint(&secp), root_path.clone());
                let derived_xprv = xprv.xkey.derive_priv(&secp, &root_path).unwrap();
                let derived_descriptor_key = derived_xprv
                    .into_descriptor_key(Some(key_source), path)
                    .unwrap();
                Arc::new(DescriptorKey {
                    descriptor_key_mutex: Mutex::new(derived_descriptor_key),
                })
            }
            // This case should never happen since we only create xkeys in the new() function
            // in the future if we decide to also support SinglePriv and SinglePub keys then
            // those types will need to be handled here.
            _ => panic!(),
        }
    }

    fn as_public(&self) -> Arc<DescriptorKey> {
        let secp = Secp256k1::new();
        let root_key = self.descriptor_key_mutex.lock().unwrap();

        match root_key.deref() {
            BdkDescriptorKey::Public(descriptor_public_key, valid_networks, _) => {
                Arc::new(DescriptorKey {
                    descriptor_key_mutex: Mutex::new(BdkDescriptorKey::from_public(
                        descriptor_public_key.clone(),
                        valid_networks.clone(),
                    )),
                })
            }
            BdkDescriptorKey::Secret(descriptor_secret_key, valid_networks, _) => {
                let descriptor_public_key = descriptor_secret_key.as_public(&secp).unwrap();
                Arc::new(DescriptorKey {
                    descriptor_key_mutex: Mutex::new(BdkDescriptorKey::from_public(
                        descriptor_public_key.clone(),
                        valid_networks.clone(),
                    )),
                })
            }
        }
    }

    fn as_string(&self) -> String {
        let descriptor_key = self.descriptor_key_mutex.lock().unwrap();
        match descriptor_key.deref() {
            BdkDescriptorKey::Public(descriptor_public_key, _valid_networks, _) => {
                descriptor_public_key.to_string()
            }
            BdkDescriptorKey::Secret(descriptor_secret_key, _valid_networks, _) => {
                descriptor_secret_key.to_string()
            }
        }
    }
}

fn to_script_pubkey(address: &str) -> Result<Script, BdkError> {
    Address::from_str(address)
        .map(|x| x.script_pubkey())
        .map_err(|e| BdkError::Generic(e.to_string()))
}

#[derive(Clone, Debug)]
enum RbfValue {
    Default,
    Value(u32),
}

#[derive(Clone, Debug)]
struct TxBuilder {
    recipients: Vec<(String, u64)>,
    fee_rate: Option<f32>,
    drain_wallet: bool,
    drain_to: Option<String>,
    rbf: Option<RbfValue>,
    data: Vec<u8>,
}

impl TxBuilder {
    fn new() -> Self {
        TxBuilder {
            recipients: Vec::new(),
            fee_rate: None,
            drain_wallet: false,
            drain_to: None,
            rbf: None,
            data: Vec::new(),
        }
    }

    fn add_recipient(&self, recipient: String, amount: u64) -> Arc<Self> {
        let mut recipients = self.recipients.to_vec();
        recipients.append(&mut vec![(recipient, amount)]);
        Arc::new(TxBuilder {
            recipients,
            ..self.clone()
        })
    }

    fn fee_rate(&self, sat_per_vb: f32) -> Arc<Self> {
        Arc::new(TxBuilder {
            fee_rate: Some(sat_per_vb),
            ..self.clone()
        })
    }

    fn drain_wallet(&self) -> Arc<Self> {
        Arc::new(TxBuilder {
            drain_wallet: true,
            ..self.clone()
        })
    }

    fn drain_to(&self, address: String) -> Arc<Self> {
        Arc::new(TxBuilder {
            drain_to: Some(address),
            ..self.clone()
        })
    }

    fn enable_rbf(&self) -> Arc<Self> {
        Arc::new(TxBuilder {
            rbf: Some(RbfValue::Default),
            ..self.clone()
        })
    }

    fn enable_rbf_with_sequence(&self, nsequence: u32) -> Arc<Self> {
        Arc::new(TxBuilder {
            rbf: Some(RbfValue::Value(nsequence)),
            ..self.clone()
        })
    }

    fn add_data(&self, data: Vec<u8>) -> Arc<Self> {
        Arc::new(TxBuilder {
            data,
            ..self.clone()
        })
    }

    fn finish(&self, wallet: &Wallet) -> Result<Arc<PartiallySignedBitcoinTransaction>, Error> {
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
        if !&self.data.is_empty() {
            tx_builder.add_data(&self.data.as_slice());
        }

        tx_builder
            .finish()
            .map(|(psbt, _)| PartiallySignedBitcoinTransaction {
                internal: Mutex::new(psbt),
            })
            .map(Arc::new)
    }
}

#[derive(Clone)]
struct BumpFeeTxBuilder {
    txid: String,
    fee_rate: f32,
    allow_shrinking: Option<String>,
    rbf: Option<RbfValue>,
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
            allow_shrinking: Some(address),
            ..self.clone()
        })
    }

    fn enable_rbf(&self) -> Arc<Self> {
        Arc::new(Self {
            rbf: Some(RbfValue::Default),
            ..self.clone()
        })
    }

    fn enable_rbf_with_sequence(&self, nsequence: u32) -> Arc<Self> {
        Arc::new(Self {
            rbf: Some(RbfValue::Value(nsequence)),
            ..self.clone()
        })
    }

    fn finish(&self, wallet: &Wallet) -> Result<Arc<PartiallySignedBitcoinTransaction>, Error> {
        let wallet = wallet.get_wallet();
        let txid = Txid::from_str(self.txid.as_str())?;
        let mut tx_builder = wallet.build_fee_bump(txid)?;
        tx_builder.fee_rate(FeeRate::from_sat_per_vb(self.fee_rate));
        if let Some(allow_shrinking) = &self.allow_shrinking {
            let address =
                Address::from_str(allow_shrinking).map_err(|e| Error::Generic(e.to_string()))?;
            let script = address.script_pubkey();
            tx_builder.allow_shrinking(script)?;
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

uniffi::deps::static_assertions::assert_impl_all!(Wallet: Sync, Send);

// The goal of these tests to to ensure `bdk-ffi` intermediate code correctly calls `bdk` APIs.
// These tests should not be used to verify `bdk` behavior that is already tested in the `bdk`
// crate.
#[cfg(test)]
mod test {
    use crate::{DerivationPath, DescriptorKey, TxBuilder, Wallet};
    use bdk::bitcoin::Network::Testnet;
    use bdk::bitcoin::{Address, Network};
    use bdk::wallet::get_funded_wallet;
    use std::str::FromStr;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_drain_wallet() {
        let test_wpkh = "wpkh(cVpPVruEDdmutPzisEsYvtST1usBR3ntr8pXSyt6D2YYqXRyPcFW)";
        let (funded_wallet, _, _) = get_funded_wallet(test_wpkh);
        let test_wallet = Wallet {
            wallet_mutex: Mutex::new(funded_wallet),
        };
        let drain_to_address = "tb1ql7w62elx9ucw4pj5lgw4l028hmuw80sndtntxt".to_string();
        let tx_builder = TxBuilder::new()
            .drain_wallet()
            .drain_to(drain_to_address.clone());
        //dbg!(&tx_builder);
        assert_eq!(tx_builder.drain_wallet, true);
        assert_eq!(tx_builder.drain_to, Some(drain_to_address));

        let psbt = tx_builder.finish(&test_wallet).unwrap();
        let psbt = psbt.internal.lock().unwrap().clone();

        // confirm one input with 50,000 sats
        assert_eq!(psbt.inputs.len(), 1);
        let input_value = psbt
            .inputs
            .get(0)
            .cloned()
            .unwrap()
            .non_witness_utxo
            .unwrap()
            .output
            .get(0)
            .unwrap()
            .value;
        assert_eq!(input_value, 50_000 as u64);

        // confirm one output to correct address with all sats - fee
        assert_eq!(psbt.outputs.len(), 1);
        let output_address = Address::from_script(
            &psbt
                .unsigned_tx
                .output
                .get(0)
                .cloned()
                .unwrap()
                .script_pubkey,
            Testnet,
        )
        .unwrap();
        assert_eq!(
            output_address,
            Address::from_str("tb1ql7w62elx9ucw4pj5lgw4l028hmuw80sndtntxt").unwrap()
        );
        let output_value = psbt.unsigned_tx.output.get(0).cloned().unwrap().value;
        assert_eq!(output_value, 49_890 as u64); // input - fee
    }

    fn get_descriptor_key() -> DescriptorKey {
        let mnemonic =
            "age nut kind clerk ceiling pony bright shrug identify rhythm blur topple".to_string();
        DescriptorKey::new(Network::Testnet, mnemonic.clone(), None).unwrap()
    }

    #[test]
    fn test_generate_descriptor_key() {
        let descriptor_key = get_descriptor_key();
        assert_eq!(descriptor_key.into_string(), "tprv8ZgxMBicQKsPf2XLuqBBi69Cpzf9mArye5uHMpFpd776atK6nyS3qkXutKmmsdXE2edbN6uqGkD1qsUNz2TRdFFnpVEt7HGKmiRWswJFbKZ/*");
        assert_eq!(descriptor_key.as_public().into_string(), "tpubD6NzVbkrYhZ4YVZ8oUqn7VoKQ2B5vW3tDPW4eLJ83NuVRNZsRNFe2F9n4U66rRwjkRyzLtRiARTo2dV5ucHU4arnXhGYr2Pxqk2bWZZ7aru/*");
    }

    #[test]
    fn test_derive_child_descriptor_key() {
        let descriptor_key_m = get_descriptor_key();

        // deriving DescriptorKey(Private) "m/0" from DescriptorKey(Private) "m"
        let origin_path = DerivationPath::new("m".to_string()).unwrap();
        let derivation_path = DerivationPath::new("m/0".to_string()).unwrap();
        let derived_descriptor_key_m_0 =
            descriptor_key_m.derive(Arc::new(origin_path), Some(Arc::new(derivation_path)));
        assert_eq!(derived_descriptor_key_m_0.into_string(), "[19294114]tprv8ZgxMBicQKsPf2XLuqBBi69Cpzf9mArye5uHMpFpd776atK6nyS3qkXutKmmsdXE2edbN6uqGkD1qsUNz2TRdFFnpVEt7HGKmiRWswJFbKZ/0/*");
        assert_eq!(derived_descriptor_key_m_0.as_public().into_string(), "[19294114]tpubD6NzVbkrYhZ4YVZ8oUqn7VoKQ2B5vW3tDPW4eLJ83NuVRNZsRNFe2F9n4U66rRwjkRyzLtRiARTo2dV5ucHU4arnXhGYr2Pxqk2bWZZ7aru/0/*");

        // deriving DescriptorKey(Private) "m/0/0" from DescriptorKey(Private) "m/0"
        let origin_path = DerivationPath::new("m/0".to_string()).unwrap();
        let derivation_path = DerivationPath::new("m/0/0".to_string()).unwrap();
        let derived_descriptor_key_m_0_0 = derived_descriptor_key_m_0
            .derive(Arc::new(origin_path), Some(Arc::new(derivation_path)));
        assert_eq!(derived_descriptor_key_m_0_0.into_string(), "[19294114/0]tprv8bkpLnjPG4v3WoQfwPLY5au3Q5sanjLMCsudSf2sdVH4snDFcJB3jkQLHaaKZ3HA4TJr4ZZFcZg4NGKdkou78w5SZULyAcS85mJp7dNgStf/0/0/*");
        assert_eq!(derived_descriptor_key_m_0_0.as_public().into_string(), "[19294114/0]tpubD8SrVCmdQSbiQGSTq318UzZ9y7PWx4XFnBWQjB5B3m5TiGU2EgzdvF2CThNXX9CDoY9WgftVhnFD2Hw5d9a1MD4Kg3F4npob3tXDxtAfvD4/0/0/*");
        
        // deriving DescriptorKey(Private) "m/0/0" from DescriptorKey(Private) "m"
        let origin_path = DerivationPath::new("m".to_string()).unwrap();
        let derivation_path = DerivationPath::new("m/0/0".to_string()).unwrap();
        let derived_descriptor_key_m_0_0 = descriptor_key_m
            .derive(Arc::new(origin_path), Some(Arc::new(derivation_path)));
        assert_eq!(derived_descriptor_key_m_0_0.into_string(), "[19294114/0]tprv8bkpLnjPG4v3WoQfwPLY5au3Q5sanjLMCsudSf2sdVH4snDFcJB3jkQLHaaKZ3HA4TJr4ZZFcZg4NGKdkou78w5SZULyAcS85mJp7dNgStf/0/0/*");
        assert_eq!(derived_descriptor_key_m_0_0.as_public().into_string(), "[19294114/0]tpubD8SrVCmdQSbiQGSTq318UzZ9y7PWx4XFnBWQjB5B3m5TiGU2EgzdvF2CThNXX9CDoY9WgftVhnFD2Hw5d9a1MD4Kg3F4npob3tXDxtAfvD4/0/0/*");
    }
}
