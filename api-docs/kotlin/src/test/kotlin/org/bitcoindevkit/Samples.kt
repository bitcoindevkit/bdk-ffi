package org.bitcoindevkit

fun networkSample() {
    val wallet = Wallet(
        descriptor = descriptor,
        changeDescriptor = changeDescriptor,
        network = Network.TESTNET,
        databaseConfig = DatabaseConfig.Memory
    )
}

fun balanceSample() {
    object LogProgress : Progress {
        override fun update(progress: Float, message: String?) {}
    }

    val memoryDatabaseConfig = DatabaseConfig.Memory
    private val blockchainConfig = BlockchainConfig.Electrum(
        ElectrumConfig(
            "ssl://electrum.blockstream.info:60002",
            null,
            5u,
            null,
            200u
        )
    )
    val wallet = Wallet(descriptor, null, Network.TESTNET, memoryDatabaseConfig)
    val blockchain = Blockchain(blockchainConfig)
    wallet.sync(blockchain, LogProgress)

    val balance: Balance = wallet.getBalance()
    println("Total wallet balance is ${balance.total}")
}

fun electrumBlockchainConfigSample() {
    val blockchainConfig = BlockchainConfig.Electrum(
        ElectrumConfig(
            url = "ssl://electrum.blockstream.info:60002",
            socks5 = null,
            retry = 5u,
            timeout = null,
            stopGap = 200u
        )
    )
}

fun memoryDatabaseConfigSample() {
    val memoryDatabaseConfig = DatabaseConfig.Memory
}

fun sqliteDatabaseConfigSample() {
    val databaseConfig = DatabaseConfig.Sqlite(SqliteDbConfiguration("bdk-sqlite"))
}

fun addressIndexSample() {
    val wallet: Wallet = Wallet(
        descriptor = descriptor,
        changeDescriptor = changeDescriptor,
        network = Network.TESTNET,
        databaseConfig = DatabaseConfig.Memory
    )

    fun getLastUnusedAddress(): AddressInfo {
        return wallet.getAddress(AddressIndex.LAST_UNUSED)
    }
}

fun addressInfoSample() {
    val wallet: Wallet = Wallet(
        descriptor = descriptor,
        changeDescriptor = changeDescriptor,
        network = Network.TESTNET,
        databaseConfig = DatabaseConfig.Memory
    )

    fun getLastUnusedAddress(): AddressInfo {
        return wallet.getAddress(AddressIndex.NEW)
    }

    val newAddress: AddressInfo = getLastUnusedAddress()

    println("New address at index ${newAddress.index} is ${newAddress.address}")
}

fun blockchainSample() {
    val blockchainConfig: BlockchainConfig = BlockchainConfig.Electrum(
        ElectrumConfig(
            electrumURL,
            null,
            5u,
            null,
            10u
        )
    )

    val blockchain: Blockchain = Blockchain(blockchainConfig)

    blockchain.broadcast(signedPsbt)
}


fun txBuilderResultSample1() {
    val faucetAddress = Address("tb1ql7w62elx9ucw4pj5lgw4l028hmuw80sndtntxt")
    // TxBuilderResult is a data class, which means you can use destructuring declarations on it to
    // open it up in its component parts
    val (psbt, txDetails) = TxBuilder()
        .addRecipient(faucetAddress.scriptPubkey(), 1000u)
        .feeRate(1.2f)
        .finish(wallet)

    println("Txid is ${txDetails.txid}")
    wallet.sign(psbt)
}

fun txBuilderResultSample2() {
    val faucetAddress = Address("tb1ql7w62elx9ucw4pj5lgw4l028hmuw80sndtntxt")
    val txBuilderResult: TxBuilderResult = TxBuilder()
        .addRecipient(faucetAddress.scriptPubkey(), 1000u)
        .feeRate(1.2f)
        .finish(wallet)

    val psbt = txBuilderResult.psbt
    val txDetails = txBuilderResult.transactionDetails

    println("Txid is ${txDetails.txid}")
    wallet.sign(psbt)
}

fun descriptorSecretKeyExtendSample() {
    // The `DescriptorSecretKey.extend()` method allows you to extend a key to any given path.

    // val mnemonic: String = generateMnemonic(WordCount.WORDS12)
    val mnemonic: Mnemonic = Mnemonic("scene change clap smart together mind wheel knee clip normal trial unusual")

    // the initial DescriptorSecretKey will always be at the "master" node,
    // i.e. the derivation path is empty
    val bip32RootKey: DescriptorSecretKey = DescriptorSecretKey(
        network = Network.TESTNET,
        mnemonic = mnemonic,
        password = ""
    )
    println(bip32RootKey.asString())
    // tprv8ZgxMBicQKsPfM8Trx2apvdEkmxbJkYY3ZsmcgKb2bfnLNcBhtCstqQTeFesMRLEJXpjGDinAUJUHprXMwph8dQBdS1HAoxEis8Knimxovf/*

    // the derive method will also automatically apply the wildcard (*) to your path,
    // i.e the following will generate the typical testnet BIP84 external wallet path
    // m/84h/1h/0h/0/*
    val bip84ExternalPath: DerivationPath = DerivationPath("m/84h/1h/0h/0")
    val externalExtendedKey: DescriptorSecretKey = bip32RootKey.extend(bip84ExternalPath).asString()
    println(externalExtendedKey)
    // tprv8ZgxMBicQKsPfM8Trx2apvdEkmxbJkYY3ZsmcgKb2bfnLNcBhtCstqQTeFesMRLEJXpjGDinAUJUHprXMwph8dQBdS1HAoxEis8Knimxovf/84'/1'/0'/0/*

    // to create the descriptor you'll need to use this extended key in a descriptor function,
    // i.e. wpkh(), tr(), etc.
    val externalDescriptor = "wpkh($externalExtendedKey)"
}

