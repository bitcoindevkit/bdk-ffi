use crate::bitcoin::{Amount, BlockHash, Network, NetworkKind};
use crate::descriptor::Descriptor;
use crate::error::{LoadWithPersistError, PersistenceError};
use crate::signer::SignersContainer;
use crate::store::{Persistence, Persister};
use crate::tx_builder::TxBuilder;
use crate::types::{ChangeSet, Update};
use crate::wallet::{CreateParams, LoadParams, Wallet};

use bdk_wallet::bitcoin::Amount as BdkAmount;
use bdk_wallet::bitcoin::Transaction as BdkTransaction;
use bdk_wallet::bitcoin::{absolute, transaction, TxOut as BdkTxOut};
use bdk_wallet::KeychainKind;

use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{mpsc, Arc, Mutex, Weak};
use std::thread;
use std::time::Duration;

const EXTERNAL_DESCRIPTOR: &str = "wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/1h/0/*)";
const INTERNAL_DESCRIPTOR: &str = "wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/1h/1/*)";
const TWO_PATH_DESCRIPTOR: &str = "wpkh([9a6a2580/84'/1'/0']tpubDDnGNapGEY6AZAdQbfRJgMg9fvz8pUBrLwvyvUqEgcUfgzM6zc2eVK4vY9x9L5FJWdX8WumXuLEDV5zDZnTfbn87vLe9XceCFwTu9so9Kks/<0;1>/*)";
const EXPECTED_FIRST_ADDRESS: &str = "tb1qhjys9wxlfykmte7ftryptx975uqgd6kcm6a7z4";

fn external_descriptor() -> Arc<Descriptor> {
    Arc::new(Descriptor::new(EXTERNAL_DESCRIPTOR.to_string(), NetworkKind::Test).unwrap())
}

fn internal_descriptor() -> Arc<Descriptor> {
    Arc::new(Descriptor::new(INTERNAL_DESCRIPTOR.to_string(), NetworkKind::Test).unwrap())
}

fn two_path_descriptor() -> Arc<Descriptor> {
    Arc::new(Descriptor::new(TWO_PATH_DESCRIPTOR.to_string(), NetworkKind::Test).unwrap())
}

fn custom_genesis_hash() -> Arc<BlockHash> {
    Arc::new(
        BlockHash::from_string(
            "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
        )
        .unwrap(),
    )
}

fn build_wallet() -> Wallet {
    Wallet::new(
        external_descriptor(),
        internal_descriptor(),
        Network::Signet,
        Arc::new(Persister::new_in_memory().unwrap()),
        25,
    )
    .unwrap()
}

fn funded_wallet() -> Wallet {
    let wallet = Wallet::new(
        external_descriptor(),
        internal_descriptor(),
        Network::Regtest,
        Arc::new(Persister::new_in_memory().unwrap()),
        25,
    )
    .unwrap();

    let address = wallet.reveal_next_address(KeychainKind::External).address;
    let funding_tx = BdkTransaction {
        version: transaction::Version::ONE,
        lock_time: absolute::LockTime::ZERO,
        input: vec![],
        output: vec![BdkTxOut {
            value: BdkAmount::from_sat(76_000),
            script_pubkey: address.script_pubkey().0.clone(),
        }],
    };
    let txid = funding_tx.compute_txid();
    let mut update = bdk_wallet::Update::default();
    update.last_active_indices.insert(KeychainKind::External, 0);
    update.tx_update.txs.push(Arc::new(funding_tx));
    update.tx_update.seen_ats.insert((txid, 1));

    wallet.apply_update(Arc::new(Update(update))).unwrap();

    wallet
}

#[test]
fn test_create_wallet_with_params_sets_custom_genesis_hash() {
    let genesis_hash = custom_genesis_hash();
    let params = CreateParams {
        genesis_hash: Some(Arc::clone(&genesis_hash)),
        lookahead: 25,
        use_spk_cache: true,
    };

    let wallet = Wallet::create_with_params(
        external_descriptor(),
        internal_descriptor(),
        Network::Signet,
        Arc::new(Persister::new_in_memory().unwrap()),
        params,
    )
    .unwrap();

    assert_eq!(wallet.network(), Network::Signet);
    assert_eq!(wallet.latest_checkpoint().hash, genesis_hash);
}

