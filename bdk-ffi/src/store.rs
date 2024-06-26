use crate::error::SqliteError;
use crate::types::ChangeSet;

use bdk_sqlite::rusqlite::Connection;
use bdk_sqlite::{Store as BdkSqliteStore, Store};
use bdk_wallet::chain::ConfirmationTimeHeightAnchor;
use bdk_wallet::KeychainKind;

use std::sync::{Arc, Mutex, MutexGuard};

pub struct SqliteStore {
    inner_mutex: Mutex<BdkSqliteStore<KeychainKind, ConfirmationTimeHeightAnchor>>,
}

impl SqliteStore {
    pub fn new(path: String) -> Result<Self, SqliteError> {
        let connection = Connection::open(path)?;
        let db = Store::new(connection)?;
        Ok(Self {
            inner_mutex: Mutex::new(db),
        })
    }

    pub(crate) fn get_store(
        &self,
    ) -> MutexGuard<BdkSqliteStore<KeychainKind, ConfirmationTimeHeightAnchor>> {
        self.inner_mutex.lock().expect("sqlite store")
    }

    pub fn write(&self, changeset: &ChangeSet) -> Result<(), SqliteError> {
        self.get_store()
            .write(&changeset.0)
            .map_err(SqliteError::from)
    }

    pub fn read(&self) -> Result<Option<Arc<ChangeSet>>, SqliteError> {
        self.get_store()
            .read()
            .map_err(SqliteError::from)
            .map(|optional_bdk_change_set| optional_bdk_change_set.map(ChangeSet::from))
            .map(|optional_change_set| optional_change_set.map(Arc::new))
    }
}
