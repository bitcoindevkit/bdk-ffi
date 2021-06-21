package org.bitcoindevkit.bdk

import org.junit.*
import org.junit.Assert.assertEquals
import org.slf4j.Logger
import org.slf4j.LoggerFactory
import kotlin.test.assertNotNull
import kotlin.test.assertNull

/**
 * Library test, which will execute on linux host.
 *
 */
abstract class LibTest {

    companion object : LibBase() {
        private val log: Logger = LoggerFactory.getLogger(LibTest::class.java)
        
        private lateinit var wallet_result: LibJna.WalletResult_t

        @BeforeClass
        @JvmStatic
        fun new_wallet() {
            val name = "test_wallet"
            val desc =
                "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/*)"
            val change =
                "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/1/*)"

            wallet_result = libJna.new_wallet_result(name, desc, change)
            log.debug("wallet created")
        }

        @AfterClass
        @JvmStatic
        fun free_wallet() {
            libJna.free_wallet_result(wallet_result)
            log.debug("wallet freed")
        }
    }

    @Test
    fun wallet_sync_error() {
        val bad_wallet_result = libJna.new_wallet_result("test", "bad", null)
        log.debug("wallet result created")
        val sync_result = libJna.sync_wallet(bad_wallet_result)
        val sync_err_pointer = libJna.get_void_err(sync_result)
        assertNotNull(sync_err_pointer)
        val sync_err = sync_err_pointer.getString(0)
        log.debug("wallet sync error $sync_err")
    }

    @Test
    fun sync() {
        val sync_result = libJna.sync_wallet(wallet_result)
        assertNull(libJna.get_void_err(sync_result))
        libJna.free_void_result(sync_result)
    }

    @Test
    fun new_newaddress_wallet() {
        val address_result = libJna.new_address(wallet_result)
        assertNull(libJna.get_string_err(address_result))
        val address_pointer = libJna.get_string_ok(address_result);
        val address = address_pointer!!.getString(0)
        
        libJna.free_string_result(address_result)
        libJna.free_string(address_pointer)
        log.debug("address created from kotlin: $address")
        assertEquals(address, "tb1qzg4mckdh50nwdm9hkzq06528rsu73hjxxzem3e")
    }
}
