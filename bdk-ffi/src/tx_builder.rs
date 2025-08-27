use crate::bitcoin::{Amount, FeeRate, OutPoint, Psbt, Script, Txid};
use crate::error::CreateTxError;
use crate::types::{CoinSelectionAlgorithm, LockTime, ScriptAmount};
use crate::wallet::Wallet;

use bdk_wallet::bitcoin::absolute::LockTime as BdkLockTime;
use bdk_wallet::bitcoin::amount::Amount as BdkAmount;
use bdk_wallet::bitcoin::script::PushBytesBuf;
use bdk_wallet::bitcoin::Psbt as BdkPsbt;
use bdk_wallet::bitcoin::ScriptBuf as BdkScriptBuf;
use bdk_wallet::bitcoin::{OutPoint as BdkOutPoint, Sequence};
use bdk_wallet::{coin_selection, KeychainKind};

use std::collections::BTreeMap;
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::sync::Arc;

type ChangeSpendPolicy = bdk_wallet::ChangeSpendPolicy;

/// A `TxBuilder` is created by calling `build_tx` on a wallet. After assigning it, you set options on it until finally
/// calling `finish` to consume the builder and generate the transaction.
#[derive(Clone, uniffi::Object)]
pub struct TxBuilder {
    add_global_xpubs: bool,
    recipients: Vec<(BdkScriptBuf, BdkAmount)>,
    utxos: Vec<BdkOutPoint>,
    unspendable: Vec<BdkOutPoint>,
    internal_policy_path: Option<BTreeMap<String, Vec<usize>>>,
    external_policy_path: Option<BTreeMap<String, Vec<usize>>>,
    change_policy: ChangeSpendPolicy,
    manually_selected_only: bool,
    fee_rate: Option<FeeRate>,
    fee_absolute: Option<Arc<Amount>>,
    drain_wallet: bool,
    drain_to: Option<BdkScriptBuf>,
    sequence: Option<u32>,
    data: Vec<u8>,
    current_height: Option<u32>,
    locktime: Option<LockTime>,
    allow_dust: bool,
    version: Option<i32>,
    coin_selection: Option<CoinSelectionAlgorithm>,
}

#[uniffi::export]
impl TxBuilder {
    #[uniffi::constructor]
    pub fn new() -> Self {
        TxBuilder {
            add_global_xpubs: false,
            recipients: Vec::new(),
            utxos: Vec::new(),
            unspendable: Vec::new(),
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
            coin_selection: None,
        }
    }

    /// Fill-in the `PSBT_GLOBAL_XPUB` field with the extended keys contained in both the external and internal
    /// descriptors.
    ///
    /// This is useful for offline signers that take part to a multisig. Some hardware wallets like BitBox and ColdCard
    /// are known to require this.
    pub fn add_global_xpubs(&self) -> Arc<Self> {
        Arc::new(TxBuilder {
            add_global_xpubs: true,
            ..self.clone()
        })
    }

    /// Add a recipient to the internal list of recipients.
    pub fn add_recipient(&self, script: &Script, amount: Arc<Amount>) -> Arc<Self> {
        let mut recipients: Vec<(BdkScriptBuf, BdkAmount)> = self.recipients.clone();
        recipients.append(&mut vec![(script.0.clone(), amount.0)]);

        Arc::new(TxBuilder {
            recipients,
            ..self.clone()
        })
    }

    /// Replace the recipients already added with a new list of recipients.
    pub fn set_recipients(&self, recipients: Vec<ScriptAmount>) -> Arc<Self> {
        let recipients = recipients
            .iter()
            .map(|script_amount| (script_amount.script.0.clone(), script_amount.amount.0)) //;
            .collect();
        Arc::new(TxBuilder {
            recipients,
            ..self.clone()
        })
    }

    /// Add a utxo to the internal list of unspendable utxos.
    ///
    /// It’s important to note that the "must-be-spent" utxos added with `TxBuilder::add_utxo` have priority over this.
    pub fn add_unspendable(&self, unspendable: OutPoint) -> Arc<Self> {
        let mut unspendable_vec: Vec<BdkOutPoint> = self.unspendable.clone();
        unspendable_vec.push(unspendable.into());

        Arc::new(TxBuilder {
            unspendable: unspendable_vec,
            ..self.clone()
        })
    }

