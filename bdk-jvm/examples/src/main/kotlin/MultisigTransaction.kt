package org.bitcoindevkit

import java.nio.file.Paths

fun main() {
    // Add your regtest URL here. Regtest environment must have esplora to run this.
    // See here for regtest podman setup https://github.com/thunderbiscuit/podman-regtest-infinity-pro
    val REGTEST_URL = "http://127.0.0.1:3002"

    //Create 3 wallets
    val aliceWallet = getNewWallet(ActiveWalletScriptType.P2WPKH, Network.REGTEST)
    val bobWallet = getNewWallet(ActiveWalletScriptType.P2WPKH, Network.REGTEST)
    val mattWallet = getNewWallet(ActiveWalletScriptType.P2WPKH, Network.REGTEST)

    // Get the public descriptors of each wallet. We only want the derivation path, so we take it out of the string
    var alicePublicDescriptor = aliceWallet.publicDescriptor(KeychainKind.EXTERNAL)
    alicePublicDescriptor = alicePublicDescriptor.substring(alicePublicDescriptor.indexOf("(") + 1, alicePublicDescriptor.indexOf(")"))
    var bobPublicDescriptor = bobWallet.publicDescriptor(KeychainKind.EXTERNAL)
    bobPublicDescriptor = bobPublicDescriptor.substring(bobPublicDescriptor.indexOf("(") + 1, bobPublicDescriptor.indexOf(")"))
    var mattPublicDescriptor = mattWallet.publicDescriptor(KeychainKind.EXTERNAL)
    mattPublicDescriptor = mattPublicDescriptor.substring(mattPublicDescriptor.indexOf("(") + 1, mattPublicDescriptor.indexOf(")"))

    // Get the change descriptors of each wallet
    var aliceChangeDescriptor = aliceWallet.publicDescriptor(KeychainKind.INTERNAL)
    aliceChangeDescriptor = aliceChangeDescriptor.substring(aliceChangeDescriptor.indexOf("(") + 1, aliceChangeDescriptor.indexOf(")"))
    var bobChangeDescriptor = bobWallet.publicDescriptor(KeychainKind.INTERNAL)
    bobChangeDescriptor = bobChangeDescriptor.substring(bobChangeDescriptor.indexOf("(") + 1, bobChangeDescriptor.indexOf(")"))
    var mattChangeDescriptor = mattWallet.publicDescriptor(KeychainKind.INTERNAL)
    mattChangeDescriptor = mattChangeDescriptor.substring(mattChangeDescriptor.indexOf("(") + 1, mattChangeDescriptor.indexOf(")"))

    // Define the descriptors for a multisig wallet with Alice, Bob and Matt's public descriptors.
    val externalDescriptor = Descriptor(
        "wsh(multi(2,$alicePublicDescriptor,$bobPublicDescriptor,$mattPublicDescriptor))",
        Network.REGTEST
    )
    val changeDescriptor = Descriptor(
        "wsh(multi(2,$aliceChangeDescriptor,$bobChangeDescriptor,$mattChangeDescriptor))",
        Network.REGTEST
    )

    //Create multisig wallet
    val connection: Persister = Persister.newSqlite(generateUniquePersistenceFilePath())
    val multisigWallet = Wallet(externalDescriptor, changeDescriptor, Network.REGTEST, connection)

    val changeAddress = multisigWallet.revealNextAddress(KeychainKind.INTERNAL)
    val address = multisigWallet.revealNextAddress(KeychainKind.EXTERNAL)
    println("Change address: ${changeAddress.address}")

    //Fund this address. Alice, Bob and Matt own funds sent here
    println("recieving address: ${address.address}")

    //Client
    //Wait 40 second for you to send funds before scanning
    println("Waiting 40 seconds for funds to arrive here before scanning ${address.address} ...")
    Thread.sleep(40000)

    val esploraClient = EsploraClient(REGTEST_URL)
    val fullScanRequest: FullScanRequest = multisigWallet.startFullScan().build()
    val update = esploraClient.fullScan(fullScanRequest, 10uL, 1uL)
    multisigWallet.applyUpdate(update)
    multisigWallet.persist(connection)
    println("Balance: ${multisigWallet.balance().total.toSat()}")
    //esploraFullScanWallet(multisigWallet, esploraClient,fullScanRequest, connection)
    Thread.sleep(40000)

    // Create PSBT
    //Add your own recipient Address here. (Address to send funds to)
    val recipient: Address = Address("bcrt1q645m0j78v9pajdfp0g0w6wacl4v8s7mvrwsjx5", Network.REGTEST)
    println("Balance: ${multisigWallet.balance().total.toSat()}")
    val psbt: Psbt = TxBuilder()
        .addRecipient(recipient.scriptPubkey(), Amount.fromSat(4420uL))
        .feeRate(FeeRate.fromSatPerVb(22uL))
        .finish(multisigWallet)

    val psbtTransaction = psbt.extractTx()
    println(psbtTransaction.toString())
    println("Before any signature ${psbt.serialize()}")
    println("Before any signature. Json: ${psbt.jsonSerialize()}")

    // Sign Psbt
    val signedByAlice = aliceWallet.sign(psbt, SignOptions(
        trustWitnessUtxo = false,
        assumeHeight = null,
        allowAllSighashes = false,
        tryFinalize = false,
        signWithTapInternalKey = true,
        allowGrinding = false
    ))
    println("After signing with aliceWallet ${psbt.serialize()}")
    println("After signing with aliceWallet Json ${psbt.jsonSerialize()}")
    println("Transaction after signing with wallet1: ${psbt.extractTx().toString()}")

    val signedByBob =  bobWallet.sign(psbt, SignOptions(
        trustWitnessUtxo = false,
        assumeHeight = null,
        allowAllSighashes = false,
        tryFinalize = false,
        signWithTapInternalKey = true,
        allowGrinding = false
    ))
    println("After signing with bobWallet ${psbt.serialize()}")
    println("After signing with bobWallet Json ${psbt.jsonSerialize()}")
    println("Transaction after signing with bobWallet: ${psbt.extractTx().toString()}")

    // Matt does not need to sign since it is a 2 or 3 multisig. Alice and Bob, have already signed, which satisfies the unlocking script.
    val signedByMatt = mattWallet.sign(psbt, SignOptions(
        trustWitnessUtxo = false,
        assumeHeight = null,
        allowAllSighashes = false,
        tryFinalize = false,
        signWithTapInternalKey = true,
        allowGrinding = false
    ))
    println("After signing with mattWallet ${psbt.serialize()}")
    println("After signing with mattWallet Json ${psbt.jsonSerialize()}")
    println("Transaction after signing with mattWallet: ${psbt.extractTx().toString()}")

    multisigWallet.finalizePsbt(psbt)
    println("After finalize: ${psbt.serialize()}")
    println("After finalize Json: ${psbt.jsonSerialize()}")
    println("Transaction after finalize: ${psbt.extractTx().toString()}")

    val tx: Transaction = psbt.extractTx()
    println("Txid is: ${tx.computeTxid()}")

    //Now you can broadcast the transaction
    esploraClient.broadcast(tx)
    println("Tx was broadcasted")
}


