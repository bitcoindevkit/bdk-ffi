use bdk_kyoto::builder::LightClientBuilder as BDKLightClientBuilder;
use bdk_kyoto::builder::ServiceFlags;
use bdk_kyoto::builder::TrustedPeer;
use bdk_kyoto::kyoto::AddrV2;
use bdk_kyoto::logger::PrintLogger;
use bdk_kyoto::EventReceiver;
use bdk_kyoto::EventSender;
use bdk_kyoto::LightClient as BDKLightClient;
use bdk_kyoto::NodeDefault;
use bdk_kyoto::NodeEventHandler;
use bdk_kyoto::TxBroadcast;
use bdk_kyoto::TxBroadcastPolicy;
use bdk_wallet::KeychainKind;
use bitcoin_ffi::FeeRate;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

use crate::bitcoin::{Address, Transaction};
use crate::error::{LightClientBuilderError, LightClientError};
use crate::wallet::Wallet;
use crate::Update;

const TIMEOUT: u64 = 10;
const DEFAULT_CONNECTIONS: u8 = 2;
const CWD_PATH: &str = ".";

pub struct LightClient {
    pub event_publisher: Arc<EventPublisher>,
    pub event_subscriber: Arc<EventSubscriber>,
    pub node: Arc<LightNode>,
}

pub struct EventPublisher {
    sender: Arc<EventSender>,
}

pub struct EventSubscriber {
    receiver: Mutex<EventReceiver<KeychainKind>>,
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
    recovery_height: Option<u32>,
    peers: Vec<Peer>,
}

impl LightClientBuilder {
    pub fn new() -> Self {
        LightClientBuilder {
            connections: DEFAULT_CONNECTIONS,
            data_dir: None,
            recovery_height: None,
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

    pub fn recovery_height(&self, recovery_height: u32) -> Arc<Self> {
        Arc::new(LightClientBuilder {
            recovery_height: Some(recovery_height),
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
        let path_buf = match self.data_dir.clone() {
            Some(path) => {
                PathBuf::from_str(&path).map_err(|e| LightClientBuilderError::DatabaseError {
                    reason: e.to_string(),
                })?
            }
            None => {
                PathBuf::from_str(CWD_PATH).map_err(|e| LightClientBuilderError::DatabaseError {
                    reason: e.to_string(),
                })?
            }
        };
        let mut builder = BDKLightClientBuilder::new()
            .connections(self.connections)
            .data_dir(path_buf)
            .timeout_duration(Duration::from_secs(TIMEOUT))
            .peers(trusted_peers);

        if let Some(recovery) = self.recovery_height {
            builder = builder.scan_after(recovery)
        }

        let BDKLightClient {
            sender,
            receiver,
            node,
        } = builder.build(&wallet)?;

        let node = LightNode { node };

        let event_publisher = EventPublisher {
            sender: Arc::new(sender),
        };

        let event_subscriber = EventSubscriber {
            receiver: Mutex::new(receiver),
        };

        Ok(LightClient {
            event_publisher: Arc::new(event_publisher),
            event_subscriber: Arc::new(event_subscriber),
            node: Arc::new(node),
        })
    }
}

impl EventSubscriber {
    pub async fn update(
        &self,
        event_handler: Option<Arc<dyn NodeEventHandler>>,
    ) -> Option<Arc<Update>> {
        let logger = event_handler.unwrap_or(Arc::new(PrintLogger::new()));
        let update = self.receiver.lock().await.update(logger.as_ref()).await;
        update.map(|update| Arc::new(Update(update.into())))
    }

    pub async fn min_broadcast_feerate(&self) -> Arc<FeeRate> {
        let receiver = self.receiver.lock().await;
        Arc::new(receiver.broadcast_minimum().into())
    }
}

impl EventPublisher {
    pub async fn broadcast(
        &self,
        wallet: &Wallet,
        transaction: &Transaction,
    ) -> Result<(), LightClientError> {
        for output in transaction.output() {
            let spk = output.script_pubkey;
            if wallet.is_mine(Arc::clone(&spk)) {
                self.sender
                    .add_script(spk.0.clone())
                    .await
                    .map_err(|_| LightClientError::NodeStopped)?;
            }
        }
        let tx = transaction.into();
        let broadcastable = TxBroadcast::new(tx, TxBroadcastPolicy::RandomPeer);
        self.sender
            .broadcast_tx(broadcastable)
            .await
            .map_err(From::from)
    }

    pub async fn watch_receive_address(
        &self,
        address: Arc<Address>,
    ) -> Result<(), LightClientError> {
        let script = address.script_pubkey().0.clone();
        self.sender.add_script(script).await.map_err(From::from)
    }

    pub async fn shutdown(&self) -> Result<(), LightClientError> {
        self.sender.shutdown().await.map_err(From::from)
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
