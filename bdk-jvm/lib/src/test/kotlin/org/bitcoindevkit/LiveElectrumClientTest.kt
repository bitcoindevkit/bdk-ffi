package org.bitcoindevkit

import kotlin.test.Test
import org.bitcoindevkit.bitcoinffi.Network

private const val SIGNET_ELECTRUM_URL = "ssl://mempool.space:60602"

class LiveElectrumClientTest {
    private val descriptor: Descriptor = Descriptor(
        "wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/0/*)",
        Network.SIGNET
    )
    private val changeDescriptor: Descriptor = Descriptor(
        "wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/1/*)",
        Network.SIGNET
    )

    @Test
    fun testSyncedBalance() {
        var conn: Connection = Connection.newInMemory()
        val wallet: Wallet = Wallet(descriptor, changeDescriptor, Network.SIGNET, conn)
        val electrumClient: ElectrumClient = ElectrumClient(SIGNET_ELECTRUM_URL)
        val fullScanRequest: FullScanRequest = wallet.startFullScan().build()
        val update = electrumClient.fullScan(fullScanRequest, 10uL, 10uL, false)
        wallet.applyUpdate(update)
        println("Balance: ${wallet.balance().total.toSat()}")

        assert(wallet.balance().total.toSat() > 0uL) {
            "Wallet balance must be greater than 0! Please send funds to ${wallet.revealNextAddress(KeychainKind.EXTERNAL).address} and try again."
        }

        println("Transactions count: ${wallet.transactions().count()}")
        val transactions = wallet.transactions().take(3)
        for (tx in transactions) {
            val sentAndReceived = wallet.sentAndReceived(tx.transaction)
            println("Transaction: ${tx.transaction.computeTxid()}")
            println("Sent ${sentAndReceived.sent.toSat()}")
            println("Received ${sentAndReceived.received.toSat()}")
        }
    }
}