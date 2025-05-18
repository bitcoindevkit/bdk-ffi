import XCTest
@testable import BitcoinDevKit

final class OfflineDescriptorTests: XCTestCase {
    func testDescriptorBip86() throws {
        let mnemonic: Mnemonic = try Mnemonic.fromString(mnemonic: "space echo position wrist orient erupt relief museum myself grain wisdom tumble")
        let descriptorSecretKey: DescriptorSecretKey = DescriptorSecretKey(
            network: Network.testnet,
            mnemonic: mnemonic,
            password: nil
        )
        let descriptor: Descriptor = Descriptor.newBip86(
            secretKey: descriptorSecretKey,
            keychainKind: KeychainKind.external,
            network: Network.testnet
        )

        XCTAssertEqual(descriptor.description, "tr([be1eec8f/86'/1'/0']tpubDCTtszwSxPx3tATqDrsSyqScPNnUChwQAVAkanuDUCJQESGBbkt68nXXKRDifYSDbeMa2Xg2euKbXaU3YphvGWftDE7ozRKPriT6vAo3xsc/0/*)#m7puekcx")
    }
}
