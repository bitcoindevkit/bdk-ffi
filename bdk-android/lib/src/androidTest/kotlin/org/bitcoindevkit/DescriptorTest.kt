package org.bitcoindevkit

import kotlin.test.Test
import androidx.test.ext.junit.runners.AndroidJUnit4
import org.junit.runner.RunWith
import kotlin.test.assertFails

@RunWith(AndroidJUnit4::class)
class DescriptorTest {
    // Create extended WPKH descriptors for all networks.
    @Test
    fun createExtendedWPKHDescriptors() {
        Descriptor("wpkh($TEST_EXTENDED_PRIVKEY/$BIP84_TEST_RECEIVE_PATH/*)", Network.REGTEST)
        Descriptor("wpkh($TEST_EXTENDED_PRIVKEY/$BIP84_TEST_RECEIVE_PATH/*)", Network.TESTNET)
        Descriptor("wpkh($TEST_EXTENDED_PRIVKEY/$BIP84_TEST_RECEIVE_PATH/*)", Network.TESTNET4)
        Descriptor("wpkh($TEST_EXTENDED_PRIVKEY/$BIP84_TEST_RECEIVE_PATH/*)", Network.SIGNET)
        Descriptor("wpkh($MAINNET_EXTENDED_PRIVKEY/$BIP84_MAINNET_RECEIVE_PATH/*)", Network.BITCOIN)
    }

    // Create extended TR descriptors for all networks.
    @Test
    fun createExtendedTRDescriptors() {
        Descriptor("tr($TEST_EXTENDED_PRIVKEY/$BIP86_TEST_RECEIVE_PATH/*)", Network.REGTEST)
        Descriptor("tr($TEST_EXTENDED_PRIVKEY/$BIP86_TEST_RECEIVE_PATH/*)", Network.TESTNET)
        Descriptor("tr($TEST_EXTENDED_PRIVKEY/$BIP86_TEST_RECEIVE_PATH/*)", Network.TESTNET4)
        Descriptor("tr($TEST_EXTENDED_PRIVKEY/$BIP86_TEST_RECEIVE_PATH/*)", Network.SIGNET)
        Descriptor("tr($MAINNET_EXTENDED_PRIVKEY/$BIP86_MAINNET_RECEIVE_PATH/*)", Network.BITCOIN)
    }

    // Create non-extended descriptors for all networks.
    @Test
    fun createNonExtendedDescriptors() {
        Descriptor("tr($TEST_EXTENDED_PRIVKEY/$BIP86_TEST_RECEIVE_PATH/0)", Network.REGTEST)
        Descriptor("tr($TEST_EXTENDED_PRIVKEY/$BIP86_TEST_RECEIVE_PATH/0)", Network.TESTNET)
        Descriptor("tr($TEST_EXTENDED_PRIVKEY/$BIP86_TEST_RECEIVE_PATH/0)", Network.TESTNET4)
        Descriptor("tr($TEST_EXTENDED_PRIVKEY/$BIP86_TEST_RECEIVE_PATH/0)", Network.SIGNET)
        Descriptor("tr($MAINNET_EXTENDED_PRIVKEY/$BIP86_MAINNET_RECEIVE_PATH/0)", Network.BITCOIN)
    }

    // Cannot create addr() descriptor.
    @Test
    fun cannotCreateAddrDescriptor() {
        assertFails {
            val descriptor: Descriptor = Descriptor(
                "addr(tb1qhjys9wxlfykmte7ftryptx975uqgd6kcm6a7z4)",
                Network.TESTNET
            )
        }
    }
}
