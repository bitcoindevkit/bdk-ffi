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
        Descriptor("wpkh($TEST_EXTENDED_PRIVKEY/$BIP84_TEST_RECEIVE_PATH/*)", NetworkKind.TEST)
        Descriptor("wpkh($MAINNET_EXTENDED_PRIVKEY/$BIP84_MAINNET_RECEIVE_PATH/*)", NetworkKind.MAIN)
    }

    // Create extended TR descriptors for all networks.
    @Test
    fun createExtendedTRDescriptors() {
        Descriptor("tr($TEST_EXTENDED_PRIVKEY/$BIP86_TEST_RECEIVE_PATH/*)", NetworkKind.TEST)
        Descriptor("tr($MAINNET_EXTENDED_PRIVKEY/$BIP86_MAINNET_RECEIVE_PATH/*)", NetworkKind.MAIN)
    }

    // Create non-extended descriptors for all networks.
    @Test
    fun createNonExtendedDescriptors() {
        Descriptor("tr($TEST_EXTENDED_PRIVKEY/$BIP86_TEST_RECEIVE_PATH/0)", NetworkKind.TEST)
        Descriptor("tr($MAINNET_EXTENDED_PRIVKEY/$BIP86_MAINNET_RECEIVE_PATH/0)", NetworkKind.MAIN)
    }

    // Create a multisig descriptor.
    @Test
    fun createMultisigDescriptor() {
        val pkList = listOf(
            "xpub661MyMwAqRbcFW31YEwpkMuc5THy2PSt5bDMsktWQcFF8syAmRUapSCGu8ED9W6oDMSgv6Zz8idoc4a6mr8BDzTJY47LJhkJ8UB7WEGuduB",
            "xpub661MyMwAqRbcFW31YEwpkMuc5THy2PSt5bDMsktWQcFF8syAmRUapSCGu8ED9W6oDMSgv6Zz8idoc4a6mr8BDzTJY47LJhkJ8UB7WEGuduB",
            "xpub661MyMwAqRbcFW31YEwpkMuc5THy2PSt5bDMsktWQcFF8syAmRUapSCGu8ED9W6oDMSgv6Zz8idoc4a6mr8BDzTJY47LJhkJ8UB7WEGuduB"
        )
        val wshSortedMultiDescriptor = Descriptor.newWshSortedmulti(2u, pkList);
        val shWshSortedMultiDescriptor = Descriptor.newShWshSortedmulti(2u, pkList);
        val shSortedMultiDescriptor = Descriptor.newShSortedmulti(2u, pkList);

        assertEquals(wshSortedMultiDescriptor.descType(), DescriptorType.WSH_SORTED_MULTI)
        assertEquals(shWshSortedMultiDescriptor.descType(), DescriptorType.SH_WSH_SORTED_MULTI)
        assertEquals(shSortedMultiDescriptor.descType(), DescriptorType.SH_SORTED_MULTI)
    }

    @Test
    fun createInvalidMultisigDescriptor() {
        val invalidPkList = listOf(
            "invalid_xpub",
            "1111111111111",
            "***"
        )
        assertFails {
            Descriptor.newWshSortedmulti(2u, invalidPkList);
        }
    }

    @Test
    fun createSingleKeyDescriptor() {
        val newPkDescriptor = Descriptor.newPk("xpub661MyMwAqRbcFW31YEwpkMuc5THy2PSt5bDMsktWQcFF8syAmRUapSCGu8ED9W6oDMSgv6Zz8idoc4a6mr8BDzTJY47LJhkJ8UB7WEGuduB")
        val newPkhDescriptor = Descriptor.newPkh("xpub661MyMwAqRbcFW31YEwpkMuc5THy2PSt5bDMsktWQcFF8syAmRUapSCGu8ED9W6oDMSgv6Zz8idoc4a6mr8BDzTJY47LJhkJ8UB7WEGuduB")
        val newWpkhDescriptor = Descriptor.newWpkh("xpub661MyMwAqRbcFW31YEwpkMuc5THy2PSt5bDMsktWQcFF8syAmRUapSCGu8ED9W6oDMSgv6Zz8idoc4a6mr8BDzTJY47LJhkJ8UB7WEGuduB")
        val newShWpkhDescriptor = Descriptor.newShWpkh("xpub661MyMwAqRbcFW31YEwpkMuc5THy2PSt5bDMsktWQcFF8syAmRUapSCGu8ED9W6oDMSgv6Zz8idoc4a6mr8BDzTJY47LJhkJ8UB7WEGuduB")

        assertEquals(newPkDescriptor.descType(), DescriptorType.BARE)
        assertEquals(newPkhDescriptor.descType(), DescriptorType.PKH)
        assertEquals(newWpkhDescriptor.descType(), DescriptorType.WPKH)
        assertEquals(newShWpkhDescriptor.descType(), DescriptorType.SH_WPKH)
    }

    @Test
    fun createMiniscriptDescriptors() {
        val newShDescriptor = Descriptor.newSh("pk(02b4632d08485ff1df2db55b9dafd23347d1c47a457072a1e87be26896549a8737)")
        val newWshDescriptor = Descriptor.newWsh("pk(02b4632d08485ff1df2db55b9dafd23347d1c47a457072a1e87be26896549a8737)")
        val newShWshDescriptor = Descriptor.newShWsh("pk(02b4632d08485ff1df2db55b9dafd23347d1c47a457072a1e87be26896549a8737)")
        val newBareDescriptor = Descriptor.newBare("pk(02b4632d08485ff1df2db55b9dafd23347d1c47a457072a1e87be26896549a8737)")

        assertEquals(newShDescriptor.descType(), DescriptorType.SH)
        assertEquals(newWshDescriptor.descType(), DescriptorType.WSH)
        assertEquals(newShWshDescriptor.descType(), DescriptorType.SH_WSH)
        assertEquals(newBareDescriptor.descType(), DescriptorType.BARE)
    }

    @Test
    fun miniscriptConstructorsRejectInvalidExpressions() {
        val invalid = "not_a_valid_miniscript"

        assertFailsWith<DescriptorException.Miniscript> { Descriptor.newWsh(invalid) }
        assertFailsWith<DescriptorException.Miniscript> { Descriptor.newSh(invalid) }
        assertFailsWith<DescriptorException.Miniscript> { Descriptor.newShWsh(invalid) }
        assertFailsWith<DescriptorException.Miniscript> { Descriptor.newBare(invalid) }
    }

    // BareCtx only allows pk(), pkh(), and multi(k<=3,...) at the top level. A timelock
    // conjunction is valid miniscript for new_wsh but is rejected by new_bare.
    @Test
    fun notAllMiniscriptsWorkEverywhere() {
        val compressedPk = "02b4632d08485ff1df2db55b9dafd23347d1c47a457072a1e87be26896549a8737"
        val timelockConjunction = "and_v(v:pk($compressedPk),after(1000))"

        // println("Complex miniscript: $timelockConjunction")
        // println("Resulting descriptor: ${Descriptor.newWsh(timelockConjunction)}")

        // Can do
        Descriptor.newSh(timelockConjunction)
        Descriptor.newWsh(timelockConjunction)

        // No can do
        assertFailsWith<DescriptorException.Miniscript> {
            Descriptor.newBare(timelockConjunction)
            Descriptor.newWpkh(timelockConjunction)
            Descriptor.newWsh(timelockConjunction)
            Descriptor.newPkh(timelockConjunction)
        }
    }

    // Segwitv0 (new_wsh) requires all keys to be compressed. An uncompressed key is valid
    // in Legacy context (new_sh) but rejected by new_wsh.
    @Test
    fun newWshRejectsUncompressedKey() {
        // Satoshi's genesis block public key — a well-known uncompressed key.
        val uncompressedPk = "04678afdb0fe5548271967f1a67130b7105cd6a828e03909a67962e0ea1f61deb649f6bc3f4cef38c4f35504e51ec112de5c384df7ba0b8d578a4c702b6bf11d5f"
        val expression = "pk($uncompressedPk)"

        // Can do
        Descriptor.newSh(expression)

        // No can do
        assertFailsWith<DescriptorException.Miniscript> {
            Descriptor.newWsh(expression)
        }
    }

    // Cannot create addr() descriptor.
    @Test
    fun cannotCreateAddrDescriptor() {
        assertFails {
            val descriptor: Descriptor = Descriptor(
                "addr(tb1qhjys9wxlfykmte7ftryptx975uqgd6kcm6a7z4)",
                NetworkKind.TEST
            )
        }
    }

    @Test
    fun sanityCheck() {
        val safeDescriptor = Descriptor("multi(1,L4rK1yDtCWekvXuE6oXD9jCYfFNV2cWRpVuPLBcCU2z8TrisoyY1,5KYZdUEo39z3FPrtuX2QbbwGnNP5zTd7yyr2SC1j299sBCnWjss)", NetworkKind.MAIN)

        safeDescriptor.sanityCheck()

        //multi(2, key1, key1)) is structurally valid but unsafe
        val unsafeDescriptor = Descriptor("multi(2,L4rK1yDtCWekvXuE6oXD9jCYfFNV2cWRpVuPLBcCU2z8TrisoyY1,L4rK1yDtCWekvXuE6oXD9jCYfFNV2cWRpVuPLBcCU2z8TrisoyY1)", NetworkKind.MAIN)

        // descriptor failed as expected because it uses the same public key twice in a 2 of 2 multisig
        assertFailsWith<DescriptorException.Miniscript> {
            unsafeDescriptor.sanityCheck()
        }
    }

    @Test
    fun checkDescriptorWildcard(){
        val wildcardDescriptor = Descriptor("wpkh($TEST_EXTENDED_PRIVKEY/$BIP84_TEST_RECEIVE_PATH/*)", NetworkKind.TEST)
        val nonWildcardDescriptor = Descriptor("wpkh($TEST_EXTENDED_PRIVKEY/$BIP84_TEST_RECEIVE_PATH/0)", NetworkKind.TEST)

        assertTrue(wildcardDescriptor.hasWildcard())

        assertFalse(nonWildcardDescriptor.hasWildcard())
    }

    @Test
    fun testDescriptorTypes() {
        // Taken from the BIPs: https://github.com/bitcoin/bips/blob/master/bip-0380.mediawiki

        val descriptor1 = Descriptor("pk(L4rK1yDtCWekvXuE6oXD9jCYfFNV2cWRpVuPLBcCU2z8TrisoyY1)", NetworkKind.TEST)
        val descriptor2 = Descriptor("pkh([deadbeef/1/2'/3/4']L4rK1yDtCWekvXuE6oXD9jCYfFNV2cWRpVuPLBcCU2z8TrisoyY1)", NetworkKind.TEST)
        val descriptor3 = Descriptor("sh(pk(03a34b99f22c790c4e36b2b3c2c35a36db06226e41c692fc82b8b56ac1c540c5bd))", NetworkKind.TEST)
        val descriptor4 = Descriptor("wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/0/*)", NetworkKind.TEST)
        val descriptor5 = Descriptor("sh(wpkh(xprv9s21ZrQH143K3QTDL4LXw2F7HEK3wJUD2nW2nRk4stbPy6cq3jPPqjiChkVvvNKmPGJxWUtg6LnF5kejMRNNU3TGtRBeJgk33yuGBxrMPHi/10/20/30/40/*h))", NetworkKind.MAIN)
        val descriptor6 = Descriptor("multi(1,L4rK1yDtCWekvXuE6oXD9jCYfFNV2cWRpVuPLBcCU2z8TrisoyY1,5KYZdUEo39z3FPrtuX2QbbwGnNP5zTd7yyr2SC1j299sBCnWjss)", NetworkKind.MAIN)
        val descriptor7 = Descriptor("tr(a34b99f22c790c4e36b2b3c2c35a36db06226e41c692fc82b8b56ac1c540c5bd)", NetworkKind.MAIN)
        val descriptor8 = Descriptor("sh(multi(2,[00000000/111'/222]xprvA1RpRA33e1JQ7ifknakTFpgNXPmW2YvmhqLQYMmrj4xJXXWYpDPS3xz7iAxn8L39njGVyuoseXzU6rcxFLJ8HFsTjSyQbLYnMpCqE2VbFWc,xprv9uPDJpEQgRQfDcW7BkF7eTya6RPxXeJCqCJGHuCJ4GiRVLzkTXBAJMu2qaMWPrS7AANYqdq6vcBcBUdJCVVFceUvJFjaPdGZ2y9WACViL4L/0))", NetworkKind.MAIN)
        val descriptor9 = Descriptor("sh(sortedmulti(2,[00000000/111'/222]xprvA1RpRA33e1JQ7ifknakTFpgNXPmW2YvmhqLQYMmrj4xJXXWYpDPS3xz7iAxn8L39njGVyuoseXzU6rcxFLJ8HFsTjSyQbLYnMpCqE2VbFWc,xprv9uPDJpEQgRQfDcW7BkF7eTya6RPxXeJCqCJGHuCJ4GiRVLzkTXBAJMu2qaMWPrS7AANYqdq6vcBcBUdJCVVFceUvJFjaPdGZ2y9WACViL4L/0))", NetworkKind.MAIN)
        val descriptor10 = Descriptor("wsh(sortedmulti(2,[00000000/111'/222]xprvA1RpRA33e1JQ7ifknakTFpgNXPmW2YvmhqLQYMmrj4xJXXWYpDPS3xz7iAxn8L39njGVyuoseXzU6rcxFLJ8HFsTjSyQbLYnMpCqE2VbFWc,xprv9uPDJpEQgRQfDcW7BkF7eTya6RPxXeJCqCJGHuCJ4GiRVLzkTXBAJMu2qaMWPrS7AANYqdq6vcBcBUdJCVVFceUvJFjaPdGZ2y9WACViL4L/0))", NetworkKind.MAIN)
        val descriptor11 = Descriptor("sh(wsh(sortedmulti(2,[00000000/111'/222]xprvA1RpRA33e1JQ7ifknakTFpgNXPmW2YvmhqLQYMmrj4xJXXWYpDPS3xz7iAxn8L39njGVyuoseXzU6rcxFLJ8HFsTjSyQbLYnMpCqE2VbFWc,xprv9uPDJpEQgRQfDcW7BkF7eTya6RPxXeJCqCJGHuCJ4GiRVLzkTXBAJMu2qaMWPrS7AANYqdq6vcBcBUdJCVVFceUvJFjaPdGZ2y9WACViL4L/0)))", NetworkKind.MAIN)
        val descriptor12 = Descriptor("wsh(multi(2,[00000000/111'/222]xprvA1RpRA33e1JQ7ifknakTFpgNXPmW2YvmhqLQYMmrj4xJXXWYpDPS3xz7iAxn8L39njGVyuoseXzU6rcxFLJ8HFsTjSyQbLYnMpCqE2VbFWc,xprv9uPDJpEQgRQfDcW7BkF7eTya6RPxXeJCqCJGHuCJ4GiRVLzkTXBAJMu2qaMWPrS7AANYqdq6vcBcBUdJCVVFceUvJFjaPdGZ2y9WACViL4L/0))", NetworkKind.MAIN)
        val descriptor13 = Descriptor("sh(wsh(multi(2,[00000000/111'/222]xprvA1RpRA33e1JQ7ifknakTFpgNXPmW2YvmhqLQYMmrj4xJXXWYpDPS3xz7iAxn8L39njGVyuoseXzU6rcxFLJ8HFsTjSyQbLYnMpCqE2VbFWc,xprv9uPDJpEQgRQfDcW7BkF7eTya6RPxXeJCqCJGHuCJ4GiRVLzkTXBAJMu2qaMWPrS7AANYqdq6vcBcBUdJCVVFceUvJFjaPdGZ2y9WACViL4L/0)))", NetworkKind.MAIN)

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
