package org.bitcoindevkit.bdk

import com.sun.jna.Native
import org.junit.*
import org.junit.Assert.assertEquals

/**
 * Library test, which will execute on linux host.
 *
 */
class LibTest {

    companion object {
        private val bdkFfi: Lib = Native.load("bdk_ffi", Lib::class.java)
        private lateinit var wallet: Lib.WalletPtr_t

        @BeforeClass
        @JvmStatic
        fun create_wallet() {
            val name = "test_wallet"
            val desc =
                "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/*)"
            val change =
                "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/1/*)"

            wallet = bdkFfi.new_wallet(name, desc, change)
            println("wallet created")
        }

        @AfterClass
        @JvmStatic
        fun free_wallet() {
            bdkFfi.free_wallet(wallet)
            println("wallet freed")
        }
    }

    @Test
    fun sync() {
        bdkFfi.sync_wallet(wallet)
    }

    @Test
    fun new_newaddress_wallet() {
        val pointer = bdkFfi.new_address(wallet)
        val address = pointer.getString(0)
        bdkFfi.free_string(pointer)
        //println("address created from kotlin: $address")
        assertEquals(address, "tb1qzg4mckdh50nwdm9hkzq06528rsu73hjxxzem3e")
    }
}
