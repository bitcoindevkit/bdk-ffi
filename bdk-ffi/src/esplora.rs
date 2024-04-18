use crate::error::EsploraError;
use crate::wallet::{Update, Wallet};

use crate::bitcoin::Transaction;
use bdk::bitcoin::Transaction as BdkTransaction;
use bdk::wallet::Update as BdkUpdate;
use bdk_esplora::esplora_client::{BlockingClient, Builder};
use bdk_esplora::EsploraExt;

use std::sync::Arc;

pub struct EsploraClient(BlockingClient);

impl EsploraClient {
    pub fn new(url: String) -> Self {
        let client = Builder::new(url.as_str()).build_blocking();
        Self(client)
    }

    // This is a temporary solution for scanning. The long-term solution involves not passing
    // the wallet to the client at all.
    pub fn full_scan(
        &self,
        wallet: Arc<Wallet>,
        stop_gap: u64,
        parallel_requests: u64,
    ) -> Result<Arc<Update>, EsploraError> {
        let wallet = wallet.get_wallet();

        let previous_tip = wallet.latest_checkpoint();
        let keychain_spks = wallet.all_unbounded_spk_iters().into_iter().collect();

        let (update_graph, last_active_indices) = self
            .0
            .full_scan(keychain_spks, stop_gap as usize, parallel_requests as usize)
            .map_err(|e| EsploraError::from(*e))?;

        let missing_heights = update_graph.missing_heights(wallet.local_chain());
        let chain_update = self
            .0
            .update_local_chain(previous_tip, missing_heights)
            .map_err(|e| EsploraError::from(*e))?;

        let update = BdkUpdate {
            last_active_indices,
            graph: update_graph,
            chain: Some(chain_update),
        };

        Ok(Arc::new(Update(update)))
    }

    // pub fn sync();

    pub fn broadcast(&self, transaction: &Transaction) -> Result<(), EsploraError> {
        let bdk_transaction: BdkTransaction = transaction.into();
        self.0
            .broadcast(&bdk_transaction)
            .map_err(EsploraError::from)
    }

    // pub fn estimate_fee();
}