#[test]
fn test_load_wallet_with_params_checks_network_and_genesis_hash() {
    let persister = Arc::new(Persister::new_in_memory().unwrap());
    let genesis_hash = custom_genesis_hash();
    let create_params = CreateParams {
        genesis_hash: Some(Arc::clone(&genesis_hash)),
        lookahead: 25,
        use_spk_cache: true,
    };

    Wallet::create_with_params(
        external_descriptor(),
        internal_descriptor(),
        Network::Signet,
        Arc::clone(&persister),
        create_params,
    )
    .unwrap();

    let load_params = LoadParams {
        check_network: Some(Network::Signet),
        check_genesis_hash: Some(Arc::clone(&genesis_hash)),
        lookahead: 25,
        use_spk_cache: true,
    };
    let wallet = Wallet::load_with_params(
        external_descriptor(),
        internal_descriptor(),
        Arc::clone(&persister),
        load_params,
    )
    .unwrap();

    assert_eq!(wallet.network(), Network::Signet);
    assert_eq!(wallet.latest_checkpoint().hash, genesis_hash);

    let mismatched_params = LoadParams {
        check_network: Some(Network::Bitcoin),
        check_genesis_hash: Some(custom_genesis_hash()),
        lookahead: 25,
        use_spk_cache: true,
    };
    let error = match Wallet::load_with_params(
        external_descriptor(),
        internal_descriptor(),
        persister,
        mismatched_params,
    ) {
        Ok(_) => panic!("loading with mismatched network should fail"),
        Err(error) => error,
    };

    match error {
        LoadWithPersistError::InvalidChangeSet { error_message } => {
            assert!(error_message.contains("Network mismatch"));
        }
        error => panic!("expected InvalidChangeSet error, got {:?}", error),
    }
}

#[test]
fn test_create_wallet() {
    let wallet = build_wallet();

    assert_eq!(wallet.network(), Network::Signet);
    assert_eq!(wallet.balance().total.to_sat(), 0u64);
    assert!(wallet.list_unspent().is_empty());
    assert_eq!(wallet.derivation_index(KeychainKind::External), None);
    assert_eq!(wallet.next_derivation_index(KeychainKind::External), 0);
}

#[test]
fn test_keychains() {
    let wallet = build_wallet();

    let keychains = wallet.keychains();

    assert_eq!(keychains.len(), 2);

    let external = keychains
        .iter()
        .find(|keychain| keychain.keychain == KeychainKind::External)
        .unwrap();
    let internal = keychains
        .iter()
        .find(|keychain| keychain.keychain == KeychainKind::Internal)
        .unwrap();
    let external_public_descriptor = external.public_descriptor.to_string();
    let internal_public_descriptor = internal.public_descriptor.to_string();

    assert_eq!(
        external_public_descriptor,
        wallet.public_descriptor(KeychainKind::External)
    );
    assert_eq!(
        internal_public_descriptor,
        wallet.public_descriptor(KeychainKind::Internal)
    );
    assert!(external.public_descriptor.has_wildcard());
    assert!(internal.public_descriptor.has_wildcard());
    assert!(!external_public_descriptor.contains("tprv"));
    assert!(!internal_public_descriptor.contains("tprv"));
}

#[test]
fn test_reveal_next_address() {
    let wallet = build_wallet();

    let address_info = wallet.reveal_next_address(KeychainKind::External);

    assert_eq!(address_info.index, 0);
    assert_eq!(address_info.keychain, KeychainKind::External);
    assert_eq!(address_info.address.to_string(), EXPECTED_FIRST_ADDRESS);
}

#[test]
fn test_signers_container_from_descriptor() {
    let secret_descriptor = external_descriptor();
    let public_descriptor = secret_descriptor.as_public();
    let secret_signers = SignersContainer::from_descriptor(Arc::clone(&secret_descriptor));
    let public_signers = SignersContainer::from_descriptor(Arc::clone(&public_descriptor));

    assert_eq!(public_descriptor.to_string(), secret_descriptor.to_string());
    assert_eq!(
        public_descriptor.to_string_with_secret(),
        secret_descriptor.to_string()
    );
    assert!(!secret_signers.is_empty());
    assert_eq!(secret_signers.len(), 1);
    assert!(public_signers.is_empty());
    assert_eq!(public_signers.len(), 0);
}

