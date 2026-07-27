use crate::bitcoin::{Amount, Network, NetworkKind, Transaction};
use crate::descriptor::Descriptor;
use crate::silent_payments::{
    SilentPaymentCode, SilentPaymentCodeParseError, SilentPaymentRecipient, SilentPaymentSendError,
};
use crate::store::Persister;
use crate::tx_builder::TxBuilder;
use crate::types::UnconfirmedTx;
use crate::wallet::Wallet;

use bdk_sp::receive::scan::Scanner;
use bdk_wallet::bitcoin::{
    absolute::LockTime, ecdsa, hashes::Hash, sighash::SighashCache, transaction::Version,
    Amount as BdkAmount, CompressedPublicKey, OutPoint as BdkOutPoint, PrivateKey, ScriptBuf,
    Sequence, Transaction as BdkTransaction, TxIn as BdkTxIn, TxOut as BdkTxOut, Txid as BdkTxid,
    Witness, XOnlyPublicKey,
};
use bdk_wallet::KeychainKind;

use std::collections::BTreeMap;
use std::sync::Arc;

const TSP_CODE: &str =
    "tsp1qq0u4yswlkqx36shz7j8mwt335p4el5txc8tt6yny3dqewlw4rwdqkqewtzh728u7mzkne3uf0a35mzqlm0jf4q2kgc5aakq4d04a9l734uxwehmt";
const REGTEST_CODE: &str =
    "sprt1qqw7zfpjcuwvq4zd3d4aealxq3d669s3kcde4wgr3zl5ugxs40twv2qccgvszutt7p796yg4h926kdnty66wxrfew26gu2gk5h5hcg4s2jqyascfz";

#[test]
fn silent_payment_code_roundtrips_and_checks_network_family() {
    let testnet_code = SilentPaymentCode::new(TSP_CODE.to_string()).unwrap();
    assert_eq!(testnet_code.to_string(), TSP_CODE);
    assert!(testnet_code.is_valid_for_network(Network::Testnet));
    assert!(testnet_code.is_valid_for_network(Network::Testnet4));
    assert!(testnet_code.is_valid_for_network(Network::Signet));
    assert!(!testnet_code.is_valid_for_network(Network::Bitcoin));
    assert!(!testnet_code.is_valid_for_network(Network::Regtest));

    let regtest_code = SilentPaymentCode::new(REGTEST_CODE.to_string()).unwrap();
    assert_eq!(regtest_code.to_string(), REGTEST_CODE);
    assert!(regtest_code.is_valid_for_network(Network::Regtest));
    assert!(!regtest_code.is_valid_for_network(Network::Testnet));

    assert!(matches!(
        SilentPaymentCode::new("not-a-silent-payment-code".to_string()),
        Err(SilentPaymentCodeParseError::Bech32 { .. })
    ));
    assert!(matches!(
        SilentPaymentCode::new("sp10ajr90".to_string()),
        Err(SilentPaymentCodeParseError::Version { .. })
    ));
}

