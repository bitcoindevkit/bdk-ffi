import XCTest
@testable import BitcoinDevKit

final class CustomSyncTests: XCTestCase {
    private let descriptor = try! Descriptor(
        descriptor: "wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/1h/0/*)",
        networkKind: NetworkKind.test
    )
    private let changeDescriptor = try! Descriptor(
        descriptor: "wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/1h/1/*)",
        networkKind: NetworkKind.test
    )

    private func walletWithRevealedAddresses(upTo index: UInt32) throws -> Wallet {
        let wallet = try Wallet(
            descriptor: descriptor,
            changeDescriptor: changeDescriptor,
            network: .signet,
            persister: try Persister.newInMemory()
        )
        _ = wallet.revealAddressesTo(keychain: KeychainKind.external, index: index)
        _ = wallet.revealAddressesTo(keychain: KeychainKind.`internal`, index: index)
        return wallet
    }

    // A request can be built from a specific set of revealed indices on either keychain, instead of
    // every revealed script of the wallet. The request is then passed to a client as usual.
    func testSyncRequestFromSpkIndices() throws {
        let wallet = try walletWithRevealedAddresses(upTo: 20)

        let request: SyncRequest = try wallet
            .startSyncWithSpkIndices(externalIndices: [0, 5, 20], internalIndices: [3])
            .build()

        XCTAssertNotNil(request)
    }

    // A keychain left out of the call contributes nothing to the request, so a request can cover
    // the internal keychain alone.
    func testSyncRequestFromInternalIndicesOnly() throws {
        let wallet = try walletWithRevealedAddresses(upTo: 20)

        let request: SyncRequest = try wallet
            .startSyncWithSpkIndices(internalIndices: [0, 1])
            .build()

        XCTAssertNotNil(request)
    }

    // Asking for the last few revealed addresses of each keychain is a short way to catch up on the
    // most recently handed out addresses.
    func testSyncRequestFromLastRevealedSpks() throws {
        let wallet = try walletWithRevealedAddresses(upTo: 20)

        let request: SyncRequest = try wallet
            .startSyncWithLastRevealedSpks(externalCount: 5, internalCount: 2)
            .build()

        XCTAssertNotNil(request)
    }

    // Counts above what the wallet has revealed are not an error: the request simply carries every
    // revealed script of that keychain.
    func testLastRevealedSpksAboveRevealedCountIsAllowed() throws {
        let wallet = try walletWithRevealedAddresses(upTo: 2)

        XCTAssertNoThrow(
            try wallet.startSyncWithLastRevealedSpks(externalCount: 50, internalCount: 50)
        )
    }

    // An index past the last revealed one is refused: the wallet's indexer does not track it, so
    // any transaction found there would not be recognized as the wallet's own.
    func testSyncWithUnrevealedIndexThrows() throws {
        let wallet = try walletWithRevealedAddresses(upTo: 5)

        XCTAssertThrowsError(
            try wallet.startSyncWithSpkIndices(externalIndices: [0, 6])
        ) { error in
            guard case let SpkIndexError.NotRevealed(keychain, index, lastRevealed) = error else {
                return XCTFail("Expected a NotRevealed error, got \(error)")
            }
            XCTAssertEqual(keychain, KeychainKind.external)
            XCTAssertEqual(index, 6)
            XCTAssertEqual(lastRevealed, 5)
            XCTAssertEqual(
                (error as! SpkIndexError).description,
                "index 6 of the External keychain has not been revealed, the last revealed index is 5"
            )
        }
    }

    // A wallet which has not revealed anything yet has no index to sync.
    func testSyncOnWalletWithoutRevealedAddressesThrows() throws {
        let wallet = try Wallet(
            descriptor: descriptor,
            changeDescriptor: changeDescriptor,
            network: .signet,
            persister: try Persister.newInMemory()
        )

        XCTAssertThrowsError(
            try wallet.startSyncWithSpkIndices(externalIndices: [0])
        ) { error in
            guard case let SpkIndexError.NoneRevealed(keychain) = error else {
                return XCTFail("Expected a NoneRevealed error, got \(error)")
            }
            XCTAssertEqual(keychain, KeychainKind.external)
        }
    }

    // A request which would carry no script at all is refused rather than quietly turning into a
    // sync that only refreshes the chain tip.
    func testSyncWithoutAnySpkThrows() throws {
        let wallet = try walletWithRevealedAddresses(upTo: 5)

        XCTAssertThrowsError(try wallet.startSyncWithSpkIndices()) { error in
            guard case SpkIndexError.NoSpksRequested = error else {
                return XCTFail("Expected a NoSpksRequested error, got \(error)")
            }
            XCTAssertEqual(
                (error as! SpkIndexError).description,
                "no spk was requested, the sync request would be empty"
            )
        }

        XCTAssertThrowsError(
            try wallet.startSyncWithSpkIndices(externalIndices: [], internalIndices: [])
        )
        XCTAssertThrowsError(try wallet.startSyncWithLastRevealedSpks())
        XCTAssertThrowsError(
            try wallet.startSyncWithLastRevealedSpks(externalCount: 0, internalCount: 0)
        )
    }

    // Counts are lenient about asking for more than was revealed, but a keychain which has
    // revealed nothing at all would produce an empty request, so it fails.
    func testLastRevealedSpksOnWalletWithoutRevealedAddressesThrows() throws {
        let wallet = try Wallet(
            descriptor: descriptor,
            changeDescriptor: changeDescriptor,
            network: .signet,
            persister: try Persister.newInMemory()
        )

        XCTAssertThrowsError(try wallet.startSyncWithLastRevealedSpks(externalCount: 5)) { error in
            guard case let SpkIndexError.NoneRevealed(keychain) = error else {
                return XCTFail("Expected a NoneRevealed error, got \(error)")
            }
            XCTAssertEqual(keychain, KeychainKind.external)
        }
    }

    // A single-descriptor wallet has no internal keychain, so asking for internal scripts fails
    // rather than quietly syncing external ones.
    func testSyncInternalIndicesOnSingleDescriptorWalletThrows() throws {
        let wallet = try Wallet.createSingle(
            descriptor: descriptor,
            network: .signet,
            persister: try Persister.newInMemory()
        )
        _ = wallet.revealAddressesTo(keychain: KeychainKind.external, index: 5)

        XCTAssertThrowsError(
            try wallet.startSyncWithSpkIndices(externalIndices: [0], internalIndices: [0])
        ) { error in
            guard case let SpkIndexError.KeychainNotFound(keychain) = error else {
                return XCTFail("Expected a KeychainNotFound error, got \(error)")
            }
            XCTAssertEqual(keychain, KeychainKind.`internal`)
        }

        XCTAssertThrowsError(
            try wallet.startSyncWithLastRevealedSpks(externalCount: 3, internalCount: 1)
        )

        // Asking for nothing on the missing keychain is fine.
        XCTAssertNoThrow(
            try wallet.startSyncWithSpkIndices(externalIndices: [0])
        )
        XCTAssertNoThrow(
            try wallet.startSyncWithLastRevealedSpks(externalCount: 3)
        )
    }
}
