use bdk_kyoto::builder::LightClientBuilder as BDKLightClientBuilder;
use bdk_kyoto::builder::ServiceFlags;
use bdk_kyoto::builder::TrustedPeer;
use bdk_kyoto::kyoto::tokio;
use bdk_kyoto::kyoto::AddrV2;
use bdk_kyoto::kyoto::ScriptBuf;
use bdk_kyoto::LightClient as BDKLightClient;
use bdk_kyoto::NodeDefault;
use bdk_kyoto::NodeState;
use bdk_kyoto::Receiver;
use bdk_kyoto::RejectReason;
use bdk_kyoto::Requester;
use bdk_kyoto::ScanType as WalletScanType;
use bdk_kyoto::UnboundedReceiver;
use bdk_kyoto::UpdateSubscriber;
use bdk_kyoto::WalletExt;
use bdk_kyoto::Warning as Warn;

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use tokio::sync::Mutex;

use crate::bitcoin::Transaction;
use crate::error::{LightClientBuilderError, LightClientError};
use crate::wallet::Wallet;
use crate::FeeRate;
use crate::Update;

const TIMEOUT: u64 = 10;
const DEFAULT_CONNECTIONS: u8 = 2;
const CWD_PATH: &str = ".";

/// Receive a [`Client`] and [`LightNode`].
#[derive(Debug, uniffi::Record)]
pub struct LightClient {
    /// Publish events to the node, like broadcasting transactions or adding scripts.
    pub client: Arc<Client>,
    /// The node to run and fetch transactions for a [`Wallet`].
    pub node: Arc<LightNode>,
}

/// A [`Client`] handles wallet updates from a [`LightNode`].
#[derive(Debug, uniffi::Object)]
pub struct Client {
    sender: Arc<Requester>,
    log_rx: Mutex<Receiver<bdk_kyoto::Log>>,
    warning_rx: Mutex<UnboundedReceiver<bdk_kyoto::Warning>>,
    update_rx: Mutex<UpdateSubscriber>,
}

/// A [`LightNode`] gathers transactions for a [`Wallet`].
/// To receive [`Update`] for [`Wallet`], refer to the
/// [`Client`]. The [`LightNode`] will run until instructed
/// to stop.
#[derive(Debug, uniffi::Object)]
pub struct LightNode {
    node: NodeDefault,
}

#[uniffi::export]
impl LightNode {
    /// Start the node on a detached OS thread and immediately return.
    pub fn run(self: Arc<Self>) {
        std::thread::spawn(|| {
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(async move {
                    let _ = self.node.run().await;
                })
        });
    }
}

/// Build a BIP 157/158 light client to fetch transactions for a `Wallet`.
///
/// Options:
/// * List of `Peer`: Bitcoin full-nodes for the light client to connect to. May be empty.
/// * `connections`: The number of connections for the light client to maintain.
/// * `scan_type`: Sync, recover, or start a new wallet. For more information see [`ScanType`].
/// * `data_dir`: Optional directory to store block headers and peers.
///
/// A note on recovering wallets. Developers should allow users to provide an
/// approximate recovery height and an estimated number of transactions for the
/// wallet. When determining how many scripts to check filters for, the `Wallet`
/// `lookahead` value will be used. To ensure all transactions are recovered, the
/// `lookahead` should be roughly the number of transactions in the wallet history.
#[derive(Clone, uniffi::Object)]
pub struct LightClientBuilder {
    connections: u8,
    data_dir: Option<String>,
    scan_type: ScanType,
    peers: Vec<Peer>,
}

#[uniffi::export]
impl LightClientBuilder {
    /// Start a new [`LightClientBuilder`]
    #[uniffi::constructor]
    pub fn new() -> Self {
        LightClientBuilder {
            connections: DEFAULT_CONNECTIONS,
            data_dir: None,
            scan_type: ScanType::default(),
            peers: Vec::new(),
        }
    }

    /// The number of connections for the light client to maintain. Default is two.
    pub fn connections(&self, connections: u8) -> Arc<Self> {
        Arc::new(LightClientBuilder {
            connections,
            ..self.clone()
        })
    }

