import Foundation
import BitcoinDevKit

// Add your regtest URL here. Regtest environment must have esplora to run this.
// See here for regtest podman setup https://github.com/thunderbiscuit/podman-regtest-infinity-pro
let REGTEST_URL = "http://127.0.0.1:3002"

// Create 3 wallets
let aliceWallet = getNewWallet(script: .p2wpkh, network: .regtest)
let bobWallet = getNewWallet(script: .p2wpkh, network: .regtest)
let mattWallet = getNewWallet(script: .p2wpkh, network: .regtest)

// Get the public descriptors of each wallet. We only want the derivation path, so we take it out of the string
let alicePublicDescriptor = getKey(aliceWallet.publicDescriptor(keychain: .external))
let bobPublicDescriptor = getKey(bobWallet.publicDescriptor(keychain: .external))
let mattPublicDescriptor = getKey(mattWallet.publicDescriptor(keychain: .external))

// Get the change descriptors of each wallet
let aliceChangeDescriptor = getKey(aliceWallet.publicDescriptor(keychain: .internal))
let bobChangeDescriptor = getKey(bobWallet.publicDescriptor(keychain: .internal))
let mattChangeDescriptor = getKey(mattWallet.publicDescriptor(keychain: .internal))

// Define the descriptors for a multisig wallet with Alice, Bob and Matt's public descriptors.
let externalDescriptor = try! Descriptor(
    descriptor: "wsh(multi(2,\(alicePublicDescriptor),\(bobPublicDescriptor),\(mattPublicDescriptor)))",
    network: .regtest
)
let changeDescriptor = try! Descriptor(
    descriptor: "wsh(multi(2,\(aliceChangeDescriptor),\(bobChangeDescriptor),\(mattChangeDescriptor)))",
    network: .regtest
)

// Create multisig wallet
let connection = try! Persister.newSqlite(path: generateUniquePersistenceFilePath())
let multisigWallet = try! Wallet(
    descriptor: externalDescriptor,
    changeDescriptor: changeDescriptor,
    network: .regtest,
    persister: connection
)

let changeAddress = multisigWallet.revealNextAddress(keychain: .internal)
let address = multisigWallet.revealNextAddress(keychain: .external)
print("Change address: \(changeAddress.address)")

// Fund this address. Alice, Bob and Matt own funds sent here
print("recieving address: \(address.address)")

// Client
// Wait 40 second for you to send funds before scanning
print("Waiting 40 seconds for funds to arrive here before scanning \(address.address) ...")
Thread.sleep(forTimeInterval: 40)

let esploraClient = EsploraClient(url: REGTEST_URL)
let fullScanRequest = try! multisigWallet.startFullScan().build()
let update = try! esploraClient.fullScan(request: fullScanRequest, stopGap: 10, parallelRequests: 1)
try! multisigWallet.applyUpdate(update: update)
_ = try! multisigWallet.persist(persister: connection)
print("Balance: \(multisigWallet.balance().total.toSat())")

Thread.sleep(forTimeInterval: 40)

// Create PSBT
// Add your own recipient Address here. (Address to send funds to)
let recipient = try! Address(address: "bcrt1q645m0j78v9pajdfp0g0w6wacl4v8s7mvrwsjx5", network: .regtest)
print("Balance: \(multisigWallet.balance().total.toSat())")
let psbt = try! TxBuilder()
    .addRecipient(script: recipient.scriptPubkey(), amount: Amount.fromSat(satoshi: 4420))
    .feeRate(feeRate: try! FeeRate.fromSatPerVb(satVb: 22))
    .finish(wallet: multisigWallet)

let psbtTransaction = try! psbt.extractTx()
print(psbtTransaction)
print("Before any signature \(psbt.serialize())")
print("Before any signature. Json: \(psbt.jsonSerialize())")

// Sign Psbt
_ = try! aliceWallet.sign(psbt: psbt, signOptions: SignOptions(
    trustWitnessUtxo: false,
    assumeHeight: nil,
    allowAllSighashes: false,
    tryFinalize: false,
    signWithTapInternalKey: true,
    allowGrinding: false
))
print("After signing with aliceWallet \(psbt.serialize())")
print("After signing with aliceWallet Json \(psbt.jsonSerialize())")
print("Transaction after signing with wallet1: \(try! psbt.extractTx())")

_ = try! bobWallet.sign(psbt: psbt, signOptions: SignOptions(
    trustWitnessUtxo: false,
    assumeHeight: nil,
    allowAllSighashes: false,
    tryFinalize: false,
    signWithTapInternalKey: true,
    allowGrinding: false
))
print("After signing with bobWallet \(psbt.serialize())")
print("After signing with bobWallet Json \(psbt.jsonSerialize())")
print("Transaction after signing with bobWallet: \(try! psbt.extractTx())")

// Matt does not need to sign since it is a 2 or 3 multisig. Alice and Bob, have already signed, which satisfies the unlocking script.
_ = try! mattWallet.sign(psbt: psbt, signOptions: SignOptions(
    trustWitnessUtxo: false,
    assumeHeight: nil,
    allowAllSighashes: false,
    tryFinalize: false,
    signWithTapInternalKey: true,
    allowGrinding: false
))
print("After signing with mattWallet \(psbt.serialize())")
print("After signing with mattWallet Json \(psbt.jsonSerialize())")
print("Transaction after signing with mattWallet: \(try! psbt.extractTx())")

_ = try! multisigWallet.finalizePsbt(psbt: psbt)
print("After finalize: \(psbt.serialize())")
print("After finalize Json: \(psbt.jsonSerialize())")
print("Transaction after finalize: \(try! psbt.extractTx())")

let tx = try! psbt.extractTx()
print("Txid is: \(tx.computeTxid())")

// Now you can broadcast the transaction
try! esploraClient.broadcast(transaction: tx)
print("Tx was broadcasted")

func getNewWallet(script: ActiveWalletScriptType, network: Network) -> Wallet {
    let (descriptor, changeDescriptor) = createDescriptorsFromBip32RootKey(
        scriptType: script,
        network: network
    )
    let connection = try! Persister.newSqlite(path: generateUniquePersistenceFilePath())
    return try! Wallet(
        descriptor: descriptor,
        changeDescriptor: changeDescriptor,
        network: network,
        persister: connection
    )
}

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

func generateUniquePersistenceFilePath() -> String {
    let cwd = URL(fileURLWithPath: FileManager.default.currentDirectoryPath)
    let persistenceDirectory = cwd.appendingPathComponent("data")

    if !FileManager.default.fileExists(atPath: persistenceDirectory.path) {
        try? FileManager.default.createDirectory(at: persistenceDirectory, withIntermediateDirectories: true)
    }

    let randomSuffix = Int.random(in: 1...100000)
    return persistenceDirectory.appendingPathComponent("bdk_persistence_\(randomSuffix).sqlite").path
}

func getKey(_ descriptor: String) -> String {
    let open = descriptor.firstIndex(of: "(")!
    let close = descriptor.firstIndex(of: ")")!
    return String(descriptor[descriptor.index(after: open)..<close])
}

enum ActiveWalletScriptType {
    case p2wpkh
    case p2tr
}
