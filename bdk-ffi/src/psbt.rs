// // The goal of these tests to to ensure `bdk-ffi` intermediate code correctly calls `bdk` APIs.
// // These tests should not be used to verify `bdk` behavior that is already tested in the `bdk`
// // crate.
// #[cfg(test)]
// mod test {
//     use crate::wallet::{TxBuilder, Wallet};
//     use bdk::wallet::get_funded_wallet;
//     use std::sync::Mutex;
//
//     #[test]
//     fn test_psbt_fee() {
//         let test_wpkh = "wpkh(cVpPVruEDdmutPzisEsYvtST1usBR3ntr8pXSyt6D2YYqXRyPcFW)";
//         let (funded_wallet, _, _) = get_funded_wallet(test_wpkh);
//         let test_wallet = Wallet {
//             inner_mutex: Mutex::new(funded_wallet),
//         };
//         let drain_to_address = "tb1ql7w62elx9ucw4pj5lgw4l028hmuw80sndtntxt".to_string();
//         let drain_to_script = crate::Address::new(drain_to_address)
//             .unwrap()
//             .script_pubkey();
//
//         let tx_builder = TxBuilder::new()
//             .fee_rate(2.0)
//             .drain_wallet()
//             .drain_to(drain_to_script.clone());
//         //dbg!(&tx_builder);
//         assert!(tx_builder.drain_wallet);
//         assert_eq!(tx_builder.drain_to, Some(drain_to_script.inner.clone()));
//
//         let tx_builder_result = tx_builder.finish(&test_wallet).unwrap();
//
//         assert!(tx_builder_result.psbt.fee_rate().is_some());
//         assert_eq!(
//             tx_builder_result.psbt.fee_rate().unwrap().as_sat_per_vb(),
//             2.682927
//         );
//
//         assert!(tx_builder_result.psbt.fee_amount().is_some());
//         assert_eq!(tx_builder_result.psbt.fee_amount().unwrap(), 220);
//     }
// }