#[test]
fn test_sign_with_signers() {
    let wallet = Arc::new(funded_wallet());
    let recipient_script = wallet
        .next_unused_address(KeychainKind::External)
        .address
        .script_pubkey();
    let psbt = TxBuilder::new()
        .add_recipient(&recipient_script, Arc::new(Amount::from_sat(10_000)))
        .finish(&wallet)
        .unwrap();
    let signers = vec![Arc::new(SignersContainer::from_descriptor(
        external_descriptor(),
    ))];

    let finalized = wallet
        .sign_with_signers(Arc::clone(&psbt), signers, None)
        .unwrap();
    let signed_tx = psbt.extract_tx().unwrap();

    assert!(finalized);
    assert!(!signed_tx.input()[0].witness.is_empty());
}

#[test]
fn test_sign_with_signers_for_public_wallet() {
    let external_signer_descriptor = external_descriptor();
    let external_public_descriptor = external_signer_descriptor.as_public();
    let internal_public_descriptor = internal_descriptor().as_public();
    let wallet = Arc::new(
        Wallet::new(
            Arc::clone(&external_public_descriptor),
            internal_public_descriptor,
            Network::Regtest,
            Arc::new(Persister::new_in_memory().unwrap()),
            25,
        )
        .unwrap(),
    );

    let address = wallet.reveal_next_address(KeychainKind::External).address;
    let funding_tx = BdkTransaction {
        version: transaction::Version::ONE,
        lock_time: absolute::LockTime::ZERO,
        input: vec![],
        output: vec![BdkTxOut {
            value: BdkAmount::from_sat(76_000),
            script_pubkey: address.script_pubkey().0.clone(),
        }],
    };
    let txid = funding_tx.compute_txid();
    let mut update = bdk_wallet::Update::default();
    update.last_active_indices.insert(KeychainKind::External, 0);
    update.tx_update.txs.push(Arc::new(funding_tx));
    update.tx_update.seen_ats.insert((txid, 1));
    wallet.apply_update(Arc::new(Update(update))).unwrap();

    let recipient_script = wallet
        .next_unused_address(KeychainKind::External)
        .address
        .script_pubkey();
    let psbt = TxBuilder::new()
        .add_recipient(&recipient_script, Arc::new(Amount::from_sat(10_000)))
        .finish(&wallet)
        .unwrap();
    let signers = vec![Arc::new(SignersContainer::from_descriptor_with_context(
        external_signer_descriptor,
        external_public_descriptor,
    ))];

    let finalized = wallet
        .sign_with_signers(Arc::clone(&psbt), signers, None)
        .unwrap();
    let signed_tx = psbt.extract_tx().unwrap();

    assert!(finalized);
    assert!(!signed_tx.input()[0].witness.is_empty());
}

#[test]
fn test_create_single_wallet() {
    let wallet = Wallet::create_single(
        external_descriptor(),
        Network::Signet,
        Arc::new(Persister::new_in_memory().unwrap()),
        25,
    )
    .unwrap();

    assert_eq!(wallet.derivation_index(KeychainKind::External), None);

    let keychains = wallet.keychains();

    assert_eq!(keychains.len(), 1);
    assert_eq!(keychains[0].keychain, KeychainKind::External);
    let public_descriptor = keychains[0].public_descriptor.to_string();
    assert_eq!(
        public_descriptor,
        wallet.public_descriptor(KeychainKind::External)
    );
    assert!(keychains[0].public_descriptor.has_wildcard());
    assert!(!public_descriptor.contains("tprv"));

    let address_info = wallet.reveal_next_address(KeychainKind::External);

    assert_eq!(address_info.index, 0);
    assert_eq!(address_info.keychain, KeychainKind::External);
    assert_eq!(address_info.address.to_string(), EXPECTED_FIRST_ADDRESS);
    assert_eq!(wallet.derivation_index(KeychainKind::External), Some(0));
    assert_eq!(wallet.next_derivation_index(KeychainKind::External), 1);
}

#[test]
fn test_create_two_path_wallet() {
    let wallet = Wallet::create_from_two_path_descriptor(
        two_path_descriptor(),
        Network::Signet,
        Arc::new(Persister::new_in_memory().unwrap()),
        25,
    )
    .unwrap();

    assert_eq!(wallet.derivation_index(KeychainKind::External), None);
    assert_eq!(wallet.derivation_index(KeychainKind::Internal), None);

    let external_address = wallet.reveal_next_address(KeychainKind::External);
    let internal_address = wallet.reveal_next_address(KeychainKind::Internal);

    assert_eq!(external_address.index, 0);
    assert_eq!(external_address.keychain, KeychainKind::External);
    assert_eq!(internal_address.index, 0);
    assert_eq!(internal_address.keychain, KeychainKind::Internal);
    assert_ne!(
        external_address.address.to_string(),
        internal_address.address.to_string()
    );
    assert_eq!(wallet.derivation_index(KeychainKind::External), Some(0));
    assert_eq!(wallet.derivation_index(KeychainKind::Internal), Some(0));
}

