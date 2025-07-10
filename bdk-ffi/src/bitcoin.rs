use crate::error::{
    AddressParseError, ExtractTxError, FeeRateError, FromScriptError, HashParseError, PsbtError,
    PsbtParseError, TransactionError,
};
use crate::error::{ParseAmountError, PsbtFinalizeError};
use crate::{impl_from_core_type, impl_hash_like, impl_into_core_type};

use bdk_wallet::bitcoin::address::NetworkChecked;
use bdk_wallet::bitcoin::address::NetworkUnchecked;
use bdk_wallet::bitcoin::address::{Address as BdkAddress, AddressData as BdkAddressData};
use bdk_wallet::bitcoin::blockdata::block::Header as BdkHeader;
use bdk_wallet::bitcoin::consensus::encode::deserialize;
use bdk_wallet::bitcoin::consensus::encode::serialize;
use bdk_wallet::bitcoin::consensus::Decodable;
use bdk_wallet::bitcoin::hashes::sha256::Hash as BitcoinSha256Hash;
use bdk_wallet::bitcoin::hashes::sha256d::Hash as BitcoinDoubleSha256Hash;
use bdk_wallet::bitcoin::io::Cursor;
use bdk_wallet::bitcoin::secp256k1::Secp256k1;
use bdk_wallet::bitcoin::Amount as BdkAmount;
use bdk_wallet::bitcoin::BlockHash as BitcoinBlockHash;
use bdk_wallet::bitcoin::FeeRate as BdkFeeRate;
use bdk_wallet::bitcoin::Network;
use bdk_wallet::bitcoin::OutPoint as BdkOutPoint;
use bdk_wallet::bitcoin::Psbt as BdkPsbt;
use bdk_wallet::bitcoin::ScriptBuf as BdkScriptBuf;
use bdk_wallet::bitcoin::Transaction as BdkTransaction;
use bdk_wallet::bitcoin::TxIn as BdkTxIn;
use bdk_wallet::bitcoin::TxOut as BdkTxOut;
use bdk_wallet::bitcoin::Txid as BitcoinTxid;
use bdk_wallet::bitcoin::Wtxid as BitcoinWtxid;
use bdk_wallet::miniscript::psbt::PsbtExt;
use bdk_wallet::serde_json;

use std::fmt::Display;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::ops::Deref;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

/// A reference to an unspent output by TXID and output index.
#[derive(Debug, Clone, Eq, PartialEq, std::hash::Hash, uniffi:: Record)]
pub struct OutPoint {
    /// The transaction.
    pub txid: Arc<Txid>,
    /// The index of the output in the transaction.
    pub vout: u32,
}

impl From<&BdkOutPoint> for OutPoint {
    fn from(outpoint: &BdkOutPoint) -> Self {
        OutPoint {
            txid: Arc::new(Txid(outpoint.txid)),
            vout: outpoint.vout,
        }
    }
}

impl From<BdkOutPoint> for OutPoint {
    fn from(value: BdkOutPoint) -> Self {
        Self {
            txid: Arc::new(Txid(value.txid)),
            vout: value.vout,
        }
    }
}

impl From<OutPoint> for BdkOutPoint {
    fn from(outpoint: OutPoint) -> Self {
        BdkOutPoint {
            txid: BitcoinTxid::from_raw_hash(outpoint.txid.0.to_raw_hash()),
            vout: outpoint.vout,
        }
    }
}

/// An [`OutPoint`] used as a key in a hash map.
///
/// Due to limitations in generating the foreign language bindings, we cannot use [`OutPoint`] as a
/// key for hash maps.
#[derive(Debug, PartialEq, Eq, std::hash::Hash, uniffi::Object)]
#[uniffi::export(Debug, Eq, Hash)]
pub struct HashableOutPoint(pub(crate) OutPoint);

#[uniffi::export]
impl HashableOutPoint {
    /// Create a key for a key-value store from an [`OutPoint`]
    #[uniffi::constructor]
    pub fn new(outpoint: OutPoint) -> Self {
        Self(outpoint)
    }

    /// Get the internal [`OutPoint`]
    pub fn outpoint(&self) -> OutPoint {
        self.0.clone()
    }
}

/// Represents fee rate.
///
/// This is an integer type representing fee rate in sat/kwu. It provides protection against mixing
/// up the types as well as basic formatting features.
#[derive(Clone, Debug, uniffi::Object)]
pub struct FeeRate(pub BdkFeeRate);

#[uniffi::export]
impl FeeRate {
    #[uniffi::constructor]
    pub fn from_sat_per_vb(sat_vb: u64) -> Result<Self, FeeRateError> {
        let fee_rate: Option<BdkFeeRate> = BdkFeeRate::from_sat_per_vb(sat_vb);
        match fee_rate {
            Some(fee_rate) => Ok(FeeRate(fee_rate)),
            None => Err(FeeRateError::ArithmeticOverflow),
        }
    }

    #[uniffi::constructor]
    pub fn from_sat_per_kwu(sat_kwu: u64) -> Self {
        FeeRate(BdkFeeRate::from_sat_per_kwu(sat_kwu))
    }

    pub fn to_sat_per_vb_ceil(&self) -> u64 {
        self.0.to_sat_per_vb_ceil()
    }

    pub fn to_sat_per_vb_floor(&self) -> u64 {
        self.0.to_sat_per_vb_floor()
    }

    pub fn to_sat_per_kwu(&self) -> u64 {
        self.0.to_sat_per_kwu()
    }
}

impl_from_core_type!(BdkFeeRate, FeeRate);
impl_into_core_type!(FeeRate, BdkFeeRate);

