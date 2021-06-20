//use ::safer_ffi::prelude::*;
use bdk::Error;

pub fn error_name(error: &bdk::Error) -> &'static str {
    match error {
        Error::InvalidU32Bytes(_) => "InvalidU32Bytes",
        Error::Generic(_) => "Generic",
        Error::ScriptDoesntHaveAddressForm => "ScriptDoesntHaveAddressForm",
        Error::SingleRecipientMultipleOutputs => "SingleRecipientMultipleOutputs",
        Error::SingleRecipientNoInputs => "SingleRecipientNoInputs",
        Error::NoRecipients => "NoRecipients",
        Error::NoUtxosSelected => "NoUtxosSelected",
        Error::OutputBelowDustLimit(_) => "OutputBelowDustLimit",
        Error::InsufficientFunds { .. } => "InsufficientFunds",
        Error::BnBTotalTriesExceeded => "BnBTotalTriesExceeded",
        Error::BnBNoExactMatch => "BnBNoExactMatch",
        Error::UnknownUtxo => "UnknownUtxo",
        Error::TransactionNotFound => "TransactionNotFound",
        Error::TransactionConfirmed => "TransactionConfirmed",
        Error::IrreplaceableTransaction => "IrreplaceableTransaction",
        Error::FeeRateTooLow { .. } => "FeeRateTooLow",
        Error::FeeTooLow { .. } => "FeeTooLow",
        Error::MissingKeyOrigin(_) => "MissingKeyOrigin",
        Error::Key(_) => "Key",
        Error::ChecksumMismatch => "ChecksumMismatch",
        Error::SpendingPolicyRequired(_) => "SpendingPolicyRequired",
        Error::InvalidPolicyPathError(_) => "InvalidPolicyPathError",
        Error::Signer(_) => "Signer",
        Error::InvalidProgressValue(_) => "InvalidProgressValue",
        Error::ProgressUpdateError => "ProgressUpdateError",
        Error::InvalidOutpoint(_) => "InvalidOutpoint",
        Error::Descriptor(_) => "Descriptor",
        Error::AddressValidator(_) => "AddressValidator",
        Error::Encode(_) => "Encode",
        Error::Miniscript(_) => "Miniscript",
        Error::Bip32(_) => "Bip32",
        Error::Secp256k1(_) => "Secp256k1",
        Error::Json(_) => "Json",
        Error::Hex(_) => "Hex",
        Error::Psbt(_) => "Psbt",
        Error::Electrum(_) => "Electrum",
//        Error::Esplora(_) => {}
//        Error::CompactFilters(_) => {}
        Error::Sled(_) => "Sled",
        _ => "Unknown",
    }
}

//// Errors that can be thrown by the [`Wallet`](crate::wallet::Wallet)
//// Simplified to work over the FFI interface
//#[derive_ReprC]
//#[repr(i16)]
//#[derive(Debug)]
//pub enum WalletError {
//    None,
//    InvalidU32Bytes,
//    Generic,
//    ScriptDoesntHaveAddressForm,
//    SingleRecipientMultipleOutputs,
//    SingleRecipientNoInputs,
//    NoRecipients,
//    NoUtxosSelected,
//    OutputBelowDustLimit,
//    InsufficientFunds,
//    BnBTotalTriesExceeded,
//    BnBNoExactMatch,
//    UnknownUtxo,
//    TransactionNotFound,
//    TransactionConfirmed,
//    IrreplaceableTransaction,
//    FeeRateTooLow,
//    FeeTooLow,
//    FeeRateUnavailable,
//    MissingKeyOrigin,
//    Key,
//    ChecksumMismatch,
//    SpendingPolicyRequired,
//    InvalidPolicyPathError,
//    Signer,
//    InvalidNetwork,
//    InvalidProgressValue,
//    ProgressUpdateError,
//    InvalidOutpoint,
//    Descriptor,
//    AddressValidator,
//    Encode,
//    Miniscript,
//    Bip32,
//    Secp256k1,
//    Json,
//    Hex,
//    Psbt,
//    PsbtParse,
//    //#[cfg(feature = "electrum")]
//    Electrum,
//    //#[cfg(feature = "esplora")]
//    //Esplora,
//    //#[cfg(feature = "compact_filters")]
//    //CompactFilters,
//    //#[cfg(feature = "key-value-db")]
//    Sled,
//}

