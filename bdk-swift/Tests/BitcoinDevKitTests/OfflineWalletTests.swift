import XCTest
@testable import BitcoinDevKit

final class OfflineWalletTests: XCTestCase {
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

        XCTAssertEqual(wallet.getBalance().total, 0)
    }
}