#[test]
fn finish_and_sign_silent_payments_derives_spendable_outputs() {
    const EXTERNAL_DESCRIPTOR: &str = "wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/1h/0/*)";
    const INTERNAL_DESCRIPTOR: &str = "wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/1h/1/*)";
    const SCAN_WIF: &str = "cTiSJ8p2zpGSkWGkvYFWfKurgWvSi9hdvzw9GEws18kS2VRPNS24";
    const SPEND_WIF: &str = "cRFcZbp7cAeZGsnYKdgSZwH6drJ3XLnPSGcjLNCpRy28tpGtZR11";

    let wallet = Arc::new(
        Wallet::new(
            Arc::new(Descriptor::new(EXTERNAL_DESCRIPTOR.to_string(), NetworkKind::Test).unwrap()),
            Arc::new(Descriptor::new(INTERNAL_DESCRIPTOR.to_string(), NetworkKind::Test).unwrap()),
            Network::Regtest,
            Arc::new(Persister::new_in_memory().unwrap()),
            25,
        )
        .unwrap(),
    );
    let funding_script = wallet
        .reveal_next_address(KeychainKind::External)
        .address
        .script_pubkey()
        .0
        .clone();
    let funding_output = BdkTxOut {
        value: BdkAmount::from_sat(100_000),
        script_pubkey: funding_script,
    };
    let funding_transaction = BdkTransaction {
        version: Version::TWO,
        lock_time: LockTime::ZERO,
        input: vec![BdkTxIn {
            previous_output: BdkOutPoint::new(BdkTxid::all_zeros(), 0),
            script_sig: Default::default(),
            sequence: Sequence::MAX,
            witness: Witness::default(),
        }],
        output: vec![funding_output.clone()],
    };
    wallet.apply_unconfirmed_txs(vec![UnconfirmedTx {
        tx: Arc::new(Transaction::from(funding_transaction)),
        last_seen: 1,
    }]);

    let code = Arc::new(SilentPaymentCode::new(REGTEST_CODE.to_string()).unwrap());
    let transaction = TxBuilder::new()
        .fee_absolute(Arc::new(Amount::from_sat(500)))
        .finish_and_sign_silent_payments(
            &wallet,
            vec![
                SilentPaymentRecipient {
                    code: Arc::clone(&code),
                    amount: Arc::new(Amount::from_sat(40_000)),
                },
                SilentPaymentRecipient {
                    code: Arc::clone(&code),
                    amount: Arc::new(Amount::from_sat(20_000)),
                },
            ],
        )
        .unwrap();
    let transaction: BdkTransaction = transaction.as_ref().into();

    assert_eq!(transaction.input.len(), 1);
    assert_eq!(
        transaction.input[0].sequence,
        Sequence::ENABLE_LOCKTIME_NO_RBF
    );
    let witness = &transaction.input[0].witness;
    assert_eq!(witness.len(), 2);
    let signature = ecdsa::Signature::from_slice(witness.nth(0).unwrap()).unwrap();
    let public_key = CompressedPublicKey::from_slice(witness.nth(1).unwrap()).unwrap();
    assert_eq!(
        ScriptBuf::new_p2wpkh(&public_key.wpubkey_hash()),
        funding_output.script_pubkey
    );
    let sighash = SighashCache::new(&transaction)
        .p2wpkh_signature_hash(
            0,
            &funding_output.script_pubkey,
            funding_output.value,
            signature.sighash_type,
        )
        .unwrap();
    let message = bdk_wallet::bitcoin::secp256k1::Message::from(sighash);
    public_key
        .verify(
            &bdk_wallet::bitcoin::secp256k1::Secp256k1::verification_only(),
            &message,
            &signature,
        )
        .unwrap();
    assert!(transaction
        .output
        .iter()
        .all(|output| output.script_pubkey != code.0.get_placeholder_p2tr_spk()));

    let secp = bdk_wallet::bitcoin::secp256k1::Secp256k1::new();
    let scan_secret = PrivateKey::from_wif(SCAN_WIF).unwrap().inner;
    let spend_secret = PrivateKey::from_wif(SPEND_WIF).unwrap().inner;
    let scanner = Scanner::new(scan_secret, code.0.spend, BTreeMap::new());
    let found_outputs = scanner.scan_tx(&transaction, &[funding_output]).unwrap();

    assert_eq!(found_outputs.len(), 2);
    let mut found_amounts = found_outputs
        .iter()
        .map(|output| output.amount.to_sat())
        .collect::<Vec<_>>();
    found_amounts.sort_unstable();
    assert_eq!(found_amounts, vec![20_000, 40_000]);
    assert_ne!(
        found_outputs[0].script_pubkey,
        found_outputs[1].script_pubkey
    );

    for found_output in found_outputs {
        assert!(transaction
            .output
            .iter()
            .any(|output| output.script_pubkey == found_output.script_pubkey));

        let output_secret = spend_secret.add_tweak(&found_output.tweak.into()).unwrap();
        let output_public_key =
            XOnlyPublicKey::from_slice(&found_output.script_pubkey.as_bytes()[2..]).unwrap();
        assert_eq!(output_secret.x_only_public_key(&secp).0, output_public_key);
    }
}

