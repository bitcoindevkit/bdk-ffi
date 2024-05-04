use crate::bitcoin::OutPoint;

use bdk::bitcoin::address::ParseError;
use bdk::bitcoin::bip32::Error as BdkBip32Error;
use bdk::bitcoin::psbt::PsbtParseError as BdkPsbtParseError;
use bdk::bitcoin::Network;
use bdk::chain;
use bdk::chain::tx_graph::CalculateFeeError as BdkCalculateFeeError;
use bdk::descriptor::DescriptorError as BdkDescriptorError;
use bdk::keys::bip39::Error as BdkBip39Error;
use bdk::miniscript::descriptor::DescriptorKeyParseError as BdkDescriptorKeyParseError;
use bdk::wallet::error::BuildFeeBumpError;
use bdk::wallet::error::CreateTxError as BdkCreateTxError;
use bdk::wallet::signer::SignerError as BdkSignerError;
use bdk::wallet::tx_builder::AddUtxoError;
use bdk::wallet::NewOrLoadError;
use bdk_esplora::esplora_client::{Error as BdkEsploraError, Error};
use bdk_file_store::FileError as BdkFileError;
use bitcoin_internals::hex::display::DisplayHex;

use std::convert::TryInto;

// ------------------------------------------------------------------------
// error definitions
// ------------------------------------------------------------------------

#[derive(Debug, thiserror::Error)]
pub enum AddressError {
    #[error("base58 address encoding error")]
    Base58,

    #[error("bech32 address encoding error")]
    Bech32,

    #[error("witness version conversion/parsing error: {error_message}")]
    WitnessVersion { error_message: String },

    #[error("witness program error: {error_message}")]
    WitnessProgram { error_message: String },

    #[error("an uncompressed pubkey was used where it is not allowed")]
    UncompressedPubkey,

    #[error("script size exceed 520 bytes")]
    ExcessiveScriptSize,

    #[error("script is not p2pkh, p2sh, or witness program")]
    UnrecognizedScript,

    #[error("address {address} is not valid on {required}")]
    NetworkValidation {
        required: Network,
        found: Network,
        address: String,
    },

    // This is required because the bdk::bitcoin::address::Error is non-exhaustive
    #[error("other address error")]
    OtherAddressErr,
}

#[derive(Debug, thiserror::Error)]
pub enum Bip32Error {
    #[error("Cannot derive from a hardened key")]
    CannotDeriveFromHardenedKey,

    #[error("Secp256k1 error: {error_message}")]
    Secp256k1 { error_message: String },

    #[error("Invalid child number: {child_number}")]
    InvalidChildNumber { child_number: u32 },

    #[error("Invalid format for child number")]
    InvalidChildNumberFormat,

    #[error("Invalid derivation path format")]
    InvalidDerivationPathFormat,

    #[error("Unknown version: {version}")]
    UnknownVersion { version: String },

    #[error("Wrong extended key length: {length}")]
    WrongExtendedKeyLength { length: u32 },

    #[error("Base58 error: {error_message}")]
    Base58 { error_message: String },

    #[error("Hexadecimal conversion error: {error_message}")]
    Hex { error_message: String },

    #[error("Invalid public key hex length: {length}")]
    InvalidPublicKeyHexLength { length: u32 },

    #[error("Unknown error: {error_message}")]
    UnknownError { error_message: String },
}

#[derive(Debug, thiserror::Error)]
pub enum Bip39Error {
    #[error("the word count {word_count} is not supported")]
    BadWordCount { word_count: u64 },

    #[error("unknown word at index {index}")]
    UnknownWord { index: u64 },

    #[error("entropy bit count {bit_count} is invalid")]
    BadEntropyBitCount { bit_count: u64 },

    #[error("checksum is invalid")]
    InvalidChecksum,

    #[error("ambiguous languages detected: {languages}")]
    AmbiguousLanguages { languages: String },
}

#[derive(Debug, thiserror::Error)]
pub enum CalculateFeeError {
    #[error("missing transaction output: {out_points:?}")]
    MissingTxOut { out_points: Vec<OutPoint> },

    #[error("negative fee value: {fee}")]
    NegativeFee { fee: i64 },
}

#[derive(Debug, thiserror::Error)]
pub enum CannotConnectError {
    #[error("cannot include height: {height}")]
    Include { height: u32 },
}

#[derive(Debug, thiserror::Error)]
pub enum CreateTxError {
    #[error("Descriptor error: {error_message}")]
    Descriptor { error_message: String },

    #[error("Persistence failure: {error_message}")]
    Persist { error_message: String },

    #[error("Policy error: {error_message}")]
    Policy { error_message: String },

    #[error("Spending policy required for {kind}")]
    SpendingPolicyRequired { kind: String },

    #[error("Unsupported version 0")]
    Version0,

    #[error("Unsupported version 1 with CSV")]
    Version1Csv,

    #[error("Lock time conflict: requested {requested}, but required {required}")]
    LockTime { requested: String, required: String },

    #[error("Transaction requires RBF sequence number")]
    RbfSequence,

    #[error("RBF sequence: {rbf}, CSV sequence: {csv}")]
    RbfSequenceCsv { rbf: String, csv: String },

    #[error("Fee too low: {required} sat required")]
    FeeTooLow { required: u64 },

    #[error("Fee rate too low: {required}")]
    FeeRateTooLow { required: String },

    #[error("No UTXOs selected for the transaction")]
    NoUtxosSelected,

    #[error("Output value below dust limit at index {index}")]
    OutputBelowDustLimit { index: u64 },

    #[error("Change policy descriptor error")]
    ChangePolicyDescriptor,

    #[error("Coin selection failed: {error_message}")]
    CoinSelection { error_message: String },

    #[error("Insufficient funds: needed {needed} sat, available {available} sat")]
    InsufficientFunds { needed: u64, available: u64 },

    #[error("Transaction has no recipients")]
    NoRecipients,

    #[error("PSBT creation error: {error_message}")]
    Psbt { error_message: String },

    #[error("Missing key origin for: {key}")]
    MissingKeyOrigin { key: String },

    #[error("Reference to an unknown UTXO: {outpoint}")]
    UnknownUtxo { outpoint: String },

    #[error("Missing non-witness UTXO for outpoint: {outpoint}")]
    MissingNonWitnessUtxo { outpoint: String },

    #[error("Miniscript PSBT error: {error_message}")]
    MiniscriptPsbt { error_message: String },
}

