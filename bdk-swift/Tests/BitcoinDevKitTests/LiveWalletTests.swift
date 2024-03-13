import XCTest
@testable import BitcoinDevKit

final class LiveWalletTests: XCTestCase {
    override func tearDownWithError() throws {
        let fileManager = FileManager.default
        let dbFileName = "bdk_persistence.db"
        let documentDirectory = fileManager.urls(for: .documentDirectory, in: .userDomainMask).first!
        let dbFilePath: URL = documentDirectory.appendingPathComponent(dbFileName)

        if fileManager.fileExists(atPath: dbFilePath.path) {
            try fileManager.removeItem(at: dbFilePath)
        }
    }

    func testSyncedBalance() throws {
        let descriptor = try Descriptor(
            descriptor: "wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/0/*)",
            network: Network.testnet
        )
        let wallet = try Wallet(
            descriptor: descriptor,
            changeDescriptor: nil,
            persistenceBackendPath: "bdk_persistence.db",
            network: .testnet
        )
        let esploraClient = EsploraClient(url: "https://esplora.testnet.kuutamo.cloud/")
//        let update = try esploraClient.fullScan(
//            wallet: wallet,
//            stopGap: 10,
//            parallelRequests: 1
//        )
        let fullScanRequest = wallet.fullScanRequest()
        let update = try esploraClient.fullScan(
            request: fullScanRequest,
            stopGap: UInt64(150),
            parallelRequests: UInt64(5)
        )
        let _ = try wallet.applyUpdate(update: update)
        let isChanged = try wallet.commit()
        try wallet.applyUpdate(update: update)

        XCTAssertGreaterThan(wallet.getBalance().total, UInt64(0))

        print("Transactions count: \(wallet.transactions().count)")
        let transactions = wallet.transactions().prefix(3)
        for tx in transactions {
            let sentAndReceived = wallet.sentAndReceived(tx: tx)
            print("Transaction: \(tx.txid())")
            print("Sent \(sentAndReceived.sent)")
            print("Received \(sentAndReceived.received)")
        }
    }
    
    func testBroadcastTransaction() throws {
        let descriptor = try Descriptor(
            descriptor: "wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/0/*)",
            network: Network.testnet
        )
        let wallet = try Wallet(
            descriptor: descriptor,
            changeDescriptor: nil,
            persistenceBackendPath: "bdk_persistence.db",
            network: .testnet
        )
        let esploraClient = EsploraClient(url: "https://esplora.testnet.kuutamo.cloud/")
//        let update = try esploraClient.fullScan(
//            wallet: wallet,
//            stopGap: 10,
//            parallelRequests: 1
//        )
        let fullScanRequest = wallet.fullScanRequest()
        let update = try esploraClient.fullScan(
            request: fullScanRequest,
            stopGap: UInt64(150),
            parallelRequests: UInt64(5)
        )
        let _ = try wallet.applyUpdate(update: update)
        let isChanged = try wallet.commit()
        try wallet.applyUpdate(update: update)

        XCTAssertGreaterThan(wallet.getBalance().total, UInt64(0), "Wallet must have positive balance, please add funds")

        print("Balance: \(wallet.getBalance().total)")

        let recipient: Address = try Address(address: "tb1qrnfslnrve9uncz9pzpvf83k3ukz22ljgees989", network: .testnet)
        let psbt: PartiallySignedTransaction = try
            TxBuilder()
                .addRecipient(script: recipient.scriptPubkey(), amount: 4200)
                .feeRate(feeRate: FeeRate.fromSatPerVb(satPerVb: 2.0))
                .finish(wallet: wallet)

        print(psbt.serialize())
        XCTAssertTrue(psbt.serialize().hasPrefix("cHNi"), "PSBT should start with cHNI")

//        let walletDidSign: Bool = try wallet.sign(psbt: psbt)
//        XCTAssertTrue(walletDidSign, "Wallet did not sign transaction")
//
//        let tx: Transaction = psbt.extractTx()
//        print(tx.txid())
//        let fee: UInt64 = try wallet.calculateFee(tx: tx)
//        print("Transaction Fee: \(fee)")
//        let feeRate: FeeRate = try wallet.calculateFeeRate(tx: tx)
//        print("Transaction Fee Rate: \(feeRate.asSatPerVb()) sat/vB")
//
//        try esploraClient.broadcast(transaction: tx)
    }
}
