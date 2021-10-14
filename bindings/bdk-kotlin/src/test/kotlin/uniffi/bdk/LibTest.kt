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
    fun memoryWalletNewAddress() {
        val config = DatabaseConfig.Memory("")
        val wallet = OfflineWallet(desc, Network.REGTEST, config)
        val address = wallet.getNewAddress()
        assertNotNull(address)
        assertEquals(address, "bcrt1qzg4mckdh50nwdm9hkzq06528rsu73hjxytqkxs")
    }

    @Test(expected=BdkException.Descriptor::class)
    fun invalidDescriptorExceptionIsThrown() {
        val config = DatabaseConfig.Memory("")
        OfflineWallet("invalid-descriptor", Network.REGTEST, config)
    }

    @Test
    fun sledWalletNewAddress() {
        val config = DatabaseConfig.Sled(SledDbConfiguration("/tmp/testdb", "testdb"))
        val wallet = OfflineWallet(desc, Network.REGTEST, config)
        val address = wallet.getNewAddress()
        assertNotNull(address)
        assertEquals(address, "bcrt1qzg4mckdh50nwdm9hkzq06528rsu73hjxytqkxs")
    }
}