    /// Replace the internal list of unspendable utxos with a new list.
    ///
    /// It’s important to note that the "must-be-spent" utxos added with `TxBuilder::add_utxo` have priority over these.
    pub fn unspendable(&self, unspendable: Vec<OutPoint>) -> Arc<Self> {
        let new_unspendable_vec: Vec<BdkOutPoint> =
            unspendable.into_iter().map(BdkOutPoint::from).collect();

        Arc::new(TxBuilder {
            unspendable: new_unspendable_vec,
            ..self.clone()
        })
    }

    /// Add a utxo to the internal list of utxos that must be spent.
    ///
    /// These have priority over the "unspendable" utxos, meaning that if a utxo is present both in the "utxos" and the
    /// "unspendable" list, it will be spent.
    pub fn add_utxo(&self, outpoint: OutPoint) -> Arc<Self> {
        self.add_utxos(vec![outpoint])
    }

    /// Add the list of outpoints to the internal list of UTXOs that must be spent.
    //
    // If an error occurs while adding any of the UTXOs then none of them are added and the error is returned.
    //
    // These have priority over the “unspendable” utxos, meaning that if a utxo is present both in the “utxos” and the “unspendable” list, it will be spent.
    pub fn add_utxos(&self, outpoints: Vec<OutPoint>) -> Arc<Self> {
        let mut utxos: Vec<BdkOutPoint> = self.utxos.clone();
        utxos.extend(outpoints.into_iter().map(BdkOutPoint::from));
        Arc::new(TxBuilder {
            utxos,
            ..self.clone()
        })
    }

