use crate::bitcoin::OutPoint;

use bdk::bitcoin::psbt::PsbtParseError as BdkPsbtParseError;
use bdk::bitcoin::Network;
use bdk::chain::tx_graph::CalculateFeeError as BdkCalculateFeeError;
use bdk::descriptor::DescriptorError as BdkDescriptorError;
use bdk::wallet::error::{BuildFeeBumpError, CreateTxError};
use bdk::wallet::tx_builder::{AddUtxoError, AllowShrinkingError};
use bdk::wallet::{NewError, NewOrLoadError};
use bdk_esplora::esplora_client::{Error as BdkEsploraError, Error};
use bdk_file_store::FileError as BdkFileError;
use bdk_file_store::IterError;
use bitcoin_internals::hex::display::DisplayHex;

use bdk::bitcoin::address::ParseError;
use std::convert::Infallible;

#[derive(Debug, thiserror::Error)]
pub enum Alpha3Error {
    #[error("generic error in ffi")]
    Generic,
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
    OtherAddressError,
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
    OtherTransactionError,
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
    OtherExtractTransactionError,
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

impl From<CreateTxError<Infallible>> for Alpha3Error {
    fn from(_: CreateTxError<Infallible>) -> Self {
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

impl From<NewError<std::io::Error>> for Alpha3Error {
    fn from(_: NewError<std::io::Error>) -> Self {
        Alpha3Error::Generic
    }
}

impl From<CreateTxError<std::io::Error>> for Alpha3Error {
    fn from(_: CreateTxError<std::io::Error>) -> Self {
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
            _ => AddressError::OtherAddressError,
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
            _ => AddressError::OtherAddressError,
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
            _ => TransactionError::OtherTransactionError,
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
            _ => ExtractTxError::OtherExtractTransactionError,
        }
    }
}
#[cfg(test)]
mod test {
    use crate::error::{EsploraError, PersistenceError, WalletCreationError};
    use crate::CalculateFeeError;
    use crate::OutPoint;
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
}
