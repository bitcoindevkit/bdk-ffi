import XCTest
@testable import BitcoinDevKit

private let SIGNET_ESPLORA_URL = "http://signet.bitcoindevkit.net"
private let TESTNET_ESPLORA_URL = "https://esplora.testnet.kuutamo.cloud"

final class LiveTxBuilderTests: XCTestCase {
    private let descriptor = try! Descriptor(
    descriptor: "wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/1h/0/*)", 
    network: Network.signet
    )
    private let changeDescriptor = try! Descriptor(
        descriptor: "wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/1h/1/*)", 
        network: Network.signet
    )
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

    func testTxBuilder() throws {
        let connection = try Connection.newInMemory()
        let wallet = try Wallet(
            descriptor: descriptor,
            changeDescriptor: changeDescriptor,
            network: .signet,
            connection: connection
        )
        let esploraClient = EsploraClient(url: SIGNET_ESPLORA_URL)
        let fullScanRequest: FullScanRequest = try wallet.startFullScan().build()
        let update = try esploraClient.fullScan(
            request: fullScanRequest,
            stopGap: 10,
            parallelRequests: 1
        )
        try wallet.applyUpdate(update: update)
        let address = wallet.revealNextAddress(keychain: KeychainKind.external).address.description

        XCTAssertGreaterThan(
            wallet.balance().total.toSat(),
            UInt64(0),
            "Wallet must have positive balance, please send funds to \(address)"
        )

        let recipient: Address = try Address(address: "tb1qrnfslnrve9uncz9pzpvf83k3ukz22ljgees989", network: .signet)
        let psbt: Psbt = try TxBuilder()
            .addRecipient(script: recipient.scriptPubkey(), amount: Amount.fromSat(fromSat: 4200))
            .feeRate(feeRate: FeeRate.fromSatPerVb(satPerVb: 2))
            .finish(wallet: wallet)

        print(psbt.serialize())
        XCTAssertTrue(psbt.serialize().hasPrefix("cHNi"), "PSBT should start with cHNI")
    }

    func testComplexTxBuilder() throws {
        let connection = try Connection.newInMemory()
        let wallet = try Wallet(
            descriptor: descriptor,
            changeDescriptor: changeDescriptor,
            network: .signet,
            connection: connection
        )
        let esploraClient = EsploraClient(url: SIGNET_ESPLORA_URL)
        let fullScanRequest: FullScanRequest = try wallet.startFullScan().build()
        let update = try esploraClient.fullScan(
            request: fullScanRequest,
            stopGap: 10,
            parallelRequests: 1
        )
        try wallet.applyUpdate(update: update)
        let address = wallet.revealNextAddress(keychain: KeychainKind.external).address.description

        XCTAssertGreaterThan(
            wallet.balance().total.toSat(),
            UInt64(0),
            "Wallet must have positive balance, please send funds to \(address)"
        )

        let recipient1: Address = try Address(address: "tb1qrnfslnrve9uncz9pzpvf83k3ukz22ljgees989", network: .signet)
        let recipient2: Address = try Address(address: "tb1qw2c3lxufxqe2x9s4rdzh65tpf4d7fssjgh8nv6", network: .signet)
        let allRecipients: [ScriptAmount] = [
            ScriptAmount(script: recipient1.scriptPubkey(), amount: Amount.fromSat(fromSat: 4200)),
            ScriptAmount(script: recipient2.scriptPubkey(), amount: Amount.fromSat(fromSat: 4200))
        ]

        let psbt: Psbt = try TxBuilder()
            .setRecipients(recipients: allRecipients)
            .feeRate(feeRate: FeeRate.fromSatPerVb(satPerVb: 4))
            .finish(wallet: wallet)

        let _ = try! wallet.sign(psbt: psbt)

        XCTAssertTrue(psbt.serialize().hasPrefix("cHNi"), "PSBT should start with cHNI")
    }
}
