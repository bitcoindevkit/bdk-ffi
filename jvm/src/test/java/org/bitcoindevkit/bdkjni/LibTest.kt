package org.bitcoindevkit.bdkjni

import com.sun.jna.Native
import com.sun.jna.NativeLong
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
}
