use crate::bitcoin::Address;
use crate::bitcoin::Block;
use crate::bitcoin::BlockHash;
use crate::bitcoin::Header;
use crate::bitcoin::Transaction;
use crate::bitcoin::Txid;
use crate::error::EsploraError;
use crate::types::KeychainKind;
use crate::types::Tx;
use crate::types::TxStatus;
use crate::types::Update;
use crate::types::{FullScanRequest, MerkleProof, OutputStatus, SyncRequest};

#[cfg(target_arch = "wasm32")]
use bdk_esplora::esplora_client::AsyncClient;
#[cfg(not(target_arch = "wasm32"))]
use bdk_esplora::esplora_client::BlockingClient;
use bdk_esplora::esplora_client::Builder;
#[cfg(target_arch = "wasm32")]
use bdk_esplora::EsploraAsyncExt;
#[cfg(not(target_arch = "wasm32"))]
use bdk_esplora::EsploraExt;
use bdk_wallet::bitcoin::Transaction as BdkTransaction;
use bdk_wallet::chain::spk_client::FullScanRequest as BdkFullScanRequest;
use bdk_wallet::chain::spk_client::FullScanResponse as BdkFullScanResponse;
use bdk_wallet::chain::spk_client::SyncRequest as BdkSyncRequest;
use bdk_wallet::chain::spk_client::SyncResponse as BdkSyncResponse;
use bdk_wallet::Update as BdkUpdate;

use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;

/// Wrapper around an esplora_client::BlockingClient which includes an internal in-memory transaction
/// cache to avoid re-fetching already downloaded transactions.
#[cfg(not(target_arch = "wasm32"))]
#[derive(uniffi::Object)]
pub struct EsploraClient(BlockingClient);

/// Wrapper around an esplora_client::AsyncClient which includes an internal in-memory transaction
/// cache to avoid re-fetching already downloaded transactions.
#[cfg(target_arch = "wasm32")]
#[derive(uniffi::Object)]
pub struct EsploraClient(AsyncClient<web_sleeper::WebSleeper>);

/// `Sleeper` implementation for wasm32 using the JavaScript `setTimeout` function.
#[cfg(target_arch = "wasm32")]
mod web_sleeper {
    use bdk_esplora::esplora_client::Sleeper;
    use futures_channel::oneshot::{channel, Receiver};
    use js_sys::{Function, Reflect};
    use std::future::Future;
    use std::pin::Pin;
    use std::task::{Context, Poll};
    use std::time::Duration;
    use wasm_bindgen::closure::Closure;
    use wasm_bindgen::{JsCast, JsValue};

    /// A oneshot receiver is used instead of a `JsFuture` because the `Sleep` future must be `Send`
    pub struct WebSleep(Receiver<()>);

    impl Future for WebSleep {
        type Output = ();

        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
            Pin::new(&mut self.0).poll(cx).map(|_| ())
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct WebSleeper;

    impl Sleeper for WebSleeper {
        type Sleep = WebSleep;

