import XCTest
@testable import BitcoinDevKit

final class BitcoinDevKitTests: XCTestCase {
    func testMemoryWalletNewAddress() throws {
        let desc = try Descriptor(
            descriptor: "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/*)",
            network: Network.regtest
        )
        let wallet = try Wallet.newNoPersist(descriptor: desc, changeDescriptor: nil, network: .testnet)
        let addressInfo = wallet.getAddress(addressIndex: AddressIndex.lastUnused)
        XCTAssertEqual(addressInfo.address.asString(), "tb1qzg4mckdh50nwdm9hkzq06528rsu73hjxxzem3e")
    }

//     func testConnectedWalletBalance() throws {
//         let descriptor = try Descriptor(
//             descriptor: "wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/0/*)",
//             network: Network.testnet
//         )
//         let wallet = try Wallet.newNoPersist(
//             descriptor: descriptor,
//             changeDescriptor: nil,
//             network: .testnet
//         )
//
//         let esploraClient = EsploraClient(url: "https://mempool.space/testnet/api")
//         // val esploraClient = EsploraClient("https://blockstream.info/testnet/api")
//         let update = try esploraClient.scan(wallet: wallet, stopGap: 10, parallelRequests: 1)
//         try wallet.applyUpdate(update: update)
//
//         print("Balance: \(wallet.getBalance().total())")
//     }
}
