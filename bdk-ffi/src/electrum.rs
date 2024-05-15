use crate::error::ElectrumClientError;
use crate::types::{FullScanRequest, SyncRequest};
use crate::wallet::Update;
use std::collections::BTreeMap;

use bdk::bitcoin::Transaction as BdkTransaction;
use bdk::chain::spk_client::FullScanRequest as BdkFullScanRequest;
use bdk::chain::spk_client::FullScanResult as BdkFullScanResult;
use bdk::chain::spk_client::SyncRequest as BdkSyncRequest;
use bdk::chain::spk_client::SyncResult as BdkSyncResult;
use bdk::KeychainKind;
use bdk_electrum::electrum_client::{Client as BdkBlockingClient, ElectrumApi};
use bdk_electrum::{ElectrumExt, ElectrumFullScanResult, ElectrumSyncResult};

use crate::bitcoin::Transaction;
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

        let electrum_result: ElectrumFullScanResult<KeychainKind> = self.0.full_scan(
            request,
            stop_gap as usize,
            batch_size as usize,
            fetch_prev_txouts,
        )?;
        let full_scan_result: BdkFullScanResult<KeychainKind> =
            electrum_result.with_confirmation_time_height_anchor(&self.0)?;

        let update = bdk::wallet::Update {
            last_active_indices: full_scan_result.last_active_indices,
            graph: full_scan_result.graph_update,
            chain: Some(full_scan_result.chain_update),
        };

        Ok(Arc::new(Update(update)))
    }

    pub fn sync(
        &self,
        request: Arc<SyncRequest>,
        batch_size: u64,
        fetch_prev_txouts: bool,
    ) -> Result<Arc<Update>, ElectrumClientError> {
        // using option and take is not ideal but the only way to take full ownership of the request
        let request: BdkSyncRequest = request
            .0
            .lock()
            .unwrap()
            .take()
            .ok_or(ElectrumClientError::RequestAlreadyConsumed)?;

        let electrum_result: ElectrumSyncResult =
            self.0
                .sync(request, batch_size as usize, fetch_prev_txouts)?;
        let sync_result: BdkSyncResult =
            electrum_result.with_confirmation_time_height_anchor(&self.0)?;

        let update = bdk::wallet::Update {
            last_active_indices: BTreeMap::default(),
            graph: sync_result.graph_update,
            chain: Some(sync_result.chain_update),
        };

        Ok(Arc::new(Update(update)))
    }

    pub fn broadcast(&self, transaction: &Transaction) -> Result<String, ElectrumClientError> {
        let bdk_transaction: BdkTransaction = transaction.into();
        self.0
            .transaction_broadcast(&bdk_transaction)
            .map_err(ElectrumClientError::from)
            .map(|txid| txid.to_string())
    }
}