/// The Amount type can be used to express Bitcoin amounts that support arithmetic and conversion
/// to various denominations. The operations that Amount implements will panic when overflow or
/// underflow occurs. Also note that since the internal representation of amounts is unsigned,
/// subtracting below zero is considered an underflow and will cause a panic.
#[derive(Debug, Clone, PartialEq, Eq, uniffi::Object)]
pub struct Amount(pub BdkAmount);

#[uniffi::export]
impl Amount {
    /// Create an Amount with satoshi precision and the given number of satoshis.
    #[uniffi::constructor]
    pub fn from_sat(satoshi: u64) -> Self {
        Amount(BdkAmount::from_sat(satoshi))
    }

    /// Convert from a value expressing bitcoins to an Amount.
    #[uniffi::constructor]
    pub fn from_btc(btc: f64) -> Result<Self, ParseAmountError> {
        let bitcoin_amount = BdkAmount::from_btc(btc).map_err(ParseAmountError::from)?;
        Ok(Amount(bitcoin_amount))
    }

    /// Get the number of satoshis in this Amount.
    pub fn to_sat(&self) -> u64 {
        self.0.to_sat()
    }

    /// Express this Amount as a floating-point value in Bitcoin. Please be aware of the risk of
    /// using floating-point numbers.
    pub fn to_btc(&self) -> f64 {
        self.0.to_btc()
    }
}

impl_from_core_type!(BdkAmount, Amount);
impl_into_core_type!(Amount, BdkAmount);

/// A bitcoin script: https://en.bitcoin.it/wiki/Script
#[derive(Clone, Debug, uniffi::Object)]
pub struct Script(pub BdkScriptBuf);

#[uniffi::export]
impl Script {
    /// Interpret an array of bytes as a bitcoin script.
    #[uniffi::constructor]
    pub fn new(raw_output_script: Vec<u8>) -> Self {
        let script: BdkScriptBuf = raw_output_script.into();
        Script(script)
    }

    /// Convert a script into an array of bytes.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_bytes()
    }
}

impl_from_core_type!(BdkScriptBuf, Script);
impl_into_core_type!(Script, BdkScriptBuf);

/// Bitcoin block header.
/// Contains all the block’s information except the actual transactions, but including a root of a merkle tree
/// committing to all transactions in the block.
#[derive(uniffi::Record)]
pub struct Header {
    /// Block version, now repurposed for soft fork signalling.
    pub version: i32,
    /// Reference to the previous block in the chain.
    pub prev_blockhash: Arc<BlockHash>,
    /// The root hash of the merkle tree of transactions in the block.
    pub merkle_root: Arc<TxMerkleNode>,
    /// The timestamp of the block, as claimed by the miner.
    pub time: u32,
    /// The target value below which the blockhash must lie.
    pub bits: u32,
    /// The nonce, selected to obtain a low enough blockhash.
    pub nonce: u32,
}

impl From<BdkHeader> for Header {
    fn from(bdk_header: BdkHeader) -> Self {
        Header {
            version: bdk_header.version.to_consensus(),
            prev_blockhash: Arc::new(BlockHash(bdk_header.prev_blockhash)),
            merkle_root: Arc::new(TxMerkleNode(bdk_header.merkle_root.to_raw_hash())),
            time: bdk_header.time,
            bits: bdk_header.bits.to_consensus(),
            nonce: bdk_header.nonce,
        }
    }
}

/// The type of address.
#[derive(Debug, uniffi::Enum)]
pub enum AddressData {
    /// Legacy.
    P2pkh { pubkey_hash: String },
    /// Wrapped Segwit
    P2sh { script_hash: String },
    /// Segwit
    Segwit { witness_program: WitnessProgram },
}

/// The version and program of a Segwit address.
#[derive(Debug, uniffi::Record)]
pub struct WitnessProgram {
    /// Version. For example 1 for Taproot.
    pub version: u8,
    /// The witness program.
    pub program: Vec<u8>,
}

/// A bitcoin address
#[derive(Debug, PartialEq, Eq, uniffi::Object)]
#[uniffi::export(Eq, Display)]
pub struct Address(BdkAddress<NetworkChecked>);

#[uniffi::export]
impl Address {
    /// Parse a string as an address for the given network.
    #[uniffi::constructor]
    pub fn new(address: String, network: Network) -> Result<Self, AddressParseError> {
        let parsed_address = address.parse::<bdk_wallet::bitcoin::Address<NetworkUnchecked>>()?;
        let network_checked_address = parsed_address.require_network(network)?;

        Ok(Address(network_checked_address))
    }

    /// Parse a script as an address for the given network
    #[uniffi::constructor]
    pub fn from_script(script: Arc<Script>, network: Network) -> Result<Self, FromScriptError> {
        let address = BdkAddress::from_script(&script.0.clone(), network)?;

        Ok(Address(address))
    }

    /// Return the `scriptPubKey` underlying an address.
    pub fn script_pubkey(&self) -> Arc<Script> {
        Arc::new(Script(self.0.script_pubkey()))
    }

    /// Return a BIP-21 URI string for this address.
    pub fn to_qr_uri(&self) -> String {
        self.0.to_qr_uri()
    }

    /// Is the address valid for the provided network
    pub fn is_valid_for_network(&self, network: Network) -> bool {
        let address_str = self.0.to_string();
        if let Ok(unchecked_address) = address_str.parse::<BdkAddress<NetworkUnchecked>>() {
            unchecked_address.is_valid_for_network(network)
        } else {
            false
        }
    }

