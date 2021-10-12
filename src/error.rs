use bdk::Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FfiError {
    #[error("data store disconnected")]
    None,
    #[error("data store disconnected")]
    InvalidU32Bytes,
    #[error("data store disconnected")]
    Generic,
    #[error("data store disconnected")]
    ScriptDoesntHaveAddressForm,
    #[error("data store disconnected")]
    NoRecipients,
    #[error("data store disconnected")]
    NoUtxosSelected,
    #[error("data store disconnected")]
    OutputBelowDustLimit,
    #[error("data store disconnected")]
    InsufficientFunds,
    #[error("data store disconnected")]
    BnBTotalTriesExceeded,
    #[error("data store disconnected")]
    BnBNoExactMatch,
    #[error("data store disconnected")]
    UnknownUtxo,
    #[error("data store disconnected")]
    TransactionNotFound,
    #[error("data store disconnected")]
    TransactionConfirmed,
    #[error("data store disconnected")]
    IrreplaceableTransaction,
    #[error("data store disconnected")]
    FeeRateTooLow,
    #[error("data store disconnected")]
    FeeTooLow,
    #[error("data store disconnected")]
    FeeRateUnavailable,
    #[error("data store disconnected")]
    MissingKeyOrigin,
    #[error("data store disconnected")]
    Key,
    #[error("data store disconnected")]
    ChecksumMismatch,
    #[error("data store disconnected")]
    SpendingPolicyRequired,
    #[error("data store disconnected")]
    InvalidPolicyPathError,
    #[error("data store disconnected")]
    Signer,
    #[error("data store disconnected")]
    InvalidNetwork,
    #[error("data store disconnected")]
    InvalidProgressValue,
    #[error("data store disconnected")]
    ProgressUpdateError,
    #[error("data store disconnected")]
    InvalidOutpoint,
    #[error("data store disconnected")]
    Descriptor,
    #[error("data store disconnected")]
    AddressValidator,
    #[error("data store disconnected")]
    Encode,
    #[error("data store disconnected")]
    Miniscript,
    #[error("data store disconnected")]
    Bip32,
    #[error("data store disconnected")]
    Secp256k1,
    #[error("data store disconnected")]
    Json,
    #[error("data store disconnected")]
    Hex,
    #[error("data store disconnected")]
    Psbt,
    #[error("data store disconnected")]
    PsbtParse,
    #[error("data store disconnected")]
    Electrum,
    // Esplora,
    // CompactFilters,
    #[error("data store disconnected")]
    Sled,
}

impl From<bdk::Error> for FfiError {
    fn from(error: bdk::Error) -> Self {
        match error {
            Error::InvalidU32Bytes(_) => FfiError::InvalidU32Bytes,
            Error::Generic(_) => FfiError::Generic,
            Error::ScriptDoesntHaveAddressForm => FfiError::ScriptDoesntHaveAddressForm,
            Error::NoRecipients => FfiError::NoRecipients,
            Error::NoUtxosSelected => FfiError::NoUtxosSelected,
            Error::OutputBelowDustLimit(_) => FfiError::OutputBelowDustLimit,
            Error::InsufficientFunds { .. } => FfiError::InsufficientFunds,
            Error::BnBTotalTriesExceeded => FfiError::BnBTotalTriesExceeded,
            Error::BnBNoExactMatch => FfiError::BnBNoExactMatch,
            Error::UnknownUtxo => FfiError::UnknownUtxo,
            Error::TransactionNotFound => FfiError::TransactionNotFound,
            Error::TransactionConfirmed => FfiError::TransactionConfirmed,
            Error::IrreplaceableTransaction => FfiError::IrreplaceableTransaction,
            Error::FeeRateTooLow { .. } => FfiError::FeeRateTooLow,
            Error::FeeTooLow { .. } => FfiError::FeeTooLow,
            Error::FeeRateUnavailable => FfiError::FeeRateUnavailable,
            Error::MissingKeyOrigin(_) => FfiError::MissingKeyOrigin,
            Error::Key(_) => FfiError::Key,
            Error::ChecksumMismatch => FfiError::ChecksumMismatch,
            Error::SpendingPolicyRequired(_) => FfiError::SpendingPolicyRequired,
            Error::InvalidPolicyPathError(_) => FfiError::InvalidPolicyPathError,
            Error::Signer(_) => FfiError::Signer,
            Error::InvalidNetwork { .. } => FfiError::InvalidNetwork,
            Error::InvalidProgressValue(_) => FfiError::InvalidProgressValue,
            Error::ProgressUpdateError => FfiError::ProgressUpdateError,
            Error::InvalidOutpoint(_) => FfiError::InvalidOutpoint,
            Error::Descriptor(_) => FfiError::Descriptor,
            Error::AddressValidator(_) => FfiError::AddressValidator,
            Error::Encode(_) => FfiError::Encode,
            Error::Miniscript(_) => FfiError::Miniscript,
            Error::Bip32(_) => FfiError::Bip32,
            Error::Secp256k1(_) => FfiError::Secp256k1,
            Error::Json(_) => FfiError::Json,
            Error::Hex(_) => FfiError::Hex,
            Error::Psbt(_) => FfiError::Psbt,
            Error::PsbtParse(_) => FfiError::PsbtParse,
            Error::Electrum(_) => FfiError::Electrum,
            //        Error::Esplora(_) => JniError::Esplora,
            //        Error::CompactFilters(_) => JniError::CompactFilters,
            //        Error::Rpc(_) => JniError::Rpc,
            Error::Sled(_) => FfiError::Sled,
        }
    }
}
