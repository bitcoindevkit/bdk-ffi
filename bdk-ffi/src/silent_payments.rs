use crate::bitcoin::{Amount, Network};

use bdk_sp::bitcoin::bech32::primitives::{decode::CheckedHrpstring, Bech32m};
use bdk_sp::encoding::{
    ParseError as BdkSilentPaymentCodeParseError, SilentPaymentCode as BdkSilentPaymentCode,
};
use bdk_sp::send::error::SpSendError;

use std::convert::TryFrom;
use std::fmt::Display;
use std::sync::Arc;

/// Represents a silent payment code containing the necessary keys and network information.
#[derive(Clone, Debug, Eq, PartialEq, uniffi::Object)]
#[uniffi::export(Eq, Display)]
pub struct SilentPaymentCode(pub(crate) BdkSilentPaymentCode);

#[uniffi::export]
impl SilentPaymentCode {
    /// Attempts to parse a string as a silent payment code.
    #[uniffi::constructor]
    pub fn new(code: String) -> Result<Self, SilentPaymentCodeParseError> {
        let checked_code = CheckedHrpstring::new::<Bech32m>(&code).map_err(|error| {
            SilentPaymentCodeParseError::Bech32 {
                error_message: error.to_string(),
            }
        })?;
        if checked_code
            .fe32_iter::<&mut dyn Iterator<Item = u8>>()
            .next()
            .is_none()
        {
            return Err(SilentPaymentCodeParseError::Version {
                error_message: "payload length does not match version spec".to_string(),
            });
        }

        BdkSilentPaymentCode::try_from(code.as_str())
            .map(Self)
            .map_err(SilentPaymentCodeParseError::from)
    }

    /// Returns whether this silent payment code can be used on `network`.
    pub fn is_valid_for_network(&self, network: Network) -> bool {
        matches!(
            (self.0.network, network),
            (Network::Bitcoin, Network::Bitcoin)
                | (Network::Regtest, Network::Regtest)
                | (
                    Network::Testnet | Network::Testnet4 | Network::Signet,
                    Network::Testnet | Network::Testnet4 | Network::Signet
                )
        )
    }
}

impl Display for SilentPaymentCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

/// A silent payment code and associated amount.
#[derive(uniffi::Record)]
pub struct SilentPaymentRecipient {
    /// The recipient's silent payment code.
    pub code: Arc<SilentPaymentCode>,
    /// The amount to send.
    pub amount: Arc<Amount>,
}

#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum SilentPaymentCodeParseError {
    #[error("invalid Bech32m silent payment code: {error_message}")]
    Bech32 { error_message: String },

    #[error("unsupported silent payment code version: {error_message}")]
    Version { error_message: String },

    #[error("unsupported silent payment code human-readable prefix: {error_message}")]
    UnknownHrp { error_message: String },

    #[error("invalid silent payment public key: {error_message}")]
    InvalidPublicKey { error_message: String },
}

impl From<BdkSilentPaymentCodeParseError> for SilentPaymentCodeParseError {
    fn from(error: BdkSilentPaymentCodeParseError) -> Self {
        match error {
            BdkSilentPaymentCodeParseError::Bech32(error) => Self::Bech32 {
                error_message: error.to_string(),
            },
            BdkSilentPaymentCodeParseError::Version(error) => Self::Version {
                error_message: error.to_string(),
            },
            BdkSilentPaymentCodeParseError::UnknownHrp(error) => Self::UnknownHrp {
                error_message: error.to_string(),
            },
            BdkSilentPaymentCodeParseError::InvalidPubKey(error) => Self::InvalidPublicKey {
                error_message: error.to_string(),
            },
        }
    }
}

