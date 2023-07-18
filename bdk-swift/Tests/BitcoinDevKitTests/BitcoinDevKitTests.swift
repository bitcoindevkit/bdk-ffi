import XCTest
@testable import BitcoinDevKit

final class BitcoinDevKitTests: XCTestCase {
    
    private let descriptor = "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/*)"
    private let databaseConfig = DatabaseConfig.memory
    private let blockchainConfig = BlockchainConfig.electrum(
        config: ElectrumConfig(
            url: "ssl://electrum.blockstream.info:60002",
            socks5: nil,
            retry: 5,
            timeout: nil,
            stopGap: 100,
            validateDomain: true
        )
    )
    
    func testMemoryWalletNewAddress() throws {
        let descriptor = try Descriptor(
            descriptor: descriptor,
            network: .testnet
        )
        let wallet = try Wallet(
            descriptor: descriptor,
            changeDescriptor: nil,
            network: .testnet,
            databaseConfig: databaseConfig
        )
        let addressInfo = try wallet.getAddress(addressIndex: AddressIndex.new)
        
        XCTAssertEqual(addressInfo.address.asString(), "tb1qzg4mckdh50nwdm9hkzq06528rsu73hjxxzem3e")
    }
    
    func testMemoryWalletBalance() throws {
        let descriptor = try Descriptor(
            descriptor: descriptor,
            network: .testnet
        )
        let wallet = try Wallet(
            descriptor: descriptor,
            changeDescriptor: nil,
            network: .testnet,
            databaseConfig: databaseConfig
        )
        let blockchain = try Blockchain(config: blockchainConfig)
        try wallet.sync(
            blockchain: blockchain,
            progress: nil
        )
        let balance = try wallet.getBalance().total
        
        XCTAssertTrue(balance > 0)
    }
    
    func testBlockInfo() throws {}
    
    func testOpreturn() throws {}
    
    func testListUnspentUTXO() throws {}
    
    func testDescriptorFromMnemonic() throws {
        let mnemonic: Mnemonic = try Mnemonic.fromString(mnemonic: "fire alter tide over object advance diamond pond region select tone pole")
        let bip32RootKey: DescriptorSecretKey = DescriptorSecretKey(
            network: .testnet,
            mnemonic: mnemonic,
            password: ""
        )
        let bip84ExternalPath: DerivationPath = try DerivationPath(path: "m/84h/1h/0h/0")
        let externalExtendedKey: String = try bip32RootKey.extend(path: bip84ExternalPath).asString()
        let descriptor = "wpkh(\(externalExtendedKey))"
        XCTAssertEqual(descriptor, "wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84'/1'/0'/0/*)")
    }
    
    func testPublicDescriptorTemplate() throws {
        let descriptorPublicKey: DescriptorPublicKey = try DescriptorPublicKey.fromString(publicKey: "tpubDCYVtmaSaDzTxcgvoP5AHZNbZKZzrvoNH9KARep88vESc6MxRqAp4LmePc2eeGX6XUxBcdhAmkthWTDqygPz2wLAyHWisD299Lkdrj5egY6")
        let descriptor = Descriptor.newBip84Public(
            publicKey: descriptorPublicKey,
            fingerprint: "9122d9e0",
            keychain: .external,
            network: .testnet
        )
        XCTAssertEqual(descriptor.asString(), "wpkh([9122d9e0/84'/1'/0']tpubDCYVtmaSaDzTxcgvoP5AHZNbZKZzrvoNH9KARep88vESc6MxRqAp4LmePc2eeGX6XUxBcdhAmkthWTDqygPz2wLAyHWisD299Lkdrj5egY6/0/*)#zpaanzgu")
    }
    
    func testTransactionDetails() throws {}
    
    func testPrivateDescriptorTemplate() throws {
        let descriptorSecretKey: DescriptorSecretKey = try DescriptorSecretKey.fromString(
            secretKey: "tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B"
        )
        let descriptor = Descriptor.newBip84(
            secretKey: descriptorSecretKey,
            keychain: .external,
            network: .testnet
        )
        
        XCTAssertEqual(descriptor.asStringPrivate(), "wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84'/1'/0'/0/*)#yl0cyza0")
        XCTAssertEqual(descriptor.asString(), "wpkh([9122d9e0/84'/1'/0']tpubDCYVtmaSaDzTxcgvoP5AHZNbZKZzrvoNH9KARep88vESc6MxRqAp4LmePc2eeGX6XUxBcdhAmkthWTDqygPz2wLAyHWisD299Lkdrj5egY6/0/*)#zpaanzgu")
    }
    
