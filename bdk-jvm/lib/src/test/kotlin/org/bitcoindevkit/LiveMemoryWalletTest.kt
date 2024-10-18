package org.bitcoindevkit

import kotlin.test.Test
import org.bitcoindevkit.bitcoinffi.Script
import org.bitcoindevkit.bitcoinffi.Network

private const val SIGNET_ESPLORA_URL = "http://signet.bitcoindevkit.net"
private const val TESTNET_ESPLORA_URL = "https://esplora.testnet.kuutamo.cloud"

class LiveMemoryWalletTest {
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
        val descriptor: Descriptor = Descriptor(
            "wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/0/*)",
            Network.SIGNET
        )
        var conn: Connection = Connection.newInMemory()
        val wallet: Wallet = Wallet(descriptor, changeDescriptor, Network.SIGNET, conn)
        val esploraClient: EsploraClient = EsploraClient(SIGNET_ESPLORA_URL)
        val fullScanRequest: FullScanRequest = wallet.startFullScan().build()
        val update = esploraClient.fullScan(fullScanRequest, 10uL, 1uL)
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

    @Test
    fun testScriptInspector() {
        var conn: Connection = Connection.newInMemory()
        val wallet: Wallet = Wallet(descriptor, changeDescriptor, Network.SIGNET, conn)
        val esploraClient: EsploraClient = EsploraClient(SIGNET_ESPLORA_URL)

        val scriptInspector: FullScriptInspector = FullScriptInspector()
        val fullScanRequest: FullScanRequest = wallet.startFullScan().inspectSpksForAllKeychains(scriptInspector).build()
        val update = esploraClient.fullScan(fullScanRequest, 21uL, 1uL)

        wallet.applyUpdate(update)
        println("Balance: ${wallet.balance().total.toSat()}")

        assert(wallet.balance().total.toSat() > 0uL) {
            "Wallet balance must be greater than 0! Please send funds to ${wallet.revealNextAddress(KeychainKind.EXTERNAL).address} and try again."
        }
    }
}

class FullScriptInspector: FullScanScriptInspector {
    override fun inspect(keychain: KeychainKind, index: UInt, script: Script){
        println("Inspecting index $index for keychain $keychain")
    }
}
