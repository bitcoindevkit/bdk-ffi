use crate::bitcoin::{Amount, FeeRate, Psbt, Script};
use crate::error::CreateTxError;
use crate::types::{LockTime, ScriptAmount};
use crate::wallet::Wallet;

use bdk_wallet::bitcoin::absolute::LockTime as BdkLockTime;
use bdk_wallet::bitcoin::amount::Amount as BdkAmount;
use bdk_wallet::bitcoin::script::PushBytesBuf;
use bdk_wallet::bitcoin::Psbt as BdkPsbt;
use bdk_wallet::bitcoin::ScriptBuf as BdkScriptBuf;
use bdk_wallet::bitcoin::{OutPoint, Sequence, Txid};
use bdk_wallet::ChangeSpendPolicy;
use bdk_wallet::KeychainKind;

use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::convert::{TryFrom, TryInto};
use std::str::FromStr;
use std::sync::Arc;

#[derive(Clone)]
pub struct TxBuilder {
    pub(crate) add_global_xpubs: bool,
    pub(crate) recipients: Vec<(BdkScriptBuf, BdkAmount)>,
    pub(crate) utxos: Vec<OutPoint>,
    pub(crate) unspendable: HashSet<OutPoint>,
    pub(crate) internal_policy_path: Option<BTreeMap<String, Vec<usize>>>,
    pub(crate) external_policy_path: Option<BTreeMap<String, Vec<usize>>>,
    pub(crate) change_policy: ChangeSpendPolicy,
    pub(crate) manually_selected_only: bool,
    pub(crate) fee_rate: Option<FeeRate>,
    pub(crate) fee_absolute: Option<Arc<Amount>>,
    pub(crate) drain_wallet: bool,
    pub(crate) drain_to: Option<BdkScriptBuf>,
    pub(crate) sequence: Option<u32>,
    pub(crate) data: Vec<u8>,
    pub(crate) current_height: Option<u32>,
    pub(crate) locktime: Option<LockTime>,
    pub(crate) allow_dust: bool,
    pub(crate) version: Option<i32>,
}

impl TxBuilder {
    pub(crate) fn new() -> Self {
        TxBuilder {
            add_global_xpubs: false,
            recipients: Vec::new(),
            utxos: Vec::new(),
            unspendable: HashSet::new(),
            internal_policy_path: None,
            external_policy_path: None,
            change_policy: ChangeSpendPolicy::ChangeAllowed,
            manually_selected_only: false,
            fee_rate: None,
            fee_absolute: None,
            drain_wallet: false,
            drain_to: None,
            sequence: None,
            data: Vec::new(),
            current_height: None,
            locktime: None,
            allow_dust: false,
            version: None,
        }
    }

    pub(crate) fn add_global_xpubs(&self) -> Arc<Self> {
        Arc::new(TxBuilder {
            add_global_xpubs: true,
            ..self.clone()
        })
    }

    pub(crate) fn add_recipient(&self, script: &Script, amount: Arc<Amount>) -> Arc<Self> {
        let mut recipients: Vec<(BdkScriptBuf, BdkAmount)> = self.recipients.clone();
        recipients.append(&mut vec![(script.0.clone(), amount.0)]);

        Arc::new(TxBuilder {
            recipients,
            ..self.clone()
        })
    }

    pub(crate) fn set_recipients(&self, recipients: Vec<ScriptAmount>) -> Arc<Self> {
        let recipients = recipients
            .iter()
            .map(|script_amount| (script_amount.script.0.clone(), script_amount.amount.0)) //;
            .collect();
        Arc::new(TxBuilder {
            recipients,
            ..self.clone()
        })
    }

    pub(crate) fn add_unspendable(&self, unspendable: OutPoint) -> Arc<Self> {
        let mut unspendable_hash_set = self.unspendable.clone();
        unspendable_hash_set.insert(unspendable);
        Arc::new(TxBuilder {
            unspendable: unspendable_hash_set,
            ..self.clone()
        })
    }

    pub(crate) fn unspendable(&self, unspendable: Vec<OutPoint>) -> Arc<Self> {
        Arc::new(TxBuilder {
            unspendable: unspendable.into_iter().collect(),
            ..self.clone()
        })
    }

    pub(crate) fn add_utxo(&self, outpoint: OutPoint) -> Arc<Self> {
        self.add_utxos(vec![outpoint])
    }

    pub(crate) fn add_utxos(&self, mut outpoints: Vec<OutPoint>) -> Arc<Self> {
        let mut utxos = self.utxos.to_vec();
        utxos.append(&mut outpoints);
        Arc::new(TxBuilder {
            utxos,
            ..self.clone()
        })
    }

