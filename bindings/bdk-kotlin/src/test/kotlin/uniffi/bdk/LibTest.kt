package uniffi.bdk

import uniffi.bdk.OfflineWallet
import org.junit.Assert.*
import org.junit.Test

/**
 * Library tests which will execute for jvm and android modules.
 */
class LibTest {

    val desc =
        "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/*)"

    @Test
    fun walletNewAddress() {
        val wallet = OfflineWallet(desc)
        val address = wallet.getNewAddress()
        assertNotNull(address)
        assertEquals(address, "bcrt1qzg4mckdh50nwdm9hkzq06528rsu73hjxytqkxs")
    }

    @Test(expected=BdkException.Descriptor::class)
    fun invalidDescriptorExceptionIsThrown() {
        OfflineWallet("invalid-descriptor")
    }
}
