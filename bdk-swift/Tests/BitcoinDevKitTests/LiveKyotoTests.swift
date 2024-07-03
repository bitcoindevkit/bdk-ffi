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
        let trustedPeer = Peer(address: peer, port: nil, v2Transport: false)
        let lightClient = try LightClientBuilder()
            .peers(peers: [trustedPeer])
            .connections(connections: 1)
            .scanType(scanType: ScanType.new)
            .dataDir(dataDir: cwd)
            .build(wallet: wallet)
        let client = lightClient.client
        let node = lightClient.node
        node.run()
        Task {
            while true {
                let log = await client.nextLog()
                print("\(log)")
            }
        }
        let update = await client.update()
        if let update = update {
            try wallet.applyUpdate(update: update)
            let address = wallet.revealNextAddress(keychain: KeychainKind.external).address.description
            XCTAssertGreaterThan(
                wallet.balance().total.toSat(),
                UInt64(0),
                "Wallet must have positive balance, please send funds to \(address)"
            )
            print("Update applied correctly")
            try await client.shutdown()
        } else {
            print("Update is nil. Ensure this test is ran infrequently.")
        }
    }
}