#[test]
fn test_load_from_two_path_descriptor() {
    let persister = Arc::new(Persister::new_in_memory().unwrap());
    let wallet = Wallet::create_from_two_path_descriptor(
        two_path_descriptor(),
        Network::Signet,
        Arc::clone(&persister),
        25,
    )
    .unwrap();

    wallet.reveal_next_address(KeychainKind::External);
    wallet.reveal_next_address(KeychainKind::Internal);
    assert!(wallet.persist(Arc::clone(&persister)).unwrap());

    let loaded_wallet =
        Wallet::load_from_two_path_descriptor(two_path_descriptor(), Arc::clone(&persister), 25)
            .unwrap();

    assert_eq!(loaded_wallet.network(), Network::Signet);
    assert_eq!(
        loaded_wallet.derivation_index(KeychainKind::External),
        Some(0)
    );
    assert_eq!(
        loaded_wallet.derivation_index(KeychainKind::Internal),
        Some(0)
    );
    assert_eq!(
        loaded_wallet.next_derivation_index(KeychainKind::External),
        1
    );
    assert_eq!(
        loaded_wallet.next_derivation_index(KeychainKind::Internal),
        1
    );
}

#[test]
fn test_load_from_two_path_descriptor_with_params() {
    let persister = Arc::new(Persister::new_in_memory().unwrap());
    Wallet::create_from_two_path_descriptor(
        two_path_descriptor(),
        Network::Signet,
        Arc::clone(&persister),
        25,
    )
    .unwrap();

    let params = LoadParams {
        check_network: Some(Network::Bitcoin),
        check_genesis_hash: None,
        lookahead: 25,
        use_spk_cache: false,
    };
    let error = match Wallet::load_from_two_path_descriptor_with_params(
        two_path_descriptor(),
        persister,
        params,
    ) {
        Ok(_) => panic!("loading with mismatched network should fail"),
        Err(error) => error,
    };

    match error {
        LoadWithPersistError::InvalidChangeSet { error_message } => {
            assert!(error_message.contains("Network mismatch"));
        }
        error => panic!("expected InvalidChangeSet error, got {:?}", error),
    }
}

#[test]
fn test_custom_persistence_callback_can_read_same_wallet() {
    struct ReentrantReadPersistence {
        wallet: Arc<Wallet>,
        reads: AtomicUsize,
    }

    impl Persistence for ReentrantReadPersistence {
        fn initialize(&self) -> Result<Arc<ChangeSet>, PersistenceError> {
            Ok(Arc::new(ChangeSet::new()))
        }

        fn persist(&self, _changeset: Arc<ChangeSet>) -> Result<(), PersistenceError> {
            let _ = self.wallet.balance();
            self.reads.fetch_add(1, Ordering::Relaxed);
            Ok(())
        }
    }

    let wallet = Arc::new(build_wallet());
    wallet.reveal_next_address(KeychainKind::External);
    let persistence = Arc::new(ReentrantReadPersistence {
        wallet: Arc::clone(&wallet),
        reads: AtomicUsize::new(0),
    });
    let persister = Arc::new(Persister::custom(persistence.clone()));
    let (sender, receiver) = mpsc::channel();
    let wallet_for_thread = Arc::clone(&wallet);
    let handle = thread::spawn(move || {
        sender.send(wallet_for_thread.persist(persister)).unwrap();
    });

    let persisted = receiver
        .recv_timeout(Duration::from_secs(2))
        .expect("reentrant persistence callback should not deadlock")
        .unwrap();
    handle.join().unwrap();

    assert!(persisted);
    assert_eq!(persistence.reads.load(Ordering::Relaxed), 1);
    assert!(wallet.staged().is_none());
}

