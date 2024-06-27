use crate::bitcoin::OutPoint;
use crate::Network;

use bdk_bitcoind_rpc::bitcoincore_rpc::bitcoin::address::ParseError;
use bdk_electrum::electrum_client::Error as BdkElectrumError;
use bdk_esplora::esplora_client::{Error as BdkEsploraError, Error};
use bdk_sqlite::Error as BdkSqliteError;
use bdk_wallet::bitcoin::address::FromScriptError as BdkFromScriptError;
use bdk_wallet::bitcoin::address::ParseError as BdkParseError;
use bdk_wallet::bitcoin::amount::ParseAmountError as BdkParseAmountError;
use bdk_wallet::bitcoin::bip32::Error as BdkBip32Error;
use bdk_wallet::bitcoin::consensus::encode::Error as BdkEncodeError;
use bdk_wallet::bitcoin::psbt::Error as BdkPsbtError;
use bdk_wallet::bitcoin::psbt::ExtractTxError as BdkExtractTxError;
use bdk_wallet::bitcoin::psbt::PsbtParseError as BdkPsbtParseError;
use bdk_wallet::chain::local_chain::CannotConnectError as BdkCannotConnectError;
use bdk_wallet::chain::tx_graph::CalculateFeeError as BdkCalculateFeeError;
use bdk_wallet::descriptor::DescriptorError as BdkDescriptorError;
use bdk_wallet::keys::bip39::Error as BdkBip39Error;
use bdk_wallet::miniscript::descriptor::DescriptorKeyParseError as BdkDescriptorKeyParseError;
use bdk_wallet::wallet::error::BuildFeeBumpError;
use bdk_wallet::wallet::error::CreateTxError as BdkCreateTxError;
use bdk_wallet::wallet::signer::SignerError as BdkSignerError;
use bdk_wallet::wallet::tx_builder::AddUtxoError;
use bdk_wallet::wallet::{NewError, NewOrLoadError};
use bdk_wallet::KeychainKind;
use bitcoin_internals::hex::display::DisplayHex;

use std::convert::TryInto;

// ------------------------------------------------------------------------
// error definitions
// ------------------------------------------------------------------------

#[derive(Debug, thiserror::Error)]
pub enum AddressParseError {
    #[error("base58 address encoding error")]
    Base58,

    #[error("bech32 address encoding error")]
    Bech32,

    #[error("witness version conversion/parsing error: {error_message}")]
    WitnessVersion { error_message: String },

    #[error("witness program error: {error_message}")]
    WitnessProgram { error_message: String },

    #[error("tried to parse an unknown hrp")]
    UnknownHrp,

    #[error("legacy address base58 string")]
    LegacyAddressTooLong,

    #[error("legacy address base58 data")]
    InvalidBase58PayloadLength,

    #[error("segwit address bech32 string")]
    InvalidLegacyPrefix,

    #[error("validation error")]
    NetworkValidation,

    // This error is required because the bdk::bitcoin::address::ParseError is non-exhaustive
    #[error("other address parse error")]
    OtherAddressParseErr,
}

#[derive(Debug, thiserror::Error)]
pub enum Bip32Error {
    #[error("cannot derive from a hardened key")]
    CannotDeriveFromHardenedKey,

    #[error("secp256k1 error: {error_message}")]
    Secp256k1 { error_message: String },

    #[error("invalid child number: {child_number}")]
    InvalidChildNumber { child_number: u32 },

    #[error("invalid format for child number")]
    InvalidChildNumberFormat,

    #[error("invalid derivation path format")]
    InvalidDerivationPathFormat,

    #[error("unknown version: {version}")]
    UnknownVersion { version: String },

    #[error("wrong extended key length: {length}")]
    WrongExtendedKeyLength { length: u32 },

    #[error("base58 error: {error_message}")]
    Base58 { error_message: String },

    #[error("hexadecimal conversion error: {error_message}")]
    Hex { error_message: String },

    #[error("invalid public key hex length: {length}")]
    InvalidPublicKeyHexLength { length: u32 },

    #[error("unknown error: {error_message}")]
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

    #[error("negative fee value: {amount}")]
    NegativeFee { amount: String },
}

#[derive(Debug, thiserror::Error)]
pub enum CannotConnectError {
    #[error("cannot include height: {height}")]
    Include { height: u32 },
}

#[derive(Debug, thiserror::Error)]
pub enum CreateTxError {
    #[error("descriptor error: {error_message}")]
    Descriptor { error_message: String },

    #[error("policy error: {error_message}")]
    Policy { error_message: String },

    #[error("spending policy required for {kind}")]
    SpendingPolicyRequired { kind: String },

    #[error("unsupported version 0")]
    Version0,

    #[error("unsupported version 1 with csv")]
    Version1Csv,

    #[error("lock time conflict: requested {requested}, but required {required}")]
    LockTime { requested: String, required: String },

    #[error("transaction requires rbf sequence number")]
    RbfSequence,

    #[error("rbf sequence: {rbf}, csv sequence: {csv}")]
    RbfSequenceCsv { rbf: String, csv: String },

