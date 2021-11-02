package org.bitcoindevkit

import org.junit.Assert.*
import org.junit.Test
import org.slf4j.Logger
import org.slf4j.LoggerFactory
import org.bitcoindevkit.*
import java.io.File

/**
 * Library tests which will execute for jvm and android modules.
 */
abstract class LibTest {

    abstract fun getTestDataDir(): String

    fun cleanupTestDataDir(testDataDir: String) {
        File(testDataDir).deleteRecursively()
    }

    val log: Logger = LoggerFactory.getLogger(LibTest::class.java)

    val desc =
        "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/*)"

    @Test
    fun memoryWalletNewAddress() {
        val config = DatabaseConfig.Memory("")
        val wallet = OfflineWallet(desc, Network.REGTEST, config)
        val address = wallet.getNewAddress()
        assertNotNull(address)
        assertEquals(address, "bcrt1qzg4mckdh50nwdm9hkzq06528rsu73hjxytqkxs")
    }

    @Test(expected = BdkException.Descriptor::class)
    fun invalidDescriptorExceptionIsThrown() {
        val config = DatabaseConfig.Memory("")
        OfflineWallet("invalid-descriptor", Network.REGTEST, config)
    }

    @Test
    fun sledWalletNewAddress() {
        val testDataDir = getTestDataDir()
        val config = DatabaseConfig.Sled(SledDbConfiguration(testDataDir, "testdb"))
        val wallet = OfflineWallet(desc, Network.REGTEST, config)
        val address = wallet.getNewAddress()
        assertNotNull(address)
        assertEquals(address, "bcrt1qzg4mckdh50nwdm9hkzq06528rsu73hjxytqkxs")
        cleanupTestDataDir(testDataDir)
    }

    @Test
    fun onlineWalletInMemory() {
        val db = DatabaseConfig.Memory("")
        val client = BlockchainConfig.Electrum(
            ElectrumConfig(
                "ssl://electrum.blockstream.info:60002",
                null,
                5u,
                null,
                100u
            )
        )
        val wallet = OnlineWallet(desc, null, Network.TESTNET, db, client)
        assertNotNull(wallet)
        val network = wallet.getNetwork()
        assertEquals(network, Network.TESTNET)
    }

    class LogProgress : BdkProgress {
        val log: Logger = LoggerFactory.getLogger(LibTest::class.java)

        override fun update(progress: Float, message: String?) {
            log.debug("Syncing...")
        }
    }

    @Test
    fun onlineWalletSyncGetBalance() {
        val db = DatabaseConfig.Memory("")
        val client = BlockchainConfig.Electrum(
            ElectrumConfig(
                "ssl://electrum.blockstream.info:60002",
                null,
                5u,
                null,
                100u
            )
        )
        val wallet = OnlineWallet(desc, null, Network.TESTNET, db, client)
        wallet.sync(LogProgress(), null)
        val balance = wallet.getBalance()
        assertTrue(balance > 0u)
    }
}