    /// Directory to store block headers and peers. If none is provided, the current
    /// working directory will be used.
    pub fn data_dir(&self, data_dir: String) -> Arc<Self> {
        Arc::new(LightClientBuilder {
            data_dir: Some(data_dir),
            ..self.clone()
        })
    }

    /// Select between syncing, recovering, or scanning for new wallets.
    pub fn scan_type(&self, scan_type: ScanType) -> Arc<Self> {
        Arc::new(LightClientBuilder {
            scan_type,
            ..self.clone()
        })
    }

    /// Bitcoin full-nodes to attempt a connection with.
    pub fn peers(&self, peers: Vec<Peer>) -> Arc<Self> {
        Arc::new(LightClientBuilder {
            peers,
            ..self.clone()
        })
    }

    /// Construct a [`LightClient`] for a [`Wallet`].
    pub fn build(&self, wallet: &Wallet) -> Result<LightClient, LightClientBuilderError> {
        let wallet = wallet.get_wallet();

        let mut trusted_peers = Vec::new();
        for peer in self.peers.iter() {
            trusted_peers.push(peer.clone().into());
        }
        let path_buf = self
            .data_dir
            .clone()
            .map(|path| PathBuf::from(&path))
            .unwrap_or(PathBuf::from(CWD_PATH));

        let builder = BDKLightClientBuilder::new()
            .connections(self.connections)
            .data_dir(path_buf)
            .scan_type(self.scan_type.into())
            .timeout_duration(Duration::from_secs(TIMEOUT))
            .peers(trusted_peers);

        let BDKLightClient {
            requester,
            log_subscriber,
            warning_subscriber,
            update_subscriber,
            node,
        } = builder.build(&wallet)?;

        let node = LightNode { node };

        let client = Client {
            sender: Arc::new(requester),
            log_rx: Mutex::new(log_subscriber),
            warning_rx: Mutex::new(warning_subscriber),
            update_rx: Mutex::new(update_subscriber),
        };

        Ok(LightClient {
            client: Arc::new(client),
            node: Arc::new(node),
        })
    }
}

#[uniffi::export]
impl Client {
    /// Return the next available log message from a node. If none is returned, the node has stopped.
    pub async fn next_log(&self) -> Result<Log, LightClientError> {
        let mut log_rx = self.log_rx.lock().await;
        log_rx
            .recv()
            .await
            .map(|log| log.into())
            .ok_or(LightClientError::NodeStopped)
    }

    /// Return the next available warning message from a node. If none is returned, the node has stopped.
    pub async fn next_warning(&self) -> Result<Warning, LightClientError> {
        let mut warn_rx = self.warning_rx.lock().await;
        warn_rx
            .recv()
            .await
            .map(|warn| warn.into())
            .ok_or(LightClientError::NodeStopped)
    }

    /// Return an [`Update`]. This is method returns once the node syncs to the rest of
    /// the network or a new block has been gossiped.
    pub async fn update(&self) -> Option<Arc<Update>> {
        let update = self.update_rx.lock().await.update().await;
        update.map(|update| Arc::new(Update(update)))
    }

    /// Add scripts for the node to watch for as they are revealed. Typically used after creating
    /// a transaction or revealing a receive address.
    ///
    /// Note that only future blocks will be checked for these scripts, not past blocks.
    pub async fn add_revealed_scripts(&self, wallet: &Wallet) -> Result<(), LightClientError> {
        let script_iter: Vec<ScriptBuf> = {
            let wallet_lock = wallet.get_wallet();
            wallet_lock.peek_revealed_plus_lookahead().collect()
        };
        for script in script_iter.into_iter() {
            self.sender
                .add_script(script)
                .await
                .map_err(|_| LightClientError::NodeStopped)?
        }
        Ok(())
    }

    /// Broadcast a transaction to the network, erroring if the node has stopped running.
    pub async fn broadcast(&self, transaction: &Transaction) -> Result<(), LightClientError> {
        let tx = transaction.into();
        self.sender.broadcast_random(tx).await.map_err(From::from)
    }