//impl From<bdk::Error> for WalletError {
//    fn from(error: bdk::Error) -> Self {
//        match error {
//            Error::InvalidU32Bytes(_) => WalletError::InvalidNetwork,
//            Error::Generic(_) => WalletError::Generic,
//            Error::ScriptDoesntHaveAddressForm => WalletError::ScriptDoesntHaveAddressForm,
//            Error::SingleRecipientMultipleOutputs => WalletError::SingleRecipientMultipleOutputs,
//            Error::SingleRecipientNoInputs => WalletError::SingleRecipientNoInputs,
//            Error::NoRecipients => WalletError::NoRecipients,
//            Error::NoUtxosSelected => WalletError::NoUtxosSelected,
//            Error::OutputBelowDustLimit(_) => WalletError::OutputBelowDustLimit,
//            Error::InsufficientFunds { .. } => WalletError::InsufficientFunds,
//            Error::BnBTotalTriesExceeded => WalletError::BnBTotalTriesExceeded,
//            Error::BnBNoExactMatch => WalletError::BnBNoExactMatch,
//            Error::UnknownUtxo => WalletError::UnknownUtxo,
//            Error::TransactionNotFound => WalletError::TransactionNotFound,
//            Error::TransactionConfirmed => WalletError::TransactionConfirmed,
//            Error::IrreplaceableTransaction => WalletError::IrreplaceableTransaction,
//            Error::FeeRateTooLow { .. } => WalletError::FeeRateTooLow,
//            Error::FeeTooLow { .. } => WalletError::FeeTooLow,
//            Error::MissingKeyOrigin(_) => WalletError::MissingKeyOrigin,
//            Error::Key(_) => WalletError::Key,
//            Error::ChecksumMismatch => WalletError::ChecksumMismatch,
//            Error::SpendingPolicyRequired(_) => WalletError::SpendingPolicyRequired,
//            Error::InvalidPolicyPathError(_) => WalletError::InvalidPolicyPathError,
//            Error::Signer(_) => WalletError::Signer,
//            Error::InvalidProgressValue(_) => WalletError::InvalidProgressValue,
//            Error::ProgressUpdateError => WalletError::ProgressUpdateError,
//            Error::InvalidOutpoint(_) => WalletError::InvalidOutpoint,
//            Error::Descriptor(_) => WalletError::Descriptor,
//            Error::AddressValidator(_) => WalletError::AddressValidator,
//            Error::Encode(_) => WalletError::Encode,
//            Error::Miniscript(_) => WalletError::Miniscript,
//            Error::Bip32(_) => WalletError::Bip32,
//            Error::Secp256k1(_) => WalletError::Secp256k1,
//            Error::Json(_) => WalletError::Json,
//            Error::Hex(_) => WalletError::Hex,
//            Error::Psbt(_) => WalletError::Psbt,
//            Error::Electrum(_) => WalletError::Electrum,
//            //Error::Esplora(_) => WalletError::Esplora,
//            //Error::CompactFilters(_) => {}
//            Error::Sled(_) => WalletError::Sled,
//        }
//    }
//}

//type error_code = i16;
//
//impl From<bdk::Error> for error_code {
//    fn from(error: bdk::Error) -> Self {
//        match error {
//            Error::InvalidU32Bytes(_) => 1,
//            Error::Generic(_) => 2,
//            Error::ScriptDoesntHaveAddressForm => 3,
//            Error::SingleRecipientMultipleOutputs => 4,
//            Error::SingleRecipientNoInputs => 5,
//            Error::NoRecipients => 6,
//            Error::NoUtxosSelected => 7,
//            Error::OutputBelowDustLimit(_) => 8,
//            Error::InsufficientFunds { .. } => 9,
//            Error::BnBTotalTriesExceeded => 10,
//            Error::BnBNoExactMatch => 11,
//            Error::UnknownUtxo => 12,
//            Error::TransactionNotFound => 13,
//            Error::TransactionConfirmed => 14,
//            Error::IrreplaceableTransaction => 15,
//            Error::FeeRateTooLow { .. } => 16,
//            Error::FeeTooLow { .. } => 17,
//            Error::MissingKeyOrigin(_) => 18,
//            Error::Key(_) => 19,
//            Error::ChecksumMismatch => 20,
//            Error::SpendingPolicyRequired(_) => 21,
//            Error::InvalidPolicyPathError(_) => 22,
//            Error::Signer(_) => 23,
//            Error::InvalidProgressValue(_) => 24,
//            Error::ProgressUpdateError => 25,
//            Error::InvalidOutpoint(_) => 26,
//            Error::Descriptor(_) => 27,
//            Error::AddressValidator(_) => 28,
//            Error::Encode(_) => 29,
//            Error::Miniscript(_) => 30,
//            Error::Bip32(_) => 31,
//            Error::Secp256k1(_) => 32,
//            Error::Json(_) => 33,
//            Error::Hex(_) => 34,
//            Error::Psbt(_) => 35,
//            Error::Electrum(_) => 36,
//            //Error::Esplora(_) => WalletError::Esplora,
//            //Error::CompactFilters(_) => {}
//            Error::Sled(_) => 37,
//            _ => -1
//        }
//    }
//}