        fn sleep(dur: Duration) -> Self::Sleep {
            let (sender, receiver) = channel::<()>();
            let callback = Closure::once_into_js(move || {
                let _ = sender.send(());
            });
            let global = js_sys::global();
            let set_timeout: Function = Reflect::get(&global, &JsValue::from_str("setTimeout"))
                .expect("setTimeout is defined on the global object")
                .unchecked_into();
            set_timeout
                .call2(
                    &global,
                    &callback,
                    &JsValue::from_f64(dur.as_millis() as f64),
                )
                .expect("setTimeout accepts a callback and a delay");
            WebSleep(receiver)
        }
    }
}

fn take_full_scan_request(
    request: &FullScanRequest,
) -> Result<BdkFullScanRequest<KeychainKind>, EsploraError> {
    // using option and take is not ideal but the only way to take full ownership of the request
    request
        .0
        .lock()
        .unwrap()
        .take()
        .ok_or(EsploraError::RequestAlreadyConsumed)
}

fn take_sync_request(
    request: &SyncRequest,
) -> Result<BdkSyncRequest<(KeychainKind, u32)>, EsploraError> {
    // using option and take is not ideal but the only way to take full ownership of the request
    request
        .0
        .lock()
        .unwrap()
        .take()
        .ok_or(EsploraError::RequestAlreadyConsumed)
}

fn full_scan_update(result: BdkFullScanResponse<KeychainKind>) -> Arc<Update> {
    Arc::new(Update(BdkUpdate {
        last_active_indices: result.last_active_indices,
        tx_update: result.tx_update,
        chain: result.chain_update,
    }))
}

fn sync_update(result: BdkSyncResponse) -> Arc<Update> {
    Arc::new(Update(BdkUpdate {
        last_active_indices: BTreeMap::default(),
        tx_update: result.tx_update,
        chain: result.chain_update,
    }))
}

#[cfg(not(target_arch = "wasm32"))]
#[uniffi::export]
impl EsploraClient {
    /// Creates a new bdk client from an esplora_client::BlockingClient.
    /// Optional: Set the proxy of the builder.
    #[uniffi::constructor(default(proxy = None))]
    pub fn new(url: String, proxy: Option<String>) -> Self {
        let mut builder = Builder::new(url.as_str());
        if let Some(proxy) = proxy {
            builder = builder.proxy(proxy.as_str());
        }
        Self(builder.build_blocking())
    }

    /// Scan keychain scripts for transactions against Esplora, returning an update that can be
    /// applied to the receiving structures.
    ///
    /// `request` provides the data required to perform a script-pubkey-based full scan
    /// (see [`FullScanRequest`]). The full scan for each keychain (`K`) stops after a gap of
    /// `stop_gap` script pubkeys with no associated transactions. `parallel_requests` specifies
    /// the maximum number of HTTP requests to make in parallel.
    pub fn full_scan(
        &self,
        request: Arc<FullScanRequest>,
        stop_gap: u64,
        parallel_requests: u64,
    ) -> Result<Arc<Update>, EsploraError> {
        let request = take_full_scan_request(&request)?;
        let result: BdkFullScanResponse<KeychainKind> =
            self.0
                .full_scan(request, stop_gap as usize, parallel_requests as usize)?;
        Ok(full_scan_update(result))
    }

    /// Sync a set of scripts, txids, and/or outpoints against Esplora.
    ///
    /// `request` provides the data required to perform a script-pubkey-based sync (see
    /// [`SyncRequest`]). `parallel_requests` specifies the maximum number of HTTP requests to make
    /// in parallel.
    pub fn sync(
        &self,
        request: Arc<SyncRequest>,
        parallel_requests: u64,
    ) -> Result<Arc<Update>, EsploraError> {
        let request = take_sync_request(&request)?;
        let result: BdkSyncResponse = self.0.sync(request, parallel_requests as usize)?;
        Ok(sync_update(result))
    }

    /// Broadcast a [`Transaction`] to Esplora.
    pub fn broadcast(&self, transaction: &Transaction) -> Result<(), EsploraError> {
        let bdk_transaction: BdkTransaction = transaction.into();
        self.0
            .broadcast(&bdk_transaction)
            .map_err(EsploraError::from)
    }

    /// Get a [`Transaction`] option given its [`Txid`].
    pub fn get_tx(&self, txid: Arc<Txid>) -> Result<Option<Arc<Transaction>>, EsploraError> {
        let tx_opt = self.0.get_tx(&txid.0)?;
        Ok(tx_opt.map(|inner| Arc::new(Transaction::from(inner))))
    }

    /// Get a `Transaction` given its `Txid`.
    pub fn get_tx_no_opt(&self, txid: Arc<Txid>) -> Result<Arc<Transaction>, EsploraError> {
        self.0
            .get_tx_no_opt(&txid.0)
            .map(Transaction::from)
            .map(Arc::new)
            .map_err(EsploraError::from)
    }

