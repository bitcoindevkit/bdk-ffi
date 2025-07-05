use bdk_kyoto::builder::NodeBuilder as BDKCbfBuilder;
use bdk_kyoto::builder::NodeBuilderExt;
use bdk_kyoto::kyoto::lookup_host;
use bdk_kyoto::kyoto::tokio;
use bdk_kyoto::kyoto::AddrV2;
use bdk_kyoto::kyoto::ScriptBuf;
use bdk_kyoto::kyoto::ServiceFlags;
use bdk_kyoto::LightClient as BDKLightClient;
use bdk_kyoto::NodeDefault;
use bdk_kyoto::Receiver;
use bdk_kyoto::RejectReason;
use bdk_kyoto::Requester;
use bdk_kyoto::TrustedPeer;
use bdk_kyoto::UnboundedReceiver;
use bdk_kyoto::UpdateSubscriber;
use bdk_kyoto::WalletExt;
use bdk_kyoto::Warning as Warn;

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use tokio::sync::Mutex;

use crate::bitcoin::BlockHash;
use crate::bitcoin::Transaction;
use crate::error::{CbfBuilderError, CbfError};
use crate::types::Update;
use crate::wallet::Wallet;
use crate::FeeRate;

type LogLevel = bdk_kyoto::kyoto::LogLevel;
type NodeState = bdk_kyoto::NodeState;
type ScanType = bdk_kyoto::ScanType;

const DEFAULT_CONNECTIONS: u8 = 2;
const CWD_PATH: &str = ".";
const TCP_HANDSHAKE_TIMEOUT: Duration = Duration::from_secs(2);
const MESSAGE_RESPONSE_TIMEOUT: Duration = Duration::from_secs(5);
const CLOUDFLARE_DNS: IpAddr = IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1));

/// Receive a [`CbfClient`] and [`CbfNode`].
#[derive(Debug, uniffi::Record)]
pub struct CbfComponents {
    /// Publish events to the node, like broadcasting transactions or adding scripts.
    pub client: Arc<CbfClient>,
    /// The node to run and fetch transactions for a [`Wallet`].
    pub node: Arc<CbfNode>,
}

/// A [`CbfClient`] handles wallet updates from a [`CbfNode`].
#[derive(Debug, uniffi::Object)]
pub struct CbfClient {
    sender: Arc<Requester>,
    log_rx: Mutex<Receiver<String>>,
    info_rx: Mutex<Receiver<bdk_kyoto::Info>>,
    warning_rx: Mutex<UnboundedReceiver<bdk_kyoto::Warning>>,
    update_rx: Mutex<UpdateSubscriber>,
    dns_resolver: IpAddr,
}

/// A [`CbfNode`] gathers transactions for a [`Wallet`].
/// To receive [`Update`] for [`Wallet`], refer to the
/// [`CbfClient`]. The [`CbfNode`] will run until instructed
/// to stop.
#[derive(Debug, uniffi::Object)]
pub struct CbfNode {
    node: NodeDefault,
}

#[uniffi::export]
impl CbfNode {
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
pub struct CbfBuilder {
    connections: u8,
    handshake_timeout: Duration,
    response_timeout: Duration,
    data_dir: Option<String>,
    scan_type: ScanType,
    log_level: LogLevel,
    dns_resolver: Option<Arc<IpAddress>>,
    socks5_proxy: Option<Socks5Proxy>,
    peers: Vec<Peer>,
}

#[uniffi::export]
impl CbfBuilder {
    /// Start a new [`CbfBuilder`]
    #[uniffi::constructor]
    pub fn new() -> Self {
        CbfBuilder {
            connections: DEFAULT_CONNECTIONS,
            handshake_timeout: TCP_HANDSHAKE_TIMEOUT,
            response_timeout: MESSAGE_RESPONSE_TIMEOUT,
            data_dir: None,
            scan_type: ScanType::default(),
            log_level: LogLevel::default(),
            dns_resolver: None,
            socks5_proxy: None,
            peers: Vec::new(),
        }
    }

