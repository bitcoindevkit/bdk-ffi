package org.bitcoindevkit.bdkjni

import com.sun.jna.Native
import com.sun.jna.NativeLong
import com.sun.jna.Pointer
import com.sun.jna.ptr.PointerByReference
import org.junit.Test

/**
 * Library test, which will execute on linux host.
 *
 */
class LibTest {

    private val lib: Lib = Native.load("bdk_ffi", Lib::class.java)

    @Test
    fun print_string() {
        lib.print_string("hello print string")
    }

    @Test
    fun concat_print_free_string() {
        val concat = lib.concat_string("hello", "concat")
        lib.print_string(concat)
        lib.free_string(concat)
    }
    
    @Test
    fun print_free_config() {
        val config = Config_t()
        config.name = "test"
        config.count = NativeLong(101)
        lib.print_config(config)
        lib.free_config(config)
    }

    @Test
    fun new_print_free_config() {
        println("Long max value = ${Long.MAX_VALUE}")
        val config = lib.new_config("test test", NativeLong(Long.MAX_VALUE))
        lib.print_config(config)
        lib.free_config(config)
    }

//    @Test
//    fun new_sync_free_wallet() {
//        val name = "test_wallet"
//        val desc = "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/*)"
//        val change = "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/1/*)"
//        
//        val wallet = lib.new_wallet(name, desc, change)
//        println("wallet created in kotlin: $wallet")
//        lib.sync_wallet(wallet)
//        //lib.free_wallet(wallet)
//    }

//    @Test
//    fun new_newaddress_wallet() {
//        val name = "test_wallet"
//        val desc = "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/*)"
//        val change = "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/1/*)"
//
//        val config = lib.new_config("test test", NativeLong(Long.MAX_VALUE))
//        lib.print_config(config)
//        lib.free_config(config)
//    }
}
