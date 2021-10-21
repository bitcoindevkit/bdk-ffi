import java.util.Optional
import kotlin.ExperimentalUnsignedTypes
import uniffi.bdk.*

class LogProgress : BdkProgress {
    override fun update(progress: Float, message: String?) {
        println("Syncing..")
    }
}

class NullProgress : BdkProgress {
    override fun update(progress: Float, message: String?) {}
}

fun getTransaction(wallet: OnlineWalletInterface, transactionId: String): Optional<Transaction> {
    wallet.sync(NullProgress(), null)
    return wallet.getTransactions()
            .stream()
            .filter({
                when (it) {
                    is Transaction.Confirmed -> it.details.id.equals(transactionId)
                    is Transaction.Unconfirmed -> it.details.id.equals(transactionId)
                }
            })
            .findFirst()
}

@ExperimentalUnsignedTypes
val unconfirmedFirstThenByTimestampDescending =
        Comparator<Transaction> { a, b ->
            when {
                (a is Transaction.Confirmed && b is Transaction.Confirmed) -> {
                    val comparison = b.confirmation.timestamp.compareTo(a.confirmation.timestamp)
                    when {
                        comparison == 0 -> b.details.id.compareTo(a.details.id)
                        else -> comparison
                    }
                }
                (a is Transaction.Confirmed && b is Transaction.Unconfirmed) -> 1
                (a is Transaction.Unconfirmed && b is Transaction.Confirmed) -> -1
                else -> 0
            }
        }

@ExperimentalUnsignedTypes
fun main(args: Array<String>) {
    val network = Network.TESTNET
    val mnemonicType = MnemonicType.WORDS12
    val password: String? = null
    println("Generating key...")
    val extendedKey = generateExtendedKey(network, mnemonicType, password)
    println("generated key: $extendedKey")
    println("Attempting restore extended key...")
    val restoredKey = restoreExtendedKey(network, extendedKey.mnemonic, password)
    println("restored key: $restoredKey")
    println("Configuring an in-memory wallet on electrum..")
    val descriptor = "wpkh(tprv8ZgxMBicQKsPeSitUfdxhsVaf4BXAASVAbHypn2jnPcjmQZvqZYkeqx7EHQTWvdubTSDa5ben7zHC7sUsx4d8tbTvWdUtHzR8uhHg2CW7MT/*)"
    val amount = 1000uL
    val recipient = "tb1ql7w62elx9ucw4pj5lgw4l028hmuw80sndtntxt"
    val db = DatabaseConfig.Memory("")
    val client =
            BlockchainConfig.Electrum(
                    ElectrumConfig("ssl://electrum.blockstream.info:60002", null, 5u, null, 10u)
            )
    val wallet = OnlineWallet(descriptor, network, db, client)
    wallet.sync(LogProgress(), null)
    println("Initial wallet balance: ${wallet.getBalance()}")
    println("Please send $amount satoshis to address: ${wallet.getNewAddress()}")
    readLine()
    wallet.sync(LogProgress(), null)
    println("New wallet balance: ${wallet.getBalance()}")
    println("Press Enter to return funds")
    readLine()
    println("Creating a PSBT with recipient $recipient and amount $amount satoshis...")
    val transaction = PartiallySignedBitcoinTransaction(wallet, recipient, amount)
    println("Signing the transaction...")
    wallet.sign(transaction)
    println("Broadcasting the signed transaction...")
    val transactionId = wallet.broadcast(transaction)
    println("Broadcasted transaction with id $transactionId")
    val take = 5
    println("Listing latest $take transactions...")
    wallet
            .getTransactions()
            .sortedWith(unconfirmedFirstThenByTimestampDescending)
            .take(take)
            .forEach { println(it) }
    println("Final wallet balance: ${wallet.getBalance()}")
}