    /// The TxBuilder::policy_path is a complex API. See the Rust docs for complete       information: https://docs.rs/bdk_wallet/latest/bdk_wallet/struct.TxBuilder.html#method.policy_path
    pub fn policy_path(
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

    /// Set a specific `ChangeSpendPolicy`. See `TxBuilder::do_not_spend_change` and `TxBuilder::only_spend_change` for
    /// some shortcuts. This method assumes the presence of an internal keychain, otherwise it has no effect.
    pub fn change_policy(&self, change_policy: ChangeSpendPolicy) -> Arc<Self> {
        Arc::new(TxBuilder {
            change_policy,
            ..self.clone()
        })
    }

    /// Do not spend change outputs.
    ///
    /// This effectively adds all the change outputs to the "unspendable" list. See `TxBuilder::unspendable`. This method
    /// assumes the presence of an internal keychain, otherwise it has no effect.
    pub fn do_not_spend_change(&self) -> Arc<Self> {
        Arc::new(TxBuilder {
            change_policy: ChangeSpendPolicy::ChangeForbidden,
            ..self.clone()
        })
    }

    /// Only spend change outputs.
    ///
    /// This effectively adds all the non-change outputs to the "unspendable" list. See `TxBuilder::unspendable`. This
    /// method assumes the presence of an internal keychain, otherwise it has no effect.
    pub fn only_spend_change(&self) -> Arc<Self> {
        Arc::new(TxBuilder {
            change_policy: ChangeSpendPolicy::OnlyChange,
            ..self.clone()
        })
    }

    /// Only spend utxos added by `TxBuilder::add_utxo`.
    ///
    /// The wallet will not add additional utxos to the transaction even if they are needed to make the transaction valid.
    pub fn manually_selected_only(&self) -> Arc<Self> {
        Arc::new(TxBuilder {
            manually_selected_only: true,
            ..self.clone()
        })
    }

    /// Set a custom fee rate.
    ///
    /// This method sets the mining fee paid by the transaction as a rate on its size. This means that the total fee paid
    /// is equal to fee_rate times the size of the transaction. Default is 1 sat/vB in accordance with Bitcoin Core’s
    /// default relay policy.
    ///
    /// Note that this is really a minimum feerate – it’s possible to overshoot it slightly since adding a change output
    /// to drain the remaining excess might not be viable.
    pub fn fee_rate(&self, fee_rate: &FeeRate) -> Arc<Self> {
        Arc::new(TxBuilder {
            fee_rate: Some(fee_rate.clone()),
            ..self.clone()
        })
    }

    /// Set an absolute fee The `fee_absolute` method refers to the absolute transaction fee in `Amount`. If anyone sets
    /// both the `fee_absolute` method and the `fee_rate` method, the `FeePolicy` enum will be set by whichever method was
    /// called last, as the `FeeRate` and `FeeAmount` are mutually exclusive.
    ///
    /// Note that this is really a minimum absolute fee – it’s possible to overshoot it slightly since adding a change output to drain the remaining excess might not be viable.
    pub fn fee_absolute(&self, fee_amount: Arc<Amount>) -> Arc<Self> {
        Arc::new(TxBuilder {
            fee_absolute: Some(fee_amount),
            ..self.clone()
        })
    }

    /// Spend all the available inputs. This respects filters like `TxBuilder::unspendable` and the change policy.
    pub fn drain_wallet(&self) -> Arc<Self> {
        Arc::new(TxBuilder {
            drain_wallet: true,
            ..self.clone()
        })
    }

    /// Sets the address to drain excess coins to.
    ///
    /// Usually, when there are excess coins they are sent to a change address generated by the wallet. This option
    /// replaces the usual change address with an arbitrary script_pubkey of your choosing. Just as with a change output,
    /// if the drain output is not needed (the excess coins are too small) it will not be included in the resulting
    /// transaction. The only difference is that it is valid to use `drain_to` without setting any ordinary recipients
    /// with `add_recipient` (but it is perfectly fine to add recipients as well).
    ///
    /// If you choose not to set any recipients, you should provide the utxos that the transaction should spend via
    /// `add_utxos`. `drain_to` is very useful for draining all the coins in a wallet with `drain_wallet` to a single
    /// address.
    pub fn drain_to(&self, script: &Script) -> Arc<Self> {
        Arc::new(TxBuilder {
            drain_to: Some(script.0.clone()),
            ..self.clone()
        })
    }

    /// Set an exact `nSequence` value.
    ///
    /// This can cause conflicts if the wallet’s descriptors contain an "older" (`OP_CSV`) operator and the given
    /// `nsequence` is lower than the CSV value.
    pub fn set_exact_sequence(&self, nsequence: u32) -> Arc<Self> {
        Arc::new(TxBuilder {
            sequence: Some(nsequence),
            ..self.clone()
        })
    }

    /// Add data as an output using `OP_RETURN`.
    pub fn add_data(&self, data: Vec<u8>) -> Arc<Self> {
        Arc::new(TxBuilder {
            data,
            ..self.clone()
        })
    }

    /// Set the current blockchain height.
    ///
    /// This will be used to:
    ///
    /// 1. Set the `nLockTime` for preventing fee sniping. Note: This will be ignored if you manually specify a
    ///    `nlocktime` using `TxBuilder::nlocktime`.
    ///
    /// 2. Decide whether coinbase outputs are mature or not. If the coinbase outputs are not mature at `current_height`,
    ///    we ignore them in the coin selection. If you want to create a transaction that spends immature coinbase inputs,
    ///    manually add them using `TxBuilder::add_utxos`.
    ///    In both cases, if you don’t provide a current height, we use the last sync height.
    pub fn current_height(&self, height: u32) -> Arc<Self> {
        Arc::new(TxBuilder {
            current_height: Some(height),
            ..self.clone()
        })
    }

    /// Use a specific nLockTime while creating the transaction.
    ///
    /// This can cause conflicts if the wallet’s descriptors contain an "after" (`OP_CLTV`) operator.
    pub fn nlocktime(&self, locktime: LockTime) -> Arc<Self> {
        Arc::new(TxBuilder {
            locktime: Some(locktime),
            ..self.clone()
        })
    }

    /// Set whether or not the dust limit is checked.
    ///
    /// Note: by avoiding a dust limit check you may end up with a transaction that is non-standard.
    pub fn allow_dust(&self, allow_dust: bool) -> Arc<Self> {
        Arc::new(TxBuilder {
            allow_dust,
            ..self.clone()
        })
    }

    /// Build a transaction with a specific version.
    ///
    /// The version should always be greater than 0 and greater than 1 if the wallet’s descriptors contain an "older"
    /// (`OP_CSV`) operator.
    pub fn version(&self, version: i32) -> Arc<Self> {
        Arc::new(TxBuilder {
            version: Some(version),
            ..self.clone()
        })
    }

    /// Choose the coin selection algorithm
    pub fn coin_selection(&self, coin_selection: CoinSelectionAlgorithm) -> Arc<Self> {
        Arc::new(TxBuilder {
            coin_selection: Some(coin_selection),
            ..self.clone()
        })
    }

    /// Finish building the transaction.
    ///
    /// Uses the thread-local random number generator (rng).
    ///
    /// Returns a new `Psbt` per BIP174.
    ///
    /// WARNING: To avoid change address reuse you must persist the changes resulting from one or more calls to this
    /// method before closing the wallet. See `Wallet::reveal_next_address`.
    pub fn finish(&self, wallet: &Arc<Wallet>) -> Result<Arc<Psbt>, CreateTxError> {
        // TODO: I had to change the wallet here to be mutable. Why is that now required with the 1.0 API?
        let mut wallet = wallet.get_wallet();
        let psbt = if let Some(coin_selection) = &self.coin_selection {
            match coin_selection {
                CoinSelectionAlgorithm::BranchAndBoundCoinSelection => {
                    let mut tx_builder = wallet.build_tx().coin_selection(
                        coin_selection::BranchAndBoundCoinSelection::<
                            coin_selection::SingleRandomDraw,
                        >::default(),
                    );
                    self.apply_config(&mut tx_builder)?;
                    tx_builder.finish().map_err(CreateTxError::from)?
                }
                CoinSelectionAlgorithm::SingleRandomDraw => {
                    let mut tx_builder = wallet
                        .build_tx()
                        .coin_selection(coin_selection::SingleRandomDraw);
                    self.apply_config(&mut tx_builder)?;
                    tx_builder.finish().map_err(CreateTxError::from)?
                }
                CoinSelectionAlgorithm::OldestFirstCoinSelection => {
                    let mut tx_builder = wallet
                        .build_tx()
                        .coin_selection(coin_selection::OldestFirstCoinSelection);
                    self.apply_config(&mut tx_builder)?;
                    tx_builder.finish().map_err(CreateTxError::from)?
                }
                CoinSelectionAlgorithm::LargestFirstCoinSelection => {
                    let mut tx_builder = wallet
                        .build_tx()
                        .coin_selection(coin_selection::LargestFirstCoinSelection);
                    self.apply_config(&mut tx_builder)?;
                    tx_builder.finish().map_err(CreateTxError::from)?
                }
            }
        } else {
            let mut tx_builder = wallet.build_tx();
            self.apply_config(&mut tx_builder)?;
            tx_builder.finish().map_err(CreateTxError::from)?
        };

        Ok(Arc::new(psbt.into()))
    }
}

impl TxBuilder {
    fn apply_config<C>(
        &self,
        tx_builder: &mut bdk_wallet::TxBuilder<'_, C>,
    ) -> Result<(), CreateTxError> {
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
            tx_builder.unspendable(self.unspendable.clone());
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
        Ok(())
    }
}

/// A `BumpFeeTxBuilder` is created by calling `build_fee_bump` on a wallet. After assigning it, you set options on it
/// until finally calling `finish` to consume the builder and generate the transaction.
#[derive(Clone, uniffi::Object)]
pub struct BumpFeeTxBuilder {
    txid: Arc<Txid>,
    fee_rate: Arc<FeeRate>,
    sequence: Option<u32>,
    current_height: Option<u32>,
    locktime: Option<LockTime>,
    allow_dust: bool,
    version: Option<i32>,
}

#[uniffi::export]
impl BumpFeeTxBuilder {
    #[uniffi::constructor]
    pub fn new(txid: Arc<Txid>, fee_rate: Arc<FeeRate>) -> Self {
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

    /// Set an exact `nSequence` value.
    ///
    /// This can cause conflicts if the wallet’s descriptors contain an "older" (`OP_CSV`) operator and the given
    /// `nsequence` is lower than the CSV value.
    pub fn set_exact_sequence(&self, nsequence: u32) -> Arc<Self> {
        Arc::new(BumpFeeTxBuilder {
            sequence: Some(nsequence),
            ..self.clone()
        })
    }

    /// Set the current blockchain height.
    ///
    /// This will be used to:
    ///
    /// 1. Set the `nLockTime` for preventing fee sniping. Note: This will be ignored if you manually specify a
    ///    `nlocktime` using `TxBuilder::nlocktime`.
    ///
    /// 2. Decide whether coinbase outputs are mature or not. If the coinbase outputs are not mature at `current_height`,
    ///    we ignore them in the coin selection. If you want to create a transaction that spends immature coinbase inputs,
    ///    manually add them using `TxBuilder::add_utxos`.
    ///    In both cases, if you don’t provide a current height, we use the last sync height.
    pub fn current_height(&self, height: u32) -> Arc<Self> {
        Arc::new(BumpFeeTxBuilder {
            current_height: Some(height),
            ..self.clone()
        })
    }

    /// Use a specific nLockTime while creating the transaction.
    ///
    /// This can cause conflicts if the wallet’s descriptors contain an "after" (`OP_CLTV`) operator.
    pub fn nlocktime(&self, locktime: LockTime) -> Arc<Self> {
        Arc::new(BumpFeeTxBuilder {
            locktime: Some(locktime),
            ..self.clone()
        })
    }

    /// Set whether the dust limit is checked.
    ///
    /// Note: by avoiding a dust limit check you may end up with a transaction that is non-standard.
    pub fn allow_dust(&self, allow_dust: bool) -> Arc<Self> {
        Arc::new(BumpFeeTxBuilder {
            allow_dust,
            ..self.clone()
        })
    }

    /// Build a transaction with a specific version.
    ///
    /// The version should always be greater than 0 and greater than 1 if the wallet’s descriptors contain an "older"
    /// (`OP_CSV`) operator.
    pub fn version(&self, version: i32) -> Arc<Self> {
        Arc::new(BumpFeeTxBuilder {
            version: Some(version),
            ..self.clone()
        })
    }

    /// Finish building the transaction.
    ///
    /// Uses the thread-local random number generator (rng).
    ///
    /// Returns a new `Psbt` per BIP174.
    ///
    /// WARNING: To avoid change address reuse you must persist the changes resulting from one or more calls to this
    /// method before closing the wallet. See `Wallet::reveal_next_address`.
    pub fn finish(&self, wallet: &Arc<Wallet>) -> Result<Arc<Psbt>, CreateTxError> {
        let mut wallet = wallet.get_wallet();
        let mut tx_builder = wallet
            .build_fee_bump(self.txid.0)
            .map_err(CreateTxError::from)?;
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

/// Policy regarding the use of change outputs when creating a transaction.
#[uniffi::remote(Enum)]
pub enum ChangeSpendPolicy {
    /// Use both change and non-change outputs (default).
    #[default]
    ChangeAllowed,
    /// Only use change outputs (see [`bdk_wallet::TxBuilder::only_spend_change`]).
    OnlyChange,
    /// Only use non-change outputs (see [`bdk_wallet::TxBuilder::do_not_spend_change`]).
    ChangeForbidden,
}

#[cfg(test)]
mod tests {
    use crate::bitcoin::{Amount, Script};
    use crate::{
        descriptor::Descriptor, esplora::EsploraClient, store::Persister,
        types::FullScanScriptInspector, wallet::Wallet,
    };
    use bdk_wallet::bitcoin::Network;
    use std::sync::Arc;

    struct FullScanInspector;
    impl FullScanScriptInspector for FullScanInspector {
        fn inspect(&self, _: bdk_wallet::KeychainKind, _: u32, _: Arc<Script>) {}
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
            Arc::new(Persister::new_in_memory().unwrap()),
            25,
        )
        .unwrap();
        let client = EsploraClient::new("https://mutinynet.com/api/".to_string(), None);
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
