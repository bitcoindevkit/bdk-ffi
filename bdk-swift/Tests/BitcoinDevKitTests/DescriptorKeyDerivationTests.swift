import XCTest
@testable import BitcoinDevKit

// derive() must apply any derivation path already recorded on the key (via extend() or
// fromString() with a path suffix) before the requested path.
final class DescriptorKeyDerivationTests: XCTestCase {
    private func master() throws -> DescriptorSecretKey {
        let mnemonic = try Mnemonic.fromString(mnemonic: "all all all all all all all all all all all all")
        return DescriptorSecretKey(networkKind: NetworkKind.test, mnemonic: mnemonic, password: nil)
    }

    func testDeriveAppliesExtendedPath() throws {
        let master = try master()
        let extendedThenDerived = try master
            .extend(path: DerivationPath(path: "1h"))
            .derive(path: DerivationPath(path: "2h"))

        XCTAssertEqual(
            extendedThenDerived.description,
            "[5c9e228d/1'/2']tprv8dka13xAJwRddg25GA5bLEd51TfF9HyJrxA5aM2tpPVGMeYXjbh3wcYr6eFiT4h9d2ZU9a3VPFjhyFbH97pEXcBMy3LS9uUJEt1c3c9zoKs"
        )
        XCTAssertEqual(
            extendedThenDerived.secretBytes(),
            try master.derive(path: DerivationPath(path: "m/1h/2h")).secretBytes()
        )
    }

    func testDeriveAppliesExtendedPathOnPublicKeys() throws {
        let extendedThenDerived = try master().asPublic()
            .extend(path: DerivationPath(path: "1"))
            .derive(path: DerivationPath(path: "2"))

        XCTAssertEqual(
            extendedThenDerived.description,
            "[5c9e228d/1/2]tpubDB9bbqvQAiLBvG44nzUN3Ec1Z4gvm9wiaGWZrRVkWCYkbvEPk9koSFqhJXDFsXbtYfoVXEcV1ZMCX2j6FwuzLNn5WgFFueuxxG3C1KEXP9x"
        )
    }

    func testDeriveAppliesAllChainedExtends() throws {
        let master = try master()
        let manyExtends = try master
            .extend(path: DerivationPath(path: "44h"))
            .extend(path: DerivationPath(path: "1h"))
            .extend(path: DerivationPath(path: "0h"))

        XCTAssertEqual(
            try manyExtends.derive(path: DerivationPath(path: "0")).description,
            try master.derive(path: DerivationPath(path: "m/44h/1h/0h/0")).description
        )
    }

    func testDeriveAppliesParsedPath() throws {
        let parsed = try DescriptorSecretKey.fromString(privateKey: "\(try master())/84h/1h/0h")

        XCTAssertEqual(
            try parsed.derive(path: DerivationPath(path: "0")).description,
            "[5c9e228d/84'/1'/0'/0]tprv8i9wbChZvMCDBn8LJ845pv2SiwjfbCGbFmVrNxbhzeqBFkJYSdLjagc4k4wXRjCxQQdD2x8EHL3FCnHABrngEodtd42WiguKT7bxBh8fsoo"
        )
    }

    // Official test vectors from BIP-32, BIP-84 and BIP-86. Each test reaches the BIP key through
    // extend() and/or fromString() with a path suffix, then derive(), so any pending derivation
    // path that is dropped or misapplied shows up as a mismatch against the published values.

    // BIP-39 mnemonic used by the BIP-84 and BIP-86 test vectors.
    private func abandonMaster() throws -> DescriptorSecretKey {
        let mnemonic = try Mnemonic.fromString(mnemonic: "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about")
        return DescriptorSecretKey(networkKind: NetworkKind.main, mnemonic: mnemonic, password: nil)
    }

    // BIP-32 test vector 1, seed 000102030405060708090a0b0c0d0e0f.
    private let bip32V1MasterXprv = "xprv9s21ZrQH143K3QTDL4LXw2F7HEK3wJUD2nW2nRk4stbPy6cq3jPPqjiChkVvvNKmPGJxWUtg6LnF5kejMRNNU3TGtRBeJgk33yuGBxrMPHi"

