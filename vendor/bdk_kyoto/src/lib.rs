#![doc = include_str!("../README.md")]
//! ## Examples
//!
//! If you have an existing project that leverages `bdk_wallet`, building the compact block filter
//! _node_ and _client_ is simple. You may construct and configure a node to integrate with your
//! wallet by using the [`BuilderExt`](crate::builder) and [`Builder`](crate::builder).
//!
//! ```no_run
//! # const RECEIVE: &str = "tr([7d94197e/86'/1'/0']tpubDCyQVJj8KzjiQsFjmb3KwECVXPvMwvAxxZGCP9XmWSopmjW3bCV3wD7TgxrUhiGSueDS1MU5X1Vb1YjYcp8jitXc5fXfdC1z68hDDEyKRNr/0/*)";
//! # const CHANGE: &str = "tr([7d94197e/86'/1'/0']tpubDCyQVJj8KzjiQsFjmb3KwECVXPvMwvAxxZGCP9XmWSopmjW3bCV3wD7TgxrUhiGSueDS1MU5X1Vb1YjYcp8jitXc5fXfdC1z68hDDEyKRNr/1/*)";
//! use bdk_wallet::Wallet;
//! use bdk_wallet::bitcoin::Network;
//! use bdk_kyoto::builder::{Builder, BuilderExt};
//! use bdk_kyoto::{LightClient, ScanType};
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let mut wallet = Wallet::create(RECEIVE, CHANGE)
//!         .network(Network::Signet)
//!         .create_wallet_no_persist()?;
//!
//!     let LightClient {
//!         requester,
//!         info_subscriber: _,
//!         warning_subscriber: _,
//!         mut update_subscriber,
//!         node
//!     } = Builder::new(Network::Signet).build_with_wallet(&wallet, ScanType::Sync)?;
//!
//!     tokio::task::spawn(async move { node.run().await });
//!
//!     loop {
//!         let update = update_subscriber.update().await?;
//!         wallet.apply_update(update)?;
//!         return Ok(());
//!     }
//! }
//! ```

#![warn(missing_docs)]
use std::collections::HashSet;

use bdk_wallet::chain::BlockId;
use bdk_wallet::chain::CheckPoint;
pub use bdk_wallet::Update;

use bdk_wallet::chain::{keychain_txout::KeychainTxOutIndex, IndexedTxGraph};
use bdk_wallet::chain::{ConfirmationBlockTime, TxUpdate};
use bdk_wallet::KeychainKind;

pub extern crate bip157;

use bip157::chain::BlockHeaderChanges;
use bip157::ScriptBuf;
#[doc(inline)]
pub use bip157::{
    BlockHash, ClientError, FeeRate, HeaderCheckpoint, Info, Node, RejectPayload, RejectReason,
    Requester, TrustedPeer, TxBroadcast, TxBroadcastPolicy, Warning, Wtxid,
};
use bip157::{Event, SyncUpdate};

#[doc(inline)]
pub use bip157::Receiver;
#[doc(inline)]
pub use bip157::UnboundedReceiver;

#[doc(inline)]
pub use builder::BuilderExt;
pub mod builder;

#[derive(Debug)]
/// A node and associated structs to send and receive events to and from the node.
pub struct LightClient {
    /// Send events to a running node (i.e. broadcast a transaction).
    pub requester: Requester,
    /// Receive informational messages as the node runs.
    pub info_subscriber: Receiver<Info>,
    /// Receive warnings from the node as it runs.
    pub warning_subscriber: UnboundedReceiver<Warning>,
    /// Receive wallet updates from a node.
    pub update_subscriber: UpdateSubscriber,
    /// The underlying node that must be run to fetch blocks from peers.
    pub node: Node,
}

/// Interpret events from a node that is running to apply
/// updates to an underlying wallet.
#[derive(Debug)]
pub struct UpdateSubscriber {
    // request information from the client
    requester: Requester,
    // channel receiver
    receiver: UnboundedReceiver<Event>,
    // changes to local chain
    cp: CheckPoint,
    // receive graph
    graph: IndexedTxGraph<ConfirmationBlockTime, KeychainTxOutIndex<KeychainKind>>,
    // queued blocks to fetch
    queued_blocks: Vec<BlockHash>,
    // queued scripts to check filters
    spk_cache: HashSet<ScriptBuf>,
}

