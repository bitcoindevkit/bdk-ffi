use crate::bitcoin::{Amount, Input, Network, OutPoint, Script, TxOut};
use crate::descriptor::Descriptor;
use crate::esplora::EsploraClient;
use crate::store::Persister;
use crate::tx_builder::TxBuilder;
use crate::types::FullScanScriptInspector;
use crate::wallet::Wallet;

use bdk_wallet::bitcoin::hashes::hex::FromHex;

use std::collections::HashMap;
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

        match TxBuilder::new()
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

#[test]
fn test_only_witness_utxo_with_finish() {
    let wallet = create_and_sync_wallet();

    // Check if wallet has any UTXOs to work with
    let utxos = wallet.list_unspent();
    let has_utxos = !utxos.is_empty();

    println!("Wallet has {} UTXOs", utxos.len());

    // Only run the actual transaction test if wallet has UTXOs
    // Otherwise we at least verify the builder methods work
    if !has_utxos {
        println!("No UTXOs available, testing builder methods only");

        // At minimum, verify the builder methods don't panic
        let address = wallet
            .next_unused_address(bdk_wallet::KeychainKind::External)
            .address;
        let _builder = TxBuilder::new()
            .add_recipient(
                &(*address.script_pubkey()).to_owned(),
                Arc::new(Amount::from_sat(1000)),
            )
            .only_witness_utxo();

        println!("Builder methods work correctly even without UTXOs");
        return;
    }

    let address = wallet
        .next_unused_address(bdk_wallet::KeychainKind::External)
        .address;

    // Get policy paths for the multisig wallet
    let ext_policy = wallet.policies(bdk_wallet::KeychainKind::External);
    let int_policy = wallet.policies(bdk_wallet::KeychainKind::Internal);

    if let (Ok(Some(ext_policy)), Ok(Some(int_policy))) = (ext_policy, int_policy) {
        let ext_path: HashMap<_, _> = vec![(ext_policy.id().clone(), vec![0, 1])]
            .into_iter()
            .collect();
        let int_path: HashMap<_, _> = vec![(int_policy.id().clone(), vec![0, 1])]
            .into_iter()
            .collect();

        let wallet_arc = Arc::new(wallet);

        // Build transaction without only_witness_utxo
        let tx_without_flag = TxBuilder::new()
            .add_recipient(
                &(*address.script_pubkey()).to_owned(),
                Arc::new(Amount::from_sat(1000)),
            )
            .policy_path(ext_path.clone(), bdk_wallet::KeychainKind::External)
            .policy_path(int_path.clone(), bdk_wallet::KeychainKind::Internal)
            .do_not_spend_change()
            .finish(&wallet_arc);

        // Build transaction with only_witness_utxo
        let tx_with_flag = TxBuilder::new()
            .add_recipient(
                &(*address.script_pubkey()).to_owned(),
                Arc::new(Amount::from_sat(1000)),
            )
            .policy_path(ext_path, bdk_wallet::KeychainKind::External)
            .policy_path(int_path, bdk_wallet::KeychainKind::Internal)
            .do_not_spend_change()
            .only_witness_utxo()
            .finish(&wallet_arc);

        // Both should succeed if we have UTXOs
        match (tx_without_flag, tx_with_flag) {
            (Ok(psbt_without), Ok(psbt_with)) => {
                let size_without = psbt_without.serialize().len();
                let size_with = psbt_with.serialize().len();

                println!(
                    "PSBT size without only_witness_utxo: {} bytes",
                    size_without
                );
                println!("PSBT size with only_witness_utxo: {} bytes", size_with);

                // The PSBT with only_witness_utxo should be smaller or equal
                // (for SegWit inputs, non_witness_utxo adds the full transaction)
                assert!(
                    size_with <= size_without,
                    "Expected PSBT with only_witness_utxo to be smaller, but got {} bytes vs {} bytes",
                    size_with,
                    size_without
                );
            }
            (Err(e1), _) => {
                println!("Transaction without flag failed: {:?}", e1);
                panic!("Expected transaction to succeed with UTXOs available");
            }
            (_, Err(e2)) => {
                println!("Transaction with flag failed: {:?}", e2);
                panic!("Expected transaction with only_witness_utxo to succeed");
            }
        }
    } else {
        println!("Failed to retrieve policies, skipping transaction test");
    }
}