#[test]
fn test_custom_persistence_callback_mutation_remains_staged() {
    struct ReentrantMutationPersistence {
        wallet: Arc<Wallet>,
        calls: AtomicUsize,
        persisted: Mutex<Vec<bdk_wallet::ChangeSet>>,
    }

    impl Persistence for ReentrantMutationPersistence {
        fn initialize(&self) -> Result<Arc<ChangeSet>, PersistenceError> {
            Ok(Arc::new(ChangeSet::new()))
        }

        fn persist(&self, changeset: Arc<ChangeSet>) -> Result<(), PersistenceError> {
            self.persisted
                .lock()
                .unwrap()
                .push(changeset.as_ref().clone().into());
            if self.calls.fetch_add(1, Ordering::Relaxed) == 0 {
                self.wallet.reveal_next_address(KeychainKind::External);
            }
            Ok(())
        }
    }

    let wallet = Arc::new(build_wallet());
    wallet.reveal_next_address(KeychainKind::External);
    let persistence = Arc::new(ReentrantMutationPersistence {
        wallet: Arc::clone(&wallet),
        calls: AtomicUsize::new(0),
        persisted: Mutex::new(Vec::new()),
    });
    let persister = Arc::new(Persister::custom(persistence.clone()));
    let (sender, receiver) = mpsc::channel();
    let wallet_for_thread = Arc::clone(&wallet);
    let persister_for_thread = Arc::clone(&persister);
    let handle = thread::spawn(move || {
        sender
            .send(wallet_for_thread.persist(persister_for_thread))
            .unwrap();
    });

    let first_persisted = receiver
        .recv_timeout(Duration::from_secs(2))
        .expect("reentrant wallet mutation should not deadlock")
        .unwrap();
    handle.join().unwrap();

    assert!(first_persisted);
    assert_eq!(wallet.derivation_index(KeychainKind::External), Some(1));
    let staged: bdk_wallet::ChangeSet = wallet.staged().unwrap().as_ref().clone().into();
    assert_eq!(
        staged.indexer.last_revealed.values().copied().max(),
        Some(1)
    );

    assert!(wallet.persist(Arc::clone(&persister)).unwrap());
    assert!(!wallet.persist(persister).unwrap());

    let persisted = persistence.persisted.lock().unwrap();
    let persisted_indexes: Vec<Option<u32>> = persisted
        .iter()
        .map(|changeset| changeset.indexer.last_revealed.values().copied().max())
        .collect();
    assert_eq!(persisted_indexes, vec![Some(0), Some(1)]);
    assert!(wallet.staged().is_none());
}

#[test]
fn test_custom_persistence_error_retains_reentrant_mutation() {
    struct FailAfterMutationPersistence {
        wallet: Arc<Wallet>,
        should_fail: AtomicBool,
        persisted: Mutex<Vec<bdk_wallet::ChangeSet>>,
    }

    impl Persistence for FailAfterMutationPersistence {
        fn initialize(&self) -> Result<Arc<ChangeSet>, PersistenceError> {
            Ok(Arc::new(ChangeSet::new()))
        }

        fn persist(&self, changeset: Arc<ChangeSet>) -> Result<(), PersistenceError> {
            self.persisted
                .lock()
                .unwrap()
                .push(changeset.as_ref().clone().into());
            if self.should_fail.swap(false, Ordering::Relaxed) {
                self.wallet.reveal_next_address(KeychainKind::External);
                return Err(PersistenceError::Reason {
                    error_message: "write failed".to_string(),
                });
            }
            Ok(())
        }
    }

    let wallet = Arc::new(build_wallet());
    wallet.reveal_next_address(KeychainKind::External);
    let persistence = Arc::new(FailAfterMutationPersistence {
        wallet: Arc::clone(&wallet),
        should_fail: AtomicBool::new(true),
        persisted: Mutex::new(Vec::new()),
    });
    let persister = Arc::new(Persister::custom(persistence.clone()));

    let error = wallet.persist(Arc::clone(&persister)).unwrap_err();
    assert!(error.to_string().contains("write failed"));
    let staged: bdk_wallet::ChangeSet = wallet.staged().unwrap().as_ref().clone().into();
    assert_eq!(
        staged.indexer.last_revealed.values().copied().max(),
        Some(1)
    );

    assert!(wallet.persist(persister).unwrap());
    let persisted = persistence.persisted.lock().unwrap();
    let persisted_indexes: Vec<Option<u32>> = persisted
        .iter()
        .map(|changeset| changeset.indexer.last_revealed.values().copied().max())
        .collect();
    assert_eq!(persisted_indexes, vec![Some(0), Some(1)]);
    assert!(wallet.staged().is_none());
}