    func testCreateTxFromRawBytes() throws {
        let rawTx = "020000000001031cfbc8f54fbfa4a33a30068841371f80dbfe166211242213188428f437445c91000000006a47304402206fbcec8d2d2e740d824d3d36cc345b37d9f65d665a99f5bd5c9e8d42270a03a8022013959632492332200c2908459547bf8dbf97c65ab1a28dec377d6f1d41d3d63e012103d7279dfb90ce17fe139ba60a7c41ddf605b25e1c07a4ddcb9dfef4e7d6710f48feffffff476222484f5e35b3f0e43f65fc76e21d8be7818dd6a989c160b1e5039b7835fc00000000171600140914414d3c94af70ac7e25407b0689e0baa10c77feffffffa83d954a62568bbc99cc644c62eb7383d7c2a2563041a0aeb891a6a4055895570000000017160014795d04cc2d4f31480d9a3710993fbd80d04301dffeffffff06fef72f000000000017a91476fd7035cd26f1a32a5ab979e056713aac25796887a5000f00000000001976a914b8332d502a529571c6af4be66399cd33379071c588ac3fda0500000000001976a914fc1d692f8de10ae33295f090bea5fe49527d975c88ac522e1b00000000001976a914808406b54d1044c429ac54c0e189b0d8061667e088ac6eb68501000000001976a914dfab6085f3a8fb3e6710206a5a959313c5618f4d88acbba20000000000001976a914eb3026552d7e3f3073457d0bee5d4757de48160d88ac0002483045022100bee24b63212939d33d513e767bc79300051f7a0d433c3fcf1e0e3bf03b9eb1d70220588dc45a9ce3a939103b4459ce47500b64e23ab118dfc03c9caa7d6bfc32b9c601210354fd80328da0f9ae6eef2b3a81f74f9a6f66761fadf96f1d1d22b1fd6845876402483045022100e29c7e3a5efc10da6269e5fc20b6a1cb8beb92130cc52c67e46ef40aaa5cac5f0220644dd1b049727d991aece98a105563416e10a5ac4221abac7d16931842d5c322012103960b87412d6e169f30e12106bdf70122aabb9eb61f455518322a18b920a4dfa887d30700"
        let hexArrayTx = try rawTx.asHexArray()
        let txFromBytes: Transaction = try Transaction(transactionBytes: hexArrayTx)
        
        let bytesArray: [UInt8] = [2, 0, 0, 0, 0, 1, 3, 28, 251, 200, 245, 79, 191, 164, 163, 58, 48, 6, 136, 65, 55, 31, 128, 219, 254, 22, 98, 17, 36, 34, 19, 24, 132, 40, 244, 55, 68, 92, 145, 0, 0, 0, 0, 106, 71, 48, 68, 2, 32, 111, 188, 236, 141, 45, 46, 116, 13, 130, 77, 61, 54, 204, 52, 91, 55, 217, 246, 93, 102, 90, 153, 245, 189, 92, 158, 141, 66, 39, 10, 3, 168, 2, 32, 19, 149, 150, 50, 73, 35, 50, 32, 12, 41, 8, 69, 149, 71, 191, 141, 191, 151, 198, 90, 177, 162, 141, 236, 55, 125, 111, 29, 65, 211, 214, 62, 1, 33, 3, 215, 39, 157, 251, 144, 206, 23, 254, 19, 155, 166, 10, 124, 65, 221, 246, 5, 178, 94, 28, 7, 164, 221, 203, 157, 254, 244, 231, 214, 113, 15, 72, 254, 255, 255, 255, 71, 98, 34, 72, 79, 94, 53, 179, 240, 228, 63, 101, 252, 118, 226, 29, 139, 231, 129, 141, 214, 169, 137, 193, 96, 177, 229, 3, 155, 120, 53, 252, 0, 0, 0, 0, 23, 22, 0, 20, 9, 20, 65, 77, 60, 148, 175, 112, 172, 126, 37, 64, 123, 6, 137, 224, 186, 161, 12, 119, 254, 255, 255, 255, 168, 61, 149, 74, 98, 86, 139, 188, 153, 204, 100, 76, 98, 235, 115, 131, 215, 194, 162, 86, 48, 65, 160, 174, 184, 145, 166, 164, 5, 88, 149, 87, 0, 0, 0, 0, 23, 22, 0, 20, 121, 93, 4, 204, 45, 79, 49, 72, 13, 154, 55, 16, 153, 63, 189, 128, 208, 67, 1, 223, 254, 255, 255, 255, 6, 254, 247, 47, 0, 0, 0, 0, 0, 23, 169, 20, 118, 253, 112, 53, 205, 38, 241, 163, 42, 90, 185, 121, 224, 86, 113, 58, 172, 37, 121, 104, 135, 165, 0, 15, 0, 0, 0, 0, 0, 25, 118, 169, 20, 184, 51, 45, 80, 42, 82, 149, 113, 198, 175, 75, 230, 99, 153, 205, 51, 55, 144, 113, 197, 136, 172, 63, 218, 5, 0, 0, 0, 0, 0, 25, 118, 169, 20, 252, 29, 105, 47, 141, 225, 10, 227, 50, 149, 240, 144, 190, 165, 254, 73, 82, 125, 151, 92, 136, 172, 82, 46, 27, 0, 0, 0, 0, 0, 25, 118, 169, 20, 128, 132, 6, 181, 77, 16, 68, 196, 41, 172, 84, 192, 225, 137, 176, 216, 6, 22, 103, 224, 136, 172, 110, 182, 133, 1, 0, 0, 0, 0, 25, 118, 169, 20, 223, 171, 96, 133, 243, 168, 251, 62, 103, 16, 32, 106, 90, 149, 147, 19, 197, 97, 143, 77, 136, 172, 187, 162, 0, 0, 0, 0, 0, 0, 25, 118, 169, 20, 235, 48, 38, 85, 45, 126, 63, 48, 115, 69, 125, 11, 238, 93, 71, 87, 222, 72, 22, 13, 136, 172, 0, 2, 72, 48, 69, 2, 33, 0, 190, 226, 75, 99, 33, 41, 57, 211, 61, 81, 62, 118, 123, 199, 147, 0, 5, 31, 122, 13, 67, 60, 63, 207, 30, 14, 59, 240, 59, 158, 177, 215, 2, 32, 88, 141, 196, 90, 156, 227, 169, 57, 16, 59, 68, 89, 206, 71, 80, 11, 100, 226, 58, 177, 24, 223, 192, 60, 156, 170, 125, 107, 252, 50, 185, 198, 1, 33, 3, 84, 253, 128, 50, 141, 160, 249, 174, 110, 239, 43, 58, 129, 247, 79, 154, 111, 102, 118, 31, 173, 249, 111, 29, 29, 34, 177, 253, 104, 69, 135, 100, 2, 72, 48, 69, 2, 33, 0, 226, 156, 126, 58, 94, 252, 16, 218, 98, 105, 229, 252, 32, 182, 161, 203, 139, 235, 146, 19, 12, 197, 44, 103, 228, 110, 244, 10, 170, 92, 172, 95, 2, 32, 100, 77, 209, 176, 73, 114, 125, 153, 26, 236, 233, 138, 16, 85, 99, 65, 110, 16, 165, 172, 66, 33, 171, 172, 125, 22, 147, 24, 66, 213, 195, 34, 1, 33, 3, 150, 11, 135, 65, 45, 110, 22, 159, 48, 225, 33, 6, 189, 247, 1, 34, 170, 187, 158, 182, 31, 69, 85, 24, 50, 42, 24, 185, 32, 164, 223, 168, 135, 211, 7, 0]
        
        XCTAssertEqual(txFromBytes.serialize(), bytesArray)
    }
    
