package org.bitcoindevkit

import kotlin.test.AfterTest
import kotlin.test.Test
import kotlin.test.assertTrue
import java.io.File
import org.bitcoindevkit.bitcoinffi.Amount
import org.bitcoindevkit.bitcoinffi.Network
import org.bitcoindevkit.bitcoinffi.FeeRate

private const val SIGNET_ESPLORA_URL = "http://signet.bitcoindevkit.net"
private const val TESTNET_ESPLORA_URL = "https://esplora.testnet.kuutamo.cloud"

class LiveWalletTest {
    private val persistenceFilePath = run {
        val currentDirectory = System.getProperty("user.dir")
        "$currentDirectory/bdk_persistence.sqlite"
    }
    private val descriptor: Descriptor = Descriptor(
        "wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/0/*)",
        Network.SIGNET
    )
    private val changeDescriptor: Descriptor = Descriptor(
        "wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/1/*)",
        Network.SIGNET
    )

    @AfterTest
    fun cleanup() {
        val file = File(persistenceFilePath)
        if (file.exists()) {
            file.delete()
        }
    }

    @Test
    fun testSyncedBalance() {
        val connection = Connection(persistenceFilePath)
        val wallet: Wallet = Wallet(descriptor, changeDescriptor, Network.SIGNET, connection)
        val esploraClient: EsploraClient = EsploraClient(SIGNET_ESPLORA_URL)
        val fullScanRequest: FullScanRequest = wallet.startFullScan().build()
        val update = esploraClient.fullScan(fullScanRequest, 10uL, 1uL)
        wallet.applyUpdate(update)
        wallet.persist(connection)
        println("Balance: ${wallet.balance().total.toSat()}")

        assert(wallet.balance().total.toSat() > 0uL) {
            "Wallet balance must be greater than 0! Please send funds to ${wallet.revealNextAddress(KeychainKind.EXTERNAL).address} and try again."
        }

        println("Transactions count: ${wallet.transactions().count()}")
        val transactions = wallet.transactions().take(3)
        for (tx in transactions) {
            val sentAndReceived = wallet.sentAndReceived(tx.transaction)
            println("Transaction: ${tx.transaction.computeTxid()}")
            println("Sent ${sentAndReceived.sent}")
            println("Received ${sentAndReceived.received}")
        }
    }

    @Test
    fun testBroadcastTransaction() {
        val connection = Connection(persistenceFilePath)
        val descriptor = Descriptor("wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/0/*)", Network.SIGNET)
        val wallet: Wallet = Wallet(descriptor, changeDescriptor, Network.SIGNET, connection)
        val esploraClient = EsploraClient(SIGNET_ESPLORA_URL)
        val fullScanRequest: FullScanRequest = wallet.startFullScan().build()
        val update = esploraClient.fullScan(fullScanRequest, 10uL, 1uL)
        wallet.applyUpdate(update)
        wallet.persist(connection)
        println("Balance: ${wallet.balance().total.toSat()}")

        assert(wallet.balance().total.toSat() > 0uL) {
            "Wallet balance must be greater than 0! Please send funds to ${wallet.revealNextAddress(KeychainKind.EXTERNAL).address} and try again."
        }

        val recipient: Address = Address("tb1qrnfslnrve9uncz9pzpvf83k3ukz22ljgees989", Network.SIGNET)

        val psbt: Psbt = TxBuilder()
            .addRecipient(recipient.scriptPubkey(), Amount.fromSat(4200uL))
            .feeRate(FeeRate.fromSatPerVb(2uL))
            .finish(wallet)

        println(psbt.serialize())
        assertTrue(psbt.serialize().startsWith("cHNi"), "PSBT should start with 'cHNi'")

        val walletDidSign = wallet.sign(psbt)
        assertTrue(walletDidSign)

        val tx: Transaction = psbt.extractTx()
        println("Txid is: ${tx.computeTxid()}")

        val txFee: Amount = wallet.calculateFee(tx)
        println("Tx fee is: ${txFee}")

        val feeRate: FeeRate = wallet.calculateFeeRate(tx)
        println("Tx fee rate is: ${feeRate.toSatPerVbCeil()} sat/vB")

        esploraClient.broadcast(tx)
    }
}
