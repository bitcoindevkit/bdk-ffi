use crate::psbt::PartiallySignedTransaction;
use crate::wallet::SignOptions;
use bdk::bitcoin::util::bip32::Fingerprint;
use bdk::bitcoincore_rpc::jsonrpc::serde_json::from_str;
use bdk::signer::{SignerId as BdkSignerId};
use std::fmt::{Debug, Display};
use std::str::FromStr;
use std::sync::Arc;
use bdk::signer::SignerOrdering as BdkSignerOrdering;

pub enum SignerId {
    /// Bitcoin HASH160 (RIPEMD160 after SHA256) hash of an ECDSA public key
    PkHash { hash: String },
    /// The fingerprint of a BIP32 extended key
    Fingerprint { fingerprint: String },
    /// Dummy identifier
    Dummy { index: u64 },
}

impl From<SignerId> for BdkSignerId {
    fn from(signer_id: SignerId) -> BdkSignerId {
        match signer_id {
            // TODO: is this the way to do this? I'm not sure about the from_str(as_str()) on the PkHash type
            SignerId::PkHash { hash } => BdkSignerId::PkHash(from_str(hash.as_str()).unwrap()),
            SignerId::Fingerprint { fingerprint } => {
                BdkSignerId::Fingerprint(Fingerprint::from_str(fingerprint.as_str()).unwrap())
            }
            SignerId::Dummy { index } => BdkSignerId::Dummy(index),
        }
    }
}

pub trait InputSigner: Send + Sync + Debug {
    fn id(&self) -> Result<SignerId, CustomError>;

    fn sign_input(
        &self,
        psbt: Arc<PartiallySignedTransaction>,
        input_index: u32,
        sign_options: SignOptions,
    ) -> Result<(), CustomError>;
}



// pub trait TransactionSigner

// impl<T: InputSigner> TransactionSigner for T {
//     fn sign_transaction(
//         &self,
//         psbt: &mut psbt::PartiallySignedTransaction,
//         sign_options: &SignOptions,
//         secp: &SecpCtx,
//     ) -> Result<(), SignerError> {
//         for input_index in 0..psbt.inputs.len() {
//             self.sign_input(psbt, input_index, sign_options, secp)?;
//         }
//
//         Ok(())
//     }
// }

pub struct SignerOrdering {
    pub order: u64
}

impl From<SignerOrdering> for BdkSignerOrdering {
    fn from(signer_ordering: SignerOrdering) -> BdkSignerOrdering {
        BdkSignerOrdering(signer_ordering.order as usize) // QUESTION 1: is this safe? We seem to use it elsewhere
    }
}

#[derive(Debug)]
pub enum CustomError {
    SignerError,
}

impl Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for CustomError {}

// // Need to implement this From<> impl in order to handle unexpected callback errors.  See the
// // Callback Interfaces section of the handbook for more info.
impl From<uniffi::UnexpectedUniFFICallbackError> for CustomError {
    fn from(_: uniffi::UnexpectedUniFFICallbackError) -> Self {
        Self::SignerError
    }
}
