use crate::error::SqliteError;
use crate::types::ChangeSet;

use bdk_sqlite::rusqlite::Connection;
use bdk_sqlite::{Store as BdkSqliteStore, Store};
use bdk_wallet::chain::ConfirmationTimeHeightAnchor;
use bdk_wallet::KeychainKind;

use std::sync::{Mutex, MutexGuard};

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

    // pub fn read(&self) -> Result<Option<CombinedChangeSet<KeychainKind, ConfirmationTimeHeightAnchor>, Error> {
    //     self.0.read().map_err(SqliteError::from)
    // }

    pub fn write(&self, changeset: &ChangeSet) -> Result<(), SqliteError> {
        self.get_store()
            .write(&changeset.0)
            .map_err(SqliteError::from)
    }
}
