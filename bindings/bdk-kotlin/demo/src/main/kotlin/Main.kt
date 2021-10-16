import uniffi.bdk.*

class LogProgress: BdkProgress {
    override fun update(progress: Float, message: String? ) {
        println(progress);
        println(message);
    }
}

fun main(args: Array<String>) {
    val descriptor =
        "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/*)";
    val db = DatabaseConfig.Memory("")
    val client = BlockchainConfig.Electrum(ElectrumConfig("ssl://electrum.blockstream.info:60002", null, 5u, null, 100u))
    val wallet = OnlineWallet(descriptor, Network.TESTNET, db, client)
    val address = wallet.getNewAddress()
    println("Please send satoshis to wallet address: $address")
    readLine()
    println("Syncing...")
    wallet.sync(LogProgress(), null)
    val balance = wallet.getBalance()
    println("New wallet balance: $balance")
    println("Press any key to exit")
    readLine()
}
