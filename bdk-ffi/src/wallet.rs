use crate::bitcoin::{FeeRate, OutPoint, Psbt, Script, Transaction};
use crate::descriptor::Descriptor;
use crate::error::{
    CalculateFeeError, CannotConnectError, CreateTxError, PersistenceError, SignerError,
    TxidParseError, WalletCreationError,
};
use crate::types::{
    AddressInfo, Balance, CanonicalTx, FullScanRequest, LocalOutput, ScriptAmount, SyncRequest,
};

use bdk::bitcoin::blockdata::script::ScriptBuf as BdkScriptBuf;
use bdk::bitcoin::Network;
use bdk::bitcoin::Psbt as BdkPsbt;
use bdk::bitcoin::{OutPoint as BdkOutPoint, Sequence, Txid};
use bdk::wallet::tx_builder::ChangeSpendPolicy;
use bdk::wallet::{ChangeSet, Update as BdkUpdate};
use bdk::Wallet as BdkWallet;
use bdk::{KeychainKind, SignOptions};
use bdk_file_store::Store;

use std::collections::HashSet;
use std::str::FromStr;
use std::sync::{Arc, Mutex, MutexGuard};

const MAGIC_BYTES: &[u8] = "bdkffi".as_bytes();

pub struct Wallet {
    inner_mutex: Mutex<BdkWallet>,
}

impl Wallet {
    pub fn new(
        descriptor: Arc<Descriptor>,
        change_descriptor: Option<Arc<Descriptor>>,
        persistence_backend_path: String,
        network: Network,
    ) -> Result<Self, WalletCreationError> {
        let descriptor = descriptor.as_string_private();
        let change_descriptor = change_descriptor.map(|d| d.as_string_private());
        let db = Store::<ChangeSet>::open_or_create_new(MAGIC_BYTES, persistence_backend_path)?;

        let wallet: BdkWallet =
            BdkWallet::new_or_load(&descriptor, change_descriptor.as_ref(), db, network)?;

        Ok(Wallet {
            inner_mutex: Mutex::new(wallet),
        })
    }

    pub(crate) fn get_wallet(&self) -> MutexGuard<BdkWallet> {
        self.inner_mutex.lock().expect("wallet")
    }

    pub fn reveal_next_address(
        &self,
        keychain_kind: KeychainKind,
    ) -> Result<AddressInfo, PersistenceError> {
        self.get_wallet()
            .reveal_next_address(keychain_kind)
            .map(|address_info| address_info.into())
            .map_err(|e| PersistenceError::Write {
                error_message: e.to_string(),
            })
    }

    pub fn apply_update(&self, update: Arc<Update>) -> Result<(), CannotConnectError> {
        self.get_wallet()
            .apply_update(update.0.clone())
            .map_err(CannotConnectError::from)
    }

    // TODO: This is the fallible version of get_internal_address; should I rename it to get_internal_address?
    //       It's a slight change of the API, the other option is to rename the get_address to try_get_address
    // pub fn try_get_internal_address(
    //     &self,
    //     address_index: AddressIndex,
    // ) -> Result<AddressInfo, PersistenceError> {
    //     let address_info = self
    //         .get_wallet()
    //         .try_get_internal_address(address_index.into())?
    //         .into();
    //     Ok(address_info)
    // }

    pub fn network(&self) -> Network {
        self.get_wallet().network()
    }

    pub fn get_balance(&self) -> Balance {
        let bdk_balance: bdk::wallet::Balance = self.get_wallet().get_balance();
        Balance::from(bdk_balance)
    }

    pub fn is_mine(&self, script: &Script) -> bool {
        self.get_wallet().is_mine(&script.0)
    }

    pub(crate) fn sign(
        &self,
        psbt: Arc<Psbt>,
        // sign_options: Option<SignOptions>,
    ) -> Result<bool, SignerError> {
        let mut psbt = psbt.0.lock().unwrap();
        self.get_wallet()
            .sign(&mut psbt, SignOptions::default())
            .map_err(SignerError::from)
    }

