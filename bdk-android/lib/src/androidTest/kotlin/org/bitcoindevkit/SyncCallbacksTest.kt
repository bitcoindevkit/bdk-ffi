package org.bitcoindevkit

import android.util.Log
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertTrue
import androidx.test.ext.junit.runners.AndroidJUnit4
import org.junit.runner.RunWith

@RunWith(AndroidJUnit4::class)
class SyncCallbacksTest {
    private val conn1: Persister = Persister.newInMemory()
    private val conn2: Persister = Persister.newInMemory()

    @Test
    fun syncWithCallbackInspector() {
        class SyncCallback : SyncScriptInspector {
            var totalSynced = 0
            var lastTotal: ULong = 0u

            override fun inspect(script: Script, total: ULong) {
                totalSynced++
                lastTotal = total
                println("Inspecting script $script ($totalSynced out of a total of $total)")
            }
        }

        val wallet: Wallet = Wallet.createSingle(
            descriptor = BIP86_DESCRIPTOR,
            network = Network.REGTEST,
            persister = conn1
        )
        wallet.revealAddressesTo(KeychainKind.EXTERNAL, 16u)
        val electrumClient = ElectrumClient(ELECTRUM_REGTEST_URL)
        val syncCallback = SyncCallback()
        val syncRequest: SyncRequest = wallet.startSyncWithRevealedSpks()
            .inspectSpks(syncCallback)
            .build()

        electrumClient.sync(
            request = syncRequest,
            batchSize = 10u,
            fetchPrevTxouts = true
        )

        // Revealing addresses up to index 16 produces 17 revealed scripts (indices 0..16),
        // each of which should be passed to the inspector exactly once.
        assertEquals(17, syncCallback.totalSynced, "Inspector should have been called once per revealed script")
        assertEquals(syncCallback.totalSynced.toULong(), syncCallback.lastTotal, "Number of inspected scripts should match the reported total")

        // Logs
        // Inspecting script OP_PUSHNUM_1 OP_PUSHBYTES_32 51af34057eae9eb1b27271904a78e7adef43a062a01c65e8ff775188d0180ffa (1 out of a total of 17)
        // Inspecting script OP_PUSHNUM_1 OP_PUSHBYTES_32 089b36f682a8182b5beaae57fca90c7200f1269aa3deff27119e9c381a65328f (2 out of a total of 17)
        // Inspecting script OP_PUSHNUM_1 OP_PUSHBYTES_32 2bd1aa8f1b4438bb302c5a5185e9f2541addbf58103eb2c32c7c5eb8d0e38147 (3 out of a total of 17)
        // ...
    }

    @Test
    fun fullScanWithCallbackInspector() {
        class FullScanCallback : FullScanScriptInspector {
            var totalSynced = 0
            val keychainsSeen = mutableSetOf<KeychainKind>()

            override fun inspect(
                keychain: KeychainKind,
                index: UInt,
                script: Script
            ) {
                totalSynced++
                keychainsSeen.add(keychain)
                val address = Address.fromScript(script, Network.REGTEST)
                Log.d("FullScanInspector", "[$keychain] index=$index address=$address (total scanned: $totalSynced)")
            }
        }

        val wallet = Wallet(
            descriptor = BIP84_DESCRIPTOR,
            changeDescriptor = BIP84_CHANGE_DESCRIPTOR,
            network = Network.REGTEST,
            persister = conn2
        )
        val electrumClient = ElectrumClient(ELECTRUM_REGTEST_URL)
        val fullScanCallback = FullScanCallback()
        val fullScanRequest = wallet.startFullScan().inspectSpksForAllKeychains(fullScanCallback).build()

        electrumClient.fullScan(
            request = fullScanRequest,
            stopGap = 25uL,
            batchSize = 1u,
            fetchPrevTxouts = true
        )

        // A full scan with a stop gap of 25 on an empty wallet inspects at least the first
        // 25 scripts of each keychain before giving up, across both keychains.
        assertEquals(
            50,
            fullScanCallback.totalSynced,
            "Inspector should have been called 50 times"
        )
        assertEquals(
            setOf(KeychainKind.EXTERNAL, KeychainKind.INTERNAL),
            fullScanCallback.keychainsSeen,
            "Both the external and internal keychains should be inspected"
        )

        // Logs
        // [EXTERNAL] index=0 address=bcrt1p2xhngpt7460trvnjwxgy57884hh58grz5qwxt68lwagc35qcplaq2hks4m (total scanned: 1)
        // [EXTERNAL] index=1 address=bcrt1ppzdnda5z4qvzkkl24etle2gvwgq0zf5650007fc3n6wrsxn9x28sshtggg (total scanned: 2)
        // [EXTERNAL] index=2 address=bcrt1p90g64rcmgsutkvpvtfgct60j2sddm06czqlt9sev030t358rs9rsjppelk (total scanned: 3)
        // ...
    }
}