#[derive(Debug, thiserror::Error)]
pub enum DescriptorError {
    #[error("invalid hd key path")]
    InvalidHdKeyPath,

    #[error("the provided descriptor doesn't match its checksum")]
    InvalidDescriptorChecksum,

    #[error("the descriptor contains hardened derivation steps on public extended keys")]
    HardenedDerivationXpub,

    #[error("the descriptor contains multipath keys, which are not supported yet")]
    MultiPath,

    #[error("key error: {error_message}")]
    Key { error_message: String },

    #[error("policy error: {error_message}")]
    Policy { error_message: String },

    #[error("invalid descriptor character: {char}")]
    InvalidDescriptorCharacter { char: String },

    #[error("BIP32 error: {error_message}")]
    Bip32 { error_message: String },

    #[error("Base58 error: {error_message}")]
    Base58 { error_message: String },

    #[error("Key-related error: {error_message}")]
    Pk { error_message: String },

    #[error("Miniscript error: {error_message}")]
    Miniscript { error_message: String },

    #[error("Hex decoding error: {error_message}")]
    Hex { error_message: String },
}

#[derive(Debug, thiserror::Error)]
pub enum DescriptorKeyError {
    #[error("error parsing descriptor key: {error_message}")]
    Parse { error_message: String },

    #[error("error invalid key type")]
    InvalidKeyType,

    #[error("error bip 32 related: {error_message}")]
    Bip32 { error_message: String },
}

#[derive(Debug, thiserror::Error)]
pub enum EsploraError {
    #[error("minreq error: {error_message}")]
    Minreq { error_message: String },

    #[error("http error with status code {status} and message {error_message}")]
    HttpResponse { status: u16, error_message: String },

    #[error("parsing error: {error_message}")]
    Parsing { error_message: String },

    #[error("Invalid status code, unable to convert to u16: {error_message}")]
    StatusCode { error_message: String },

    #[error("bitcoin encoding error: {error_message}")]
    BitcoinEncoding { error_message: String },

    #[error("invalid hex data returned: {error_message}")]
    HexToArray { error_message: String },

    #[error("invalid hex data returned: {error_message}")]
    HexToBytes { error_message: String },

    #[error("transaction not found")]
    TransactionNotFound,

    #[error("header height {height} not found")]
    HeaderHeightNotFound { height: u32 },

    #[error("header hash not found")]
    HeaderHashNotFound,

    #[error("invalid HTTP header name: {name}")]
    InvalidHttpHeaderName { name: String },

    #[error("invalid HTTP header value: {value}")]
    InvalidHttpHeaderValue { value: String },
}

#[derive(Debug, thiserror::Error)]
pub enum ExtractTxError {
    #[error("an absurdly high fee rate of {fee_rate} sat/vbyte")]
    AbsurdFeeRate { fee_rate: u64 },

    #[error("one of the inputs lacked value information (witness_utxo or non_witness_utxo)")]
    MissingInputValue,

    #[error("transaction would be invalid due to output value being greater than input value")]
    SendingTooMuch,

    #[error(
        "this error is required because the bdk::bitcoin::psbt::ExtractTxError is non-exhaustive"
    )]
    OtherExtractTxErr,
}

#[derive(Debug, thiserror::Error)]
pub enum FeeRateError {
    #[error("arithmetic overflow on feerate")]
    ArithmeticOverflow,
}

#[derive(Debug, thiserror::Error)]
pub enum PersistenceError {
    #[error("writing to persistence error: {error_message}")]
    Write { error_message: String },
}

#[derive(Debug, thiserror::Error)]
pub enum PsbtParseError {
    #[error("error in internal PSBT data structure: {error_message}")]
    PsbtEncoding { error_message: String },

    #[error("error in PSBT base64 encoding: {error_message}")]
    Base64Encoding { error_message: String },
}

#[derive(Debug, thiserror::Error)]
pub enum SignerError {
    #[error("Missing key for signing")]
    MissingKey,

    #[error("Invalid key provided")]
    InvalidKey,

    #[error("User canceled operation")]
    UserCanceled,

    #[error("Input index out of range")]
    InputIndexOutOfRange,

    #[error("Missing non-witness UTXO information")]
    MissingNonWitnessUtxo,

    #[error("Invalid non-witness UTXO information provided")]
    InvalidNonWitnessUtxo,

    #[error("Missing witness UTXO")]
    MissingWitnessUtxo,

    #[error("Missing witness script")]
    MissingWitnessScript,

    #[error("Missing HD keypath")]
    MissingHdKeypath,

    #[error("Non-standard sighash type used")]
    NonStandardSighash,

    #[error("Invalid sighash type provided")]
    InvalidSighash,

    #[error("Error with sighash computation: {error_message}")]
    SighashError { error_message: String },

    #[error("Miniscript Psbt error: {error_message}")]
    MiniscriptPsbt { error_message: String },

    #[error("External error: {error_message}")]
    External { error_message: String },
}

#[derive(Debug, thiserror::Error)]
pub enum TransactionError {
    #[error("IO error")]
    Io,

    #[error("allocation of oversized vector")]
    OversizedVectorAllocation,

    #[error("invalid checksum: expected={expected} actual={actual}")]
    InvalidChecksum { expected: String, actual: String },

    #[error("non-minimal varint")]
    NonMinimalVarInt,

    #[error("parse failed")]
    ParseFailed,

    #[error("unsupported segwit version: {flag}")]
    UnsupportedSegwitFlag { flag: u8 },

    // This is required because the bdk::bitcoin::consensus::encode::Error is non-exhaustive
    #[error("other transaction error")]
    OtherTransactionErr,
}

#[derive(Debug, thiserror::Error)]
pub enum TxidParseError {
    #[error("invalid txid: {txid}")]
    InvalidTxid { txid: String },
}

// This error combines the Rust bdk::wallet::NewOrLoadError and bdk_file_store::FileError
#[derive(Debug, thiserror::Error)]
pub enum WalletCreationError {
    #[error("io error trying to read file: {error_message}")]
    Io { error_message: String },

    #[error("file has invalid magic bytes: expected={expected:?} got={got:?}")]
    InvalidMagicBytes { got: Vec<u8>, expected: Vec<u8> },

    #[error("error with descriptor")]
    Descriptor,

    #[error("failed to either write to or load from persistence, {error_message}")]
    Persist { error_message: String },

    #[error("wallet is not initialized, persistence backend is empty")]
    NotInitialized,

    #[error("loaded genesis hash {got} does not match the expected one {expected}")]
    LoadedGenesisDoesNotMatch { expected: String, got: String },

