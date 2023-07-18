import XCTest
@testable import BitcoinDevKit

final class BitcoinDevKitTests: XCTestCase {
    
    private let descriptor = "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/*)"
    private let databaseConfig = DatabaseConfig.memory
    private let blockchainConfig = BlockchainConfig.electrum(
        config: ElectrumConfig(
            url: "ssl://electrum.blockstream.info:60002",
            socks5: nil,
            retry: 5,
            timeout: nil,
            stopGap: 100,
            validateDomain: true
        )
    )
    
    func testMemoryWalletNewAddress() throws {
        let descriptor = try Descriptor(
            descriptor: descriptor,
            network: .testnet
        )
        let wallet = try Wallet(
            descriptor: descriptor,
            changeDescriptor: nil,
            network: .testnet,
            databaseConfig: databaseConfig
        )
        let addressInfo = try wallet.getAddress(addressIndex: AddressIndex.new)
        
        XCTAssertEqual(addressInfo.address.asString(), "tb1qzg4mckdh50nwdm9hkzq06528rsu73hjxxzem3e")
    }
    
    func testMemoryWalletBalance() throws {
        let descriptor = try Descriptor(
            descriptor: descriptor,
            network: .testnet
        )
        let wallet = try Wallet(
            descriptor: descriptor,
            changeDescriptor: nil,
            network: .testnet,
            databaseConfig: databaseConfig
        )
        let blockchain = try Blockchain(config: blockchainConfig)
        try wallet.sync(
            blockchain: blockchain,
            progress: nil
        )
        let balance = try wallet.getBalance().total
        
        XCTAssertTrue(balance > 0)
    }
    
    
}
