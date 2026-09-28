package org.bitcoindevkit

import androidx.test.ext.junit.runners.AndroidJUnit4
import org.junit.runner.RunWith
import kotlin.test.Test
import kotlin.test.assertEquals

// The expected bytes are the private keys embedded in the xprv/WIF strings published in the BIP
// test vectors, so secretBytes() is checked against values that do not come from this library.
@RunWith(AndroidJUnit4::class)
class DescriptorSecretKeyTest {
    private fun ByteArray.toHex(): String = joinToString("") { "%02x".format(it) }

    private val abandonMnemonic = Mnemonic.fromString(
        "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"
    )

    // BIP-32 test vector 1, seed 000102030405060708090a0b0c0d0e0f.
    // https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki#test-vector-1
    @Test
    fun secretBytesMatchBip32TestVector1() {
        val master = DescriptorSecretKey.fromString(
            "xprv9s21ZrQH143K3QTDL4LXw2F7HEK3wJUD2nW2nRk4stbPy6cq3jPPqjiChkVvvNKmPGJxWUtg6LnF5kejMRNNU3TGtRBeJgk33yuGBxrMPHi"
        )
        assertEquals(
            expected = "e8f32e723decf4051aefac8e2c93c9c5b214313817cdb01a1494b917c8436b35",
            actual = master.secretBytes().toHex()
        )

        val vectors = mapOf(
            "m/0h/1/2h" to "cbce0d719ecf7431d88e6a89fa1483e02e35092af60c042b1df2ff59fa424dca",
            "m/0h/1/2h/2/1000000000" to "471b76e389e528d6de6d816857e012c5455051cad6660850e58372a6c3e6e7c8",
        )
        for ((path, expected) in vectors) {
            assertEquals(expected, master.extend(DerivationPath(path)).secretBytes().toHex(), "extend $path")
            assertEquals(expected, master.derive(DerivationPath(path)).secretBytes().toHex(), "derive $path")
        }
    }

    // BIP-84 account 0, walked from the account key the way a wallet derives its addresses.
    // https://github.com/bitcoin/bips/blob/master/bip-0084.mediawiki#test-vectors
    @Test
    fun secretBytesMatchBip84TestVectors() {
        val root = DescriptorSecretKey(NetworkKind.MAIN, abandonMnemonic, null)
        val account = root.extend(DerivationPath("m/84h/0h/0h"))

        val vectors = mapOf(
            "m/0/0" to "4604b4b710fe91f584fff084e1a9159fe4f8408fff380596a604948474ce4fa3",
            "m/0/1" to "2fd0affa51529f940a358ec0c50de81267d0bf5158ca61887347676946362c5b",
            "m/1/0" to "3277578a56b721e4c9f071f1e24aa0f94c4ff72e7967fea03b134f605f07c8fd",
        )
        for ((path, expected) in vectors) {
            assertEquals(expected, account.extend(DerivationPath(path)).secretBytes().toHex(), path)
        }
    }

    // BIP-86 account 0: https://github.com/bitcoin/bips/blob/master/bip-0086.mediawiki#test-vectors
    @Test
    fun secretBytesMatchBip86TestVectors() {
        val root = DescriptorSecretKey(NetworkKind.MAIN, abandonMnemonic, null)
        assertEquals(
            expected = "xprv9s21ZrQH143K3GJpoapnV8SFfukcVBSfeCficPSGfubmSFDxo1kuHnLisriDvSnRRuL2Qrg5ggqHKNVpxR86QEC8w35uxmGoggxtQTPvfUu",
            actual = root.toString()
        )
        assertEquals(
            expected = "1837c1be8e2995ec11cda2b066151be2cfb48adf9e47b151d46adab3a21cdf67",
            actual = root.secretBytes().toHex()
        )
        assertEquals(
            expected = "41f41d69260df4cf277826a9b65a3717e4eeddbeedf637f212ca096576479361",
            actual = root.extend(DerivationPath("m/86h/0h/0h/0/0")).secretBytes().toHex()
        )
    }
}
