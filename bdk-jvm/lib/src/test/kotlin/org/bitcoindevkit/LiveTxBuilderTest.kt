package org.bitcoindevkit

import java.io.File
import kotlin.test.AfterTest
import kotlin.test.Test
import kotlin.test.assertTrue

class LiveTxBuilderTest {
    private val persistenceFilePath = run {
        val currentDirectory = System.getProperty("user.dir")
        "$currentDirectory/bdk_persistence.db"
    }

    @AfterTest
    fun cleanup() {
        val file = File(persistenceFilePath)
        if (file.exists()) {
            file.delete()
        }
    }

    @Test
    fun testTxBuilder() {
        val descriptor = Descriptor("wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/0/*)", Network.TESTNET)
        val wallet = Wallet(descriptor, null, persistenceFilePath, Network.SIGNET)
        // val esploraClient = EsploraClient("https://esplora.testnet.kuutamo.cloud/")
        val esploraClient = EsploraClient("http://signet.bitcoindevkit.net")
        val fullScanRequest: FullScanRequest = wallet.startFullScan()
        val update = esploraClient.fullScan(fullScanRequest, 10uL, 1uL)
        wallet.applyUpdate(update)
        wallet.commit()
        println("Balance: ${wallet.getBalance().total}")

        assert(wallet.getBalance().total > 0uL)

        val recipient: Address = Address("tb1qrnfslnrve9uncz9pzpvf83k3ukz22ljgees989", Network.SIGNET)
        val psbt: Psbt = TxBuilder()
            .addRecipient(recipient.scriptPubkey(), 4200uL)
            .feeRate(FeeRate.fromSatPerVb(2uL))
            .finish(wallet)

        println(psbt.serialize())

        assertTrue(psbt.serialize().startsWith("cHNi"), "PSBT should start with 'cHNi'")
    }

    @Test
    fun complexTxBuilder() {
        val externalDescriptor = Descriptor("wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/0/*)", Network.TESTNET)
        val changeDescriptor = Descriptor("wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/1/*)", Network.TESTNET)
        val wallet = Wallet(externalDescriptor, changeDescriptor, persistenceFilePath, Network.SIGNET)
        // val esploraClient = EsploraClient("https://esplora.testnet.kuutamo.cloud/")
        val esploraClient = EsploraClient("http://signet.bitcoindevkit.net")
        val fullScanRequest: FullScanRequest = wallet.startFullScan()
        val update = esploraClient.fullScan(fullScanRequest, 10uL, 1uL)
        wallet.applyUpdate(update)
        wallet.commit()
        println("Balance: ${wallet.getBalance().total}")

        assert(wallet.getBalance().total > 0uL)

        val recipient1: Address = Address("tb1qrnfslnrve9uncz9pzpvf83k3ukz22ljgees989", Network.SIGNET)
        val recipient2: Address = Address("tb1qw2c3lxufxqe2x9s4rdzh65tpf4d7fssjgh8nv6", Network.SIGNET)
        val allRecipients: List<ScriptAmount> = listOf(
            ScriptAmount(recipient1.scriptPubkey(), 4200uL),
            ScriptAmount(recipient2.scriptPubkey(), 4200uL),
        )

        val psbt: Psbt = TxBuilder()
            .setRecipients(allRecipients)
            .feeRate(FeeRate.fromSatPerVb(4uL))
            .changePolicy(ChangeSpendPolicy.CHANGE_FORBIDDEN)
            .enableRbf()
            .finish(wallet)

        wallet.sign(psbt)
        assertTrue(psbt.serialize().startsWith("cHNi"), "PSBT should start with 'cHNi'")
    }
}
