use crate::error::ElectrumClientError;

use bdk_electrum::electrum_client::Client as BdkBlockingClient;

pub struct ElectrumClient(BdkBlockingClient);

impl ElectrumClient {
    pub fn new(url: String) -> Result<Self, ElectrumClientError> {
        let client = BdkBlockingClient::new(url.as_str())?;
        Ok(Self(client))
    }
}