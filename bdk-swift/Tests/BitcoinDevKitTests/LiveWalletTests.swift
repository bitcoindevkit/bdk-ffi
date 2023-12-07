import XCTest
@testable import BitcoinDevKit

final class LiveWalletTests: XCTestCase {
    func testSyncedBalance() throws {
        let descriptor = try Descriptor(
            descriptor: "wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/0/*)",
            network: Network.testnet
        )
        let wallet = try Wallet.newNoPersist(
            descriptor: descriptor,
            changeDescriptor: nil,
            network: .testnet
        )
        let esploraClient = EsploraClient(url: "https://mempool.space/testnet/api")
        let update = try esploraClient.scan(
            wallet: wallet,
            stopGap: 10,
            parallelRequests: 1
        )
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
        let wallet = try Wallet.newNoPersist(
            descriptor: descriptor,
            changeDescriptor: nil,
            network: .testnet
        )
        let esploraClient = EsploraClient(url: "https://mempool.space/testnet/api")
        let update = try esploraClient.scan(
            wallet: wallet,
            stopGap: 10,
            parallelRequests: 1
        )
        try wallet.applyUpdate(update: update)

        XCTAssertGreaterThan(wallet.getBalance().total, UInt64(0), "Wallet must have positive balance, please add funds")
        
        print("Balance: \(wallet.getBalance().total)")

        let recipient: Address = try Address(address: "tb1qrnfslnrve9uncz9pzpvf83k3ukz22ljgees989", network: .testnet)
        let psbt: PartiallySignedTransaction = try
            TxBuilder()
                .addRecipient(script: recipient.scriptPubkey(), amount: 4200)
                .feeRate(satPerVbyte: 2.0)
                .finish(wallet: wallet)

        print(psbt.serialize())
        XCTAssertTrue(psbt.serialize().hasPrefix("cHNi"), "PSBT should start with cHNI")
        
        let walletDidSign: Bool = try wallet.sign(psbt: psbt)
        XCTAssertTrue(walletDidSign, "Wallet did not sign transaction")
        
        let tx: Transaction = psbt.extractTx()
        print(tx.txid())
        try esploraClient.broadcast(transaction: tx)
    }
}
