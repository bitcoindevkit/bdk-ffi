package uniffi.bdk

import uniffi.bdk.OfflineWallet
import org.junit.Assert.*
import org.junit.Test

class LogProgress: BdkProgress {
    override fun update(progress: Float, message: String? ) {
        println("progress: $progress, message: $message")
    }
}

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

    @Test
    fun onlineWalletInMemory() {
        val db = DatabaseConfig.Memory("")
        val client = BlockchainConfig.Electrum(ElectrumConfig("ssl://electrum.blockstream.info:60002", null, 5u, null, 100u))
        val wallet = OnlineWallet(desc, Network.TESTNET, db, client)
        assertNotNull(wallet)
        val network = wallet.getNetwork()
        assertEquals(network, Network.TESTNET)
    }

    @Test
    fun onlineWalletSyncGetBalance() {
        val db = DatabaseConfig.Memory("")
        val client = BlockchainConfig.Electrum(ElectrumConfig("ssl://electrum.blockstream.info:60002", null, 5u, null, 100u))
        val wallet = OnlineWallet(desc, Network.TESTNET, db, client)
        wallet.sync(LogProgress(), null)
        val balance = wallet.getBalance()
        assertTrue(balance > 0u)
    }
}
