use bdk::bitcoin::blockdata::script::Script as BdkScript;
use bdk::bitcoin::hashes::hex::ToHex;
use bdk::bitcoin::secp256k1::Secp256k1;
use bdk::bitcoin::util::bip32::{DerivationPath as BdkDerivationPath, Fingerprint};
use bdk::bitcoin::util::psbt::serialize::Serialize;
use bdk::bitcoin::util::psbt::PartiallySignedTransaction as BdkPartiallySignedTransaction;
use bdk::bitcoin::Sequence;
use bdk::bitcoin::{Address as BdkAddress, Network, OutPoint as BdkOutPoint, Txid};
use bdk::blockchain::any::{AnyBlockchain, AnyBlockchainConfig};
use bdk::blockchain::rpc::Auth as BdkAuth;
use bdk::blockchain::GetBlockHash;
use bdk::blockchain::GetHeight;
use bdk::blockchain::{
    electrum::ElectrumBlockchainConfig, esplora::EsploraBlockchainConfig,
    rpc::RpcConfig as BdkRpcConfig, rpc::RpcSyncParams as BdkRpcSyncParams, ConfigurableBlockchain,
};
use bdk::blockchain::{Blockchain as BdkBlockchain, Progress as BdkProgress};
use bdk::database::any::{AnyDatabase, SledDbConfiguration, SqliteDbConfiguration};
use bdk::database::{AnyDatabaseConfig, ConfigurableDatabase};
use bdk::descriptor::{DescriptorXKey, IntoWalletDescriptor};
use bdk::keys::bip39::{Language, Mnemonic as BdkMnemonic, WordCount};
use bdk::keys::{
    DerivableKey, DescriptorPublicKey as BdkDescriptorPublicKey,
    DescriptorSecretKey as BdkDescriptorSecretKey, ExtendedKey, GeneratableKey, GeneratedKey,
};
use bdk::miniscript::BareCtx;
use bdk::psbt::PsbtUtils;
use bdk::template::{Bip44, Bip44Public, Bip49, Bip49Public, Bip84, Bip84Public, DescriptorTemplate, DescriptorTemplateOut};
use bdk::wallet::tx_builder::ChangeSpendPolicy;
use bdk::wallet::AddressIndex as BdkAddressIndex;
use bdk::wallet::AddressInfo as BdkAddressInfo;
use bdk::{
    Balance as BdkBalance, BlockTime, Error as BdkError, FeeRate, KeychainKind, SignOptions,
    SyncOptions as BdkSyncOptions, Wallet as BdkWallet,
};
use std::collections::HashSet;
use std::convert::{From, TryFrom};
use std::fmt;
use std::ops::Deref;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::{Arc, Mutex, MutexGuard};

uniffi_macros::include_scaffolding!("bdk");

/// A output script and an amount of satoshis.
pub struct ScriptAmount {
    pub script: Arc<Script>,
    pub amount: u64,
}

/// A derived address and the index it was found at.
pub struct AddressInfo {
    /// Child index of this address
    pub index: u32,
    /// Address
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

/// The address index selection strategy to use to derived an address from the wallet's external
/// descriptor.
pub enum AddressIndex {
    /// Return a new address after incrementing the current descriptor index.
    New,
    /// Return the address for the current descriptor index if it has not been used in a received
    /// transaction. Otherwise return a new address as with AddressIndex::New.
    /// Use with caution, if the wallet has not yet detected an address has been used it could
    /// return an already used address. This function is primarily meant for situations where the
    /// caller is untrusted; for example when deriving donation addresses on-demand for a public
    /// web page.
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

/// Type that can contain any of the database configurations defined by the library
/// This allows storing a single configuration that can be loaded into an AnyDatabaseConfig
/// instance. Wallets that plan to offer users the ability to switch blockchain backend at runtime
/// will find this particularly useful.
pub enum DatabaseConfig {
    /// Memory database has no config
    Memory,
    /// Simple key-value embedded database based on sled
    Sled { config: SledDbConfiguration },
    /// Sqlite embedded database using rusqlite
    Sqlite { config: SqliteDbConfiguration },
}

/// Configuration for an ElectrumBlockchain
pub struct ElectrumConfig {
    /// URL of the Electrum server (such as ElectrumX, Esplora, BWT) may start with ssl:// or tcp:// and include a port
    /// e.g. ssl://electrum.blockstream.info:60002
    pub url: String,
    /// URL of the socks5 proxy server or a Tor service
    pub socks5: Option<String>,
    /// Request retry count
    pub retry: u8,
    /// Request timeout (seconds)
    pub timeout: Option<u8>,
    /// Stop searching addresses for transactions after finding an unused gap of this length
    pub stop_gap: u64,
}

/// Configuration for an EsploraBlockchain
pub struct EsploraConfig {
    /// Base URL of the esplora service
    /// e.g. https://blockstream.info/api/
    pub base_url: String,
    /// Optional URL of the proxy to use to make requests to the Esplora server
    /// The string should be formatted as: <protocol>://<user>:<password>@host:<port>.
    /// Note that the format of this value and the supported protocols change slightly between the
    /// sync version of esplora (using ureq) and the async version (using reqwest). For more
    /// details check with the documentation of the two crates. Both of them are compiled with
    /// the socks feature enabled.
    /// The proxy is ignored when targeting wasm32.
    pub proxy: Option<String>,
    /// Number of parallel requests sent to the esplora service (default: 4)
    pub concurrency: Option<u8>,
    /// Stop searching addresses for transactions after finding an unused gap of this length.
    pub stop_gap: u64,
    /// Socket timeout.
    pub timeout: Option<u64>,
}

pub enum Auth {
    /// No authentication
    None,
    /// Authentication with username and password, usually [Auth::Cookie] should be preferred
    UserPass {
        /// Username
        username: String,
        /// Password
        password: String,
    },
    /// Authentication with a cookie file
    Cookie {
        /// Cookie file
        file: String,
    },
}

impl From<Auth> for BdkAuth {
    fn from(auth: Auth) -> Self {
        match auth {
            Auth::None => BdkAuth::None,
            Auth::UserPass { username, password } => BdkAuth::UserPass { username, password },
            Auth::Cookie { file } => BdkAuth::Cookie {
                file: PathBuf::from(file),
            },
        }
    }
}

/// Sync parameters for Bitcoin Core RPC.
///
/// In general, BDK tries to sync `scriptPubKey`s cached in `Database` with
/// `scriptPubKey`s imported in the Bitcoin Core Wallet. These parameters are used for determining
/// how the `importdescriptors` RPC calls are to be made.
pub struct RpcSyncParams {
    /// The minimum number of scripts to scan for on initial sync.
    pub start_script_count: u64,
    /// Time in unix seconds in which initial sync will start scanning from (0 to start from genesis).
    pub start_time: u64,
    /// Forces every sync to use `start_time` as import timestamp.
    pub force_start_time: bool,
    /// RPC poll rate (in seconds) to get state updates.
    pub poll_rate_sec: u64,
}

impl From<RpcSyncParams> for BdkRpcSyncParams {
    fn from(params: RpcSyncParams) -> Self {
        BdkRpcSyncParams {
            start_script_count: params.start_script_count as usize,
            start_time: params.start_time,
            force_start_time: params.force_start_time,
            poll_rate_sec: params.poll_rate_sec,
        }
    }
}

/// RpcBlockchain configuration options
pub struct RpcConfig {
    /// The bitcoin node url
    pub url: String,
    /// The bitcoin node authentication mechanism
    pub auth: Auth,
    /// The network we are using (it will be checked the bitcoin node network matches this)
    pub network: Network,
    /// The wallet name in the bitcoin node, consider using [crate::wallet::wallet_name_from_descriptor] for this
    pub wallet_name: String,
    /// Sync parameters
    pub sync_params: Option<RpcSyncParams>,
}

/// Type that can contain any of the blockchain configurations defined by the library.
pub enum BlockchainConfig {
    /// Electrum client
    Electrum { config: ElectrumConfig },
    /// Esplora client
    Esplora { config: EsploraConfig },
    /// Bitcoin Core RPC client
    Rpc { config: RpcConfig },
}

/// A wallet transaction
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TransactionDetails {
    /// Transaction id.
    pub txid: String,
    /// Received value (sats)
    /// Sum of owned outputs of this transaction.
    pub received: u64,
    /// Sent value (sats)
    /// Sum of owned inputs of this transaction.
    pub sent: u64,
    /// Fee value (sats) if confirmed.
    /// The availability of the fee depends on the backend. It's never None with an Electrum
    /// Server backend, but it could be None with a Bitcoin RPC node without txindex that receive
    /// funds while offline.
    pub fee: Option<u64>,
    /// If the transaction is confirmed, contains height and timestamp of the block containing the
    /// transaction, unconfirmed transaction contains `None`.
    pub confirmation_time: Option<BlockTime>,
}

impl From<&bdk::TransactionDetails> for TransactionDetails {
    fn from(x: &bdk::TransactionDetails) -> TransactionDetails {
        TransactionDetails {
            fee: x.fee,
            txid: x.txid.to_string(),
            received: x.received,
            sent: x.sent,
            confirmation_time: x.confirmation_time.clone(),
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
            BlockchainConfig::Rpc { config } => AnyBlockchainConfig::Rpc(BdkRpcConfig {
                url: config.url,
                auth: config.auth.into(),
                network: config.network,
                wallet_name: config.wallet_name,
                sync_params: config.sync_params.map(|p| p.into()),
            }),
        };
        let blockchain = AnyBlockchain::from_config(&any_blockchain_config)?;
        Ok(Self {
            blockchain_mutex: Mutex::new(blockchain),
        })
    }

