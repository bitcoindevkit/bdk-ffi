package org.bitcoindevkit

import kotlinx.coroutines.runBlocking
import kotlin.test.Test

class KyotoTest {
    private val descriptor: Descriptor = Descriptor("wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/0/*)", Network.SIGNET)
    private val changeDescriptor: Descriptor = Descriptor("wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/1/*)", Network.SIGNET)

    @Test
    fun testKyoto() {
        val wallet: Wallet = Wallet(descriptor, changeDescriptor, Network.SIGNET)
        val peers: List<Peer> = listOf(
            Peer.V4(23u, 137u, 57u, 100u)
        )
        val logger = KotlinKyotoLogger()

        val (node, client) = buildLightClient(
            wallet = wallet,
            peers = peers,
            connections = 1u,
            recoveryHeight = 170000u,
            dataDir = "./kyotodata/",
            logger = logger
        )
        runNode(node)

        runBlocking {
            println("Starting the sync")
            val update = client.update()
            require(update != null) { "Update should not be null" }
            println("Applying update")
            wallet.applyUpdate(update)
            val balance = wallet.balance().total.toSat()
            println("Balance: $balance")
            val transactions = wallet.transactions().take(3)
            for (tx in transactions) {
                println("Transaction: ${tx.transaction.computeTxid()}")
            }
        }
    }
}

class KotlinKyotoLogger: NodeMessageHandler {
    override fun handleDialog(dialog: String) {
        // println("Here's a dialog bud: $dialog")
    }

    override fun handleStateChange(state: NodeState) {
        println("Here's a state change bud: $state")
    }

    override fun handleWarning(warning: String) {
        println("Here's a warning bud: $warning")
    }
}
