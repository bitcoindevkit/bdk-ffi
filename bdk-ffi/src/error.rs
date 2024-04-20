use crate::bitcoin::OutPoint;

use bdk::bitcoin::psbt::PsbtParseError as BdkPsbtParseError;
use bdk::bitcoin::Network;
use bdk::chain::tx_graph::CalculateFeeError as BdkCalculateFeeError;
use bdk::descriptor::DescriptorError as BdkDescriptorError;
use bdk::wallet::error::BuildFeeBumpError;
use bdk::wallet::signer::SignerError as BdkSignerError;
use bdk::wallet::tx_builder::{AddUtxoError, AllowShrinkingError};
use bdk::wallet::{NewError, NewOrLoadError};
use bdk_esplora::esplora_client::{Error as BdkEsploraError, Error};
use bdk_file_store::FileError as BdkFileError;
use bdk_file_store::IterError;
use bitcoin_internals::hex::display::DisplayHex;

use crate::error::bip32::Error as BdkBip32Error;
use bdk::bitcoin::address::ParseError;
use bdk::keys::bip39::Error as BdkBip39Error;
use bdk::miniscript::descriptor::DescriptorKeyParseError as BdkDescriptorKeyParseError;

use bdk::bitcoin::bip32;

use bdk::wallet::error::CreateTxError as BdkCreateTxError;
use std::convert::TryInto;

#[derive(Debug, thiserror::Error)]
pub enum Alpha3Error {
    #[error("generic error in ffi")]
    Generic,
}

#[derive(Debug, thiserror::Error)]
pub enum Bip32Error {
    #[error("Cannot derive from a hardened key")]
    CannotDeriveFromHardenedKey,

    #[error("Secp256k1 error: {e}")]
    Secp256k1 { e: String },

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

    #[error("Base58 error: {e}")]
    Base58 { e: String },

    #[error("Hexadecimal conversion error: {e}")]
    Hex { e: String },

    #[error("Invalid public key hex length: {length}")]
    InvalidPublicKeyHexLength { length: u32 },

    #[error("Unknown error: {e}")]
    UnknownError { e: String },
}

#[derive(Debug, thiserror::Error)]
pub enum CreateTxError {
    #[error("Descriptor error: {e}")]
    Descriptor { e: String },

    #[error("Persistence failure: {e}")]
    Persist { e: String },

    #[error("Policy error: {e}")]
    Policy { e: String },

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

    #[error("Coin selection failed: {e}")]
    CoinSelection { e: String },

    #[error("Insufficient funds: needed {needed} sat, available {available} sat")]
    InsufficientFunds { needed: u64, available: u64 },

    #[error("Transaction has no recipients")]
    NoRecipients,

    #[error("PSBT creation error: {e}")]
    Psbt { e: String },

    #[error("Missing key origin for: {key}")]
    MissingKeyOrigin { key: String },

    #[error("Reference to an unknown UTXO: {outpoint}")]
    UnknownUtxo { outpoint: String },

    #[error("Missing non-witness UTXO for outpoint: {outpoint}")]
    MissingNonWitnessUtxo { outpoint: String },

    #[error("Miniscript PSBT error: {e}")]
    MiniscriptPsbt { e: String },
}

#[derive(Debug, thiserror::Error)]
pub enum DescriptorKeyError {
    #[error("error parsing descriptor key: {e}")]
    Parse { e: String },

    #[error("error invalid key type")]
    InvalidKeyType,

    #[error("error bip 32 related: {e}")]
    Bip32 { e: String },
}

#[derive(Debug, thiserror::Error)]
pub enum CalculateFeeError {
    #[error("missing transaction output: {out_points:?}")]
    MissingTxOut { out_points: Vec<OutPoint> },

    #[error("negative fee value: {fee}")]
    NegativeFee { fee: i64 },
}

#[derive(Debug, thiserror::Error)]
pub enum WalletCreationError {
    // Errors coming from the FileError enum
    #[error("io error trying to read file: {e}")]
    Io { e: String },

    #[error("file has invalid magic bytes: expected={expected:?} got={got:?}")]
    InvalidMagicBytes { got: Vec<u8>, expected: Vec<u8> },

