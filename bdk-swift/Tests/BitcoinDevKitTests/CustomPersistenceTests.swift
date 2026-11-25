import XCTest
@testable import BitcoinDevKit

/// Keeps the aggregate `ChangeSet` in memory, the way an app-provided persister would store it.
private final class InMemoryPersistence: Persistence, @unchecked Sendable {
    private let lock = NSLock()
    private var aggregate: ChangeSet?
    private(set) var lastPersisted: ChangeSet?

    func initialize() throws -> ChangeSet {
        lock.lock()
        defer { lock.unlock() }
        return aggregate ?? ChangeSet()
    }

    func persist(changeset: ChangeSet) throws {
        lock.lock()
        defer { lock.unlock() }
        aggregate = aggregate.map { ChangeSet.fromMerge(left: $0, right: changeset) } ?? changeset
        lastPersisted = changeset
    }

    func aggregateSpkCache() -> [DescriptorId: [UInt32: Script]] {
        lock.lock()
        defer { lock.unlock() }
        return aggregate?.indexerChangeset().spkCache ?? [:]
    }
}

final class CustomPersistenceTests: XCTestCase {
    /// Number of script pubkeys the wallet derives past the last revealed index.
    private let lookahead: UInt32 = 5

    private let descriptor = try! Descriptor(
        descriptor: "wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/1h/0/*)",
        networkKind: NetworkKind.test
    )
    private let changeDescriptor = try! Descriptor(
        descriptor: "wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/1h/1/*)",
        networkKind: NetworkKind.test
    )

    private func createWallet(persister: Persister) throws -> Wallet {
        try Wallet.createWithParams(
            descriptor: descriptor,
            changeDescriptor: changeDescriptor,
            network: .signet,
            persister: persister,
            params: CreateParams(genesisHash: nil, lookahead: lookahead, useSpkCache: true)
        )
    }

    // Code written before `spkCache` existed still compiles, and gets an empty cache.
    func testIndexerChangeSetDefaultsToEmptySpkCache() {
        let changeset = IndexerChangeSet(lastRevealed: [descriptor.descriptorId(): 3])

        XCTAssertTrue(changeset.spkCache.isEmpty)
    }

    // The script pubkeys the wallet derives reach a custom persister, so it can store them.
    func testCustomPersistenceReceivesSpkCache() throws {
        let persistence = InMemoryPersistence()
        let persister = Persister.custom(persistence: persistence)
        let wallet = try createWallet(persister: persister)

        // The derived script pubkeys are handed over together with the next reveal.
        _ = wallet.revealNextAddress(keychain: .external)
        XCTAssertTrue(try wallet.persist(persister: persister))

        let spkCache = persistence.aggregateSpkCache()
        let expected: [(KeychainKind, DescriptorId, UInt32)] = [
            // Revealing index 0 keeps `lookahead` more script pubkeys derived past it.
            (.external, descriptor.descriptorId(), lookahead + 1),
            (.internal, changeDescriptor.descriptorId(), lookahead),
        ]
        for (keychain, descriptorId, count) in expected {
            let cached = try XCTUnwrap(spkCache[descriptorId], "no spk cache for \(keychain)")
            XCTAssertEqual(UInt32(cached.count), count)
            for (index, script) in cached {
                let address = wallet.peekAddress(keychain: keychain, index: index).address
                XCTAssertEqual(script.toBytes(), address.scriptPubkey().toBytes())
            }
        }
    }

    // A wallet loaded from a custom persister reuses the stored script pubkeys instead of deriving
    // them again, so the next reveal only stages the one new script pubkey.
    func testLoadingFromCustomPersistenceReusesSpkCache() throws {
        let persistence = InMemoryPersistence()
        let persister = Persister.custom(persistence: persistence)
        let wallet = try createWallet(persister: persister)
        _ = wallet.revealNextAddress(keychain: .external)
        XCTAssertTrue(try wallet.persist(persister: persister))

        let loaded = try Wallet.loadWithParams(
            descriptor: descriptor,
            changeDescriptor: changeDescriptor,
            persister: persister,
            params: LoadParams(
                checkNetwork: nil,
                checkGenesisHash: nil,
                lookahead: lookahead,
                useSpkCache: true
            )
        )
        let revealed = loaded.revealNextAddress(keychain: .external)
        XCTAssertTrue(try loaded.persist(persister: persister))

        let staged = try XCTUnwrap(persistence.lastPersisted).indexerChangeset().spkCache
        let stagedIndices = staged.flatMap { descriptorId, spks in
            spks.keys.map { (descriptorId, $0) }
        }
        XCTAssertEqual(stagedIndices.count, 1)
        XCTAssertEqual(stagedIndices.first?.0, descriptor.descriptorId())
        XCTAssertEqual(stagedIndices.first?.1, revealed.index + lookahead)
    }
}