    /// Return the data for the address.
    pub fn to_address_data(&self) -> AddressData {
        match self.0.to_address_data() {
            BdkAddressData::P2pkh { pubkey_hash } => AddressData::P2pkh {
                pubkey_hash: pubkey_hash.to_string(),
            },
            BdkAddressData::P2sh { script_hash } => AddressData::P2sh {
                script_hash: script_hash.to_string(),
            },
            BdkAddressData::Segwit { witness_program } => AddressData::Segwit {
                witness_program: WitnessProgram {
                    version: witness_program.version().to_num(),
                    program: witness_program.program().as_bytes().to_vec(),
                },
            },
            // AddressData is marked #[non_exhaustive] in bitcoin crate
            _ => unimplemented!("Unsupported address type"),
        }
    }
}

impl Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl_from_core_type!(BdkAddress, Address);
impl_into_core_type!(Address, BdkAddress);

/// Bitcoin transaction.
/// An authenticated movement of coins.
#[derive(Debug, Clone, PartialEq, Eq, uniffi::Object)]
#[uniffi::export(Eq, Display)]
pub struct Transaction(BdkTransaction);

#[uniffi::export]
impl Transaction {
    /// Creates a new `Transaction` instance from serialized transaction bytes.
    #[uniffi::constructor]
    pub fn new(transaction_bytes: Vec<u8>) -> Result<Self, TransactionError> {
        let mut decoder = Cursor::new(transaction_bytes);
        let tx: BdkTransaction = BdkTransaction::consensus_decode(&mut decoder)?;
        Ok(Transaction(tx))
    }

    /// Computes the Txid.
    /// Hashes the transaction excluding the segwit data (i.e. the marker, flag bytes, and the witness fields themselves).
    pub fn compute_txid(&self) -> Arc<Txid> {
        Arc::new(Txid(self.0.compute_txid()))
    }

    /// Compute the Wtxid, which includes the witness in the transaction hash.
    pub fn compute_wtxid(&self) -> Arc<Wtxid> {
        Arc::new(Wtxid(self.0.compute_wtxid()))
    }

    /// Returns the weight of this transaction, as defined by BIP-141.
    ///
    /// > Transaction weight is defined as Base transaction size * 3 + Total transaction size (ie.
    /// > the same method as calculating Block weight from Base size and Total size).
    ///
    /// For transactions with an empty witness, this is simply the consensus-serialized size times
    /// four. For transactions with a witness, this is the non-witness consensus-serialized size
    /// multiplied by three plus the with-witness consensus-serialized size.
    ///
    /// For transactions with no inputs, this function will return a value 2 less than the actual
    /// weight of the serialized transaction. The reason is that zero-input transactions, post-segwit,
    /// cannot be unambiguously serialized; we make a choice that adds two extra bytes. For more
    /// details see [BIP 141](https://github.com/bitcoin/bips/blob/master/bip-0141.mediawiki)
    /// which uses a "input count" of `0x00` as a `marker` for a Segwit-encoded transaction.
    ///
    /// If you need to use 0-input transactions, we strongly recommend you do so using the PSBT
    /// API. The unsigned transaction encoded within PSBT is always a non-segwit transaction
    /// and can therefore avoid this ambiguity.
    #[inline]
    pub fn weight(&self) -> u64 {
        self.0.weight().to_wu()
    }

    /// Returns the total transaction size
    ///
    /// Total transaction size is the transaction size in bytes serialized as described in BIP144,
    /// including base data and witness data.
    pub fn total_size(&self) -> u64 {
        self.0.total_size() as u64
    }

    /// Returns the "virtual size" (vsize) of this transaction.
    ///
    /// Will be `ceil(weight / 4.0)`. Note this implements the virtual size as per [`BIP141`], which
    /// is different to what is implemented in Bitcoin Core.
    /// > Virtual transaction size is defined as Transaction weight / 4 (rounded up to the next integer).
    ///
    /// [`BIP141`]: https://github.com/bitcoin/bips/blob/master/bip-0141.mediawiki
    #[inline]
    pub fn vsize(&self) -> u64 {
        self.0.vsize() as u64
    }

    /// Checks if this is a coinbase transaction.
    /// The first transaction in the block distributes the mining reward and is called the coinbase transaction.
    /// It is impossible to check if the transaction is first in the block, so this function checks the structure
    /// of the transaction instead - the previous output must be all-zeros (creates satoshis “out of thin air”).
    pub fn is_coinbase(&self) -> bool {
        self.0.is_coinbase()
    }

    /// Returns `true` if the transaction itself opted in to be BIP-125-replaceable (RBF).
    ///
    /// # Warning
    ///
    /// **Incorrectly relying on RBF may lead to monetary loss!**
    ///
    /// This **does not** cover the case where a transaction becomes replaceable due to ancestors
    /// being RBF. Please note that transactions **may be replaced** even if they **do not** include
    /// the RBF signal: <https://bitcoinops.org/en/newsletters/2022/10/19/#transaction-replacement-option>.
    pub fn is_explicitly_rbf(&self) -> bool {
        self.0.is_explicitly_rbf()
    }

    /// Returns `true` if this transactions nLockTime is enabled ([BIP-65]).
    ///
    /// [BIP-65]: https://github.com/bitcoin/bips/blob/master/bip-0065.mediawiki
    pub fn is_lock_time_enabled(&self) -> bool {
        self.0.is_lock_time_enabled()
    }

    /// The protocol version, is currently expected to be 1 or 2 (BIP 68).
    pub fn version(&self) -> i32 {
        self.0.version.0
    }

    /// Serialize transaction into consensus-valid format. See https://docs.rs/bitcoin/latest/bitcoin/struct.Transaction.html#serialization-notes for more notes on transaction serialization.
    pub fn serialize(&self) -> Vec<u8> {
        serialize(&self.0)
    }

    /// List of transaction inputs.
    pub fn input(&self) -> Vec<TxIn> {
        self.0.input.iter().map(|tx_in| tx_in.into()).collect()
    }