    private func address(_ descriptor: String) throws -> String {
        try Descriptor(descriptor: descriptor, networkKind: NetworkKind.main)
            .deriveAddress(index: 0, network: Network.bitcoin)
            .description
    }

    func testBip32Vector1ExtendThenDerive() throws {
        let master = try DescriptorSecretKey.fromString(privateKey: bip32V1MasterXprv)

        // Chain m/0H/1/2H
        let key = try master
            .extend(path: DerivationPath(path: "0h"))
            .extend(path: DerivationPath(path: "1"))
            .derive(path: DerivationPath(path: "2h"))
        XCTAssertEqual(
            key.description,
            "[3442193e/0'/1/2']xprv9z4pot5VBttmtdRTWfWQmoH1taj2axGVzFqSb8C9xaxKymcFzXBDptWmT7FwuEzG3ryjH4ktypQSAewRiNMjANTtpgP4mLTj34bhnZX7UiM"
        )

        // Chain m/0H/1/2H/2/1000000000
        let deepKey = try master
            .extend(path: DerivationPath(path: "0h/1/2h"))
            .derive(path: DerivationPath(path: "2/1000000000"))
        XCTAssertEqual(
            deepKey.description,
            "[3442193e/0'/1/2'/2/1000000000]xprvA41z7zogVVwxVSgdKUHDy1SKmdb533PjDz7J6N6mV6uS3ze1ai8FHa8kmHScGpWmj4WggLyQjgPie1rFSruoUihUZREPSL39UNdE3BBDu76"
        )
        XCTAssertEqual(
            deepKey.asPublic().description,
            "[3442193e/0'/1/2'/2/1000000000]xpub6H1LXWLaKsWFhvm6RVpEL9P4KfRZSW7abD2ttkWP3SSQvnyA8FSVqNTEcYFgJS2UaFcxupHiYkro49S8yGasTvXEYBVPamhGW6cFJodrTHy"
        )
    }

    func testBip32Vector1PublicParsedPathThenDerive() throws {
        // Chain m/0H/1/2H xpub with its origin, and a pending "/2" parsed from the string.
        let key = try DescriptorPublicKey.fromString(
            publicKey: "[3442193e/0'/1/2']xpub6D4BDPcP2GT577Vvch3R8wDkScZWzQzMMUm3PWbmWvVJrZwQY4VUNgqFJPMM3No2dFDFGTsxxpG5uJh7n7epu4trkrX7x7DogT5Uv6fcLW5/2"
        )

        // Chain m/0H/1/2H/2/1000000000
        XCTAssertEqual(
            try key.derive(path: DerivationPath(path: "1000000000")).description,
            "[3442193e/0'/1/2'/2/1000000000]xpub6H1LXWLaKsWFhvm6RVpEL9P4KfRZSW7abD2ttkWP3SSQvnyA8FSVqNTEcYFgJS2UaFcxupHiYkro49S8yGasTvXEYBVPamhGW6cFJodrTHy"
        )
    }

