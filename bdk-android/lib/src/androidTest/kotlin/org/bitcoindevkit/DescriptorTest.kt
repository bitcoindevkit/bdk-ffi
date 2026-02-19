package org.bitcoindevkit

import kotlin.test.Test
import androidx.test.ext.junit.runners.AndroidJUnit4
import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.runner.RunWith
import kotlin.test.assertEquals
import kotlin.test.assertFails
import kotlin.test.assertFailsWith

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

    @Test
    fun sanityCheck() {
        val safeDescriptor = Descriptor("multi(1,L4rK1yDtCWekvXuE6oXD9jCYfFNV2cWRpVuPLBcCU2z8TrisoyY1,5KYZdUEo39z3FPrtuX2QbbwGnNP5zTd7yyr2SC1j299sBCnWjss)", Network.BITCOIN)

        safeDescriptor.sanityCheck()

        //multi(2, key1, key1)) is structurally valid but unsafe
        val unsafeDescriptor = Descriptor("multi(2,L4rK1yDtCWekvXuE6oXD9jCYfFNV2cWRpVuPLBcCU2z8TrisoyY1,L4rK1yDtCWekvXuE6oXD9jCYfFNV2cWRpVuPLBcCU2z8TrisoyY1)", Network.BITCOIN)

        // descriptor failed as expected because it uses the same public key twice in a 2 of 2 multisig
        assertFailsWith<DescriptorException.Miniscript> {
            unsafeDescriptor.sanityCheck()
        }
    }

    @Test
    fun checkDescriptorWildcard(){
        val wildcardDescriptor = Descriptor("wpkh($TEST_EXTENDED_PRIVKEY/$BIP84_TEST_RECEIVE_PATH/*)", Network.REGTEST)
        val nonWildcardDescriptor = Descriptor("wpkh($TEST_EXTENDED_PRIVKEY/$BIP84_TEST_RECEIVE_PATH/0)", Network.REGTEST)

        assertTrue(wildcardDescriptor.hasWildcard())

        assertFalse(nonWildcardDescriptor.hasWildcard())
    }

    @Test
    fun testDescriptorTypes() {
        // Taken from the BIPs: https://github.com/bitcoin/bips/blob/master/bip-0380.mediawiki

        val descriptor1 = Descriptor("pk(L4rK1yDtCWekvXuE6oXD9jCYfFNV2cWRpVuPLBcCU2z8TrisoyY1)", Network.SIGNET)
        val descriptor2 = Descriptor("pkh([deadbeef/1/2'/3/4']L4rK1yDtCWekvXuE6oXD9jCYfFNV2cWRpVuPLBcCU2z8TrisoyY1)", Network.SIGNET)
        val descriptor3 = Descriptor("sh(pk(03a34b99f22c790c4e36b2b3c2c35a36db06226e41c692fc82b8b56ac1c540c5bd))", Network.SIGNET)
        val descriptor4 = Descriptor("wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/0/*)", Network.SIGNET)
        val descriptor5 = Descriptor("sh(wpkh(xprv9s21ZrQH143K3QTDL4LXw2F7HEK3wJUD2nW2nRk4stbPy6cq3jPPqjiChkVvvNKmPGJxWUtg6LnF5kejMRNNU3TGtRBeJgk33yuGBxrMPHi/10/20/30/40/*h))", Network.BITCOIN)
        val descriptor6 = Descriptor("multi(1,L4rK1yDtCWekvXuE6oXD9jCYfFNV2cWRpVuPLBcCU2z8TrisoyY1,5KYZdUEo39z3FPrtuX2QbbwGnNP5zTd7yyr2SC1j299sBCnWjss)", Network.BITCOIN)
        val descriptor7 = Descriptor("tr(a34b99f22c790c4e36b2b3c2c35a36db06226e41c692fc82b8b56ac1c540c5bd)", Network.BITCOIN)
        val descriptor8 = Descriptor("sh(multi(2,[00000000/111'/222]xprvA1RpRA33e1JQ7ifknakTFpgNXPmW2YvmhqLQYMmrj4xJXXWYpDPS3xz7iAxn8L39njGVyuoseXzU6rcxFLJ8HFsTjSyQbLYnMpCqE2VbFWc,xprv9uPDJpEQgRQfDcW7BkF7eTya6RPxXeJCqCJGHuCJ4GiRVLzkTXBAJMu2qaMWPrS7AANYqdq6vcBcBUdJCVVFceUvJFjaPdGZ2y9WACViL4L/0))", Network.BITCOIN)
        val descriptor9 = Descriptor("sh(sortedmulti(2,[00000000/111'/222]xprvA1RpRA33e1JQ7ifknakTFpgNXPmW2YvmhqLQYMmrj4xJXXWYpDPS3xz7iAxn8L39njGVyuoseXzU6rcxFLJ8HFsTjSyQbLYnMpCqE2VbFWc,xprv9uPDJpEQgRQfDcW7BkF7eTya6RPxXeJCqCJGHuCJ4GiRVLzkTXBAJMu2qaMWPrS7AANYqdq6vcBcBUdJCVVFceUvJFjaPdGZ2y9WACViL4L/0))", Network.BITCOIN)
        val descriptor10 = Descriptor("wsh(sortedmulti(2,[00000000/111'/222]xprvA1RpRA33e1JQ7ifknakTFpgNXPmW2YvmhqLQYMmrj4xJXXWYpDPS3xz7iAxn8L39njGVyuoseXzU6rcxFLJ8HFsTjSyQbLYnMpCqE2VbFWc,xprv9uPDJpEQgRQfDcW7BkF7eTya6RPxXeJCqCJGHuCJ4GiRVLzkTXBAJMu2qaMWPrS7AANYqdq6vcBcBUdJCVVFceUvJFjaPdGZ2y9WACViL4L/0))", Network.BITCOIN)
        val descriptor11 = Descriptor("sh(wsh(sortedmulti(2,[00000000/111'/222]xprvA1RpRA33e1JQ7ifknakTFpgNXPmW2YvmhqLQYMmrj4xJXXWYpDPS3xz7iAxn8L39njGVyuoseXzU6rcxFLJ8HFsTjSyQbLYnMpCqE2VbFWc,xprv9uPDJpEQgRQfDcW7BkF7eTya6RPxXeJCqCJGHuCJ4GiRVLzkTXBAJMu2qaMWPrS7AANYqdq6vcBcBUdJCVVFceUvJFjaPdGZ2y9WACViL4L/0)))", Network.BITCOIN)
        val descriptor12 = Descriptor("wsh(multi(2,[00000000/111'/222]xprvA1RpRA33e1JQ7ifknakTFpgNXPmW2YvmhqLQYMmrj4xJXXWYpDPS3xz7iAxn8L39njGVyuoseXzU6rcxFLJ8HFsTjSyQbLYnMpCqE2VbFWc,xprv9uPDJpEQgRQfDcW7BkF7eTya6RPxXeJCqCJGHuCJ4GiRVLzkTXBAJMu2qaMWPrS7AANYqdq6vcBcBUdJCVVFceUvJFjaPdGZ2y9WACViL4L/0))", Network.BITCOIN)
        val descriptor13 = Descriptor("sh(wsh(multi(2,[00000000/111'/222]xprvA1RpRA33e1JQ7ifknakTFpgNXPmW2YvmhqLQYMmrj4xJXXWYpDPS3xz7iAxn8L39njGVyuoseXzU6rcxFLJ8HFsTjSyQbLYnMpCqE2VbFWc,xprv9uPDJpEQgRQfDcW7BkF7eTya6RPxXeJCqCJGHuCJ4GiRVLzkTXBAJMu2qaMWPrS7AANYqdq6vcBcBUdJCVVFceUvJFjaPdGZ2y9WACViL4L/0)))", Network.BITCOIN)

        assertEquals(
            expected = DescriptorType.BARE,
            actual = descriptor1.descType()
        )
        assertEquals(
            expected = DescriptorType.PKH,
            actual = descriptor2.descType()
        )
        assertEquals(
            expected = DescriptorType.SH,
            actual = descriptor3.descType()
        )
        assertEquals(
            expected = DescriptorType.WPKH,
            actual = descriptor4.descType()
        )
        assertEquals(
            expected = DescriptorType.SH_WPKH,
            actual = descriptor5.descType()
        )
        assertEquals(
            expected = DescriptorType.BARE,
            actual = descriptor6.descType()
        )
        assertEquals(
            expected = DescriptorType.TR,
            actual = descriptor7.descType()
        )
        assertEquals(
            expected = DescriptorType.SH,
            actual = descriptor8.descType()
        )
        assertEquals(
            expected = DescriptorType.SH_SORTED_MULTI,
            actual = descriptor9.descType()
        )
        assertEquals(
            expected = DescriptorType.WSH_SORTED_MULTI,
            actual = descriptor10.descType()
        )
        assertEquals(
            expected = DescriptorType.SH_WSH_SORTED_MULTI,
            actual = descriptor11.descType()
        )
        assertEquals(
            expected = DescriptorType.WSH,
            actual = descriptor12.descType()
        )
        assertEquals(
            expected = DescriptorType.SH_WSH,
            actual = descriptor13.descType()
        )
    }

}