    pub(crate) fn policy_path(
        &self,
        policy_path: HashMap<String, Vec<u64>>,
        keychain: KeychainKind,
    ) -> Arc<Self> {
        let mut updated_self = self.clone();
        let to_update = match keychain {
            KeychainKind::Internal => &mut updated_self.internal_policy_path,
            KeychainKind::External => &mut updated_self.external_policy_path,
        };
        *to_update = Some(
            policy_path
                .into_iter()
                .map(|(key, value)| (key, value.into_iter().map(|x| x as usize).collect()))
                .collect::<BTreeMap<String, Vec<usize>>>(),
        );
        Arc::new(updated_self)
    }

    pub(crate) fn change_policy(&self, change_policy: ChangeSpendPolicy) -> Arc<Self> {
        Arc::new(TxBuilder {
            change_policy,
            ..self.clone()
        })
    }

    pub(crate) fn do_not_spend_change(&self) -> Arc<Self> {
        Arc::new(TxBuilder {
            change_policy: ChangeSpendPolicy::ChangeForbidden,
            ..self.clone()
        })
    }

    pub(crate) fn only_spend_change(&self) -> Arc<Self> {
        Arc::new(TxBuilder {
            change_policy: ChangeSpendPolicy::OnlyChange,
            ..self.clone()
        })
    }

    pub(crate) fn manually_selected_only(&self) -> Arc<Self> {
        Arc::new(TxBuilder {
            manually_selected_only: true,
            ..self.clone()
        })
    }

    pub(crate) fn fee_rate(&self, fee_rate: &FeeRate) -> Arc<Self> {
        Arc::new(TxBuilder {
            fee_rate: Some(fee_rate.clone()),
            ..self.clone()
        })
    }

    pub(crate) fn fee_absolute(&self, fee_amount: Arc<Amount>) -> Arc<Self> {
        Arc::new(TxBuilder {
            fee_absolute: Some(fee_amount),
            ..self.clone()
        })
    }

    pub(crate) fn drain_wallet(&self) -> Arc<Self> {
        Arc::new(TxBuilder {
            drain_wallet: true,
            ..self.clone()
        })
    }

    pub(crate) fn drain_to(&self, script: &Script) -> Arc<Self> {
        Arc::new(TxBuilder {
            drain_to: Some(script.0.clone()),
            ..self.clone()
        })
    }

    pub(crate) fn set_exact_sequence(&self, nsequence: u32) -> Arc<Self> {
        Arc::new(TxBuilder {
            sequence: Some(nsequence),
            ..self.clone()
        })
    }

    pub(crate) fn add_data(&self, data: Vec<u8>) -> Arc<Self> {
        Arc::new(TxBuilder {
            data,
            ..self.clone()
        })
    }

    pub(crate) fn current_height(&self, height: u32) -> Arc<Self> {
        Arc::new(TxBuilder {
            current_height: Some(height),
            ..self.clone()
        })
    }

    pub(crate) fn nlocktime(&self, locktime: LockTime) -> Arc<Self> {
        Arc::new(TxBuilder {
            locktime: Some(locktime),
            ..self.clone()
        })
    }

    pub(crate) fn allow_dust(&self, allow_dust: bool) -> Arc<Self> {
        Arc::new(TxBuilder {
            allow_dust,
            ..self.clone()
        })
    }

    pub(crate) fn version(&self, version: i32) -> Arc<Self> {
        Arc::new(TxBuilder {
            version: Some(version),
            ..self.clone()
        })
    }

