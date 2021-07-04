package org.bitcoindevkit.bdk

import org.bitcoindevkit.bdk.wallet.Wallet
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

    val blockchainConfig = ElectrumConfig("ssl://electrum.blockstream.info:60002", null, 5, 30)
    val databaseConfig = MemoryConfig()

    abstract fun getTestDataDir(): String

    fun cleanupTestDataDir() {
        File(getTestDataDir()).deleteRecursively()
    }

    @Test
    fun walletResultError() {
        val jnaException = assertThrows(FfiException::class.java) {
            Wallet("bad", "bad", blockchainConfig, databaseConfig)
        }
        assertEquals(jnaException.err, FfiError.Descriptor)
    }

//    @Test
//    fun walletResultFinalize() {
//        run {
//            val desc =
//                "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/*)"
//            val change =
//                "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/1/*)"
//
//            val blockchainConfig = ElectrumConfig("ssl://electrum.blockstream.info:60002", null, 5, 30)
//            val databaseConfig = MemoryConfig()
//            val wallet = Wallet(desc, change, blockchainConfig, databaseConfig)
//            wallet.sync()
//            assertNotNull(wallet.getAddress())
//        }
//        System.gc()
//        Thread.sleep(2000)
//        // The only way to verify wallets freed is by checking the log
//    }

    @Test
    fun walletSync() {
        val blockchainConfig = ElectrumConfig("ssl://electrum.blockstream.info:60002", null, 5, 30)
        val testDataDir = getTestDataDir()
        // log.debug("testDataDir = $testDataDir")
        val databaseConfig = SledConfig(testDataDir, "steve-test")
        val wallet = Wallet(desc, change, blockchainConfig, databaseConfig)
        wallet.sync()
        cleanupTestDataDir()
    }

    @Test
    fun walletNewAddress() {
        val wallet = Wallet(desc, change, blockchainConfig, databaseConfig)
        val address = wallet.getAddress()
        assertNotNull(address)
        // log.debug("address created from kotlin: $address")
        assertEquals(address, "tb1qzg4mckdh50nwdm9hkzq06528rsu73hjxxzem3e")
    }

    @Test
    fun walletUnspent() {
        val wallet = Wallet(desc, change, blockchainConfig, databaseConfig)
        wallet.sync()
        val unspent = wallet.listUnspent()
        assertTrue(unspent.isNotEmpty())
        
        unspent.iterator().forEach {
            //log.debug("unspent.outpoint.txid: ${it.outpoint!!.txid}")
            assertNotNull(it.outpoint?.txid)
            //log.debug("unspent.outpoint.vout: ${it.outpoint?.vout}")
            assertTrue(it.outpoint?.vout!! >= 0)
            //log.debug("unspent.txout.value: ${it.txout?.value}")
            assertTrue(it.txout?.value!! > 0)
            //log.debug("unspent.txout.script_pubkey: ${it.txout?.script_pubkey}")
            assertNotNull(it.txout?.script_pubkey)
            //log.debug("unspent.keychain: ${it.keychain}")
            assertTrue(it.keychain!! >= 0)
        }
    }

    @Test
    fun walletBalance() {
        val wallet = Wallet(desc, change, blockchainConfig, databaseConfig)
        wallet.sync()
        val balance = wallet.balance()
        //log.debug("balance from kotlin: $balance")
        assertTrue(balance > 0)
    }
}