#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum SilentPaymentSendError {
    #[error("at least one silent payment recipient is required")]
    NoRecipients,

    #[error("a silent payment scan key cannot have more than 2323 recipients")]
    RecipientLimitExceeded,

    #[error("silent payment code {code} is not valid for wallet network {wallet_network}")]
    NetworkMismatch {
        code: String,
        wallet_network: String,
    },

    #[error("foreign inputs are not supported for silent payment sending")]
    ForeignInputsUnsupported,

    #[error("only_witness_utxo is not supported for silent payment sending")]
    OnlyWitnessUtxoUnsupported,

    #[error("P2SH inputs are not supported for silent payment sending")]
    P2shInputsUnsupported,

    #[error("replace-by-fee is not supported for silent payment sending")]
    RbfUnsupported,

    #[error("silent payment sending only supports SIGHASH_ALL or Taproot SIGHASH_DEFAULT")]
    UnsupportedSighash,

    #[error("failed to create silent payment transaction: {error_message}")]
    CreateTransaction { error_message: String },

    #[error("the initial silent payment signing pass failed: {error_message}")]
    InitialSigning { error_message: String },

    #[error("the wallet could not finalize the initial silent payment signing pass")]
    InitialSigningIncomplete,

    #[error("silent payment secp256k1 derivation failed: {error_message}")]
    Secp256k1 { error_message: String },

    #[error("silent payment BIP32 derivation failed: {error_message}")]
    Bip32 { error_message: String },

    #[error("silent payment derivation requires at least one input outpoint: {error_message}")]
    NoOutpoints { error_message: String },

    #[error("no eligible inputs are available for silent payment derivation")]
    MissingInputsForSharedSecretDerivation,

    #[error("an input is missing its finalized witness")]
    MissingWitness,

    #[error("an input is missing its previous output")]
    MissingPrevout,

    #[error("a transaction output index is invalid: {error_message}")]
    OutputIndex { error_message: String },

    #[error("the PSBT is missing a silent payment placeholder output")]
    MissingPlaceholderScript,

    #[error("a required private key for an eligible silent payment input is unavailable")]
    MissingEligibleInputKey,

    #[error("there are fewer silent payment derivations than placeholder outputs")]
    MissingDerivations,

    #[error("there are fewer placeholder outputs than silent payment derivations")]
    MissingOutputs,

    #[error(
        "expected {expected} silent payment placeholder outputs in the built transaction, found {actual}"
    )]
    PlaceholderOutputMismatch { expected: u64, actual: u64 },

    #[error("silent payment output derivation left a placeholder output in the transaction")]
    PlaceholderNotReplaced,

    #[error("the final silent payment signing pass failed: {error_message}")]
    FinalSigning { error_message: String },

    #[error("the wallet could not finalize the final silent payment signing pass")]
    FinalSigningIncomplete,

    #[error("failed to extract the signed silent payment transaction: {error_message}")]
    ExtractTransaction { error_message: String },
}

impl From<SpSendError> for SilentPaymentSendError {
    fn from(error: SpSendError) -> Self {
        match error {
            SpSendError::Secp256k1Error(error) => Self::Secp256k1 {
                error_message: error.to_string(),
            },
            SpSendError::Bip32Error(error) => Self::Bip32 {
                error_message: error.to_string(),
            },
            SpSendError::NoOutpoints(error) => Self::NoOutpoints {
                error_message: error.to_string(),
            },
            SpSendError::MissingInputsForSharedSecretDerivation => {
                Self::MissingInputsForSharedSecretDerivation
            }
            SpSendError::MissingWitness => Self::MissingWitness,
            SpSendError::MissingPrevout => Self::MissingPrevout,
            SpSendError::IndexError(error) => Self::OutputIndex {
                error_message: error.to_string(),
            },
            SpSendError::MissingPlaceholderScript => Self::MissingPlaceholderScript,
            SpSendError::KeyError => Self::MissingEligibleInputKey,
            SpSendError::MissingDerivations => Self::MissingDerivations,
            SpSendError::MissingOutputs => Self::MissingOutputs,
        }
    }
}
