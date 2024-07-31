use bdk_wallet::rusqlite::Connection as BdkConnection;

use std::{borrow::BorrowMut, sync::Mutex};

use crate::error::SqliteError;

pub struct Connection(Mutex<BdkConnection>);

impl Connection {
    pub fn new(path: String) -> Result<Self, SqliteError> {
        let connection = BdkConnection::open(path)?;
        Ok(Self(Mutex::new(connection)))
    }

    pub(crate) fn get_store(&self) -> &mut BdkConnection {
        self.0.borrow_mut()
    }
}