    pub(crate) fn finish(&self, wallet: &Arc<Wallet>) -> Result<Arc<Psbt>, CreateTxError> {
        // TODO: I had to change the wallet here to be mutable. Why is that now required with the 1.0 API?
        let mut wallet = wallet.get_wallet();
        let mut tx_builder = wallet.build_tx();
        if self.add_global_xpubs {
            tx_builder.add_global_xpubs();
        }
        for (script, amount) in &self.recipients {
            tx_builder.add_recipient(script.clone(), *amount);
        }
        if let Some(policy_path) = &self.external_policy_path {
            tx_builder.policy_path(policy_path.clone(), KeychainKind::External);
        }
        if let Some(policy_path) = &self.internal_policy_path {
            tx_builder.policy_path(policy_path.clone(), KeychainKind::Internal);
        }
        tx_builder.change_policy(self.change_policy);
        if !self.utxos.is_empty() {
            tx_builder
                .add_utxos(&self.utxos)
                .map_err(CreateTxError::from)?;
        }
        if !self.unspendable.is_empty() {
            let bdk_unspendable: Vec<OutPoint> = self.unspendable.clone().into_iter().collect();
            tx_builder.unspendable(bdk_unspendable);
        }
        if self.manually_selected_only {
            tx_builder.manually_selected_only();
        }
        if let Some(fee_rate) = &self.fee_rate {
            tx_builder.fee_rate(fee_rate.0);
        }
        if let Some(fee_amount) = &self.fee_absolute {
            tx_builder.fee_absolute(fee_amount.0);
        }
        if self.drain_wallet {
            tx_builder.drain_wallet();
        }
        if let Some(script) = &self.drain_to {
            tx_builder.drain_to(script.clone());
        }
        if let Some(sequence) = self.sequence {
            tx_builder.set_exact_sequence(Sequence(sequence));
        }
        if !&self.data.is_empty() {
            let push_bytes = PushBytesBuf::try_from(self.data.clone())?;
            tx_builder.add_data(&push_bytes);
        }
        if let Some(height) = self.current_height {
            tx_builder.current_height(height);
        }
        if let Some(locktime) = &self.locktime {
            let bdk_locktime: BdkLockTime = locktime.try_into()?;
            tx_builder.nlocktime(bdk_locktime);
        }
        if self.allow_dust {
            tx_builder.allow_dust(self.allow_dust);
        }
        if let Some(version) = self.version {
            tx_builder.version(version);
        }

        let psbt = tx_builder.finish().map_err(CreateTxError::from)?;

        Ok(Arc::new(psbt.into()))
    }
}

#[derive(Clone)]
pub(crate) struct BumpFeeTxBuilder {
    pub(crate) txid: String,
    pub(crate) fee_rate: Arc<FeeRate>,
    pub(crate) sequence: Option<u32>,
    pub(crate) current_height: Option<u32>,
    pub(crate) locktime: Option<LockTime>,
    pub(crate) allow_dust: bool,
    pub(crate) version: Option<i32>,
}

impl BumpFeeTxBuilder {
    pub(crate) fn new(txid: String, fee_rate: Arc<FeeRate>) -> Self {
        BumpFeeTxBuilder {
            txid,
            fee_rate,
            sequence: None,
            current_height: None,
            locktime: None,
            allow_dust: false,
            version: None,
        }
    }

    pub(crate) fn set_exact_sequence(&self, nsequence: u32) -> Arc<Self> {
        Arc::new(BumpFeeTxBuilder {
            sequence: Some(nsequence),
            ..self.clone()
        })
    }

    pub(crate) fn current_height(&self, height: u32) -> Arc<Self> {
        Arc::new(BumpFeeTxBuilder {
            current_height: Some(height),
            ..self.clone()
        })
    }

    pub(crate) fn nlocktime(&self, locktime: LockTime) -> Arc<Self> {
        Arc::new(BumpFeeTxBuilder {
            locktime: Some(locktime),
            ..self.clone()
        })
    }

    pub(crate) fn allow_dust(&self, allow_dust: bool) -> Arc<Self> {
        Arc::new(BumpFeeTxBuilder {
            allow_dust,
            ..self.clone()
        })
    }

    pub(crate) fn version(&self, version: i32) -> Arc<Self> {
        Arc::new(BumpFeeTxBuilder {
            version: Some(version),
            ..self.clone()
        })
    }