    /// The number of connections for the light client to maintain. Default is two.
    pub fn connections(&self, connections: u8) -> Arc<Self> {
        Arc::new(CbfBuilder {
            connections,
            ..self.clone()
        })
    }

    /// Directory to store block headers and peers. If none is provided, the current
    /// working directory will be used.
    pub fn data_dir(&self, data_dir: String) -> Arc<Self> {
        Arc::new(CbfBuilder {
            data_dir: Some(data_dir),
            ..self.clone()
        })
    }

    /// Select between syncing, recovering, or scanning for new wallets.
    pub fn scan_type(&self, scan_type: ScanType) -> Arc<Self> {
        Arc::new(CbfBuilder {
            scan_type,
            ..self.clone()
        })
    }

    /// Set the log level for the node. Production applications may want to omit `Debug` messages
    /// to avoid heap allocations.
    pub fn log_level(&self, log_level: LogLevel) -> Arc<Self> {
        Arc::new(CbfBuilder {
            log_level,
            ..self.clone()
        })
    }

    /// Bitcoin full-nodes to attempt a connection with.
    pub fn peers(&self, peers: Vec<Peer>) -> Arc<Self> {
        Arc::new(CbfBuilder {
            peers,
            ..self.clone()
        })
    }

    /// Configure the time in milliseconds that a node has to:
    /// 1. Respond to the initial connection
    /// 2. Respond to a request
    pub fn configure_timeout_millis(&self, handshake: u64, response: u64) -> Arc<Self> {
        Arc::new(CbfBuilder {
            handshake_timeout: Duration::from_millis(handshake),
            response_timeout: Duration::from_millis(response),
            ..self.clone()
        })
    }

    /// Configure a custom DNS resolver when querying DNS seeds. Default is `1.1.1.1` managed by
    /// CloudFlare.
    pub fn dns_resolver(&self, dns_resolver: Arc<IpAddress>) -> Arc<Self> {
        Arc::new(CbfBuilder {
            dns_resolver: Some(dns_resolver),
            ..self.clone()
        })
    }

    /// Configure connections to be established through a `Socks5 proxy. The vast majority of the
    /// time, the connection is to a local Tor daemon, which is typically exposed at
    /// `127.0.0.1:9050`.
    pub fn socks5_proxy(&self, proxy: Socks5Proxy) -> Arc<Self> {
        Arc::new(CbfBuilder {
            socks5_proxy: Some(proxy),
            ..self.clone()
        })
    }

    /// Construct a [`CbfComponents`] for a [`Wallet`].
    pub fn build(&self, wallet: &Wallet) -> Result<CbfComponents, CbfBuilderError> {
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

        let mut builder = BDKCbfBuilder::new(wallet.network())
            .required_peers(self.connections)
            .data_dir(path_buf)
            .handshake_timeout(self.handshake_timeout)
            .response_timeout(self.response_timeout)
            .log_level(self.log_level)
            .add_peers(trusted_peers);

        if let Some(ip_addr) = self.dns_resolver.clone().map(|ip| ip.inner) {
            builder = builder.dns_resolver(ip_addr);
        }

        if let Some(proxy) = &self.socks5_proxy {
            let port = proxy.port;
            let addr = proxy.address.inner;
            builder = builder.socks5_proxy((addr, port));
        }

        let BDKLightClient {
            requester,
            log_subscriber,
            info_subscriber,
            warning_subscriber,
            update_subscriber,
            node,
        } = builder
            .build_with_wallet(&wallet, self.scan_type)
            .map_err(|e| CbfBuilderError::DatabaseError {
                reason: e.to_string(),
            })?;

        let node = CbfNode { node };
        let client_resolver = self
            .dns_resolver
            .clone()
            .map(|ip| ip.inner)
            .unwrap_or(CLOUDFLARE_DNS);

        let client = CbfClient {
            sender: Arc::new(requester),
            log_rx: Mutex::new(log_subscriber),
            info_rx: Mutex::new(info_subscriber),
            warning_rx: Mutex::new(warning_subscriber),
            update_rx: Mutex::new(update_subscriber),
            dns_resolver: client_resolver,
        };

        Ok(CbfComponents {
            client: Arc::new(client),
            node: Arc::new(node),
        })
    }
}

#[uniffi::export]
impl CbfClient {
    /// Return the next available log message from a node. If none is returned, the node has stopped.
    pub async fn next_log(&self) -> Result<String, CbfError> {
        let mut log_rx = self.log_rx.lock().await;
        log_rx.recv().await.ok_or(CbfError::NodeStopped)
    }

