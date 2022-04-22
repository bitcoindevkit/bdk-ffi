package org.bitcoindevkit

import org.junit.Assert.*
import org.junit.Test
import org.slf4j.Logger
import org.slf4j.LoggerFactory
import java.io.File
import java.nio.file.Files

/**
 * Library test, which will execute on linux host.
 *
 */
class JvmLibTest {

    fun getTestDataDir(): String {
        return Files.createTempDirectory("bdk-test").toString()
        //return Paths.get(System.getProperty("java.io.tmpdir"), "bdk-test").toString()
    }

    fun cleanupTestDataDir(testDataDir: String) {
        File(testDataDir).deleteRecursively()
    }

    val log: Logger = LoggerFactory.getLogger(JvmLibTest::class.java)

    val descriptor =
        "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/*)"

    val databaseConfig = DatabaseConfig.Memory
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
        val wallet = Wallet(descriptor, null, Network.TESTNET, databaseConfig)
        val address = wallet.getNewAddress()
        assertNotNull(address)
        assertEquals("tb1qzg4mckdh50nwdm9hkzq06528rsu73hjxxzem3e", address)
    }

    @Test(expected = BdkException.Descriptor::class)
    fun invalidDescriptorExceptionIsThrown() {
        Wallet("invalid-descriptor", null, Network.TESTNET, databaseConfig)
    }

    @Test
    fun sledWalletNewAddress() {
        val testDataDir = getTestDataDir()
        val databaseConfig = DatabaseConfig.Sled(SledDbConfiguration(testDataDir, "testdb"))
        val wallet = Wallet(descriptor, null, Network.TESTNET, databaseConfig)
        val address = wallet.getNewAddress()
        assertNotNull(address)
        assertEquals("tb1qzg4mckdh50nwdm9hkzq06528rsu73hjxxzem3e", address)
        cleanupTestDataDir(testDataDir)
    }

    @Test
    fun sqliteWalletSyncGetBalance() {
        val testDataDir = getTestDataDir() + "/bdk-wallet.sqlite"
        val databaseConfig = DatabaseConfig.Sqlite(SqliteDbConfiguration(testDataDir))
        val wallet = Wallet(descriptor, null, Network.TESTNET, databaseConfig)
        val blockchain = Blockchain(blockchainConfig);
        wallet.sync(blockchain, LogProgress())
        val balance = wallet.getBalance()
        assertTrue(balance > 0u)
        cleanupTestDataDir(testDataDir)
    }

    @Test
    fun onlineWalletInMemory() {
        val database = DatabaseConfig.Memory
        val blockchain = BlockchainConfig.Electrum(
            ElectrumConfig(
                "ssl://electrum.blockstream.info:60002",
                null,
                5u,
                null,
                100u
            )
        )
        val wallet = Wallet(descriptor, null, Network.TESTNET, database)
        assertNotNull(wallet)
        val network = wallet.getNetwork()
        assertEquals(network, Network.TESTNET)
    }

    class LogProgress : Progress {
        val log: Logger = LoggerFactory.getLogger(JvmLibTest::class.java)

        override fun update(progress: Float, message: String?) {
            log.debug("Syncing...")
        }
    }

    @Test
    fun onlineWalletSyncGetBalance() {
        val wallet = Wallet(descriptor, null, Network.TESTNET, databaseConfig)
        val blockchain = Blockchain(blockchainConfig);
        wallet.sync(blockchain, LogProgress())
        val balance = wallet.getBalance()
        assertTrue(balance > 0u)
    }

    @Test
    fun validPsbtSerde() {
        val validSerializedPsbt =
            "cHNidP8BAHUCAAAAASaBcTce3/KF6Tet7qSze3gADAVmy7OtZGQXE8pCFxv2AAAAAAD+////AtPf9QUAAAAAGXapFNDFmQPFusKGh2DpD9UhpGZap2UgiKwA4fUFAAAAABepFDVF5uM7gyxHBQ8k0+65PJwDlIvHh7MuEwAAAQD9pQEBAAAAAAECiaPHHqtNIOA3G7ukzGmPopXJRjr6Ljl/hTPMti+VZ+UBAAAAFxYAFL4Y0VKpsBIDna89p95PUzSe7LmF/////4b4qkOnHf8USIk6UwpyN+9rRgi7st0tAXHmOuxqSJC0AQAAABcWABT+Pp7xp0XpdNkCxDVZQ6vLNL1TU/////8CAMLrCwAAAAAZdqkUhc/xCX/Z4Ai7NK9wnGIZeziXikiIrHL++E4sAAAAF6kUM5cluiHv1irHU6m80GfWx6ajnQWHAkcwRAIgJxK+IuAnDzlPVoMR3HyppolwuAJf3TskAinwf4pfOiQCIAGLONfc0xTnNMkna9b7QPZzMlvEuqFEyADS8vAtsnZcASED0uFWdJQbrUqZY3LLh+GFbTZSYG2YVi/jnF6efkE/IQUCSDBFAiEA0SuFLYXc2WHS9fSrZgZU327tzHlMDDPOXMMJ/7X85Y0CIGczio4OFyXBl/saiK9Z9R5E5CVbIBZ8hoQDHAXR8lkqASECI7cr7vCWXRC+B3jv7NYfysb3mk6haTkzgHNEZPhPKrMAAAAAAAAA"
        val psbt = PartiallySignedBitcoinTransaction(validSerializedPsbt)
        val psbtSerialized = psbt.serialize()
        assertEquals(psbtSerialized, validSerializedPsbt)
    }

    // TODO switch all tests to REGTEST, especially this one
    @Test
    fun walletTxBuilderBroadcast() {
        val descriptor =
            "wpkh([c1ed86ca/84'/1'/0'/0]tprv8hTkxK6QT7fCQx1wbuHuwbNh4STr2Ruz8RwEX7ymk6qnpixtbRG4T99mHxJwKTHPuKQ61heWrrpxZ8jpHj4sbisrQhDxnyx3HoQEZebtraN/*)"
        val wallet = Wallet(descriptor, null, Network.TESTNET, databaseConfig)
        val blockchain = Blockchain(blockchainConfig);
        wallet.sync(blockchain, LogProgress())
        val balance = wallet.getBalance()
        if (balance > 2000u) {
            println("balance $balance")
            // send coins back to https://bitcoinfaucet.uo1.net
            val faucetAddress = "tb1ql7w62elx9ucw4pj5lgw4l028hmuw80sndtntxt"
            val txBuilder = TxBuilder().addRecipient(faucetAddress, 1000u).feeRate(1.2f)
            val psbt = txBuilder.build(wallet)
            wallet.sign(psbt)
            blockchain.broadcast(psbt)
            val txid = psbt.txid()
            println("https://mempool.space/testnet/tx/$txid")
            assertNotNull(txid)
        } else {
            val depositAddress = wallet.getLastUnusedAddress()
            fail("Send more testnet coins to: $depositAddress")
        }
    }

    @Test(expected = BdkException.Generic::class)
    fun walletTxBuilderInvalidAddress() {
        val descriptor =
            "wpkh([c1ed86ca/84'/1'/0'/0]tprv8hTkxK6QT7fCQx1wbuHuwbNh4STr2Ruz8RwEX7ymk6qnpixtbRG4T99mHxJwKTHPuKQ61heWrrpxZ8jpHj4sbisrQhDxnyx3HoQEZebtraN/*)"
        val wallet = Wallet(descriptor, null, Network.TESTNET, databaseConfig)
        val txBuilder = TxBuilder().addRecipient("INVALID_ADDRESS", 1000u).feeRate(1.2f)
        txBuilder.build(wallet)
    }

      // Comment this test in for local testing, you will need let it fail ones to get an address
      // to pre-fund the test wallet before the test will pass.