    #[error("loaded network type is not {expected}, got {got:?}")]
    LoadedNetworkDoesNotMatch {
        expected: Network,
        got: Option<Network>,
    },
}

// ------------------------------------------------------------------------
// error conversions
// ------------------------------------------------------------------------

impl From<bdk::bitcoin::address::Error> for AddressError {
    fn from(error: bdk::bitcoin::address::Error) -> Self {
        match error {
            bdk::bitcoin::address::Error::WitnessVersion(error_message) => {
                AddressError::WitnessVersion {
                    error_message: error_message.to_string(),
                }
            }
            bdk::bitcoin::address::Error::WitnessProgram(e) => AddressError::WitnessProgram {
                error_message: e.to_string(),
            },
            bdk::bitcoin::address::Error::UncompressedPubkey => AddressError::UncompressedPubkey,
            bdk::bitcoin::address::Error::ExcessiveScriptSize => AddressError::ExcessiveScriptSize,
            bdk::bitcoin::address::Error::UnrecognizedScript => AddressError::UnrecognizedScript,
            bdk::bitcoin::address::Error::NetworkValidation {
                required,
                found,
                address,
            } => AddressError::NetworkValidation {
                required,
                found,
                address: format!("{:?}", address),
            },
            _ => AddressError::OtherAddressErr,
        }
    }
}

impl From<ParseError> for AddressError {
    fn from(error: ParseError) -> Self {
        match error {
            ParseError::Base58(_) => AddressError::Base58,
            ParseError::Bech32(_) => AddressError::Bech32,
            ParseError::WitnessVersion(e) => AddressError::WitnessVersion {
                error_message: e.to_string(),
            },
            ParseError::WitnessProgram(e) => AddressError::WitnessProgram {
                error_message: e.to_string(),
            },
            _ => AddressError::OtherAddressErr,
        }
    }
}

impl From<BdkBip32Error> for Bip32Error {
    fn from(error: BdkBip32Error) -> Self {
        match error {
            BdkBip32Error::CannotDeriveFromHardenedKey => Bip32Error::CannotDeriveFromHardenedKey,
            BdkBip32Error::Secp256k1(e) => Bip32Error::Secp256k1 {
                error_message: e.to_string(),
            },
            BdkBip32Error::InvalidChildNumber(num) => {
                Bip32Error::InvalidChildNumber { child_number: num }
            }
            BdkBip32Error::InvalidChildNumberFormat => Bip32Error::InvalidChildNumberFormat,
            BdkBip32Error::InvalidDerivationPathFormat => Bip32Error::InvalidDerivationPathFormat,
            BdkBip32Error::UnknownVersion(bytes) => Bip32Error::UnknownVersion {
                version: bytes.to_lower_hex_string(),
            },
            BdkBip32Error::WrongExtendedKeyLength(len) => {
                Bip32Error::WrongExtendedKeyLength { length: len as u32 }
            }
            BdkBip32Error::Base58(e) => Bip32Error::Base58 {
                error_message: e.to_string(),
            },
            BdkBip32Error::Hex(e) => Bip32Error::Hex {
                error_message: e.to_string(),
            },
            BdkBip32Error::InvalidPublicKeyHexLength(len) => {
                Bip32Error::InvalidPublicKeyHexLength { length: len as u32 }
            }
            _ => Bip32Error::UnknownError {
                error_message: format!("Unhandled error: {:?}", error),
            },
        }
    }
}

impl From<BdkBip39Error> for Bip39Error {
    fn from(error: BdkBip39Error) -> Self {
        match error {
            BdkBip39Error::BadWordCount(word_count) => Bip39Error::BadWordCount {
                word_count: word_count.try_into().expect("word count exceeds u64"),
            },
            BdkBip39Error::UnknownWord(index) => Bip39Error::UnknownWord {
                index: index.try_into().expect("index exceeds u64"),
            },
            BdkBip39Error::BadEntropyBitCount(bit_count) => Bip39Error::BadEntropyBitCount {
                bit_count: bit_count.try_into().expect("bit count exceeds u64"),
            },
            BdkBip39Error::InvalidChecksum => Bip39Error::InvalidChecksum,
            BdkBip39Error::AmbiguousLanguages(info) => Bip39Error::AmbiguousLanguages {
                languages: format!("{:?}", info),
            },
        }
    }
}

impl From<BdkCalculateFeeError> for CalculateFeeError {
    fn from(error: BdkCalculateFeeError) -> Self {
        match error {
            BdkCalculateFeeError::MissingTxOut(out_points) => CalculateFeeError::MissingTxOut {
                out_points: out_points.iter().map(|op| op.into()).collect(),
            },
            BdkCalculateFeeError::NegativeFee(fee) => CalculateFeeError::NegativeFee { fee },
        }
    }
}

impl From<chain::local_chain::CannotConnectError> for CannotConnectError {
    fn from(error: chain::local_chain::CannotConnectError) -> Self {
        CannotConnectError::Include {
            height: error.try_include_height,
        }
    }
}

