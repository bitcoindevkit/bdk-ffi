use crate::error::Alpha3Error;
use crate::wallet::{Update, Wallet};
use std::convert::TryInto;

use bdk::bitcoin::{ScriptBuf, Transaction as BdkTransaction};
use bdk::wallet::Update as BdkUpdate;
use bdk_esplora::esplora_client::{BlockingClient, Builder};
use bdk_esplora::EsploraExt;

use crate::bitcoin::Transaction;
use bdk::chain::spk_client::{FullScanResult as BdkFullScanResult, FullScanRequest as BdkFullScanRequest, SyncResult};
use std::sync::Arc;
use bdk::KeychainKind;


pub struct FullScanRequest(pub BdkFullScanRequest<KeychainKind, dyn Iterator<Item = (u32, ScriptBuf)> + Send >);

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
        request: FullScanRequest,
        stop_gap: u64,
        parallel_requests: u64,
    ) -> Result<Arc<Update>, Alpha3Error> {
        // let wallet = wallet.get_wallet();

        // 1. get data required to do a wallet full_scan
        let request = request.0;//wallet.full_scan_request();

        // 2. full scan to discover wallet transactions
        let BdkFullScanResult {
            graph_update,
            chain_update,
            last_active_indices,
        } = self.0.full_scan(
            request,
            stop_gap.try_into().unwrap(),
            parallel_requests.try_into().unwrap(),
        )?;

        // 3. create wallet update
        let update = BdkUpdate {
            last_active_indices,
            graph: graph_update,
            chain: Some(chain_update),
        };

        Ok(Arc::new(Update(update)))
    }

    pub fn sync(
        &self,
        wallet: Arc<Wallet>,
        parallel_requests: u64,
    ) -> Result<Arc<Update>, Alpha3Error> {
        let wallet = wallet.get_wallet();

        // 1. get data required to do a wallet sync, if also syncing previously used addresses set unused_spks_only = false
        let request = wallet.sync_revealed_spks_request();

        // 2. sync unused wallet spks (addresses), unconfirmed tx, and utxos
        let SyncResult {
            graph_update,
            chain_update,
        } = self
            .0
            .sync(request, parallel_requests.try_into().unwrap())?;

        // 3. create wallet update
        let update = BdkUpdate {
            graph: graph_update,
            chain: Some(chain_update),
            ..BdkUpdate::default()
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
