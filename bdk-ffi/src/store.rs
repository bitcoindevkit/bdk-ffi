use crate::error::PersistenceError;
use crate::types::ChangeSet;

use bdk_wallet::rusqlite::Connection as BdkConnection;
use bdk_wallet::WalletPersister;

use std::ops::DerefMut;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::MutexGuard;

/// A connection to a SQLite database.
#[derive(uniffi::Object)]
pub struct Connection(Mutex<BdkConnection>);

#[uniffi::export]
impl Connection {
    /// Open a new connection to a SQLite database. If a database does not exist at the path, one is
    /// created.
    #[uniffi::constructor]
    pub fn new(path: String) -> Result<Self, PersistenceError> {
        let connection = BdkConnection::open(path)?;
        Ok(Self(Mutex::new(connection)))
    }

    /// Open a new connection to an in-memory SQLite database.
    #[uniffi::constructor]
    pub fn new_in_memory() -> Result<Self, PersistenceError> {
        let connection = BdkConnection::open_in_memory()?;
        Ok(Self(Mutex::new(connection)))
    }
}

impl Connection {
    pub(crate) fn get_store(&self) -> MutexGuard<BdkConnection> {
        self.0.lock().expect("must lock")
    }
}

#[uniffi::export]
pub trait Persister: Send + Sync {
    fn initialize(&self) -> Result<ChangeSet, PersistenceError>;

    fn persist(&self, changeset: &ChangeSet) -> Result<(), PersistenceError>;
}

impl Persister for Connection {
    fn initialize(&self) -> Result<ChangeSet, PersistenceError> {
        let mut store = self.get_store();
        let mut db = store.deref_mut();
        let changeset_res = bdk_wallet::rusqlite::Connection::initialize(&mut db);
        let changeset = changeset_res.map_err(|e| PersistenceError::Reason {
            error_message: e.to_string(),
        })?;
        Ok(changeset.into())
    }

    fn persist(&self, changeset: &ChangeSet) -> Result<(), PersistenceError> {
        let mut store = self.get_store();
        let mut db = store.deref_mut();
        let changeset = changeset.clone().into();
        bdk_wallet::rusqlite::Connection::persist(&mut db, &changeset).map_err(|e| {
            PersistenceError::Reason {
                error_message: e.to_string(),
            }
        })
    }
}

pub(crate) struct AnyPersistence {
    inner: Arc<dyn Persister>,
}

impl AnyPersistence {
    pub(crate) fn new(persister: Arc<dyn Persister>) -> Self {
        Self { inner: persister }
    }
}

impl WalletPersister for AnyPersistence {
    type Error = PersistenceError;

    fn persist(persister: &mut Self, changeset: &bdk_wallet::ChangeSet) -> Result<(), Self::Error> {
        let changeset = changeset.clone().into();
        persister.inner.persist(&changeset)
    }

    fn initialize(persister: &mut Self) -> Result<bdk_wallet::ChangeSet, Self::Error> {
        persister
            .inner
            .initialize()
            .map(|changeset| changeset.into())
    }
}
