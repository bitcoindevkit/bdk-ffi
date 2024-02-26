use crate::bitcoin::OutPoint;

use bdk::chain::tx_graph::CalculateFeeError as BdkCalculateFeeError;
use bdk_esplora::esplora_client::Error as BdkEsploraError;

use std::fmt;

use bdk::bitcoin::Network;
use bdk::descriptor::DescriptorError;
use bdk::wallet::error::{BuildFeeBumpError, CreateTxError};
use bdk::wallet::tx_builder::{AddUtxoError, AllowShrinkingError};
use bdk::wallet::{NewError, NewOrLoadError};
use bdk_file_store::FileError as BdkFileError;
use bdk_file_store::IterError;
use std::convert::Infallible;

#[derive(Debug)]
pub enum Alpha3Error {
    Generic,
}

impl fmt::Display for Alpha3Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Alpha3Error::Generic => write!(f, "Error in FFI"),
        }
    }
}

impl std::error::Error for Alpha3Error {}

#[derive(Debug)]
pub enum WalletError {
    // Errors coming from the FileError enum
    Io {
        e: String,
    },
    InvalidMagicBytes {
        got: Vec<u8>,
        expected: Vec<u8>,
    },

    // Errors coming from the NewOrLoadError enum
    Descriptor,
    Write,
    Load,
    NotInitialized,
    LoadedGenesisDoesNotMatch,
    LoadedNetworkDoesNotMatch {
        expected: Network,
        got: Option<Network>,
    },
}

impl fmt::Display for WalletError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io { e } => write!(f, "io error trying to read file: {}", e),
            Self::InvalidMagicBytes { got, expected } => write!(
                f,
                "file has invalid magic bytes: expected={:?} got={:?}",
                expected, got,
            ),
            Self::Descriptor => write!(f, "error with descriptor"),
            Self::Write => write!(f, "failed to write to persistence"),
            Self::Load => write!(f, "failed to load from persistence"),
            Self::NotInitialized => {
                write!(f, "wallet is not initialized, persistence backend is empty")
            }
            Self::LoadedGenesisDoesNotMatch => {
                write!(f, "loaded genesis hash does not match the expected one")
            }
            Self::LoadedNetworkDoesNotMatch { expected, got } => {
                write!(f, "loaded network type is not {}, got {:?}", expected, got)
            }
        }
    }
}

impl std::error::Error for WalletError {}

impl From<BdkFileError> for WalletError {
    fn from(error: BdkFileError) -> Self {
        match error {
            BdkFileError::Io(_) => WalletError::Io {
                e: "io error trying to read file".to_string(),
            },
            BdkFileError::InvalidMagicBytes { got, expected } => {
                WalletError::InvalidMagicBytes { got, expected }
            }
        }
    }
}

