package org.bitcoindevkit

import org.junit.Assert.*
import org.junit.Test
import android.app.Application
import android.content.Context.MODE_PRIVATE
import androidx.test.core.app.ApplicationProvider
import androidx.test.ext.junit.runners.AndroidJUnit4
import org.junit.runner.RunWith
import org.slf4j.Logger
import org.slf4j.LoggerFactory
import java.io.File

/**
 * Instrumented test, which will execute on an Android device.
 *
 * See [testing documentation](http://d.android.com/tools/testing).
 */
@RunWith(AndroidJUnit4::class)
class AndroidLibTest {

    private fun getTestDataDir(): String {
        val context = ApplicationProvider.getApplicationContext<Application>()
        return context.getDir("bdk-test", MODE_PRIVATE).toString()
    }

    private fun cleanupTestDataDir(testDataDir: String) {
        File(testDataDir).deleteRecursively()
    }

    class LogProgress : Progress {
        private val log: Logger = LoggerFactory.getLogger(AndroidLibTest::class.java)

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
            100u
        )
    )

    @Test
    fun memoryWalletNewAddress() {
        val wallet = Wallet(descriptor, null, Network.TESTNET, databaseConfig)
        val address = wallet.getAddress(AddressIndex.NEW).address
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
