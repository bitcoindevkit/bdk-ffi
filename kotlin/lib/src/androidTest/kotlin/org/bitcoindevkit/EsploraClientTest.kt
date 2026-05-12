package org.bitcoindevkit

import kotlin.test.Test
import androidx.test.ext.junit.runners.AndroidJUnit4
import kotlinx.coroutines.delay
import kotlinx.coroutines.runBlocking
import org.junit.runner.RunWith
import org.kotlinbitcointools.regtesttoolbox.regenv.RegEnv
import kotlin.time.Duration.Companion.milliseconds

@RunWith(AndroidJUnit4::class)
class EsploraClientTest {
    private val conn: Persister = Persister.newInMemory()

    @Test
    fun walletSyncWithEsploraClient() {
        runBlocking {
            val regtestEnv = RegEnv.connectTo(
                host = "10.0.2.2",
                walletName = "faucet",
                username = "regtest",
                password = "password",
            )

            val wallet: Wallet = Wallet.createSingle(
                descriptor = BIP86_DESCRIPTOR,
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
            assert(balance > 0uL) {
                "Balance should be greater than zero, but was $balance"
            }
        }
    }
}