    // Errors coming from the NewOrLoadError enum
    #[error("error with descriptor")]
    Descriptor,

    #[error("failed to write to persistence")]
    Write,

    #[error("failed to load from persistence")]
    Load,

    #[error("wallet is not initialized, persistence backend is empty")]
    NotInitialized,

    #[error("loaded genesis hash does not match the expected one")]
    LoadedGenesisDoesNotMatch,

    #[error("loaded network type is not {expected}, got {got:?}")]
    LoadedNetworkDoesNotMatch {
        expected: Network,
        got: Option<Network>,
    },
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
pub enum PersistenceError {
    #[error("writing to persistence error: {e}")]
    Write { e: String },
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
pub enum FeeRateError {
    #[error("arithmetic overflow on feerate")]
    ArithmeticOverflow,
}

#[derive(Debug, thiserror::Error)]
pub enum TxidParseError {
    #[error("invalid txid: {txid}")]
    InvalidTxid { txid: String },
}

#[derive(Debug, thiserror::Error)]
pub enum CannotConnectError {
    #[error("cannot include height: {height}")]
    Include { height: u32 },
}

#[derive(Debug, thiserror::Error)]
pub enum AddressError {
    // Errors coming from the ParseError enum
    #[error("base58 address encoding error")]
    Base58,

    #[error("bech32 address encoding error")]
    Bech32,

    // Errors coming from the bitcoin::address::Error enum
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
        /// Network that was required.
        required: Network,
        /// Network on which the address was found to be valid.
        found: Network,
        /// The address itself
        address: String,
    },

    // This is required because the bdk::bitcoin::address::Error is non-exhaustive
    #[error("other address error")]
    OtherAddressErr,
}

// Mapping https://docs.rs/bitcoin/latest/src/bitcoin/consensus/encode.rs.html#40-63
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
pub enum PsbtParseError {
    #[error("error in internal PSBT data structure: {e}")]
    PsbtEncoding { e: String },

    #[error("error in PSBT base64 encoding: {e}")]
    Base64Encoding { e: String },
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

    #[error("key error: {e}")]
    Key { e: String },

    #[error("policy error: {e}")]
    Policy { e: String },

    #[error("invalid descriptor character: {char}")]
    InvalidDescriptorCharacter { char: String },

    #[error("BIP32 error: {e}")]
    Bip32 { e: String },

    #[error("Base58 error: {e}")]
    Base58 { e: String },

    #[error("Key-related error: {e}")]
    Pk { e: String },

    #[error("Miniscript error: {e}")]
    Miniscript { e: String },

    #[error("Hex decoding error: {e}")]
    Hex { e: String },
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

    #[error("Error with sighash computation: {e}")]
    SighashError { e: String },

    #[error("Miniscript Psbt error: {e}")]
    MiniscriptPsbt { e: String },

