package org.bitcoindevkit

import org.junit.Assert.*
import org.junit.Test
import android.app.Application
import android.content.Context.MODE_PRIVATE
import androidx.test.core.app.ApplicationProvider
import androidx.test.ext.junit.runners.AndroidJUnit4
import org.junit.runner.RunWith
import org.slf4j.Logger
import org.slf4j.LoggerFactory
import java.io.File

/**
 * Instrumented test, which will execute on an Android device.
 *
 * See [testing documentation](http://d.android.com/tools/testing).
 */
@RunWith(AndroidJUnit4::class)
class AndroidLibTest {

    fun getTestDataDir(): String {
        val context = ApplicationProvider.getApplicationContext<Application>()
        return context.getDir("bdk-test", MODE_PRIVATE).toString()
    }

    fun cleanupTestDataDir(testDataDir: String) {
        File(testDataDir).deleteRecursively()
    }

    val log: Logger = LoggerFactory.getLogger(AndroidLibTest::class.java)

    val descriptor =
        "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/*)"

    val databaseConfig = DatabaseConfig.Memory("")
    val blockchainConfig = BlockchainConfig.Electrum(
        ElectrumConfig(
            "ssl://electrum.blockstream.info:60002",
            null,
            5u,
            null,
            100u
        )
    )

    @Test
    fun memoryWalletNewAddress() {
        val wallet = Wallet(descriptor, null, Network.REGTEST, databaseConfig, blockchainConfig)
        val address = wallet.getNewAddress()
        assertNotNull(address)
        assertEquals(address, "bcrt1qzg4mckdh50nwdm9hkzq06528rsu73hjxytqkxs")
    }

    @Test(expected = BdkException.Descriptor::class)
    fun invalidDescriptorExceptionIsThrown() {
        Wallet("invalid-descriptor", null, Network.REGTEST, databaseConfig, blockchainConfig)
    }

    @Test
    fun sledWalletNewAddress() {
        val testDataDir = getTestDataDir()
        val databaseConfig = DatabaseConfig.Sled(SledDbConfiguration(testDataDir, "testdb"))
        val wallet = Wallet(descriptor, null, Network.REGTEST, databaseConfig, blockchainConfig)
        val address = wallet.getNewAddress()
        assertNotNull(address)
        assertEquals(address, "bcrt1qzg4mckdh50nwdm9hkzq06528rsu73hjxytqkxs")
        cleanupTestDataDir(testDataDir)
    }

    @Test
    fun onlineWalletInMemory() {
        val database = DatabaseConfig.Memory("")
        val blockchain = BlockchainConfig.Electrum(
            ElectrumConfig(
                "ssl://electrum.blockstream.info:60002",
                null,
                5u,
                null,
                100u
            )
        )
        val wallet = Wallet(descriptor, null, Network.TESTNET, database, blockchain)
        assertNotNull(wallet)
        val network = wallet.getNetwork()
        assertEquals(network, Network.TESTNET)
    }

    class LogProgress : BdkProgress {
        val log: Logger = LoggerFactory.getLogger(AndroidLibTest::class.java)

        override fun update(progress: Float, message: String?) {
            log.debug("Syncing...")
        }
    }

    @Test
    fun onlineWalletSyncGetBalance() {
        val db = DatabaseConfig.Memory("")
        val client = BlockchainConfig.Electrum(
            ElectrumConfig(
                "ssl://electrum.blockstream.info:60002",
                null,
                5u,
                null,
                100u
            )
        )
        val wallet = Wallet(descriptor, null, Network.REGTEST, databaseConfig, blockchainConfig)
        wallet.sync(LogProgress(), null)
        val balance = wallet.getBalance()
        assertTrue(balance > 0u)
    }

    @Test
    fun validPsbtSerde() {
        val validSerializedPsbt = "cHNidP8BAHUCAAAAASaBcTce3/KF6Tet7qSze3gADAVmy7OtZGQXE8pCFxv2AAAAAAD+////AtPf9QUAAAAAGXapFNDFmQPFusKGh2DpD9UhpGZap2UgiKwA4fUFAAAAABepFDVF5uM7gyxHBQ8k0+65PJwDlIvHh7MuEwAAAQD9pQEBAAAAAAECiaPHHqtNIOA3G7ukzGmPopXJRjr6Ljl/hTPMti+VZ+UBAAAAFxYAFL4Y0VKpsBIDna89p95PUzSe7LmF/////4b4qkOnHf8USIk6UwpyN+9rRgi7st0tAXHmOuxqSJC0AQAAABcWABT+Pp7xp0XpdNkCxDVZQ6vLNL1TU/////8CAMLrCwAAAAAZdqkUhc/xCX/Z4Ai7NK9wnGIZeziXikiIrHL++E4sAAAAF6kUM5cluiHv1irHU6m80GfWx6ajnQWHAkcwRAIgJxK+IuAnDzlPVoMR3HyppolwuAJf3TskAinwf4pfOiQCIAGLONfc0xTnNMkna9b7QPZzMlvEuqFEyADS8vAtsnZcASED0uFWdJQbrUqZY3LLh+GFbTZSYG2YVi/jnF6efkE/IQUCSDBFAiEA0SuFLYXc2WHS9fSrZgZU327tzHlMDDPOXMMJ/7X85Y0CIGczio4OFyXBl/saiK9Z9R5E5CVbIBZ8hoQDHAXR8lkqASECI7cr7vCWXRC+B3jv7NYfysb3mk6haTkzgHNEZPhPKrMAAAAAAAAA"
        val psbt = PartiallySignedBitcoinTransaction.deserialize(validSerializedPsbt)
        val psbtSerialized = psbt.serialize()
        assertEquals(psbtSerialized, validSerializedPsbt)
    }

}
