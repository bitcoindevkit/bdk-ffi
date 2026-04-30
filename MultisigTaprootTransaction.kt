package org.bitcoindevkit

fun main() {
    // Add your regtest URL here. Regtest environment must have esplora to run this.
    // See here for regtest podman setup https://github.com/thunderbiscuit/podman-regtest-infinity-pro
    val REGTEST_URL = "http://127.0.0.1:3002"

    // An unspendable compressed public key to use as internal key for Taproot multisig
    // This could be any valid compressed public key that no one has the private key for or
    // full descriptor path including derivation.
    val UNSPENDABLE_KEY = "0250929b74c1a04954b78b4b6035e97a5e078a5a0f28ec96d547bfee9ace803ac0"

    // Create 3 wallets using existing helper (P2TR = Taproot)
    val aliceWallet = getNewWallet(ActiveWalletScriptType.P2TR, Network.REGTEST)
    val bobWallet = getNewWallet(ActiveWalletScriptType.P2TR, Network.REGTEST)
    val mattWallet = getNewWallet(ActiveWalletScriptType.P2TR, Network.REGTEST)

    // Get the public descriptors of each wallet. Extract inner key expressions
    var alicePublicDescriptor = aliceWallet.publicDescriptor(KeychainKind.EXTERNAL)
    alicePublicDescriptor = alicePublicDescriptor.substring(alicePublicDescriptor.indexOf("(") + 1, alicePublicDescriptor.indexOf(")"))
    var bobPublicDescriptor = bobWallet.publicDescriptor(KeychainKind.EXTERNAL)
    bobPublicDescriptor = bobPublicDescriptor.substring(bobPublicDescriptor.indexOf("(") + 1, bobPublicDescriptor.indexOf(")"))
    var mattPublicDescriptor = mattWallet.publicDescriptor(KeychainKind.EXTERNAL)
    mattPublicDescriptor = mattPublicDescriptor.substring(mattPublicDescriptor.indexOf("(") + 1, mattPublicDescriptor.indexOf(")"))

    // Get the change descriptors of each wallet (for change path)
    var aliceChangeDescriptor = aliceWallet.publicDescriptor(KeychainKind.INTERNAL)
    aliceChangeDescriptor = aliceChangeDescriptor.substring(aliceChangeDescriptor.indexOf("(") + 1, aliceChangeDescriptor.indexOf(")"))
    var bobChangeDescriptor = bobWallet.publicDescriptor(KeychainKind.INTERNAL)
    bobChangeDescriptor = bobChangeDescriptor.substring(bobChangeDescriptor.indexOf("(") + 1, bobChangeDescriptor.indexOf(")"))
    var mattChangeDescriptor = mattWallet.publicDescriptor(KeychainKind.INTERNAL)
    mattChangeDescriptor = mattChangeDescriptor.substring(mattChangeDescriptor.indexOf("(") + 1, mattChangeDescriptor.indexOf(")"))

    // Define Taproot multisig descriptors. For Taproot, an internal key is required.
    // We use a random unspendable compressed public key as the internal key. This means
    // that the wallet can only be spent via the script path (no key-path spending).
    val externalDescriptor = Descriptor(
        descriptor = "tr($UNSPENDABLE_KEY,multi_a(2,$alicePublicDescriptor,$bobPublicDescriptor,$mattPublicDescriptor))",
        network = Network.REGTEST
    )
    val changeDescriptor = Descriptor(
        descriptor = "tr($UNSPENDABLE_KEY,multi_a(2,$aliceChangeDescriptor,$bobChangeDescriptor,$mattChangeDescriptor))",
        network = Network.REGTEST
    )

    // Create multisig wallet (reuse shared persistence helper)
    val connection: Persister = Persister.newSqlite(generateUniquePersistenceFilePath())
    val multisigWallet = Wallet(externalDescriptor, changeDescriptor, Network.REGTEST, connection)

    val changeAddress = multisigWallet.revealNextAddress(KeychainKind.INTERNAL)
    val address = multisigWallet.revealNextAddress(KeychainKind.EXTERNAL)
    println("Change address: ${changeAddress.address}")

    // Fund this address. Alice, Bob and Matt own funds sent here
    println("receiving address: ${address.address}")

    // Client: wait for funding then scan
    println("Waiting 40 seconds for funds to arrive here before scanning ${address.address} ...")
    Thread.sleep(40000)

    val esploraClient = EsploraClient(REGTEST_URL)
    val fullScanRequest: FullScanRequest = multisigWallet.startFullScan().build()
    val update = esploraClient.fullScan(fullScanRequest, 10uL, 1uL)
    multisigWallet.applyUpdate(update)
    multisigWallet.persist(connection)
    println("Balance: ${multisigWallet.balance().total.toSat()}")

    Thread.sleep(40000)

    // Create PSBT
    val recipient = Address("bcrt1q645m0j78v9pajdfp0g0w6wacl4v8s7mvrwsjx5", Network.REGTEST)
    println("Balance: ${multisigWallet.balance().total.toSat()}")
    val psbt: Psbt = TxBuilder()
        .addRecipient(recipient.scriptPubkey(), Amount.fromSat(4420uL))
        .feeRate(FeeRate.fromSatPerVb(22uL))
        .finish(multisigWallet)

    val psbtTransaction = psbt.extractTx()
    println(psbtTransaction.toString())
    println("Before any signature ${psbt.serialize()}")
    println("Before any signature. Json: ${psbt.jsonSerialize()}")

    // Sign Psbt via script path; do not allow key-path signing so taptree fields are populated
    aliceWallet.sign(psbt, SignOptions(
        trustWitnessUtxo = false,
        assumeHeight = null,
        allowAllSighashes = false,
        tryFinalize = false,
        signWithTapInternalKey = false,
        allowGrinding = false
    ))
    println("After signing with aliceWallet ${psbt.serialize()}")
    println("After signing with aliceWallet Json ${psbt.jsonSerialize()}")
    println("Transaction after signing with wallet1: ${psbt.extractTx().toString()}")

    bobWallet.sign(psbt, SignOptions(
        trustWitnessUtxo = false,
        assumeHeight = null,
        allowAllSighashes = false,
        tryFinalize = false,
        signWithTapInternalKey = false,
        allowGrinding = false
    ))
    println("After signing with bobWallet ${psbt.serialize()}")
    println("After signing with bobWallet Json ${psbt.jsonSerialize()}")
    println("Transaction after signing with bobWallet: ${psbt.extractTx().toString()}")

    // Matt does not need to sign since it is a 2 of 3 multisig. Alice and Bob have already signed.
    mattWallet.sign(psbt, SignOptions(
        trustWitnessUtxo = false,
        assumeHeight = null,
        allowAllSighashes = false,
        tryFinalize = false,
        signWithTapInternalKey = false,
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

    // Now you can broadcast the transaction
    esploraClient.broadcast(tx)
    println("Tx was broadcasted")
}