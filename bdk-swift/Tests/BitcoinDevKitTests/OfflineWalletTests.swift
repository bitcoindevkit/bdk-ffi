import XCTest
@testable import BitcoinDevKit

final class OfflineWalletTests: XCTestCase {
    var dbFilePath: URL!

    override func setUpWithError() throws {
        super.setUp()
        let fileManager = FileManager.default
        let documentDirectory = fileManager.urls(for: .documentDirectory, in: .userDomainMask).first!
        let uniqueDbFileName = "bdk_persistence_\(UUID().uuidString).sqlite"
        dbFilePath = documentDirectory.appendingPathComponent(uniqueDbFileName)

        if fileManager.fileExists(atPath: dbFilePath.path) {
            try fileManager.removeItem(at: dbFilePath)
        }
    }

    override func tearDownWithError() throws {
        let fileManager = FileManager.default
        if fileManager.fileExists(atPath: dbFilePath.path) {
            try fileManager.removeItem(at: dbFilePath)
        }
    }

    func testNewAddress() throws {
        let descriptor: Descriptor = try Descriptor(
            descriptor: "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/*)",
            network: Network.testnet
        )
        let wallet = try Wallet(
            descriptor: descriptor,
            changeDescriptor: nil,
            persistenceBackendPath: dbFilePath.path,
            network: .testnet
        )
        let addressInfo: AddressInfo = try wallet.revealNextAddress(keychain: KeychainKind.external)

        XCTAssertTrue(addressInfo.address.isValidForNetwork(network: Network.testnet),
                     "Address is not valid for testnet network")
        XCTAssertTrue(addressInfo.address.isValidForNetwork(network: Network.signet),
                     "Address is not valid for signet network")
        XCTAssertFalse(addressInfo.address.isValidForNetwork(network: Network.regtest),
                      "Address is valid for regtest network, but it shouldn't be")
        XCTAssertFalse(addressInfo.address.isValidForNetwork(network: Network.bitcoin),
                      "Address is valid for bitcoin network, but it shouldn't be")

        XCTAssertEqual(addressInfo.address.description, "tb1qzg4mckdh50nwdm9hkzq06528rsu73hjxxzem3e")
    }
    
    func testBalance() throws {
        let descriptor: Descriptor = try Descriptor(
            descriptor: "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/*)",
            network: Network.testnet
        )
        let wallet = try Wallet(
            descriptor: descriptor,
            changeDescriptor: nil,
            persistenceBackendPath: dbFilePath.path,
            network: .testnet
        )

        XCTAssertEqual(wallet.getBalance().total.toSat(), 0)
    }
}
