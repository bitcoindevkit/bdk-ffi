use crate::bitcoin::{Amount, BlockHash, FeeRate, Network, NetworkKind};
use crate::descriptor::Descriptor;
use crate::error::{CreateTxError, LoadWithPersistError, PersistenceError};
use crate::signer::SignersContainer;
use crate::store::{Persistence, Persister};
use crate::tx_builder::{BumpFeeTxBuilder, TxBuilder};
use crate::types::{ChangeSet, IndexerChangeSet, UnconfirmedTx, Update};
use crate::wallet::{CreateParams, LoadParams, Wallet};

use bdk_wallet::bitcoin::Amount as BdkAmount;
use bdk_wallet::bitcoin::ScriptBuf as BdkScriptBuf;
use bdk_wallet::bitcoin::Transaction as BdkTransaction;
use bdk_wallet::bitcoin::{absolute, transaction, TxOut as BdkTxOut};
use bdk_wallet::chain::DescriptorId as BdkDescriptorId;
use bdk_wallet::KeychainKind;

use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

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
fn test_tx_builder_invalid_current_height_returns_error() {
    let wallet = Arc::new(funded_wallet());
    let recipient_script = wallet
        .next_unused_address(KeychainKind::External)
        .address
        .script_pubkey();

    let result = TxBuilder::new()
        .add_recipient(&recipient_script, Arc::new(Amount::from_sat(10_000)))
        .current_height(500_000_000)
        .finish(&wallet);

    assert!(matches!(
        result,
        Err(CreateTxError::LockTimeConversionError)
    ));
}