#[test]
fn finish_and_sign_silent_payments_with_p2tr_input_derives_spendable_output() {
    const EXTERNAL_DESCRIPTOR: &str = "tr(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/86h/1h/1h/0/*)";
    const INTERNAL_DESCRIPTOR: &str = "tr(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/86h/1h/1h/1/*)";
    const SCAN_WIF: &str = "cTiSJ8p2zpGSkWGkvYFWfKurgWvSi9hdvzw9GEws18kS2VRPNS24";
    const SPEND_WIF: &str = "cRFcZbp7cAeZGsnYKdgSZwH6drJ3XLnPSGcjLNCpRy28tpGtZR11";

    let wallet = Arc::new(
        Wallet::new(
            Arc::new(Descriptor::new(EXTERNAL_DESCRIPTOR.to_string(), NetworkKind::Test).unwrap()),
            Arc::new(Descriptor::new(INTERNAL_DESCRIPTOR.to_string(), NetworkKind::Test).unwrap()),
            Network::Regtest,
            Arc::new(Persister::new_in_memory().unwrap()),
            25,
        )
        .unwrap(),
    );
    let funding_script = wallet
        .reveal_next_address(KeychainKind::External)
        .address
        .script_pubkey()
        .0
        .clone();
    let funding_output = BdkTxOut {
        value: BdkAmount::from_sat(100_000),
        script_pubkey: funding_script,
    };
    let funding_transaction = BdkTransaction {
        version: Version::TWO,
        lock_time: LockTime::ZERO,
        input: vec![BdkTxIn {
            previous_output: BdkOutPoint::new(BdkTxid::all_zeros(), 0),
            script_sig: Default::default(),
            sequence: Sequence::MAX,
            witness: Witness::default(),
        }],
        output: vec![funding_output.clone()],
    };
    wallet.apply_unconfirmed_txs(vec![UnconfirmedTx {
        tx: Arc::new(Transaction::from(funding_transaction)),
        last_seen: 1,
    }]);

    let code = Arc::new(SilentPaymentCode::new(REGTEST_CODE.to_string()).unwrap());
    let transaction = TxBuilder::new()
        .fee_absolute(Arc::new(Amount::from_sat(500)))
        .finish_and_sign_silent_payments(
            &wallet,
            vec![SilentPaymentRecipient {
                code: Arc::clone(&code),
                amount: Arc::new(Amount::from_sat(40_000)),
            }],
        )
        .unwrap();
    let transaction: BdkTransaction = transaction.as_ref().into();

    assert_eq!(transaction.input.len(), 1);
    let witness = &transaction.input[0].witness;
    assert_eq!(witness.len(), 1);
    let signature =
        bdk_wallet::bitcoin::taproot::Signature::from_slice(witness.nth(0).unwrap()).unwrap();
    assert!(funding_output.script_pubkey.is_p2tr());
    let funding_public_key =
        XOnlyPublicKey::from_slice(&funding_output.script_pubkey.as_bytes()[2..]).unwrap();
    let prevouts = [funding_output.clone()];
    let sighash = SighashCache::new(&transaction)
        .taproot_key_spend_signature_hash(
            0,
            &bdk_wallet::bitcoin::sighash::Prevouts::All(&prevouts),
            signature.sighash_type,
        )
        .unwrap();
    let message = bdk_wallet::bitcoin::secp256k1::Message::from(sighash);
    let secp = bdk_wallet::bitcoin::secp256k1::Secp256k1::new();
    secp.verify_schnorr(&signature.signature, &message, &funding_public_key)
        .unwrap();
    assert!(transaction
        .output
        .iter()
        .all(|output| output.script_pubkey != code.0.get_placeholder_p2tr_spk()));

    let scan_secret = PrivateKey::from_wif(SCAN_WIF).unwrap().inner;
    let spend_secret = PrivateKey::from_wif(SPEND_WIF).unwrap().inner;
    let scanner = Scanner::new(scan_secret, code.0.spend, BTreeMap::new());
    let found_outputs = scanner.scan_tx(&transaction, &[funding_output]).unwrap();

    assert_eq!(found_outputs.len(), 1);
    assert_eq!(found_outputs[0].amount, BdkAmount::from_sat(40_000));
    assert!(transaction
        .output
        .iter()
        .any(|output| output.script_pubkey == found_outputs[0].script_pubkey));

    let output_secret = spend_secret
        .add_tweak(&found_outputs[0].tweak.into())
        .unwrap();
    let output_public_key =
        XOnlyPublicKey::from_slice(&found_outputs[0].script_pubkey.as_bytes()[2..]).unwrap();
    assert_eq!(output_secret.x_only_public_key(&secp).0, output_public_key);
}