#[test]
fn test_nested_persist_on_same_wallet_fails_fast() {
    struct CountingPersistence(AtomicUsize);

    impl Persistence for CountingPersistence {
        fn initialize(&self) -> Result<Arc<ChangeSet>, PersistenceError> {
            Ok(Arc::new(ChangeSet::new()))
        }

        fn persist(&self, _changeset: Arc<ChangeSet>) -> Result<(), PersistenceError> {
            self.0.fetch_add(1, Ordering::Relaxed);
            Ok(())
        }
    }

    struct NestedWalletPersistence {
        wallet: Arc<Wallet>,
        nested_persister: Arc<Persister>,
        nested_result: Mutex<Option<Result<bool, PersistenceError>>>,
    }

    impl Persistence for NestedWalletPersistence {
        fn initialize(&self) -> Result<Arc<ChangeSet>, PersistenceError> {
            Ok(Arc::new(ChangeSet::new()))
        }

        fn persist(&self, _changeset: Arc<ChangeSet>) -> Result<(), PersistenceError> {
            let result = self.wallet.persist(Arc::clone(&self.nested_persister));
            *self.nested_result.lock().unwrap() = Some(result);
            Ok(())
        }
    }

    let wallet = Arc::new(build_wallet());
    wallet.reveal_next_address(KeychainKind::External);
    let nested_persistence = Arc::new(CountingPersistence(AtomicUsize::new(0)));
    let nested_persister = Arc::new(Persister::custom(nested_persistence.clone()));
    let persistence = Arc::new(NestedWalletPersistence {
        wallet: Arc::clone(&wallet),
        nested_persister,
        nested_result: Mutex::new(None),
    });
    let persister = Arc::new(Persister::custom(persistence.clone()));
    let (sender, receiver) = mpsc::channel();
    let wallet_for_thread = Arc::clone(&wallet);
    let handle = thread::spawn(move || {
        sender.send(wallet_for_thread.persist(persister)).unwrap();
    });

    let persisted = receiver
        .recv_timeout(Duration::from_secs(2))
        .expect("nested wallet persistence should fail instead of deadlocking")
        .unwrap();
    handle.join().unwrap();

    assert!(persisted);
    match persistence.nested_result.lock().unwrap().take().unwrap() {
        Err(PersistenceError::Reason { error_message }) => {
            assert_eq!(
                error_message,
                "wallet persistence operation already in progress"
            );
        }
        result => panic!("expected nested persistence to fail, got {:?}", result),
    }
    assert_eq!(nested_persistence.0.load(Ordering::Relaxed), 0);
    assert!(wallet.staged().is_none());
}

#[test]
fn test_reentrant_persist_with_same_persister_fails_fast() {
    struct ReentrantPersisterPersistence {
        nested_wallet: Arc<Wallet>,
        persister: Mutex<Option<Weak<Persister>>>,
        calls: AtomicUsize,
        nested_result: Mutex<Option<Result<bool, PersistenceError>>>,
    }

    impl Persistence for ReentrantPersisterPersistence {
        fn initialize(&self) -> Result<Arc<ChangeSet>, PersistenceError> {
            Ok(Arc::new(ChangeSet::new()))
        }

        fn persist(&self, _changeset: Arc<ChangeSet>) -> Result<(), PersistenceError> {
            if self.calls.fetch_add(1, Ordering::Relaxed) == 0 {
                let persister = self
                    .persister
                    .lock()
                    .unwrap()
                    .as_ref()
                    .unwrap()
                    .upgrade()
                    .unwrap();
                let result = self.nested_wallet.persist(persister);
                *self.nested_result.lock().unwrap() = Some(result);
            }
            Ok(())
        }
    }

    let wallet = Arc::new(build_wallet());
    let nested_wallet = Arc::new(build_wallet());
    wallet.reveal_next_address(KeychainKind::External);
    nested_wallet.reveal_next_address(KeychainKind::External);
    let persistence = Arc::new(ReentrantPersisterPersistence {
        nested_wallet: Arc::clone(&nested_wallet),
        persister: Mutex::new(None),
        calls: AtomicUsize::new(0),
        nested_result: Mutex::new(None),
    });
    let persister = Arc::new(Persister::custom(persistence.clone()));
    *persistence.persister.lock().unwrap() = Some(Arc::downgrade(&persister));
    let (sender, receiver) = mpsc::channel();
    let wallet_for_thread = Arc::clone(&wallet);
    let persister_for_thread = Arc::clone(&persister);
    let handle = thread::spawn(move || {
        sender
            .send(wallet_for_thread.persist(persister_for_thread))
            .unwrap();
    });

    let persisted = receiver
        .recv_timeout(Duration::from_secs(2))
        .expect("reentrant persister use should fail instead of deadlocking")
        .unwrap();
    handle.join().unwrap();

    assert!(persisted);
    match persistence.nested_result.lock().unwrap().take().unwrap() {
        Err(PersistenceError::Reason { error_message }) => {
            assert_eq!(
                error_message,
                "custom persistence operation already in progress"
            );
        }
        result => panic!("expected reentrant persister use to fail, got {:?}", result),
    }
    assert!(nested_wallet.staged().is_some());
    assert!(nested_wallet.persist(Arc::clone(&persister)).unwrap());
    assert_eq!(persistence.calls.load(Ordering::Relaxed), 2);
    assert!(nested_wallet.staged().is_none());
}

