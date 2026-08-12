import XCTest
@testable import BitcoinDevKit

final class PersistenceTests: XCTestCase {
    private let descriptor = try! Descriptor(
        descriptor: "wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/0/*)",
        networkKind: NetworkKind.test
    )
    private let changeDescriptor = try! Descriptor(
        descriptor: "wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/1/*)",
        networkKind: NetworkKind.test
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

    func testUnexpectedInitializeErrorReturnsLoadPersistenceError() {
        struct UnexpectedInitializeError: Error, CustomStringConvertible {
            var description: String { "unexpected initialize failure" }
        }

        final class ThrowingPersistence: Persistence, @unchecked Sendable {
            func initialize() throws -> ChangeSet {
                throw UnexpectedInitializeError()
            }

            func persist(changeset: ChangeSet) throws {}
        }

        let persister = Persister.custom(persistence: ThrowingPersistence())

        XCTAssertThrowsError(
            try Wallet.load(
                descriptor: descriptor,
                changeDescriptor: changeDescriptor,
                persister: persister
            )
        ) { error in
            guard let loadError = error as? LoadWithPersistError else {
                XCTFail("Expected LoadWithPersistError, got \(error)")
                return
            }

            guard case let .Persist(errorMessage) = loadError else {
                XCTFail("Expected .Persist, got \(loadError)")
                return
            }

            XCTAssertEqual(
                errorMessage,
                "persistence error: unexpected initialize failure"
            )
        }
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
            networkKind: NetworkKind.test
        )
        let changeDescriptorPub = try Descriptor(
            descriptor: "wpkh([9122d9e0/84'/1'/0']tpubDCYVtmaSaDzTxcgvoP5AHZNbZKZzrvoNH9KARep88vESc6MxRqAp4LmePc2eeGX6XUxBcdhAmkthWTDqygPz2wLAyHWisD299Lkdrj5egY6/1/*)#n4cuwhcy",
            networkKind: NetworkKind.test
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
