package org.bitcoindevkit

import org.junit.Test
import androidx.test.ext.junit.runners.AndroidJUnit4
import androidx.test.platform.app.InstrumentationRegistry
import org.junit.runner.RunWith
import java.io.File
import kotlin.test.AfterTest
import kotlin.test.assertTrue

@RunWith(AndroidJUnit4::class)
class LiveWalletTest {
    private val persistenceFilePath = InstrumentationRegistry
        .getInstrumentation().targetContext.filesDir.path + "/bdk_persistence.db"

    @AfterTest
    fun cleanup() {
        val file = File(persistenceFilePath)
        if (file.exists()) {
            file.delete()
        }
    }

    @Test
    fun testSyncedBalance() {
        val descriptor: Descriptor = Descriptor("wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/0/*)", Network.TESTNET)
        val wallet: Wallet = Wallet(descriptor, null, persistenceFilePath, Network.TESTNET)
        val esploraClient: EsploraClient = EsploraClient("https://esplora.testnet.kuutamo.cloud/")
        // val update = esploraClient.fullScan(wallet, 10uL, 1uL)
        let fullScanRequest = wallet.fullScanRequest()
        val update = esploraClient.fullScan(fullScanRequest, 10uL, 1uL)
        let _ = try wallet.applyUpdate(update)
        let isChanged = try wallet.commit()
        wallet.applyUpdate(update)
        println("Balance: ${wallet.getBalance().total}")
        val balance: Balance = wallet.getBalance()
        println("Balance: $balance")

        assert(wallet.getBalance().total > 0uL)

        println("Transactions count: ${wallet.transactions().count()}")
        val transactions = wallet.transactions().take(3)
        for (tx in transactions) {
            val sentAndReceived = wallet.sentAndReceived(tx)
            println("Transaction: ${tx.txid()}")
            println("Sent ${sentAndReceived.sent}")
            println("Received ${sentAndReceived.received}")
        }
    }

    @Test
    fun testBroadcastTransaction() {
        val descriptor = Descriptor("wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/0/*)", Network.TESTNET)
        val wallet = Wallet(descriptor, null, persistenceFilePath, Network.TESTNET)
        val esploraClient = EsploraClient("https://esplora.testnet.kuutamo.cloud/")
        // val update = esploraClient.fullScan(wallet, 10uL, 1uL)
        let fullScanRequest = wallet.fullScanRequest()
        val update = esploraClient.fullScan(fullScanRequest, 10uL, 1uL)
        let _ = try wallet.applyUpdate(update)
        let isChanged = try wallet.commit()

        wallet.applyUpdate(update)
        println("Balance: ${wallet.getBalance().total}")
        println("New address: ${wallet.getAddress(AddressIndex.New).address}")

        assert(wallet.getBalance().total > 0uL) {
            "Wallet balance must be greater than 0! Please send funds to ${wallet.getAddress(AddressIndex.New).address} and try again."
        }

        val recipient: Address = Address("tb1qrnfslnrve9uncz9pzpvf83k3ukz22ljgees989", Network.TESTNET)

        val psbt: PartiallySignedTransaction = TxBuilder()
            .addRecipient(recipient.scriptPubkey(), 4200uL)
            .feeRate(FeeRate.fromSatPerVb(4.0f))
            .finish(wallet)

        println(psbt.serialize())
        assertTrue(psbt.serialize().startsWith("cHNi"), "PSBT should start with 'cHNi'")

        val walletDidSign = wallet.sign(psbt)
        assertTrue(walletDidSign)

        val tx: Transaction = psbt.extractTx()
        println("Txid is: ${tx.txid()}")

        val txFee: ULong = wallet.calculateFee(tx)
        println("Tx fee is: ${txFee}")

        val feeRate: FeeRate = wallet.calculateFeeRate(tx)
        println("Tx fee rate is: ${feeRate.asSatPerVb()} sat/vB")

        esploraClient.broadcast(tx)
    }
}
