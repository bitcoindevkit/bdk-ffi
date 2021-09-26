use ::safer_ffi::prelude::*;
use bdk::blockchain::{AnyBlockchainConfig, ElectrumBlockchainConfig};
use safer_ffi::boxed::Box;
use safer_ffi::char_p::char_p_ref;

#[derive_ReprC]
#[ReprC::opaque]
#[derive(Debug)]
pub struct BlockchainConfig {
    pub raw: AnyBlockchainConfig,
}

#[ffi_export]
fn new_electrum_config(
    url: char_p_ref,
    socks5: Option<char_p_ref>,
    retry: i16,
    timeout: i16,
    stop_gap: usize,
) -> Box<BlockchainConfig> {
    let url = url.to_string();
    let socks5 = socks5.map(|s| s.to_string());
    let retry = short_to_u8(retry);
    let timeout = short_to_optional_u8(timeout);

    let electrum_config = AnyBlockchainConfig::Electrum(ElectrumBlockchainConfig {
        url,
        socks5,
        retry,
        timeout,
        stop_gap,
    });
    Box::new(BlockchainConfig {
        raw: electrum_config,
    })
}

#[ffi_export]
fn free_blockchain_config(blockchain_config: Box<BlockchainConfig>) {
    drop(blockchain_config);
}

// TODO compact_filter rocksdb not compiling on android, switch to sqlite?
//#[derive_ReprC]
//#[repr(C)]
//#[derive(Debug)]
//pub struct BitcoinPeerConfig {
//    pub address: char_p_boxed,
//    pub socks5: Option<char_p_boxed>,
//    pub socks5_credentials: Option<Box<Tuple2<char_p_boxed, char_p_boxed>>>,
//}
//
//impl From<&BitcoinPeerConfig> for BdkBitcoinPeerConfig {
//    fn from(config: &BitcoinPeerConfig) -> Self {
//        let address = config.address.to_string();
//        let socks5 = config.socks5.as_ref().map(|p| p.to_string());
//        let socks5_credentials = config
//            .socks5_credentials.as_ref()
//            .map(|c| (c._0.to_string(), c._1.to_string()));
//
//        BdkBitcoinPeerConfig {
//            address,
//            socks5: socks5,
//            socks5_credentials: socks5_credentials,
//        }
//    }
//}
//
//
//#[ffi_export]
//fn new_compact_filters_config<'lt>(
//    peers: c_slice::Ref<'lt, BitcoinPeerConfig>,
//    network: char_p_ref,
//    storage_dir: char_p_ref,
//    skip_blocks: usize,
//) -> Box<BlockchainConfig> {
//    let peers = peers.iter().map(|p| p.into()).collect();
//    let network = Network::from_str(network.to_str()).unwrap();
//    let storage_dir = storage_dir.to_string();
//    let skip_blocks = Some(skip_blocks);
//    let cf_config = AnyBlockchainConfig::CompactFilters(CompactFiltersBlockchainConfig {
//        peers,
//        network,
//        storage_dir,
//        skip_blocks,
//    });
//    Box::new(BlockchainConfig { raw: cf_config })
//}

// utility functions

fn short_to_optional_u8(short: i16) -> Option<u8> {
    if short < 0 {
        None
    } else {
        Some(short_to_u8(short))
    }
}

fn short_to_u8(short: i16) -> u8 {
    if short < 0 {
        u8::MIN
    } else {
        u8::try_from(short).unwrap_or(u8::MAX)
    }
}