//    @Test
//    fun walletTxBuilderDrainWallet() {
//        val descriptor =
//            "wpkh([8da6afbe/84'/1'/0'/0]tprv8hY7jbMbe17EH1cLw2feTyNDYvjcFYauLmbneBqVnDERBrV51LrxWjCYRZwWS5keYn3ijb7iHJuEzXQk7EzgPeKRBVNBgC4oFPDxGND5S3V/*)"
//        val wallet = Wallet(descriptor, null, Network.TESTNET, databaseConfig, blockchainConfig)
//        wallet.sync(LogProgress(), null)
//        val balance = wallet.getBalance()
//        if (balance > 2000u) {
//            println("balance $balance")
//            // send all coins back to https://bitcoinfaucet.uo1.net
//            val faucetAddress = "tb1ql7w62elx9ucw4pj5lgw4l028hmuw80sndtntxt"
//            val txBuilder = TxBuilder().drainWallet().drainTo(faucetAddress).feeRate(1.1f)
//            val psbt = txBuilder.build(wallet)
//            wallet.sign(psbt)
//            val txid = wallet.broadcast(psbt)
//            println("https://mempool.space/testnet/tx/$txid")
//            assertNotNull(txid)
//        } else {
//            val depositAddress = wallet.getLastUnusedAddress()
//            fail("Send more testnet coins to: $depositAddress")
//        }
//    }

}