    /// The minimum fee rate required to broadcast a transcation to all connected peers.
    pub async fn min_broadcast_feerate(&self) -> Result<Arc<FeeRate>, LightClientError> {
        self.sender
            .broadcast_min_feerate()
            .await
            .map_err(|_| LightClientError::NodeStopped)
            .map(|fee| Arc::new(FeeRate(fee)))
    }

    /// Check if the node is still running in the background.
    pub async fn is_running(&self) -> bool {
        self.sender.is_running().await
    }

    /// Stop the [`LightNode`]. Errors if the node is already stopped.
    pub async fn shutdown(&self) -> Result<(), LightClientError> {
        self.sender.shutdown().await.map_err(From::from)
    }
}

/// A log message from the node.
#[derive(Debug, uniffi::Enum)]
pub enum Log {
    /// A human-readable debug message.
    Debug { log: String },
    /// All the required connections have been met. This is subject to change.
    ConnectionsMet,
    /// A percentage value of filters that have been scanned.
    Progress { progress: f32 },
    /// A state in the node syncing process.
    StateUpdate { node_state: NodeState },
    /// A transaction was broadcast over the wire.
    /// The transaction may or may not be rejected by recipient nodes.
    TxSent { txid: String },
}

impl From<bdk_kyoto::Log> for Log {
    fn from(value: bdk_kyoto::Log) -> Log {
        match value {
            bdk_kyoto::Log::Debug(log) => Log::Debug { log },
            bdk_kyoto::Log::ConnectionsMet => Log::ConnectionsMet,
            bdk_kyoto::Log::Progress(progress) => Log::Progress {
                progress: progress.percentage_complete(),
            },
            bdk_kyoto::Log::TxSent(txid) => Log::TxSent {
                txid: txid.to_string(),
            },
            bdk_kyoto::Log::StateChange(state) => Log::StateUpdate { node_state: state },
        }
    }
}

/// Warnings a node may issue while running.
#[derive(Debug, uniffi::Enum)]
pub enum Warning {
    /// The node is looking for connections to peers.
    NeedConnections,
    /// A connection to a peer timed out.
    PeerTimedOut,
    /// The node was unable to connect to a peer in the database.
    CouldNotConnect,
    /// A connection was maintained, but the peer does not signal for compact block filers.
    NoCompactFilters,
    /// The node has been waiting for new inv and will find new peers to avoid block withholding.
    PotentialStaleTip,
    /// A peer sent us a peer-to-peer message the node did not request.
    UnsolicitedMessage,
    /// The provided starting height is deeper than the database history.
    /// This should not occur under normal use.
    InvalidStartHeight,
    /// The headers in the database do not link together.
    /// Recoverable by deleting the database.
    CorruptedHeaders,
    /// A transaction got rejected, likely for being an insufficient fee or non-standard transaction.
    TransactionRejected {
        txid: String,
        reason: Option<String>,
    },
    /// A database failed to persist some data and may retry again
    FailedPersistence { warning: String },
    /// The peer sent us a potential fork.
    EvaluatingFork,
    /// The peer database has no values.
    EmptyPeerDatabase,
    /// An unexpected error occured processing a peer-to-peer message.
    UnexpectedSyncError { warning: String },
    /// The node failed to respond to a message sent from the client.
    RequestFailed,
}

impl From<Warn> for Warning {
    fn from(value: Warn) -> Warning {
        match value {
            Warn::NeedConnections {
                connected: _,
                required: _,
            } => Warning::NeedConnections,
            Warn::PeerTimedOut => Warning::PeerTimedOut,
            Warn::CouldNotConnect => Warning::CouldNotConnect,
            Warn::NoCompactFilters => Warning::NoCompactFilters,
            Warn::PotentialStaleTip => Warning::PotentialStaleTip,
            Warn::UnsolicitedMessage => Warning::UnsolicitedMessage,
            Warn::InvalidStartHeight => Warning::InvalidStartHeight,
            Warn::CorruptedHeaders => Warning::CorruptedHeaders,
            Warn::TransactionRejected { payload } => {
                let reason = payload.reason.map(|r| r.into_string());
                Warning::TransactionRejected {
                    txid: payload.txid.to_string(),
                    reason,
                }
            }
            Warn::FailedPersistence { warning } => Warning::FailedPersistence { warning },
            Warn::EvaluatingFork => Warning::EvaluatingFork,
            Warn::EmptyPeerDatabase => Warning::EmptyPeerDatabase,
            Warn::UnexpectedSyncError { warning } => Warning::UnexpectedSyncError { warning },
            Warn::ChannelDropped => Warning::RequestFailed,
        }
    }
}