    pub fn sent_and_received(&self, tx: &Transaction) -> SentAndReceivedValues {
        let (sent, received): (u64, u64) = self.get_wallet().sent_and_received(&tx.into());
        SentAndReceivedValues { sent, received }
    }

    pub fn transactions(&self) -> Vec<CanonicalTx> {
        self.get_wallet()
            .transactions()
            .map(|tx| tx.into())
            .collect()
    }

    pub fn get_tx(&self, txid: String) -> Result<Option<CanonicalTx>, TxidParseError> {
        let txid =
            Txid::from_str(txid.as_str()).map_err(|_| TxidParseError::InvalidTxid { txid })?;
        Ok(self.get_wallet().get_tx(txid).map(|tx| tx.into()))
    }

    pub fn calculate_fee(&self, tx: &Transaction) -> Result<u64, CalculateFeeError> {
        self.get_wallet()
            .calculate_fee(&tx.into())
            .map_err(|e| e.into())
    }

    pub fn calculate_fee_rate(&self, tx: &Transaction) -> Result<Arc<FeeRate>, CalculateFeeError> {
        self.get_wallet()
            .calculate_fee_rate(&tx.into())
            .map(|bdk_fee_rate| Arc::new(FeeRate(bdk_fee_rate)))
            .map_err(|e| e.into())
    }

    pub fn list_unspent(&self) -> Vec<LocalOutput> {
        self.get_wallet().list_unspent().map(|o| o.into()).collect()
    }

    pub fn list_output(&self) -> Vec<LocalOutput> {
        self.get_wallet().list_output().map(|o| o.into()).collect()
    }

    pub fn start_full_scan(&self) -> Arc<FullScanRequest> {
        let request = self.get_wallet().start_full_scan();
        Arc::new(FullScanRequest(Mutex::new(Some(request))))
    }

    pub fn start_sync_with_revealed_spks(&self) -> Arc<SyncRequest> {
        let request = self.get_wallet().start_sync_with_revealed_spks();
        Arc::new(SyncRequest(Mutex::new(Some(request))))
    }
}

pub struct SentAndReceivedValues {
    pub sent: u64,
    pub received: u64,
}

pub struct Update(pub(crate) BdkUpdate);

#[derive(Clone, Debug)]
pub struct TxBuilder {
    pub(crate) recipients: Vec<(BdkScriptBuf, u64)>,
    pub(crate) utxos: Vec<OutPoint>,
    pub(crate) unspendable: HashSet<OutPoint>,
    pub(crate) change_policy: ChangeSpendPolicy,
    pub(crate) manually_selected_only: bool,
    pub(crate) fee_rate: Option<FeeRate>,
    pub(crate) fee_absolute: Option<u64>,
    pub(crate) drain_wallet: bool,
    pub(crate) drain_to: Option<BdkScriptBuf>,
    pub(crate) rbf: Option<RbfValue>,
    // pub(crate) data: Vec<u8>,
}

impl TxBuilder {
    pub(crate) fn new() -> Self {
        TxBuilder {
            recipients: Vec::new(),
            utxos: Vec::new(),
            unspendable: HashSet::new(),
            change_policy: ChangeSpendPolicy::ChangeAllowed,
            manually_selected_only: false,
            fee_rate: None,
            fee_absolute: None,
            drain_wallet: false,
            drain_to: None,
            rbf: None,
            // data: Vec::new(),
        }
    }

    pub(crate) fn add_recipient(&self, script: &Script, amount: u64) -> Arc<Self> {
        let mut recipients: Vec<(BdkScriptBuf, u64)> = self.recipients.clone();
        recipients.append(&mut vec![(script.0.clone(), amount)]);

        Arc::new(TxBuilder {
            recipients,
            ..self.clone()
        })
    }

