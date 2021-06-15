package org.bitcoindevkit.bdk

import android.util.Log
import androidx.test.ext.junit.runners.AndroidJUnit4
import com.sun.jna.Native
import org.junit.*

import org.junit.runner.RunWith

import org.junit.Assert.*

//import org.bitcoindevkit.bdkjni.Types.Network
//import org.bitcoindevkit.bdkjni.Types.WalletConstructor
//import org.bitcoindevkit.bdkjni.Types.WalletPtr

/**
 * Instrumented test, which will execute on an Android device.
 *
 * See [testing documentation](http://d.android.com/tools/testing).
 */
@RunWith(AndroidJUnit4::class)
class AndroidLibTest {

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
            Log.d("create_wallet", "wallet created")
        }

        @AfterClass
        @JvmStatic
        fun free_wallet() {
            bdkFfi.free_wallet(wallet)
            Log.d("free_wallet", "wallet freed")
        }
    }

    @Test
    fun sync() {
        bdkFfi.sync_wallet(wallet)
        Log.d("sync", "wallet synced")
    }

    @Test
    fun new_address() {
        val address = bdkFfi.new_address(wallet)
        //println("address created from kotlin: $address")
        assertEquals(address, "tb1qzg4mckdh50nwdm9hkzq06528rsu73hjxxzem3e")
        Log.d("new_address", "new address: $address")
    }
}
