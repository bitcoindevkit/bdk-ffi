package org.bitcoindevkit
import java.io.File
import java.nio.file.Paths

fun main(){
    val (descriptor, changeDescriptor) = createDescriptorsFromBip32RootKey(
        ActiveWalletScriptType.P2WPKH,
        Network.REGTEST
    )
    println("Descriptor: $descriptor")
    println("Change descriptor: $changeDescriptor")

    val persistenceFilePath = getPersistenceFilePath()
    println("Persistence file path: $persistenceFilePath")

    val connection: Persister = Persister.newSqlite(persistenceFilePath)
    val wallet = Wallet(descriptor, changeDescriptor, Network.REGTEST, connection)
    val changeAddress = wallet.revealNextAddress(KeychainKind.INTERNAL).address
    val address = wallet.revealNextAddress(KeychainKind.EXTERNAL).address

    println("Change address: $changeAddress")
    println("Address: $address")
}

fun createDescriptorsFromBip32RootKey (
    activeWalletScriptType: ActiveWalletScriptType,
    network: Network) : Array<Descriptor>{
    val mnemonic = Mnemonic(WordCount.WORDS12)
    val bip32ExtendedRootKey = DescriptorSecretKey(network, mnemonic, null)
    println("Bip32 root key: $bip32ExtendedRootKey")
    
    val descriptor: Descriptor = createScriptAppropriateDescriptor(
        activeWalletScriptType,
        bip32ExtendedRootKey,
        network,
        KeychainKind.EXTERNAL,
    )
    val changeDescriptor: Descriptor = createScriptAppropriateDescriptor(
        activeWalletScriptType,
        bip32ExtendedRootKey,
        network,
        KeychainKind.INTERNAL,
    )
    return arrayOf(descriptor, changeDescriptor)
}


fun createScriptAppropriateDescriptor(
    scriptType: ActiveWalletScriptType,
    bip32ExtendedRootKey: DescriptorSecretKey,
    network: Network,
    keychain: KeychainKind,
): Descriptor {
    return when (scriptType) {
        ActiveWalletScriptType.P2WPKH -> Descriptor.newBip84(
            bip32ExtendedRootKey,
            keychain,
            network
        )
        ActiveWalletScriptType.P2TR -> Descriptor.newBip86(
            bip32ExtendedRootKey,
            keychain,
            network
        )
    }
}
fun getPersistenceFilePath(): String {
    // Resolve absolute path to the fixed `examples/data` directory
    val projectRoot = Paths.get("").toAbsolutePath().parent.resolve("examples")
    val persistenceDirectory = projectRoot.resolve("data").toFile()
    val persistenceFilePath = projectRoot.resolve("data/bdk_persistence.sqlite").toString()

    persistenceDirectory.apply {
        if (!exists()) mkdirs()
    }

    File(persistenceFilePath).apply {
        if (exists()) delete()
    }
    return persistenceFilePath
}

enum class ActiveWalletScriptType {
    P2WPKH,
    P2TR,
}
