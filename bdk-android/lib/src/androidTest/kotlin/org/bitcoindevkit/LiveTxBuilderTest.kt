package org.bitcoindevkit

import androidx.test.ext.junit.runners.AndroidJUnit4
import androidx.test.platform.app.InstrumentationRegistry
import org.junit.Test
import org.junit.runner.RunWith
import kotlin.test.AfterTest
import kotlin.test.assertTrue
import java.io.File
import org.bitcoindevkit.bitcoinffi.Network
import org.bitcoindevkit.bitcoinffi.Amount
import org.bitcoindevkit.bitcoinffi.FeeRate

private const val SIGNET_ESPLORA_URL = "http://signet.bitcoindevkit.net"
private const val TESTNET_ESPLORA_URL = "https://esplora.testnet.kuutamo.cloud"

@RunWith(AndroidJUnit4::class)
class LiveTxBuilderTest {
    private val persistenceFilePath = InstrumentationRegistry.getInstrumentation().targetContext.filesDir.path + "/bdk_persistence3.sqlite"
    private val descriptor = Descriptor("wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/0/*)", Network.SIGNET)
    private val changeDescriptor = Descriptor("wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/1/*)", Network.SIGNET)

    @AfterTest
    fun cleanup() {
        val file = File(persistenceFilePath)
        if (file.exists()) {
            file.delete()
        }
    }

    @Test
    fun testTxBuilder() {
        var conn: Connection = Connection.newInMemory()
        val wallet = Wallet(descriptor, changeDescriptor, Network.SIGNET, conn)
        val esploraClient: EsploraClient = EsploraClient(SIGNET_ESPLORA_URL)
        val fullScanRequest: FullScanRequest = wallet.startFullScan().build()
        val update = esploraClient.fullScan(fullScanRequest, 10uL, 1uL)
        wallet.applyUpdate(update)
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
    }

    @Test
    fun complexTxBuilder() {
        var conn: Connection = Connection.newInMemory()
        val wallet = Wallet(descriptor, changeDescriptor, Network.SIGNET, conn)
        val esploraClient: EsploraClient = EsploraClient(SIGNET_ESPLORA_URL)
        val fullScanRequest: FullScanRequest = wallet.startFullScan().build()
        val update = esploraClient.fullScan(fullScanRequest, 10uL, 1uL)
        wallet.applyUpdate(update)

        println("Balance: ${wallet.balance().total.toSat()}")

        assert(wallet.balance().total.toSat() > 0uL) {
            "Wallet balance must be greater than 0! Please send funds to ${wallet.revealNextAddress(KeychainKind.EXTERNAL).address} and try again."
        }

        val recipient1: Address = Address("tb1qrnfslnrve9uncz9pzpvf83k3ukz22ljgees989", Network.SIGNET)
        val recipient2: Address = Address("tb1qw2c3lxufxqe2x9s4rdzh65tpf4d7fssjgh8nv6", Network.SIGNET)
        val allRecipients: List<ScriptAmount> = listOf(
            ScriptAmount(recipient1.scriptPubkey(), Amount.fromSat(4200uL)),
            ScriptAmount(recipient2.scriptPubkey(), Amount.fromSat(4200uL)),
        )

        val psbt: Psbt = TxBuilder()
            .setRecipients(allRecipients)
            .feeRate(FeeRate.fromSatPerVb(4uL))
            .changePolicy(ChangeSpendPolicy.CHANGE_FORBIDDEN)
            .finish(wallet)

        wallet.sign(psbt)
        assertTrue(psbt.serialize().startsWith("cHNi"), "PSBT should start with 'cHNi'")
    }
}