fun esploraFullScanWallet(
    wallet: Wallet,
    esploraClient: EsploraClient,
    fullScanRequest: FullScanRequest,
    connection: Persister
) {
    val update = esploraClient.fullScan(fullScanRequest, 10uL, 1uL)
    wallet.applyUpdate(update)
    wallet.persist(connection)
    println("Balance: ${wallet.balance().total.toSat()}")
}

fun getNewWallet(script: ActiveWalletScriptType, network: Network): Wallet {
    val (descriptor, changeDescriptor) =  createDescriptorsFromBip32RootKey(
        script,
        network
    )
    val connection: Persister = Persister.newSqlite(generateUniquePersistenceFilePath())
    val wallet = Wallet(descriptor, changeDescriptor, network, connection)
    return wallet
}

fun generateUniquePersistenceFilePath(): String {
    // Resolve the absolute path to the fixed `examples/data` directory
    val projectRoot = Paths.get("").toAbsolutePath().parent.resolve("examples")
    val persistenceDirectory = projectRoot.resolve("data").toFile()

    persistenceDirectory.apply {
        if (!exists()) mkdirs()
    }

    // Generate a file path with a unique random suffix
    val randomSuffix = (1..100000).random() // Generate a random number
    val persistenceFilePath = projectRoot.resolve("data/bdk_persistence_$randomSuffix.sqlite").toString()
    return persistenceFilePath
}