    #[error("External error: {e}")]
    External { e: String },
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

impl From<BdkCreateTxError<std::io::Error>> for CreateTxError {
    fn from(error: BdkCreateTxError<std::io::Error>) -> Self {
        match error {
            BdkCreateTxError::Descriptor(e) => CreateTxError::Descriptor { e: e.to_string() },
            BdkCreateTxError::Persist(e) => CreateTxError::Persist { e: e.to_string() },
            BdkCreateTxError::Policy(e) => CreateTxError::Policy { e: e.to_string() },
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
            BdkCreateTxError::CoinSelection(e) => CreateTxError::CoinSelection { e: e.to_string() },
            BdkCreateTxError::InsufficientFunds { needed, available } => {
                CreateTxError::InsufficientFunds { needed, available }
            }
            BdkCreateTxError::NoRecipients => CreateTxError::NoRecipients,
            BdkCreateTxError::Psbt(e) => CreateTxError::Psbt { e: e.to_string() },
            BdkCreateTxError::MissingKeyOrigin(key) => CreateTxError::MissingKeyOrigin { key },
            BdkCreateTxError::UnknownUtxo => CreateTxError::UnknownUtxo {
                outpoint: "Unknown".to_string(),
            },
            BdkCreateTxError::MissingNonWitnessUtxo(outpoint) => {
                CreateTxError::MissingNonWitnessUtxo {
                    outpoint: outpoint.to_string(),
                }
            }
            BdkCreateTxError::MiniscriptPsbt(e) => {
                CreateTxError::MiniscriptPsbt { e: e.to_string() }
            }
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

impl From<BdkDescriptorError> for DescriptorError {
    fn from(error: BdkDescriptorError) -> Self {
        match error {
            BdkDescriptorError::InvalidHdKeyPath => DescriptorError::InvalidHdKeyPath,
            BdkDescriptorError::InvalidDescriptorChecksum => {
                DescriptorError::InvalidDescriptorChecksum
            }
            BdkDescriptorError::HardenedDerivationXpub => DescriptorError::HardenedDerivationXpub,
            BdkDescriptorError::MultiPath => DescriptorError::MultiPath,
            BdkDescriptorError::Key(e) => DescriptorError::Key { e: e.to_string() },
            BdkDescriptorError::Policy(e) => DescriptorError::Policy { e: e.to_string() },
            BdkDescriptorError::InvalidDescriptorCharacter(char) => {
                DescriptorError::InvalidDescriptorCharacter {
                    char: char.to_string(),
                }
            }
            BdkDescriptorError::Bip32(e) => DescriptorError::Bip32 { e: e.to_string() },
            BdkDescriptorError::Base58(e) => DescriptorError::Base58 { e: e.to_string() },
            BdkDescriptorError::Pk(e) => DescriptorError::Pk { e: e.to_string() },
            BdkDescriptorError::Miniscript(e) => DescriptorError::Miniscript { e: e.to_string() },
            BdkDescriptorError::Hex(e) => DescriptorError::Hex { e: e.to_string() },
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
            BdkSignerError::SighashError(e) => SignerError::SighashError { e: e.to_string() },
            BdkSignerError::MiniscriptPsbt(e) => SignerError::MiniscriptPsbt {
                e: format!("{:?}", e),
            },
            BdkSignerError::External(e) => SignerError::External { e },
        }
    }
}

impl From<BdkDescriptorKeyParseError> for DescriptorKeyError {
    fn from(err: BdkDescriptorKeyParseError) -> DescriptorKeyError {
        DescriptorKeyError::Parse {
            e: format!("DescriptorKeyError error: {:?}", err),
        }
    }
}

impl From<bdk::bitcoin::bip32::Error> for DescriptorKeyError {
    fn from(err: bdk::bitcoin::bip32::Error) -> DescriptorKeyError {
        DescriptorKeyError::Bip32 {
            e: format!("BIP32 derivation error: {:?}", err),
        }
    }
}

impl From<BdkPsbtParseError> for PsbtParseError {
    fn from(error: BdkPsbtParseError) -> Self {
        match error {
            BdkPsbtParseError::PsbtEncoding(e) => PsbtParseError::PsbtEncoding { e: e.to_string() },
            BdkPsbtParseError::Base64Encoding(e) => {
                PsbtParseError::Base64Encoding { e: e.to_string() }
            }
            _ => {
                unreachable!("this is required because of the non-exhaustive enum in rust-bitcoin")
            }
        }
    }
}

impl From<BdkFileError> for WalletCreationError {
    fn from(error: BdkFileError) -> Self {
        match error {
            BdkFileError::Io(e) => WalletCreationError::Io { e: e.to_string() },
            BdkFileError::InvalidMagicBytes { got, expected } => {
                WalletCreationError::InvalidMagicBytes { got, expected }
            }
        }
    }
}

impl From<NewOrLoadError<std::io::Error, IterError>> for WalletCreationError {
    fn from(error: NewOrLoadError<std::io::Error, IterError>) -> Self {
        match error {
            NewOrLoadError::Descriptor(_) => WalletCreationError::Descriptor,
            NewOrLoadError::Write(_) => WalletCreationError::Write,
            NewOrLoadError::Load(_) => WalletCreationError::Load,
            NewOrLoadError::NotInitialized => WalletCreationError::NotInitialized,
            NewOrLoadError::LoadedGenesisDoesNotMatch { .. } => {
                WalletCreationError::LoadedGenesisDoesNotMatch
            }
            NewOrLoadError::LoadedNetworkDoesNotMatch { expected, got } => {
                WalletCreationError::LoadedNetworkDoesNotMatch { expected, got }
            }
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

impl From<std::io::Error> for PersistenceError {
    fn from(error: std::io::Error) -> Self {
        PersistenceError::Write {
            e: error.to_string(),
        }
    }
}

impl From<AllowShrinkingError> for Alpha3Error {
    fn from(_: AllowShrinkingError) -> Self {
        Alpha3Error::Generic
    }
}

impl From<BuildFeeBumpError> for Alpha3Error {
    fn from(_: BuildFeeBumpError) -> Self {
        Alpha3Error::Generic
    }
}

impl From<AddUtxoError> for Alpha3Error {
    fn from(_: AddUtxoError) -> Self {
        Alpha3Error::Generic
    }
}

impl From<bdk::bitcoin::bip32::Error> for Alpha3Error {
    fn from(_: bdk::bitcoin::bip32::Error) -> Self {
        Alpha3Error::Generic
    }
}

impl From<BdkBip32Error> for Bip32Error {
    fn from(error: BdkBip32Error) -> Self {
        match error {
            BdkBip32Error::CannotDeriveFromHardenedKey => Bip32Error::CannotDeriveFromHardenedKey,
            BdkBip32Error::Secp256k1(err) => Bip32Error::Secp256k1 { e: err.to_string() },
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
            BdkBip32Error::Base58(err) => Bip32Error::Base58 { e: err.to_string() },
            BdkBip32Error::Hex(err) => Bip32Error::Hex { e: err.to_string() },
            BdkBip32Error::InvalidPublicKeyHexLength(len) => {
                Bip32Error::InvalidPublicKeyHexLength { length: len as u32 }
            }
            _ => Bip32Error::UnknownError {
                e: format!("Unhandled error: {:?}", error),
            },
        }
    }
}

impl From<NewError<std::io::Error>> for Alpha3Error {
    fn from(_: NewError<std::io::Error>) -> Self {
        Alpha3Error::Generic
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
#[cfg(test)]
mod test {
    use crate::error::{
        Bip32Error, Bip39Error, CannotConnectError, EsploraError, PersistenceError,
        WalletCreationError,
    };
    use crate::CalculateFeeError;
    use crate::OutPoint;
    use crate::SignerError;
    use bdk::bitcoin::Network;

    #[test]
    fn test_error_missing_tx_out() {
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

        let error = CalculateFeeError::MissingTxOut { out_points };

        let expected_message: String = format!(
            "missing transaction output: [{:?}, {:?}]",
            OutPoint {
                txid: "0000000000000000000000000000000000000000000000000000000000000001"
                    .to_string(),
                vout: 0
            },
            OutPoint {
                txid: "0000000000000000000000000000000000000000000000000000000000000002"
                    .to_string(),
                vout: 1
            }
        );

        assert_eq!(error.to_string(), expected_message);
    }

    #[test]
    fn test_error_negative_fee() {
        let error = CalculateFeeError::NegativeFee { fee: -100 };

        assert_eq!(error.to_string(), "negative fee value: -100");
    }

    #[test]
    fn test_esplora_errors() {
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
    fn test_persistence_error() {
        let io_err = std::io::Error::new(
            std::io::ErrorKind::Other,
            "unable to persist the new address",
        );
        let op_err: PersistenceError = io_err.into();

        let PersistenceError::Write { e } = op_err;
        assert_eq!(e, "unable to persist the new address");
    }

    #[test]
    fn test_error_wallet_creation() {
        let io_error = WalletCreationError::Io {
            e: "io error".to_string(),
        };
        assert_eq!(
            io_error.to_string(),
            "io error trying to read file: io error"
        );

        let invalid_magic_bytes_error = WalletCreationError::InvalidMagicBytes {
            got: vec![1, 2, 3, 4],
            expected: vec![4, 3, 2, 1],
        };
        assert_eq!(
            invalid_magic_bytes_error.to_string(),
            "file has invalid magic bytes: expected=[4, 3, 2, 1] got=[1, 2, 3, 4]"
        );

        let descriptor_error = WalletCreationError::Descriptor;
        assert_eq!(descriptor_error.to_string(), "error with descriptor");

        let write_error = WalletCreationError::Write;
        assert_eq!(write_error.to_string(), "failed to write to persistence");

        let load_error = WalletCreationError::Load;
        assert_eq!(load_error.to_string(), "failed to load from persistence");

        let not_initialized_error = WalletCreationError::NotInitialized;
        assert_eq!(
            not_initialized_error.to_string(),
            "wallet is not initialized, persistence backend is empty"
        );

        let loaded_genesis_does_not_match_error = WalletCreationError::LoadedGenesisDoesNotMatch;
        assert_eq!(
            loaded_genesis_does_not_match_error.to_string(),
            "loaded genesis hash does not match the expected one"
        );

        let loaded_network_does_not_match_error = WalletCreationError::LoadedNetworkDoesNotMatch {
            expected: Network::Bitcoin,
            got: Some(Network::Testnet),
        };
        assert_eq!(
            loaded_network_does_not_match_error.to_string(),
            "loaded network type is not bitcoin, got Some(Testnet)"
        );
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
                    e: "dummy error".into(),
                },
                "Error with sighash computation: dummy error",
            ),
            (
                SignerError::MiniscriptPsbt {
                    e: "psbt issue".into(),
                },
                "Miniscript Psbt error: psbt issue",
            ),
            (
                SignerError::External {
                    e: "external error".into(),
                },
                "External error: external error",
            ),
        ];

        for (error, message) in errors {
            assert_eq!(error.to_string(), message);
        }
    }

    #[test]
    fn test_cannot_connect_error_include() {
        let error = CannotConnectError::Include { height: 42 };

        assert_eq!(format!("{}", error), "cannot include height: 42");
    }

    #[test]
    fn test_error_bip39() {
        let error = Bip39Error::BadWordCount { word_count: 15 };
        assert_eq!(format!("{}", error), "the word count 15 is not supported");

        let error = Bip39Error::UnknownWord { index: 102 };
        assert_eq!(format!("{}", error), "unknown word at index 102");

        let error = Bip39Error::BadEntropyBitCount { bit_count: 128 };
        assert_eq!(format!("{}", error), "entropy bit count 128 is invalid");

        let error = Bip39Error::InvalidChecksum;
        assert_eq!(format!("{}", error), "checksum is invalid");

        let error = Bip39Error::AmbiguousLanguages {
            languages: "English, Spanish".to_string(),
        };
        assert_eq!(
            format!("{}", error),
            "ambiguous languages detected: English, Spanish"
        )
    }

    #[test]
    fn test_error_bip32() {
        let error = Bip32Error::CannotDeriveFromHardenedKey;
        assert_eq!(format!("{}", error), "Cannot derive from a hardened key");

        let error = Bip32Error::Secp256k1 {
            e: "Secp256k1 failure".to_string(),
        };
        assert_eq!(format!("{}", error), "Secp256k1 error: Secp256k1 failure");

        let error = Bip32Error::InvalidChildNumber { child_number: 42 };
        assert_eq!(format!("{}", error), "Invalid child number: 42");

        let error = Bip32Error::InvalidChildNumberFormat;
        assert_eq!(format!("{}", error), "Invalid format for child number");

        let error = Bip32Error::InvalidDerivationPathFormat;
        assert_eq!(format!("{}", error), "Invalid derivation path format");

        let error = Bip32Error::UnknownVersion {
            version: "deadbeef".to_string(),
        };
        assert_eq!(format!("{}", error), "Unknown version: deadbeef");

        let error = Bip32Error::WrongExtendedKeyLength { length: 128 };
        assert_eq!(format!("{}", error), "Wrong extended key length: 128");

        let error = Bip32Error::Base58 {
            e: "Base58 error".to_string(),
        };
        assert_eq!(format!("{}", error), "Base58 error: Base58 error");

        let error = Bip32Error::Hex {
            e: "Hex error".to_string(),
        };
        assert_eq!(
            format!("{}", error),
            "Hexadecimal conversion error: Hex error"
        );

        let error = Bip32Error::InvalidPublicKeyHexLength { length: 65 };
        assert_eq!(format!("{}", error), "Invalid public key hex length: 65");

        let error = Bip32Error::UnknownError {
            e: "An unknown error occurred".to_string(),
        };
        assert_eq!(
            format!("{}", error),
            "Unknown error: An unknown error occurred"
        );
    }
}
