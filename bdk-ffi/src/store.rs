use crate::error::SqliteError;

use bdk_wallet::rusqlite::Connection as BdkConnection;

use std::sync::Mutex;
use std::sync::MutexGuard;

pub struct Connection(Mutex<BdkConnection>);

impl Connection {
    pub fn new(path: String) -> Result<Self, SqliteError> {
        let connection = BdkConnection::open(path)?;
        Ok(Self(Mutex::new(connection)))
    }

    pub fn new_in_memory() -> Result<Self, SqliteError> {
        let connection = BdkConnection::open_in_memory()?;
        Ok(Self(Mutex::new(connection)))
    }

    pub(crate) fn get_store(&self) -> MutexGuard<BdkConnection> {
        self.0.lock().expect("must lock")
    }
}