fun descriptorSecretKeyDeriveSample() {
    // The DescriptorSecretKey.derive() method allows you to derive an extended key for a given
    // node in the derivation tree (for example to create an xpub for a particular account)

    val mnemonic: Mnemonic = Mnemonic("scene change clap smart together mind wheel knee clip normal trial unusual")
    val bip32RootKey: DescriptorSecretKey = DescriptorSecretKey(
        network = Network.TESTNET,
        mnemonic = mnemonic,
        password = ""
    )

    val bip84Account0: DerivationPath = DerivationPath("m/84h/1h/0h")
    val xpubAccount0: DescriptorSecretKey = bip32RootKey.derive(bip84Account0)
    println(xpubAccount0.asString())
    // [5512949b/84'/1'/0']tprv8ghw3FWfWTeLCEXcr8f8Q8Lz4QPCELYv3jhBXjAm7XagA6R5hreeWLTJeLBfMj7Ni6Q3PdV1o8NbvNBHE59W97EkRJSU4JkvTQjaNUmQubE/*

    val internalPath: DerivationPath = DerivationPath("m/0")
    val externalExtendedKey = xpubAccount0.extend(internalPath).asString()
    println(externalExtendedKey)
    // [5512949b/84'/1'/0']tprv8ghw3FWfWTeLCEXcr8f8Q8Lz4QPCELYv3jhBXjAm7XagA6R5hreeWLTJeLBfMj7Ni6Q3PdV1o8NbvNBHE59W97EkRJSU4JkvTQjaNUmQubE/0/*

    // to create the descriptor you'll need to use this extended key in a descriptor function,
    // i.e. wpkh(), tr(), etc.
    val externalDescriptor = "wpkh($externalExtendedKey)"
}

fun createTransaction() {
    val wallet = BdkWallet(
        descriptor = externalDescriptor,
        changeDescriptor = internalDescriptor,
        network = Network.TESTNET,
        databaseConfig = memoryDatabaseConfig,
    )
    val blockchainConfig = BlockchainConfig.Electrum(
        ElectrumConfig(
            "ssl://electrum.blockstream.info:60002",
            null,
            5u,
            null,
            200u
        )
    )

    val paymentAddress: Address = Address("tb1ql7w62elx9ucw4pj5lgw4l028hmuw80sndtntxt")
    val (psbt, txDetails) = TxBuilder()
        .addRecipient(faucetAddress.scriptPubkey(), 1000u)
        .feeRate(1.2f)
        .finish(wallet)

    wallet.sign(psbt)
    blockchain.broadcast(psbt)
}

fun walletSample() {
    val externalDescriptor = "wpkh(tprv8hwWMmPE4BVNxGdVt3HhEERZhondQvodUY7Ajyseyhudr4WabJqWKWLr4Wi2r26CDaNCQhhxEfVULesmhEfZYyBXdE/84h/1h/0h/0/*)"
    val internalDescriptor = "wpkh(tprv8hwWMmPE4BVNxGdVt3HhEERZhondQvodUY7Ajyseyhudr4WabJqWKWLr4Wi2r26CDaNCQhhxEfVULesmhEfZYyBXdE/84h/1h/0h/1/*)"
    val sqliteDatabaseConfig = DatabaseConfig.Sqlite(SqliteDbConfiguration("bdk-sqlite"))

    val wallet = BdkWallet(
        descriptor = externalDescriptor,
        changeDescriptor = internalDescriptor,
        network = Network.TESTNET,
        databaseConfig = sqliteDatabaseConfig,
    )
}

fun mnemonicSample() {
    val mnemonic0: Mnemonic = Mnemonic(WordCount.WORDS12)

    val mnemonic1: Mnemonic = Mnemonic.fromString("scene change clap smart together mind wheel knee clip normal trial unusual")

    val entropy: List<UByte> = listOf<UByte>(0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u)
    val mnemonic2: Mnemonic = Mnemonic.fromEntropy(entropy)

    println(mnemonic0.asString(), mnemonic1.asString(), mnemonic2.asString())
}