    fn get_blockchain(&self) -> MutexGuard<AnyBlockchain> {
        self.blockchain_mutex.lock().expect("blockchain")
    }

    fn broadcast(&self, psbt: &PartiallySignedTransaction) -> Result<(), BdkError> {
        let tx = psbt.internal.lock().unwrap().clone().extract_tx();
        self.get_blockchain().broadcast(&tx)
    }

    fn get_height(&self) -> Result<u32, BdkError> {
        self.get_blockchain().get_height()
    }

    fn get_block_hash(&self, height: u32) -> Result<String, BdkError> {
        self.get_blockchain()
            .get_block_hash(u64::from(height))
            .map(|hash| hash.to_string())
    }
}

struct Wallet {
    wallet_mutex: Mutex<BdkWallet<AnyDatabase>>,
}

/// A reference to a transaction output.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct OutPoint {
    /// The referenced transaction's txid.
    txid: String,
    /// The index of the referenced output in its transaction's vout.
    vout: u32,
}

impl From<&OutPoint> for BdkOutPoint {
    fn from(x: &OutPoint) -> BdkOutPoint {
        BdkOutPoint {
            txid: Txid::from_str(&x.txid).unwrap(),
            vout: x.vout,
        }
    }
}

pub struct Balance {
    // All coinbase outputs not yet matured
    pub immature: u64,
    /// Unconfirmed UTXOs generated by a wallet tx
    pub trusted_pending: u64,
    /// Unconfirmed UTXOs received from an external wallet
    pub untrusted_pending: u64,
    /// Confirmed and immediately spendable balance
    pub confirmed: u64,
    /// Get sum of trusted_pending and confirmed coins
    pub spendable: u64,
    /// Get the whole balance visible to the wallet
    pub total: u64,
}

impl From<BdkBalance> for Balance {
    fn from(bdk_balance: BdkBalance) -> Self {
        Balance {
            immature: bdk_balance.immature,
            trusted_pending: bdk_balance.trusted_pending,
            untrusted_pending: bdk_balance.untrusted_pending,
            confirmed: bdk_balance.confirmed,
            spendable: bdk_balance.get_spendable(),
            total: bdk_balance.get_total(),
        }
    }
}

/// A transaction output, which defines new coins to be created from old ones.
pub struct TxOut {
    /// The value of the output, in satoshis.
    value: u64,
    /// The address of the output.
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
                address: BdkAddress::from_script(&x.txout.script_pubkey, network)
                    .unwrap()
                    .to_string(),
            },
            keychain: x.keychain,
            is_spent: x.is_spent,
        }
    }
}

/// Trait that logs at level INFO every update received (if any).
pub trait Progress: Send + Sync + 'static {
    /// Send a new progress update. The progress value should be in the range 0.0 - 100.0, and the message value is an
    /// optional text message that can be displayed to the user.
    fn update(&self, progress: f32, message: Option<String>);
}

struct ProgressHolder {
    progress: Box<dyn Progress>,
}

