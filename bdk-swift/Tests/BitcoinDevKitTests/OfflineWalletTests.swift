import XCTest
@testable import BitcoinDevKit

private let descriptor = try! Descriptor(
    descriptor: "wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/0/*)", 
    network: Network.signet
)
private let changeDescriptor = try! Descriptor(
    descriptor: "wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/1/*)", 
    network: Network.signet
)

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
        let wallet = try Wallet(
            descriptor: descriptor,
            changeDescriptor: changeDescriptor,
            network: .testnet
        )
        let addressInfo: AddressInfo = wallet.revealNextAddress(keychain: KeychainKind.external)

        XCTAssertTrue(addressInfo.address.isValidForNetwork(network: Network.testnet),
                     "Address is not valid for testnet network")
        XCTAssertTrue(addressInfo.address.isValidForNetwork(network: Network.signet),
                     "Address is not valid for signet network")
        XCTAssertFalse(addressInfo.address.isValidForNetwork(network: Network.regtest),
                      "Address is valid for regtest network, but it shouldn't be")
        XCTAssertFalse(addressInfo.address.isValidForNetwork(network: Network.bitcoin),
                      "Address is valid for bitcoin network, but it shouldn't be")

        XCTAssertEqual(addressInfo.address.description, "tb1qrnfslnrve9uncz9pzpvf83k3ukz22ljgees989")
    }
    
    func testBalance() throws {
        let wallet = try Wallet(
            descriptor: descriptor,
            changeDescriptor: changeDescriptor,
            network: .testnet
        )

        XCTAssertEqual(wallet.balance().total.toSat(), 0)
    }
}
