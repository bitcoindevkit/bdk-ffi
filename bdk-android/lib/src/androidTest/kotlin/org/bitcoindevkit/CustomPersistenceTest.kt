package org.bitcoindevkit

import androidx.test.ext.junit.runners.AndroidJUnit4
import org.junit.runner.RunWith
import kotlin.test.Test
import kotlin.test.assertContentEquals
import kotlin.test.assertEquals
import kotlin.test.assertNotNull
import kotlin.test.assertTrue

/** Keeps the aggregate `ChangeSet` in memory, the way an app-provided persister would store it. */
private class InMemoryPersistence : Persistence {
    private var aggregate: ChangeSet? = null
    var lastPersisted: ChangeSet? = null
        private set

    @Synchronized
    override fun initialize(): ChangeSet = aggregate ?: ChangeSet()

    @Synchronized
    override fun persist(changeset: ChangeSet) {
        aggregate = aggregate?.let { ChangeSet.fromMerge(it, changeset) } ?: changeset
        lastPersisted = changeset
    }

    @Synchronized
    fun aggregateSpkCache(): Map<DescriptorId, Map<UInt, Script>> =
        aggregate?.indexerChangeset()?.spkCache ?: emptyMap()
}

@RunWith(AndroidJUnit4::class)
class CustomPersistenceTest {
    /** Number of script pubkeys the wallet derives past the last revealed index. */
    private val lookahead: UInt = 5u

    private val descriptor: Descriptor = Descriptor(
        "wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/1h/0/*)",
        NetworkKind.TEST
    )
    private val changeDescriptor: Descriptor = Descriptor(
        "wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/1h/1/*)",
        NetworkKind.TEST
    )

    private fun createWallet(persister: Persister): Wallet = Wallet.createWithParams(
        descriptor = descriptor,
        changeDescriptor = changeDescriptor,
        network = Network.SIGNET,
        persister = persister,
        params = CreateParams(genesisHash = null, lookahead = lookahead, useSpkCache = true),
    )

    // Code written before `spkCache` existed still compiles, and gets an empty cache.
    @Test
    fun indexerChangeSetDefaultsToEmptySpkCache() {
        val changeset = IndexerChangeSet(lastRevealed = mapOf(descriptor.descriptorId() to 3u))

        assertTrue(changeset.spkCache.isEmpty())
    }

    // The script pubkeys the wallet derives reach a custom persister, so it can store them.
    @Test
    fun customPersistenceReceivesSpkCache() {
        val persistence = InMemoryPersistence()
        val persister = Persister.custom(persistence)
        val wallet = createWallet(persister)

        // The derived script pubkeys are handed over together with the next reveal.
        wallet.revealNextAddress(KeychainKind.EXTERNAL)
        assertTrue(wallet.persist(persister))

        val spkCache = persistence.aggregateSpkCache()
        val expected = listOf(
            // Revealing index 0 keeps `lookahead` more script pubkeys derived past it.
            Triple(KeychainKind.EXTERNAL, descriptor.descriptorId(), lookahead + 1u),
            Triple(KeychainKind.INTERNAL, changeDescriptor.descriptorId(), lookahead),
        )
        for ((keychain, descriptorId, count) in expected) {
            val cached = assertNotNull(spkCache[descriptorId], "no spk cache for $keychain")
            assertEquals(count, cached.size.toUInt())
            for ((index, script) in cached) {
                val address = wallet.peekAddress(keychain, index).address
                assertContentEquals(address.scriptPubkey().toBytes(), script.toBytes())
            }
        }
    }

    // A wallet loaded from a custom persister reuses the stored script pubkeys instead of deriving
    // them again, so the next reveal only stages the one new script pubkey.
    @Test
    fun loadingFromCustomPersistenceReusesSpkCache() {
        val persistence = InMemoryPersistence()
        val persister = Persister.custom(persistence)
        val wallet = createWallet(persister)
        wallet.revealNextAddress(KeychainKind.EXTERNAL)
        assertTrue(wallet.persist(persister))

        val loaded = Wallet.loadWithParams(
            descriptor = descriptor,
            changeDescriptor = changeDescriptor,
            persister = persister,
            params = LoadParams(
                checkNetwork = null,
                checkGenesisHash = null,
                lookahead = lookahead,
                useSpkCache = true,
            ),
        )
        val revealed = loaded.revealNextAddress(KeychainKind.EXTERNAL)
        assertTrue(loaded.persist(persister))

        val staged = assertNotNull(persistence.lastPersisted).indexerChangeset().spkCache
            .flatMap { (descriptorId, spks) -> spks.keys.map { descriptorId to it } }
        assertEquals(listOf(descriptor.descriptorId() to revealed.index + lookahead), staged)
    }
}
