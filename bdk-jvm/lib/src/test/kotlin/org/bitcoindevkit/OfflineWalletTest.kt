package org.bitcoindevkit

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertTrue

class OfflineWalletTest {
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

        assertEquals(
            expected = "tb1qzg4mckdh50nwdm9hkzq06528rsu73hjxxzem3e",
            actual = addressInfo.address.asString()
        )
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

        assertEquals(
            expected = 0uL,
            actual = wallet.getBalance().total
        )
    }
}