    /// List of transaction outputs.
    pub fn output(&self) -> Vec<TxOut> {
        self.0.output.iter().map(|tx_out| tx_out.into()).collect()
    }

    /// Block height or timestamp. Transaction cannot be included in a block until this height/time.
    ///
    /// /// ### Relevant BIPs
    ///
    /// * [BIP-65 OP_CHECKLOCKTIMEVERIFY](https://github.com/bitcoin/bips/blob/master/bip-0065.mediawiki)
    /// * [BIP-113 Median time-past as endpoint for lock-time calculations](https://github.com/bitcoin/bips/blob/master/bip-0113.mediawiki)
    pub fn lock_time(&self) -> u32 {
        self.0.lock_time.to_consensus_u32()
    }
}

impl From<BdkTransaction> for Transaction {
    fn from(tx: BdkTransaction) -> Self {
        Transaction(tx)
    }
}

impl From<&BdkTransaction> for Transaction {
    fn from(tx: &BdkTransaction) -> Self {
        Transaction(tx.clone())
    }
}

impl From<&Transaction> for BdkTransaction {
    fn from(tx: &Transaction) -> Self {
        tx.0.clone()
    }
}

impl Display for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

/// A Partially Signed Transaction.
#[derive(uniffi::Object)]
pub struct Psbt(pub(crate) Mutex<BdkPsbt>);

#[uniffi::export]
impl Psbt {
    /// Creates a new `Psbt` instance from a base64-encoded string.
    #[uniffi::constructor]
    pub(crate) fn new(psbt_base64: String) -> Result<Self, PsbtParseError> {
        let psbt: BdkPsbt = BdkPsbt::from_str(&psbt_base64)?;
        Ok(Psbt(Mutex::new(psbt)))
    }

    /// Creates a PSBT from an unsigned transaction.
    ///
    /// # Errors
    ///
    /// If transactions is not unsigned.
    #[uniffi::constructor]
    pub(crate) fn from_unsigned_tx(tx: Arc<Transaction>) -> Result<Arc<Psbt>, PsbtError> {
        let psbt: BdkPsbt = BdkPsbt::from_unsigned_tx(tx.0.clone())?;
        Ok(Arc::new(Psbt(Mutex::new(psbt))))
    }

    /// Create a new `Psbt` from a `.psbt` file.
    #[uniffi::constructor]
    pub(crate) fn from_file(path: String) -> Result<Self, PsbtError> {
        let file = File::open(path)?;
        let mut buf_read = BufReader::new(file);
        let psbt: BdkPsbt = BdkPsbt::deserialize_from_reader(&mut buf_read)?;
        Ok(Psbt(Mutex::new(psbt)))
    }

    /// Serialize the PSBT into a base64-encoded string.
    pub(crate) fn serialize(&self) -> String {
        let psbt = self.0.lock().unwrap().clone();
        psbt.to_string()
    }

    /// Extracts the `Transaction` from a `Psbt` by filling in the available signature information.
    ///
    /// #### Errors
    ///
    /// `ExtractTxError` variants will contain either the `Psbt` itself or the `Transaction`
    /// that was extracted. These can be extracted from the Errors in order to recover.
    /// See the error documentation for info on the variants. In general, it covers large fees.
    pub(crate) fn extract_tx(&self) -> Result<Arc<Transaction>, ExtractTxError> {
        let tx: BdkTransaction = self.0.lock().unwrap().clone().extract_tx()?;
        let transaction: Transaction = tx.into();
        Ok(Arc::new(transaction))
    }

    /// Calculates transaction fee.
    ///
    /// 'Fee' being the amount that will be paid for mining a transaction with the current inputs
    /// and outputs i.e., the difference in value of the total inputs and the total outputs.
    ///
    /// #### Errors
    ///
    /// - `MissingUtxo` when UTXO information for any input is not present or is invalid.
    /// - `NegativeFee` if calculated value is negative.
    /// - `FeeOverflow` if an integer overflow occurs.
    pub(crate) fn fee(&self) -> Result<u64, PsbtError> {
        self.0
            .lock()
            .unwrap()
            .fee()
            .map(|fee| fee.to_sat())
            .map_err(PsbtError::from)
    }

    /// Combines this `Psbt` with `other` PSBT as described by BIP 174.
    ///
    /// In accordance with BIP 174 this function is commutative i.e., `A.combine(B) == B.combine(A)`
    pub(crate) fn combine(&self, other: Arc<Psbt>) -> Result<Arc<Psbt>, PsbtError> {
        let mut original_psbt = self.0.lock().unwrap().clone();
        let other_psbt = other.0.lock().unwrap().clone();
        original_psbt.combine(other_psbt)?;
        Ok(Arc::new(Psbt(Mutex::new(original_psbt))))
    }

    /// Finalizes the current PSBT and produces a result indicating
    ///
    /// whether the finalization was successful or not.
    pub(crate) fn finalize(&self) -> FinalizedPsbtResult {
        let curve = Secp256k1::verification_only();
        let finalized = self.0.lock().unwrap().clone().finalize(&curve);
        match finalized {
            Ok(psbt) => FinalizedPsbtResult {
                psbt: Arc::new(psbt.into()),
                could_finalize: true,
                errors: None,
            },
            Err((psbt, errors)) => {
                let errors = errors.into_iter().map(|e| e.into()).collect();
                FinalizedPsbtResult {
                    psbt: Arc::new(psbt.into()),
                    could_finalize: false,
                    errors: Some(errors),
                }
            }
        }
    }

    /// Write the `Psbt` to a file. Note that the file must not yet exist.
    pub(crate) fn write_to_file(&self, path: String) -> Result<(), PsbtError> {
        let file = File::create_new(path)?;
        let mut writer = BufWriter::new(file);
        let psbt = self.0.lock().unwrap();
        psbt.serialize_to_writer(&mut writer)?;
        Ok(())
    }

