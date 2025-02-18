use bdk_kyoto::builder::LightClientBuilder as BDKLightClientBuilder;
use bdk_kyoto::builder::ServiceFlags;
use bdk_kyoto::builder::TrustedPeer;
use bdk_kyoto::kyoto::AddrV2;
use bdk_kyoto::kyoto::ScriptBuf;
use bdk_kyoto::LightClient as BDKLightClient;
use bdk_kyoto::LogSubscriber;
use bdk_kyoto::NodeDefault;
use bdk_kyoto::NodeState;
use bdk_kyoto::RejectReason;
use bdk_kyoto::Requester;
use bdk_kyoto::ScanType;
use bdk_kyoto::UpdateSubscriber;
use bdk_kyoto::WalletExt;
use bdk_kyoto::Warning as Warn;
use bdk_kyoto::WarningSubscriber;

use bitcoin_ffi::FeeRate;

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use tokio::sync::Mutex;

use crate::bitcoin::Transaction;
use crate::error::{LightClientBuilderError, LightClientError};
use crate::wallet::Wallet;
use crate::Update;

const TIMEOUT: u64 = 10;
const DEFAULT_CONNECTIONS: u8 = 2;
const CWD_PATH: &str = ".";

pub struct LightClient {
    pub client: Arc<Client>,
    pub node: Arc<LightNode>,
}

pub struct Client {
    sender: Arc<Requester>,
    log_rx: Mutex<LogSubscriber>,
    warning_rx: Mutex<WarningSubscriber>,
    update_rx: Mutex<UpdateSubscriber>,
}

pub struct LightNode {
    node: NodeDefault,
}

impl LightNode {
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

#[derive(Clone)]
pub struct LightClientBuilder {
    connections: u8,
    data_dir: Option<String>,
    scan_type: ScanType,
    peers: Vec<Peer>,
}

impl LightClientBuilder {
    pub fn new() -> Self {
        LightClientBuilder {
            connections: DEFAULT_CONNECTIONS,
            data_dir: None,
            scan_type: ScanType::default(),
            peers: Vec::new(),
        }
    }

    pub fn connections(&self, connections: u8) -> Arc<Self> {
        Arc::new(LightClientBuilder {
            connections,
            ..self.clone()
        })
    }

    pub fn data_dir(&self, data_dir: String) -> Arc<Self> {
        Arc::new(LightClientBuilder {
            data_dir: Some(data_dir),
            ..self.clone()
        })
    }

    pub fn scan_type(&self, scan_type: ScanType) -> Arc<Self> {
        Arc::new(LightClientBuilder {
            scan_type,
            ..self.clone()
        })
    }

    pub fn peers(&self, peers: Vec<Peer>) -> Arc<Self> {
        Arc::new(LightClientBuilder {
            peers,
            ..self.clone()
        })
    }

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
            .scan_type(self.scan_type)
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

impl Client {
    pub async fn next_log(&self) -> Log {
        let mut log_rx = self.log_rx.lock().await;
        log_rx.next_log().await.into()
    }

    pub async fn next_warning(&self) -> Warning {
        let mut warn_rx = self.warning_rx.lock().await;
        warn_rx.next_warning().await.into()
    }

    pub async fn update(&self) -> Option<Arc<Update>> {
        let update = self.update_rx.lock().await.update().await;
        update.map(|update| Arc::new(Update(update)))
    }

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

    pub async fn broadcast(&self, transaction: &Transaction) -> Result<(), LightClientError> {
        let tx = transaction.into();
        self.sender.broadcast_random(tx).await.map_err(From::from)
    }

    pub async fn min_broadcast_feerate(&self) -> Result<Arc<FeeRate>, LightClientError> {
        self.sender
            .broadcast_min_feerate()
            .await
            .map_err(|_| LightClientError::NodeStopped)
            .map(|fee| Arc::new(FeeRate(fee)))
    }

    pub async fn is_running(&self) -> bool {
        self.sender.is_running().await
    }

    pub async fn shutdown(&self) -> Result<(), LightClientError> {
        self.sender.shutdown().await.map_err(From::from)
    }
}

pub enum Log {
    Dialog { log: String },
    ConnectionsMet,
    Progress { progress: f32 },
    StateUpdate { node_state: NodeState },
    TxSent { txid: String },
}

impl From<bdk_kyoto::Log> for Log {
    fn from(value: bdk_kyoto::Log) -> Log {
        match value {
            bdk_kyoto::Log::Dialog(log) => Log::Dialog { log },
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

pub enum Warning {
    NeedConnections,
    PeerTimedOut,
    CouldNotConnect,
    NoCompactFilters,
    PotentialStaleTip,
    UnsolicitedMessage,
    InvalidStartHeight,
    CorruptedHeaders,
    TransactionRejected {
        txid: String,
        reason: Option<String>,
    },
    FailedPersistence {
        warning: String,
    },
    EvaluatingFork,
    EmptyPeerDatabase,
    UnexpectedSyncError {
        warning: String,
    },
    RequestFailed,
}

impl From<Warn> for Warning {
    fn from(value: Warn) -> Warning {
        match value {
            Warn::NotEnoughConnections => Warning::NeedConnections,
            Warn::PeerTimedOut => Warning::PeerTimedOut,
            Warn::CouldNotConnect => Warning::CouldNotConnect,
            Warn::NoCompactFilters => Warning::NoCompactFilters,
            Warn::PotentialStaleTip => Warning::PotentialStaleTip,
            Warn::UnsolicitedMessage => Warning::UnsolicitedMessage,
            Warn::UnlinkableAnchor => Warning::InvalidStartHeight,
            Warn::CorruptedHeaders => Warning::CorruptedHeaders,
            Warn::TransactionRejected(reject) => {
                let reason = reject.reason.map(|r| r.into_string());
                Warning::TransactionRejected {
                    txid: reject.txid.to_string(),
                    reason,
                }
            }
            Warn::FailedPersistance { warning } => Warning::FailedPersistence { warning },
            Warn::EvaluatingFork => Warning::EvaluatingFork,
            Warn::EmptyPeerDatabase => Warning::EmptyPeerDatabase,
            Warn::UnexpectedSyncError { warning } => Warning::UnexpectedSyncError { warning },
            Warn::ChannelDropped => Warning::RequestFailed,
        }
    }
}

#[derive(Clone)]
pub struct Peer {
    pub address: Arc<IpAddress>,
    pub port: Option<u16>,
    pub v2_transport: bool,
}

pub struct IpAddress {
    inner: IpAddr,
}

impl IpAddress {
    pub fn from_ipv4(q1: u8, q2: u8, q3: u8, q4: u8) -> Self {
        Self {
            inner: IpAddr::V4(Ipv4Addr::new(q1, q2, q3, q4)),
        }
    }

    #[allow(clippy::too_many_arguments)]
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