#[test]
fn finish_and_sign_silent_payments_rejects_p2sh_input() {
    const EXTERNAL_DESCRIPTOR: &str = "sh(wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/49h/1h/1h/0/*))";
    const INTERNAL_DESCRIPTOR: &str = "sh(wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/49h/1h/1h/1/*))";

    let wallet = Arc::new(
        Wallet::new(
            Arc::new(Descriptor::new(EXTERNAL_DESCRIPTOR.to_string(), NetworkKind::Test).unwrap()),
            Arc::new(Descriptor::new(INTERNAL_DESCRIPTOR.to_string(), NetworkKind::Test).unwrap()),
            Network::Regtest,
            Arc::new(Persister::new_in_memory().unwrap()),
            25,
        )
        .unwrap(),
    );
    let funding_script = wallet
        .reveal_next_address(KeychainKind::External)
        .address
        .script_pubkey()
        .0
        .clone();
    assert!(funding_script.is_p2sh());
    let funding_transaction = BdkTransaction {
        version: Version::TWO,
        lock_time: LockTime::ZERO,
        input: vec![BdkTxIn {
            previous_output: BdkOutPoint::new(BdkTxid::all_zeros(), 0),
            script_sig: Default::default(),
            sequence: Sequence::MAX,
            witness: Witness::default(),
        }],
        output: vec![BdkTxOut {
            value: BdkAmount::from_sat(100_000),
            script_pubkey: funding_script,
        }],
    };
    wallet.apply_unconfirmed_txs(vec![UnconfirmedTx {
        tx: Arc::new(Transaction::from(funding_transaction)),
        last_seen: 1,
    }]);

    let code = Arc::new(SilentPaymentCode::new(REGTEST_CODE.to_string()).unwrap());
    assert!(matches!(
        TxBuilder::new()
            .fee_absolute(Arc::new(Amount::from_sat(500)))
            .finish_and_sign_silent_payments(
                &wallet,
                vec![SilentPaymentRecipient {
                    code,
                    amount: Arc::new(Amount::from_sat(40_000)),
                }],
            ),
        Err(SilentPaymentSendError::P2shInputsUnsupported)
    ));
}

#[test]
fn finish_and_sign_silent_payments_rejects_unsupported_configuration() {
    const EXTERNAL_DESCRIPTOR: &str = "wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/1h/0/*)";
    const INTERNAL_DESCRIPTOR: &str = "wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/1h/1/*)";

    let wallet = Arc::new(
        Wallet::new(
            Arc::new(Descriptor::new(EXTERNAL_DESCRIPTOR.to_string(), NetworkKind::Test).unwrap()),
            Arc::new(Descriptor::new(INTERNAL_DESCRIPTOR.to_string(), NetworkKind::Test).unwrap()),
            Network::Regtest,
            Arc::new(Persister::new_in_memory().unwrap()),
            25,
        )
        .unwrap(),
    );
    let code = Arc::new(SilentPaymentCode::new(TSP_CODE.to_string()).unwrap());

    assert!(matches!(
        TxBuilder::new().finish_and_sign_silent_payments(
            &wallet,
            vec![SilentPaymentRecipient {
                code,
                amount: Arc::new(Amount::from_sat(40_000)),
            }],
        ),
        Err(SilentPaymentSendError::NetworkMismatch { .. })
    ));

    let code = Arc::new(SilentPaymentCode::new(REGTEST_CODE.to_string()).unwrap());
    let amount = Arc::new(Amount::from_sat(1));
    let recipients = (0..2324)
        .map(|_| SilentPaymentRecipient {
            code: Arc::clone(&code),
            amount: Arc::clone(&amount),
        })
        .collect();
    assert!(matches!(
        TxBuilder::new().finish_and_sign_silent_payments(&wallet, recipients),
        Err(SilentPaymentSendError::RecipientLimitExceeded)
    ));

    assert!(matches!(
        TxBuilder::new()
            .set_exact_sequence(Sequence::ENABLE_RBF_NO_LOCKTIME.to_consensus_u32())
            .finish_and_sign_silent_payments(
                &wallet,
                vec![SilentPaymentRecipient {
                    code: Arc::clone(&code),
                    amount: Arc::new(Amount::from_sat(40_000)),
                }],
            ),
        Err(SilentPaymentSendError::RbfUnsupported)
    ));

    assert!(matches!(
        TxBuilder::new()
            .only_witness_utxo()
            .finish_and_sign_silent_payments(
                &wallet,
                vec![SilentPaymentRecipient {
                    code,
                    amount: Arc::new(Amount::from_sat(40_000)),
                }],
            ),
        Err(SilentPaymentSendError::OnlyWitnessUtxoUnsupported)
    ));
}