#[test]
fn test_bump_fee_tx_builder_invalid_current_height_returns_error() {
    let wallet = Arc::new(funded_wallet());
    let recipient_script = wallet
        .next_unused_address(KeychainKind::External)
        .address
        .script_pubkey();
    let original_tx = TxBuilder::new()
        .add_recipient(&recipient_script, Arc::new(Amount::from_sat(10_000)))
        .finish(&wallet)
        .unwrap()
        .extract_tx()
        .unwrap();

    assert!(original_tx.is_explicitly_rbf());

    let txid = original_tx.compute_txid();
    wallet.apply_unconfirmed_txs(vec![UnconfirmedTx {
        tx: original_tx,
        last_seen: 2,
    }]);

    let result = BumpFeeTxBuilder::new(txid, Arc::new(FeeRate::from_sat_per_vb(2).unwrap()))
        .current_height(500_000_000)
        .finish(&wallet);

    assert!(matches!(
        result,
        Err(CreateTxError::LockTimeConversionError)
    ));
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

/// Number of script pubkeys derived ahead of the last revealed index in the spk cache tests.
const SPK_CACHE_LOOKAHEAD: u32 = 5;

/// A `Persistence` implementation that keeps the aggregate `ChangeSet` in memory, the same way a
/// foreign-language custom persister would.
#[derive(Default)]
struct InMemoryCustomPersistence {
    aggregate: Mutex<Option<Arc<ChangeSet>>>,
    last_persisted: Mutex<Option<Arc<ChangeSet>>>,
}

type BdkSpkCache = BTreeMap<BdkDescriptorId, BTreeMap<u32, BdkScriptBuf>>;

impl InMemoryCustomPersistence {
    fn spk_cache_of(changeset: &Mutex<Option<Arc<ChangeSet>>>) -> BdkSpkCache {
        let changeset: bdk_wallet::ChangeSet = changeset
            .lock()
            .unwrap()
            .as_ref()
            .expect("nothing was persisted")
            .as_ref()
            .clone()
            .into();
        changeset.indexer.spk_cache
    }

    /// The spk cache in the aggregate of everything persisted so far.
    fn aggregate_spk_cache(&self) -> BdkSpkCache {
        Self::spk_cache_of(&self.aggregate)
    }

    /// The spk cache in the most recent `persist` call only.
    fn last_persisted_spk_cache(&self) -> BdkSpkCache {
        Self::spk_cache_of(&self.last_persisted)
    }
}

impl Persistence for InMemoryCustomPersistence {
    fn initialize(&self) -> Result<Arc<ChangeSet>, PersistenceError> {
        Ok(self
            .aggregate
            .lock()
            .unwrap()
            .clone()
            .unwrap_or_else(|| Arc::new(ChangeSet::new())))
    }

    fn persist(&self, changeset: Arc<ChangeSet>) -> Result<(), PersistenceError> {
        let mut aggregate = self.aggregate.lock().unwrap();
        let merged = match aggregate.take() {
            Some(existing) => Arc::new(ChangeSet::from_merge(existing, Arc::clone(&changeset))),
            None => Arc::clone(&changeset),
        };
        *aggregate = Some(merged);
        *self.last_persisted.lock().unwrap() = Some(changeset);
        Ok(())
    }
}

fn spk_cache_load_params() -> LoadParams {
    LoadParams {
        check_network: None,
        check_genesis_hash: None,
        lookahead: SPK_CACHE_LOOKAHEAD,
        use_spk_cache: true,
    }
}

#[test]
fn test_custom_persistence_persists_spk_cache() {
    let persistence = Arc::new(InMemoryCustomPersistence::default());
    let persister = Arc::new(Persister::custom(persistence.clone()));
    let wallet = Wallet::create_with_params(
        external_descriptor(),
        internal_descriptor(),
        Network::Signet,
        Arc::clone(&persister),
        CreateParams {
            genesis_hash: None,
            lookahead: SPK_CACHE_LOOKAHEAD,
            use_spk_cache: true,
        },
    )
    .unwrap();

    // The index stages the script pubkeys it derives and hands them over on the next reveal.
    wallet.reveal_next_address(KeychainKind::External);
    assert!(wallet.persist(Arc::clone(&persister)).unwrap());

    let spk_cache = persistence.aggregate_spk_cache();
    let external_id = BdkDescriptorId(external_descriptor().descriptor_id().0);
    let internal_id = BdkDescriptorId(internal_descriptor().descriptor_id().0);
    // Revealing index 0 keeps `SPK_CACHE_LOOKAHEAD` more script pubkeys derived past it.
    let expected_lengths = [
        (KeychainKind::External, external_id, SPK_CACHE_LOOKAHEAD + 1),
        (KeychainKind::Internal, internal_id, SPK_CACHE_LOOKAHEAD),
    ];
    for (keychain, descriptor_id, expected_len) in expected_lengths {
        let cached = spk_cache
            .get(&descriptor_id)
            .unwrap_or_else(|| panic!("no spk cache persisted for {:?}", keychain));
        assert_eq!(cached.len() as u32, expected_len);
        for (index, script) in cached {
            let expected = wallet
                .peek_address(keychain, *index)
                .address
                .script_pubkey();
            assert_eq!(script.as_bytes(), expected.to_bytes().as_slice());
        }
    }
}

#[test]
fn test_custom_persistence_reloads_spk_cache_without_rederiving() {
    let persistence = Arc::new(InMemoryCustomPersistence::default());
    let persister = Arc::new(Persister::custom(persistence.clone()));
    let wallet = Wallet::create_with_params(
        external_descriptor(),
        internal_descriptor(),
        Network::Signet,
        Arc::clone(&persister),
        CreateParams {
            genesis_hash: None,
            lookahead: SPK_CACHE_LOOKAHEAD,
            use_spk_cache: true,
        },
    )
    .unwrap();
    wallet.reveal_next_address(KeychainKind::External);
    assert!(wallet.persist(Arc::clone(&persister)).unwrap());
    drop(wallet);

    let loaded = Wallet::load_with_params(
        external_descriptor(),
        internal_descriptor(),
        Arc::clone(&persister),
        spk_cache_load_params(),
    )
    .unwrap();
    let revealed = loaded.reveal_next_address(KeychainKind::External);
    assert!(loaded.persist(Arc::clone(&persister)).unwrap());

    // With the cache restored, only the one script pubkey newly derived past the lookahead window
    // is staged. Without it, every script pubkey would be derived again on load and staged here.
    let newly_derived_index = revealed.index + SPK_CACHE_LOOKAHEAD;
    let external_id = BdkDescriptorId(external_descriptor().descriptor_id().0);
    let staged: Vec<(BdkDescriptorId, u32)> = persistence
        .last_persisted_spk_cache()
        .into_iter()
        .flat_map(|(id, spks)| spks.into_keys().map(move |index| (id, index)))
        .collect();
    assert_eq!(staged, vec![(external_id, newly_derived_index)]);
}

/// Arbitrary derivation indices for the `IndexerChangeSet` round-trip test.
const ROUND_TRIP_LAST_REVEALED_INDEX: u32 = 3;
const ROUND_TRIP_CACHED_SPK_INDEX: u32 = 7;

#[test]
fn test_indexer_changeset_round_trips_spk_cache() {
    let descriptor_id = BdkDescriptorId(external_descriptor().descriptor_id().0);
    let script = build_wallet()
        .peek_address(KeychainKind::External, ROUND_TRIP_CACHED_SPK_INDEX)
        .address
        .script_pubkey()
        .0
        .clone();
    let original = bdk_wallet::chain::indexer::keychain_txout::ChangeSet {
        last_revealed: [(descriptor_id, ROUND_TRIP_LAST_REVEALED_INDEX)].into(),
        spk_cache: [(
            descriptor_id,
            [(ROUND_TRIP_CACHED_SPK_INDEX, script)].into(),
        )]
        .into(),
    };

    let ffi: IndexerChangeSet = original.clone().into();
    let round_tripped: bdk_wallet::chain::indexer::keychain_txout::ChangeSet = ffi.into();

    assert_eq!(round_tripped, original);
}
