import Foundation
import BitcoinDevKit

let (descriptor, changeDescriptor) = createDescriptorsFromBip32RootKey(
    scriptType: .p2wpkh,
    network: .regtest
)
print("Descriptor: \(descriptor)")
print("Change descriptor: \(changeDescriptor)")

let persistenceFilePath = getPersistenceFilePath()
print("Persistence file path: \(persistenceFilePath)")

let connection = try! Persister.newSqlite(path: persistenceFilePath)
let wallet = try! Wallet(
    descriptor: descriptor,
    changeDescriptor: changeDescriptor,
    network: .regtest,
    persister: connection
)
let changeAddress = wallet.revealNextAddress(keychain: .internal).address
let address = wallet.revealNextAddress(keychain: .external).address

print("Change address: \(changeAddress)")
print("Address: \(address)")

func createDescriptorsFromBip32RootKey(
    scriptType: ActiveWalletScriptType,
    network: Network
) -> (Descriptor, Descriptor) {
    let mnemonic = Mnemonic(wordCount: .words12)
    let bip32ExtendedRootKey = DescriptorSecretKey(
        network: network,
        mnemonic: mnemonic,
        password: nil
    )
    print("Bip32 root key: \(bip32ExtendedRootKey)")

    let descriptor = createScriptAppropriateDescriptor(
        scriptType: scriptType,
        bip32ExtendedRootKey: bip32ExtendedRootKey,
        network: network,
        keychain: .external
    )
    let changeDescriptor = createScriptAppropriateDescriptor(
        scriptType: scriptType,
        bip32ExtendedRootKey: bip32ExtendedRootKey,
        network: network,
        keychain: .internal
    )
    return (descriptor, changeDescriptor)
}

func createScriptAppropriateDescriptor(
    scriptType: ActiveWalletScriptType,
    bip32ExtendedRootKey: DescriptorSecretKey,
    network: Network,
    keychain: KeychainKind
) -> Descriptor {
    switch scriptType {
    case .p2wpkh:
        return Descriptor.newBip84(
            secretKey: bip32ExtendedRootKey,
            keychainKind: keychain,
            network: network
        )
    case .p2tr:
        return Descriptor.newBip86(
            secretKey: bip32ExtendedRootKey,
            keychainKind: keychain,
            network: network
        )
    }
}

func getPersistenceFilePath() -> String {
    let cwd = URL(fileURLWithPath: FileManager.default.currentDirectoryPath)
    let persistenceDirectory = cwd.appendingPathComponent("data")
    let persistenceFilePath = persistenceDirectory.appendingPathComponent("bdk_persistence.sqlite").path

    if !FileManager.default.fileExists(atPath: persistenceDirectory.path) {
        try? FileManager.default.createDirectory(at: persistenceDirectory, withIntermediateDirectories: true)
    }

    if FileManager.default.fileExists(atPath: persistenceFilePath) {
        try? FileManager.default.removeItem(atPath: persistenceFilePath)
    }

    return persistenceFilePath
}

enum ActiveWalletScriptType {
    case p2wpkh
    case p2tr
}