impl From<BdkCreateTxError> for CreateTxError {
    fn from(error: BdkCreateTxError) -> Self {
        match error {
            BdkCreateTxError::Descriptor(e) => CreateTxError::Descriptor {
                error_message: e.to_string(),
            },
            BdkCreateTxError::Persist(e) => CreateTxError::Persist {
                error_message: e.to_string(),
            },
            BdkCreateTxError::Policy(e) => CreateTxError::Policy {
                error_message: e.to_string(),
            },
            BdkCreateTxError::SpendingPolicyRequired(kind) => {
                CreateTxError::SpendingPolicyRequired {
                    kind: format!("{:?}", kind),
                }
            }
            BdkCreateTxError::Version0 => CreateTxError::Version0,
            BdkCreateTxError::Version1Csv => CreateTxError::Version1Csv,
            BdkCreateTxError::LockTime {
                requested,
                required,
            } => CreateTxError::LockTime {
                requested: requested.to_string(),
                required: required.to_string(),
            },
            BdkCreateTxError::RbfSequence => CreateTxError::RbfSequence,
            BdkCreateTxError::RbfSequenceCsv { rbf, csv } => CreateTxError::RbfSequenceCsv {
                rbf: rbf.to_string(),
                csv: csv.to_string(),
            },
            BdkCreateTxError::FeeTooLow { required } => CreateTxError::FeeTooLow { required },
            BdkCreateTxError::FeeRateTooLow { required } => CreateTxError::FeeRateTooLow {
                required: required.to_string(),
            },
            BdkCreateTxError::NoUtxosSelected => CreateTxError::NoUtxosSelected,
            BdkCreateTxError::OutputBelowDustLimit(index) => CreateTxError::OutputBelowDustLimit {
                index: index as u64,
            },
            BdkCreateTxError::ChangePolicyDescriptor => CreateTxError::ChangePolicyDescriptor,
            BdkCreateTxError::CoinSelection(e) => CreateTxError::CoinSelection {
                error_message: e.to_string(),
            },
            BdkCreateTxError::InsufficientFunds { needed, available } => {
                CreateTxError::InsufficientFunds { needed, available }
            }
            BdkCreateTxError::NoRecipients => CreateTxError::NoRecipients,
            BdkCreateTxError::Psbt(e) => CreateTxError::Psbt {
                error_message: e.to_string(),
            },
            BdkCreateTxError::MissingKeyOrigin(key) => CreateTxError::MissingKeyOrigin { key },
            BdkCreateTxError::UnknownUtxo => CreateTxError::UnknownUtxo {
                outpoint: "Unknown".to_string(),
            },
            BdkCreateTxError::MissingNonWitnessUtxo(outpoint) => {
                CreateTxError::MissingNonWitnessUtxo {
                    outpoint: outpoint.to_string(),
                }
            }
            BdkCreateTxError::MiniscriptPsbt(e) => CreateTxError::MiniscriptPsbt {
                error_message: e.to_string(),
            },
        }
    }
}

impl From<AddUtxoError> for CreateTxError {
    fn from(error: AddUtxoError) -> Self {
        match error {
            AddUtxoError::UnknownUtxo(outpoint) => CreateTxError::UnknownUtxo {
                outpoint: outpoint.to_string(),
            },
        }
    }
}

impl From<BuildFeeBumpError> for CreateTxError {
    fn from(error: BuildFeeBumpError) -> Self {
        match error {
            BuildFeeBumpError::UnknownUtxo(outpoint) => CreateTxError::UnknownUtxo {
                outpoint: outpoint.to_string(),
            },
            BuildFeeBumpError::TransactionNotFound(txid) => CreateTxError::UnknownUtxo {
                outpoint: txid.to_string(),
            },
            BuildFeeBumpError::TransactionConfirmed(txid) => CreateTxError::UnknownUtxo {
                outpoint: txid.to_string(),
            },
            BuildFeeBumpError::IrreplaceableTransaction(txid) => CreateTxError::UnknownUtxo {
                outpoint: txid.to_string(),
            },
            BuildFeeBumpError::FeeRateUnavailable => CreateTxError::FeeRateTooLow {
                required: "unavailable".to_string(),
            },
        }
    }
}

impl From<BdkDescriptorError> for DescriptorError {
    fn from(error: BdkDescriptorError) -> Self {
        match error {
            BdkDescriptorError::InvalidHdKeyPath => DescriptorError::InvalidHdKeyPath,
            BdkDescriptorError::InvalidDescriptorChecksum => {
                DescriptorError::InvalidDescriptorChecksum
            }
            BdkDescriptorError::HardenedDerivationXpub => DescriptorError::HardenedDerivationXpub,
            BdkDescriptorError::MultiPath => DescriptorError::MultiPath,
            BdkDescriptorError::Key(e) => DescriptorError::Key {
                error_message: e.to_string(),
            },
            BdkDescriptorError::Policy(e) => DescriptorError::Policy {
                error_message: e.to_string(),
            },
            BdkDescriptorError::InvalidDescriptorCharacter(char) => {
                DescriptorError::InvalidDescriptorCharacter {
                    char: char.to_string(),
                }
            }
            BdkDescriptorError::Bip32(e) => DescriptorError::Bip32 {
                error_message: e.to_string(),
            },
            BdkDescriptorError::Base58(e) => DescriptorError::Base58 {
                error_message: e.to_string(),
            },
            BdkDescriptorError::Pk(e) => DescriptorError::Pk {
                error_message: e.to_string(),
            },
            BdkDescriptorError::Miniscript(e) => DescriptorError::Miniscript {
                error_message: e.to_string(),
            },
            BdkDescriptorError::Hex(e) => DescriptorError::Hex {
                error_message: e.to_string(),
            },
        }
    }
}

impl From<BdkDescriptorKeyParseError> for DescriptorKeyError {
    fn from(err: BdkDescriptorKeyParseError) -> DescriptorKeyError {
        DescriptorKeyError::Parse {
            error_message: format!("DescriptorKeyError error: {:?}", err),
        }
    }
}

impl From<bdk::bitcoin::bip32::Error> for DescriptorKeyError {
    fn from(error: bdk::bitcoin::bip32::Error) -> DescriptorKeyError {
        DescriptorKeyError::Bip32 {
            error_message: format!("BIP32 derivation error: {:?}", error),
        }
    }
}

impl From<BdkEsploraError> for EsploraError {
    fn from(error: BdkEsploraError) -> Self {
        match error {
            BdkEsploraError::Minreq(e) => EsploraError::Minreq {
                error_message: e.to_string(),
            },
            BdkEsploraError::HttpResponse { status, message } => EsploraError::HttpResponse {
                status,
                error_message: message,
            },
            BdkEsploraError::Parsing(e) => EsploraError::Parsing {
                error_message: e.to_string(),
            },
            Error::StatusCode(e) => EsploraError::StatusCode {
                error_message: e.to_string(),
            },
            BdkEsploraError::BitcoinEncoding(e) => EsploraError::BitcoinEncoding {
                error_message: e.to_string(),
            },
            BdkEsploraError::HexToArray(e) => EsploraError::HexToArray {
                error_message: e.to_string(),
            },
            BdkEsploraError::HexToBytes(e) => EsploraError::HexToBytes {
                error_message: e.to_string(),
            },
            BdkEsploraError::TransactionNotFound(_) => EsploraError::TransactionNotFound,
            BdkEsploraError::HeaderHeightNotFound(height) => {
                EsploraError::HeaderHeightNotFound { height }
            }
            BdkEsploraError::HeaderHashNotFound(_) => EsploraError::HeaderHashNotFound,
            Error::InvalidHttpHeaderName(name) => EsploraError::InvalidHttpHeaderName { name },
            BdkEsploraError::InvalidHttpHeaderValue(value) => {
                EsploraError::InvalidHttpHeaderValue { value }
            }
        }
    }
}

