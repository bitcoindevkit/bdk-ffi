package org.bitcoindevkit.bdk

import com.sun.jna.Native
import org.junit.Test
import kotlin.test.assertEquals

/**
 * Library test, which will execute on linux host.
 *
 */
class LibTest {

    private val bdkFfi: Lib = Native.load("bdk_ffi", Lib::class.java)

    @Test
    fun new_sync_free_wallet() {
        val name = "test_wallet"
        val desc = "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/*)"
        val change = "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/1/*)"

        val wallet = bdkFfi.new_wallet(name, desc, change)
        bdkFfi.sync_wallet(wallet)
        bdkFfi.free_wallet(wallet)
    }

    @Test
    fun new_newaddress_wallet() {
        val name = "test_wallet"
        val desc = "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/*)"
        val change = "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/1/*)"

        val wallet = bdkFfi.new_wallet(name, desc, change)
        val address = bdkFfi.new_address(wallet)
        //println("address created from kotlin: $address")
        assertEquals(address, "tb1qzg4mckdh50nwdm9hkzq06528rsu73hjxxzem3e")
        bdkFfi.free_string(address)
        bdkFfi.free_wallet(wallet)
    }
}
