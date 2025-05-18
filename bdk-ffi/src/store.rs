use crate::error::SqliteError;

use bdk_wallet::rusqlite::Connection as BdkConnection;

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
    pub fn new(path: String) -> Result<Self, SqliteError> {
        let connection = BdkConnection::open(path)?;
        Ok(Self(Mutex::new(connection)))
    }

    /// Open a new connection to an in-memory SQLite database.
    #[uniffi::constructor]
    pub fn new_in_memory() -> Result<Self, SqliteError> {
        let connection = BdkConnection::open_in_memory()?;
        Ok(Self(Mutex::new(connection)))
    }
}

impl Connection {
    pub(crate) fn get_store(&self) -> MutexGuard<BdkConnection> {
        self.0.lock().expect("must lock")
    }
}
