package org.bitcoindevkit

import org.junit.Test
import androidx.test.ext.junit.runners.AndroidJUnit4
import org.junit.runner.RunWith
import kotlin.test.assertTrue

@RunWith(AndroidJUnit4::class)
class LiveTxBuilderTest {
    @Test
    fun testTxBuilder() {
        val descriptor = Descriptor("wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/0/*)", Network.TESTNET)
        val wallet = Wallet.newNoPersist(descriptor, null, Network.TESTNET)
        val esploraClient = EsploraClient("https://mempool.space/testnet/api")
        val update = esploraClient.fullScan(wallet, 10uL, 1uL)
        wallet.applyUpdate(update)
        println("Balance: ${wallet.getBalance().total}")

        assert(wallet.getBalance().total > 0uL)

        val recipient: Address = Address("tb1qrnfslnrve9uncz9pzpvf83k3ukz22ljgees989", Network.TESTNET)
        val psbt: PartiallySignedTransaction = TxBuilder()
            .addRecipient(recipient.scriptPubkey(), 4200uL)
            .feeRate(FeeRate.fromSatPerVb(2.0f))
            .finish(wallet)

        println(psbt.serialize())
        assertTrue(psbt.serialize().startsWith("cHNi"), "PSBT should start with 'cHNi'")
    }

    @Test
    fun complexTxBuilder() {
        val externalDescriptor = Descriptor("wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/0/*)", Network.TESTNET)
        val changeDescriptor = Descriptor("wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/1/*)", Network.TESTNET)
        val wallet = Wallet.newNoPersist(externalDescriptor, changeDescriptor, Network.TESTNET)
        val esploraClient = EsploraClient("https://mempool.space/testnet/api")
        val update = esploraClient.fullScan(wallet, 10uL, 1uL)
        wallet.applyUpdate(update)
        println("Balance: ${wallet.getBalance().total}")

        assert(wallet.getBalance().total > 0uL)

        val recipient1: Address = Address("tb1qrnfslnrve9uncz9pzpvf83k3ukz22ljgees989", Network.TESTNET)
        val recipient2: Address = Address("tb1qw2c3lxufxqe2x9s4rdzh65tpf4d7fssjgh8nv6", Network.TESTNET)
        val allRecipients: List<ScriptAmount> = listOf(
            ScriptAmount(recipient1.scriptPubkey(), 4200uL),
            ScriptAmount(recipient2.scriptPubkey(), 4200uL),
        )

        val psbt: PartiallySignedTransaction = TxBuilder()
            .setRecipients(allRecipients)
            .feeRate(FeeRate.fromSatPerVb(4.0f))
            .changePolicy(ChangeSpendPolicy.CHANGE_FORBIDDEN)
            .enableRbf()
            .finish(wallet)

        wallet.sign(psbt)
        assertTrue(psbt.serialize().startsWith("cHNi"), "PSBT should start with 'cHNi'")
    }
}
