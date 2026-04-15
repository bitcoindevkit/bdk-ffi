use crate::error::{PersistenceError, PreV1MigrationError};
use crate::types::ChangeSet;

use bdk_wallet::migration::{
    get_pre_v1_wallet_keychains as bdk_get_pre_v1_wallet_keychains,
    PreV1WalletKeychain as BdkPreV1WalletKeychain,
};
use bdk_wallet::{rusqlite::Connection as BdkConnection, WalletPersister};

use std::ops::DerefMut;
use std::sync::{Arc, Mutex};

/// Definition of a wallet persistence implementation.
#[uniffi::export(with_foreign)]
pub trait Persistence: Send + Sync {
    /// Initialize the total aggregate `ChangeSet` for the underlying wallet.
    fn initialize(&self) -> Result<Arc<ChangeSet>, PersistenceError>;

    /// Persist a `ChangeSet` to the total aggregate changeset of the wallet.
    fn persist(&self, changeset: Arc<ChangeSet>) -> Result<(), PersistenceError>;
}

pub(crate) enum PersistenceType {
    Custom(Arc<dyn Persistence>),
    Sql(Mutex<BdkConnection>),
}

/// `PreV1WalletKeychain` represents a structure that holds the keychain details
/// and metadata required for managing a wallet's keys.
#[derive(Debug, Clone, uniffi::Record)]
pub struct PreV1WalletKeychain {
    /// The name of the wallet keychains, "External" or "Internal".
    pub keychain: bdk_wallet::KeychainKind,
    /// The index of the last derived key in the wallet keychain.
    pub last_derivation_index: u32,
    /// Checksum of the keychain descriptor, it must match the corresponding
    /// post-1.0 bdk wallet descriptor checksum.
    pub checksum: String,
}

/// Wallet backend implementations.
#[derive(uniffi::Object)]
pub struct Persister {
    pub(crate) inner: Mutex<PersistenceType>,
}

#[uniffi::export]
impl Persister {
    /// Create a new Sqlite connection at the specified file path.
    #[uniffi::constructor]
    pub fn new_sqlite(path: String) -> Result<Self, PersistenceError> {
        let conn = BdkConnection::open(path)?;
        Ok(Self {
            inner: PersistenceType::Sql(conn.into()).into(),
        })
    }

    /// Create a new connection in memory.
    #[uniffi::constructor]
    pub fn new_in_memory() -> Result<Self, PersistenceError> {
        let conn = BdkConnection::open_in_memory()?;
        Ok(Self {
            inner: PersistenceType::Sql(conn.into()).into(),
        })
    }

    /// Use a native persistence layer.
    #[uniffi::constructor]
    pub fn custom(persistence: Arc<dyn Persistence>) -> Self {
        Self {
            inner: PersistenceType::Custom(persistence).into(),
        }
    }

    /// Retrieve keychain metadata from a pre-v1 BDK SQLite wallet database.
    pub fn get_pre_v1_wallet_keychains(
        &self,
    ) -> Result<Vec<PreV1WalletKeychain>, PreV1MigrationError> {
        let mut lock = self.inner.lock().unwrap();
        match lock.deref_mut() {
            PersistenceType::Sql(ref conn) => {
                let mut conn_lock = conn.lock().unwrap();
                bdk_get_pre_v1_wallet_keychains(conn_lock.deref_mut())
                    .map(|keychains| keychains.into_iter().map(Into::into).collect())
                    .map_err(Into::into)
            }
            PersistenceType::Custom(_) => Err(PreV1MigrationError::SqliteOnly),
        }
    }
}

impl From<BdkPreV1WalletKeychain> for PreV1WalletKeychain {
    fn from(value: BdkPreV1WalletKeychain) -> Self {
        Self {
            keychain: value.keychain,
            last_derivation_index: value.last_derivation_index,
            checksum: value.checksum,
        }
    }
}

impl WalletPersister for PersistenceType {
    type Error = PersistenceError;

    fn initialize(persister: &mut Self) -> Result<bdk_wallet::ChangeSet, Self::Error> {
        match persister {
            PersistenceType::Sql(ref conn) => {
                let mut lock = conn.lock().unwrap();
                let deref = lock.deref_mut();
                Ok(BdkConnection::initialize(deref)?)
            }
            PersistenceType::Custom(any) => any
                .initialize()
                .map(|changeset| changeset.as_ref().clone().into()),
        }
    }

    fn persist(persister: &mut Self, changeset: &bdk_wallet::ChangeSet) -> Result<(), Self::Error> {
        match persister {
            PersistenceType::Sql(ref conn) => {
                let mut lock = conn.lock().unwrap();
                let deref = lock.deref_mut();
                Ok(BdkConnection::persist(deref, changeset)?)
            }
            PersistenceType::Custom(any) => {
                let ffi_changeset: ChangeSet = changeset.clone().into();
                any.persist(Arc::new(ffi_changeset))
            }
        }
    }
}
