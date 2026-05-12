package org.bitcoindevkit

import androidx.test.ext.junit.runners.AndroidJUnit4
import org.junit.runner.RunWith
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFailsWith
import kotlin.test.assertTrue

@RunWith(AndroidJUnit4::class)
class MnemonicTest {
    // Mnemonics create valid descriptors.
    @Test
    fun mnemonicsCreateValidDescriptors() {
        val mnemonic: Mnemonic = Mnemonic.fromString("space echo position wrist orient erupt relief museum myself grain wisdom tumble")
        val descriptorSecretKey: DescriptorSecretKey = DescriptorSecretKey(NetworkKind.TEST, mnemonic, null)
        val descriptor: Descriptor = Descriptor.newBip86(descriptorSecretKey, KeychainKind.EXTERNAL, NetworkKind.TEST)

        assertEquals(
            expected = "tr([be1eec8f/86'/1'/0']tpubDCTtszwSxPx3tATqDrsSyqScPNnUChwQAVAkanuDUCJQESGBbkt68nXXKRDifYSDbeMa2Xg2euKbXaU3YphvGWftDE7ozRKPriT6vAo3xsc/0/*)#m7puekcx",
            actual = descriptor.toString()
        )
    }

    @Test
    fun descriptorSecretKeyAddUnhardenedWildcard() {
        val mnemonic = Mnemonic.fromString("awesome awesome awesome awesome awesome awesome awesome awesome awesome awesome awesome awesome")
        val secretKey = DescriptorSecretKey(NetworkKind.TEST, mnemonic, null)
        val derived = secretKey.derive(DerivationPath("m/86'/1'/0'"))
        val withWildcard = derived.addWildcard(WildcardType.UNHARDENED)
        assertTrue(withWildcard.toString().endsWith("/*"))
    }

    @Test
    fun descriptorSecretKeyAddHardenedWildcard() {
        val mnemonic = Mnemonic.fromString("awesome awesome awesome awesome awesome awesome awesome awesome awesome awesome awesome awesome")
        val secretKey = DescriptorSecretKey(NetworkKind.TEST, mnemonic, null)
        val derived = secretKey.derive(DerivationPath("m/86'/1'/0'"))
        val withWildcard = derived.addWildcard(WildcardType.HARDENED)
        assertTrue(withWildcard.toString().endsWith("/*h"))
    }

    @Test
    fun descriptorSecretKeyAddWildcardIsIdempotent() {
        val mnemonic = Mnemonic.fromString("awesome awesome awesome awesome awesome awesome awesome awesome awesome awesome awesome awesome")
        val secretKey = DescriptorSecretKey(NetworkKind.TEST, mnemonic, null)
        val derived = secretKey.derive(DerivationPath("m/86'/1'/0'"))
        val withWildcard = derived.addWildcard(WildcardType.UNHARDENED)
        val withWildcardAgain = withWildcard.addWildcard(WildcardType.UNHARDENED)
        assertEquals(withWildcard.toString(), withWildcardAgain.toString())
    }

    @Test
    fun descriptorSecretKeyCannotChangeWildcardType() {
        val mnemonic = Mnemonic.fromString("awesome awesome awesome awesome awesome awesome awesome awesome awesome awesome awesome awesome")
        val secretKey = DescriptorSecretKey(NetworkKind.TEST, mnemonic, null)
        val derived = secretKey.derive(DerivationPath("m/86'/1'/0'"))
        val withWildcard = derived.addWildcard(WildcardType.UNHARDENED)
        assertFailsWith<DescriptorKeyException> {
            withWildcard.addWildcard(WildcardType.HARDENED)
        }
    }

    @Test
    fun descriptorPublicKeyAddWildcard() {
        val mnemonic = Mnemonic.fromString("awesome awesome awesome awesome awesome awesome awesome awesome awesome awesome awesome awesome")
        val secretKey = DescriptorSecretKey(NetworkKind.TEST, mnemonic, null)
        val derived = secretKey.derive(DerivationPath("m/86'/1'/0'"))
        val publicKey = derived.asPublic()
        val withWildcard = publicKey.addWildcard()
        assertTrue(withWildcard.toString().endsWith("/*"))
    }
}