    #[error("fee too low: required {required}")]
    FeeTooLow { required: String },

    #[error("fee rate too low: {required}")]
    FeeRateTooLow { required: String },

    #[error("no utxos selected for the transaction")]
    NoUtxosSelected,

    #[error("output value below dust limit at index {index}")]
    OutputBelowDustLimit { index: u64 },

    #[error("change policy descriptor error")]
    ChangePolicyDescriptor,

    #[error("coin selection failed: {error_message}")]
    CoinSelection { error_message: String },

    #[error("insufficient funds: needed {needed} sat, available {available} sat")]
    InsufficientFunds { needed: u64, available: u64 },

    #[error("transaction has no recipients")]
    NoRecipients,

    #[error("psbt creation error: {error_message}")]
    Psbt { error_message: String },

    #[error("missing key origin for: {key}")]
    MissingKeyOrigin { key: String },

    #[error("reference to an unknown utxo: {outpoint}")]
    UnknownUtxo { outpoint: String },

    #[error("missing non-witness utxo for outpoint: {outpoint}")]
    MissingNonWitnessUtxo { outpoint: String },

    #[error("miniscript psbt error: {error_message}")]
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

    #[error("bip32 error: {error_message}")]
    Bip32 { error_message: String },

    #[error("base58 error: {error_message}")]
    Base58 { error_message: String },

    #[error("key-related error: {error_message}")]
    Pk { error_message: String },

    #[error("miniscript error: {error_message}")]
    Miniscript { error_message: String },

    #[error("hex decoding error: {error_message}")]
    Hex { error_message: String },

    #[error("external and internal descriptors are the same")]
    ExternalAndInternalAreTheSame,
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
pub enum ElectrumError {
    #[error("{error_message}")]
    IOError { error_message: String },

    #[error("{error_message}")]
    Json { error_message: String },

    #[error("{error_message}")]
    Hex { error_message: String },

    #[error("electrum server error: {error_message}")]
    Protocol { error_message: String },

    #[error("{error_message}")]
    Bitcoin { error_message: String },

    #[error("already subscribed to the notifications of an address")]
    AlreadySubscribed,

    #[error("not subscribed to the notifications of an address")]
    NotSubscribed,

    #[error("error during the deserialization of a response from the server: {error_message}")]
    InvalidResponse { error_message: String },

    #[error("{error_message}")]
    Message { error_message: String },

    #[error("invalid domain name {domain} not matching SSL certificate")]
    InvalidDNSNameError { domain: String },

    #[error("missing domain while it was explicitly asked to validate it")]
    MissingDomain,

    #[error("made one or multiple attempts, all errored")]
    AllAttemptsErrored,

    #[error("{error_message}")]
    SharedIOError { error_message: String },

    #[error("couldn't take a lock on the reader mutex. This means that there's already another reader thread is running")]
    CouldntLockReader,

    #[error("broken IPC communication channel: the other thread probably has exited")]
    Mpsc,

    #[error("{error_message}")]
    CouldNotCreateConnection { error_message: String },

    #[error("the request has already been consumed")]
    RequestAlreadyConsumed,
}

#[derive(Debug, thiserror::Error)]
pub enum EsploraError {
    #[error("minreq error: {error_message}")]
    Minreq { error_message: String },

    #[error("http error with status code {status} and message {error_message}")]
    HttpResponse { status: u16, error_message: String },

    #[error("parsing error: {error_message}")]
    Parsing { error_message: String },

    #[error("invalid status code, unable to convert to u16: {error_message}")]
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

    #[error("invalid http header name: {name}")]
    InvalidHttpHeaderName { name: String },

    #[error("invalid http header value: {value}")]
    InvalidHttpHeaderValue { value: String },

    #[error("the request has already been consumed")]
    RequestAlreadyConsumed,
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
pub enum FromScriptError {
    #[error("script is not a p2pkh, p2sh or witness program")]
    UnrecognizedScript,

    #[error("witness program error: {error_message}")]
    WitnessProgram { error_message: String },

    #[error("witness version construction error: {error_message}")]
    WitnessVersion { error_message: String },

    // This error is required because the bdk::bitcoin::address::FromScriptError is non-exhaustive
    #[error("other from script error")]
    OtherFromScriptErr,
}

#[derive(Debug, thiserror::Error)]
pub enum InspectError {
    #[error("the request has already been consumed")]
    RequestAlreadyConsumed,
}

#[derive(Debug, thiserror::Error)]
pub enum ParseAmountError {
    #[error("amount out of range")]
    OutOfRange,

    #[error("amount has a too high precision")]
    TooPrecise,

    #[error("the input has too few digits")]
    MissingDigits,

    #[error("the input is too large")]
    InputTooLarge,

    #[error("invalid character: {error_message}")]
    InvalidCharacter { error_message: String },

    // Has to handle non-exhaustive
    #[error("unknown parse amount error")]
    OtherParseAmountErr,
}

#[derive(Debug, thiserror::Error)]
pub enum PersistenceError {
    #[error("writing to persistence error: {error_message}")]
    Write { error_message: String },
}

#[derive(Debug, thiserror::Error)]
pub enum PsbtError {
    #[error("invalid magic")]
    InvalidMagic,

