import XCTest
@testable import BitcoinDevKit

private let SIGNET_ESPLORA_URL = "https://blockstream.info/signet/api/"
private let TESTNET_ESPLORA_URL = "https://esplora.testnet.kuutamo.cloud"


final class LiveTransactionTests: XCTestCase {
    private let descriptor = try! Descriptor(
    descriptor: "wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/1h/0/*)", 
    network: Network.signet
    )
    private let changeDescriptor = try! Descriptor(
        descriptor: "wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/1h/1/*)", 
        network: Network.signet
    )

    func testSyncedBalance() throws {
        let persister = try Persister.newInMemory()
        let wallet = try Wallet(
            descriptor: descriptor,
            changeDescriptor: changeDescriptor,
            network: .signet,
            persister: persister
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
            
        guard let transaction = wallet.transactions().first?.transaction else {
            print("No transactions found")
            return
        }
        print("First transaction:")
        print("Txid: \(transaction.computeTxid())")
        print("Version: \(transaction.version())")
        print("Total size: \(transaction.totalSize())")
        print("Vsize: \(transaction.vsize())")
        print("Weight: \(transaction.weight())")
        print("Coinbase transaction: \(transaction.isCoinbase())")
        print("Is explicitly RBF: \(transaction.isExplicitlyRbf())")
        print("Inputs: \(transaction.input())")
        print("Outputs: \(transaction.output())")

    }
}
