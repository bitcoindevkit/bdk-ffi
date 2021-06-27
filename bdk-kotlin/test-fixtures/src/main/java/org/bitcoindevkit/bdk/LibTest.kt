package org.bitcoindevkit.bdk

import org.junit.Assert.*
import org.junit.Test
import org.slf4j.Logger
import org.slf4j.LoggerFactory
import java.io.File

/**
 * Library tests which will execute for jvm and android modules.
 */
abstract class LibTest : LibBase() {

    private val log: Logger = LoggerFactory.getLogger(LibTest::class.java)

    val name = "test_wallet"
    val desc =
        "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/*)"
    val change =
        "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/1/*)"

    abstract fun getTestDataDir(): String

    fun cleanupTestDataDir() {
        File(getTestDataDir()).deleteRecursively()
    }

    @Test
    fun walletResultError() {
        val blockchainConfig = ElectrumConfig("ssl://electrum.blockstream.info:60002", null, 5, 30)
        val databaseConfig = MemoryConfig()
        val jnaException = assertThrows(JnaException::class.java) {
            Wallet("bad", "bad", blockchainConfig, databaseConfig)
        }
        assertEquals(jnaException.err, JnaError.Descriptor)
    }

    @Test
    fun walletResultFinalize() {
        val blockchainConfig = ElectrumConfig("ssl://electrum.blockstream.info:60002", null, 5, 30)
        val databaseConfig = MemoryConfig()
        val names = listOf("one", "two", "three")
        names.map {
            val wallet = Wallet(desc, change, blockchainConfig, databaseConfig)
            assertNotNull(wallet)
        }
        System.gc()
        // The only way to verify wallets freed is by checking the log
    }

    @Test
    fun walletSync() {
        val blockchainConfig = ElectrumConfig("ssl://electrum.blockstream.info:60002", null, 5, 30)
        val testDataDir = getTestDataDir()
        log.debug("testDataDir = $testDataDir")
        val databaseConfig = SledConfig(testDataDir, "steve-test")
        val wallet = Wallet(desc, change, blockchainConfig, databaseConfig)
        assertNotNull(wallet)
        wallet.sync()
        cleanupTestDataDir()
    }

    @Test
    fun walletNewAddress() {
        val blockchainConfig = ElectrumConfig("ssl://electrum.blockstream.info:60002", null, 5, 30)
        val databaseConfig = MemoryConfig()
        val wallet = Wallet(desc, change, blockchainConfig, databaseConfig)
        assertNotNull(wallet)
        val address = wallet.getAddress()
        assertNotNull(address)
        log.debug("address created from kotlin: $address")
        assertEquals(address, "tb1qzg4mckdh50nwdm9hkzq06528rsu73hjxxzem3e")
    }
}
