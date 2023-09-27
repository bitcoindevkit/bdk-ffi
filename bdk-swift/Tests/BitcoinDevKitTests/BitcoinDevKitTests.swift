import XCTest
@testable import BitcoinDevKit

final class BitcoinDevKitTests: XCTestCase {
//    func testMemoryWalletNewAddress() throws {
//        let desc = try Descriptor(
//            descriptor: "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/*)",
//            network: Network.regtest
//        )
//        let databaseConfig = DatabaseConfig.memory
//        let wallet = try Wallet.init(descriptor: desc, changeDescriptor: nil, network: Network.regtest, databaseConfig: databaseConfig)
//        let addressInfo = try wallet.getAddress(addressIndex: AddressIndex.new)
//        XCTAssertEqual(addressInfo.address.asString(), "bcrt1qzg4mckdh50nwdm9hkzq06528rsu73hjxytqkxs")
//    }
    func testHelloWorld() {
        let message = BitcoinDevKit.helloWorld()
        XCTAssertEqual(message, "Hello World")
    }
    func testNetwork() {
        let signetNetwork = Network.signet
        XCTAssertEqual(signetNetwork, Network.signet)
    }
    func testDescriptor() {
        let mnemonic = Mnemonic(wordCount: WordCount.words12)
        let descriptorSecretKey = DescriptorSecretKey(network: Network.testnet, mnemonic: mnemonic, password: nil)
        let descriptor = Descriptor.newBip86(secretKey: descriptorSecretKey, keychain: KeychainKind.external, network: Network.testnet)
    }
}
