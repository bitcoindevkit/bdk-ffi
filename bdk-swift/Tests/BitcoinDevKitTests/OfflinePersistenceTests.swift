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

final class OfflinePersistenceTests: XCTestCase {
    var dbFilePath: URL!

    override func setUpWithError() throws {
        super.setUp()
        let currentDirectoryURL = URL(fileURLWithPath: FileManager.default.currentDirectoryPath)
        let dbFileName = "pre_existing_wallet_persistence_test.sqlite"
        dbFilePath = currentDirectoryURL.appendingPathComponent(dbFileName)
    }

    func testPersistence() throws {
        let sqliteStore = try! SqliteStore(path: dbFilePath.path)
        let initialChangeSet = try! sqliteStore.read()
        XCTAssertTrue(initialChangeSet != nil, "ChangeSet should not be nil after loading a valid database")
        
        let wallet = try Wallet.newOrLoad(
            descriptor: descriptor,
            changeDescriptor: changeDescriptor,
            changeSet: initialChangeSet,
            network: .signet
        )
        let nextAddress: AddressInfo = wallet.revealNextAddress(keychain: KeychainKind.external)
        print("Address: \(nextAddress)")

        XCTAssertTrue(nextAddress.address.description == "tb1qan3lldunh37ma6c0afeywgjyjgnyc8uz975zl2")
        XCTAssertTrue(nextAddress.index == 7)
    }
}
