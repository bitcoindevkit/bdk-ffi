package org.bitcoindevkit

import kotlin.test.Test
import kotlin.test.assertEquals
import org.bitcoindevkit.bitcoinffi.Network

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
}