    /// Serializes the PSBT into a JSON string representation.
    pub(crate) fn json_serialize(&self) -> String {
        let psbt = self.0.lock().unwrap();
        serde_json::to_string(psbt.deref()).unwrap()
    }

    /// Returns the spending utxo for this PSBT's input at `input_index`.
    pub(crate) fn spend_utxo(&self, input_index: u64) -> String {
        let psbt = self.0.lock().unwrap();
        let utxo = psbt.spend_utxo(input_index as usize).unwrap();
        serde_json::to_string(&utxo).unwrap()
    }
}

impl From<BdkPsbt> for Psbt {
    fn from(psbt: BdkPsbt) -> Self {
        Psbt(Mutex::new(psbt))
    }
}

#[derive(uniffi::Record)]
pub struct FinalizedPsbtResult {
    pub psbt: Arc<Psbt>,
    pub could_finalize: bool,
    pub errors: Option<Vec<PsbtFinalizeError>>,
}

/// A transcation input.
#[derive(Debug, Clone, uniffi::Record)]
pub struct TxIn {
    /// A pointer to the previous output this input spends from.
    pub previous_output: OutPoint,
    /// The script corresponding to the `scriptPubKey`, empty in SegWit transactions.
    pub script_sig: Arc<Script>,
    /// https://bitcoin.stackexchange.com/questions/87372/what-does-the-sequence-in-a-transaction-input-mean
    pub sequence: u32,
    /// A proof for the script that authorizes the spend of the output.
    pub witness: Vec<Vec<u8>>,
}

impl From<&BdkTxIn> for TxIn {
    fn from(tx_in: &BdkTxIn) -> Self {
        TxIn {
            previous_output: OutPoint {
                txid: Arc::new(Txid(tx_in.previous_output.txid)),
                vout: tx_in.previous_output.vout,
            },
            script_sig: Arc::new(Script(tx_in.script_sig.clone())),
            sequence: tx_in.sequence.0,
            witness: tx_in.witness.to_vec(),
        }
    }
}

/// Bitcoin transaction output.
///
/// Defines new coins to be created as a result of the transaction,
/// along with spending conditions ("script", aka "output script"),
/// which an input spending it must satisfy.
///
/// An output that is not yet spent by an input is called Unspent Transaction Output ("UTXO").
#[derive(Debug, Clone, uniffi::Record)]
pub struct TxOut {
    /// The value of the output, in satoshis.
    pub value: Arc<Amount>,
    /// The script which must be satisfied for the output to be spent.
    pub script_pubkey: Arc<Script>,
}

impl From<&BdkTxOut> for TxOut {
    fn from(tx_out: &BdkTxOut) -> Self {
        TxOut {
            value: Arc::new(Amount(tx_out.value)),
            script_pubkey: Arc::new(Script(tx_out.script_pubkey.clone())),
        }
    }
}

impl From<BdkTxOut> for TxOut {
    fn from(tx_out: BdkTxOut) -> Self {
        Self {
            value: Arc::new(Amount(tx_out.value)),
            script_pubkey: Arc::new(Script(tx_out.script_pubkey)),
        }
    }
}

impl From<TxOut> for BdkTxOut {
    fn from(tx_out: TxOut) -> Self {
        Self {
            value: tx_out.value.0,
            script_pubkey: tx_out.script_pubkey.0.clone(),
        }
    }
}

/// A bitcoin Block hash
#[derive(Debug, Clone, Copy, PartialEq, Eq, std::hash::Hash, uniffi::Object)]
#[uniffi::export(Display, Eq, Hash)]
pub struct BlockHash(pub(crate) BitcoinBlockHash);

impl_hash_like!(BlockHash, BitcoinBlockHash);

/// A bitcoin transaction identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, std::hash::Hash, uniffi::Object)]
#[uniffi::export(Display, Eq, Hash)]
pub struct Txid(pub(crate) BitcoinTxid);

impl_hash_like!(Txid, BitcoinTxid);

/// A bitcoin transaction identifier, including witness data.
/// For transactions with no SegWit inputs, the `txid` will be equivalent to `wtxid`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, std::hash::Hash, uniffi::Object)]
#[uniffi::export(Display, Eq, Hash)]
pub struct Wtxid(pub(crate) BitcoinWtxid);

impl_hash_like!(Wtxid, BitcoinWtxid);

/// A collision-proof unique identifier for a descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, std::hash::Hash, uniffi::Object)]
#[uniffi::export(Display, Eq, Hash)]
pub struct DescriptorId(pub(crate) BitcoinSha256Hash);

impl_hash_like!(DescriptorId, BitcoinSha256Hash);

/// The merkle root of the merkle tree corresponding to a block's transactions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, std::hash::Hash, uniffi::Object)]
#[uniffi::export(Display, Eq, Hash)]
pub struct TxMerkleNode(pub(crate) BitcoinDoubleSha256Hash);

impl_hash_like!(TxMerkleNode, BitcoinDoubleSha256Hash);

#[cfg(test)]
mod tests {
    use crate::bitcoin::Address;
    use crate::bitcoin::Network;
    use crate::bitcoin::Psbt;

