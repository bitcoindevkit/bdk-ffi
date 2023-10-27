import XCTest
@testable import BitcoinDevKit

final class LiveWalletTests: XCTestCase {
    func testSyncedBalance() throws {
        let descriptor = try Descriptor(
            descriptor: "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/*)",
            network: Network.testnet
        )
        let wallet = try Wallet.newNoPersist(
            descriptor: descriptor,
            changeDescriptor: nil,
            network: .testnet
        )
        let esploraClient = EsploraClient(url: "https://mempool.space/testnet/api")
        let update = try esploraClient.scan(
            wallet: wallet,
            stopGap: 10,
            parallelRequests: 1
        )
        try wallet.applyUpdate(update: update)

        XCTAssertGreaterThan(wallet.getBalance().total(), UInt64(0))
    }
}
