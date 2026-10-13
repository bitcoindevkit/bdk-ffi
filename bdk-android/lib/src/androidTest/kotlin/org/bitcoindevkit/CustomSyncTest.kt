package org.bitcoindevkit

import androidx.test.ext.junit.runners.AndroidJUnit4
import org.junit.runner.RunWith
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFailsWith

@RunWith(AndroidJUnit4::class)
class CustomSyncTest {
    // Counts the scripts a sync request actually carries, and remembers the total it reports.
    private class SpkCounter : SyncScriptInspector {
        var inspected = 0
        var lastTotal: ULong = 0u

        override fun inspect(script: Script, total: ULong) {
            inspected++
            lastTotal = total
        }
    }

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
        val counter = SpkCounter()

        val syncRequest: SyncRequest = wallet
            .startSyncWithSpkIndices(
                externalIndices = listOf(0u, 5u, 20u),
                internalIndices = listOf(3u)
            )
            .inspectSpks(counter)
            .build()

        ElectrumClient(ELECTRUM_REGTEST_URL).sync(
            request = syncRequest,
            batchSize = 10u,
            fetchPrevTxouts = true
        )

        // 4 scripts requested, against the 42 a full `startSyncWithRevealedSpks()` would have used.
        assertEquals(4, counter.inspected, "Only the 4 requested scripts should be synced")
        assertEquals(4uL, counter.lastTotal, "The request should report a total of 4 scripts")
    }

    // A keychain left out of the call contributes nothing to the request, so a sync can cover the
    // internal keychain alone.
    @Test
    fun syncWithInternalIndicesOnly() {
        val wallet = walletWithRevealedAddresses(upTo = 20u)
        val counter = SpkCounter()

        val syncRequest: SyncRequest = wallet
            .startSyncWithSpkIndices(internalIndices = listOf(0u, 1u))
            .inspectSpks(counter)
            .build()

        ElectrumClient(ELECTRUM_REGTEST_URL).sync(
            request = syncRequest,
            batchSize = 10u,
            fetchPrevTxouts = true
        )

        assertEquals(2, counter.inspected, "Only the 2 requested internal scripts should be synced")
    }

    // Asking for the last few revealed addresses of each keychain is a short way to catch up on
    // the most recently handed out addresses.
    @Test
    fun syncWithLastRevealedSpks() {
        val wallet = walletWithRevealedAddresses(upTo = 20u)
        val counter = SpkCounter()

        val syncRequest: SyncRequest = wallet
            .startSyncWithLastRevealedSpks(externalCount = 5u, internalCount = 2u)
            .inspectSpks(counter)
            .build()

        ElectrumClient(ELECTRUM_REGTEST_URL).sync(
            request = syncRequest,
            batchSize = 10u,
            fetchPrevTxouts = true
        )

        assertEquals(7, counter.inspected, "Only the 5 external and 2 internal scripts should be synced")
    }

    // Counts above what the wallet has revealed are not an error: the request simply carries every
    // revealed script of that keychain.
    @Test
    fun lastRevealedSpksIsCappedAtWhatWasRevealed() {
        val wallet = walletWithRevealedAddresses(upTo = 2u)
        val counter = SpkCounter()

        val syncRequest: SyncRequest = wallet
            .startSyncWithLastRevealedSpks(externalCount = 50u)
            .inspectSpks(counter)
            .build()

        ElectrumClient(ELECTRUM_REGTEST_URL).sync(
            request = syncRequest,
            batchSize = 10u,
            fetchPrevTxouts = true
        )

        // Indices 0, 1 and 2 were revealed, so 3 scripts are available.
        assertEquals(3, counter.inspected, "The request should be capped at the 3 revealed scripts")
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

        assertFailsWith<SpkIndexException.NoSpksRequested> {
            wallet.startSyncWithLastRevealedSpks()
        }

        assertFailsWith<SpkIndexException.NoSpksRequested> {
            wallet.startSyncWithLastRevealedSpks(externalCount = 0u, internalCount = 0u)
        }
    }

    // Counts are lenient about asking for more than was revealed, but a keychain which has
    // revealed nothing at all would produce an empty request, so it fails.
    @Test
    fun lastRevealedSpksOnWalletWithoutRevealedAddressesFails() {
        val wallet = Wallet(
            descriptor = BIP84_DESCRIPTOR,
            changeDescriptor = BIP84_CHANGE_DESCRIPTOR,
            network = Network.REGTEST,
            persister = Persister.newInMemory()
        )

        assertFailsWith<SpkIndexException.NoneRevealed> {
            wallet.startSyncWithLastRevealedSpks(externalCount = 5u)
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

        // The same holds for a count of the last revealed internal scripts.
        assertFailsWith<SpkIndexException.KeychainNotFound> {
            wallet.startSyncWithLastRevealedSpks(externalCount = 3u, internalCount = 1u)
        }

        // Leaving the missing keychain out of the request is fine, as is passing it an empty list.
        wallet.startSyncWithSpkIndices(externalIndices = listOf(0u))
        wallet.startSyncWithSpkIndices(externalIndices = listOf(0u), internalIndices = emptyList())
        wallet.startSyncWithLastRevealedSpks(externalCount = 3u)
    }
}