    #[test]
    fn test_is_valid_for_network() {
        // ====Docs tests====
        // https://docs.rs/bitcoin/0.29.2/src/bitcoin/util/address.rs.html#798-802

        let docs_address_testnet_str = "2N83imGV3gPwBzKJQvWJ7cRUY2SpUyU6A5e";
        let docs_address_testnet =
            Address::new(docs_address_testnet_str.to_string(), Network::Testnet).unwrap();
        assert!(
            docs_address_testnet.is_valid_for_network(Network::Testnet),
            "Address should be valid for Testnet"
        );
        assert!(
            docs_address_testnet.is_valid_for_network(Network::Signet),
            "Address should be valid for Signet"
        );
        assert!(
            docs_address_testnet.is_valid_for_network(Network::Regtest),
            "Address should be valid for Regtest"
        );

        let docs_address_mainnet_str = "32iVBEu4dxkUQk9dJbZUiBiQdmypcEyJRf";
        let docs_address_mainnet =
            Address::new(docs_address_mainnet_str.to_string(), Network::Bitcoin).unwrap();
        assert!(
            docs_address_mainnet.is_valid_for_network(Network::Bitcoin),
            "Address should be valid for Bitcoin"
        );

        // ====Bech32====

        //     | Network         | Prefix  | Address Type |
        //     |-----------------|---------|--------------|
        //     | Bitcoin Mainnet | `bc1`   | Bech32       |
        //     | Bitcoin Testnet | `tb1`   | Bech32       |
        //     | Bitcoin Signet  | `tb1`   | Bech32       |
        //     | Bitcoin Regtest | `bcrt1` | Bech32       |

        // Bech32 - Bitcoin
        // Valid for:
        // - Bitcoin
        // Not valid for:
        // - Testnet
        // - Signet
        // - Regtest
        let bitcoin_mainnet_bech32_address_str = "bc1qxhmdufsvnuaaaer4ynz88fspdsxq2h9e9cetdj";
        let bitcoin_mainnet_bech32_address = Address::new(
            bitcoin_mainnet_bech32_address_str.to_string(),
            Network::Bitcoin,
        )
        .unwrap();
        assert!(
            bitcoin_mainnet_bech32_address.is_valid_for_network(Network::Bitcoin),
            "Address should be valid for Bitcoin"
        );
        assert!(
            !bitcoin_mainnet_bech32_address.is_valid_for_network(Network::Testnet),
            "Address should not be valid for Testnet"
        );
        assert!(
            !bitcoin_mainnet_bech32_address.is_valid_for_network(Network::Signet),
            "Address should not be valid for Signet"
        );
        assert!(
            !bitcoin_mainnet_bech32_address.is_valid_for_network(Network::Regtest),
            "Address should not be valid for Regtest"
        );

        // Bech32 - Testnet
        // Valid for:
        // - Testnet
        // - Regtest
        // Not valid for:
        // - Bitcoin
        // - Regtest
        let bitcoin_testnet_bech32_address_str =
            "tb1p4nel7wkc34raczk8c4jwk5cf9d47u2284rxn98rsjrs4w3p2sheqvjmfdh";
        let bitcoin_testnet_bech32_address = Address::new(
            bitcoin_testnet_bech32_address_str.to_string(),
            Network::Testnet,
        )
        .unwrap();
        assert!(
            !bitcoin_testnet_bech32_address.is_valid_for_network(Network::Bitcoin),
            "Address should not be valid for Bitcoin"
        );
        assert!(
            bitcoin_testnet_bech32_address.is_valid_for_network(Network::Testnet),
            "Address should be valid for Testnet"
        );
        assert!(
            bitcoin_testnet_bech32_address.is_valid_for_network(Network::Signet),
            "Address should be valid for Signet"
        );
        assert!(
            !bitcoin_testnet_bech32_address.is_valid_for_network(Network::Regtest),
            "Address should not not be valid for Regtest"
        );

        // Bech32 - Signet
        // Valid for:
        // - Signet
        // - Testnet
        // Not valid for:
        // - Bitcoin
        // - Regtest
        let bitcoin_signet_bech32_address_str =
            "tb1pwzv7fv35yl7ypwj8w7al2t8apd6yf4568cs772qjwper74xqc99sk8x7tk";
        let bitcoin_signet_bech32_address = Address::new(
            bitcoin_signet_bech32_address_str.to_string(),
            Network::Signet,
        )
        .unwrap();
        assert!(
            !bitcoin_signet_bech32_address.is_valid_for_network(Network::Bitcoin),
            "Address should not be valid for Bitcoin"
        );
        assert!(
            bitcoin_signet_bech32_address.is_valid_for_network(Network::Testnet),
            "Address should be valid for Testnet"
        );
        assert!(
            bitcoin_signet_bech32_address.is_valid_for_network(Network::Signet),
            "Address should be valid for Signet"
        );
        assert!(
            !bitcoin_signet_bech32_address.is_valid_for_network(Network::Regtest),
            "Address should not not be valid for Regtest"
        );

        // Bech32 - Regtest
        // Valid for:
        // - Regtest
        // Not valid for:
        // - Bitcoin
        // - Testnet
        // - Signet
        let bitcoin_regtest_bech32_address_str = "bcrt1q39c0vrwpgfjkhasu5mfke9wnym45nydfwaeems";
        let bitcoin_regtest_bech32_address = Address::new(
            bitcoin_regtest_bech32_address_str.to_string(),
            Network::Regtest,
        )
        .unwrap();
        assert!(
            !bitcoin_regtest_bech32_address.is_valid_for_network(Network::Bitcoin),
            "Address should not be valid for Bitcoin"
        );
        assert!(
            !bitcoin_regtest_bech32_address.is_valid_for_network(Network::Testnet),
            "Address should not be valid for Testnet"
        );
        assert!(
            !bitcoin_regtest_bech32_address.is_valid_for_network(Network::Signet),
            "Address should not be valid for Signet"
        );
        assert!(
            bitcoin_regtest_bech32_address.is_valid_for_network(Network::Regtest),
            "Address should be valid for Regtest"
        );

        // ====P2PKH====

        //     | Network                            | Prefix for P2PKH | Prefix for P2SH |
        //     |------------------------------------|------------------|-----------------|
        //     | Bitcoin Mainnet                    | `1`              | `3`             |
        //     | Bitcoin Testnet, Regtest, Signet   | `m` or `n`       | `2`             |

        // P2PKH - Bitcoin
        // Valid for:
        // - Bitcoin
        // Not valid for:
        // - Testnet
        // - Regtest
        let bitcoin_mainnet_p2pkh_address_str = "1FfmbHfnpaZjKFvyi1okTjJJusN455paPH";
        let bitcoin_mainnet_p2pkh_address = Address::new(
            bitcoin_mainnet_p2pkh_address_str.to_string(),
            Network::Bitcoin,
        )
        .unwrap();
        assert!(
            bitcoin_mainnet_p2pkh_address.is_valid_for_network(Network::Bitcoin),
            "Address should be valid for Bitcoin"
        );
        assert!(
            !bitcoin_mainnet_p2pkh_address.is_valid_for_network(Network::Testnet),
            "Address should not be valid for Testnet"
        );
        assert!(
            !bitcoin_mainnet_p2pkh_address.is_valid_for_network(Network::Regtest),
            "Address should not be valid for Regtest"
        );

        // P2PKH - Testnet
        // Valid for:
        // - Testnet
        // - Regtest
        // Not valid for:
        // - Bitcoin
        let bitcoin_testnet_p2pkh_address_str = "mucFNhKMYoBQYUAEsrFVscQ1YaFQPekBpg";
        let bitcoin_testnet_p2pkh_address = Address::new(
            bitcoin_testnet_p2pkh_address_str.to_string(),
            Network::Testnet,
        )
        .unwrap();
        assert!(
            !bitcoin_testnet_p2pkh_address.is_valid_for_network(Network::Bitcoin),
            "Address should not be valid for Bitcoin"
        );
        assert!(
            bitcoin_testnet_p2pkh_address.is_valid_for_network(Network::Testnet),
            "Address should be valid for Testnet"
        );
        assert!(
            bitcoin_testnet_p2pkh_address.is_valid_for_network(Network::Regtest),
            "Address should be valid for Regtest"
        );

        // P2PKH - Regtest
        // Valid for:
        // - Testnet
        // - Regtest
        // Not valid for:
        // - Bitcoin
        let bitcoin_regtest_p2pkh_address_str = "msiGFK1PjCk8E6FXeoGkQPTscmcpyBdkgS";
        let bitcoin_regtest_p2pkh_address = Address::new(
            bitcoin_regtest_p2pkh_address_str.to_string(),
            Network::Regtest,
        )
        .unwrap();
        assert!(
            !bitcoin_regtest_p2pkh_address.is_valid_for_network(Network::Bitcoin),
            "Address should not be valid for Bitcoin"
        );
        assert!(
            bitcoin_regtest_p2pkh_address.is_valid_for_network(Network::Testnet),
            "Address should be valid for Testnet"
        );
        assert!(
            bitcoin_regtest_p2pkh_address.is_valid_for_network(Network::Regtest),
            "Address should be valid for Regtest"
        );

        // ====P2SH====

        //     | Network                            | Prefix for P2PKH | Prefix for P2SH |
        //     |------------------------------------|------------------|-----------------|
        //     | Bitcoin Mainnet                    | `1`              | `3`             |
        //     | Bitcoin Testnet, Regtest, Signet   | `m` or `n`       | `2`             |

        // P2SH - Bitcoin
        // Valid for:
        // - Bitcoin
        // Not valid for:
        // - Testnet
        // - Regtest
        let bitcoin_mainnet_p2sh_address_str = "3J98t1WpEZ73CNmQviecrnyiWrnqRhWNLy";
        let bitcoin_mainnet_p2sh_address = Address::new(
            bitcoin_mainnet_p2sh_address_str.to_string(),
            Network::Bitcoin,
        )
        .unwrap();
        assert!(
            bitcoin_mainnet_p2sh_address.is_valid_for_network(Network::Bitcoin),
            "Address should be valid for Bitcoin"
        );
        assert!(
            !bitcoin_mainnet_p2sh_address.is_valid_for_network(Network::Testnet),
            "Address should not be valid for Testnet"
        );
        assert!(
            !bitcoin_mainnet_p2sh_address.is_valid_for_network(Network::Regtest),
            "Address should not be valid for Regtest"
        );

        // P2SH - Testnet
        // Valid for:
        // - Testnet
        // - Regtest
        // Not valid for:
        // - Bitcoin
        let bitcoin_testnet_p2sh_address_str = "2NFUBBRcTJbYc1D4HSCbJhKZp6YCV4PQFpQ";
        let bitcoin_testnet_p2sh_address = Address::new(
            bitcoin_testnet_p2sh_address_str.to_string(),
            Network::Testnet,
        )
        .unwrap();
        assert!(
            !bitcoin_testnet_p2sh_address.is_valid_for_network(Network::Bitcoin),
            "Address should not be valid for Bitcoin"
        );
        assert!(
            bitcoin_testnet_p2sh_address.is_valid_for_network(Network::Testnet),
            "Address should be valid for Testnet"
        );
        assert!(
            bitcoin_testnet_p2sh_address.is_valid_for_network(Network::Regtest),
            "Address should be valid for Regtest"
        );

        // P2SH - Regtest
        // Valid for:
        // - Testnet
        // - Regtest
        // Not valid for:
        // - Bitcoin
        let bitcoin_regtest_p2sh_address_str = "2NEb8N5B9jhPUCBchz16BB7bkJk8VCZQjf3";
        let bitcoin_regtest_p2sh_address = Address::new(
            bitcoin_regtest_p2sh_address_str.to_string(),
            Network::Regtest,
        )
        .unwrap();
        assert!(
            !bitcoin_regtest_p2sh_address.is_valid_for_network(Network::Bitcoin),
            "Address should not be valid for Bitcoin"
        );
        assert!(
            bitcoin_regtest_p2sh_address.is_valid_for_network(Network::Testnet),
            "Address should be valid for Testnet"
        );
        assert!(
            bitcoin_regtest_p2sh_address.is_valid_for_network(Network::Regtest),
            "Address should be valid for Regtest"
        );
    }

