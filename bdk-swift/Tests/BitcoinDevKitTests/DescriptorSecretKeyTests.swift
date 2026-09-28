import XCTest
@testable import BitcoinDevKit

// The expected bytes are the private keys embedded in the xprv/WIF strings published in the BIP
// test vectors, so secretBytes() is checked against values that do not come from this library.
final class DescriptorSecretKeyTests: XCTestCase {
    private func hex(_ bytes: Data) -> String {
        bytes.map { String(format: "%02x", $0) }.joined()
    }

    private func abandonMnemonic() throws -> Mnemonic {
        try Mnemonic.fromString(mnemonic: "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about")
    }

    // BIP-32 test vector 1, seed 000102030405060708090a0b0c0d0e0f.
    // https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki#test-vector-1
    func testSecretBytesMatchBip32TestVector1() throws {
        let master = try DescriptorSecretKey.fromString(
            privateKey: "xprv9s21ZrQH143K3QTDL4LXw2F7HEK3wJUD2nW2nRk4stbPy6cq3jPPqjiChkVvvNKmPGJxWUtg6LnF5kejMRNNU3TGtRBeJgk33yuGBxrMPHi"
        )
        XCTAssertEqual(hex(master.secretBytes()), "e8f32e723decf4051aefac8e2c93c9c5b214313817cdb01a1494b917c8436b35")

        let vectors = [
            ("m/0h/1/2h", "cbce0d719ecf7431d88e6a89fa1483e02e35092af60c042b1df2ff59fa424dca"),
            ("m/0h/1/2h/2/1000000000", "471b76e389e528d6de6d816857e012c5455051cad6660850e58372a6c3e6e7c8"),
        ]
        for (path, expected) in vectors {
            XCTAssertEqual(hex(try master.extend(path: DerivationPath(path: path)).secretBytes()), expected, "extend \(path)")
            XCTAssertEqual(hex(try master.derive(path: DerivationPath(path: path)).secretBytes()), expected, "derive \(path)")
        }
    }

    // BIP-84 account 0, walked from the account key the way a wallet derives its addresses.
    // https://github.com/bitcoin/bips/blob/master/bip-0084.mediawiki#test-vectors
    func testSecretBytesMatchBip84TestVectors() throws {
        let root = DescriptorSecretKey(networkKind: NetworkKind.main, mnemonic: try abandonMnemonic(), password: nil)
        let account = try root.extend(path: DerivationPath(path: "m/84h/0h/0h"))

        let vectors = [
            ("m/0/0", "4604b4b710fe91f584fff084e1a9159fe4f8408fff380596a604948474ce4fa3"),
            ("m/0/1", "2fd0affa51529f940a358ec0c50de81267d0bf5158ca61887347676946362c5b"),
            ("m/1/0", "3277578a56b721e4c9f071f1e24aa0f94c4ff72e7967fea03b134f605f07c8fd"),
        ]
        for (path, expected) in vectors {
            XCTAssertEqual(hex(try account.extend(path: DerivationPath(path: path)).secretBytes()), expected, path)
        }
    }

    // BIP-86 account 0: https://github.com/bitcoin/bips/blob/master/bip-0086.mediawiki#test-vectors
    func testSecretBytesMatchBip86TestVectors() throws {
        let root = DescriptorSecretKey(networkKind: NetworkKind.main, mnemonic: try abandonMnemonic(), password: nil)
        XCTAssertEqual(
            root.description,
            "xprv9s21ZrQH143K3GJpoapnV8SFfukcVBSfeCficPSGfubmSFDxo1kuHnLisriDvSnRRuL2Qrg5ggqHKNVpxR86QEC8w35uxmGoggxtQTPvfUu"
        )
        XCTAssertEqual(hex(root.secretBytes()), "1837c1be8e2995ec11cda2b066151be2cfb48adf9e47b151d46adab3a21cdf67")
        XCTAssertEqual(
            hex(try root.extend(path: DerivationPath(path: "m/86h/0h/0h/0/0")).secretBytes()),
            "41f41d69260df4cf277826a9b65a3717e4eeddbeedf637f212ca096576479361"
        )
    }
}