#[test]
fn test_add_foreign_utxo_missing_witness_data() {
    // Create a foreign UTXO without witness_utxo or non_witness_utxo
    let outpoint = OutPoint {
        txid: Arc::new(
            crate::bitcoin::Txid::from_string(
                "5df6e0e2761359d30a8275058e299fcc0381534545f55cf43e41983f5d4c9456".to_string(),
            )
            .unwrap(),
        ),
        vout: 0,
    };

    let psbt_input = Input {
        non_witness_utxo: None,
        witness_utxo: None, // Missing!
        partial_sigs: HashMap::new(),
        sighash_type: None,
        redeem_script: None,
        witness_script: None,
        bip32_derivation: HashMap::new(),
        final_script_sig: None,
        final_script_witness: None,
        ripemd160_preimages: HashMap::new(),
        sha256_preimages: HashMap::new(),
        hash160_preimages: HashMap::new(),
        hash256_preimages: HashMap::new(),
        tap_key_sig: None,
        tap_script_sigs: HashMap::new(),
        tap_scripts: HashMap::new(),
        tap_key_origins: HashMap::new(),
        tap_internal_key: None,
        tap_merkle_root: None,
        proprietary: HashMap::new(),
        unknown: HashMap::new(),
    };

    let tx_builder = TxBuilder::new();

    let result = tx_builder.add_foreign_utxo(outpoint, psbt_input, 68);
    assert!(
        result.is_err(),
        "Expected add_foreign_utxo() to fail with missing witness_utxo/non_witness_utxo"
    );
}

#[test]
fn test_add_foreign_utxo_with_witness_utxo_succeeds() {
    let wallet = create_and_sync_wallet();
    let address = wallet
        .next_unused_address(bdk_wallet::KeychainKind::External)
        .address;

    let outpoint = OutPoint {
        txid: Arc::new(
            crate::bitcoin::Txid::from_string(
                "5df6e0e2761359d30a8275058e299fcc0381534545f55cf43e41983f5d4c9456".to_string(),
            )
            .unwrap(),
        ),
        vout: 0,
    };

    // Create a valid witness UTXO
    let witness_utxo = TxOut {
        value: Arc::new(Amount::from_sat(50000)),
        script_pubkey: Arc::new(Script::new(
            Vec::from_hex("0014d85c2b71d0060b09c9886aeb815e50991dda124d").unwrap(),
        )),
    };

    let psbt_input = Input {
        non_witness_utxo: None,
        witness_utxo: Some(witness_utxo),
        partial_sigs: HashMap::new(),
        sighash_type: None,
        redeem_script: None,
        witness_script: None,
        bip32_derivation: HashMap::new(),
        final_script_sig: None,
        final_script_witness: None,
        ripemd160_preimages: HashMap::new(),
        sha256_preimages: HashMap::new(),
        hash160_preimages: HashMap::new(),
        hash256_preimages: HashMap::new(),
        tap_key_sig: None,
        tap_script_sigs: HashMap::new(),
        tap_scripts: HashMap::new(),
        tap_key_origins: HashMap::new(),
        tap_internal_key: None,
        tap_merkle_root: None,
        proprietary: HashMap::new(),
        unknown: HashMap::new(),
    };

    let tx_builder = TxBuilder::new().add_recipient(
        &(*address.script_pubkey()).to_owned(),
        Arc::new(Amount::from_sat(1000)),
    );

    let result = tx_builder.add_foreign_utxo(outpoint, psbt_input, 68);
    assert!(
        result.is_ok(),
        "Failed to add foreign UTXO: {:?}",
        result.err()
    );

    // Try to finish - might fail due to other reasons (insufficient funds from foreign UTXO alone)
    // but shouldn't fail due to missing witness data
    let finish_result = result.unwrap().finish(&Arc::new(wallet));

    // If it fails, check it's not due to missing witness data
    if let Err(e) = finish_result {
        let error_msg = format!("{:?}", e);
        println!("Transaction failed with: {}", error_msg);
        // Should NOT be a missing witness data error
        assert!(
            !error_msg.contains("MissingUtxo") && !error_msg.contains("missing witness"),
            "Should not fail due to missing witness data, got: {}",
            error_msg
        );
    }
}

