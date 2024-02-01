use crate::bitcoin::OutPoint;

use bdk::chain::tx_graph::CalculateFeeError as BdkCalculateFeeError;

use std::fmt;

use bdk::descriptor::DescriptorError;
use bdk::wallet::error::{BuildFeeBumpError, CreateTxError};
use bdk::wallet::tx_builder::{AddUtxoError, AllowShrinkingError};
use bdk::wallet::{NewError, NewOrLoadError};
use bdk_file_store::{FileError, IterError};
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

impl From<FileError> for Alpha3Error {
    fn from(_: FileError) -> Self {
        Alpha3Error::Generic
    }
}

impl From<NewOrLoadError<std::io::Error, IterError>> for Alpha3Error {
    fn from(_: NewOrLoadError<std::io::Error, IterError>) -> Self {
        Alpha3Error::Generic
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

#[cfg(test)]
mod test {
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
}
