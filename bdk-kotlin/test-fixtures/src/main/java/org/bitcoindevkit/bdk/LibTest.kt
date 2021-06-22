package org.bitcoindevkit.bdk

import org.junit.*
import org.junit.Assert.*
import org.slf4j.Logger
import org.slf4j.LoggerFactory

/**
 * Library test, which will execute on linux host.
 *
 */
abstract class LibTest : LibBase() {

    private val log: Logger = LoggerFactory.getLogger(LibTest::class.java)

    val name = "test_wallet"
    val desc =
        "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/*)"
    val change =
        "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/1/*)"

    @Test
    fun walletResultError() {
        val badWalletResult = WalletResult("bad", "bad", "bad")
        assertTrue(badWalletResult.isErr())
        val walletErr = badWalletResult.err()
        assertNotNull(walletErr)
        log.debug("wallet error $walletErr")
        assertEquals(Error.Descriptor, walletErr)
        val wallet = badWalletResult.ok()
        assertNull(wallet)
    }

    @Test
    fun walletResultFinalize() {
        val names = listOf("one", "two", "three")
        names.map {
            val wallet = WalletResult(it, desc, change)
            assertNotNull(wallet)
        }
        System.gc()
        // The only way to verify wallets freed is by checking the log
    }

    @Test
    fun walletSync() {
        val walletResult = WalletResult(name, desc, change)
        val wallet = walletResult.ok()
        assertNotNull(wallet)
        val syncResult = wallet!!.sync();
        assertFalse(syncResult.isErr())
        assertNull(syncResult.err())
    }

    @Test
    fun walletNewAddress() {
        val walletResult = WalletResult(name, desc, change)
        val wallet = walletResult.ok()
        assertNotNull(wallet)
        val addressResult = wallet!!.getAddress()
        assertFalse(addressResult.isErr())
        assertNull(addressResult.err())
        val address = addressResult.ok()
        assertNotNull(address)
        log.debug("address created from kotlin: $address")
        assertEquals(address, "tb1qzg4mckdh50nwdm9hkzq06528rsu73hjxxzem3e")
    }
}