    /// Get the height of the current blockchain tip.
    pub fn get_height(&self) -> Result<u32, EsploraError> {
        self.0.get_height().map_err(EsploraError::from)
    }

    /// Get the `BlockHash` of the current blockchain tip.
    pub fn get_tip_hash(&self) -> Result<Arc<BlockHash>, EsploraError> {
        self.0
            .get_tip_hash()
            .map(|hash| Arc::new(BlockHash(hash)))
            .map_err(EsploraError::from)
    }

    /// Get a map where the key is the confirmation target (in number of
    /// blocks) and the value is the estimated feerate (in sat/vB).
    pub fn get_fee_estimates(&self) -> Result<HashMap<u16, f64>, EsploraError> {
        self.0.get_fee_estimates().map_err(EsploraError::from)
    }

    /// Get the [`BlockHash`] of a specific block height.
    pub fn get_block_hash(&self, block_height: u32) -> Result<Arc<BlockHash>, EsploraError> {
        self.0
            .get_block_hash(block_height)
            .map(|hash| Arc::new(BlockHash(hash)))
            .map_err(EsploraError::from)
    }

    /// Get a Block given a particular BlockHash.
    pub fn get_block_by_hash(
        &self,
        block_hash: Arc<BlockHash>,
    ) -> Result<Option<Block>, EsploraError> {
        self.0
            .get_block_by_hash(&block_hash.0)
            .map(|block| block.map(|block| block.into()))
            .map_err(EsploraError::from)
    }

    /// Get a `Txid` of a transaction given its index in a block with a given hash.
    pub fn get_txid_at_block_index(
        &self,
        block_hash: Arc<BlockHash>,
        index: u64,
    ) -> Result<Option<Arc<Txid>>, EsploraError> {
        self.0
            .get_txid_at_block_index(&block_hash.0, index as usize)
            .map(|txid| txid.map(Txid).map(Arc::new))
            .map_err(EsploraError::from)
    }

    /// Get a `Header` given a particular block hash.
    pub fn get_header_by_hash(&self, block_hash: Arc<BlockHash>) -> Result<Header, EsploraError> {
        self.0
            .get_header_by_hash(&block_hash.0)
            .map(Header::from)
            .map_err(EsploraError::from)
    }

    /// Get the status of a [`Transaction`] given its [`Txid`].
    pub fn get_tx_status(&self, txid: Arc<Txid>) -> Result<TxStatus, EsploraError> {
        self.0
            .get_tx_status(&txid.0)
            .map(TxStatus::from)
            .map_err(EsploraError::from)
    }

    /// Get transaction info given its [`Txid`].
    pub fn get_tx_info(&self, txid: Arc<Txid>) -> Result<Option<Tx>, EsploraError> {
        self.0
            .get_tx_info(&txid.0)
            .map(|tx| tx.map(Tx::from))
            .map_err(EsploraError::from)
    }

    /// Get transaction history for the specified address, sorted with newest first.
    ///
    /// Returns up to 50 mempool transactions plus the first 25 confirmed transactions.
    /// More can be requested by specifying the last txid seen by the previous query.
    pub fn get_address_txs(
        &self,
        address: Arc<Address>,
        last_seen: Option<Arc<Txid>>,
    ) -> Result<Vec<Tx>, EsploraError> {
        let last_seen = last_seen.as_ref().map(|txid| txid.0);
        let txs = self.0.get_address_txs(&address.as_ref().0, last_seen)?;

        Ok(txs.into_iter().map(Tx::from).collect())
    }

    /// Get a merkle inclusion proof for a [`Transaction`] with the given
    /// [`Txid`].
    pub fn get_merkle_proof(&self, txid: &Txid) -> Result<Option<MerkleProof>, EsploraError> {
        self.0
            .get_merkle_proof(&txid.0)
            .map(|proof| proof.map(MerkleProof::from))
            .map_err(EsploraError::from)
    }