    pub(crate) fn finish(&self, wallet: &Arc<Wallet>) -> Result<Arc<Psbt>, CreateTxError> {
        let txid = Txid::from_str(self.txid.as_str()).map_err(|_| CreateTxError::UnknownUtxo {
            outpoint: self.txid.clone(),
        })?;
        let mut wallet = wallet.get_wallet();
        let mut tx_builder = wallet.build_fee_bump(txid).map_err(CreateTxError::from)?;
        tx_builder.fee_rate(self.fee_rate.0);
        if let Some(sequence) = self.sequence {
            tx_builder.set_exact_sequence(Sequence(sequence));
        }
        if let Some(height) = self.current_height {
            tx_builder.current_height(height);
        }
        if let Some(locktime) = &self.locktime {
            let bdk_locktime: BdkLockTime = locktime.try_into()?;
            tx_builder.nlocktime(bdk_locktime);
        }
        if self.allow_dust {
            tx_builder.allow_dust(self.allow_dust);
        }
        if let Some(version) = self.version {
            tx_builder.version(version);
        }

        let psbt: BdkPsbt = tx_builder.finish()?;

        Ok(Arc::new(psbt.into()))
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use bitcoin_ffi::Network;

    use crate::bitcoin::Amount;
    use crate::{
        descriptor::Descriptor, esplora::EsploraClient, store::Connection,
        types::FullScanScriptInspector, wallet::Wallet,
    };

    struct FullScanInspector;
    impl FullScanScriptInspector for FullScanInspector {
        fn inspect(&self, _: bdk_wallet::KeychainKind, _: u32, _: Arc<bitcoin_ffi::Script>) {}
    }

    #[test]
    fn test_policy_path() {
        let wallet = create_and_sync_wallet();
        let address = wallet
            .next_unused_address(bdk_wallet::KeychainKind::External)
            .address;
        println!("Wallet address: {:?}", address);

        let ext_policy = wallet.policies(bdk_wallet::KeychainKind::External);
        let int_policy = wallet.policies(bdk_wallet::KeychainKind::Internal);

        if let (Ok(Some(ext_policy)), Ok(Some(int_policy))) = (ext_policy, int_policy) {
            let ext_path = vec![(ext_policy.id().clone(), vec![0, 1])]
                .into_iter()
                .collect();
            println!("External Policy path : {:?}\n", ext_path);
            let int_path = vec![(int_policy.id().clone(), vec![0, 1])]
                .into_iter()
                .collect();
            println!("Internal Policy Path: {:?}\n", int_path);

            match crate::tx_builder::TxBuilder::new()
                .add_recipient(
                    &(*address.script_pubkey()).to_owned(),
                    Arc::new(Amount::from_sat(1000)),
                )
                .do_not_spend_change()
                .policy_path(int_path, bdk_wallet::KeychainKind::Internal)
                .policy_path(ext_path, bdk_wallet::KeychainKind::External)
                .finish(&Arc::new(wallet))
            {
                Ok(tx) => println!("Transaction serialized: {}\n", tx.serialize()),
                Err(e) => eprintln!("Error: {:?}", e),
            }
        } else {
            println!("Failed to retrieve valid policies for keychains.");
        }
    }

    fn create_and_sync_wallet() -> Wallet {
        let external_descriptor = format!(
            "wsh(thresh(2,pk({}/0/*),sj:and_v(v:pk({}/0/*),n:older(6)),snj:and_v(v:pk({}/0/*),after(630000))))",
            "tpubD6NzVbkrYhZ4XJBfEJ6gt9DiVdfWJijsQTCE3jtXByW3Tk6AVGQ3vL1NNxg3SjB7QkJAuutACCQjrXD8zdZSM1ZmBENszCqy49ECEHmD6rf",
            "tpubD6NzVbkrYhZ4YfAr3jCBRk4SpqB9L1Hh442y83njwfMaker7EqZd7fHMqyTWrfRYJ1e5t2ue6BYjW5i5yQnmwqbzY1a3kfqNxog1AFcD1aE",
            "tprv8ZgxMBicQKsPeitVUz3s6cfyCECovNP7t82FaKPa4UKqV1kssWcXgLkMDjzDbgG9GWoza4pL7z727QitfzkiwX99E1Has3T3a1MKHvYWmQZ"
        );
        let internal_descriptor = format!(
            "wsh(thresh(2,pk({}/1/*),sj:and_v(v:pk({}/1/*),n:older(6)),snj:and_v(v:pk({}/1/*),after(630000))))",
            "tpubD6NzVbkrYhZ4XJBfEJ6gt9DiVdfWJijsQTCE3jtXByW3Tk6AVGQ3vL1NNxg3SjB7QkJAuutACCQjrXD8zdZSM1ZmBENszCqy49ECEHmD6rf",
            "tpubD6NzVbkrYhZ4YfAr3jCBRk4SpqB9L1Hh442y83njwfMaker7EqZd7fHMqyTWrfRYJ1e5t2ue6BYjW5i5yQnmwqbzY1a3kfqNxog1AFcD1aE",
            "tprv8ZgxMBicQKsPeitVUz3s6cfyCECovNP7t82FaKPa4UKqV1kssWcXgLkMDjzDbgG9GWoza4pL7z727QitfzkiwX99E1Has3T3a1MKHvYWmQZ"
        );
        let wallet = Wallet::new(
            Arc::new(Descriptor::new(external_descriptor, Network::Signet).unwrap()),
            Arc::new(Descriptor::new(internal_descriptor, Network::Signet).unwrap()),
            Network::Signet,
            Arc::new(Connection::new_in_memory().unwrap()),
        )
        .unwrap();
        let client = EsploraClient::new("https://mutinynet.com/api/".to_string());
        let full_scan_builder = wallet.start_full_scan();
        let full_scan_request = full_scan_builder
            .inspect_spks_for_all_keychains(Arc::new(FullScanInspector))
            .unwrap()
            .build()
            .unwrap();
        let update = client.full_scan(full_scan_request, 10, 10).unwrap();
        wallet.apply_update(update).unwrap();
        println!("Wallet balance: {:?}", wallet.balance().total.to_sat());
        wallet
    }
}
