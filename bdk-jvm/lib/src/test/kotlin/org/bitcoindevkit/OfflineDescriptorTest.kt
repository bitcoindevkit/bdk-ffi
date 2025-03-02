package org.bitcoindevkit

import kotlin.test.Test
import kotlin.test.assertEquals

class OfflineDescriptorTest {
    @Test
    fun testDescriptorBip86() {
        val mnemonic: Mnemonic = Mnemonic.fromString("space echo position wrist orient erupt relief museum myself grain wisdom tumble")
        val descriptorSecretKey: DescriptorSecretKey = DescriptorSecretKey(Network.TESTNET, mnemonic, null)
        val descriptor: Descriptor = Descriptor.newBip86(descriptorSecretKey, KeychainKind.EXTERNAL, Network.TESTNET)

        assertEquals(
            expected = "tr([be1eec8f/86'/1'/0']tpubDCTtszwSxPx3tATqDrsSyqScPNnUChwQAVAkanuDUCJQESGBbkt68nXXKRDifYSDbeMa2Xg2euKbXaU3YphvGWftDE7ozRKPriT6vAo3xsc/0/*)#m7puekcx",
            actual = descriptor.toString()
        )
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
        // val descriptor7 = Descriptor("sortedmulti(1,04a34b99f22c790c4e36b2b3c2c35a36db06226e41c692fc82b8b56ac1c540c5bd5b8dec5235a0fa8722476c7709c02559e3aa73aa03918ba2d492eea75abea235,03a34b99f22c790c4e36b2b3c2c35a36db06226e41c692fc82b8b56ac1c540c5bd)", Network.BITCOIN)
        // val descriptor8 = Descriptor("combo(04a34b99f22c790c4e36b2b3c2c35a36db06226e41c692fc82b8b56ac1c540c5bd5b8dec5235a0fa8722476c7709c02559e3aa73aa03918ba2d492eea75abea235)", Network.BITCOIN)
        // val descriptor9 = Descriptor("raw(512103a34b99f22c790c4e36b2b3c2c35a36db06226e41c692fc82b8b56ac1c540c5bd4104a34b99f22c790c4e36b2b3c2c35a36db06226e41c692fc82b8b56ac1c540c5bd5b8dec5235a0fa8722476c7709c02559e3aa73aa03918ba2d492eea75abea23552ae)", Network.BITCOIN)
        // val descriptor10 = Descriptor("addr(3PUNyaW7M55oKWJ3kDukwk9bsKvryra15j)", Network.BITCOIN)
        val descriptor11 = Descriptor("tr(a34b99f22c790c4e36b2b3c2c35a36db06226e41c692fc82b8b56ac1c540c5bd)", Network.BITCOIN)
        val descriptor12 = Descriptor("sh(multi(2,[00000000/111'/222]xprvA1RpRA33e1JQ7ifknakTFpgNXPmW2YvmhqLQYMmrj4xJXXWYpDPS3xz7iAxn8L39njGVyuoseXzU6rcxFLJ8HFsTjSyQbLYnMpCqE2VbFWc,xprv9uPDJpEQgRQfDcW7BkF7eTya6RPxXeJCqCJGHuCJ4GiRVLzkTXBAJMu2qaMWPrS7AANYqdq6vcBcBUdJCVVFceUvJFjaPdGZ2y9WACViL4L/0))", Network.BITCOIN)

        println(descriptor1.descType())
        println(descriptor2.descType())
        println(descriptor3.descType())
        println(descriptor4.descType())
        println(descriptor5.descType())
        println(descriptor6.descType())
        // println(descriptor7.descType())
        // println(descriptor8.descType())
        // println(descriptor9.descType())
        // println(descriptor10.descType())
        println(descriptor11.descType())
        println(descriptor12.descType())

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
            actual = descriptor11.descType()
        )
        assertEquals(
            expected = DescriptorType.SH,
            actual = descriptor12.descType()
        )
    }
}