    pub async fn next_info(&self) -> Result<Info, CbfError> {
        let mut info_rx = self.info_rx.lock().await;
        info_rx
            .recv()
            .await
            .map(|e| e.into())
            .ok_or(CbfError::NodeStopped)
    }

    /// Return the next available warning message from a node. If none is returned, the node has stopped.
    pub async fn next_warning(&self) -> Result<Warning, CbfError> {
        let mut warn_rx = self.warning_rx.lock().await;
        warn_rx
            .recv()
            .await
            .map(|warn| warn.into())
            .ok_or(CbfError::NodeStopped)
    }

    /// Return an [`Update`]. This is method returns once the node syncs to the rest of
    /// the network or a new block has been gossiped.
    pub async fn update(&self) -> Result<Update, CbfError> {
        let update = self
            .update_rx
            .lock()
            .await
            .update()
            .await
            .map_err(|_| CbfError::NodeStopped)?;
        Ok(Update(update))
    }

    /// Add scripts for the node to watch for as they are revealed. Typically used after creating
    /// a transaction or revealing a receive address.
    ///
    /// Note that only future blocks will be checked for these scripts, not past blocks.
    pub fn add_revealed_scripts(&self, wallet: &Wallet) -> Result<(), CbfError> {
        let script_iter: Vec<ScriptBuf> = {
            let wallet_lock = wallet.get_wallet();
            wallet_lock.peek_revealed_plus_lookahead().collect()
        };
        for script in script_iter.into_iter() {
            self.sender
                .add_script(script)
                .map_err(|_| CbfError::NodeStopped)?
        }
        Ok(())
    }

    /// Broadcast a transaction to the network, erroring if the node has stopped running.
    pub fn broadcast(&self, transaction: &Transaction) -> Result<(), CbfError> {
        let tx = transaction.into();
        self.sender.broadcast_random(tx).map_err(From::from)
    }

    /// The minimum fee rate required to broadcast a transcation to all connected peers.
    pub async fn min_broadcast_feerate(&self) -> Result<Arc<FeeRate>, CbfError> {
        self.sender
            .broadcast_min_feerate()
            .await
            .map_err(|_| CbfError::NodeStopped)
            .map(|fee| Arc::new(FeeRate(fee)))
    }

    /// Fetch the average fee rate for a block by requesting it from a peer. Not recommend for
    /// resource-limited devices.
    pub async fn average_fee_rate(
        &self,
        blockhash: Arc<BlockHash>,
    ) -> Result<Arc<FeeRate>, CbfError> {
        let fee_rate = self
            .sender
            .average_fee_rate(blockhash.0)
            .await
            .map_err(|_| CbfError::NodeStopped)?;
        Ok(Arc::new(fee_rate.into()))
    }

    /// Add another [`Peer`] to attempt a connection with.
    pub fn connect(&self, peer: Peer) -> Result<(), CbfError> {
        self.sender
            .add_peer(peer)
            .map_err(|_| CbfError::NodeStopped)
    }

    /// Query a Bitcoin DNS seeder using the configured resolver.
    ///
    /// This is **not** a generic DNS implementation. Host names are prefixed with a `x849` to filter
    /// for compact block filter nodes from the seeder. For example `dns.myseeder.com` will be queried
    /// as `x849.dns.myseeder.com`. This has no guarantee to return any `IpAddr`.
    pub async fn lookup_host(&self, hostname: String) -> Vec<Arc<IpAddress>> {
        let nodes = lookup_host(hostname, self.dns_resolver).await;
        nodes
            .into_iter()
            .map(|ip| Arc::new(IpAddress { inner: ip }))
            .collect()
    }
    /// Check if the node is still running in the background.
    pub fn is_running(&self) -> bool {
        self.sender.is_running()
    }

