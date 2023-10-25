import XCTest
@testable import BitcoinDevKit

final class BitcoinDevKitTests: XCTestCase {
    
    func testDescriptorBip86() {
        let mnemonic: Mnemonic = Mnemonic(wordCount: WordCount.words12)
        let descriptorSecretKey: DescriptorSecretKey = DescriptorSecretKey(
            network: Network.testnet,
            mnemonic: mnemonic,
            password: nil
        )
        let descriptor: Descriptor = Descriptor.newBip86(
            secretKey: descriptorSecretKey,
            keychain: KeychainKind.external,
            network: Network.testnet
        )

        XCTAssertTrue(descriptor.asString().hasPrefix("tr"), "Bip86 Descriptor does not start with 'tr'")
    }
    
    func testNewAddress() throws {
        let descriptor: Descriptor = try Descriptor(
            descriptor: "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/*)",
            network: Network.testnet
        )
        let wallet: Wallet = try Wallet.newNoPersist(
            descriptor: descriptor,
            changeDescriptor: nil,
            network: .testnet
        )
        let addressInfo: AddressInfo = wallet.getAddress(addressIndex: AddressIndex.new)
        
        XCTAssertEqual(addressInfo.address.asString(), "tb1qzg4mckdh50nwdm9hkzq06528rsu73hjxxzem3e")
    }
    
    func testBalance() throws {
        let descriptor: Descriptor = try Descriptor(
            descriptor: "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/*)",
            network: Network.testnet
        )
        let wallet: Wallet = try Wallet.newNoPersist(
            descriptor: descriptor,
            changeDescriptor: nil,
            network: .testnet
        )

        XCTAssertEqual(wallet.getBalance().total(), 0)
    }

//    func testSyncedBalance() throws {
//        let descriptor = try Descriptor(
//            descriptor: "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/*)",
//            network: Network.testnet
//        )
//        let wallet = try Wallet.newNoPersist(
//            descriptor: descriptor,
//            changeDescriptor: nil,
//            network: .testnet
//        )
//        let esploraClient = EsploraClient("https://mempool.space/testnet/api")
//        let update = esploraClient.scan(wallet, 10, 1)
//        wallet.applyUpdate(update)
//
//        XCTAssertEqual(wallet.getBalance().total(), 0)
//    }

}
