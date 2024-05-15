use crate::error::ElectrumClientError;
use crate::types::FullScanRequest;
use crate::wallet::Update;

use bdk::chain::spk_client::FullScanRequest as BdkFullScanRequest;
use bdk::chain::spk_client::FullScanResult as BdkFullScanResult;
use bdk_electrum::{ElectrumExt, ElectrumFullScanResult};
use bdk::KeychainKind;
use bdk_electrum::electrum_client::Client as BdkBlockingClient;

use std::sync::Arc;

pub struct ElectrumClient(BdkBlockingClient);

impl ElectrumClient {
    pub fn new(url: String) -> Result<Self, ElectrumClientError> {
        let client = BdkBlockingClient::new(url.as_str())?;
        Ok(Self(client))
    }

    pub fn full_scan(
        &self,
        request: Arc<FullScanRequest>,
        stop_gap: u64,
        batch_size: u64,
        fetch_prev_txouts: bool,
    ) -> Result<Arc<Update>, ElectrumClientError> {
        // using option and take is not ideal but the only way to take full ownership of the request
        let request: BdkFullScanRequest<KeychainKind> = request
            .0
            .lock()
            .unwrap()
            .take()
            .ok_or(ElectrumClientError::RequestAlreadyConsumed)?;

        let electrum_result: ElectrumFullScanResult<KeychainKind> =
            self.0
                .full_scan(
                    request,
                    stop_gap as usize,
                    batch_size as usize,
                    fetch_prev_txouts,
                )?;
        let full_scan_result: BdkFullScanResult<KeychainKind> = electrum_result
            .with_confirmation_time_height_anchor(&self.0)?;

        let update = bdk::wallet::Update {
            last_active_indices: full_scan_result.last_active_indices,
            graph: full_scan_result.graph_update,
            chain: Some(full_scan_result.chain_update),
        };

        Ok(Arc::new(Update(update)))
    }
}