import XCTest
@testable import BitcoinDevKit

final class OfflinePersistenceTests: XCTestCase {
    private let descriptor = try! Descriptor(
    descriptor: "wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/0/*)", 
    network: Network.signet
    )
    private let changeDescriptor = try! Descriptor(
        descriptor: "wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/1/*)", 
        network: Network.signet
    )
    var dbFilePath: URL!

    override func setUpWithError() throws {
        super.setUp()

        guard let resourceUrl = Bundle.module.url(
          forResource: "pre_existing_wallet_persistence_test",
          withExtension: "sqlite"
        ) else {
            print("error finding resourceURL")
            return
        }
        dbFilePath = resourceUrl
    }

    func testPersistence() throws {
        let persister = try Persister.newSqlite(path: dbFilePath.path)
        let wallet = try Wallet.load(
            descriptor: descriptor,
            changeDescriptor: changeDescriptor,
            persister: persister
        )
        let nextAddress: AddressInfo = wallet.revealNextAddress(keychain: KeychainKind.external)
        print("Address: \(nextAddress)")

        XCTAssertTrue(nextAddress.address.description == "tb1qan3lldunh37ma6c0afeywgjyjgnyc8uz975zl2")
        XCTAssertTrue(nextAddress.index == 7)
    }

    func testPersistenceWithDescriptor() throws {
        let persister = try Persister.newSqlite(path: dbFilePath.path)
        
        let descriptorPub = try Descriptor(
            descriptor: "wpkh([9122d9e0/84'/1'/0']tpubDCYVtmaSaDzTxcgvoP5AHZNbZKZzrvoNH9KARep88vESc6MxRqAp4LmePc2eeGX6XUxBcdhAmkthWTDqygPz2wLAyHWisD299Lkdrj5egY6/0/*)#zpaanzgu",
            network: Network.signet
        )
        let changeDescriptorPub = try Descriptor(
            descriptor: "wpkh([9122d9e0/84'/1'/0']tpubDCYVtmaSaDzTxcgvoP5AHZNbZKZzrvoNH9KARep88vESc6MxRqAp4LmePc2eeGX6XUxBcdhAmkthWTDqygPz2wLAyHWisD299Lkdrj5egY6/1/*)#n4cuwhcy",
            network: Network.signet
        )
        
        let wallet = try Wallet.load(
            descriptor: descriptorPub,
            changeDescriptor: changeDescriptorPub,
            persister: persister
        )
        let nextAddress: AddressInfo = wallet.revealNextAddress(keychain: KeychainKind.external)
        print("Address: \(nextAddress)")
        
        XCTAssertEqual(nextAddress.index, 7)
        XCTAssertEqual(nextAddress.address.description, "tb1qan3lldunh37ma6c0afeywgjyjgnyc8uz975zl2")
    }
}