#[test]
fn test_custom_persister_guards_complete_wallet_creation() {
    struct ReentrantCreatePersistence {
        persister: Mutex<Option<Weak<Persister>>>,
        attempted: AtomicBool,
        nested_result: Mutex<Option<Result<(), String>>>,
    }

    impl Persistence for ReentrantCreatePersistence {
        fn initialize(&self) -> Result<Arc<ChangeSet>, PersistenceError> {
            Ok(Arc::new(ChangeSet::new()))
        }

        fn persist(&self, _changeset: Arc<ChangeSet>) -> Result<(), PersistenceError> {
            if !self.attempted.swap(true, Ordering::Relaxed) {
                let persister = self
                    .persister
                    .lock()
                    .unwrap()
                    .as_ref()
                    .unwrap()
                    .upgrade()
                    .unwrap();
                let result = Wallet::new(
                    external_descriptor(),
                    internal_descriptor(),
                    Network::Signet,
                    persister,
                    25,
                )
                .map(|_| ())
                .map_err(|error| error.to_string());
                *self.nested_result.lock().unwrap() = Some(result);
            }
            Ok(())
        }
    }

    let persistence = Arc::new(ReentrantCreatePersistence {
        persister: Mutex::new(None),
        attempted: AtomicBool::new(false),
        nested_result: Mutex::new(None),
    });
    let persister = Arc::new(Persister::custom(persistence.clone()));
    *persistence.persister.lock().unwrap() = Some(Arc::downgrade(&persister));
    let (sender, receiver) = mpsc::channel();
    let handle = thread::spawn(move || {
        let result = Wallet::new(
            external_descriptor(),
            internal_descriptor(),
            Network::Signet,
            persister,
            25,
        )
        .map(|_| ())
        .map_err(|error| error.to_string());
        sender.send(result).unwrap();
    });

    receiver
        .recv_timeout(Duration::from_secs(2))
        .expect("reentrant wallet creation should fail instead of deadlocking")
        .unwrap();
    handle.join().unwrap();

    let nested_error = persistence
        .nested_result
        .lock()
        .unwrap()
        .take()
        .unwrap()
        .unwrap_err();
    assert!(nested_error.contains("custom persistence operation already in progress"));
}

#[test]
fn test_custom_persistence_panic_releases_operation_guards() {
    struct PanicOncePersistence(AtomicBool);

    impl Persistence for PanicOncePersistence {
        fn initialize(&self) -> Result<Arc<ChangeSet>, PersistenceError> {
            Ok(Arc::new(ChangeSet::new()))
        }

        fn persist(&self, _changeset: Arc<ChangeSet>) -> Result<(), PersistenceError> {
            if self.0.swap(false, Ordering::Relaxed) {
                panic!("persistence callback panicked");
            }
            Ok(())
        }
    }

    let wallet = Arc::new(build_wallet());
    wallet.reveal_next_address(KeychainKind::External);
    let persister = Arc::new(Persister::custom(Arc::new(PanicOncePersistence(
        AtomicBool::new(true),
    ))));

    let panic_result = catch_unwind(AssertUnwindSafe(|| wallet.persist(Arc::clone(&persister))));
    assert!(panic_result.is_err());
    assert!(wallet.staged().is_some());

    assert!(wallet.persist(persister).unwrap());
    assert!(wallet.staged().is_none());
}
