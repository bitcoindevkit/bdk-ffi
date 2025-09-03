package org.bitcoindevkit

import kotlin.test.Test
import kotlin.test.assertFails
import androidx.test.ext.junit.runners.AndroidJUnit4
import org.junit.runner.RunWith

@RunWith(AndroidJUnit4::class)
class CreatingWalletTest {
    private val conn: Persister = Persister.newInMemory()

    // Create a WPKH wallet.
    @Test
    fun createWPKHWallet() {
        Wallet(
            descriptor = BIP84_DESCRIPTOR,
            changeDescriptor = BIP84_CHANGE_DESCRIPTOR,
            network = Network.TESTNET,
            persister = conn
        )
    }

    // Create a TR wallet.
    @Test
    fun createTRWallet() {
        Wallet(
            descriptor = BIP86_DESCRIPTOR,
            changeDescriptor = BIP86_CHANGE_DESCRIPTOR,
            network = Network.TESTNET,
            persister = conn
        )
    }

    // Create a wallet with a non-extended descriptor.
    @Test
    fun createWalletWithNonExtendedDescriptor() {
        Wallet(
            descriptor = NON_EXTENDED_DESCRIPTOR_0,
            changeDescriptor = NON_EXTENDED_DESCRIPTOR_1,
            network = Network.TESTNET,
            persister = conn
        )
    }

    // Descriptors do not match provided network.
    @Test
    fun failsIfDescriptorsDontMatchNetwork() {
        // The descriptors provided are for Testnet 3, but the wallet attempts to build for Mainnet
        assertFails {
            Wallet(
                descriptor = NON_EXTENDED_DESCRIPTOR_0,
                changeDescriptor = NON_EXTENDED_DESCRIPTOR_1,
                network = Network.BITCOIN,
                persister = conn
            )
        }
    }
}
