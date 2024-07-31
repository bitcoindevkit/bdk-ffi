import XCTest
@testable import BitcoinDevKit

final class OfflineWalletTests: XCTestCase {
    private let descriptor = try! Descriptor(
    descriptor: "wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/0/*)", 
    network: Network.signet
    )
    private let changeDescriptor = try! Descriptor(
        descriptor: "wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/1/*)", 
        network: Network.signet
    )
    
    func testNewAddress() throws {
        let connection = try Connection.newInMemory()
        let wallet = try Wallet(
            descriptor: descriptor,
            changeDescriptor: changeDescriptor,
            network: .signet,
            connection: connection
        )
        let addressInfo: AddressInfo = wallet.revealNextAddress(keychain: KeychainKind.external)

        XCTAssertTrue(addressInfo.address.isValidForNetwork(network: Network.testnet),
                     "Address is not valid for testnet network")
        XCTAssertTrue(addressInfo.address.isValidForNetwork(network: Network.signet),
                     "Address is not valid for signet network")
        XCTAssertFalse(addressInfo.address.isValidForNetwork(network: Network.regtest),
                      "Address is valid for regtest network, but it shouldn't be")
        XCTAssertFalse(addressInfo.address.isValidForNetwork(network: Network.bitcoin),
                      "Address is valid for bitcoin network, but it shouldn't be")

        XCTAssertEqual(addressInfo.address.description, "tb1qrnfslnrve9uncz9pzpvf83k3ukz22ljgees989")
    }
    
    func testBalance() throws {
        let connection = try Connection.newInMemory()
        let wallet = try Wallet(
            descriptor: descriptor,
            changeDescriptor: changeDescriptor,
            network: .signet,
            connection: connection
        )

        XCTAssertEqual(wallet.balance().total.toSat(), 0)
    }
}