    pub(crate) fn set_recipients(&self, recipients: Vec<ScriptAmount>) -> Arc<Self> {
        let recipients = recipients
            .iter()
            .map(|script_amount| (script_amount.script.0.clone(), script_amount.amount))
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

    pub(crate) fn fee_absolute(&self, fee_amount: u64) -> Arc<Self> {
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

    pub(crate) fn enable_rbf(&self) -> Arc<Self> {
        Arc::new(TxBuilder {
            rbf: Some(RbfValue::Default),
            ..self.clone()
        })
    }

    pub(crate) fn enable_rbf_with_sequence(&self, nsequence: u32) -> Arc<Self> {
        Arc::new(TxBuilder {
            rbf: Some(RbfValue::Value(nsequence)),
            ..self.clone()
        })
    }

    pub(crate) fn finish(&self, wallet: &Arc<Wallet>) -> Result<Arc<Psbt>, CreateTxError> {
        // TODO: I had to change the wallet here to be mutable. Why is that now required with the 1.0 API?
        let mut wallet = wallet.get_wallet();
        let mut tx_builder = wallet.build_tx();
        for (script, amount) in &self.recipients {
            tx_builder.add_recipient(script.clone(), *amount);
        }
        tx_builder.change_policy(self.change_policy);
        if !self.utxos.is_empty() {
            let bdk_utxos: Vec<BdkOutPoint> = self.utxos.iter().map(BdkOutPoint::from).collect();
            tx_builder
                .add_utxos(&bdk_utxos)
                .map_err(CreateTxError::from)?;
        }
        if !self.unspendable.is_empty() {
            let bdk_unspendable: Vec<BdkOutPoint> =
                self.unspendable.iter().map(BdkOutPoint::from).collect();
            tx_builder.unspendable(bdk_unspendable);
        }
        if self.manually_selected_only {
            tx_builder.manually_selected_only();
        }
        if let Some(fee_rate) = &self.fee_rate {
            tx_builder.fee_rate(fee_rate.0);
        }
        if let Some(fee_amount) = self.fee_absolute {
            tx_builder.fee_absolute(fee_amount);
        }
        if self.drain_wallet {
            tx_builder.drain_wallet();
        }
        if let Some(script) = &self.drain_to {
            tx_builder.drain_to(script.clone());
        }
        if let Some(rbf) = &self.rbf {
            match *rbf {
                RbfValue::Default => {
                    tx_builder.enable_rbf();
                }
                RbfValue::Value(nsequence) => {
                    tx_builder.enable_rbf_with_sequence(Sequence(nsequence));
                }
            }
        }

        let psbt = tx_builder.finish().map_err(CreateTxError::from)?;

        Ok(Arc::new(psbt.into()))
    }
}

#[derive(Clone)]
pub(crate) struct BumpFeeTxBuilder {
    pub(crate) txid: String,
    pub(crate) fee_rate: Arc<FeeRate>,
    pub(crate) rbf: Option<RbfValue>,
}

impl BumpFeeTxBuilder {
    pub(crate) fn new(txid: String, fee_rate: Arc<FeeRate>) -> Self {
        Self {
            txid,
            fee_rate,
            rbf: None,
        }
    }

    pub(crate) fn enable_rbf(&self) -> Arc<Self> {
        Arc::new(Self {
            rbf: Some(RbfValue::Default),
            ..self.clone()
        })
    }

    pub(crate) fn enable_rbf_with_sequence(&self, nsequence: u32) -> Arc<Self> {
        Arc::new(Self {
            rbf: Some(RbfValue::Value(nsequence)),
            ..self.clone()
        })
    }

    pub(crate) fn finish(&self, wallet: &Wallet) -> Result<Arc<Psbt>, CreateTxError> {
        let txid = Txid::from_str(self.txid.as_str()).map_err(|_| CreateTxError::UnknownUtxo {
            outpoint: self.txid.clone(),
        })?;
        let mut wallet = wallet.get_wallet();
        let mut tx_builder = wallet.build_fee_bump(txid).map_err(CreateTxError::from)?;
        tx_builder.fee_rate(self.fee_rate.0);
        if let Some(rbf) = &self.rbf {
            match *rbf {
                RbfValue::Default => {
                    tx_builder.enable_rbf();
                }
                RbfValue::Value(nsequence) => {
                    tx_builder.enable_rbf_with_sequence(Sequence(nsequence));
                }
            }
        }
        let psbt: BdkPsbt = tx_builder.finish()?;

        Ok(Arc::new(psbt.into()))
    }
}

#[derive(Clone, Debug)]
pub enum RbfValue {
    Default,
    Value(u32),
}

// #[cfg(test)]
// mod test {
//     use crate::database::DatabaseConfig;
//     use crate::descriptor::Descriptor;
//     use crate::keys::{DescriptorSecretKey, Mnemonic};
//     use crate::wallet::{AddressIndex, TxBuilder, Wallet};
//     use crate::Script;
//     use assert_matches::assert_matches;
//     use bdk::bitcoin::{Address, Network};
//     // use bdk::wallet::get_funded_wallet;
//     use bdk::KeychainKind;
//     use std::str::FromStr;
//     use std::sync::{Arc, Mutex};
//
//     // #[test]
//     // fn test_drain_wallet() {
//     //     let test_wpkh = "wpkh(cVpPVruEDdmutPzisEsYvtST1usBR3ntr8pXSyt6D2YYqXRyPcFW)";
//     //     let (funded_wallet, _, _) = get_funded_wallet(test_wpkh);
//     //     let test_wallet = Wallet {
//     //         inner_mutex: Mutex::new(funded_wallet),
//     //     };
//     //     let drain_to_address = "tb1ql7w62elx9ucw4pj5lgw4l028hmuw80sndtntxt".to_string();
//     //     let drain_to_script = crate::Address::new(drain_to_address)
//     //         .unwrap()
//     //         .script_pubkey();
//     //     let tx_builder = TxBuilder::new()
//     //         .drain_wallet()
//     //         .drain_to(drain_to_script.clone());
//     //     assert!(tx_builder.drain_wallet);
//     //     assert_eq!(tx_builder.drain_to, Some(drain_to_script.inner.clone()));
//     //
//     //     let tx_builder_result = tx_builder.finish(&test_wallet).unwrap();
//     //     let psbt = tx_builder_result.psbt.inner.lock().unwrap().clone();
//     //     let tx_details = tx_builder_result.transaction_details;
//     //
//     //     // confirm one input with 50,000 sats
//     //     assert_eq!(psbt.inputs.len(), 1);
//     //     let input_value = psbt
//     //         .inputs
//     //         .get(0)
//     //         .cloned()
//     //         .unwrap()
//     //         .non_witness_utxo
//     //         .unwrap()
//     //         .output
//     //         .get(0)
//     //         .unwrap()
//     //         .value;
//     //     assert_eq!(input_value, 50_000_u64);
//     //
//     //     // confirm one output to correct address with all sats - fee
//     //     assert_eq!(psbt.outputs.len(), 1);
//     //     let output_address = Address::from_script(
//     //         &psbt
//     //             .unsigned_tx
//     //             .output
//     //             .get(0)
//     //             .cloned()
//     //             .unwrap()
//     //             .script_pubkey,
//     //         Network::Testnet,
//     //     )
//     //     .unwrap();
//     //     assert_eq!(
//     //         output_address,
//     //         Address::from_str("tb1ql7w62elx9ucw4pj5lgw4l028hmuw80sndtntxt").unwrap()
//     //     );
//     //     let output_value = psbt.unsigned_tx.output.get(0).cloned().unwrap().value;
//     //     assert_eq!(output_value, 49_890_u64); // input - fee
//     //
//     //     assert_eq!(
//     //         tx_details.txid,
//     //         "312f1733badab22dc26b8dcbc83ba5629fb7b493af802e8abe07d865e49629c5"
//     //     );
//     //     assert_eq!(tx_details.received, 0);
//     //     assert_eq!(tx_details.sent, 50000);
//     //     assert!(tx_details.fee.is_some());
//     //     assert_eq!(tx_details.fee.unwrap(), 110);
//     //     assert!(tx_details.confirmation_time.is_none());
//     // }
//
//     #[test]
//     fn test_peek_reset_address() {
//         let test_wpkh = "wpkh(tprv8hwWMmPE4BVNxGdVt3HhEERZhondQvodUY7Ajyseyhudr4WabJqWKWLr4Wi2r26CDaNCQhhxEftEaNzz7dPGhWuKFU4VULesmhEfZYyBXdE/0/*)";
//         let descriptor = Descriptor::new(test_wpkh.to_string(), Network::Regtest).unwrap();
//         let change_descriptor = Descriptor::new(
//             test_wpkh.to_string().replace("/0/*", "/1/*"),
//             Network::Regtest,
//         )
//         .unwrap();
//
//         let wallet = Wallet::new(
//             Arc::new(descriptor),
//             Some(Arc::new(change_descriptor)),
//             Network::Regtest,
//             DatabaseConfig::Memory,
//         )
//         .unwrap();
//
//         assert_eq!(
//             wallet
//                 .get_address(AddressIndex::Peek { index: 2 })
//                 .unwrap()
//                 .address
//                 .as_string(),
//             "bcrt1q5g0mq6dkmwzvxscqwgc932jhgcxuqqkjv09tkj"
//         );
//
//         assert_eq!(
//             wallet
//                 .get_address(AddressIndex::Peek { index: 1 })
//                 .unwrap()
//                 .address
//                 .as_string(),
//             "bcrt1q0xs7dau8af22rspp4klya4f7lhggcnqfun2y3a"
//         );
//
//         // new index still 0
//         assert_eq!(
//             wallet
//                 .get_address(AddressIndex::New)
//                 .unwrap()
//                 .address
//                 .as_string(),
//             "bcrt1qqjn9gky9mkrm3c28e5e87t5akd3twg6xezp0tv"
//         );
//
//         // new index now 1
//         assert_eq!(
//             wallet
//                 .get_address(AddressIndex::New)
//                 .unwrap()
//                 .address
//                 .as_string(),
//             "bcrt1q0xs7dau8af22rspp4klya4f7lhggcnqfun2y3a"
//         );
//
//         // new index now 2
//         assert_eq!(
//             wallet
//                 .get_address(AddressIndex::New)
//                 .unwrap()
//                 .address
//                 .as_string(),
//             "bcrt1q5g0mq6dkmwzvxscqwgc932jhgcxuqqkjv09tkj"
//         );
//
//         // peek index 1
//         assert_eq!(
//             wallet
//                 .get_address(AddressIndex::Peek { index: 1 })
//                 .unwrap()
//                 .address
//                 .as_string(),
//             "bcrt1q0xs7dau8af22rspp4klya4f7lhggcnqfun2y3a"
//         );
//
//         // reset to index 0
//         // assert_eq!(
//         //     wallet
//         //         .get_address(AddressIndex::Reset { index: 0 })
//         //         .unwrap()
//         //         .address
//         //         .as_string(),
//         //     "bcrt1qqjn9gky9mkrm3c28e5e87t5akd3twg6xezp0tv"
//         // );
//
//         // new index 1 again
//         assert_eq!(
//             wallet
//                 .get_address(AddressIndex::New)
//                 .unwrap()
//                 .address
//                 .as_string(),
//             "bcrt1q0xs7dau8af22rspp4klya4f7lhggcnqfun2y3a"
//         );
//     }
//
//     #[test]
//     fn test_get_address() {
//         let test_wpkh = "wpkh(tprv8hwWMmPE4BVNxGdVt3HhEERZhondQvodUY7Ajyseyhudr4WabJqWKWLr4Wi2r26CDaNCQhhxEftEaNzz7dPGhWuKFU4VULesmhEfZYyBXdE/0/*)";
//         let descriptor = Descriptor::new(test_wpkh.to_string(), Network::Regtest).unwrap();
//         let change_descriptor = Descriptor::new(
//             test_wpkh.to_string().replace("/0/*", "/1/*"),
//             Network::Regtest,
//         )
//         .unwrap();
//
//         let wallet = Wallet::new(
//             Arc::new(descriptor),
//             Some(Arc::new(change_descriptor)),
//             Network::Regtest,
//             DatabaseConfig::Memory,
//         )
//         .unwrap();
//
//         assert_eq!(
//             wallet
//                 .get_address(AddressIndex::New)
//                 .unwrap()
//                 .address
//                 .as_string(),
//             "bcrt1qqjn9gky9mkrm3c28e5e87t5akd3twg6xezp0tv"
//         );
//
//         assert_eq!(
//             wallet
//                 .get_address(AddressIndex::New)
//                 .unwrap()
//                 .address
//                 .as_string(),
//             "bcrt1q0xs7dau8af22rspp4klya4f7lhggcnqfun2y3a"
//         );
//
//         assert_eq!(
//             wallet
//                 .get_address(AddressIndex::LastUnused)
//                 .unwrap()
//                 .address
//                 .as_string(),
//             "bcrt1q0xs7dau8af22rspp4klya4f7lhggcnqfun2y3a"
//         );
//
//         assert_eq!(
//             wallet
//                 .get_internal_address(AddressIndex::New)
//                 .unwrap()
//                 .address
//                 .as_string(),
//             "bcrt1qpmz73cyx00r4a5dea469j40ax6d6kqyd67nnpj"
//         );
//
//         assert_eq!(
//             wallet
//                 .get_internal_address(AddressIndex::New)
//                 .unwrap()
//                 .address
//                 .as_string(),
//             "bcrt1qaux734vuhykww9632v8cmdnk7z2mw5lsf74v6k"
//         );
//
//         assert_eq!(
//             wallet
//                 .get_internal_address(AddressIndex::LastUnused)
//                 .unwrap()
//                 .address
//                 .as_string(),
//             "bcrt1qaux734vuhykww9632v8cmdnk7z2mw5lsf74v6k"
//         );
//     }
//
//     #[test]
//     fn test_is_mine() {
//         // is_mine should return true for addresses generated by the wallet
//         let mnemonic: Mnemonic = Mnemonic::from_string("chaos fabric time speed sponsor all flat solution wisdom trophy crack object robot pave observe combine where aware bench orient secret primary cable detect".to_string()).unwrap();
//         let secret_key: DescriptorSecretKey =
//             DescriptorSecretKey::new(Network::Testnet, Arc::new(mnemonic), None);
//         let descriptor: Descriptor = Descriptor::new_bip84(
//             Arc::new(secret_key),
//             KeychainKind::External,
//             Network::Testnet,
//         );
//         let wallet: Wallet = Wallet::new(
//             Arc::new(descriptor),
//             None,
//             Network::Testnet,
//             DatabaseConfig::Memory,
//         )
//         .unwrap();
//
//         // let address = wallet.get_address(AddressIndex::New).unwrap();
//         // let script: Arc<Script> = address.address.script_pubkey();
//
//         // let is_mine_1: bool = wallet.is_mine(script).unwrap();
//         // assert!(is_mine_1);
//
//         // is_mine returns false when provided a script that is not in the wallet
//         let other_wpkh = "wpkh(tprv8hwWMmPE4BVNxGdVt3HhEERZhondQvodUY7Ajyseyhudr4WabJqWKWLr4Wi2r26CDaNCQhhxEftEaNzz7dPGhWuKFU4VULesmhEfZYyBXdE/0/*)";
//         let other_descriptor = Descriptor::new(other_wpkh.to_string(), Network::Testnet).unwrap();
//
//         let other_wallet = Wallet::new(
//             Arc::new(other_descriptor),
//             None,
//             Network::Testnet,
//             DatabaseConfig::Memory,
//         )
//         .unwrap();
//
//         let other_address = other_wallet.get_address(AddressIndex::New).unwrap();
//         let other_script: Arc<Script> = other_address.address.script_pubkey();
//         let is_mine_2: bool = wallet.is_mine(other_script).unwrap();
//         assert_matches!(is_mine_2, false);
//     }
// }
