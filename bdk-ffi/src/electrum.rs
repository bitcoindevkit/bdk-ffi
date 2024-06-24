use crate::bitcoin::Transaction;
use crate::error::ElectrumError;
use crate::types::{FullScanRequest, SyncRequest};
use crate::wallet::Update;

use bdk_electrum::BdkElectrumClient as BdkBdkElectrumClient;
use bdk_electrum::{ElectrumFullScanResult, ElectrumSyncResult};
use bdk_wallet::bitcoin::Transaction as BdkTransaction;
use bdk_wallet::chain::spk_client::FullScanRequest as BdkFullScanRequest;
use bdk_wallet::chain::spk_client::FullScanResult as BdkFullScanResult;
use bdk_wallet::chain::spk_client::SyncRequest as BdkSyncRequest;
use bdk_wallet::chain::spk_client::SyncResult as BdkSyncResult;
use bdk_wallet::wallet::Update as BdkUpdate;
use bdk_wallet::KeychainKind;

use std::collections::BTreeMap;
use std::sync::Arc;

// NOTE: We are keeping our naming convention where the alias of the inner type is the Rust type
//       prefixed with `Bdk`. In this case the inner type is `BdkElectrumClient`, so the alias is
//       funnily enough named `BdkBdkElectrumClient`.
pub struct ElectrumClient(BdkBdkElectrumClient<bdk_electrum::electrum_client::Client>);

impl ElectrumClient {
    pub fn new(url: String) -> Result<Self, ElectrumError> {
        let inner_client: bdk_electrum::electrum_client::Client =
            bdk_electrum::electrum_client::Client::new(url.as_str())?;
        let client = BdkBdkElectrumClient::new(inner_client);
        Ok(Self(client))
    }

    pub fn full_scan(
        &self,
        request: Arc<FullScanRequest>,
        stop_gap: u64,
        batch_size: u64,
        fetch_prev_txouts: bool,
    ) -> Result<Arc<Update>, ElectrumError> {
        // using option and take is not ideal but the only way to take full ownership of the request
        let request: BdkFullScanRequest<KeychainKind> = request
            .0
            .lock()
            .unwrap()
            .take()
            .ok_or(ElectrumError::RequestAlreadyConsumed)?;

        let electrum_result: ElectrumFullScanResult<KeychainKind> = self.0.full_scan(
            request,
            stop_gap as usize,
            batch_size as usize,
            fetch_prev_txouts,
        )?;
        let full_scan_result: BdkFullScanResult<KeychainKind> =
            electrum_result.with_confirmation_time_height_anchor(&self.0)?;

        let update = BdkUpdate {
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
    ) -> Result<Arc<Update>, ElectrumError> {
        // using option and take is not ideal but the only way to take full ownership of the request
        let request: BdkSyncRequest = request
            .0
            .lock()
            .unwrap()
            .take()
            .ok_or(ElectrumError::RequestAlreadyConsumed)?;

        let electrum_result: ElectrumSyncResult =
            self.0
                .sync(request, batch_size as usize, fetch_prev_txouts)?;
        let sync_result: BdkSyncResult =
            electrum_result.with_confirmation_time_height_anchor(&self.0)?;

        let update = BdkUpdate {
            last_active_indices: BTreeMap::default(),
            graph: sync_result.graph_update,
            chain: Some(sync_result.chain_update),
        };

        Ok(Arc::new(Update(update)))
    }

    pub fn broadcast(&self, transaction: &Transaction) -> Result<String, ElectrumError> {
        let bdk_transaction: BdkTransaction = transaction.into();
        self.0
            .transaction_broadcast(&bdk_transaction)
            .map_err(ElectrumError::from)
            .map(|txid| txid.to_string())
    }
}
