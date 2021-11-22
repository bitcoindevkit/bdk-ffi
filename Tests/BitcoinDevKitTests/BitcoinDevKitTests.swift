import XCTest
@testable import BitcoinDevKit

final class BitcoinDevKitTests: XCTestCase {
    func testMemoryWalletNewAddress() throws {
        let desc = "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/*)"
        let config = DatabaseConfig.memory(junk: "")
        let wallet = try OfflineWallet.init(descriptor: desc, network: Network.regtest, databaseConfig: config)
        let address = wallet.getNewAddress()
        XCTAssertEqual(address, "bcrt1qzg4mckdh50nwdm9hkzq06528rsu73hjxytqkxs")
    }
}
