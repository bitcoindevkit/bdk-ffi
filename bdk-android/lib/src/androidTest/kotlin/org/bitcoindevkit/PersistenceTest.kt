package org.bitcoindevkit

import androidx.test.platform.app.InstrumentationRegistry
import java.io.File
import kotlin.test.Test
import kotlin.test.assertEquals

class PersistenceTest {
    private val persistenceFilePath: String by lazy {
        val context = InstrumentationRegistry.getInstrumentation().targetContext
        val dbFileName = "persistence_test_db.sqlite3"

        // Copy the file from assets to a writable location (databases dir)
        val destFile = File(context.getDatabasePath(dbFileName).path)
        context.assets.open(dbFileName).use { input ->
            destFile.outputStream().use { output ->
                input.copyTo(output)
            }
        }
        destFile.absolutePath
    }

    private val descriptor: Descriptor = Descriptor(
        "wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/0/*)",
        Network.SIGNET
    )
    private val changeDescriptor: Descriptor = Descriptor(
        "wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/1/*)",
        Network.SIGNET
    )

    // fun `Correctly load wallet from sqlite persistence`() {
    @Test
    fun correctlyLoadFromPersistence() {
        val connection = Persister.newSqlite(persistenceFilePath)

        val wallet: Wallet = Wallet.load(
            descriptor,
            changeDescriptor,
            connection
        )
        val addressInfo: AddressInfo = wallet.revealNextAddress(KeychainKind.EXTERNAL)

        assertEquals(
            expected = 7u,
            actual = addressInfo.index,
        )
        assertEquals(
            expected = "tb1qan3lldunh37ma6c0afeywgjyjgnyc8uz975zl2",
            actual = addressInfo.address.toString(),
        )
    }
}