    #[error("UTXO information is not present in PSBT")]
    MissingUtxo,

    #[error("invalid separator")]
    InvalidSeparator,

    #[error("output index is out of bounds of non witness script output array")]
    PsbtUtxoOutOfBounds,

    #[error("invalid key: {key}")]
    InvalidKey { key: String },

    #[error("non-proprietary key type found when proprietary key was expected")]
    InvalidProprietaryKey,

    #[error("duplicate key: {key}")]
    DuplicateKey { key: String },

    #[error("the unsigned transaction has script sigs")]
    UnsignedTxHasScriptSigs,

    #[error("the unsigned transaction has script witnesses")]
    UnsignedTxHasScriptWitnesses,

    #[error("partially signed transactions must have an unsigned transaction")]
    MustHaveUnsignedTx,

    #[error("no more key-value pairs for this psbt map")]
    NoMorePairs,

    // Note: this error would be nice to unpack and provide the two transactions
    #[error("different unsigned transaction")]
    UnexpectedUnsignedTx,

    #[error("non-standard sighash type: {sighash}")]
    NonStandardSighashType { sighash: u32 },

    #[error("invalid hash when parsing slice: {hash}")]
    InvalidHash { hash: String },

    // Note: to provide the data returned in Rust, we need to dereference the fields
    #[error("preimage does not match")]
    InvalidPreimageHashPair,

    #[error("combine conflict: {xpub}")]
    CombineInconsistentKeySources { xpub: String },

    #[error("bitcoin consensus encoding error: {encoding_error}")]
    ConsensusEncoding { encoding_error: String },

    #[error("PSBT has a negative fee which is not allowed")]
    NegativeFee,

    #[error("integer overflow in fee calculation")]
    FeeOverflow,

    #[error("invalid public key {error_message}")]
    InvalidPublicKey { error_message: String },

    #[error("invalid secp256k1 public key: {secp256k1_error}")]
    InvalidSecp256k1PublicKey { secp256k1_error: String },

    #[error("invalid xonly public key")]
    InvalidXOnlyPublicKey,

    #[error("invalid ECDSA signature: {error_message}")]
    InvalidEcdsaSignature { error_message: String },

    #[error("invalid taproot signature: {error_message}")]
    InvalidTaprootSignature { error_message: String },

    #[error("invalid control block")]
    InvalidControlBlock,

    #[error("invalid leaf version")]
    InvalidLeafVersion,

    #[error("taproot error")]
    Taproot,

    #[error("taproot tree error: {error_message}")]
    TapTree { error_message: String },

    #[error("xpub key error")]
    XPubKey,

    #[error("version error: {error_message}")]
    Version { error_message: String },

    #[error("data not consumed entirely when explicitly deserializing")]
    PartialDataConsumption,

    #[error("I/O error: {error_message}")]
    Io { error_message: String },

    #[error("other PSBT error")]
    OtherPsbtErr,
}

#[derive(Debug, thiserror::Error)]
pub enum PsbtParseError {
    #[error("error in internal psbt data structure: {error_message}")]
    PsbtEncoding { error_message: String },

    #[error("error in psbt base64 encoding: {error_message}")]
    Base64Encoding { error_message: String },
}

#[derive(Debug, thiserror::Error)]
pub enum SignerError {
    #[error("missing key for signing")]
    MissingKey,

    #[error("invalid key provided")]
    InvalidKey,

    #[error("user canceled operation")]
    UserCanceled,

    #[error("input index out of range")]
    InputIndexOutOfRange,

    #[error("missing non-witness utxo information")]
    MissingNonWitnessUtxo,

    #[error("invalid non-witness utxo information provided")]
    InvalidNonWitnessUtxo,

    #[error("missing witness utxo")]
    MissingWitnessUtxo,

    #[error("missing witness script")]
    MissingWitnessScript,

    #[error("missing hd keypath")]
    MissingHdKeypath,

    #[error("non-standard sighash type used")]
    NonStandardSighash,

    #[error("invalid sighash type provided")]
    InvalidSighash,

    #[error("error while computing the hash to sign a P2WPKH input: {error_message}")]
    SighashP2wpkh { error_message: String },

    #[error("error while computing the hash to sign a taproot input: {error_message}")]
    SighashTaproot { error_message: String },

    #[error("Error while computing the hash, out of bounds access on the transaction inputs: {error_message}")]
    TxInputsIndexError { error_message: String },

    #[error("miniscript psbt error: {error_message}")]
    MiniscriptPsbt { error_message: String },

    #[error("external error: {error_message}")]
    External { error_message: String },
}

#[derive(Debug, thiserror::Error)]
pub enum SqliteError {
    // NOTE: This error is renamed from Network to InvalidNetwork to avoid conflict with the Network
    //       enum in uniffi.
    #[error("invalid network, cannot change the one already stored in the database")]
    InvalidNetwork { expected: Network, given: Network },

    #[error("SQLite error: {rusqlite_error}")]
    Sqlite { rusqlite_error: String },
}

#[derive(Debug, thiserror::Error)]
pub enum TransactionError {
    #[error("io error")]
    Io,

