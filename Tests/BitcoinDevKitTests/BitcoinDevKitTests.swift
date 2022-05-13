import XCTest
@testable import BitcoinDevKit

final class BitcoinDevKitTests: XCTestCase {
    func testMemoryWalletNewAddress() throws {
        let desc = "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/*)"
        let databaseConfig = DatabaseConfig.memory
        let wallet = try Wallet.init(descriptor: desc, changeDescriptor: nil, network: Network.regtest, databaseConfig: databaseConfig)
        let address = wallet.getNewAddress()
        XCTAssertEqual(address, "bcrt1qzg4mckdh50nwdm9hkzq06528rsu73hjxytqkxs")
    }
}