    func testDeriveCustomPath() throws {
        let bip32RootKey: DescriptorSecretKey = try DescriptorSecretKey.fromString(
            secretKey: "tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B"
        )
        
        let veryCustomDerivationPath: DerivationPath = try DerivationPath(path: "m/800h/2020h/1234h/77/42")
        let xprv: DescriptorSecretKey = try bip32RootKey.derive(path: veryCustomDerivationPath)
        let extendCustomPath: DerivationPath = try DerivationPath(path: "m/2/2/42")
        let keysAtVeryCustomLocation: DescriptorSecretKey = try xprv.extend(path: extendCustomPath)

        XCTAssertEqual(xprv.asString(), "[9122d9e0/800'/2020'/1234'/77/42]tprv8khyti4SM3F8YDWRwoaqUL6u21ZiUGFQYEYWPZVSn8o8ugeBepJHLoqLvkWw1qcEMDXBeRtHJxeDtQ67ttTEm4xpL3JCRdCTVsg531DNiu1")
        XCTAssertEqual(keysAtVeryCustomLocation.asString(), "[9122d9e0/800'/2020'/1234'/77/42]tprv8khyti4SM3F8YDWRwoaqUL6u21ZiUGFQYEYWPZVSn8o8ugeBepJHLoqLvkWw1qcEMDXBeRtHJxeDtQ67ttTEm4xpL3JCRdCTVsg531DNiu1/2/2/42")
    }
    
    func testBasicTransaction() throws {}
    
}


// Helpers

enum HexConvertError: Error {
    case wrongInputStringLength
    case wrongInputStringCharacters
}

extension StringProtocol {
    func asHexArrayFromNonValidatedSource() -> [UInt8] {
        var startIndex = self.startIndex
        return stride(from: 0, to: count, by: 2).compactMap { _ in
            let endIndex = index(startIndex, offsetBy: 2, limitedBy: self.endIndex) ?? self.endIndex
            defer { startIndex = endIndex }
            return UInt8(self[startIndex..<endIndex], radix: 16)
        }
    }

    func asHexArray() throws -> [UInt8] {
        if count % 2 != 0 { throw HexConvertError.wrongInputStringLength }
        let characterSet = "0123456789ABCDEFabcdef"
        let wrongCharacter = first { return !characterSet.contains($0) }
        if wrongCharacter != nil { throw HexConvertError.wrongInputStringCharacters }
        return asHexArrayFromNonValidatedSource()
    }
}