    func testBip86Vectors() throws {
        let master = try abandonMaster()
        XCTAssertEqual(
            master.description,
            "xprv9s21ZrQH143K3GJpoapnV8SFfukcVBSfeCficPSGfubmSFDxo1kuHnLisriDvSnRRuL2Qrg5ggqHKNVpxR86QEC8w35uxmGoggxtQTPvfUu"
        )

        // First receiving address, m/86'/0'/0'/0/0, via extend() then derive()
        let firstReceive = try master
            .extend(path: DerivationPath(path: "86h/0h/0h"))
            .derive(path: DerivationPath(path: "0/0"))
        XCTAssertEqual(
            firstReceive.description,
            "[73c5da0a/86'/0'/0'/0/0]xprvA449goEeU9okwCzzZaxiy475EQGQzBkc65su82nXEvcwzfSskb2hAt2WymrjyRL6kpbVTGL3cKtp9herYXSjjQ1j4stsXXiRF7kXkCacK3T"
        )
        XCTAssertEqual(
            try address("tr(\(firstReceive.asPublic()))"),
            "bc1p5cyxnuxmeuwuvkwfem96lqzszd02n6xdcjrs20cac6yqjjwudpxqkedrcr"
        )

        // Second receiving address, m/86'/0'/0'/0/1, via fromString() with a path suffix
        let account = try DescriptorSecretKey.fromString(privateKey: "\(master)/86h/0h/0h")
        let secondReceive = try account.derive(path: DerivationPath(path: "0/1"))
        XCTAssertEqual(
            secondReceive.description,
            "[73c5da0a/86'/0'/0'/0/1]xprvA449goEeU9okyiF1LmKiDaTgeXvmh87DVyRd35VPbsSop8n8uALpbtrUhUXByPFKK7C2yuqrB1FrhiDkEMC4RGmA5KTwsE1aB5jRu9zHsuQ"
        )
        XCTAssertEqual(
            try address("tr(\(secondReceive.asPublic()))"),
            "bc1p4qhjn9zdvkux4e44uhx8tc55attvtyu358kutcqkudyccelu0was9fqzwh"
        )

        // First change address, m/86'/0'/0'/1/0, from the published account xpub
        let accountXpub = try DescriptorPublicKey.fromString(
            publicKey: "[73c5da0a/86'/0'/0']xpub6BgBgsespWvERF3LHQu6CnqdvfEvtMcQjYrcRzx53QJjSxarj2afYWcLteoGVky7D3UKDP9QyrLprQ3VCECoY49yfdDEHGCtMMj92pReUsQ"
        )
        let firstChange = try accountXpub
            .extend(path: DerivationPath(path: "1"))
            .derive(path: DerivationPath(path: "0"))
        XCTAssertEqual(
            firstChange.description,
            "[73c5da0a/86'/0'/0'/1/0]xpub6GL8SnQwRCGDhB59LEz9HMyM6sRYoByXBzXK3iEKWgCz8XrZNHUzd9L3AUBELW5NzA7dEFvMas1F84TuPH3xqdUA5tumaGWFgihJzWytXe3"
        )
        XCTAssertEqual(
            try address("tr(\(firstChange))"),
            "bc1p3qkhfews2uk44qtvauqyr2ttdsw7svhkl9nkm9s9c3x4ax5h60wqwruhk7"
        )
    }

    func testBip84Vectors() throws {
        // BIP-84 publishes zprv/zpub keys, which BDK does not produce, so these checks compare
        // the published addresses instead.
        let master = try abandonMaster()

        // First receiving address, m/84'/0'/0'/0/0, via extend() then derive()
        let firstReceive = try master
            .extend(path: DerivationPath(path: "84h/0h/0h"))
            .derive(path: DerivationPath(path: "0/0"))
        XCTAssertEqual(
            try address("wpkh(\(firstReceive.asPublic()))"),
            "bc1qcr8te4kr609gcawutmrza0j4xv80jy8z306fyu"
        )

        // Second receiving address, m/84'/0'/0'/0/1, via fromString() with a path suffix
        let account = try DescriptorSecretKey.fromString(privateKey: "\(master)/84h/0h/0h/0")
        XCTAssertEqual(
            try address("wpkh(\(try account.derive(path: DerivationPath(path: "1")).asPublic()))"),
            "bc1qnjg0jd8228aq7egyzacy8cys3knf9xvrerkf9g"
        )

        // First change address, m/84'/0'/0'/1/0, from the account xpub
        let accountXpub = try master.derive(path: DerivationPath(path: "m/84h/0h/0h")).asPublic()
        let firstChange = try accountXpub
            .extend(path: DerivationPath(path: "1"))
            .derive(path: DerivationPath(path: "0"))
        XCTAssertEqual(
            try address("wpkh(\(firstChange))"),
            "bc1q8c6fshw2dlwun7ekn9qwf37cu2rn755upcp6el"
        )
    }
}