    /// Get the spending status of an output given a `Txid` and the output
    /// index.
    pub fn get_output_status(
        &self,
        txid: Arc<Txid>,
        vout: u64,
    ) -> Result<Option<OutputStatus>, EsploraError> {
        self.0
            .get_output_status(&txid.0, vout)
            .map(|status| status.map(OutputStatus::from))
            .map_err(EsploraError::from)
    }
}

#[cfg(target_arch = "wasm32")]
#[uniffi::export]
impl EsploraClient {
    /// Creates a new bdk client from an esplora_client::AsyncClient.
    /// Optional: Set the proxy of the builder. The proxy is ignored on wasm32.
    #[uniffi::constructor(default(proxy = None))]
    pub fn new(url: String, proxy: Option<String>) -> Result<Self, EsploraError> {
        let mut builder = Builder::new(url.as_str());
        if let Some(proxy) = proxy {
            builder = builder.proxy(proxy.as_str());
        }
        Ok(Self(
            builder.build_async_with_sleeper::<web_sleeper::WebSleeper>()?,
        ))
    }

    /// Scan keychain scripts for transactions against Esplora, returning an update that can be
    /// applied to the receiving structures.
    ///
    /// `request` provides the data required to perform a script-pubkey-based full scan
    /// (see [`FullScanRequest`]). The full scan for each keychain (`K`) stops after a gap of
    /// `stop_gap` script pubkeys with no associated transactions. `parallel_requests` specifies
    /// the maximum number of HTTP requests to make in parallel.
    pub async fn full_scan(
        &self,
        request: Arc<FullScanRequest>,
        stop_gap: u64,
        parallel_requests: u64,
    ) -> Result<Arc<Update>, EsploraError> {
        let request = take_full_scan_request(&request)?;
        let result: BdkFullScanResponse<KeychainKind> = self
            .0
            .full_scan(request, stop_gap as usize, parallel_requests as usize)
            .await?;
        Ok(full_scan_update(result))
    }

    /// Sync a set of scripts, txids, and/or outpoints against Esplora.
    ///
    /// `request` provides the data required to perform a script-pubkey-based sync (see
    /// [`SyncRequest`]). `parallel_requests` specifies the maximum number of HTTP requests to make
    /// in parallel.
    pub async fn sync(
        &self,
        request: Arc<SyncRequest>,
        parallel_requests: u64,
    ) -> Result<Arc<Update>, EsploraError> {
        let request = take_sync_request(&request)?;
        let result: BdkSyncResponse = self.0.sync(request, parallel_requests as usize).await?;
        Ok(sync_update(result))
    }

    /// Broadcast a [`Transaction`] to Esplora.
    pub async fn broadcast(&self, transaction: &Transaction) -> Result<(), EsploraError> {
        let bdk_transaction: BdkTransaction = transaction.into();
        self.0
            .broadcast(&bdk_transaction)
            .await
            .map_err(EsploraError::from)
    }

    /// Get a [`Transaction`] option given its [`Txid`].
    pub async fn get_tx(&self, txid: Arc<Txid>) -> Result<Option<Arc<Transaction>>, EsploraError> {
        let tx_opt = self.0.get_tx(&txid.0).await?;
        Ok(tx_opt.map(|inner| Arc::new(Transaction::from(inner))))
    }

    /// Get a `Transaction` given its `Txid`.
    pub async fn get_tx_no_opt(&self, txid: Arc<Txid>) -> Result<Arc<Transaction>, EsploraError> {
        self.0
            .get_tx_no_opt(&txid.0)
            .await
            .map(Transaction::from)
            .map(Arc::new)
            .map_err(EsploraError::from)
    }

    /// Get the height of the current blockchain tip.
    pub async fn get_height(&self) -> Result<u32, EsploraError> {
        self.0.get_height().await.map_err(EsploraError::from)
    }

    /// Get the `BlockHash` of the current blockchain tip.
    pub async fn get_tip_hash(&self) -> Result<Arc<BlockHash>, EsploraError> {
        self.0
            .get_tip_hash()
            .await
            .map(|hash| Arc::new(BlockHash(hash)))
            .map_err(EsploraError::from)
    }

