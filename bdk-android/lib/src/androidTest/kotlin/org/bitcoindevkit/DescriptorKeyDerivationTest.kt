package org.bitcoindevkit

import androidx.test.ext.junit.runners.AndroidJUnit4
import org.junit.runner.RunWith
import kotlin.test.Test
import kotlin.test.assertEquals

// derive() must apply any derivation path already recorded on the key (via extend() or
// fromString() with a path suffix) before the requested path.
@RunWith(AndroidJUnit4::class)
class DescriptorKeyDerivationTest {
    private val master: DescriptorSecretKey = DescriptorSecretKey(
        NetworkKind.TEST,
        Mnemonic.fromString("all all all all all all all all all all all all"),
        null
    )

    @Test
    fun deriveAppliesExtendedPath() {
        val extendedThenDerived = master.extend(DerivationPath("1h")).derive(DerivationPath("2h"))

        assertEquals(
            expected = "[5c9e228d/1'/2']tprv8dka13xAJwRddg25GA5bLEd51TfF9HyJrxA5aM2tpPVGMeYXjbh3wcYr6eFiT4h9d2ZU9a3VPFjhyFbH97pEXcBMy3LS9uUJEt1c3c9zoKs",
            actual = extendedThenDerived.toString()
        )
        assertEquals(
            expected = master.derive(DerivationPath("m/1h/2h")).secretBytes().toList(),
            actual = extendedThenDerived.secretBytes().toList()
        )
    }

    @Test
    fun deriveAppliesExtendedPathOnPublicKeys() {
        val extendedThenDerived = master.asPublic().extend(DerivationPath("1")).derive(DerivationPath("2"))

        assertEquals(
            expected = "[5c9e228d/1/2]tpubDB9bbqvQAiLBvG44nzUN3Ec1Z4gvm9wiaGWZrRVkWCYkbvEPk9koSFqhJXDFsXbtYfoVXEcV1ZMCX2j6FwuzLNn5WgFFueuxxG3C1KEXP9x",
            actual = extendedThenDerived.toString()
        )
    }

    @Test
    fun deriveAppliesAllChainedExtends() {
        val manyExtends = master
            .extend(DerivationPath("44h"))
            .extend(DerivationPath("1h"))
            .extend(DerivationPath("0h"))

        assertEquals(
            expected = master.derive(DerivationPath("m/44h/1h/0h/0")).toString(),
            actual = manyExtends.derive(DerivationPath("0")).toString()
        )
    }

    @Test
    fun deriveAppliesParsedPath() {
        val parsed = DescriptorSecretKey.fromString("$master/84h/1h/0h")

        assertEquals(
            expected = "[5c9e228d/84'/1'/0'/0]tprv8i9wbChZvMCDBn8LJ845pv2SiwjfbCGbFmVrNxbhzeqBFkJYSdLjagc4k4wXRjCxQQdD2x8EHL3FCnHABrngEodtd42WiguKT7bxBh8fsoo",
            actual = parsed.derive(DerivationPath("0")).toString()
        )
    }

    // Official test vectors from BIP-32, BIP-84 and BIP-86. Each test reaches the BIP key through
    // extend() and/or fromString() with a path suffix, then derive(), so any pending derivation
    // path that is dropped or misapplied shows up as a mismatch against the published values.

    // BIP-39 mnemonic used by the BIP-84 and BIP-86 test vectors.
    private val abandonMaster: DescriptorSecretKey = DescriptorSecretKey(
        NetworkKind.MAIN,
        Mnemonic.fromString("abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"),
        null
    )

    // BIP-32 test vector 1, seed 000102030405060708090a0b0c0d0e0f.
    private val bip32V1MasterXprv =
        "xprv9s21ZrQH143K3QTDL4LXw2F7HEK3wJUD2nW2nRk4stbPy6cq3jPPqjiChkVvvNKmPGJxWUtg6LnF5kejMRNNU3TGtRBeJgk33yuGBxrMPHi"

    private fun address(descriptor: String): String =
        Descriptor(descriptor, NetworkKind.MAIN).deriveAddress(0u, Network.BITCOIN).toString()

    @Test
    fun bip32Vector1ExtendThenDerive() {
        val master = DescriptorSecretKey.fromString(bip32V1MasterXprv)

        // Chain m/0H/1/2H
        val key = master
            .extend(DerivationPath("0h"))
            .extend(DerivationPath("1"))
            .derive(DerivationPath("2h"))
        assertEquals(
            expected = "[3442193e/0'/1/2']xprv9z4pot5VBttmtdRTWfWQmoH1taj2axGVzFqSb8C9xaxKymcFzXBDptWmT7FwuEzG3ryjH4ktypQSAewRiNMjANTtpgP4mLTj34bhnZX7UiM",
            actual = key.toString()
        )

        // Chain m/0H/1/2H/2/1000000000
        val deepKey = master
            .extend(DerivationPath("0h/1/2h"))
            .derive(DerivationPath("2/1000000000"))
        assertEquals(
            expected = "[3442193e/0'/1/2'/2/1000000000]xprvA41z7zogVVwxVSgdKUHDy1SKmdb533PjDz7J6N6mV6uS3ze1ai8FHa8kmHScGpWmj4WggLyQjgPie1rFSruoUihUZREPSL39UNdE3BBDu76",
            actual = deepKey.toString()
        )
        assertEquals(
            expected = "[3442193e/0'/1/2'/2/1000000000]xpub6H1LXWLaKsWFhvm6RVpEL9P4KfRZSW7abD2ttkWP3SSQvnyA8FSVqNTEcYFgJS2UaFcxupHiYkro49S8yGasTvXEYBVPamhGW6cFJodrTHy",
            actual = deepKey.asPublic().toString()
        )
    }

