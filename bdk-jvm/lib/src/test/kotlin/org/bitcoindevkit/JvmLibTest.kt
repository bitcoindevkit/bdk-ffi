package org.bitcoindevkit

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertTrue

class WalletTest {

    @Test
    fun testDescriptorBip86() {
        val mnemonic: Mnemonic = Mnemonic(WordCount.WORDS12)
        val descriptorSecretKey: DescriptorSecretKey = DescriptorSecretKey(Network.TESTNET, mnemonic, null)
        val descriptor: Descriptor = Descriptor.newBip86(descriptorSecretKey, KeychainKind.EXTERNAL, Network.TESTNET)

        assertTrue(descriptor.asString().startsWith("tr"), "Bip86 Descriptor does not start with 'tr'")
    }

   @Test
    fun testNewAddress() {
        val descriptor: Descriptor = Descriptor(
            "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/*)",
            Network.TESTNET
        )
        val wallet: Wallet = Wallet.newNoPersist(
            descriptor,
            null,
            Network.TESTNET
        )
        val addressInfo: AddressInfo = wallet.getAddress(AddressIndex.New)

        assertEquals("tb1qzg4mckdh50nwdm9hkzq06528rsu73hjxxzem3e", addressInfo.address.asString())
    }

    @Test
    fun testBalance() {
        val descriptor: Descriptor = Descriptor(
            "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/*)",
            Network.TESTNET
        )
        val wallet: Wallet = Wallet.newNoPersist(
            descriptor,
            null,
            Network.TESTNET
        )

        assertEquals(0uL, wallet.getBalance().total())
    }

    // @Test
    // fun testSyncedBalance() {
    //     val descriptor = Descriptor("wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/0/*)", Network.TESTNET)
    //     val wallet = Wallet.newNoPersist(descriptor, null, Network.TESTNET)
    //     val esploraClient = EsploraClient("https://mempool.space/testnet/api")
    //     // val esploraClient = EsploraClient("https://blockstream.info/testnet/api")
    //     val update = esploraClient.scan(wallet, 10uL, 1uL)
    //     wallet.applyUpdate(update)
    //     println("Balance: ${wallet.getBalance().total()}")
    // }

}
