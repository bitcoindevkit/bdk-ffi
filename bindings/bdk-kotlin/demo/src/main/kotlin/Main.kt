import uniffi.bdk.*

class LogProgress: BdkProgress {
    override fun update(progress: Float, message: String? ) {
        println("progress: $progress, message: $message")
    }
}

fun getConfirmedTransaction(wallet: OnlineWalletInterface, transactionId: String): ConfirmedTransaction? {
    println("Syncing...")
    wallet.sync(LogProgress(), null)
    return wallet.getTransactions().stream().filter({ it.id.equals(transactionId) }).findFirst().orElse(null)
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
    println("Syncing...")
    wallet.sync(LogProgress(), null)
    println("Initial wallet balance: ${wallet.getBalance()}")
    println("Please send $amount satoshis to address: ${wallet.getNewAddress()}")
    readLine()
    println("Syncing...")
    wallet.sync(LogProgress(), null)
    println("New wallet balance: ${wallet.getBalance()}")
    println("Press Enter to return funds")
    readLine()
    println("Creating a partially signed bitcoin transaction with recipient $recipient and amount $amount satoshis...")
    val transaction = PartiallySignedBitcoinTransaction(wallet, recipient, amount)
    println("Signing the transaction...")
    wallet.sign(transaction)
    println("Broadcasting the signed transaction...")
    val transactionId = wallet.broadcast(transaction)
    println("Refunded $amount satoshis to $recipient via transaction id $transactionId")
    println("Confirming transaction...")
    var confirmedTransaction = getConfirmedTransaction(wallet, transactionId)
    while(confirmedTransaction == null) {
        confirmedTransaction = getConfirmedTransaction(wallet, transactionId)
    }
    println("Confirmed transaction: $confirmedTransaction")
    println("Final wallet balance: ${wallet.getBalance()}")
}