impl BdkProgress for ProgressHolder {
    fn update(&self, progress: f32, message: Option<String>) -> Result<(), BdkError> {
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
pub struct PartiallySignedTransaction {
    internal: Mutex<BdkPartiallySignedTransaction>,
}

impl PartiallySignedTransaction {
    fn new(psbt_base64: String) -> Result<Self, BdkError> {
        let psbt: BdkPartiallySignedTransaction =
            BdkPartiallySignedTransaction::from_str(&psbt_base64)?;
        Ok(PartiallySignedTransaction {
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

    /// Return the transaction as bytes.
    fn extract_tx(&self) -> Vec<u8> {
        self.internal
            .lock()
            .unwrap()
            .clone()
            .extract_tx()
            .serialize()
    }

    /// Combines this PartiallySignedTransaction with other PSBT as described by BIP 174.
    ///
    /// In accordance with BIP 174 this function is commutative i.e., `A.combine(B) == B.combine(A)`
    fn combine(
        &self,
        other: Arc<PartiallySignedTransaction>,
    ) -> Result<Arc<PartiallySignedTransaction>, BdkError> {
        let other_psbt = other.internal.lock().unwrap().clone();
        let mut original_psbt = self.internal.lock().unwrap().clone();

        original_psbt.combine(other_psbt)?;
        Ok(Arc::new(PartiallySignedTransaction {
            internal: Mutex::new(original_psbt),
        }))
    }

    /// The total transaction fee amount, sum of input amounts minus sum of output amounts, in Sats.
    /// If the PSBT is missing a TxOut for an input returns None.
    fn fee_amount(&self) -> Option<u64> {
        self.internal.lock().unwrap().fee_amount()
    }

    /// The transaction's fee rate. This value will only be accurate if calculated AFTER the
    /// `PartiallySignedTransaction` is finalized and all witness/signature data is added to the
    /// transaction.
    /// If the PSBT is missing a TxOut for an input returns None.
    fn fee_rate(&self) -> Option<Arc<FeeRate>> {
        self.internal.lock().unwrap().fee_rate().map(Arc::new)
    }
}

/// A Bitcoin wallet.
/// The Wallet acts as a way of coherently interfacing with output descriptors and related transactions. Its main components are:
///     1. Output descriptors from which it can derive addresses.
///     2. A Database where it tracks transactions and utxos related to the descriptors.
///     3. Signers that can contribute signatures to addresses instantiated from the descriptors.
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

    /// Get the Bitcoin network the wallet is using.
    fn network(&self) -> Network {
        self.get_wallet().network()
    }

    /// Sync the internal database with the blockchain.
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

    /// Return a derived address using the external descriptor, see AddressIndex for available address index selection
    /// strategies. If none of the keys in the descriptor are derivable (i.e. the descriptor does not end with a * character)
    /// then the same address will always be returned for any AddressIndex.
    fn get_address(&self, address_index: AddressIndex) -> Result<AddressInfo, BdkError> {
        self.get_wallet()
            .get_address(address_index.into())
            .map(AddressInfo::from)
    }

    /// Return the balance, meaning the sum of this wallet’s unspent outputs’ values. Note that this method only operates
    /// on the internal database, which first needs to be Wallet.sync manually.
    fn get_balance(&self) -> Result<Balance, BdkError> {
        self.get_wallet().get_balance().map(|b| b.into())
    }

    /// Sign a transaction with all the wallet’s signers.
    fn sign(&self, psbt: &PartiallySignedTransaction) -> Result<bool, BdkError> {
        let mut psbt = psbt.internal.lock().unwrap();
        self.get_wallet().sign(&mut psbt, SignOptions::default())
    }

    /// Return the list of transactions made and received by the wallet. Note that this method only operate on the internal database, which first needs to be [Wallet.sync] manually.
    fn list_transactions(&self) -> Result<Vec<TransactionDetails>, BdkError> {
        let transaction_details = self.get_wallet().list_transactions(true)?;
        Ok(transaction_details
            .iter()
            .map(TransactionDetails::from)
            .collect())
    }

    /// Return the list of unspent outputs of this wallet. Note that this method only operates on the internal database,
    /// which first needs to be Wallet.sync manually.
    fn list_unspent(&self) -> Result<Vec<LocalUtxo>, BdkError> {
        let unspents = self.get_wallet().list_unspent()?;
        Ok(unspents
            .iter()
            .map(|u| LocalUtxo::from_utxo(u, self.network()))
            .collect())
    }
}

/// A Bitcoin address.
struct Address {
    address: BdkAddress,
}

impl Address {
    fn new(address: String) -> Result<Self, BdkError> {
        BdkAddress::from_str(address.as_str())
            .map(|a| Address { address: a })
            .map_err(|e| BdkError::Generic(e.to_string()))
    }

    fn script_pubkey(&self) -> Arc<Script> {
        Arc::new(Script {
            script: self.address.script_pubkey(),
        })
    }
}

/// A Bitcoin script.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Script {
    script: BdkScript,
}

impl Script {
    fn new(raw_output_script: Vec<u8>) -> Self {
        let script: BdkScript = BdkScript::from(raw_output_script);
        Script { script }
    }
}

#[derive(Clone, Debug)]
enum RbfValue {
    Default,
    Value(u32),
}

/// The result after calling the TxBuilder finish() function. Contains unsigned PSBT and
/// transaction details.
pub struct TxBuilderResult {
    pub psbt: Arc<PartiallySignedTransaction>,
    pub transaction_details: TransactionDetails,
}

/// A transaction builder.
/// After creating the TxBuilder, you set options on it until finally calling finish to consume the builder and generate the transaction.
/// Each method on the TxBuilder returns an instance of a new TxBuilder with the option set/added.
#[derive(Clone, Debug)]
struct TxBuilder {
    recipients: Vec<(BdkScript, u64)>,
    utxos: Vec<OutPoint>,
    unspendable: HashSet<OutPoint>,
    change_policy: ChangeSpendPolicy,
    manually_selected_only: bool,
    fee_rate: Option<f32>,
    fee_absolute: Option<u64>,
    drain_wallet: bool,
    drain_to: Option<BdkScript>,
    rbf: Option<RbfValue>,
    data: Vec<u8>,
}

impl TxBuilder {
    fn new() -> Self {
        TxBuilder {
            recipients: Vec::new(),
            utxos: Vec::new(),
            unspendable: HashSet::new(),
            change_policy: ChangeSpendPolicy::ChangeAllowed,
            manually_selected_only: false,
            fee_rate: None,
            fee_absolute: None,
            drain_wallet: false,
            drain_to: None,
            rbf: None,
            data: Vec::new(),
        }
    }

    /// Add a recipient to the internal list.
    fn add_recipient(&self, script: Arc<Script>, amount: u64) -> Arc<Self> {
        let mut recipients: Vec<(BdkScript, u64)> = self.recipients.clone();
        recipients.append(&mut vec![(script.script.clone(), amount)]);
        Arc::new(TxBuilder {
            recipients,
            ..self.clone()
        })
    }

    fn set_recipients(&self, recipients: Vec<ScriptAmount>) -> Arc<Self> {
        let recipients = recipients
            .iter()
            .map(|script_amount| (script_amount.script.script.clone(), script_amount.amount))
            .collect();
        Arc::new(TxBuilder {
            recipients,
            ..self.clone()
        })
    }

    /// Add a utxo to the internal list of unspendable utxos. It’s important to note that the "must-be-spent"
    /// utxos added with [TxBuilder.addUtxo] have priority over this. See the Rust docs of the two linked methods for more details.
    fn add_unspendable(&self, unspendable: OutPoint) -> Arc<Self> {
        let mut unspendable_hash_set = self.unspendable.clone();
        unspendable_hash_set.insert(unspendable);
        Arc::new(TxBuilder {
            unspendable: unspendable_hash_set,
            ..self.clone()
        })
    }

    /// Add an outpoint to the internal list of UTXOs that must be spent. These have priority over the "unspendable"
    /// utxos, meaning that if a utxo is present both in the "utxos" and the "unspendable" list, it will be spent.
    fn add_utxo(&self, outpoint: OutPoint) -> Arc<Self> {
        self.add_utxos(vec![outpoint])
    }

    /// Add the list of outpoints to the internal list of UTXOs that must be spent. If an error occurs while adding
    /// any of the UTXOs then none of them are added and the error is returned. These have priority over the "unspendable"
    /// utxos, meaning that if a utxo is present both in the "utxos" and the "unspendable" list, it will be spent.
    fn add_utxos(&self, mut outpoints: Vec<OutPoint>) -> Arc<Self> {
        let mut utxos = self.utxos.to_vec();
        utxos.append(&mut outpoints);
        Arc::new(TxBuilder {
            utxos,
            ..self.clone()
        })
    }

    /// Do not spend change outputs. This effectively adds all the change outputs to the "unspendable" list. See TxBuilder.unspendable.
    fn do_not_spend_change(&self) -> Arc<Self> {
        Arc::new(TxBuilder {
            change_policy: ChangeSpendPolicy::ChangeForbidden,
            ..self.clone()
        })
    }

    /// Only spend utxos added by [add_utxo]. The wallet will not add additional utxos to the transaction even if they are
    /// needed to make the transaction valid.
    fn manually_selected_only(&self) -> Arc<Self> {
        Arc::new(TxBuilder {
            manually_selected_only: true,
            ..self.clone()
        })
    }

    /// Only spend change outputs. This effectively adds all the non-change outputs to the "unspendable" list. See TxBuilder.unspendable.
    fn only_spend_change(&self) -> Arc<Self> {
        Arc::new(TxBuilder {
            change_policy: ChangeSpendPolicy::OnlyChange,
            ..self.clone()
        })
    }

    /// Replace the internal list of unspendable utxos with a new list. It’s important to note that the "must-be-spent" utxos added with
    /// TxBuilder.addUtxo have priority over these. See the Rust docs of the two linked methods for more details.
    fn unspendable(&self, unspendable: Vec<OutPoint>) -> Arc<Self> {
        Arc::new(TxBuilder {
            unspendable: unspendable.into_iter().collect(),
            ..self.clone()
        })
    }

    /// Set a custom fee rate.
    fn fee_rate(&self, sat_per_vb: f32) -> Arc<Self> {
        Arc::new(TxBuilder {
            fee_rate: Some(sat_per_vb),
            ..self.clone()
        })
    }

    /// Set an absolute fee.
    fn fee_absolute(&self, fee_amount: u64) -> Arc<Self> {
        Arc::new(TxBuilder {
            fee_absolute: Some(fee_amount),
            ..self.clone()
        })
    }

    /// Spend all the available inputs. This respects filters like TxBuilder.unspendable and the change policy.
    fn drain_wallet(&self) -> Arc<Self> {
        Arc::new(TxBuilder {
            drain_wallet: true,
            ..self.clone()
        })
    }

    /// Sets the address to drain excess coins to. Usually, when there are excess coins they are sent to a change address
    /// generated by the wallet. This option replaces the usual change address with an arbitrary ScriptPubKey of your choosing.
    /// Just as with a change output, if the drain output is not needed (the excess coins are too small) it will not be included
    /// in the resulting transaction. The only difference is that it is valid to use drain_to without setting any ordinary recipients
    /// with add_recipient (but it is perfectly fine to add recipients as well). If you choose not to set any recipients, you should
    /// either provide the utxos that the transaction should spend via add_utxos, or set drain_wallet to spend all of them.
    /// When bumping the fees of a transaction made with this option, you probably want to use BumpFeeTxBuilder.allow_shrinking
    /// to allow this output to be reduced to pay for the extra fees.
    fn drain_to(&self, script: Arc<Script>) -> Arc<Self> {
        Arc::new(TxBuilder {
            drain_to: Some(script.script.clone()),
            ..self.clone()
        })
    }

    /// Enable signaling RBF. This will use the default `nsequence` value of `0xFFFFFFFD`.
    fn enable_rbf(&self) -> Arc<Self> {
        Arc::new(TxBuilder {
            rbf: Some(RbfValue::Default),
            ..self.clone()
        })
    }

    /// Enable signaling RBF with a specific nSequence value. This can cause conflicts if the wallet's descriptors contain an
    /// "older" (OP_CSV) operator and the given `nsequence` is lower than the CSV value. If the `nsequence` is higher than `0xFFFFFFFD`
    /// an error will be thrown, since it would not be a valid nSequence to signal RBF.
    fn enable_rbf_with_sequence(&self, nsequence: u32) -> Arc<Self> {
        Arc::new(TxBuilder {
            rbf: Some(RbfValue::Value(nsequence)),
            ..self.clone()
        })
    }

    /// Add data as an output using OP_RETURN.
    fn add_data(&self, data: Vec<u8>) -> Arc<Self> {
        Arc::new(TxBuilder {
            data,
            ..self.clone()
        })
    }

    /// Finish building the transaction. Returns the BIP174 PSBT.
    fn finish(&self, wallet: &Wallet) -> Result<TxBuilderResult, BdkError> {
        let wallet = wallet.get_wallet();
        let mut tx_builder = wallet.build_tx();
        for (script, amount) in &self.recipients {
            tx_builder.add_recipient(script.clone(), *amount);
        }
        tx_builder.change_policy(self.change_policy);
        if !self.utxos.is_empty() {
            let bdk_utxos: Vec<BdkOutPoint> = self.utxos.iter().map(BdkOutPoint::from).collect();
            let utxos: &[BdkOutPoint] = &bdk_utxos;
            tx_builder.add_utxos(utxos)?;
        }
        if !self.unspendable.is_empty() {
            let bdk_unspendable: Vec<BdkOutPoint> =
                self.unspendable.iter().map(BdkOutPoint::from).collect();
            tx_builder.unspendable(bdk_unspendable);
        }
        if self.manually_selected_only {
            tx_builder.manually_selected_only();
        }
        if let Some(sat_per_vb) = self.fee_rate {
            tx_builder.fee_rate(FeeRate::from_sat_per_vb(sat_per_vb));
        }
        if let Some(fee_amount) = self.fee_absolute {
            tx_builder.fee_absolute(fee_amount);
        }
        if self.drain_wallet {
            tx_builder.drain_wallet();
        }
        if let Some(script) = &self.drain_to {
            tx_builder.drain_to(script.clone());
        }
        if let Some(rbf) = &self.rbf {
            match *rbf {
                RbfValue::Default => {
                    tx_builder.enable_rbf();
                }
                RbfValue::Value(nsequence) => {
                    tx_builder.enable_rbf_with_sequence(Sequence(nsequence));
                }
            }
        }
        if !&self.data.is_empty() {
            tx_builder.add_data(self.data.as_slice());
        }

        tx_builder
            .finish()
            .map(|(psbt, tx_details)| TxBuilderResult {
                psbt: Arc::new(PartiallySignedTransaction {
                    internal: Mutex::new(psbt),
                }),
                transaction_details: TransactionDetails::from(&tx_details),
            })
    }
}

/// The BumpFeeTxBuilder is used to bump the fee on a transaction that has been broadcast and has its RBF flag set to true.
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

    /// Explicitly tells the wallet that it is allowed to reduce the amount of the output matching this script_pubkey
    /// in order to bump the transaction fee. Without specifying this the wallet will attempt to find a change output to
    /// shrink instead. Note that the output may shrink to below the dust limit and therefore be removed. If it is preserved
    /// then it is currently not guaranteed to be in the same position as it was originally. Returns an error if script_pubkey
    /// can’t be found among the recipients of the transaction we are bumping.
    fn allow_shrinking(&self, address: String) -> Arc<Self> {
        Arc::new(Self {
            allow_shrinking: Some(address),
            ..self.clone()
        })
    }

    /// Enable signaling RBF. This will use the default `nsequence` value of `0xFFFFFFFD`.
    fn enable_rbf(&self) -> Arc<Self> {
        Arc::new(Self {
            rbf: Some(RbfValue::Default),
            ..self.clone()
        })
    }

    /// Enable signaling RBF with a specific nSequence value. This can cause conflicts if the wallet's descriptors contain an
    /// "older" (OP_CSV) operator and the given `nsequence` is lower than the CSV value. If the `nsequence` is higher than `0xFFFFFFFD`
    /// an error will be thrown, since it would not be a valid nSequence to signal RBF.
    fn enable_rbf_with_sequence(&self, nsequence: u32) -> Arc<Self> {
        Arc::new(Self {
            rbf: Some(RbfValue::Value(nsequence)),
            ..self.clone()
        })
    }

    /// Finish building the transaction. Returns the BIP174 PSBT.
    fn finish(&self, wallet: &Wallet) -> Result<Arc<PartiallySignedTransaction>, BdkError> {
        let wallet = wallet.get_wallet();
        let txid = Txid::from_str(self.txid.as_str())?;
        let mut tx_builder = wallet.build_fee_bump(txid)?;
        tx_builder.fee_rate(FeeRate::from_sat_per_vb(self.fee_rate));
        if let Some(allow_shrinking) = &self.allow_shrinking {
            let address = BdkAddress::from_str(allow_shrinking)
                .map_err(|e| BdkError::Generic(e.to_string()))?;
            let script = address.script_pubkey();
            tx_builder.allow_shrinking(script)?;
        }
        if let Some(rbf) = &self.rbf {
            match *rbf {
                RbfValue::Default => {
                    tx_builder.enable_rbf();
                }
                RbfValue::Value(nsequence) => {
                    tx_builder.enable_rbf_with_sequence(Sequence(nsequence));
                }
            }
        }
        tx_builder
            .finish()
            .map(|(psbt, _)| PartiallySignedTransaction {
                internal: Mutex::new(psbt),
            })
            .map(Arc::new)
    }
}

/// Mnemonic phrases are a human-readable version of the private keys.
/// Supported number of words are 12, 15, 18, 21 and 24.
struct Mnemonic {
    internal: BdkMnemonic,
}

impl Mnemonic {
    /// Generates Mnemonic with a random entropy
    fn new(word_count: WordCount) -> Self {
        let generated_key: GeneratedKey<_, BareCtx> =
            BdkMnemonic::generate((word_count, Language::English)).unwrap();
        let mnemonic = BdkMnemonic::parse_in(Language::English, generated_key.to_string()).unwrap();
        Mnemonic { internal: mnemonic }
    }

    /// Parse a Mnemonic with given string
    fn from_string(mnemonic: String) -> Result<Self, BdkError> {
        BdkMnemonic::from_str(&mnemonic)
            .map(|m| Mnemonic { internal: m })
            .map_err(|e| BdkError::Generic(e.to_string()))
    }

    /// Create a new Mnemonic in the specified language from the given entropy.
    /// Entropy must be a multiple of 32 bits (4 bytes) and 128-256 bits in length.
    fn from_entropy(entropy: Vec<u8>) -> Result<Self, BdkError> {
        BdkMnemonic::from_entropy(entropy.as_slice())
            .map(|m| Mnemonic { internal: m })
            .map_err(|e| BdkError::Generic(e.to_string()))
    }

    /// Returns Mnemonic as string
    fn as_string(&self) -> String {
        self.internal.to_string()
    }
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

#[derive(Debug)]
struct DescriptorSecretKey {
    descriptor_secret_key_mutex: Mutex<BdkDescriptorSecretKey>,
}

impl DescriptorSecretKey {
    fn new(network: Network, mnemonic: Arc<Mnemonic>, password: Option<String>) -> Self {
        let mnemonic = mnemonic.internal.clone();
        let xkey: ExtendedKey = (mnemonic, password).into_extended_key().unwrap();
        let descriptor_secret_key = BdkDescriptorSecretKey::XPrv(DescriptorXKey {
            origin: None,
            xkey: xkey.into_xprv(network).unwrap(),
            derivation_path: BdkDerivationPath::master(),
            wildcard: bdk::descriptor::Wildcard::Unhardened,
        });
        Self {
            descriptor_secret_key_mutex: Mutex::new(descriptor_secret_key),
        }
    }

    fn from_string(private_key: String) -> Result<Self, BdkError> {
        let descriptor_secret_key = BdkDescriptorSecretKey::from_str(private_key.as_str())
            .map_err(|e| BdkError::Generic(e.to_string()))?;
        Ok(Self {
            descriptor_secret_key_mutex: Mutex::new(descriptor_secret_key),
        })
    }

    fn derive(&self, path: Arc<DerivationPath>) -> Result<Arc<Self>, BdkError> {
        let secp = Secp256k1::new();
        let descriptor_secret_key = self.descriptor_secret_key_mutex.lock().unwrap();
        let path = path.derivation_path_mutex.lock().unwrap().deref().clone();
        match descriptor_secret_key.deref() {
            BdkDescriptorSecretKey::XPrv(descriptor_x_key) => {
                let derived_xprv = descriptor_x_key.xkey.derive_priv(&secp, &path)?;
                let key_source = match descriptor_x_key.origin.clone() {
                    Some((fingerprint, origin_path)) => (fingerprint, origin_path.extend(path)),
                    None => (descriptor_x_key.xkey.fingerprint(&secp), path),
                };
                let derived_descriptor_secret_key = BdkDescriptorSecretKey::XPrv(DescriptorXKey {
                    origin: Some(key_source),
                    xkey: derived_xprv,
                    derivation_path: BdkDerivationPath::default(),
                    wildcard: descriptor_x_key.wildcard,
                });
                Ok(Arc::new(Self {
                    descriptor_secret_key_mutex: Mutex::new(derived_descriptor_secret_key),
                }))
            }
            BdkDescriptorSecretKey::Single(_) => Err(BdkError::Generic(
                "Cannot derive from a single key".to_string(),
            )),
        }
    }

    fn extend(&self, path: Arc<DerivationPath>) -> Result<Arc<Self>, BdkError> {
        let descriptor_secret_key = self.descriptor_secret_key_mutex.lock().unwrap();
        let path = path.derivation_path_mutex.lock().unwrap().deref().clone();
        match descriptor_secret_key.deref() {
            BdkDescriptorSecretKey::XPrv(descriptor_x_key) => {
                let extended_path = descriptor_x_key.derivation_path.extend(path);
                let extended_descriptor_secret_key = BdkDescriptorSecretKey::XPrv(DescriptorXKey {
                    origin: descriptor_x_key.origin.clone(),
                    xkey: descriptor_x_key.xkey,
                    derivation_path: extended_path,
                    wildcard: descriptor_x_key.wildcard,
                });
                Ok(Arc::new(Self {
                    descriptor_secret_key_mutex: Mutex::new(extended_descriptor_secret_key),
                }))
            }
            BdkDescriptorSecretKey::Single(_) => Err(BdkError::Generic(
                "Cannot extend from a single key".to_string(),
            )),
        }
    }

    fn as_public(&self) -> Arc<DescriptorPublicKey> {
        let secp = Secp256k1::new();
        let descriptor_public_key = self
            .descriptor_secret_key_mutex
            .lock()
            .unwrap()
            .to_public(&secp)
            .unwrap();
        Arc::new(DescriptorPublicKey {
            descriptor_public_key_mutex: Mutex::new(descriptor_public_key),
        })
    }

    /// Get the private key as bytes.
    fn secret_bytes(&self) -> Vec<u8> {
        let descriptor_secret_key = self.descriptor_secret_key_mutex.lock().unwrap();
        let secret_bytes: Vec<u8> = match descriptor_secret_key.deref() {
            BdkDescriptorSecretKey::XPrv(descriptor_x_key) => {
                descriptor_x_key.xkey.private_key.secret_bytes().to_vec()
            }
            BdkDescriptorSecretKey::Single(_) => {
                unreachable!()
            }
        };

        secret_bytes
    }

    fn as_string(&self) -> String {
        self.descriptor_secret_key_mutex.lock().unwrap().to_string()
    }
}

#[derive(Debug)]
struct DescriptorPublicKey {
    descriptor_public_key_mutex: Mutex<BdkDescriptorPublicKey>,
}

impl DescriptorPublicKey {
    fn from_string(public_key: String) -> Result<Self, BdkError> {
        let descriptor_public_key = BdkDescriptorPublicKey::from_str(public_key.as_str())
            .map_err(|e| BdkError::Generic(e.to_string()))?;
        Ok(Self {
            descriptor_public_key_mutex: Mutex::new(descriptor_public_key),
        })
    }

    fn derive(&self, path: Arc<DerivationPath>) -> Result<Arc<Self>, BdkError> {
        let secp = Secp256k1::new();
        let descriptor_public_key = self.descriptor_public_key_mutex.lock().unwrap();
        let path = path.derivation_path_mutex.lock().unwrap().deref().clone();

        match descriptor_public_key.deref() {
            BdkDescriptorPublicKey::XPub(descriptor_x_key) => {
                let derived_xpub = descriptor_x_key.xkey.derive_pub(&secp, &path)?;
                let key_source = match descriptor_x_key.origin.clone() {
                    Some((fingerprint, origin_path)) => (fingerprint, origin_path.extend(path)),
                    None => (descriptor_x_key.xkey.fingerprint(), path),
                };
                let derived_descriptor_public_key = BdkDescriptorPublicKey::XPub(DescriptorXKey {
                    origin: Some(key_source),
                    xkey: derived_xpub,
                    derivation_path: BdkDerivationPath::default(),
                    wildcard: descriptor_x_key.wildcard,
                });
                Ok(Arc::new(Self {
                    descriptor_public_key_mutex: Mutex::new(derived_descriptor_public_key),
                }))
            }
            BdkDescriptorPublicKey::Single(_) => Err(BdkError::Generic(
                "Cannot derive from a single key".to_string(),
            )),
        }
    }

    fn extend(&self, path: Arc<DerivationPath>) -> Result<Arc<Self>, BdkError> {
        let descriptor_public_key = self.descriptor_public_key_mutex.lock().unwrap();
        let path = path.derivation_path_mutex.lock().unwrap().deref().clone();
        match descriptor_public_key.deref() {
            BdkDescriptorPublicKey::XPub(descriptor_x_key) => {
                let extended_path = descriptor_x_key.derivation_path.extend(path);
                let extended_descriptor_public_key = BdkDescriptorPublicKey::XPub(DescriptorXKey {
                    origin: descriptor_x_key.origin.clone(),
                    xkey: descriptor_x_key.xkey,
                    derivation_path: extended_path,
                    wildcard: descriptor_x_key.wildcard,
                });
                Ok(Arc::new(Self {
                    descriptor_public_key_mutex: Mutex::new(extended_descriptor_public_key),
                }))
            }
            BdkDescriptorPublicKey::Single(_) => Err(BdkError::Generic(
                "Cannot extend from a single key".to_string(),
            )),
        }
    }

    fn as_string(&self) -> String {
        self.descriptor_public_key_mutex.lock().unwrap().to_string()
    }
}

#[derive(Debug)]
struct Descriptor {
    pub descriptor: DescriptorTemplateOut,
}

impl Descriptor {
    fn new_bip44(
        secret_key: Arc<DescriptorSecretKey>,
        keychain_kind: KeychainKind,
        network: Network,
    ) -> Self {
        let derivable_key = secret_key.descriptor_secret_key_mutex.lock().unwrap();

        match derivable_key.deref() {
            BdkDescriptorSecretKey::XPrv(descriptor_x_key) => {
                let derivable_key = descriptor_x_key.xkey;
                let descriptor_template_out =
                    Bip44(derivable_key, keychain_kind).build(network).unwrap();
                Self {
                    descriptor: descriptor_template_out,
                }
            }
            BdkDescriptorSecretKey::Single(_) => {
                unreachable!()
            }
        }
    }

    fn new_bip44_public(
        public_key: Arc<DescriptorPublicKey>,
        fingerprint: String,
        keychain_kind: KeychainKind,
        network: Network,
    ) -> Self {
        let fingerprint = Fingerprint::from_str(fingerprint.as_str()).unwrap();
        let derivable_key = public_key.descriptor_public_key_mutex.lock().unwrap();

        match derivable_key.deref() {
            BdkDescriptorPublicKey::XPub(descriptor_x_key) => {
                let derivable_key = descriptor_x_key.xkey;
                let descriptor_template_out =
                    Bip44Public(derivable_key, fingerprint, keychain_kind)
                        .build(network)
                        .unwrap();

                Self {
                    descriptor: descriptor_template_out,
                }
            }
            BdkDescriptorPublicKey::Single(_) => {
                unreachable!()
            }
        }
    }

    fn new_bip49(
        secret_key: Arc<DescriptorSecretKey>,
        keychain_kind: KeychainKind,
        network: Network,
    ) -> Self {
        let derivable_key = secret_key.descriptor_secret_key_mutex.lock().unwrap();

        match derivable_key.deref() {
            BdkDescriptorSecretKey::XPrv(descriptor_x_key) => {
                let derivable_key = descriptor_x_key.xkey;
                let descriptor_template_out =
                    Bip49(derivable_key, keychain_kind).build(network).unwrap();
                Self {
                    descriptor: descriptor_template_out,
                }
            }
            BdkDescriptorSecretKey::Single(_) => {
                unreachable!()
            }
        }
    }

    fn new_bip49_public(
        public_key: Arc<DescriptorPublicKey>,
        fingerprint: String,
        keychain_kind: KeychainKind,
        network: Network,
    ) -> Self {
        let fingerprint = Fingerprint::from_str(fingerprint.as_str()).unwrap();
        let derivable_key = public_key.descriptor_public_key_mutex.lock().unwrap();

        match derivable_key.deref() {
            BdkDescriptorPublicKey::XPub(descriptor_x_key) => {
                let derivable_key = descriptor_x_key.xkey;
                let descriptor_template_out =
                    Bip49Public(derivable_key, fingerprint, keychain_kind)
                        .build(network)
                        .unwrap();

                Self {
                    descriptor: descriptor_template_out,
                }
            }
            BdkDescriptorPublicKey::Single(_) => {
                unreachable!()
            }
        }
    }

    fn new_bip84(
        secret_key: Arc<DescriptorSecretKey>,
        keychain_kind: KeychainKind,
        network: Network,
    ) -> Self {
        let derivable_key = secret_key.descriptor_secret_key_mutex.lock().unwrap();

        match derivable_key.deref() {
            BdkDescriptorSecretKey::XPrv(descriptor_x_key) => {
                let derivable_key = descriptor_x_key.xkey;
                let descriptor_template_out =
                    Bip84(derivable_key, keychain_kind).build(network).unwrap();
                Self {
                    descriptor: descriptor_template_out,
                }
            }
            BdkDescriptorSecretKey::Single(_) => {
                unreachable!()
            }
        }
    }

    fn new_bip84_public(
        public_key: Arc<DescriptorPublicKey>,
        fingerprint: String,
        keychain_kind: KeychainKind,
        network: Network,
    ) -> Self {
        let fingerprint = Fingerprint::from_str(fingerprint.as_str()).unwrap();
        let derivable_key = public_key.descriptor_public_key_mutex.lock().unwrap();

        match derivable_key.deref() {
            BdkDescriptorPublicKey::XPub(descriptor_x_key) => {
                let derivable_key = descriptor_x_key.xkey;
                let descriptor_template_out =
                    Bip84Public(derivable_key, fingerprint, keychain_kind)
                        .build(network)
                        .unwrap();

                Self {
                    descriptor: descriptor_template_out,
                }
            }
            BdkDescriptorPublicKey::Single(_) => {
                unreachable!()
            }
        }
    }

    fn as_string_private(&self) -> String {
        let descriptor = &self.descriptor.0;
        let key_map = &self.descriptor.1;
        descriptor.to_string_with_secret(key_map)
    }

    fn as_string(&self) -> String {
        self.descriptor.0.to_string()
    }
}

uniffi::deps::static_assertions::assert_impl_all!(Wallet: Sync, Send);

// The goal of these tests to to ensure `bdk-ffi` intermediate code correctly calls `bdk` APIs.
// These tests should not be used to verify `bdk` behavior that is already tested in the `bdk`
// crate.
#[cfg(test)]
mod test {
    use crate::*;
    use bdk::bitcoin::Address;
    use bdk::bitcoin::Network::Testnet;
    use bdk::wallet::get_funded_wallet;
    use std::str::FromStr;
    use std::sync::Mutex;

    #[test]
    fn test_drain_wallet() {
        let test_wpkh = "wpkh(cVpPVruEDdmutPzisEsYvtST1usBR3ntr8pXSyt6D2YYqXRyPcFW)";
        let (funded_wallet, _, _) = get_funded_wallet(test_wpkh);
        let test_wallet = Wallet {
            wallet_mutex: Mutex::new(funded_wallet),
        };
        let drain_to_address = "tb1ql7w62elx9ucw4pj5lgw4l028hmuw80sndtntxt".to_string();
        let drain_to_script = crate::Address::new(drain_to_address)
            .unwrap()
            .script_pubkey();
        let tx_builder = TxBuilder::new()
            .drain_wallet()
            .drain_to(drain_to_script.clone());
        assert!(tx_builder.drain_wallet);
        assert_eq!(tx_builder.drain_to, Some(drain_to_script.script.clone()));

        let tx_builder_result = tx_builder.finish(&test_wallet).unwrap();
        let psbt = tx_builder_result.psbt.internal.lock().unwrap().clone();
        let tx_details = tx_builder_result.transaction_details;

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
        assert_eq!(input_value, 50_000_u64);

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
        assert_eq!(output_value, 49_890_u64); // input - fee

        assert_eq!(
            tx_details.txid,
            "312f1733badab22dc26b8dcbc83ba5629fb7b493af802e8abe07d865e49629c5"
        );
        assert_eq!(tx_details.received, 0);
        assert_eq!(tx_details.sent, 50000);
        assert!(tx_details.fee.is_some());
        assert_eq!(tx_details.fee.unwrap(), 110);
        assert!(tx_details.confirmation_time.is_none());
    }

    fn get_descriptor_secret_key() -> DescriptorSecretKey {
        let mnemonic = Mnemonic::from_string("chaos fabric time speed sponsor all flat solution wisdom trophy crack object robot pave observe combine where aware bench orient secret primary cable detect".to_string()).unwrap();
        DescriptorSecretKey::new(Testnet, Arc::new(mnemonic), None)
    }

    fn derive_dsk(
        key: &DescriptorSecretKey,
        path: &str,
    ) -> Result<Arc<DescriptorSecretKey>, BdkError> {
        let path = Arc::new(DerivationPath::new(path.to_string()).unwrap());
        key.derive(path)
    }

    fn extend_dsk(
        key: &DescriptorSecretKey,
        path: &str,
    ) -> Result<Arc<DescriptorSecretKey>, BdkError> {
        let path = Arc::new(DerivationPath::new(path.to_string()).unwrap());
        key.extend(path)
    }

    fn derive_dpk(
        key: &DescriptorPublicKey,
        path: &str,
    ) -> Result<Arc<DescriptorPublicKey>, BdkError> {
        let path = Arc::new(DerivationPath::new(path.to_string()).unwrap());
        key.derive(path)
    }

    fn extend_dpk(
        key: &DescriptorPublicKey,
        path: &str,
    ) -> Result<Arc<DescriptorPublicKey>, BdkError> {
        let path = Arc::new(DerivationPath::new(path.to_string()).unwrap());
        key.extend(path)
    }

    #[test]
    fn test_generate_descriptor_secret_key() {
        let master_dsk = get_descriptor_secret_key();
        assert_eq!(master_dsk.as_string(), "tprv8ZgxMBicQKsPdWuqM1t1CDRvQtQuBPyfL6GbhQwtxDKgUAVPbxmj71pRA8raTqLrec5LyTs5TqCxdABcZr77bt2KyWA5bizJHnC4g4ysm4h/*");
        assert_eq!(master_dsk.as_public().as_string(), "tpubD6NzVbkrYhZ4WywdEfYbbd62yuvqLjAZuPsNyvzCNV85JekAEMbKHWSHLF9h3j45SxewXDcLv328B1SEZrxg4iwGfmdt1pDFjZiTkGiFqGa/*");
    }

    #[test]
    fn test_derive_self() {
        let master_dsk = get_descriptor_secret_key();
        let derived_dsk: &DescriptorSecretKey = &derive_dsk(&master_dsk, "m").unwrap();
        assert_eq!(derived_dsk.as_string(), "[d1d04177]tprv8ZgxMBicQKsPdWuqM1t1CDRvQtQuBPyfL6GbhQwtxDKgUAVPbxmj71pRA8raTqLrec5LyTs5TqCxdABcZr77bt2KyWA5bizJHnC4g4ysm4h/*");

        let master_dpk: &DescriptorPublicKey = &master_dsk.as_public();
        let derived_dpk: &DescriptorPublicKey = &derive_dpk(master_dpk, "m").unwrap();
        assert_eq!(derived_dpk.as_string(), "[d1d04177]tpubD6NzVbkrYhZ4WywdEfYbbd62yuvqLjAZuPsNyvzCNV85JekAEMbKHWSHLF9h3j45SxewXDcLv328B1SEZrxg4iwGfmdt1pDFjZiTkGiFqGa/*");
    }

    #[test]
    fn test_derive_descriptors_keys() {
        let master_dsk = get_descriptor_secret_key();
        let derived_dsk: &DescriptorSecretKey = &derive_dsk(&master_dsk, "m/0").unwrap();
        assert_eq!(derived_dsk.as_string(), "[d1d04177/0]tprv8d7Y4JLmD25jkKbyDZXcdoPHu1YtMHuH21qeN7mFpjfumtSU7eZimFYUCSa3MYzkEYfSNRBV34GEr2QXwZCMYRZ7M1g6PUtiLhbJhBZEGYJ/*");

        let master_dpk: &DescriptorPublicKey = &master_dsk.as_public();
        let derived_dpk: &DescriptorPublicKey = &derive_dpk(master_dpk, "m/0").unwrap();
        assert_eq!(derived_dpk.as_string(), "[d1d04177/0]tpubD9oaCiP1MPmQdndm7DCD3D3QU34pWd6BbKSRedoZF1UJcNhEk3PJwkALNYkhxeTKL29oGNR7psqvT1KZydCGqUDEKXN6dVQJY2R8ooLPy8m/*");
    }

    #[test]
    fn test_extend_descriptor_keys() {
        let master_dsk = get_descriptor_secret_key();
        let extended_dsk: &DescriptorSecretKey = &extend_dsk(&master_dsk, "m/0").unwrap();
        assert_eq!(extended_dsk.as_string(), "tprv8ZgxMBicQKsPdWuqM1t1CDRvQtQuBPyfL6GbhQwtxDKgUAVPbxmj71pRA8raTqLrec5LyTs5TqCxdABcZr77bt2KyWA5bizJHnC4g4ysm4h/0/*");

        let master_dpk: &DescriptorPublicKey = &master_dsk.as_public();
        let extended_dpk: &DescriptorPublicKey = &extend_dpk(master_dpk, "m/0").unwrap();
        assert_eq!(extended_dpk.as_string(), "tpubD6NzVbkrYhZ4WywdEfYbbd62yuvqLjAZuPsNyvzCNV85JekAEMbKHWSHLF9h3j45SxewXDcLv328B1SEZrxg4iwGfmdt1pDFjZiTkGiFqGa/0/*");

        let wif = "L2wTu6hQrnDMiFNWA5na6jB12ErGQqtXwqpSL7aWquJaZG8Ai3ch";
        let extended_key = DescriptorSecretKey::from_string(wif.to_string()).unwrap();
        let result = extended_key.derive(Arc::new(DerivationPath::new("m/0".to_string()).unwrap()));
        dbg!(&result);
        assert!(result.is_err());
    }

    #[test]
    fn test_from_str_descriptor_secret_key() {
        let key1 = "L2wTu6hQrnDMiFNWA5na6jB12ErGQqtXwqpSL7aWquJaZG8Ai3ch";
        let key2 = "tprv8ZgxMBicQKsPcwcD4gSnMti126ZiETsuX7qwrtMypr6FBwAP65puFn4v6c3jrN9VwtMRMph6nyT63NrfUL4C3nBzPcduzVSuHD7zbX2JKVc/1/1/1/*";

        let private_descriptor_key1 = DescriptorSecretKey::from_string(key1.to_string()).unwrap();
        let private_descriptor_key2 = DescriptorSecretKey::from_string(key2.to_string()).unwrap();

        dbg!(private_descriptor_key1);
        dbg!(private_descriptor_key2);

        // Should error out because you can't produce a DescriptorSecretKey from an xpub
        let key0 = "tpubDBrgjcxBxnXyL575sHdkpKohWu5qHKoQ7TJXKNrYznh5fVEGBv89hA8ENW7A8MFVpFUSvgLqc4Nj1WZcpePX6rrxviVtPowvMuGF5rdT2Vi";
        assert!(DescriptorSecretKey::from_string(key0.to_string()).is_err());
    }

    #[test]
    fn test_derive_and_extend_descriptor_secret_key() {
        let master_dsk = get_descriptor_secret_key();
        // derive DescriptorSecretKey with path "m/0" from master
        let derived_dsk: &DescriptorSecretKey = &derive_dsk(&master_dsk, "m/0").unwrap();
        assert_eq!(derived_dsk.as_string(), "[d1d04177/0]tprv8d7Y4JLmD25jkKbyDZXcdoPHu1YtMHuH21qeN7mFpjfumtSU7eZimFYUCSa3MYzkEYfSNRBV34GEr2QXwZCMYRZ7M1g6PUtiLhbJhBZEGYJ/*");

        // extend derived_dsk with path "m/0"
        let extended_dsk: &DescriptorSecretKey = &extend_dsk(derived_dsk, "m/0").unwrap();
        assert_eq!(extended_dsk.as_string(), "[d1d04177/0]tprv8d7Y4JLmD25jkKbyDZXcdoPHu1YtMHuH21qeN7mFpjfumtSU7eZimFYUCSa3MYzkEYfSNRBV34GEr2QXwZCMYRZ7M1g6PUtiLhbJhBZEGYJ/0/*");
    }

    #[test]
    fn test_derive_hardened_path_using_public() {
        let master_dpk = get_descriptor_secret_key().as_public();
        let derived_dpk = &derive_dpk(&master_dpk, "m/84h/1h/0h");
        assert!(derived_dpk.is_err());
    }

    #[test]
    fn test_retrieve_master_secret_key() {
        let master_dpk = get_descriptor_secret_key();
        let master_private_key = master_dpk.secret_bytes().to_hex();
        assert_eq!(
            master_private_key,
            "e93315d6ce401eb4db803a56232f0ed3e69b053774e6047df54f1bd00e5ea936"
        )
    }

    #[test]
    fn test_psbt_fee() {
        let test_wpkh = "wpkh(cVpPVruEDdmutPzisEsYvtST1usBR3ntr8pXSyt6D2YYqXRyPcFW)";
        let (funded_wallet, _, _) = get_funded_wallet(test_wpkh);
        let test_wallet = Wallet {
            wallet_mutex: Mutex::new(funded_wallet),
        };
        let drain_to_address = "tb1ql7w62elx9ucw4pj5lgw4l028hmuw80sndtntxt".to_string();
        let drain_to_script = crate::Address::new(drain_to_address)
            .unwrap()
            .script_pubkey();

        let tx_builder = TxBuilder::new()
            .fee_rate(2.0)
            .drain_wallet()
            .drain_to(drain_to_script.clone());
        //dbg!(&tx_builder);
        assert!(tx_builder.drain_wallet);
        assert_eq!(tx_builder.drain_to, Some(drain_to_script.script.clone()));

        let tx_builder_result = tx_builder.finish(&test_wallet).unwrap();

        assert!(tx_builder_result.psbt.fee_rate().is_some());
        assert_eq!(
            tx_builder_result.psbt.fee_rate().unwrap().as_sat_per_vb(),
            2.682927
        );

        assert!(tx_builder_result.psbt.fee_amount().is_some());
        assert_eq!(tx_builder_result.psbt.fee_amount().unwrap(), 220);
    }
}
