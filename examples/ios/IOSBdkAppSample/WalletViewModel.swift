//
//  WalletViewModel.swift
//  IOSBdkAppSample
//
//  Created by Sudarsan Balaji on 29/10/21.
//

import Foundation

class WalletViewModel: ObservableObject {
    enum State {
        case empty
        case loading
        case failed(Error)
        case loaded(OnlineWallet)
    }
    private(set) var key = "private_key"
    @Published private(set) var state = State.empty
    
    func load() {
        state = .loading
        let db = DatabaseConfig.memory(junk: "")
        let descriptor = "wpkh(tprv8ZgxMBicQKsPeSitUfdxhsVaf4BXAASVAbHypn2jnPcjmQZvqZYkeqx7EHQTWvdubTSDa5ben7zHC7sUsx4d8tbTvWdUtHzR8uhHg2CW7MT/*)"
        let electrum = ElectrumConfig(url: "ssl://electrum.blockstream.info:60002", socks5: nil, retry: 5, timeout: nil, stopGap: 10)
        let blockchain = BlockchainConfig.electrum(config: electrum)
        do {
            let wallet = try OnlineWallet(descriptor: descriptor, changeDescriptor: nil, network: Network.testnet, databaseConfig: db, blockchainConfig: blockchain)
            state = State.loaded(wallet)
        } catch let error {
            state = State.failed(error)
        }
    }
}
