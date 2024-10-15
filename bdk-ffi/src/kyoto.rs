use bdk_core::bitcoin::p2p::address::AddrV2;
use bdk_kyoto::builder::LightClientBuilder;
use bdk_kyoto::logger::{NodeMessageHandler, PrintLogger};
use bdk_kyoto::{Client, TrustedPeer};
use bdk_kyoto::{NodeDefault, ServiceFlags};
use bdk_wallet::bitcoin::Transaction as BdkTransaction;
use bdk_wallet::KeychainKind;
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

pub struct LightClient {
    client: Mutex<Client<KeychainKind>>,
}

pub struct LightNode {
    node: Mutex<NodeDefault>,
}

pub struct NodePair {
    pub node: Arc<LightNode>,
    pub client: Arc<LightClient>,
}

pub fn build_light_client(
    wallet: &Wallet,
    peers: Vec<Peer>,
    connections: u8,
    recovery_height: Option<u32>,
    data_dir: String,
) -> Result<NodePair, LightClientBuilderError> {
    let mut trusted_peers = Vec::new();
    for peer in peers {
        let services = if peer.v2_transport {
            ServiceFlags::P2P_V2
        } else {
            ServiceFlags::COMPACT_FILTERS
        };
        let addr_v2 = match peer.address.inner {
            IpAddr::V4(ipv4_addr) => AddrV2::Ipv4(ipv4_addr),
            IpAddr::V6(ipv6_addr) => AddrV2::Ipv6(ipv6_addr),
        };
        let trusted_peer = TrustedPeer::new(addr_v2, peer.port, services);
        trusted_peers.push(trusted_peer);
    }

    let wallet = wallet.get_wallet();

    let mut builder = LightClientBuilder::new(&wallet)
        .connections(connections)
        .data_dir(PathBuf::from_str(&data_dir).map_err(|e| {
            LightClientBuilderError::DatabaseError {
                reason: e.to_string(),
            }
        })?)
        .timeout_duration(Duration::from_secs(TIMEOUT))
        .peers(trusted_peers);

    if let Some(recovery) = recovery_height {
        builder = builder.scan_after(recovery)
    }

    let (node, bdk_kyoto_client) = builder.build()?;

    let node = LightNode {
        node: Mutex::new(node),
    };

    let client = LightClient {
        client: Mutex::new(bdk_kyoto_client),
    };

    Ok(NodePair {
        node: Arc::new(node),
        client: Arc::new(client),
    })
}

pub fn run_node(node: Arc<LightNode>) {
    std::thread::spawn(|| {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async move {
                node.as_ref().run().await;
            })
    });
}

impl LightClient {
    pub async fn update(&self, logger: Option<Arc<dyn NodeMessageHandler>>) -> Option<Arc<Update>> {
        let logger = logger.unwrap_or(Arc::new(PrintLogger::new()));
        let update = self.client.lock().await.update(logger.as_ref()).await;
        update.map(|update| Arc::new(Update(update.into())))
    }

    pub async fn broadcast(&self, transaction: Arc<Transaction>) -> Result<(), LightClientError> {
        let client = self.client.lock().await;
        let tx: BdkTransaction = match Arc::try_unwrap(transaction) {
            Ok(val) => val.0,
            Err(arc) => arc.0.clone(),
        };
        client
            .broadcast(tx, bdk_kyoto::TxBroadcastPolicy::RandomPeer)
            .await
            .map_err(From::from)
    }

    pub async fn watch_address(&self, address: Arc<Address>) -> Result<(), LightClientError> {
        let client = self.client.lock().await;
        let script = match Arc::try_unwrap(address.script_pubkey()) {
            Ok(script) => script.into(),
            Err(arc) => arc.0.clone(),
        };
        client.add_script(script).await.map_err(From::from)
    }

    pub async fn shutdown(&self) -> Result<(), LightClientError> {
        let client = self.client.lock().await;
        client.shutdown().await.map_err(From::from)
    }
}

impl LightNode {
    pub async fn run(&self) {
        let _ = self.node.lock().await.run().await;
    }
}

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
