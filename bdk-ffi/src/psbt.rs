use bdk::bitcoin::hashes::hex::ToHex;
use bdk::bitcoin::util::psbt::Input as BdkInput;
use bdk::bitcoin::util::psbt::PartiallySignedTransaction as BdkPartiallySignedTransaction;
use bdk::bitcoin::util::psbt::PsbtSighashType as BdkPsbtSighashType;
use bdk::bitcoin::{EcdsaSighashType, SchnorrSighashType};
use bdk::bitcoincore_rpc::jsonrpc::serde_json;
use bdk::psbt::PsbtUtils;
use std::ops::Deref;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

use crate::{BdkError, FeeRate, Transaction};

#[derive(Debug)]
pub(crate) struct PartiallySignedTransaction {
    pub(crate) internal: Mutex<BdkPartiallySignedTransaction>,
}

impl PartiallySignedTransaction {
    pub(crate) fn new(psbt_base64: String) -> Result<Self, BdkError> {
        let psbt: BdkPartiallySignedTransaction =
            BdkPartiallySignedTransaction::from_str(&psbt_base64)?;
        Ok(PartiallySignedTransaction {
            internal: Mutex::new(psbt),
        })
    }

    pub(crate) fn serialize(&self) -> String {
        let psbt = self.internal.lock().unwrap().clone();
        psbt.to_string()
    }

    pub(crate) fn txid(&self) -> String {
        let tx = self.internal.lock().unwrap().clone().extract_tx();
        let txid = tx.txid();
        txid.to_hex()
    }

    /// Return the transaction.
    pub(crate) fn extract_tx(&self) -> Arc<Transaction> {
        let tx = self.internal.lock().unwrap().clone().extract_tx();
        Arc::new(tx.into())
    }

    /// Combines this PartiallySignedTransaction with other PSBT as described by BIP 174.
    ///
    /// In accordance with BIP 174 this function is commutative i.e., `A.combine(B) == B.combine(A)`
    pub(crate) fn combine(
        &self,
        other: Arc<PartiallySignedTransaction>,
    ) -> Result<Arc<PartiallySignedTransaction>, BdkError> {
        let other_psbt = other.internal.lock().unwrap().clone();
        let mut original_psbt = self.internal.lock().unwrap().clone();

        original_psbt.combine(other_psbt)?;
        Ok(Arc::new(PartiallySignedTransaction {
            internal: Mutex::new(original_psbt),
        }))
    }

    /// The total transaction fee amount, sum of input amounts minus sum of output amounts, in Sats.
    /// If the PSBT is missing a TxOut for an input returns None.
    pub(crate) fn fee_amount(&self) -> Option<u64> {
        self.internal.lock().unwrap().fee_amount()
    }

    /// The transaction's fee rate. This value will only be accurate if calculated AFTER the
    /// `PartiallySignedTransaction` is finalized and all witness/signature data is added to the
    /// transaction.
    /// If the PSBT is missing a TxOut for an input returns None.
    pub(crate) fn fee_rate(&self) -> Option<Arc<FeeRate>> {
        self.internal.lock().unwrap().fee_rate().map(Arc::new)
    }

    /// Serialize the PSBT data structure as a String of JSON.
    pub(crate) fn json_serialize(&self) -> String {
        let psbt = self.internal.lock().unwrap();
        serde_json::to_string(psbt.deref()).unwrap()
    }
}

/// A key-value map for an input of the corresponding index in the unsigned
/// transaction.
#[derive(Debug)]
pub(crate) struct Input {
    inner: BdkInput,
}

impl Input {
    /// Create a new PSBT Input from a JSON String.
    pub(crate) fn from_json(input_json: String) -> Result<Self, BdkError> {
        let input = serde_json::from_str(input_json.as_str())?;
        Ok(Self { inner: input })
    }

    /// Serialize the PSBT Input data structure as a JSON String.
    pub(crate) fn json_serialize(&self) -> String {
        let input = &self.inner;
        serde_json::to_string(input).unwrap()
    }
}

impl From<BdkInput> for Input {
    fn from(input: BdkInput) -> Self {
        Input { inner: input }
    }
}

/// A Signature hash type for the corresponding input. As of taproot upgrade, the signature hash
/// type can be either [`EcdsaSighashType`] or [`SchnorrSighashType`] but it is not possible to know
/// directly which signature hash type the user is dealing with. Therefore, the user is responsible
/// for converting to/from [`PsbtSighashType`] from/to the desired signature hash type they need.
#[derive(Debug)]
pub(crate) struct PsbtSighashType {
    inner: BdkPsbtSighashType,
}

impl PsbtSighashType {
    pub(crate) fn from_ecdsa(ecdsa_hash_ty: EcdsaSighashType) -> Self {
        PsbtSighashType {
            inner: BdkPsbtSighashType::from(ecdsa_hash_ty),
        }
    }

    pub(crate) fn from_schnorr(schnorr_hash_ty: SchnorrSighashType) -> Self {
        PsbtSighashType {
            inner: BdkPsbtSighashType::from(schnorr_hash_ty),
        }
    }
}

impl From<&PsbtSighashType> for BdkPsbtSighashType {
    fn from(psbt_hash_ty: &PsbtSighashType) -> Self {
        psbt_hash_ty.inner
    }
}

// The goal of these tests to to ensure `bdk-ffi` intermediate code correctly calls `bdk` APIs.
// These tests should not be used to verify `bdk` behavior that is already tested in the `bdk`
// crate.
#[cfg(test)]
mod test {
    use crate::wallet::{TxBuilder, Wallet};
    use bdk::wallet::get_funded_wallet;
    use std::sync::Mutex;

    #[test]
    fn test_psbt_fee() {
        let test_wpkh = "wpkh(cVpPVruEDdmutPzisEsYvtST1usBR3ntr8pXSyt6D2YYqXRyPcFW)";
        let (funded_wallet, _, _) = get_funded_wallet(test_wpkh);
        let test_wallet = Wallet {
            wallet_mutex: Mutex::new(funded_wallet),
        };
        let drain_to_address = "tb1ql7w62elx9ucw4pj5lgw4l028hmuw80sndtntxt".to_string();
        let drain_to_script = crate::Address::new(drain_to_address)
            .unwrap()
            .script_pubkey();

        let tx_builder = TxBuilder::new()
            .fee_rate(2.0)
            .drain_wallet()
            .drain_to(drain_to_script.clone());
        //dbg!(&tx_builder);
        assert!(tx_builder.drain_wallet);
        assert_eq!(tx_builder.drain_to, Some(drain_to_script.script.clone()));

        let tx_builder_result = tx_builder.finish(&test_wallet).unwrap();

        assert!(tx_builder_result.psbt.fee_rate().is_some());
        assert_eq!(
            tx_builder_result.psbt.fee_rate().unwrap().as_sat_per_vb(),
            2.682927
        );

        assert!(tx_builder_result.psbt.fee_amount().is_some());
        assert_eq!(tx_builder_result.psbt.fee_amount().unwrap(), 220);
    }
}
