use crate::error::{Alpha3Error, EsploraError};
use crate::wallet::Update;
use std::convert::TryInto;
//use std::path::Iter;
use std::ops::DerefMut;

use bdk::bitcoin::Transaction as BdkTransaction;
use bdk_esplora::esplora_client::{BlockingClient, Builder};
use bdk_esplora::EsploraExt;

use crate::bitcoin::Transaction;
use bdk::chain::spk_client::{FullScanResult as BdkFullScanResult, SyncResult as BdkSyncResult};
use bdk::KeychainKind;
use std::sync::Arc;

use crate::wallet::{FullScanRequest, SyncRequest};

pub struct EsploraClient(BlockingClient);

impl EsploraClient {
    pub fn new(url: String) -> Self {
        let client = Builder::new(url.as_str()).build_blocking().unwrap();
        Self(client)
    }

    // This is a temporary solution for scanning. The long-term solution involves not passing
    // the wallet to the client at all.
    pub fn full_scan(
        &self,
        request: Arc<FullScanRequest>, //wallet: Arc<Wallet>,
        stop_gap: u64,
        parallel_requests: u64,
    ) -> Result<Arc<Update>, EsploraError> {
        let result: BdkFullScanResult<KeychainKind> = self.0.full_scan(
            request.0.lock().unwrap().deref_mut(),
            stop_gap.try_into().unwrap(),
            parallel_requests.try_into().unwrap(),
        )?;

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
        let result: BdkSyncResult = self.0.sync(
            request.0.lock().unwrap().deref_mut(),
            parallel_requests.try_into().unwrap(),
        )?;

        let update = bdk::wallet::Update {
            graph: result.graph_update,
            chain: Some(result.chain_update),
            ..bdk::wallet::Update::default()
        };

        Ok(Arc::new(Update(update)))
    }

    pub fn broadcast(&self, transaction: &Transaction) -> Result<(), Alpha3Error> {
        let bdk_transaction: BdkTransaction = transaction.into();
        self.0
            .broadcast(&bdk_transaction)
            .map_err(|_| Alpha3Error::Generic)
    }

    // pub fn estimate_fee();
}
