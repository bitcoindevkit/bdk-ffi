use crate::bitcoin::Transaction;
use crate::error::EsploraError;
use crate::types::{FullScanRequest, SyncRequest};
use crate::wallet::Update;
use std::collections::BTreeMap;

use bdk::bitcoin::Transaction as BdkTransaction;
use bdk::chain::spk_client::FullScanRequest as BdkFullScanRequest;
use bdk::chain::spk_client::FullScanResult as BdkFullScanResult;
use bdk::chain::spk_client::SyncRequest as BdkSyncRequest;
use bdk::chain::spk_client::SyncResult as BdkSyncResult;
use bdk::KeychainKind;
use bdk_esplora::esplora_client::{BlockingClient, Builder};
use bdk_esplora::EsploraExt;

use std::sync::Arc;

pub struct EsploraClient(BlockingClient);

impl EsploraClient {
    pub fn new(url: String) -> Self {
        let client = Builder::new(url.as_str()).build_blocking();
        Self(client)
    }

    pub fn full_scan(
        &self,
        request: Arc<FullScanRequest>,
        stop_gap: u64,
        parallel_requests: u64,
    ) -> Result<Arc<Update>, EsploraError> {
        // using option and take is not ideal but the only way to take full ownership of the request
        let request: BdkFullScanRequest<KeychainKind> = request
            .0
            .lock()
            .unwrap()
            .take()
            .ok_or(EsploraError::RequestAlreadyConsumed)?;

        let result: BdkFullScanResult<KeychainKind> =
            self.0
                .full_scan(request, stop_gap as usize, parallel_requests as usize)?;

        let update = bdk::wallet::Update {
            last_active_indices: result.last_active_indices,
            graph: result.graph_update,
            chain: Some(result.chain_update),
        };

        Ok(Arc::new(Update(update)))
    }

    pub fn sync(
        &self,
        request: Arc<SyncRequest>,
        parallel_requests: u64,
    ) -> Result<Arc<Update>, EsploraError> {
        // using option and take is not ideal but the only way to take full ownership of the request
        let request: BdkSyncRequest = request
            .0
            .lock()
            .unwrap()
            .take()
            .ok_or(EsploraError::RequestAlreadyConsumed)?;

        let result: BdkSyncResult = self.0.sync(request, parallel_requests as usize)?;

        let update = bdk::wallet::Update {
            last_active_indices: BTreeMap::default(),
            graph: result.graph_update,
            chain: Some(result.chain_update),
        };

        Ok(Arc::new(Update(update)))
    }

    pub fn broadcast(&self, transaction: &Transaction) -> Result<(), EsploraError> {
        let bdk_transaction: BdkTransaction = transaction.into();
        self.0
            .broadcast(&bdk_transaction)
            .map_err(EsploraError::from)
    }
}