impl From<NewOrLoadError<std::io::Error, IterError>> for WalletError {
    fn from(error: NewOrLoadError<std::io::Error, IterError>) -> Self {
        match error {
            NewOrLoadError::Descriptor(_) => WalletError::Descriptor,
            NewOrLoadError::Write(_) => WalletError::Write,
            NewOrLoadError::Load(_) => WalletError::Load,
            NewOrLoadError::NotInitialized => WalletError::NotInitialized,
            NewOrLoadError::LoadedGenesisDoesNotMatch { .. } => {
                WalletError::LoadedGenesisDoesNotMatch
            }
            NewOrLoadError::LoadedNetworkDoesNotMatch { expected, got } => {
                WalletError::LoadedNetworkDoesNotMatch { expected, got }
            }
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

#[derive(Debug)]
pub enum CalculateFeeError {
    MissingTxOut { out_points: Vec<OutPoint> },
    NegativeFee { fee: i64 },
}

impl fmt::Display for CalculateFeeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CalculateFeeError::MissingTxOut { out_points } => {
                write!(f, "Missing transaction output: {:?}", out_points)
            }
            CalculateFeeError::NegativeFee { fee } => write!(f, "Negative fee value: {}", fee),
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

impl std::error::Error for CalculateFeeError {}

#[derive(Debug)]
pub enum EsploraError {
    Ureq { error_message: String },
    UreqTransport { error_message: String },
    Http { status_code: u16 },
    Io { error_message: String },
    NoHeader,
    Parsing { error_message: String },
    BitcoinEncoding { error_message: String },
    Hex { error_message: String },
    TransactionNotFound,
    HeaderHeightNotFound { height: u32 },
    HeaderHashNotFound,
}

impl fmt::Display for EsploraError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EsploraError::Ureq { error_message } => write!(f, "Ureq error: {}", error_message),
            EsploraError::UreqTransport { error_message } => {
                write!(f, "Ureq transport error: {}", error_message)
            }
            EsploraError::Http { status_code } => {
                write!(f, "HTTP error with status code: {}", status_code)
            }
            EsploraError::Io { error_message } => write!(f, "IO error: {}", error_message),
            EsploraError::NoHeader => write!(f, "No header found in the response"),
            EsploraError::Parsing { error_message } => {
                write!(f, "Parsing error: {}", error_message)
            }
            EsploraError::BitcoinEncoding { error_message } => {
                write!(f, "Bitcoin encoding error: {}", error_message)
            }
            EsploraError::Hex { error_message } => {
                write!(f, "Hex decoding error: {}", error_message)
            }
            EsploraError::TransactionNotFound => write!(f, "Transaction not found"),
            EsploraError::HeaderHeightNotFound { height } => {
                write!(f, "Header height {} not found", height)
            }
            EsploraError::HeaderHashNotFound => write!(f, "Header hash not found"),
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

impl std::error::Error for EsploraError {}

#[cfg(test)]
mod test {
    use crate::error::EsploraError;
    use crate::CalculateFeeError;
    use crate::OutPoint;
    use crate::WalletError;
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
            "Missing transaction output: [{:?}, {:?}]",
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

        assert_eq!(error.to_string(), "Negative fee value: -100");
    }

    #[test]
    fn test_esplora_errors() {
        let cases = vec![
            (
                EsploraError::Ureq {
                    error_message: "Network error".to_string(),
                },
                "Ureq error: Network error",
            ),
            (
                EsploraError::UreqTransport {
                    error_message: "Timeout occurred".to_string(),
                },
                "Ureq transport error: Timeout occurred",
            ),
            (
                EsploraError::Http { status_code: 404 },
                "HTTP error with status code: 404",
            ),
            (
                EsploraError::Io {
                    error_message: "File not found".to_string(),
                },
                "IO error: File not found",
            ),
            (EsploraError::NoHeader, "No header found in the response"),
            (
                EsploraError::Parsing {
                    error_message: "Invalid JSON".to_string(),
                },
                "Parsing error: Invalid JSON",
            ),
            (
                EsploraError::BitcoinEncoding {
                    error_message: "Bad format".to_string(),
                },
                "Bitcoin encoding error: Bad format",
            ),
            (
                EsploraError::Hex {
                    error_message: "Invalid hex".to_string(),
                },
                "Hex decoding error: Invalid hex",
            ),
            (EsploraError::TransactionNotFound, "Transaction not found"),
            (
                EsploraError::HeaderHeightNotFound { height: 123456 },
                "Header height 123456 not found",
            ),
            (EsploraError::HeaderHashNotFound, "Header hash not found"),
        ];

        for (error, expected_message) in cases {
            assert_eq!(error.to_string(), expected_message);
        }
    }

    #[test]
    fn test_wallet_error() {
        let io_error = WalletError::Io {
            e: "io error".to_string(),
        };
        assert_eq!(
            io_error.to_string(),
            "io error trying to read file: io error"
        );

        let invalid_magic_bytes_error = WalletError::InvalidMagicBytes {
            got: vec![1, 2, 3, 4],
            expected: vec![4, 3, 2, 1],
        };
        assert_eq!(
            invalid_magic_bytes_error.to_string(),
            "file has invalid magic bytes: expected=[4, 3, 2, 1] got=[1, 2, 3, 4]"
        );

        let descriptor_error = WalletError::Descriptor;
        assert_eq!(descriptor_error.to_string(), "error with descriptor");

        let write_error = WalletError::Write;
        assert_eq!(write_error.to_string(), "failed to write to persistence");

        let load_error = WalletError::Load;
        assert_eq!(load_error.to_string(), "failed to load from persistence");

        let not_initialized_error = WalletError::NotInitialized;
        assert_eq!(
            not_initialized_error.to_string(),
            "wallet is not initialized, persistence backend is empty"
        );

        let loaded_genesis_does_not_match_error = WalletError::LoadedGenesisDoesNotMatch;
        assert_eq!(
            loaded_genesis_does_not_match_error.to_string(),
            "loaded genesis hash does not match the expected one"
        );

        let loaded_network_does_not_match_error = WalletError::LoadedNetworkDoesNotMatch {
            expected: Network::Bitcoin,
            got: Some(Network::Testnet),
        };
        assert_eq!(
            loaded_network_does_not_match_error.to_string(),
            "loaded network type is not bitcoin, got Some(Testnet)"
        );
    }
}