impl From<Box<BdkEsploraError>> for EsploraError {
    fn from(error: Box<BdkEsploraError>) -> Self {
        match *error {
            BdkEsploraError::Minreq(e) => EsploraError::Minreq {
                error_message: e.to_string(),
            },
            BdkEsploraError::HttpResponse { status, message } => EsploraError::HttpResponse {
                status,
                error_message: message,
            },
            BdkEsploraError::Parsing(e) => EsploraError::Parsing {
                error_message: e.to_string(),
            },
            Error::StatusCode(e) => EsploraError::StatusCode {
                error_message: e.to_string(),
            },
            BdkEsploraError::BitcoinEncoding(e) => EsploraError::BitcoinEncoding {
                error_message: e.to_string(),
            },
            BdkEsploraError::HexToArray(e) => EsploraError::HexToArray {
                error_message: e.to_string(),
            },
            BdkEsploraError::HexToBytes(e) => EsploraError::HexToBytes {
                error_message: e.to_string(),
            },
            BdkEsploraError::TransactionNotFound(_) => EsploraError::TransactionNotFound,
            BdkEsploraError::HeaderHeightNotFound(height) => {
                EsploraError::HeaderHeightNotFound { height }
            }
            BdkEsploraError::HeaderHashNotFound(_) => EsploraError::HeaderHashNotFound,
            Error::InvalidHttpHeaderName(name) => EsploraError::InvalidHttpHeaderName { name },
            BdkEsploraError::InvalidHttpHeaderValue(value) => {
                EsploraError::InvalidHttpHeaderValue { value }
            }
        }
    }
}

impl From<bdk::bitcoin::psbt::ExtractTxError> for ExtractTxError {
    fn from(error: bdk::bitcoin::psbt::ExtractTxError) -> Self {
        match error {
            bdk::bitcoin::psbt::ExtractTxError::AbsurdFeeRate { fee_rate, .. } => {
                let sat_per_vbyte = fee_rate.to_sat_per_vb_ceil();
                ExtractTxError::AbsurdFeeRate {
                    fee_rate: sat_per_vbyte,
                }
            }
            bdk::bitcoin::psbt::ExtractTxError::MissingInputValue { .. } => {
                ExtractTxError::MissingInputValue
            }
            bdk::bitcoin::psbt::ExtractTxError::SendingTooMuch { .. } => {
                ExtractTxError::SendingTooMuch
            }
            _ => ExtractTxError::OtherExtractTxErr,
        }
    }
}

impl From<std::io::Error> for PersistenceError {
    fn from(error: std::io::Error) -> Self {
        PersistenceError::Write {
            error_message: error.to_string(),
        }
    }
}

impl From<BdkPsbtParseError> for PsbtParseError {
    fn from(error: BdkPsbtParseError) -> Self {
        match error {
            BdkPsbtParseError::PsbtEncoding(e) => PsbtParseError::PsbtEncoding {
                error_message: e.to_string(),
            },
            BdkPsbtParseError::Base64Encoding(e) => PsbtParseError::Base64Encoding {
                error_message: e.to_string(),
            },
            _ => {
                unreachable!("this is required because of the non-exhaustive enum in rust-bitcoin")
            }
        }
    }
}

impl From<BdkSignerError> for SignerError {
    fn from(error: BdkSignerError) -> Self {
        match error {
            BdkSignerError::MissingKey => SignerError::MissingKey,
            BdkSignerError::InvalidKey => SignerError::InvalidKey,
            BdkSignerError::UserCanceled => SignerError::UserCanceled,
            BdkSignerError::InputIndexOutOfRange => SignerError::InputIndexOutOfRange,
            BdkSignerError::MissingNonWitnessUtxo => SignerError::MissingNonWitnessUtxo,
            BdkSignerError::InvalidNonWitnessUtxo => SignerError::InvalidNonWitnessUtxo,
            BdkSignerError::MissingWitnessUtxo => SignerError::MissingWitnessUtxo,
            BdkSignerError::MissingWitnessScript => SignerError::MissingWitnessScript,
            BdkSignerError::MissingHdKeypath => SignerError::MissingHdKeypath,
            BdkSignerError::NonStandardSighash => SignerError::NonStandardSighash,
            BdkSignerError::InvalidSighash => SignerError::InvalidSighash,
            BdkSignerError::SighashError(e) => SignerError::SighashError {
                error_message: e.to_string(),
            },
            BdkSignerError::MiniscriptPsbt(e) => SignerError::MiniscriptPsbt {
                error_message: format!("{:?}", e),
            },
            BdkSignerError::External(e) => SignerError::External { error_message: e },
        }
    }
}

impl From<bdk::bitcoin::consensus::encode::Error> for TransactionError {
    fn from(error: bdk::bitcoin::consensus::encode::Error) -> Self {
        match error {
            bdk::bitcoin::consensus::encode::Error::Io(_) => TransactionError::Io,
            bdk::bitcoin::consensus::encode::Error::OversizedVectorAllocation { .. } => {
                TransactionError::OversizedVectorAllocation
            }
            bdk::bitcoin::consensus::encode::Error::InvalidChecksum { expected, actual } => {
                TransactionError::InvalidChecksum {
                    expected: DisplayHex::to_lower_hex_string(&expected),
                    actual: DisplayHex::to_lower_hex_string(&actual),
                }
            }
            bdk::bitcoin::consensus::encode::Error::NonMinimalVarInt => {
                TransactionError::NonMinimalVarInt
            }
            bdk::bitcoin::consensus::encode::Error::ParseFailed(_) => TransactionError::ParseFailed,
            bdk::bitcoin::consensus::encode::Error::UnsupportedSegwitFlag(flag) => {
                TransactionError::UnsupportedSegwitFlag { flag }
            }
            _ => TransactionError::OtherTransactionErr,
        }
    }
}

impl From<BdkFileError> for WalletCreationError {
    fn from(error: BdkFileError) -> Self {
        match error {
            BdkFileError::Io(e) => WalletCreationError::Io {
                error_message: e.to_string(),
            },
            BdkFileError::InvalidMagicBytes { got, expected } => {
                WalletCreationError::InvalidMagicBytes { got, expected }
            }
        }
    }
}

