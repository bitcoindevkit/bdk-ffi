use crate::bitcoin::Psbt;
use crate::error::CreateTxError;
use crate::types::ScriptAmount;
use crate::wallet::Wallet;

use bitcoin_ffi::{Amount, FeeRate, Script};

use bdk_wallet::bitcoin::amount::Amount as BdkAmount;
use bdk_wallet::bitcoin::Psbt as BdkPsbt;
use bdk_wallet::bitcoin::ScriptBuf as BdkScriptBuf;
use bdk_wallet::bitcoin::{OutPoint, Sequence, Txid};
use bdk_wallet::ChangeSpendPolicy;

use std::collections::HashSet;
use std::str::FromStr;
use std::sync::Arc;

#[derive(Clone)]
pub struct TxBuilder {
    pub(crate) add_global_xpubs: bool,
    pub(crate) recipients: Vec<(BdkScriptBuf, BdkAmount)>,
    pub(crate) utxos: Vec<OutPoint>,
    pub(crate) unspendable: HashSet<OutPoint>,
    pub(crate) change_policy: ChangeSpendPolicy,
    pub(crate) manually_selected_only: bool,
    pub(crate) fee_rate: Option<FeeRate>,
    pub(crate) fee_absolute: Option<Arc<Amount>>,
    pub(crate) drain_wallet: bool,
    pub(crate) drain_to: Option<BdkScriptBuf>,
    pub(crate) sequence: Option<u32>,
}

impl TxBuilder {
    pub(crate) fn new() -> Self {
        TxBuilder {
            add_global_xpubs: false,
            recipients: Vec::new(),
            utxos: Vec::new(),
            unspendable: HashSet::new(),
            change_policy: ChangeSpendPolicy::ChangeAllowed,
            manually_selected_only: false,
            fee_rate: None,
            fee_absolute: None,
            drain_wallet: false,
            drain_to: None,
            sequence: None,
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

        let psbt = tx_builder.finish().map_err(CreateTxError::from)?;

        Ok(Arc::new(psbt.into()))
    }
}

#[derive(Clone)]
pub(crate) struct BumpFeeTxBuilder {
    pub(crate) txid: String,
    pub(crate) fee_rate: Arc<FeeRate>,
    pub(crate) sequence: Option<u32>,
}

impl BumpFeeTxBuilder {
    pub(crate) fn new(txid: String, fee_rate: Arc<FeeRate>) -> Self {
        Self {
            txid,
            fee_rate,
            sequence: None,
        }
    }

    pub(crate) fn set_exact_sequence(&self, nsequence: u32) -> Arc<Self> {
        Arc::new(BumpFeeTxBuilder {
            sequence: Some(nsequence),
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

        let psbt: BdkPsbt = tx_builder.finish()?;

        Ok(Arc::new(psbt.into()))
    }
}
