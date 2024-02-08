import XCTest
@testable import BitcoinDevKit

final class OfflineWalletTests: XCTestCase {
    override func tearDownWithError() throws {
        let fileManager = FileManager.default
        let dbFileName = "bdk_persistence.db"
        let documentDirectory = fileManager.urls(for: .documentDirectory, in: .userDomainMask).first!
        let dbFilePath: URL = documentDirectory.appendingPathComponent(dbFileName)

        if fileManager.fileExists(atPath: dbFilePath.path) {
            try fileManager.removeItem(at: dbFilePath)
        }
    }

    func testNewAddress() throws {
        let descriptor: Descriptor = try Descriptor(
            descriptor: "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/*)",
            network: Network.testnet
        )
        let wallet: Wallet = try Wallet(
            descriptor: descriptor,
            changeDescriptor: nil,
            persistenceBackendPath: "bdk_persistence.db",
            network: .testnet
        )
        let addressInfo: AddressInfo = wallet.getAddress(addressIndex: AddressIndex.new)

        XCTAssertTrue(addressInfo.address.isValidForNetwork(network: Network.testnet),
                     "Address is not valid for testnet network")
        XCTAssertTrue(addressInfo.address.isValidForNetwork(network: Network.signet),
                     "Address is not valid for signet network")
        XCTAssertFalse(addressInfo.address.isValidForNetwork(network: Network.regtest),
                      "Address is valid for regtest network, but it shouldn't be")
        XCTAssertFalse(addressInfo.address.isValidForNetwork(network: Network.bitcoin),
                      "Address is valid for bitcoin network, but it shouldn't be")

        XCTAssertEqual(addressInfo.address.asString(), "tb1qzg4mckdh50nwdm9hkzq06528rsu73hjxxzem3e")
    }
    
    func testBalance() throws {
        let descriptor: Descriptor = try Descriptor(
            descriptor: "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/*)",
            network: Network.testnet
        )
        let wallet: Wallet = try Wallet(
            descriptor: descriptor,
            changeDescriptor: nil,
            persistenceBackendPath: "bdk_persistence.db",
            network: .testnet
        )

        XCTAssertEqual(wallet.getBalance().total, 0)
    }
}
