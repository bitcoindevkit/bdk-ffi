import XCTest
@testable import BitcoinDevKit

final class OfflineDescriptorTests: XCTestCase {
    func testDescriptorBip86() throws {
        let mnemonic: Mnemonic = try Mnemonic.fromString(mnemonic: "space echo position wrist orient erupt relief museum myself grain wisdom tumble")
        let descriptorSecretKey: DescriptorSecretKey = DescriptorSecretKey(
            networkKind: NetworkKind.test,
            mnemonic: mnemonic,
            password: nil
        )
        let descriptor: Descriptor = Descriptor.newBip86(
            secretKey: descriptorSecretKey,
            keychainKind: KeychainKind.external,
            networkKind: NetworkKind.test
        )

        XCTAssertEqual(descriptor.description, "tr([be1eec8f/86'/1'/0']tpubDCTtszwSxPx3tATqDrsSyqScPNnUChwQAVAkanuDUCJQESGBbkt68nXXKRDifYSDbeMa2Xg2euKbXaU3YphvGWftDE7ozRKPriT6vAo3xsc/0/*)#m7puekcx")
    }

    func testDescriptorSecretKeyAddUnhardenedWildcard() throws {
        let mnemonic = try Mnemonic.fromString(mnemonic: "awesome awesome awesome awesome awesome awesome awesome awesome awesome awesome awesome awesome")
        let secretKey = DescriptorSecretKey(networkKind: NetworkKind.test, mnemonic: mnemonic, password: nil)
        let derived = try secretKey.derive(path: DerivationPath(path: "m/86'/1'/0'"))
        let withWildcard = try derived.addWildcard(wildcardType: WildcardType.unhardened)
        XCTAssertTrue(withWildcard.description.hasSuffix("/*"))
    }

    func testDescriptorSecretKeyAddHardenedWildcard() throws {
        let mnemonic = try Mnemonic.fromString(mnemonic: "awesome awesome awesome awesome awesome awesome awesome awesome awesome awesome awesome awesome")
        let secretKey = DescriptorSecretKey(networkKind: NetworkKind.test, mnemonic: mnemonic, password: nil)
        let derived = try secretKey.derive(path: DerivationPath(path: "m/86'/1'/0'"))
        let withWildcard = try derived.addWildcard(wildcardType: WildcardType.hardened)
        XCTAssertTrue(withWildcard.description.hasSuffix("/*h"))
    }

    func testDescriptorSecretKeyAddWildcardIsIdempotent() throws {
        let mnemonic = try Mnemonic.fromString(mnemonic: "awesome awesome awesome awesome awesome awesome awesome awesome awesome awesome awesome awesome")
        let secretKey = DescriptorSecretKey(networkKind: NetworkKind.test, mnemonic: mnemonic, password: nil)
        let derived = try secretKey.derive(path: DerivationPath(path: "m/86'/1'/0'"))
        let withWildcard = try derived.addWildcard(wildcardType: WildcardType.unhardened)
        let withWildcardAgain = try withWildcard.addWildcard(wildcardType: WildcardType.unhardened)
        XCTAssertEqual(withWildcard.description, withWildcardAgain.description)
    }

    func testDescriptorSecretKeyCannotChangeWildcardType() throws {
        let mnemonic = try Mnemonic.fromString(mnemonic: "awesome awesome awesome awesome awesome awesome awesome awesome awesome awesome awesome awesome")
        let secretKey = DescriptorSecretKey(networkKind: NetworkKind.test, mnemonic: mnemonic, password: nil)
        let derived = try secretKey.derive(path: DerivationPath(path: "m/86'/1'/0'"))
        let withWildcard = try derived.addWildcard(wildcardType: WildcardType.unhardened)
        XCTAssertThrowsError(try withWildcard.addWildcard(wildcardType: WildcardType.hardened))
    }

    func testDescriptorPublicKeyAddWildcard() throws {
        let mnemonic = try Mnemonic.fromString(mnemonic: "awesome awesome awesome awesome awesome awesome awesome awesome awesome awesome awesome awesome")
        let secretKey = DescriptorSecretKey(networkKind: NetworkKind.test, mnemonic: mnemonic, password: nil)
        let derived = try secretKey.derive(path: DerivationPath(path: "m/86'/1'/0'"))
        let publicKey = derived.asPublic()
        let withWildcard = try publicKey.addWildcard()
        XCTAssertTrue(withWildcard.description.hasSuffix("/*"))
    }
}
