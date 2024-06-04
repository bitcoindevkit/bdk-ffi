package org.bitcoindevkit

import kotlin.test.Test

private const val SIGNET_ELECTRUM_URL = "ssl://mempool.space:60602"

class LiveElectrumClientTest {
    @Test
    fun testSyncedBalance() {
        val descriptor: Descriptor = Descriptor(
            "wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/0/*)",
            Network.SIGNET
        )
        val wallet: Wallet = Wallet.newNoPersist(descriptor, null, Network.SIGNET)
        val electrumClient: ElectrumClient = ElectrumClient(SIGNET_ELECTRUM_URL)
        val fullScanRequest: FullScanRequest = wallet.startFullScan()
        val update = electrumClient.fullScan(fullScanRequest, 10uL, 10uL, false)
        wallet.applyUpdate(update)
        wallet.commit()
        println("Balance: ${wallet.getBalance().total.toSat()}")

        assert(wallet.getBalance().total.toSat() > 0uL) {
            "Wallet balance must be greater than 0! Please send funds to ${wallet.revealNextAddress(KeychainKind.EXTERNAL).address} and try again."
        }

        println("Transactions count: ${wallet.transactions().count()}")
        val transactions = wallet.transactions().take(3)
        for (tx in transactions) {
            val sentAndReceived = wallet.sentAndReceived(tx.transaction)
            println("Transaction: ${tx.transaction.txid()}")
            println("Sent ${sentAndReceived.sent.toSat()}")
            println("Received ${sentAndReceived.received.toSat()}")
        }
    }
}