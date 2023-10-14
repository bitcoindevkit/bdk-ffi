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
    func testWallet() {
        do {
            let descriptor1 = try Descriptor(
                descriptor: "wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/0/*)",
                network: Network.testnet
            )
            let wallet = try Wallet(descriptor: descriptor1, changeDescriptor: nil, network: .testnet, walletType: .memory)
            let addressInfo = wallet.getAddress(addressIndex: .lastUnused)
            print("Address \(addressInfo.address.asString()) at index \(addressInfo.index)")
        } catch {
            print("testWallet error")
        }
    }
        func testBalance() {
            do {
                let descriptor = try Descriptor(
                    descriptor: "wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/0/*)",
                    network: .testnet
                )
                let wallet = try Wallet(
                    descriptor: descriptor,
                    changeDescriptor: nil,
                    network: .testnet,
                    walletType: .memory
                )
                let total = wallet.getBalance().total()
                self.balance = String(total)
            } catch {
                print("testBalance error")
            }
        }
}
