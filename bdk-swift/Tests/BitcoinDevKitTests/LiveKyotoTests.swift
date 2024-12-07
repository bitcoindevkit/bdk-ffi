import XCTest
@testable import BitcoinDevKit

final class LiveKyotoTests: XCTestCase {
    private let descriptor = try! Descriptor(
        descriptor: "wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/0/*)",
        network: Network.signet
    )
    private let changeDescriptor = try! Descriptor(
        descriptor: "wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/1/*)",
        network: Network.signet
    )
    private let peer = IpAddress.fromIpv4(q1: 68, q2: 47, q3: 229, q4: 218)
    private let cwd = FileManager.default.currentDirectoryPath.appending("/temp")

    override func tearDownWithError() throws {
        let fileManager = FileManager.default
        if fileManager.fileExists(atPath: cwd) {
            try fileManager.removeItem(atPath: cwd)
        }
    }

    func testSuccessfullySyncs() async throws {
        let connection = try Connection.newInMemory()
        let wallet = try Wallet(
            descriptor: descriptor,
            changeDescriptor: changeDescriptor,
            network: Network.signet,
            connection: connection
        )
        let trusted_peer = Peer(address: peer, port: nil, v2Transport: false)
        let nodePair = try buildLightClient(wallet: wallet, peers: [trusted_peer], connections: 1, recoveryHeight: nil, dataDir: cwd)
        let client = nodePair.client
        let node = nodePair.node
        runNode(node: node)
        let update = await client.update(eventHandler: nil)
        if let update = update {
            try wallet.applyUpdate(update: update)
            let address = wallet.revealNextAddress(keychain: KeychainKind.external).address.description
            XCTAssertGreaterThan(
                wallet.balance().total.toSat(),
                UInt64(0),
                "Wallet must have positive balance, please send funds to \(address)"
            )
            print("Update applied correctly")
        } else {
            print("Update is nil. Ensure this test is ran infrequently.")
        }
    }
}
