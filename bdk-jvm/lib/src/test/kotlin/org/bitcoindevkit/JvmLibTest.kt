package org.bitcoindevkit

import org.junit.Assert.*
import org.junit.Test
import org.slf4j.Logger
import org.slf4j.LoggerFactory
import java.io.File
import java.nio.file.Files

/**
 * Library test, which will execute on linux host.
 */
class JvmLibTest {

    private fun getTestDataDir(): String {
        return Files.createTempDirectory("bdk-test").toString()
    }

    private fun cleanupTestDataDir(testDataDir: String) {
        File(testDataDir).deleteRecursively()
    }

    class LogProgress : Progress {
        private val log: Logger = LoggerFactory.getLogger(JvmLibTest::class.java)

        override fun update(progress: Float, message: String?) {
            log.debug("Syncing...")
        }
    }

    private val descriptor = Descriptor("wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/*)", Network.TESTNET)

    private val databaseConfig = DatabaseConfig.Memory

    private val blockchainConfig = BlockchainConfig.Electrum(
        ElectrumConfig(
            "ssl://electrum.blockstream.info:60002",
            null,
            5u,
            null,
            100u,
            true,
        )
    )

    @Test
    fun memoryWalletNewAddress() {
        val wallet = Wallet(descriptor, null, Network.TESTNET, databaseConfig)
        val address = wallet.getAddress(AddressIndex.New).address.asString()
        assertEquals("tb1qzg4mckdh50nwdm9hkzq06528rsu73hjxxzem3e", address)
    }

    @Test
    fun memoryWalletSyncGetBalance() {
        val wallet = Wallet(descriptor, null, Network.TESTNET, databaseConfig)
        val blockchain = Blockchain(blockchainConfig)
        wallet.sync(blockchain, LogProgress())
        val balance: Balance = wallet.getBalance()
        assertTrue(balance.total > 0u)
    }

    @Test
    fun sqliteWalletSyncGetBalance() {
        val testDataDir = getTestDataDir() + "/bdk-wallet.sqlite"
        val databaseConfig = DatabaseConfig.Sqlite(SqliteDbConfiguration(testDataDir))
        val wallet = Wallet(descriptor, null, Network.TESTNET, databaseConfig)
        val blockchain = Blockchain(blockchainConfig)
        wallet.sync(blockchain, LogProgress())
        val balance: Balance = wallet.getBalance()
        assertTrue(balance.total > 0u)
        cleanupTestDataDir(testDataDir)
    }
}
