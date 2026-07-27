package org.bitcoindevkit


import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertTrue
import kotlin.test.assertFalse
import androidx.test.ext.junit.runners.AndroidJUnit4
import kotlinx.coroutines.delay
import kotlinx.coroutines.runBlocking
import org.junit.runner.RunWith
import org.kotlinbitcointools.regtesttoolbox.regenv.RegEnv
import kotlin.time.Duration.Companion.milliseconds

@RunWith(AndroidJUnit4::class)
class PsbtTest {
    val conn: Persister = Persister.newInMemory()

    @Test
    fun signTaprootPsbt() {
        runBlocking {
            val regtestEnv = RegEnv.connectTo(
                host = "10.0.2.2",
                walletName = "faucet",
                username = "regtest",
                password = "password",
            )

            val wallet: Wallet = Wallet(
                descriptor = BIP86_DESCRIPTOR,
                changeDescriptor = BIP86_CHANGE_DESCRIPTOR,
                network = Network.REGTEST,
                persister = conn
            )
            val newAddress = wallet.revealNextAddress(KeychainKind.EXTERNAL).address

            val txidString = regtestEnv.send(newAddress.toString(), 0.12345678, 2.0)
            regtestEnv.mine(2)

            val esploraClient = EsploraClient(ESPLORA_REGTEST_URL)
            val txid = Txid.fromString(txidString)
            // Wait for the Esplora client to see the transaction. Try 5x per second for 20 seconds.
            for (i in 0..99) {
                if (esploraClient.getTx(txid) != null) break
                delay(200.milliseconds)
            }

            val fullScanRequest: FullScanRequest = wallet.startFullScan().build()
            val update = esploraClient.fullScan(fullScanRequest, 10uL, 1uL)

            wallet.applyUpdate(update)
            wallet.persist(conn)

            val balance = wallet.balance().total.toSat()

            val recipient: Address = Address("bcrt1q645m0j78v9pajdfp0g0w6wacl4v8s7mvrwsjx5", Network.REGTEST)

            val psbt: Psbt = TxBuilder()
                .addRecipient(recipient.scriptPubkey(), Amount.fromSat(4420uL))
                .feeRate(FeeRate.fromSatPerVb(22uL))
                .finish(wallet)
            val keyMap = BIP86_DESCRIPTOR.getKeyMap()
            val keyMapWrapper = KeyMapWrapper.from(keyMap)

            val signedPsbt = psbt.sign(keyMapWrapper)

            val inputs = signedPsbt.input()
            val tapKeySig = inputs[0].tapKeySig

            assertTrue(tapKeySig != null, "tapKeySig should not be null after signing the PSBT input")
        }
    }
}