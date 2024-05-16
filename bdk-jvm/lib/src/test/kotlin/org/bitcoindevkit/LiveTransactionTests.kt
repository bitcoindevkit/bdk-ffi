package org.bitcoindevkit

import kotlin.test.Test

private const val SIGNET_ESPLORA_URL = "http://signet.bitcoindevkit.net"
private const val TESTNET_ESPLORA_URL = "https://esplora.testnet.kuutamo.cloud"

class LiveTransactionTests {
    @Test
    fun testSyncedBalance() {
        val descriptor: Descriptor = Descriptor(
            "wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/0/*)",
            Network.SIGNET
        )
        val wallet: Wallet = Wallet.newNoPersist(descriptor, null, Network.SIGNET)
        val esploraClient: EsploraClient = EsploraClient(SIGNET_ESPLORA_URL)
        val fullScanRequest: FullScanRequest = wallet.startFullScan()
        val update = esploraClient.fullScan(fullScanRequest, 10uL, 1uL)
        wallet.applyUpdate(update)
        wallet.commit()
        println("Wallet balance: ${wallet.getBalance().total.toSat()}")

        assert(wallet.getBalance().total.toSat() > 0uL) {
            "Wallet balance must be greater than 0! Please send funds to ${wallet.revealNextAddress(KeychainKind.EXTERNAL).address.asString()} and try again."
        }

        val transaction: Transaction = wallet.transactions().first().transaction
        println("First transaction:")
        println("Txid: ${transaction.txid()}")
        println("Version: ${transaction.version()}")
        println("Total size: ${transaction.totalSize()}")
        println("Vsize: ${transaction.vsize()}")
        println("Weight: ${transaction.weight()}")
        println("Coinbase transaction: ${transaction.isCoinbase()}")
        println("Is explicitly RBF: ${transaction.isExplicitlyRbf()}")
        println("Inputs: ${transaction.input()}")
        println("Outputs: ${transaction.output()}")
    }
}