    #[error("allocation of oversized vector")]
    OversizedVectorAllocation,

    #[error("invalid checksum: expected={expected} actual={actual}")]
    InvalidChecksum { expected: String, actual: String },

    #[error("non-minimal var int")]
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

// This error combines the Rust bdk_wallet::wallet::NewError and bdk_wallet::wallet::NewOrLoadError
#[derive(Debug, thiserror::Error)]
pub enum WalletCreationError {
    // From NewError and NewOrLoadError
    #[error("error with descriptor: {error_message}")]
    Descriptor { error_message: String },

    // From NewOrLoadError
    #[error("loaded genesis hash '{got}' does not match the expected one '{expected}'")]
    LoadedGenesisDoesNotMatch { expected: String, got: String },

    // From NewOrLoadError
    #[error("loaded network type is not {expected}, got {got:?}")]
    LoadedNetworkDoesNotMatch {
        expected: Network,
        got: Option<Network>,
    },

    // From NewOrLoadError
    #[error("loaded descriptor '{got}' does not match what was provided '{keychain:?}'")]
    LoadedDescriptorDoesNotMatch { got: String, keychain: KeychainKind },
}

// ------------------------------------------------------------------------
// error conversions
// ------------------------------------------------------------------------

impl From<BdkElectrumError> for ElectrumError {
    fn from(error: BdkElectrumError) -> Self {
        match error {
            BdkElectrumError::IOError(e) => ElectrumError::IOError {
                error_message: e.to_string(),
            },
            BdkElectrumError::JSON(e) => ElectrumError::Json {
                error_message: e.to_string(),
            },
            BdkElectrumError::Hex(e) => ElectrumError::Hex {
                error_message: e.to_string(),
            },
            BdkElectrumError::Protocol(e) => ElectrumError::Protocol {
                error_message: e.to_string(),
            },
            BdkElectrumError::Bitcoin(e) => ElectrumError::Bitcoin {
                error_message: e.to_string(),
            },
            BdkElectrumError::AlreadySubscribed(_) => ElectrumError::AlreadySubscribed,
            BdkElectrumError::NotSubscribed(_) => ElectrumError::NotSubscribed,
            BdkElectrumError::InvalidResponse(e) => ElectrumError::InvalidResponse {
                error_message: e.to_string(),
            },
            BdkElectrumError::Message(e) => ElectrumError::Message {
                error_message: e.to_string(),
            },
            BdkElectrumError::InvalidDNSNameError(domain) => {
                ElectrumError::InvalidDNSNameError { domain }
            }
            BdkElectrumError::MissingDomain => ElectrumError::MissingDomain,
            BdkElectrumError::AllAttemptsErrored(_) => ElectrumError::AllAttemptsErrored,
            BdkElectrumError::SharedIOError(e) => ElectrumError::SharedIOError {
                error_message: e.to_string(),
            },
            BdkElectrumError::CouldntLockReader => ElectrumError::CouldntLockReader,
            BdkElectrumError::Mpsc => ElectrumError::Mpsc,
            BdkElectrumError::CouldNotCreateConnection(error_message) => {
                ElectrumError::CouldNotCreateConnection {
                    error_message: error_message.to_string(),
                }
            }
        }
    }
}

impl From<BdkParseError> for AddressParseError {
    fn from(error: BdkParseError) -> Self {
        match error {
            BdkParseError::Base58(_) => AddressParseError::Base58,
            BdkParseError::Bech32(_) => AddressParseError::Bech32,
            BdkParseError::WitnessVersion(e) => AddressParseError::WitnessVersion {
                error_message: e.to_string(),
            },
            BdkParseError::WitnessProgram(e) => AddressParseError::WitnessProgram {
                error_message: e.to_string(),
            },
            ParseError::UnknownHrp(_) => AddressParseError::UnknownHrp,
            ParseError::LegacyAddressTooLong(_) => AddressParseError::LegacyAddressTooLong,
            ParseError::InvalidBase58PayloadLength(_) => {
                AddressParseError::InvalidBase58PayloadLength
            }
            ParseError::InvalidLegacyPrefix(_) => AddressParseError::InvalidLegacyPrefix,
            ParseError::NetworkValidation(_) => AddressParseError::NetworkValidation,
            _ => AddressParseError::OtherAddressParseErr,
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
            BdkCalculateFeeError::NegativeFee(signed_amount) => CalculateFeeError::NegativeFee {
                amount: signed_amount.to_string(),
            },
        }
    }
}

impl From<BdkCannotConnectError> for CannotConnectError {
    fn from(error: BdkCannotConnectError) -> Self {
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
            BdkCreateTxError::FeeTooLow { required } => CreateTxError::FeeTooLow {
                required: required.to_string(),
            },
            BdkCreateTxError::FeeRateTooLow { required } => CreateTxError::FeeRateTooLow {
                required: required.to_string(),
            },
            BdkCreateTxError::NoUtxosSelected => CreateTxError::NoUtxosSelected,
            BdkCreateTxError::OutputBelowDustLimit(index) => CreateTxError::OutputBelowDustLimit {
                index: index as u64,
            },
            BdkCreateTxError::CoinSelection(e) => CreateTxError::CoinSelection {
                error_message: e.to_string(),
            },
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
            BdkDescriptorError::ExternalAndInternalAreTheSame => {
                DescriptorError::ExternalAndInternalAreTheSame
            }
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

impl From<BdkBip32Error> for DescriptorKeyError {
    fn from(error: BdkBip32Error) -> DescriptorKeyError {
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

impl From<BdkExtractTxError> for ExtractTxError {
    fn from(error: BdkExtractTxError) -> Self {
        match error {
            BdkExtractTxError::AbsurdFeeRate { fee_rate, .. } => {
                let sat_per_vbyte = fee_rate.to_sat_per_vb_ceil();
                ExtractTxError::AbsurdFeeRate {
                    fee_rate: sat_per_vbyte,
                }
            }
            BdkExtractTxError::MissingInputValue { .. } => ExtractTxError::MissingInputValue,
            BdkExtractTxError::SendingTooMuch { .. } => ExtractTxError::SendingTooMuch,
            _ => ExtractTxError::OtherExtractTxErr,
        }
    }
}

impl From<BdkFromScriptError> for FromScriptError {
    fn from(error: BdkFromScriptError) -> Self {
        match error {
            BdkFromScriptError::UnrecognizedScript => FromScriptError::UnrecognizedScript,
            BdkFromScriptError::WitnessProgram(e) => FromScriptError::WitnessProgram {
                error_message: e.to_string(),
            },
            BdkFromScriptError::WitnessVersion(e) => FromScriptError::WitnessVersion {
                error_message: e.to_string(),
            },
            _ => FromScriptError::OtherFromScriptErr,
        }
    }
}

impl From<BdkParseAmountError> for ParseAmountError {
    fn from(error: BdkParseAmountError) -> Self {
        match error {
            BdkParseAmountError::OutOfRange(_) => ParseAmountError::OutOfRange,
            BdkParseAmountError::TooPrecise(_) => ParseAmountError::TooPrecise,
            BdkParseAmountError::MissingDigits(_) => ParseAmountError::MissingDigits,
            BdkParseAmountError::InputTooLarge(_) => ParseAmountError::InputTooLarge,
            BdkParseAmountError::InvalidCharacter(c) => ParseAmountError::InvalidCharacter {
                error_message: c.to_string(),
            },
            _ => ParseAmountError::OtherParseAmountErr,
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

impl From<BdkPsbtError> for PsbtError {
    fn from(error: BdkPsbtError) -> Self {
        match error {
            BdkPsbtError::InvalidMagic => PsbtError::InvalidMagic,
            BdkPsbtError::MissingUtxo => PsbtError::MissingUtxo,
            BdkPsbtError::InvalidSeparator => PsbtError::InvalidSeparator,
            BdkPsbtError::PsbtUtxoOutOfbounds => PsbtError::PsbtUtxoOutOfBounds,
            BdkPsbtError::InvalidKey(key) => PsbtError::InvalidKey {
                key: key.to_string(),
            },
            BdkPsbtError::InvalidProprietaryKey => PsbtError::InvalidProprietaryKey,
            BdkPsbtError::DuplicateKey(key) => PsbtError::DuplicateKey {
                key: key.to_string(),
            },
            BdkPsbtError::UnsignedTxHasScriptSigs => PsbtError::UnsignedTxHasScriptSigs,
            BdkPsbtError::UnsignedTxHasScriptWitnesses => PsbtError::UnsignedTxHasScriptWitnesses,
            BdkPsbtError::MustHaveUnsignedTx => PsbtError::MustHaveUnsignedTx,
            BdkPsbtError::NoMorePairs => PsbtError::NoMorePairs,
            BdkPsbtError::UnexpectedUnsignedTx { .. } => PsbtError::UnexpectedUnsignedTx,
            BdkPsbtError::NonStandardSighashType(sighash) => {
                PsbtError::NonStandardSighashType { sighash }
            }
            BdkPsbtError::InvalidHash(hash) => PsbtError::InvalidHash {
                hash: hash.to_string(),
            },
            BdkPsbtError::InvalidPreimageHashPair { .. } => PsbtError::InvalidPreimageHashPair,
            BdkPsbtError::CombineInconsistentKeySources(xpub) => {
                PsbtError::CombineInconsistentKeySources {
                    xpub: xpub.to_string(),
                }
            }
            BdkPsbtError::ConsensusEncoding(encoding_error) => PsbtError::ConsensusEncoding {
                encoding_error: encoding_error.to_string(),
            },
            BdkPsbtError::NegativeFee => PsbtError::NegativeFee,
            BdkPsbtError::FeeOverflow => PsbtError::FeeOverflow,
            BdkPsbtError::InvalidPublicKey(e) => PsbtError::InvalidPublicKey {
                error_message: e.to_string(),
            },
            BdkPsbtError::InvalidSecp256k1PublicKey(e) => PsbtError::InvalidSecp256k1PublicKey {
                secp256k1_error: e.to_string(),
            },
            BdkPsbtError::InvalidXOnlyPublicKey => PsbtError::InvalidXOnlyPublicKey,
            BdkPsbtError::InvalidEcdsaSignature(e) => PsbtError::InvalidEcdsaSignature {
                error_message: e.to_string(),
            },
            BdkPsbtError::InvalidTaprootSignature(e) => PsbtError::InvalidTaprootSignature {
                error_message: e.to_string(),
            },
            BdkPsbtError::InvalidControlBlock => PsbtError::InvalidControlBlock,
            BdkPsbtError::InvalidLeafVersion => PsbtError::InvalidLeafVersion,
            BdkPsbtError::Taproot(_) => PsbtError::Taproot,
            BdkPsbtError::TapTree(e) => PsbtError::TapTree {
                error_message: e.to_string(),
            },
            BdkPsbtError::XPubKey(_) => PsbtError::XPubKey,
            BdkPsbtError::Version(e) => PsbtError::Version {
                error_message: e.to_string(),
            },
            BdkPsbtError::PartialDataConsumption => PsbtError::PartialDataConsumption,
            BdkPsbtError::Io(e) => PsbtError::Io {
                error_message: e.to_string(),
            },
            _ => PsbtError::OtherPsbtErr,
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
            BdkSignerError::SighashP2wpkh(e) => SignerError::SighashP2wpkh {
                error_message: e.to_string(),
            },
            BdkSignerError::SighashTaproot(e) => SignerError::SighashTaproot {
                error_message: e.to_string(),
            },
            BdkSignerError::TxInputsIndexError(e) => SignerError::TxInputsIndexError {
                error_message: e.to_string(),
            },
            BdkSignerError::MiniscriptPsbt(e) => SignerError::MiniscriptPsbt {
                error_message: e.to_string(),
            },
            BdkSignerError::External(e) => SignerError::External { error_message: e },
        }
    }
}

impl From<BdkEncodeError> for TransactionError {
    fn from(error: BdkEncodeError) -> Self {
        match error {
            BdkEncodeError::Io(_) => TransactionError::Io,
            BdkEncodeError::OversizedVectorAllocation { .. } => {
                TransactionError::OversizedVectorAllocation
            }
            BdkEncodeError::InvalidChecksum { expected, actual } => {
                TransactionError::InvalidChecksum {
                    expected: DisplayHex::to_lower_hex_string(&expected),
                    actual: DisplayHex::to_lower_hex_string(&actual),
                }
            }
            BdkEncodeError::NonMinimalVarInt => TransactionError::NonMinimalVarInt,
            BdkEncodeError::ParseFailed(_) => TransactionError::ParseFailed,
            BdkEncodeError::UnsupportedSegwitFlag(flag) => {
                TransactionError::UnsupportedSegwitFlag { flag }
            }
            _ => TransactionError::OtherTransactionErr,
        }
    }
}

impl From<BdkSqliteError> for SqliteError {
    fn from(error: BdkSqliteError) -> Self {
        match error {
            BdkSqliteError::Network { expected, given } => {
                SqliteError::InvalidNetwork { expected, given }
            }
            BdkSqliteError::Sqlite(e) => SqliteError::Sqlite {
                rusqlite_error: e.to_string(),
            },
        }
    }
}

impl From<bdk_sqlite::rusqlite::Error> for SqliteError {
    fn from(error: bdk_sqlite::rusqlite::Error) -> Self {
        SqliteError::Sqlite {
            rusqlite_error: error.to_string(),
        }
    }
}

impl From<NewError> for WalletCreationError {
    fn from(error: NewError) -> Self {
        WalletCreationError::Descriptor {
            error_message: error.to_string(),
        }
    }
}

impl From<NewOrLoadError> for WalletCreationError {
    fn from(error: NewOrLoadError) -> Self {
        match error {
            NewOrLoadError::Descriptor(e) => WalletCreationError::Descriptor {
                error_message: e.to_string(),
            },
            NewOrLoadError::LoadedGenesisDoesNotMatch { expected, got } => {
                WalletCreationError::LoadedGenesisDoesNotMatch {
                    expected: expected.to_string(),
                    got: format!("{:?}", got),
                }
            }
            NewOrLoadError::LoadedNetworkDoesNotMatch { expected, got } => {
                WalletCreationError::LoadedNetworkDoesNotMatch { expected, got }
            }
            NewOrLoadError::LoadedDescriptorDoesNotMatch { got, keychain } => {
                WalletCreationError::LoadedDescriptorDoesNotMatch {
                    got: format!("{:?}", got),
                    keychain,
                }
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
        Bip32Error, Bip39Error, CannotConnectError, DescriptorError, DescriptorKeyError,
        ElectrumError, EsploraError, ExtractTxError, FeeRateError, InspectError, PersistenceError,
        PsbtError, PsbtParseError, TransactionError, TxidParseError,
    };
    use crate::SignerError;

    #[test]
    fn test_error_bip32() {
        let cases = vec![
            (
                Bip32Error::CannotDeriveFromHardenedKey,
                "cannot derive from a hardened key",
            ),
            (
                Bip32Error::Secp256k1 {
                    error_message: "failure".to_string(),
                },
                "secp256k1 error: failure",
            ),
            (
                Bip32Error::InvalidChildNumber { child_number: 123 },
                "invalid child number: 123",
            ),
            (
                Bip32Error::InvalidChildNumberFormat,
                "invalid format for child number",
            ),
            (
                Bip32Error::InvalidDerivationPathFormat,
                "invalid derivation path format",
            ),
            (
                Bip32Error::UnknownVersion {
                    version: "0x123".to_string(),
                },
                "unknown version: 0x123",
            ),
            (
                Bip32Error::WrongExtendedKeyLength { length: 512 },
                "wrong extended key length: 512",
            ),
            (
                Bip32Error::Base58 {
                    error_message: "error".to_string(),
                },
                "base58 error: error",
            ),
            (
                Bip32Error::Hex {
                    error_message: "error".to_string(),
                },
                "hexadecimal conversion error: error",
            ),
            (
                Bip32Error::InvalidPublicKeyHexLength { length: 66 },
                "invalid public key hex length: 66",
            ),
            (
                Bip32Error::UnknownError {
                    error_message: "mystery".to_string(),
                },
                "unknown error: mystery",
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
    fn test_error_cannot_connect() {
        let error = CannotConnectError::Include { height: 42 };

        assert_eq!(format!("{}", error), "cannot include height: 42");
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
                "bip32 error: Bip32 error",
            ),
            (
                DescriptorError::Base58 {
                    error_message: "Base58 decode error".to_string(),
                },
                "base58 error: Base58 decode error",
            ),
            (
                DescriptorError::Pk {
                    error_message: "Public key error".to_string(),
                },
                "key-related error: Public key error",
            ),
            (
                DescriptorError::Miniscript {
                    error_message: "Miniscript evaluation error".to_string(),
                },
                "miniscript error: Miniscript evaluation error",
            ),
            (
                DescriptorError::Hex {
                    error_message: "Hexadecimal decoding error".to_string(),
                },
                "hex decoding error: Hexadecimal decoding error",
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
    fn test_error_electrum_client() {
        let cases = vec![
            (
                ElectrumError::IOError { error_message: "message".to_string(), },
                "message",
            ),
            (
                ElectrumError::Json { error_message: "message".to_string(), },
                "message",
            ),
            (
                ElectrumError::Hex { error_message: "message".to_string(), },
                "message",
            ),
            (
                ElectrumError::Protocol { error_message: "message".to_string(), },
                "electrum server error: message",
            ),
            (
                ElectrumError::Bitcoin {
                    error_message: "message".to_string(),
                },
                "message",
            ),
            (
                ElectrumError::AlreadySubscribed,
                "already subscribed to the notifications of an address",
            ),
            (
                ElectrumError::NotSubscribed,
                "not subscribed to the notifications of an address",
            ),
            (
                ElectrumError::InvalidResponse {
                    error_message: "message".to_string(),
                },
                "error during the deserialization of a response from the server: message",
            ),
            (
                ElectrumError::Message {
                    error_message: "message".to_string(),
                },
                "message",
            ),
            (
                ElectrumError::InvalidDNSNameError {
                    domain: "domain".to_string(),
                },
                "invalid domain name domain not matching SSL certificate",
            ),
            (
                ElectrumError::MissingDomain,
                "missing domain while it was explicitly asked to validate it",
            ),
            (
                ElectrumError::AllAttemptsErrored,
                "made one or multiple attempts, all errored",
            ),
            (
                ElectrumError::SharedIOError {
                    error_message: "message".to_string(),
                },
                "message",
            ),
            (
                ElectrumError::CouldntLockReader,
                "couldn't take a lock on the reader mutex. This means that there's already another reader thread is running"
            ),
            (
                ElectrumError::Mpsc,
                "broken IPC communication channel: the other thread probably has exited",
            ),
            (
                ElectrumError::CouldNotCreateConnection {
                    error_message: "message".to_string(),
                },
                "message",
            )
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
                "invalid status code, unable to convert to u16: code 1234567",
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
            (
                EsploraError::RequestAlreadyConsumed,
                "the request has already been consumed",
            ),
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
    fn test_error_inspect() {
        let cases = vec![(
            InspectError::RequestAlreadyConsumed,
            "the request has already been consumed",
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
    fn test_error_psbt() {
        let cases = vec![
            (PsbtError::InvalidMagic, "invalid magic"),
            (
                PsbtError::MissingUtxo,
                "UTXO information is not present in PSBT",
            ),
            (PsbtError::InvalidSeparator, "invalid separator"),
            (
                PsbtError::PsbtUtxoOutOfBounds,
                "output index is out of bounds of non witness script output array",
            ),
            (
                PsbtError::InvalidKey {
                    key: "key".to_string(),
                },
                "invalid key: key",
            ),
            (
                PsbtError::InvalidProprietaryKey,
                "non-proprietary key type found when proprietary key was expected",
            ),
            (
                PsbtError::DuplicateKey {
                    key: "key".to_string(),
                },
                "duplicate key: key",
            ),
            (
                PsbtError::UnsignedTxHasScriptSigs,
                "the unsigned transaction has script sigs",
            ),
            (
                PsbtError::UnsignedTxHasScriptWitnesses,
                "the unsigned transaction has script witnesses",
            ),
            (
                PsbtError::MustHaveUnsignedTx,
                "partially signed transactions must have an unsigned transaction",
            ),
            (
                PsbtError::NoMorePairs,
                "no more key-value pairs for this psbt map",
            ),
            (
                PsbtError::UnexpectedUnsignedTx,
                "different unsigned transaction",
            ),
            (
                PsbtError::NonStandardSighashType { sighash: 200 },
                "non-standard sighash type: 200",
            ),
            (
                PsbtError::InvalidHash {
                    hash: "abcde".to_string(),
                },
                "invalid hash when parsing slice: abcde",
            ),
            (
                PsbtError::InvalidPreimageHashPair,
                "preimage does not match",
            ),
            (
                PsbtError::CombineInconsistentKeySources {
                    xpub: "xpub".to_string(),
                },
                "combine conflict: xpub",
            ),
            (
                PsbtError::ConsensusEncoding {
                    encoding_error: "encoding error".to_string(),
                },
                "bitcoin consensus encoding error: encoding error",
            ),
            (
                PsbtError::NegativeFee,
                "PSBT has a negative fee which is not allowed",
            ),
            (
                PsbtError::FeeOverflow,
                "integer overflow in fee calculation",
            ),
            (
                PsbtError::InvalidPublicKey {
                    error_message: "invalid public key".to_string(),
                },
                "invalid public key invalid public key",
            ),
            (
                PsbtError::InvalidSecp256k1PublicKey {
                    secp256k1_error: "invalid secp256k1 public key".to_string(),
                },
                "invalid secp256k1 public key: invalid secp256k1 public key",
            ),
            (PsbtError::InvalidXOnlyPublicKey, "invalid xonly public key"),
            (
                PsbtError::InvalidEcdsaSignature {
                    error_message: "invalid ecdsa signature".to_string(),
                },
                "invalid ECDSA signature: invalid ecdsa signature",
            ),
            (
                PsbtError::InvalidTaprootSignature {
                    error_message: "invalid taproot signature".to_string(),
                },
                "invalid taproot signature: invalid taproot signature",
            ),
            (PsbtError::InvalidControlBlock, "invalid control block"),
            (PsbtError::InvalidLeafVersion, "invalid leaf version"),
            (PsbtError::Taproot, "taproot error"),
            (
                PsbtError::TapTree {
                    error_message: "tap tree error".to_string(),
                },
                "taproot tree error: tap tree error",
            ),
            (PsbtError::XPubKey, "xpub key error"),
            (
                PsbtError::Version {
                    error_message: "version error".to_string(),
                },
                "version error: version error",
            ),
            (
                PsbtError::PartialDataConsumption,
                "data not consumed entirely when explicitly deserializing",
            ),
            (
                PsbtError::Io {
                    error_message: "io error".to_string(),
                },
                "I/O error: io error",
            ),
            (PsbtError::OtherPsbtErr, "other PSBT error"),
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
                "error in internal psbt data structure: invalid PSBT structure",
            ),
            (
                PsbtParseError::Base64Encoding {
                    error_message: "base64 decode error".to_string(),
                },
                "error in psbt base64 encoding: base64 decode error",
            ),
        ];

        for (error, expected_message) in cases {
            assert_eq!(error.to_string(), expected_message);
        }
    }

    #[test]
    fn test_signer_errors() {
        let errors = vec![
            (SignerError::MissingKey, "missing key for signing"),
            (SignerError::InvalidKey, "invalid key provided"),
            (SignerError::UserCanceled, "user canceled operation"),
            (
                SignerError::InputIndexOutOfRange,
                "input index out of range",
            ),
            (
                SignerError::MissingNonWitnessUtxo,
                "missing non-witness utxo information",
            ),
            (
                SignerError::InvalidNonWitnessUtxo,
                "invalid non-witness utxo information provided",
            ),
            (SignerError::MissingWitnessUtxo, "missing witness utxo"),
            (SignerError::MissingWitnessScript, "missing witness script"),
            (SignerError::MissingHdKeypath, "missing hd keypath"),
            (
                SignerError::NonStandardSighash,
                "non-standard sighash type used",
            ),
            (SignerError::InvalidSighash, "invalid sighash type provided"),
            (
                SignerError::MiniscriptPsbt {
                    error_message: "psbt issue".into(),
                },
                "miniscript psbt error: psbt issue",
            ),
            (
                SignerError::External {
                    error_message: "external error".into(),
                },
                "external error: external error",
            ),
        ];

        for (error, message) in errors {
            assert_eq!(error.to_string(), message);
        }
    }

    #[test]
    fn test_error_transaction() {
        let cases = vec![
            (TransactionError::Io, "io error"),
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
            (TransactionError::NonMinimalVarInt, "non-minimal var int"),
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
}