impl From<NewOrLoadError> for WalletCreationError {
    fn from(error: NewOrLoadError) -> Self {
        match error {
            NewOrLoadError::Descriptor(_) => WalletCreationError::Descriptor,
            NewOrLoadError::Persist(e) => WalletCreationError::Persist {
                error_message: e.to_string(),
            },
            NewOrLoadError::NotInitialized => WalletCreationError::NotInitialized,
            NewOrLoadError::LoadedGenesisDoesNotMatch { expected, got } => {
                WalletCreationError::LoadedGenesisDoesNotMatch {
                    expected: expected.to_string(),
                    got: format!("{:?}", got),
                }
            }
            NewOrLoadError::LoadedNetworkDoesNotMatch { expected, got } => {
                WalletCreationError::LoadedNetworkDoesNotMatch { expected, got }
            }
        }
    }
}

// ------------------------------------------------------------------------
// error tests
// ------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use crate::error::{
        AddressError, Bip32Error, Bip39Error, CannotConnectError, CreateTxError, DescriptorError,
        DescriptorKeyError, EsploraError, ExtractTxError, FeeRateError, PersistenceError,
        PsbtParseError, TransactionError, TxidParseError, WalletCreationError,
    };
    use crate::CalculateFeeError;
    use crate::OutPoint;
    use crate::SignerError;
    use bdk::bitcoin::Network;

    #[test]
    fn test_error_address() {
        let cases = vec![
            (AddressError::Base58, "base58 address encoding error"),
            (AddressError::Bech32, "bech32 address encoding error"),
            (
                AddressError::WitnessVersion {
                    error_message: "version error".to_string(),
                },
                "witness version conversion/parsing error: version error",
            ),
            (
                AddressError::WitnessProgram {
                    error_message: "program error".to_string(),
                },
                "witness program error: program error",
            ),
            (
                AddressError::UncompressedPubkey,
                "an uncompressed pubkey was used where it is not allowed",
            ),
            (
                AddressError::ExcessiveScriptSize,
                "script size exceed 520 bytes",
            ),
            (
                AddressError::UnrecognizedScript,
                "script is not p2pkh, p2sh, or witness program",
            ),
            (
                AddressError::NetworkValidation {
                    required: Network::Bitcoin,
                    found: Network::Testnet,
                    address: "1BitcoinEaterAddressDontSendf59kuE".to_string(),
                },
                "address 1BitcoinEaterAddressDontSendf59kuE is not valid on bitcoin",
            ),
            (AddressError::OtherAddressErr, "other address error"),
        ];

        for (error, expected_message) in cases {
            assert_eq!(error.to_string(), expected_message);
        }
    }

    #[test]
    fn test_error_bip32() {
        let cases = vec![
            (
                Bip32Error::CannotDeriveFromHardenedKey,
                "Cannot derive from a hardened key",
            ),
            (
                Bip32Error::Secp256k1 {
                    error_message: "failure".to_string(),
                },
                "Secp256k1 error: failure",
            ),
            (
                Bip32Error::InvalidChildNumber { child_number: 123 },
                "Invalid child number: 123",
            ),
            (
                Bip32Error::InvalidChildNumberFormat,
                "Invalid format for child number",
            ),
            (
                Bip32Error::InvalidDerivationPathFormat,
                "Invalid derivation path format",
            ),
            (
                Bip32Error::UnknownVersion {
                    version: "0x123".to_string(),
                },
                "Unknown version: 0x123",
            ),
            (
                Bip32Error::WrongExtendedKeyLength { length: 512 },
                "Wrong extended key length: 512",
            ),
            (
                Bip32Error::Base58 {
                    error_message: "error".to_string(),
                },
                "Base58 error: error",
            ),
            (
                Bip32Error::Hex {
                    error_message: "error".to_string(),
                },
                "Hexadecimal conversion error: error",
            ),
            (
                Bip32Error::InvalidPublicKeyHexLength { length: 66 },
                "Invalid public key hex length: 66",
            ),
            (
                Bip32Error::UnknownError {
                    error_message: "mystery".to_string(),
                },
                "Unknown error: mystery",
            ),
        ];

        for (error, expected_message) in cases {
            assert_eq!(error.to_string(), expected_message);
        }
    }

    #[test]
    fn test_error_bip39() {
        let cases = vec![
            (
                Bip39Error::BadWordCount { word_count: 15 },
                "the word count 15 is not supported",
            ),
            (
                Bip39Error::UnknownWord { index: 102 },
                "unknown word at index 102",
            ),
            (
                Bip39Error::BadEntropyBitCount { bit_count: 128 },
                "entropy bit count 128 is invalid",
            ),
            (Bip39Error::InvalidChecksum, "checksum is invalid"),
            (
                Bip39Error::AmbiguousLanguages {
                    languages: "English, Spanish".to_string(),
                },
                "ambiguous languages detected: English, Spanish",
            ),
        ];

        for (error, expected_message) in cases {
            assert_eq!(error.to_string(), expected_message);
        }
    }

    #[test]
    fn test_error_calculate_fee() {
        let out_points: Vec<OutPoint> = vec![
            OutPoint {
                txid: "0000000000000000000000000000000000000000000000000000000000000001"
                    .to_string(),
                vout: 0,
            },
            OutPoint {
                txid: "0000000000000000000000000000000000000000000000000000000000000002"
                    .to_string(),
                vout: 1,
            },
        ];

        let cases = vec![
            (
                CalculateFeeError::MissingTxOut {
                    out_points: out_points.clone(),
                },
                format!(
                    "missing transaction output: [{:?}, {:?}]",
                    out_points[0], out_points[1]
                ),
            ),
            (
                CalculateFeeError::NegativeFee { fee: -100 },
                "negative fee value: -100".to_string(),
            ),
        ];

        for (error, expected_message) in cases {
            assert_eq!(error.to_string(), expected_message);
        }
    }

    #[test]
    fn test_error_cannot_connect() {
        let error = CannotConnectError::Include { height: 42 };

        assert_eq!(format!("{}", error), "cannot include height: 42");
    }

    #[test]
    fn test_error_create_tx() {
        let cases = vec![
            (
                CreateTxError::Descriptor {
                    error_message: "Descriptor failure".to_string(),
                },
                "Descriptor error: Descriptor failure",
            ),
            (
                CreateTxError::Persist {
                    error_message: "Persistence error".to_string(),
                },
                "Persistence failure: Persistence error",
            ),
            (
                CreateTxError::Policy {
                    error_message: "Policy violation".to_string(),
                },
                "Policy error: Policy violation",
            ),
            (
                CreateTxError::SpendingPolicyRequired {
                    kind: "multisig".to_string(),
                },
                "Spending policy required for multisig",
            ),
            (CreateTxError::Version0, "Unsupported version 0"),
            (CreateTxError::Version1Csv, "Unsupported version 1 with CSV"),
            (
                CreateTxError::LockTime {
                    requested: "today".to_string(),
                    required: "tomorrow".to_string(),
                },
                "Lock time conflict: requested today, but required tomorrow",
            ),
            (
                CreateTxError::RbfSequence,
                "Transaction requires RBF sequence number",
            ),
            (
                CreateTxError::RbfSequenceCsv {
                    rbf: "123".to_string(),
                    csv: "456".to_string(),
                },
                "RBF sequence: 123, CSV sequence: 456",
            ),
            (
                CreateTxError::FeeTooLow { required: 1000 },
                "Fee too low: 1000 sat required",
            ),
            (
                CreateTxError::FeeRateTooLow {
                    required: "5 sat/vB".to_string(),
                },
                "Fee rate too low: 5 sat/vB",
            ),
            (
                CreateTxError::NoUtxosSelected,
                "No UTXOs selected for the transaction",
            ),
            (
                CreateTxError::OutputBelowDustLimit { index: 2 },
                "Output value below dust limit at index 2",
            ),
            (
                CreateTxError::ChangePolicyDescriptor,
                "Change policy descriptor error",
            ),
            (
                CreateTxError::CoinSelection {
                    error_message: "No suitable outputs".to_string(),
                },
                "Coin selection failed: No suitable outputs",
            ),
            (
                CreateTxError::InsufficientFunds {
                    needed: 5000,
                    available: 3000,
                },
                "Insufficient funds: needed 5000 sat, available 3000 sat",
            ),
            (CreateTxError::NoRecipients, "Transaction has no recipients"),
            (
                CreateTxError::Psbt {
                    error_message: "PSBT creation failed".to_string(),
                },
                "PSBT creation error: PSBT creation failed",
            ),
            (
                CreateTxError::MissingKeyOrigin {
                    key: "xpub...".to_string(),
                },
                "Missing key origin for: xpub...",
            ),
            (
                CreateTxError::UnknownUtxo {
                    outpoint: "outpoint123".to_string(),
                },
                "Reference to an unknown UTXO: outpoint123",
            ),
            (
                CreateTxError::MissingNonWitnessUtxo {
                    outpoint: "outpoint456".to_string(),
                },
                "Missing non-witness UTXO for outpoint: outpoint456",
            ),
            (
                CreateTxError::MiniscriptPsbt {
                    error_message: "Miniscript error".to_string(),
                },
                "Miniscript PSBT error: Miniscript error",
            ),
        ];

        for (error, expected_message) in cases {
            assert_eq!(error.to_string(), expected_message);
        }
    }

    #[test]
    fn test_error_descriptor() {
        let cases = vec![
            (DescriptorError::InvalidHdKeyPath, "invalid hd key path"),
            (
                DescriptorError::InvalidDescriptorChecksum,
                "the provided descriptor doesn't match its checksum",
            ),
            (
                DescriptorError::HardenedDerivationXpub,
                "the descriptor contains hardened derivation steps on public extended keys",
            ),
            (
                DescriptorError::MultiPath,
                "the descriptor contains multipath keys, which are not supported yet",
            ),
            (
                DescriptorError::Key {
                    error_message: "Invalid key format".to_string(),
                },
                "key error: Invalid key format",
            ),
            (
                DescriptorError::Policy {
                    error_message: "Policy rule failed".to_string(),
                },
                "policy error: Policy rule failed",
            ),
            (
                DescriptorError::InvalidDescriptorCharacter {
                    char: "}".to_string(),
                },
                "invalid descriptor character: }",
            ),
            (
                DescriptorError::Bip32 {
                    error_message: "Bip32 error".to_string(),
                },
                "BIP32 error: Bip32 error",
            ),
            (
                DescriptorError::Base58 {
                    error_message: "Base58 decode error".to_string(),
                },
                "Base58 error: Base58 decode error",
            ),
            (
                DescriptorError::Pk {
                    error_message: "Public key error".to_string(),
                },
                "Key-related error: Public key error",
            ),
            (
                DescriptorError::Miniscript {
                    error_message: "Miniscript evaluation error".to_string(),
                },
                "Miniscript error: Miniscript evaluation error",
            ),
            (
                DescriptorError::Hex {
                    error_message: "Hexadecimal decoding error".to_string(),
                },
                "Hex decoding error: Hexadecimal decoding error",
            ),
        ];

        for (error, expected_message) in cases {
            assert_eq!(error.to_string(), expected_message);
        }
    }

    #[test]
    fn test_error_descriptor_key() {
        let cases = vec![
            (
                DescriptorKeyError::Parse {
                    error_message: "Failed to parse descriptor key".to_string(),
                },
                "error parsing descriptor key: Failed to parse descriptor key",
            ),
            (DescriptorKeyError::InvalidKeyType, "error invalid key type"),
            (
                DescriptorKeyError::Bip32 {
                    error_message: "BIP32 derivation error".to_string(),
                },
                "error bip 32 related: BIP32 derivation error",
            ),
        ];

        for (error, expected_message) in cases {
            assert_eq!(error.to_string(), expected_message);
        }
    }

    #[test]
    fn test_error_esplora() {
        let cases = vec![
            (
                EsploraError::Minreq {
                    error_message: "Network error".to_string(),
                },
                "minreq error: Network error",
            ),
            (
                EsploraError::HttpResponse {
                    status: 404,
                    error_message: "Not found".to_string(),
                },
                "http error with status code 404 and message Not found",
            ),
            (
                EsploraError::StatusCode {
                    error_message: "code 1234567".to_string(),
                },
                "Invalid status code, unable to convert to u16: code 1234567",
            ),
            (
                EsploraError::Parsing {
                    error_message: "Invalid JSON".to_string(),
                },
                "parsing error: Invalid JSON",
            ),
            (
                EsploraError::BitcoinEncoding {
                    error_message: "Bad format".to_string(),
                },
                "bitcoin encoding error: Bad format",
            ),
            (
                EsploraError::HexToArray {
                    error_message: "Invalid hex".to_string(),
                },
                "invalid hex data returned: Invalid hex",
            ),
            (
                EsploraError::HexToBytes {
                    error_message: "Invalid hex".to_string(),
                },
                "invalid hex data returned: Invalid hex",
            ),
            (EsploraError::TransactionNotFound, "transaction not found"),
            (
                EsploraError::HeaderHeightNotFound { height: 123456 },
                "header height 123456 not found",
            ),
            (EsploraError::HeaderHashNotFound, "header hash not found"),
        ];

        for (error, expected_message) in cases {
            assert_eq!(error.to_string(), expected_message);
        }
    }

    #[test]
    fn test_error_extract_tx() {
        let cases = vec![
            (
                ExtractTxError::AbsurdFeeRate { fee_rate: 10000 },
                "an absurdly high fee rate of 10000 sat/vbyte",
            ),
            (
                ExtractTxError::MissingInputValue,
                "one of the inputs lacked value information (witness_utxo or non_witness_utxo)",
            ),
            (
                ExtractTxError::SendingTooMuch,
                "transaction would be invalid due to output value being greater than input value",
            ),
            (
                ExtractTxError::OtherExtractTxErr,
                "this error is required because the bdk::bitcoin::psbt::ExtractTxError is non-exhaustive"
            ),
        ];

        for (error, expected_message) in cases {
            assert_eq!(error.to_string(), expected_message);
        }
    }

    #[test]
    fn test_error_fee_rate() {
        let cases = vec![(
            FeeRateError::ArithmeticOverflow,
            "arithmetic overflow on feerate",
        )];

        for (error, expected_message) in cases {
            assert_eq!(error.to_string(), expected_message);
        }
    }

    #[test]
    fn test_persistence_error() {
        let cases = vec![
            (
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "unable to persist the new address",
                )
                .into(),
                "writing to persistence error: unable to persist the new address",
            ),
            (
                PersistenceError::Write {
                    error_message: "failed to write to storage".to_string(),
                },
                "writing to persistence error: failed to write to storage",
            ),
        ];

        for (error, expected_message) in cases {
            assert_eq!(error.to_string(), expected_message);
        }
    }

    #[test]
    fn test_error_psbt_parse() {
        let cases = vec![
            (
                PsbtParseError::PsbtEncoding {
                    error_message: "invalid PSBT structure".to_string(),
                },
                "error in internal PSBT data structure: invalid PSBT structure",
            ),
            (
                PsbtParseError::Base64Encoding {
                    error_message: "base64 decode error".to_string(),
                },
                "error in PSBT base64 encoding: base64 decode error",
            ),
        ];

        for (error, expected_message) in cases {
            assert_eq!(error.to_string(), expected_message);
        }
    }

    #[test]
    fn test_signer_errors() {
        let errors = vec![
            (SignerError::MissingKey, "Missing key for signing"),
            (SignerError::InvalidKey, "Invalid key provided"),
            (SignerError::UserCanceled, "User canceled operation"),
            (
                SignerError::InputIndexOutOfRange,
                "Input index out of range",
            ),
            (
                SignerError::MissingNonWitnessUtxo,
                "Missing non-witness UTXO information",
            ),
            (
                SignerError::InvalidNonWitnessUtxo,
                "Invalid non-witness UTXO information provided",
            ),
            (SignerError::MissingWitnessUtxo, "Missing witness UTXO"),
            (SignerError::MissingWitnessScript, "Missing witness script"),
            (SignerError::MissingHdKeypath, "Missing HD keypath"),
            (
                SignerError::NonStandardSighash,
                "Non-standard sighash type used",
            ),
            (SignerError::InvalidSighash, "Invalid sighash type provided"),
            (
                SignerError::SighashError {
                    error_message: "dummy error".into(),
                },
                "Error with sighash computation: dummy error",
            ),
            (
                SignerError::MiniscriptPsbt {
                    error_message: "psbt issue".into(),
                },
                "Miniscript Psbt error: psbt issue",
            ),
            (
                SignerError::External {
                    error_message: "external error".into(),
                },
                "External error: external error",
            ),
        ];

        for (error, message) in errors {
            assert_eq!(error.to_string(), message);
        }
    }

    #[test]
    fn test_error_transaction() {
        let cases = vec![
            (TransactionError::Io, "IO error"),
            (
                TransactionError::OversizedVectorAllocation,
                "allocation of oversized vector",
            ),
            (
                TransactionError::InvalidChecksum {
                    expected: "deadbeef".to_string(),
                    actual: "beadbeef".to_string(),
                },
                "invalid checksum: expected=deadbeef actual=beadbeef",
            ),
            (TransactionError::NonMinimalVarInt, "non-minimal varint"),
            (TransactionError::ParseFailed, "parse failed"),
            (
                TransactionError::UnsupportedSegwitFlag { flag: 1 },
                "unsupported segwit version: 1",
            ),
            (
                TransactionError::OtherTransactionErr,
                "other transaction error",
            ),
        ];

        for (error, expected_message) in cases {
            assert_eq!(error.to_string(), expected_message);
        }
    }

    #[test]
    fn test_error_txid_parse() {
        let cases = vec![(
            TxidParseError::InvalidTxid {
                txid: "123abc".to_string(),
            },
            "invalid txid: 123abc",
        )];

        for (error, expected_message) in cases {
            assert_eq!(error.to_string(), expected_message);
        }
    }

    #[test]
    fn test_error_wallet_creation() {
        let errors = vec![
            (
                WalletCreationError::Io {
                    error_message: "io error".to_string(),
                },
                "io error trying to read file: io error".to_string(),
            ),
            (
                WalletCreationError::InvalidMagicBytes {
                    got: vec![1, 2, 3, 4],
                    expected: vec![4, 3, 2, 1],
                },
                "file has invalid magic bytes: expected=[4, 3, 2, 1] got=[1, 2, 3, 4]".to_string(),
            ),
            (
                WalletCreationError::Descriptor,
                "error with descriptor".to_string(),
            ),
            (
                WalletCreationError::Persist {
                    error_message: "persistence error".to_string(),
                },
                "failed to either write to or load from persistence, persistence error".to_string(),
            ),
            (
                WalletCreationError::NotInitialized,
                "wallet is not initialized, persistence backend is empty".to_string(),
            ),
            (
                WalletCreationError::LoadedGenesisDoesNotMatch {
                    expected: "bitcoin".to_string(),
                    got: "testnet".to_string(),
                },
                "loaded genesis hash testnet does not match the expected one bitcoin".to_string(),
            ),
            (
                WalletCreationError::LoadedNetworkDoesNotMatch {
                    expected: Network::Bitcoin,
                    got: Some(Network::Testnet),
                },
                "loaded network type is not bitcoin, got Some(Testnet)".to_string(),
            ),
        ];

        for (error, expected) in errors {
            assert_eq!(error.to_string(), expected);
        }
    }
}