/// Sync a wallet from the last known block hash, recover a wallet from a specified height,
/// or perform an expedited block header download for a new wallet.
#[derive(Debug, Clone, Copy, Default, uniffi::Enum)]
pub enum ScanType {
    /// Perform an expedited header and filter download for a new wallet.
    /// If this option is not set, and the wallet has no history, the
    /// entire chain will be scanned for script inclusions.
    New,
    /// Sync an existing wallet from the last stored chain checkpoint.
    #[default]
    Sync,
    /// Recover an existing wallet by scanning from the specified height.
    Recovery { from_height: u32 },
}

impl From<ScanType> for WalletScanType {
    fn from(value: ScanType) -> Self {
        match value {
            ScanType::New => WalletScanType::New,
            ScanType::Recovery { from_height } => WalletScanType::Recovery { from_height },
            ScanType::Sync => WalletScanType::Sync,
        }
    }
}

/// A peer to connect to over the Bitcoin peer-to-peer network.
#[derive(Clone, uniffi::Record)]
pub struct Peer {
    /// The IP address to reach the node.
    pub address: Arc<IpAddress>,
    /// The port to reach the node. If none is provided, the default
    /// port for the selected network will be used.
    pub port: Option<u16>,
    /// Does the remote node offer encrypted peer-to-peer connection.
    pub v2_transport: bool,
}

/// An IP address to connect to over TCP.
#[derive(Debug, uniffi::Object)]
pub struct IpAddress {
    inner: IpAddr,
}

#[uniffi::export]
impl IpAddress {
    /// Build an IPv4 address.
    #[uniffi::constructor]
    pub fn from_ipv4(q1: u8, q2: u8, q3: u8, q4: u8) -> Self {
        Self {
            inner: IpAddr::V4(Ipv4Addr::new(q1, q2, q3, q4)),
        }
    }

    /// Build an IPv6 address.
    #[allow(clippy::too_many_arguments)]
    #[uniffi::constructor]
    pub fn from_ipv6(a: u16, b: u16, c: u16, d: u16, e: u16, f: u16, g: u16, h: u16) -> Self {
        Self {
            inner: IpAddr::V6(Ipv6Addr::new(a, b, c, d, e, f, g, h)),
        }
    }
}

impl From<Peer> for TrustedPeer {
    fn from(peer: Peer) -> Self {
        let services = if peer.v2_transport {
            let mut services = ServiceFlags::P2P_V2;
            services.add(ServiceFlags::NETWORK);
            services.add(ServiceFlags::COMPACT_FILTERS);
            services
        } else {
            let mut services = ServiceFlags::COMPACT_FILTERS;
            services.add(ServiceFlags::NETWORK);
            services
        };
        let addr_v2 = match peer.address.inner {
            IpAddr::V4(ipv4_addr) => AddrV2::Ipv4(ipv4_addr),
            IpAddr::V6(ipv6_addr) => AddrV2::Ipv6(ipv6_addr),
        };
        TrustedPeer::new(addr_v2, peer.port, services)
    }
}

trait DisplayExt {
    fn into_string(self) -> String;
}

impl DisplayExt for RejectReason {
    fn into_string(self) -> String {
        let message = match self {
            RejectReason::Malformed => "Message could not be decoded.",
            RejectReason::Invalid => "Transaction was invalid for some reason.",
            RejectReason::Obsolete => "Client version is no longer supported.",
            RejectReason::Duplicate => "Duplicate version message received.",
            RejectReason::NonStandard => "Transaction was nonstandard.",
            RejectReason::Dust => "One or more outputs are below the dust threshold.",
            RejectReason::Fee => "Transaction does not have enough fee to be mined.",
            RejectReason::Checkpoint => "Inconsistent with compiled checkpoint.",
        };
        message.into()
    }
}
