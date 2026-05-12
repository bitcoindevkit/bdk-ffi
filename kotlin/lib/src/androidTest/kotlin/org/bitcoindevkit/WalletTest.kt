package org.bitcoindevkit

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertTrue
import kotlin.test.assertFalse
import androidx.test.ext.junit.runners.AndroidJUnit4
import org.junit.runner.RunWith

@RunWith(AndroidJUnit4::class)
class WalletTest {
    val conn: Persister = Persister.newInMemory()

    // Wallet produces valid addresses for its network.
    @Test
    fun walletProducesValidAddresses() {
        val wallet: Wallet = Wallet(
            descriptor = BIP84_DESCRIPTOR,
            changeDescriptor = BIP84_CHANGE_DESCRIPTOR,
            network = Network.TESTNET,
            persister = conn
        )
        val addressInfo: AddressInfo = wallet.revealNextAddress(KeychainKind.EXTERNAL)

        assertTrue(addressInfo.address.isValidForNetwork(Network.TESTNET), "Address is not valid for Testnet 3 network but it should be")
        assertTrue(addressInfo.address.isValidForNetwork(Network.TESTNET4), "Address is not valid for Testnet 4 network but it should be")
        assertTrue(addressInfo.address.isValidForNetwork(Network.SIGNET), "Address is not valid for Signet network but it should be")

        assertFalse(addressInfo.address.isValidForNetwork(Network.REGTEST), "Address is valid for Regtest network, but it should not be")
        assertFalse(addressInfo.address.isValidForNetwork(Network.BITCOIN), "Address is valid for Mainnet network, but it should not be")
    }

    // Wallet has 0 balance prior to sync.
    @Test
    fun newWalletHas0Balance() {
        val wallet: Wallet = Wallet(
            descriptor = BIP84_DESCRIPTOR,
            changeDescriptor = BIP84_CHANGE_DESCRIPTOR,
            network = Network.TESTNET,
            persister = conn
        )

        assertEquals(
            expected = 0uL,
            actual = wallet.balance().total.toSat()
        )
    }

    // Single-descriptor wallets return an address on the external keychain even if a change descriptor is not provided
    // and the wallet.revealNextAddress(KeychainKind.INTERNAL) or wallet.peekAddress(KeychainKind.EXTERNAL, 0u) APIs are
    // used.
    @Test
    fun singleDescriptorWalletCanCreateAddresses() {
        val wallet: Wallet = Wallet.createSingle(
            descriptor = BIP84_DESCRIPTOR,
            network = Network.TESTNET,
            persister = conn
        )
        val address1 = wallet.peekAddress(KeychainKind.EXTERNAL, 0u)
        val address2 = wallet.peekAddress(KeychainKind.INTERNAL, 0u)

        assertEquals(
            expected = address1.address,
            actual = address2.address,
            "Addresses should be the same"
        )
    }
}
