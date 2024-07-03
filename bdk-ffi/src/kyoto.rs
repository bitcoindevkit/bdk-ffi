use bdk_kyoto::builder::LightClientBuilder;
use bdk_kyoto::logger::{NodeMessageHandler, PrintLogger};
use bdk_kyoto::Node;
use bdk_kyoto::{Client, TrustedPeer};
use bdk_wallet::KeychainKind;
use std::net::{IpAddr, Ipv4Addr};
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::wallet::Update;
use crate::wallet::Wallet;

pub struct LightClient {
    client: Mutex<Client<KeychainKind>>,
}

pub struct LightNode {
    node: Mutex<Node>,
}

pub struct NodePair {
    pub node: Arc<LightNode>,
    pub client: Arc<LightClient>,
}

pub fn build_light_client(
    wallet: Arc<Wallet>,
    peers: Vec<Peer>,
    connections: u8,
    recovery_height: Option<u32>,
    data_dir: String,
    logger: Option<Arc<dyn NodeMessageHandler>>,
) -> NodePair {
    let peers = peers
        .into_iter()
        .map(|ip| match ip {
            Peer::V4 { q1, q2, q3, q4 } => IpAddr::V4(Ipv4Addr::new(q1, q2, q3, q4)),
            Peer::V6 { addr } => IpAddr::V6(addr.parse().unwrap()),
        })
        .map(TrustedPeer::from_ip)
        .collect::<Vec<TrustedPeer>>();

    let wallet = wallet.get_wallet();
    let logger = logger.unwrap_or(Arc::new(PrintLogger::new()));

    let mut builder = LightClientBuilder::new(&wallet)
        .connections(connections)
        .logger(logger)
        .data_dir(PathBuf::from_str(&data_dir).unwrap())
        .peers(peers);

    if let Some(recovery) = recovery_height {
        builder = builder.scan_after(recovery)
    }

    let (node, bdk_kyoto_client) = builder.build();

    let node = LightNode {
        node: Mutex::new(node),
    };

    let client = LightClient {
        client: Mutex::new(bdk_kyoto_client),
    };

    NodePair {
        node: Arc::new(node),
        client: Arc::new(client),
    }
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
    pub async fn update(&self) -> Option<Arc<Update>> {
        let update = self.client.lock().await.update().await;
        update.map(|update| Arc::new(Update(update.into())))
    }
}

impl LightNode {
    pub async fn run(&self) {
        let _ = self.node.lock().await.run().await;
    }
}

pub enum Peer {
    V4 { q1: u8, q2: u8, q3: u8, q4: u8 },
    V6 { addr: String },
}
