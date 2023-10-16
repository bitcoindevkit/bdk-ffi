// // The goal of these tests to to ensure `bdk-ffi` intermediate code correctly calls `bdk` APIs.
// // These tests should not be used to verify `bdk` behavior that is already tested in the `bdk`
// // crate.
// #[cfg(test)]
// mod test {
//     use crate::database::DatabaseConfig;
//     use crate::descriptor::Descriptor;
//     use crate::keys::{DescriptorSecretKey, Mnemonic};
//     use crate::wallet::{AddressIndex, TxBuilder, Wallet};
//     use crate::Script;
//     use assert_matches::assert_matches;
//     use bdk::bitcoin::{Address, Network};
//     use bdk::wallet::get_funded_wallet;
//     use bdk::KeychainKind;
//     use std::str::FromStr;
//     use std::sync::{Arc, Mutex};
//
//     #[test]
//     fn test_drain_wallet() {
//         let test_wpkh = "wpkh(cVpPVruEDdmutPzisEsYvtST1usBR3ntr8pXSyt6D2YYqXRyPcFW)";
//         let (funded_wallet, _, _) = get_funded_wallet(test_wpkh);
//         let test_wallet = Wallet {
//             inner_mutex: Mutex::new(funded_wallet),
//         };
//         let drain_to_address = "tb1ql7w62elx9ucw4pj5lgw4l028hmuw80sndtntxt".to_string();
//         let drain_to_script = crate::Address::new(drain_to_address)
//             .unwrap()
//             .script_pubkey();
//         let tx_builder = TxBuilder::new()
//             .drain_wallet()
//             .drain_to(drain_to_script.clone());
//         assert!(tx_builder.drain_wallet);
//         assert_eq!(tx_builder.drain_to, Some(drain_to_script.inner.clone()));
//
//         let tx_builder_result = tx_builder.finish(&test_wallet).unwrap();
//         let psbt = tx_builder_result.psbt.inner.lock().unwrap().clone();
//         let tx_details = tx_builder_result.transaction_details;
//
//         // confirm one input with 50,000 sats
//         assert_eq!(psbt.inputs.len(), 1);
//         let input_value = psbt
//             .inputs
//             .get(0)
//             .cloned()
//             .unwrap()
//             .non_witness_utxo
//             .unwrap()
//             .output
//             .get(0)
//             .unwrap()
//             .value;
//         assert_eq!(input_value, 50_000_u64);
//
//         // confirm one output to correct address with all sats - fee
//         assert_eq!(psbt.outputs.len(), 1);
//         let output_address = Address::from_script(
//             &psbt
//                 .unsigned_tx
//                 .output
//                 .get(0)
//                 .cloned()
//                 .unwrap()
//                 .script_pubkey,
//             Network::Testnet,
//         )
//         .unwrap();
//         assert_eq!(
//             output_address,
//             Address::from_str("tb1ql7w62elx9ucw4pj5lgw4l028hmuw80sndtntxt").unwrap()
//         );
//         let output_value = psbt.unsigned_tx.output.get(0).cloned().unwrap().value;
//         assert_eq!(output_value, 49_890_u64); // input - fee
//
//         assert_eq!(
//             tx_details.txid,
//             "312f1733badab22dc26b8dcbc83ba5629fb7b493af802e8abe07d865e49629c5"
//         );
//         assert_eq!(tx_details.received, 0);
//         assert_eq!(tx_details.sent, 50000);
//         assert!(tx_details.fee.is_some());
//         assert_eq!(tx_details.fee.unwrap(), 110);
//         assert!(tx_details.confirmation_time.is_none());
//     }
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
//         assert_eq!(
//             wallet
//                 .get_address(AddressIndex::Reset { index: 0 })
//                 .unwrap()
//                 .address
//                 .as_string(),
//             "bcrt1qqjn9gky9mkrm3c28e5e87t5akd3twg6xezp0tv"
//         );
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
//         let address = wallet.get_address(AddressIndex::New).unwrap();
//         let script: Arc<Script> = address.address.script_pubkey();
//
//         let is_mine_1: bool = wallet.is_mine(script).unwrap();
//         assert!(is_mine_1);
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
use crate::descriptor::Descriptor;
use crate::Balance;
use crate::{AddressIndex, AddressInfo, Network};
use bdk::wallet::Update as BdkUpdate;
use bdk::Error as BdkError;
use bdk::Wallet as BdkWallet;
use std::sync::{Arc, Mutex, MutexGuard};
pub enum WalletType {
    Memory,
    FlatFile,
}

pub struct Wallet {
    pub inner_mutex: Mutex<BdkWallet>,
}

impl Wallet {
    pub fn new(
        descriptor: Arc<Descriptor>,
        change_descriptor: Option<Arc<Descriptor>>,
        network: Network,
        wallet_type: WalletType,
    ) -> Result<Self, BdkError> {
        let descriptor = descriptor.as_string_private();
        let change_descriptor = change_descriptor.map(|d| d.as_string_private());

        match wallet_type {
            WalletType::Memory => {
                let wallet = BdkWallet::new_no_persist(
                    &descriptor,
                    change_descriptor.as_ref(),
                    network.into(),
                )?;
                Ok(Wallet {
                    inner_mutex: Mutex::new(wallet),
                })
            }
            WalletType::FlatFile => {
                panic!("FlatFile wallet type not yet implemented")
            }
        }
    }

    pub fn get_address(&self, address_index: AddressIndex) -> AddressInfo {
        self.get_wallet().get_address(address_index.into()).into()
    }

    // TODO: Do we need this mutex
    pub(crate) fn get_wallet(&self) -> MutexGuard<BdkWallet> {
        self.inner_mutex.lock().expect("wallet")
    }

    // TODO: Why is the Arc required here?
    pub fn get_balance(&self) -> Arc<Balance> {
        // Arc::new(self.get_wallet().get_balance().into())
        let bdk_balance = self.get_wallet().get_balance();
        let balance = Balance { inner: bdk_balance };
        Arc::new(balance)
    }

    pub fn network(&self) -> Network {
        self.get_wallet().network().into()
    }

    pub fn apply_update(&self, update: Arc<Update>) -> Result<(), BdkError> {
        // self.get_wallet(). .apply_update(update.0).map_err(|e| BdkError::Generic(e.to_string()))
        self.get_wallet()
            .apply_update(update.0.clone())
            .map_err(|e| BdkError::Generic(e.to_string()))
    }
}

pub struct Update(pub(crate) BdkUpdate);