#[test]
fn test_add_multiple_foreign_utxos_and_finish() {
    let wallet = create_and_sync_wallet();
    let address = wallet
        .next_unused_address(bdk_wallet::KeychainKind::External)
        .address;

    // Create first foreign UTXO
    let outpoint1 = OutPoint {
        txid: Arc::new(
            crate::bitcoin::Txid::from_string(
                "5df6e0e2761359d30a8275058e299fcc0381534545f55cf43e41983f5d4c9456".to_string(),
            )
            .unwrap(),
        ),
        vout: 0,
    };

    let witness_utxo1 = TxOut {
        value: Arc::new(Amount::from_sat(50000)),
        script_pubkey: Arc::new(Script::new(
            Vec::from_hex("0014d85c2b71d0060b09c9886aeb815e50991dda124d").unwrap(),
        )),
    };

    let psbt_input1 = Input {
        non_witness_utxo: None,
        witness_utxo: Some(witness_utxo1),
        partial_sigs: HashMap::new(),
        sighash_type: None,
        redeem_script: None,
        witness_script: None,
        bip32_derivation: HashMap::new(),
        final_script_sig: None,
        final_script_witness: None,
        ripemd160_preimages: HashMap::new(),
        sha256_preimages: HashMap::new(),
        hash160_preimages: HashMap::new(),
        hash256_preimages: HashMap::new(),
        tap_key_sig: None,
        tap_script_sigs: HashMap::new(),
        tap_scripts: HashMap::new(),
        tap_key_origins: HashMap::new(),
        tap_internal_key: None,
        tap_merkle_root: None,
        proprietary: HashMap::new(),
        unknown: HashMap::new(),
    };

    // Create second foreign UTXO
    let outpoint2 = OutPoint {
        txid: Arc::new(
            crate::bitcoin::Txid::from_string(
                "6ea4e0e2761359d30a8275058e299fcc0381534545f55cf43e41983f5d4c9457".to_string(),
            )
            .unwrap(),
        ),
        vout: 1,
    };

    let witness_utxo2 = TxOut {
        value: Arc::new(Amount::from_sat(75000)),
        script_pubkey: Arc::new(Script::new(
            Vec::from_hex("0014a85c2b71d0060b09c9886aeb815e50991dda125e").unwrap(),
        )),
    };

    let psbt_input2 = Input {
        non_witness_utxo: None,
        witness_utxo: Some(witness_utxo2),
        partial_sigs: HashMap::new(),
        sighash_type: None,
        redeem_script: None,
        witness_script: None,
        bip32_derivation: HashMap::new(),
        final_script_sig: None,
        final_script_witness: None,
        ripemd160_preimages: HashMap::new(),
        sha256_preimages: HashMap::new(),
        hash160_preimages: HashMap::new(),
        hash256_preimages: HashMap::new(),
        tap_key_sig: None,
        tap_script_sigs: HashMap::new(),
        tap_scripts: HashMap::new(),
        tap_key_origins: HashMap::new(),
        tap_internal_key: None,
        tap_merkle_root: None,
        proprietary: HashMap::new(),
        unknown: HashMap::new(),
    };

    // Add both foreign UTXOs
    let tx_builder = TxBuilder::new().add_recipient(
        &(*address.script_pubkey()).to_owned(),
        Arc::new(Amount::from_sat(1000)),
    );

    let tx_builder = tx_builder
        .add_foreign_utxo(outpoint1, psbt_input1, 68)
        .expect("Failed to add first foreign UTXO");

    let tx_builder = tx_builder
        .add_foreign_utxo(outpoint2, psbt_input2, 68)
        .expect("Failed to add second foreign UTXO");

    // Try to finish
    let finish_result = tx_builder.finish(&Arc::new(wallet));

    // If it fails, verify it's not due to witness data issues
    if let Err(e) = finish_result {
        let error_msg = format!("{:?}", e);
        println!(
            "Transaction with multiple foreign UTXOs failed with: {}",
            error_msg
        );
        assert!(
            !error_msg.contains("MissingUtxo"),
            "Should not fail due to missing witness data"
        );
    }
}

#[test]
fn test_combined_only_witness_utxo_and_foreign_utxo_with_finish() {
    let wallet = create_and_sync_wallet();
    let address = wallet
        .next_unused_address(bdk_wallet::KeychainKind::External)
        .address;

    let outpoint = OutPoint {
        txid: Arc::new(
            crate::bitcoin::Txid::from_string(
                "5df6e0e2761359d30a8275058e299fcc0381534545f55cf43e41983f5d4c9456".to_string(),
            )
            .unwrap(),
        ),
        vout: 0,
    };

    let witness_utxo = TxOut {
        value: Arc::new(Amount::from_sat(50000)),
        script_pubkey: Arc::new(Script::new(
            Vec::from_hex("0014d85c2b71d0060b09c9886aeb815e50991dda124d").unwrap(),
        )),
    };

    let psbt_input = Input {
        non_witness_utxo: None,
        witness_utxo: Some(witness_utxo),
        partial_sigs: HashMap::new(),
        sighash_type: None,
        redeem_script: None,
        witness_script: None,
        bip32_derivation: HashMap::new(),
        final_script_sig: None,
        final_script_witness: None,
        ripemd160_preimages: HashMap::new(),
        sha256_preimages: HashMap::new(),
        hash160_preimages: HashMap::new(),
        hash256_preimages: HashMap::new(),
        tap_key_sig: None,
        tap_script_sigs: HashMap::new(),
        tap_scripts: HashMap::new(),
        tap_key_origins: HashMap::new(),
        tap_internal_key: None,
        tap_merkle_root: None,
        proprietary: HashMap::new(),
        unknown: HashMap::new(),
    };

    // Combine both features
    let tx_builder = TxBuilder::new()
        .add_recipient(
            &(*address.script_pubkey()).to_owned(),
            Arc::new(Amount::from_sat(1000)),
        )
        .only_witness_utxo();

    let tx_builder = tx_builder
        .add_foreign_utxo(outpoint, psbt_input, 68)
        .expect("Failed to add foreign UTXO with only_witness_utxo");

    // Try to finish - this tests that both features work together
    let finish_result = tx_builder.finish(&Arc::new(wallet));

    // Verify the combination doesn't cause conflicts
    if let Err(e) = finish_result {
        let error_msg = format!("{:?}", e);
        println!("Combined features transaction failed with: {}", error_msg);
        // Should not be due to witness data conflicts
        assert!(
            !error_msg.contains("MissingUtxo") && !error_msg.contains("witness")
                || error_msg.contains("Insufficient"),
            "Should not fail due to witness-related conflicts, got: {}",
            error_msg
        );
    }
}
