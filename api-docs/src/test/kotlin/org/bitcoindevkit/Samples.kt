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

