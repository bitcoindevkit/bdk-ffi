package org.bitcoindevkit.bdk.wallet

import org.bitcoindevkit.bdk.BlockchainConfig
import org.bitcoindevkit.bdk.DatabaseConfig
import org.bitcoindevkit.bdk.LibBase
import org.bitcoindevkit.bdk.LibJna
import org.bitcoindevkit.bdk.types.StringResult
import org.bitcoindevkit.bdk.types.UInt64Result
import org.bitcoindevkit.bdk.types.VoidResult
import org.slf4j.Logger
import org.slf4j.LoggerFactory

class Wallet constructor(
    descriptor: String,
    changeDescriptor: String?,
    blockchainConfig: BlockchainConfig,
    databaseConfig: DatabaseConfig,
) : LibBase() {

    val log: Logger = LoggerFactory.getLogger(Wallet::class.java)

    private val walletResult = WalletResult(
        libJna.new_wallet_result(
            descriptor,
            changeDescriptor,
            blockchainConfig.blockchainConfigT,
            databaseConfig.databaseConfigT
        )
    )
    private val wallet = walletResult.value()

    fun sync() {
        val voidResult = VoidResult(libJna.sync_wallet(wallet))
        return voidResult.value()
    }

    fun getAddress(): String {
        val stringResult = StringResult(libJna.new_address(wallet))
        return stringResult.value()
    }
    
    fun listUnspent(): Array<LibJna.LocalUtxo_t.ByReference> {
        val vecLocalUtxoResult = VecLocalUtxoResult(libJna.list_unspent(wallet))
        return vecLocalUtxoResult.value()
    }
    
    fun balance(): Long {
        val longResult = UInt64Result(libJna.balance(wallet))
        return longResult.value()
    }
    
    fun listTransactionDetails(): Array<LibJna.TransactionDetails_t.ByReference> {
        val vecTxDetailsResult = VecTxDetailsResult(libJna.list_transactions((wallet)))
        return vecTxDetailsResult.value()
    }
}