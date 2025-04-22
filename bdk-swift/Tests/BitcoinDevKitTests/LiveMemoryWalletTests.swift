import XCTest
@testable import BitcoinDevKit

private let SIGNET_ESPLORA_URL = "http://signet.bitcoindevkit.net"
private let TESTNET_ESPLORA_URL = "https://esplora.testnet.kuutamo.cloud"

final class LiveMemoryWalletTests: XCTestCase {
    private let descriptor = try! Descriptor(
        descriptor: "wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/1h/0/*)", 
        network: Network.signet
    )
    private let changeDescriptor = try! Descriptor(
        descriptor: "wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/1h/1/*)", 
        network: Network.signet
    )

    func testSyncedBalance() throws {
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

        print("Transactions count: \(wallet.transactions().count)")
        let transactions = wallet.transactions().prefix(3)
        for tx in transactions {
            let sentAndReceived = wallet.sentAndReceived(tx: tx.transaction)
            print("Transaction: \(tx.transaction.computeTxid())")
            print("Sent \(sentAndReceived.sent.toSat())")
            print("Received \(sentAndReceived.received.toSat())")
        }
    }

    func testScriptInspector() throws {
        let connection = try Connection.newInMemory()
        let wallet = try Wallet(
            descriptor: descriptor,
            changeDescriptor: changeDescriptor,
            network: .signet,
            connection: connection
        )
        let esploraClient = EsploraClient(url: SIGNET_ESPLORA_URL)
        let scriptInspector = FullScriptInspector()
        let fullScanRequest = try wallet.startFullScan().inspectSpksForAllKeychains(inspector: scriptInspector).build()

        let update = try esploraClient.fullScan(
            request: fullScanRequest,
            stopGap: 21,
            parallelRequests: 1
        )
        try wallet.applyUpdate(update: update)
        let address = wallet.revealNextAddress(keychain: KeychainKind.external).address.description

        XCTAssertGreaterThan(
            wallet.balance().total.toSat(),
            UInt64(0),
            "Wallet must have positive balance, please send funds to \(address)"
        )
    }

}

final class FullScriptInspector: FullScanScriptInspector {
    func inspect(keychain: KeychainKind, index: UInt32, script: Script) {
        print("Inspecting index \(index) for keychain \(keychain)")
    }
}