    /// Get a map where the key is the confirmation target (in number of
    /// blocks) and the value is the estimated feerate (in sat/vB).
    pub async fn get_fee_estimates(&self) -> Result<HashMap<u16, f64>, EsploraError> {
        self.0.get_fee_estimates().await.map_err(EsploraError::from)
    }

    /// Get the [`BlockHash`] of a specific block height.
    pub async fn get_block_hash(&self, block_height: u32) -> Result<Arc<BlockHash>, EsploraError> {
        self.0
            .get_block_hash(block_height)
            .await
            .map(|hash| Arc::new(BlockHash(hash)))
            .map_err(EsploraError::from)
    }

    /// Get a Block given a particular BlockHash.
    pub async fn get_block_by_hash(
        &self,
        block_hash: Arc<BlockHash>,
    ) -> Result<Option<Block>, EsploraError> {
        self.0
            .get_block_by_hash(&block_hash.0)
            .await
            .map(|block| block.map(|block| block.into()))
            .map_err(EsploraError::from)
    }

    /// Get a `Txid` of a transaction given its index in a block with a given hash.
    pub async fn get_txid_at_block_index(
        &self,
        block_hash: Arc<BlockHash>,
        index: u64,
    ) -> Result<Option<Arc<Txid>>, EsploraError> {
        self.0
            .get_txid_at_block_index(&block_hash.0, index as usize)
            .await
            .map(|txid| txid.map(Txid).map(Arc::new))
            .map_err(EsploraError::from)
    }

    /// Get a `Header` given a particular block hash.
    pub async fn get_header_by_hash(
        &self,
        block_hash: Arc<BlockHash>,
    ) -> Result<Header, EsploraError> {
        self.0
            .get_header_by_hash(&block_hash.0)
            .await
            .map(Header::from)
            .map_err(EsploraError::from)
    }

    /// Get the status of a [`Transaction`] given its [`Txid`].
    pub async fn get_tx_status(&self, txid: Arc<Txid>) -> Result<TxStatus, EsploraError> {
        self.0
            .get_tx_status(&txid.0)
            .await
            .map(TxStatus::from)
            .map_err(EsploraError::from)
    }

    /// Get transaction info given its [`Txid`].
    pub async fn get_tx_info(&self, txid: Arc<Txid>) -> Result<Option<Tx>, EsploraError> {
        self.0
            .get_tx_info(&txid.0)
            .await
            .map(|tx| tx.map(Tx::from))
            .map_err(EsploraError::from)
    }

    /// Get transaction history for the specified address, sorted with newest first.
    ///
    /// Returns up to 50 mempool transactions plus the first 25 confirmed transactions.
    /// More can be requested by specifying the last txid seen by the previous query.
    pub async fn get_address_txs(
        &self,
        address: Arc<Address>,
        last_seen: Option<Arc<Txid>>,
    ) -> Result<Vec<Tx>, EsploraError> {
        let last_seen = last_seen.as_ref().map(|txid| txid.0);
        let txs = self
            .0
            .get_address_txs(&address.as_ref().0, last_seen)
            .await?;

        Ok(txs.into_iter().map(Tx::from).collect())
    }

    /// Get a merkle inclusion proof for a [`Transaction`] with the given
    /// [`Txid`].
    pub async fn get_merkle_proof(&self, txid: &Txid) -> Result<Option<MerkleProof>, EsploraError> {
        self.0
            .get_merkle_proof(&txid.0)
            .await
            .map(|proof| proof.map(MerkleProof::from))
            .map_err(EsploraError::from)
    }

    /// Get the spending status of an output given a `Txid` and the output
    /// index.
    pub async fn get_output_status(
        &self,
        txid: Arc<Txid>,
        vout: u64,
    ) -> Result<Option<OutputStatus>, EsploraError> {
        self.0
            .get_output_status(&txid.0, vout)
            .await
            .map(|status| status.map(OutputStatus::from))
            .map_err(EsploraError::from)
    }
}
