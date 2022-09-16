package org.bitcoindevkit

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

fun electrumBlockchainConfigExample() {
    val blockchainConfig = BlockchainConfig.Electrum(
        ElectrumConfig(
            "ssl://electrum.blockstream.info:60002",
            null,
            5u,
            null,
            200u
        )
    )
}

fun memoryDatabaseConfigExample() {
    val memoryDatabaseConfig = DatabaseConfig.Memory
}

fun sqliteDatabaseConfigExample() {
    val databaseConfig = DatabaseConfig.Sqlite(SqliteDbConfiguration("bdk-sqlite"))
}
