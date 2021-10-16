import uniffi.bdk.*

class LogProgress: BdkProgress {
    override fun update(progress: Float, message: String? ) {
        println("progress: $progress, message: $message")
    }
}

fun main(args: Array<String>) {
    println("Configuring an in-memory wallet on electrum..")
    val descriptor =
        "pkh(cSQPHDBwXGjVzWRqAHm6zfvQhaTuj1f2bFH58h55ghbjtFwvmeXR)";
    val amount = 1000uL;
    val recipient = "tb1ql7w62elx9ucw4pj5lgw4l028hmuw80sndtntxt";
    val db = DatabaseConfig.Memory("")
    val client = BlockchainConfig.Electrum(ElectrumConfig("ssl://electrum.blockstream.info:60002", null, 5u, null, 10u))
    val wallet = OnlineWallet(descriptor, Network.TESTNET, db, client)
    val address = wallet.getNewAddress()
    println("Please send $amount satoshis to address: $address")
    readLine()
    println("Syncing...")
    wallet.sync(LogProgress(), null)
    val balance = wallet.getBalance()
    println("New wallet balance: $balance")
    println("Press Enter to return funds")
    readLine()
    println("Creating a partially signed bitcoin transaction with recipient $recipient and amount $amount satoshis...")
    val transaction = PartiallySignedBitcoinTransaction(wallet, recipient, amount)
    println("Signing the transaction...")
    wallet.sign(transaction)
    println("Broadcasting the signed transaction...")
    val transactionId = wallet.broadcast(transaction)
    println("Refunded $amount satoshis to $recipient via transaction id $transactionId")
    println("Syncing...")
    wallet.sync(LogProgress(), null)
    val final_balance = wallet.getBalance()
    println("New wallet balance: $final_balance")
    println("Press Enter to exit")
    readLine()
}