    #[test]
    fn test_to_address_data() {
        // P2PKH address
        let p2pkh = Address::new(
            "1BvBMSEYstWetqTFn5Au4m4GFg7xJaNVN2".to_string(),
            Network::Bitcoin,
        )
        .unwrap();
        let p2pkh_data = p2pkh.to_address_data();
        println!("P2PKH data: {:#?}", p2pkh_data);

        // P2SH address
        let p2sh = Address::new(
            "3J98t1WpEZ73CNmQviecrnyiWrnqRhWNLy".to_string(),
            Network::Bitcoin,
        )
        .unwrap();
        let p2sh_data = p2sh.to_address_data();
        println!("P2SH data: {:#?}", p2sh_data);

        // Segwit address (P2WPKH)
        let segwit = Address::new(
            "bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4".to_string(),
            Network::Bitcoin,
        )
        .unwrap();
        let segwit_data = segwit.to_address_data();
        println!("Segwit data: {:#?}", segwit_data);
    }

    #[test]
    fn test_psbt_spend_utxo() {
        let psbt = Psbt::new("cHNidP8BAH0CAAAAAXHl8cCbj84lm1v42e54IGI6CQru/nBXwrPE3q2fiGO4AAAAAAD9////Ar4DAAAAAAAAIgAgYw/rnGd4Bifj8s7TaMgR2tal/lq+L1jVv2Sqd1mxMbJEEQAAAAAAABYAFNVpt8vHYUPZNSF6Hu07uP1YeHts4QsAAAABALUCAAAAAAEBAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAD/////BAJ+CwD/////AkAlAAAAAAAAIgAgQyrnn86L9D3vDiH959KJbPudDHc/bp6nI9E5EBLQD1YAAAAAAAAAACZqJKohqe3i9hw/cdHe/T+pmd+jaVN1XGkGiXmZYrSL69g2l06M+QEgAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAQErQCUAAAAAAAAiACBDKuefzov0Pe8OIf3n0ols+50Mdz9unqcj0TkQEtAPViICAy4V+d/Qff71zzPXxK4FWG5x+wL/Ku93y/LG5p+0rI2xSDBFAiEA9b0OdASAs0P2uhQinjN7QGP5jX/b32LcShBmny8U0RUCIBebxvCDbpchCjqLAhOMjydT80DAzokaalGzV7XVTsbiASICA1tMY+46EgxIHU18bgHnUvAAlAkMq5LfwkpOGZ97sDKRRzBEAiBpmlZwJocNEiKLxexEX0Par6UgG8a89AklTG3/z9AHlAIgQH/ybCvfKJzr2dq0+IyueDebm7FamKIJdzBYWMXRr/wBIgID+aCzK9nclwhbbN7KbIVGUQGLWZsjcaqWPxk9gFeG+FxIMEUCIQDRPBzb0i9vaUmxCcs1yz8uq4tq1mdDAYvvYn3isKEhFAIgfmeTLLzMo0mmQ23ooMnyx6iPceE8xV5CvARuJsd88tEBAQVpUiEDW0xj7joSDEgdTXxuAedS8ACUCQyrkt/CSk4Zn3uwMpEhAy4V+d/Qff71zzPXxK4FWG5x+wL/Ku93y/LG5p+0rI2xIQP5oLMr2dyXCFts3spshUZRAYtZmyNxqpY/GT2AV4b4XFOuIgYDLhX539B9/vXPM9fErgVYbnH7Av8q73fL8sbmn7SsjbEYCapBE1QAAIABAACAAAAAgAAAAAAAAAAAIgYDW0xj7joSDEgdTXxuAedS8ACUCQyrkt/CSk4Zn3uwMpEY2bvrelQAAIABAACAAAAAgAAAAAAAAAAAIgYD+aCzK9nclwhbbN7KbIVGUQGLWZsjcaqWPxk9gFeG+FwYAKVFVFQAAIABAACAAAAAgAAAAAAAAAAAAAEBaVIhA7cr8fTHOPtE+t0zM3iWJvpfPvsNaVyQ0Sar6nIe9tQXIQMm7k7OY+q+Lsge3bVACuSa9r19Js+lNuTtEhehWkpe1iECelHmzmhzDsQTDnApIcnWRz3oFR68UX1ag8jfk/SKuopTriICAnpR5s5ocw7EEw5wKSHJ1kc96BUevFF9WoPI35P0irqKGAClRVRUAACAAQAAgAAAAIABAAAAAAAAACICAybuTs5j6r4uyB7dtUAK5Jr2vX0mz6U25O0SF6FaSl7WGAmqQRNUAACAAQAAgAAAAIABAAAAAAAAACICA7cr8fTHOPtE+t0zM3iWJvpfPvsNaVyQ0Sar6nIe9tQXGNm763pUAACAAQAAgAAAAIABAAAAAAAAAAAA".to_string())
        .unwrap();
        let psbt_utxo = psbt.spend_utxo(0);

        println!("Psbt utxo: {:?}", psbt_utxo);

        assert_eq!(
            psbt_utxo,
            r#"{"value":9536,"script_pubkey":"0020432ae79fce8bf43def0e21fde7d2896cfb9d0c773f6e9ea723d1391012d00f56"}"#,
            "Psbt utxo does not match the expected value"
        );
    }
}
