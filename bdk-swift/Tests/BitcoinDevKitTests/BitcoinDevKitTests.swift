import XCTest
@testable import BitcoinDevKit

final class BitcoinDevKitTests: XCTestCase {
    
    func testMemoryWalletNewAddress() throws {
        let desc = try Descriptor(
            descriptor: "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/*)",
            network: Network.regtest
        )
        let databaseConfig = DatabaseConfig.memory
        let wallet = try Wallet.init(descriptor: desc, changeDescriptor: nil, network: Network.regtest, databaseConfig: databaseConfig)
        let addressInfo = try wallet.getAddress(addressIndex: AddressIndex.new)
        XCTAssertEqual(addressInfo.address.asString(), "bcrt1qzg4mckdh50nwdm9hkzq06528rsu73hjxytqkxs")
    }
    
    private let descriptor = "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/*)"
    let databaseConfig = DatabaseConfig.memory
    let blockchainConfig = BlockchainConfig.electrum(
        config: ElectrumConfig(
            url: "ssl://electrum.blockstream.info:60002",
            socks5: nil,
            retry: 5,
            timeout: nil,
            stopGap: 100,
            validateDomain: true
        )
    )
    
    func testMemoryWalletBalance() throws {
        let descriptor = try Descriptor(
            descriptor: descriptor,
            network: .testnet
        )
        let blockchain = try Blockchain(config: blockchainConfig)
        let wallet = try Wallet(descriptor: descriptor, changeDescriptor: nil, network: Network.testnet, databaseConfig: databaseConfig)
        try wallet.sync(
            blockchain: blockchain,
            progress: nil
        )
        let balance = try wallet.getBalance().total
        
        XCTAssertTrue(balance > 0)
    }
    
    
}
