use crate::bitcoin::OutPoint;

use bdk::chain::tx_graph::CalculateFeeError as BdkCalculateFeeError;
use bdk_esplora::esplora_client::Error as BdkEsploraError;

use bdk::bitcoin::Network;
use bdk::descriptor::DescriptorError;
use bdk::wallet::error::{BuildFeeBumpError, CreateTxError};
use bdk::wallet::tx_builder::{AddUtxoError, AllowShrinkingError};
use bdk::wallet::{NewError, NewOrLoadError};
use bdk_file_store::FileError as BdkFileError;
use bdk_file_store::IterError;
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
    #[error("ureq error: {error_message}")]
    Ureq { error_message: String },

    #[error("ureq transport error: {error_message}")]
    UreqTransport { error_message: String },

    #[error("http error with status code: {status_code}")]
    Http { status_code: u16 },

    #[error("io error: {error_message}")]
    Io { error_message: String },

    #[error("no header found in the response")]
    NoHeader,

    #[error("parsing error: {error_message}")]
    Parsing { error_message: String },

    #[error("bitcoin encoding error: {error_message}")]
    BitcoinEncoding { error_message: String },

    #[error("hex decoding error: {error_message}")]
    Hex { error_message: String },

    #[error("transaction not found")]
    TransactionNotFound,

    #[error("header height {height} not found")]
    HeaderHeightNotFound { height: u32 },

    #[error("header hash not found")]
    HeaderHashNotFound,
}

#[derive(Debug, thiserror::Error)]
pub enum FeeRateError {
    #[error("arithmetic overflow on feerate")]
    ArithmeticOverflow,
}

#[derive(Debug, thiserror::Error)]
pub enum AddressError {
    #[error("base58 address encoding error")]
    Base58,

    #[error("bech32 address encoding error")]
    Bech32,

    #[error("the bech32 payload was empty")]
    EmptyBech32Payload,

    #[error("invalid bech32 checksum variant found")]
    InvalidBech32Variant,

    #[error("invalid witness script version: {version}")]
    InvalidWitnessVersion { version: u8 },

    #[error("incorrect format of a witness version byte")]
    UnparsableWitnessVersion,

    #[error(
        "bitcoin script opcode does not match any known witness version, the script is malformed"
    )]
    MalformedWitnessVersion,

    #[error("the witness program must be between 2 and 40 bytes in length: length={length}")]
    InvalidWitnessProgramLength { length: u64 },

    #[error("a v0 witness program must be either of length 20 or 32 bytes: length={length}")]
    InvalidSegwitV0ProgramLength { length: u64 },

    #[error("an uncompressed pubkey was used where it is not allowed")]
    UncompressedPubkey,

    #[error("script size exceed 520 bytes")]
    ExcessiveScriptSize,

    #[error("script is not p2pkh, p2sh, or witness program")]
    UnrecognizedScript,

    #[error("unknown address type: '{s}' is either invalid or not supported")]
    UnknownAddressType { s: String },

    #[error(
        "address {address} belongs to network {found} which is different from required {required}"
    )]
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

impl From<DescriptorError> for Alpha3Error {
    fn from(_: DescriptorError) -> Self {
        Alpha3Error::Generic
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
            BdkEsploraError::Ureq(e) => EsploraError::Ureq {
                error_message: e.to_string(),
            },
            BdkEsploraError::UreqTransport(e) => EsploraError::UreqTransport {
                error_message: e.to_string(),
            },
            BdkEsploraError::HttpResponse(code) => EsploraError::Http { status_code: code },
            BdkEsploraError::Io(e) => EsploraError::Io {
                error_message: e.to_string(),
            },
            BdkEsploraError::NoHeader => EsploraError::NoHeader,
            BdkEsploraError::Parsing(e) => EsploraError::Parsing {
                error_message: e.to_string(),
            },
            BdkEsploraError::BitcoinEncoding(e) => EsploraError::BitcoinEncoding {
                error_message: e.to_string(),
            },
            BdkEsploraError::Hex(e) => EsploraError::Hex {
                error_message: e.to_string(),
            },
            BdkEsploraError::TransactionNotFound(_) => EsploraError::TransactionNotFound,
            BdkEsploraError::HeaderHeightNotFound(height) => {
                EsploraError::HeaderHeightNotFound { height }
            }
            BdkEsploraError::HeaderHashNotFound(_) => EsploraError::HeaderHashNotFound,
        }
    }
}

impl From<bdk::bitcoin::address::Error> for AddressError {
    fn from(error: bdk::bitcoin::address::Error) -> Self {
        match error {
            bdk::bitcoin::address::Error::Base58(_) => AddressError::Base58,
            bdk::bitcoin::address::Error::Bech32(_) => AddressError::Bech32,
            bdk::bitcoin::address::Error::EmptyBech32Payload => AddressError::EmptyBech32Payload,
            bdk::bitcoin::address::Error::InvalidBech32Variant { .. } => {
                AddressError::InvalidBech32Variant
            }
            bdk::bitcoin::address::Error::InvalidWitnessVersion(version) => {
                AddressError::InvalidWitnessVersion { version }
            }
            bdk::bitcoin::address::Error::UnparsableWitnessVersion(_) => {
                AddressError::UnparsableWitnessVersion
            }
            bdk::bitcoin::address::Error::MalformedWitnessVersion => {
                AddressError::MalformedWitnessVersion
            }
            bdk::bitcoin::address::Error::InvalidWitnessProgramLength(length) => {
                let length = length as u64;
                AddressError::InvalidWitnessProgramLength { length }
            }
            bdk::bitcoin::address::Error::InvalidSegwitV0ProgramLength(length) => {
                let length = length as u64;
                AddressError::InvalidSegwitV0ProgramLength { length }
            }
            bdk::bitcoin::address::Error::UncompressedPubkey => AddressError::UncompressedPubkey,
            bdk::bitcoin::address::Error::ExcessiveScriptSize => AddressError::ExcessiveScriptSize,
            bdk::bitcoin::address::Error::UnrecognizedScript => AddressError::UnrecognizedScript,
            bdk::bitcoin::address::Error::UnknownAddressType(s) => {
                AddressError::UnknownAddressType { s }
            }
            bdk::bitcoin::address::Error::NetworkValidation {
                required,
                found,
                address,
            } => {
                // let address = address.to_string();
                let address = format!("{:?}", address);
                AddressError::NetworkValidation {
                    required,
                    found,
                    address,
                }
            }
            _ => AddressError::OtherAddressError,
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
                EsploraError::Ureq {
                    error_message: "Network error".to_string(),
                },
                "ureq error: Network error",
            ),
            (
                EsploraError::UreqTransport {
                    error_message: "Timeout occurred".to_string(),
                },
                "ureq transport error: Timeout occurred",
            ),
            (
                EsploraError::Http { status_code: 404 },
                "http error with status code: 404",
            ),
            (
                EsploraError::Io {
                    error_message: "File not found".to_string(),
                },
                "io error: File not found",
            ),
            (EsploraError::NoHeader, "no header found in the response"),
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
                EsploraError::Hex {
                    error_message: "Invalid hex".to_string(),
                },
                "hex decoding error: Invalid hex",
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
