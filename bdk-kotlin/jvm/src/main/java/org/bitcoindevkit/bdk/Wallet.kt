package org.bitcoindevkit.bdk

import org.slf4j.Logger
import org.slf4j.LoggerFactory

class Wallet constructor(
    descriptor: String,
    changeDescriptor: String?,
    blockchainConfig: BlockchainConfig,
    databaseConfig: DatabaseConfig,
) : LibBase() {

    val log: Logger = LoggerFactory.getLogger(Wallet::class.java)

    private val walletResult =
        WalletResult(
            libJna.new_wallet_result(
                descriptor,
                changeDescriptor,
                blockchainConfig.blockchainConfigT,
                databaseConfig.databaseConfigT
            )
        )
    private val walletRefT = walletResult.value()

    fun sync() {
        val voidResult = VoidResult(libJna.sync_wallet(walletRefT.pointer))
        return voidResult.value()
    }

    fun getAddress(): String {
        val stringResult = StringResult(libJna.new_address(walletRefT.pointer))
        return stringResult.value()
    }

    protected fun finalize(walletRefT: LibJna.WalletRef_t) {
        libJna.free_wallet_ref(walletRefT)
        log.debug("$walletRefT freed")
    }
}