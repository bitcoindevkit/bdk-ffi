package org.bitcoindevkit

import kotlin.test.Test
import androidx.test.ext.junit.runners.AndroidJUnit4
import androidx.test.platform.app.InstrumentationRegistry
import kotlinx.coroutines.delay
import kotlinx.coroutines.launch
import kotlinx.coroutines.runBlocking
import org.junit.runner.RunWith
// import org.kotlinbitcointools.regtesttoolbox.regenv.RegEnv
import java.io.File
import kotlin.time.Duration.Companion.seconds

@RunWith(AndroidJUnit4::class)
class CbfSyncTest {
    private val conn: Persister = Persister.newInMemory()

    private val kyotoDataDir: String by lazy {
        val context = InstrumentationRegistry.getInstrumentation().targetContext
        val dir = File(context.filesDir, "kyoto_test_data")
        dir.mkdirs()
        dir.absolutePath
    }

    @Test
    fun syncWithKyotoClient() {
        runBlocking {
            // println("Hello, Compact Block Filters!")
            // val regtestEnv = RegEnv.connectTo(walletName = "faucet", username = "regtest", password = "password")

            val wallet: Wallet = Wallet.createSingle(
                descriptor = BIP86_DESCRIPTOR,
                network = Network.REGTEST,
                persister = conn
            )
            val newAddress = wallet.revealNextAddress(KeychainKind.EXTERNAL).address
            println("New address: $newAddress")
            delay(2.seconds)

            // regtestEnv.send(newAddress.toString(), 0.12345678, 2.0)
            // regtestEnv.mine(2)

            val ip: IpAddress = IpAddress.fromIpv4(10u, 0u, 2u, 2u)
            val peer1: Peer = Peer(ip, 18444u, false)
            val peers: List<Peer> = listOf(peer1)
            val node = CbfBuilder()
                .peers(peers)
                .connections(1u)
                .scanType(ScanType.Sync)
                .dataDir(kyotoDataDir)
                .build(wallet)

            println("Test is complete")
            val client = node.start()

            val warningJob = launch {
                try {
                    while (true) println("Warning: ${client.nextWarning()}")
                } catch (e: CbfException) { println("Warning channel closed: $e") }
            }
            val infoJob = launch {
                try {
                    while (true) println("Info: ${client.nextInfo()}")
                } catch (e: CbfException) { println("Info channel closed: $e") }
            }

            val update: Update = client.update()
            delay(8.seconds)
            wallet.applyUpdate(update)

            val balance = wallet.balance().total.toSat()
            assert(balance > 0uL)

            warningJob.cancel()
            infoJob.cancel()
            client.shutdown()
        }
    }
}