    @Test
    fun bip32Vector1PublicParsedPathThenDerive() {
        // Chain m/0H/1/2H xpub with its origin, and a pending "/2" parsed from the string.
        val key = DescriptorPublicKey.fromString(
            "[3442193e/0'/1/2']xpub6D4BDPcP2GT577Vvch3R8wDkScZWzQzMMUm3PWbmWvVJrZwQY4VUNgqFJPMM3No2dFDFGTsxxpG5uJh7n7epu4trkrX7x7DogT5Uv6fcLW5/2"
        )

        // Chain m/0H/1/2H/2/1000000000
        assertEquals(
            expected = "[3442193e/0'/1/2'/2/1000000000]xpub6H1LXWLaKsWFhvm6RVpEL9P4KfRZSW7abD2ttkWP3SSQvnyA8FSVqNTEcYFgJS2UaFcxupHiYkro49S8yGasTvXEYBVPamhGW6cFJodrTHy",
            actual = key.derive(DerivationPath("1000000000")).toString()
        )
    }

    @Test
    fun bip86Vectors() {
        val master = abandonMaster
        assertEquals(
            expected = "xprv9s21ZrQH143K3GJpoapnV8SFfukcVBSfeCficPSGfubmSFDxo1kuHnLisriDvSnRRuL2Qrg5ggqHKNVpxR86QEC8w35uxmGoggxtQTPvfUu",
            actual = master.toString()
        )

        // First receiving address, m/86'/0'/0'/0/0, via extend() then derive()
        val firstReceive = master.extend(DerivationPath("86h/0h/0h")).derive(DerivationPath("0/0"))
        assertEquals(
            expected = "[73c5da0a/86'/0'/0'/0/0]xprvA449goEeU9okwCzzZaxiy475EQGQzBkc65su82nXEvcwzfSskb2hAt2WymrjyRL6kpbVTGL3cKtp9herYXSjjQ1j4stsXXiRF7kXkCacK3T",
            actual = firstReceive.toString()
        )
        assertEquals(
            expected = "bc1p5cyxnuxmeuwuvkwfem96lqzszd02n6xdcjrs20cac6yqjjwudpxqkedrcr",
            actual = address("tr(${firstReceive.asPublic()})")
        )

        // Second receiving address, m/86'/0'/0'/0/1, via fromString() with a path suffix
        val account = DescriptorSecretKey.fromString("$master/86h/0h/0h")
        val secondReceive = account.derive(DerivationPath("0/1"))
        assertEquals(
            expected = "[73c5da0a/86'/0'/0'/0/1]xprvA449goEeU9okyiF1LmKiDaTgeXvmh87DVyRd35VPbsSop8n8uALpbtrUhUXByPFKK7C2yuqrB1FrhiDkEMC4RGmA5KTwsE1aB5jRu9zHsuQ",
            actual = secondReceive.toString()
        )
        assertEquals(
            expected = "bc1p4qhjn9zdvkux4e44uhx8tc55attvtyu358kutcqkudyccelu0was9fqzwh",
            actual = address("tr(${secondReceive.asPublic()})")
        )

        // First change address, m/86'/0'/0'/1/0, from the published account xpub
        val accountXpub = DescriptorPublicKey.fromString(
            "[73c5da0a/86'/0'/0']xpub6BgBgsespWvERF3LHQu6CnqdvfEvtMcQjYrcRzx53QJjSxarj2afYWcLteoGVky7D3UKDP9QyrLprQ3VCECoY49yfdDEHGCtMMj92pReUsQ"
        )
        val firstChange = accountXpub.extend(DerivationPath("1")).derive(DerivationPath("0"))
        assertEquals(
            expected = "[73c5da0a/86'/0'/0'/1/0]xpub6GL8SnQwRCGDhB59LEz9HMyM6sRYoByXBzXK3iEKWgCz8XrZNHUzd9L3AUBELW5NzA7dEFvMas1F84TuPH3xqdUA5tumaGWFgihJzWytXe3",
            actual = firstChange.toString()
        )
        assertEquals(
            expected = "bc1p3qkhfews2uk44qtvauqyr2ttdsw7svhkl9nkm9s9c3x4ax5h60wqwruhk7",
            actual = address("tr($firstChange)")
        )
    }

    @Test
    fun bip84Vectors() {
        // BIP-84 publishes zprv/zpub keys, which BDK does not produce, so these checks compare
        // the published addresses instead.
        val master = abandonMaster

        // First receiving address, m/84'/0'/0'/0/0, via extend() then derive()
        val firstReceive = master.extend(DerivationPath("84h/0h/0h")).derive(DerivationPath("0/0"))
        assertEquals(
            expected = "bc1qcr8te4kr609gcawutmrza0j4xv80jy8z306fyu",
            actual = address("wpkh(${firstReceive.asPublic()})")
        )

        // Second receiving address, m/84'/0'/0'/0/1, via fromString() with a path suffix
        val account = DescriptorSecretKey.fromString("$master/84h/0h/0h/0")
        assertEquals(
            expected = "bc1qnjg0jd8228aq7egyzacy8cys3knf9xvrerkf9g",
            actual = address("wpkh(${account.derive(DerivationPath("1")).asPublic()})")
        )

        // First change address, m/84'/0'/0'/1/0, from the account xpub
        val accountXpub = master.derive(DerivationPath("m/84h/0h/0h")).asPublic()
        val firstChange = accountXpub.extend(DerivationPath("1")).derive(DerivationPath("0"))
        assertEquals(
            expected = "bc1q8c6fshw2dlwun7ekn9qwf37cu2rn755upcp6el",
            actual = address("wpkh($firstChange)")
        )
    }
}
