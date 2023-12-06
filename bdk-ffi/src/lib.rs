mod bitcoin;
mod descriptor;
mod esplora;
mod keys;
mod types;
mod wallet;

use crate::bitcoin::Address;
use crate::bitcoin::Network;
use crate::bitcoin::OutPoint;
use crate::bitcoin::PartiallySignedTransaction;
use crate::bitcoin::Script;
use crate::bitcoin::Transaction;
use crate::bitcoin::TxOut;
use crate::descriptor::Descriptor;
use crate::esplora::EsploraClient;
use crate::keys::DerivationPath;
use crate::keys::DescriptorPublicKey;
use crate::keys::DescriptorSecretKey;
use crate::keys::Mnemonic;
use crate::types::AddressIndex;
use crate::types::AddressInfo;
use crate::types::Balance;
use crate::types::LocalUtxo;
use crate::types::ScriptAmount;
use crate::wallet::TxBuilder;
use crate::wallet::Update;
use crate::wallet::Wallet;

use bdk::keys::bip39::WordCount;
use bdk::wallet::tx_builder::ChangeSpendPolicy;
use bdk::Error as BdkError;
use bdk::KeychainKind;

uniffi::include_scaffolding!("bdk");

// #[derive(Debug, Clone)]
// pub struct TxIn {
//     pub previous_output: OutPoint,
//     pub script_sig: Arc<Script>,
//     pub sequence: u32,
//     pub witness: Vec<Vec<u8>>,
// }
//
// impl From<&BdkTxIn> for TxIn {
//     fn from(tx_in: &BdkTxIn) -> Self {
//         TxIn {
//             previous_output: OutPoint {
//                 txid: tx_in.previous_output.txid.to_string(),
//                 vout: tx_in.previous_output.vout,
//             },
//             script_sig: Arc::new(Script {
//                 inner: tx_in.script_sig.clone(),
//             }),
//             sequence: tx_in.sequence.0,
//             witness: tx_in.witness.to_vec(),
//         }
//     }
// }

// /// The method used to produce an address.
// #[derive(Debug)]
// pub enum Payload {
//     /// P2PKH address.
//     PubkeyHash { pubkey_hash: Vec<u8> },
//     /// P2SH address.
//     ScriptHash { script_hash: Vec<u8> },
//     /// Segwit address.
//     WitnessProgram {
//         /// The witness program version.
//         version: WitnessVersion,
//         /// The witness program.
//         program: Vec<u8>,
//     },
// }

// /// The result after calling the TxBuilder finish() function. Contains unsigned PSBT and
// /// transaction details.
// pub struct TxBuilderResult {
//     pub(crate) psbt: Arc<PartiallySignedTransaction>,
//     pub transaction_details: TransactionDetails,
// }
//
// uniffi::deps::static_assertions::assert_impl_all!(Wallet: Sync, Send);
//
// // The goal of these tests to to ensure `bdk-ffi` intermediate code correctly calls `bdk` APIs.
// // These tests should not be used to verify `bdk` behavior that is already tested in the `bdk`
// // crate.
// #[cfg(test)]
// mod test {
//     use super::Transaction;
//     use crate::Network::Regtest;
//     use crate::{Address, Payload};
//     use assert_matches::assert_matches;
//     use bdk::bitcoin::hashes::hex::FromHex;
//     use bdk::bitcoin::util::address::WitnessVersion;
//
//     // Verify that bdk-ffi Transaction can be created from valid bytes and serialized back into the same bytes.
//     #[test]
//     fn test_transaction_serde() {
//         let test_tx_bytes = Vec::from_hex("020000000001031cfbc8f54fbfa4a33a30068841371f80dbfe166211242213188428f437445c91000000006a47304402206fbcec8d2d2e740d824d3d36cc345b37d9f65d665a99f5bd5c9e8d42270a03a8022013959632492332200c2908459547bf8dbf97c65ab1a28dec377d6f1d41d3d63e012103d7279dfb90ce17fe139ba60a7c41ddf605b25e1c07a4ddcb9dfef4e7d6710f48feffffff476222484f5e35b3f0e43f65fc76e21d8be7818dd6a989c160b1e5039b7835fc00000000171600140914414d3c94af70ac7e25407b0689e0baa10c77feffffffa83d954a62568bbc99cc644c62eb7383d7c2a2563041a0aeb891a6a4055895570000000017160014795d04cc2d4f31480d9a3710993fbd80d04301dffeffffff06fef72f000000000017a91476fd7035cd26f1a32a5ab979e056713aac25796887a5000f00000000001976a914b8332d502a529571c6af4be66399cd33379071c588ac3fda0500000000001976a914fc1d692f8de10ae33295f090bea5fe49527d975c88ac522e1b00000000001976a914808406b54d1044c429ac54c0e189b0d8061667e088ac6eb68501000000001976a914dfab6085f3a8fb3e6710206a5a959313c5618f4d88acbba20000000000001976a914eb3026552d7e3f3073457d0bee5d4757de48160d88ac0002483045022100bee24b63212939d33d513e767bc79300051f7a0d433c3fcf1e0e3bf03b9eb1d70220588dc45a9ce3a939103b4459ce47500b64e23ab118dfc03c9caa7d6bfc32b9c601210354fd80328da0f9ae6eef2b3a81f74f9a6f66761fadf96f1d1d22b1fd6845876402483045022100e29c7e3a5efc10da6269e5fc20b6a1cb8beb92130cc52c67e46ef40aaa5cac5f0220644dd1b049727d991aece98a105563416e10a5ac4221abac7d16931842d5c322012103960b87412d6e169f30e12106bdf70122aabb9eb61f455518322a18b920a4dfa887d30700").unwrap();
//         let new_tx_from_bytes = Transaction::new(test_tx_bytes.clone()).unwrap();
//         let serialized_tx_to_bytes = new_tx_from_bytes.serialize();
//         assert_eq!(test_tx_bytes, serialized_tx_to_bytes);
//     }
//
//     // Verify that bdk-ffi Address.payload includes expected WitnessProgram variant, version and program bytes.
//     #[test]
//     fn test_address_witness_program() {
//         let address =
//             Address::new("bcrt1qqjn9gky9mkrm3c28e5e87t5akd3twg6xezp0tv".to_string()).unwrap();
//         let payload = address.payload();
//         assert_matches!(payload, Payload::WitnessProgram { version, program } => {
//             assert_eq!(version,WitnessVersion::V0);
//             assert_eq!(program, Vec::from_hex("04a6545885dd87b8e147cd327f2e9db362b72346").unwrap());
//         });
//         assert_eq!(address.network(), Regtest);
//     }
// }