    /// Stop the [`CbfNode`]. Errors if the node is already stopped.
    pub fn shutdown(&self) -> Result<(), CbfError> {
        self.sender.shutdown().map_err(From::from)
    }
}

/// A log message from the node.
#[derive(Debug, uniffi::Enum)]
pub enum Info {
    /// All the required connections have been met. This is subject to change.
    ConnectionsMet,
    /// The node was able to successfully connect to a remote peer.
    SuccessfulHandshake,
    /// The block header chain of most work was extended to this height.
    NewChainHeight { height: u32 },
    /// A new fork was advertised to the node, but has not been selected yet.
    NewFork { height: u32 },
    /// A percentage value of filters that have been scanned.
    Progress { progress: f32 },
    /// A state in the node syncing process.
    StateUpdate { node_state: NodeState },
    /// A transaction was broadcast over the wire to a peer that requested it from our inventory.
    /// The transaction may or may not be rejected by recipient nodes.
    TxGossiped { wtxid: String },
}

impl From<bdk_kyoto::Info> for Info {
    fn from(value: bdk_kyoto::Info) -> Info {
        match value {
            bdk_kyoto::Info::ConnectionsMet => Info::ConnectionsMet,
            bdk_kyoto::Info::SuccessfulHandshake => Info::SuccessfulHandshake,
            bdk_kyoto::Info::NewChainHeight(height) => Info::NewChainHeight { height },
            bdk_kyoto::Info::NewFork { tip } => Info::NewFork { height: tip.height },
            bdk_kyoto::Info::Progress(progress) => Info::Progress {
                progress: progress.percentage_complete(),
            },
            bdk_kyoto::Info::TxGossiped(wtxid) => Info::TxGossiped {
                wtxid: wtxid.to_string(),
            },
            bdk_kyoto::Info::StateChange(state) => Info::StateUpdate { node_state: state },
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
        wtxid: String,
        reason: Option<String>,
    },
    /// A database failed to persist some data and may retry again
    FailedPersistence { warning: String },
    /// The peer sent us a potential fork.
    EvaluatingFork,
    /// The peer database has no values.
    EmptyPeerDatabase,
    /// An unexpected error occurred processing a peer-to-peer message.
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
                    wtxid: payload.wtxid.to_string(),
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

/// Select the category of messages for the node to emit.
#[uniffi::remote(Enum)]
pub enum LogLevel {
    /// Send string messages. These messages are intended for debugging or troubleshooting
    /// node operation.
    Debug,
    /// Send info and warning messages, but omit debug strings - including their memory allocations.
    /// Ideal for a production application that uses minimal logging.
    Info,
    /// Omit debug strings and info messages, including their memory allocations.
    Warning,
}

/// The state of the node with respect to connected peers.
#[uniffi::remote(Enum)]
pub enum NodeState {
    /// We are behind on block headers according to our peers.
    Behind,
    /// We may start downloading compact block filter headers.
    HeadersSynced,
    /// We may start scanning compact block filters.
    FilterHeadersSynced,
    /// We may start asking for blocks with matches.
    FiltersSynced,
    /// We found all known transactions to the wallet.
    TransactionsSynced,
}

/// Sync a wallet from the last known block hash, recover a wallet from a specified height,
/// or perform an expedited block header download for a new wallet.
#[uniffi::remote(Enum)]
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

/// A proxy to route network traffic, most likely through a Tor daemon. Normally this proxy is
/// exposed at 127.0.0.1:9050.
#[derive(Debug, Clone, uniffi::Record)]
pub struct Socks5Proxy {
    /// The IP address, likely `127.0.0.1`
    pub address: Arc<IpAddress>,
    /// The listening port, likely `9050`
    pub port: u16,
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
