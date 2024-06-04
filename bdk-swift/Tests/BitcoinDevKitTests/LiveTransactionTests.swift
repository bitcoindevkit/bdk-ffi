import XCTest
@testable import BitcoinDevKit

private let SIGNET_ESPLORA_URL = "http://signet.bitcoindevkit.net"
private let TESTNET_ESPLORA_URL = "https://esplora.testnet.kuutamo.cloud"

final class LiveTransactionTests: XCTestCase {
    func testSyncedBalance() throws {
        let descriptor = try Descriptor(
            descriptor: "wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/0/*)",
            network: Network.signet
        )
        let wallet = try Wallet.newNoPersist(
            descriptor: descriptor,
            changeDescriptor: nil,
            network: .signet
        )
        let esploraClient = EsploraClient(url: SIGNET_ESPLORA_URL)
        let fullScanRequest: FullScanRequest = wallet.startFullScan()
        let update = try esploraClient.fullScan(
            fullScanRequest: fullScanRequest,
            stopGap: 10,
            parallelRequests: 1
        )
        try wallet.applyUpdate(update: update)
        let _ = try wallet.commit()
        let address = try wallet.revealNextAddress(keychain: KeychainKind.external).address.description

        XCTAssertGreaterThan(
            wallet.getBalance().total.toSat(),
            UInt64(0),
            "Wallet must have positive balance, please send funds to \(address)"
        )
            
        guard let transaction = wallet.transactions().first?.transaction else {
            print("No transactions found")
            return
        }
        print("First transaction:")
        print("Txid: \(transaction.txid())")
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
