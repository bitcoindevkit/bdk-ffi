package org.bitcoindevkit

import androidx.test.ext.junit.runners.AndroidJUnit4
import org.junit.runner.RunWith
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFailsWith

private fun ByteArray.toHex(): String = joinToString("") { "%02x".format(it) }

@RunWith(AndroidJUnit4::class)
class CustomSyncTest {
    // Records the scripts a sync request actually carries (as hex, since `Script` has no value
    // equality), and remembers the total it reports.
    private class SpkRecorder : SyncScriptInspector {
        val scripts = mutableListOf<String>()
        var lastTotal: ULong = 0u

        override fun inspect(script: Script, total: ULong) {
            scripts.add(script.toBytes().toHex())
            lastTotal = total
        }
    }

    // The hex scripts the wallet derives at the given indices of a keychain.
    private fun Wallet.scriptsAt(keychain: KeychainKind, indices: List<UInt>): List<String> =
        indices.map { peekAddress(keychain, it).address.scriptPubkey().toBytes().toHex() }

    private fun walletWithRevealedAddresses(upTo: UInt): Wallet {
        val wallet = Wallet(
            descriptor = BIP84_DESCRIPTOR,
            changeDescriptor = BIP84_CHANGE_DESCRIPTOR,
            network = Network.REGTEST,
            persister = Persister.newInMemory()
        )
        wallet.revealAddressesTo(KeychainKind.EXTERNAL, upTo)
        wallet.revealAddressesTo(KeychainKind.INTERNAL, upTo)
        return wallet
    }

    // A sync request built from a specific set of indices carries only those scripts, rather than
    // every revealed script of the wallet.
    @Test
    fun syncWithSpecificIndices() {
        val wallet = walletWithRevealedAddresses(upTo = 20u)
        val recorder = SpkRecorder()
        val externalIndices = listOf(0u, 5u, 20u)
        val internalIndices = listOf(3u)

        val syncRequest: SyncRequest = wallet
            .startSyncWithSpkIndices(
                externalIndices = externalIndices,
                internalIndices = internalIndices
            )
            .inspectSpks(recorder)
            .build()

        ElectrumClient(ELECTRUM_REGTEST_URL).sync(
            request = syncRequest,
            batchSize = 10u,
            fetchPrevTxouts = true
        )

        // 4 scripts requested, against the 42 a full `startSyncWithRevealedSpks()` would have used.
        assertEquals(4, recorder.scripts.size, "Only the 4 requested scripts should be synced")
        assertEquals(4uL, recorder.lastTotal, "The request should report a total of 4 scripts")
        assertEquals(
            expected = (wallet.scriptsAt(KeychainKind.EXTERNAL, externalIndices) +
                wallet.scriptsAt(KeychainKind.INTERNAL, internalIndices)).toSet(),
            actual = recorder.scripts.toSet(),
            message = "The synced scripts should be exactly those at the requested indices"
        )
    }

    // A keychain left out of the call contributes nothing to the request, so a sync can cover the
    // internal keychain alone.
    @Test
    fun syncWithInternalIndicesOnly() {
        val wallet = walletWithRevealedAddresses(upTo = 20u)
        val recorder = SpkRecorder()
        val internalIndices = listOf(0u, 1u)

        val syncRequest: SyncRequest = wallet
            .startSyncWithSpkIndices(internalIndices = internalIndices)
            .inspectSpks(recorder)
            .build()

        ElectrumClient(ELECTRUM_REGTEST_URL).sync(
            request = syncRequest,
            batchSize = 10u,
            fetchPrevTxouts = true
        )

        assertEquals(2, recorder.scripts.size, "Only the 2 requested internal scripts should be synced")
        assertEquals(
            expected = wallet.scriptsAt(KeychainKind.INTERNAL, internalIndices).toSet(),
            actual = recorder.scripts.toSet(),
            message = "The synced scripts should be exactly the requested internal ones"
        )
    }

    // An index past the last revealed one is refused: the wallet's indexer does not track it, so
    // any transaction found there would not be recognized as the wallet's own.
    @Test
    fun syncWithUnrevealedIndexFails() {
        val wallet = walletWithRevealedAddresses(upTo = 5u)

        val exception = assertFailsWith<SpkIndexException.NotRevealed> {
            wallet.startSyncWithSpkIndices(externalIndices = listOf(0u, 6u))
        }

        assertEquals(
            expected = "index 6 of the External keychain has not been revealed, the last revealed index is 5",
            actual = exception.toString()
        )
        assertEquals(expected = KeychainKind.EXTERNAL, actual = exception.keychain)
        assertEquals(expected = 6u, actual = exception.index)
        assertEquals(expected = 5u, actual = exception.lastRevealed)
    }

    // A wallet which has not revealed anything yet has no index to sync.
    @Test
    fun syncOnWalletWithoutRevealedAddressesFails() {
        val wallet = Wallet(
            descriptor = BIP84_DESCRIPTOR,
            changeDescriptor = BIP84_CHANGE_DESCRIPTOR,
            network = Network.REGTEST,
            persister = Persister.newInMemory()
        )

        val exception = assertFailsWith<SpkIndexException.NoneRevealed> {
            wallet.startSyncWithSpkIndices(externalIndices = listOf(0u))
        }

        assertEquals(
            expected = "no index of the External keychain has been revealed",
            actual = exception.toString()
        )
    }

    // A request which would carry no script at all is refused rather than quietly turning into a
    // sync that only refreshes the chain tip.
    @Test
    fun syncWithoutAnySpkFails() {
        val wallet = walletWithRevealedAddresses(upTo = 5u)

        val exception = assertFailsWith<SpkIndexException.NoSpksRequested> {
            wallet.startSyncWithSpkIndices()
        }
        assertEquals(
            expected = "no spk was requested, the sync request would be empty",
            actual = exception.toString()
        )

        assertFailsWith<SpkIndexException.NoSpksRequested> {
            wallet.startSyncWithSpkIndices(externalIndices = emptyList(), internalIndices = emptyList())
        }
    }

    // A single-descriptor wallet has no internal keychain, so asking for internal indices fails
    // rather than quietly syncing external scripts.
    @Test
    fun syncInternalIndicesOnSingleDescriptorWalletFails() {
        val wallet = Wallet.createSingle(
            descriptor = BIP84_DESCRIPTOR,
            network = Network.REGTEST,
            persister = Persister.newInMemory()
        )
        wallet.revealAddressesTo(KeychainKind.EXTERNAL, 5u)

        val indicesException = assertFailsWith<SpkIndexException.KeychainNotFound> {
            wallet.startSyncWithSpkIndices(
                externalIndices = listOf(0u),
                internalIndices = listOf(0u)
            )
        }
        assertEquals(
            expected = "this wallet has no Internal keychain",
            actual = indicesException.toString()
        )

        // Leaving the missing keychain out of the request is fine, as is passing it an empty list.
        wallet.startSyncWithSpkIndices(externalIndices = listOf(0u))
        wallet.startSyncWithSpkIndices(externalIndices = listOf(0u), internalIndices = emptyList())
    }
}
