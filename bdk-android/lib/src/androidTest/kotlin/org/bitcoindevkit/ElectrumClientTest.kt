package org.bitcoindevkit

import kotlin.test.Test
import androidx.test.ext.junit.runners.AndroidJUnit4
import org.junit.runner.RunWith

@RunWith(AndroidJUnit4::class)
class ElectrumClientTest {
    private val conn: Persister = Persister.newInMemory()

    @Test
    fun syncWithCallbackInspector() {
        val wallet: Wallet = Wallet.createSingle(
            descriptor = BIP86_DESCRIPTOR,
            network = Network.REGTEST,
            persister = conn
        )
        wallet.revealAddressesTo(KeychainKind.EXTERNAL, 16u)

        val electrumClient = ElectrumClient(ELECTRUM_REGTEST_URL)

        class SyncCallback : SyncScriptInspector {
            var totalSynced = 0

            override fun inspect(script: Script, total: ULong) {
                totalSynced++
                println("Inspecting script $script ($totalSynced out of a total of $total)")
            }
        }

        val syncRequest: SyncRequest = wallet.startSyncWithRevealedSpks().inspectSpks(SyncCallback()).build()
        electrumClient.sync(syncRequest, 10uL, true)

        // Logs
        // Inspecting script OP_PUSHNUM_1 OP_PUSHBYTES_32 51af34057eae9eb1b27271904a78e7adef43a062a01c65e8ff775188d0180ffa (1 out of a total of 17)
        // Inspecting script OP_PUSHNUM_1 OP_PUSHBYTES_32 089b36f682a8182b5beaae57fca90c7200f1269aa3deff27119e9c381a65328f (2 out of a total of 17)
        // Inspecting script OP_PUSHNUM_1 OP_PUSHBYTES_32 2bd1aa8f1b4438bb302c5a5185e9f2541addbf58103eb2c32c7c5eb8d0e38147 (3 out of a total of 17)
        // ...
    }
}