impl UpdateSubscriber {
    fn new(
        requester: Requester,
        scan_type: ScanType,
        receiver: UnboundedReceiver<Event>,
        cp: CheckPoint,
        graph: IndexedTxGraph<ConfirmationBlockTime, KeychainTxOutIndex<KeychainKind>>,
    ) -> Self {
        let spk_cache = match scan_type {
            ScanType::Sync => Self::peek_scripts(&graph.index, graph.index.lookahead()),
            ScanType::Recovery {
                used_script_index,
                checkpoint: _,
            } => Self::peek_scripts(&graph.index, used_script_index),
        };
        Self {
            requester,
            receiver,
            cp,
            graph,
            queued_blocks: Vec::new(),
            spk_cache,
        }
    }
    /// Return the most recent update from the node once it has synced to the network's tip.
    /// This may take a significant portion of time during wallet recoveries or dormant wallets.
    /// Note that you may call this method in a loop as long as the node is running.
    ///
    /// **Warning**
    ///
    /// This method is _not_ cancel safe. You cannot use it within a `tokio::select` arm.
    pub async fn update(&mut self) -> Result<Update, UpdateError> {
        let mut cp = self.cp.clone();
        while let Some(message) = self.receiver.recv().await {
            match message {
                Event::IndexedFilter(filter) => {
                    let block_hash = filter.block_hash();
                    if filter.contains_any(self.spk_cache.iter()) {
                        self.queued_blocks.push(block_hash);
                    }
                }
                // these are emitted for every block
                Event::ChainUpdate(BlockHeaderChanges::Connected(at)) => {
                    let block_id = BlockId {
                        hash: at.block_hash(),
                        height: at.height,
                    };
                    cp = cp.insert(block_id);
                }
                Event::ChainUpdate(BlockHeaderChanges::Reorganized {
                    accepted,
                    reorganized: _,
                }) => {
                    for header in accepted {
                        let block_id = BlockId {
                            hash: header.block_hash(),
                            height: header.height,
                        };
                        cp = cp.insert(block_id);
                    }
                }
                Event::FiltersSynced(SyncUpdate {
                    tip: _,
                    recent_history: _,
                }) => {
                    for hash in core::mem::take(&mut self.queued_blocks) {
                        let block = self
                            .requester
                            .get_block(hash)
                            .await
                            .map_err(|_| UpdateError::NodeStopped)?;
                        let height = block.height;
                        let block = block.block;
                        let _ = self.graph.apply_block_relevant(&block, height);
                    }
                    self.cp = cp;
                    self.spk_cache.extend(Self::peek_scripts(
                        &self.graph.index,
                        self.graph.index.lookahead(),
                    ));
                    return Ok(self.get_scan_response());
                }
                _ => (),
            }
        }
        Err(UpdateError::NodeStopped)
    }

    // When the client is believed to have synced to the chain tip of most work,
    // we can return a wallet update.
    fn get_scan_response(&mut self) -> Update {
        let tx_update = TxUpdate::from(self.graph.graph().clone());
        let graph = core::mem::take(&mut self.graph);
        let last_active_indices = graph.index.last_used_indices();
        self.graph = IndexedTxGraph::new(graph.index);
        Update {
            tx_update,
            last_active_indices,
            chain: Some(self.cp.clone()),
        }
    }

    fn peek_scripts(
        keychain: &KeychainTxOutIndex<KeychainKind>,
        to_index: u32,
    ) -> HashSet<ScriptBuf> {
        let mut spk_cache = HashSet::new();
        // We pre-compute an SPK cache so as to not call `unbounded_spk_iter` for each filter
        let last_revealed = keychain.last_revealed_indices();
        let ext_index = last_revealed
            .get(&KeychainKind::External)
            .copied()
            .unwrap_or(0);
        let unbounded_ext_spk_iter = keychain
            .unbounded_spk_iter(KeychainKind::External)
            .expect("wallet must have external keychain");
        let bound = (ext_index + to_index) as usize;
        let bounded_ext_iter = unbounded_ext_spk_iter.take(bound).map(|(_, script)| script);
        spk_cache.extend(bounded_ext_iter);
        let int_index = last_revealed
            .get(&KeychainKind::Internal)
            .copied()
            .unwrap_or(0);
        let unbounded_int_spk_iter = keychain.unbounded_spk_iter(KeychainKind::Internal);
        if let Some(int_spk_iter) = unbounded_int_spk_iter {
            let bound = (int_index + to_index) as usize;
            let bounded_int_iter = int_spk_iter.take(bound).map(|(_, script)| script);
            spk_cache.extend(bounded_int_iter);
        }
        spk_cache
    }
}

/// Errors encountered when attempting to construct a wallet update.
#[derive(Debug, Clone, Copy)]
pub enum UpdateError {
    /// The node has stopped running.
    NodeStopped,
}

impl std::fmt::Display for UpdateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UpdateError::NodeStopped => write!(f, "the node halted execution."),
        }
    }
}

impl std::error::Error for UpdateError {}

/// How to scan compact block filters on start up.
#[derive(Debug, Clone, Copy, Default)]
pub enum ScanType {
    /// Sync the wallet from the last known wallet checkpoint to the rest of the network.
    #[default]
    Sync,
    /// Recover an old wallet by scanning after the specified height.
    Recovery {
        /// The amount of scripts used by the wallet that is being recovered.
        used_script_index: u32,
        /// The height in the block chain to begin searching for transactions.
        checkpoint: HeaderCheckpoint,
    },
}
