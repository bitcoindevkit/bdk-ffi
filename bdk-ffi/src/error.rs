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

#[cfg(test)]
mod test {
    use crate::error::{EsploraError, PersistenceError};
    use crate::CalculateFeeError;
    use crate::OutPoint;

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
}
