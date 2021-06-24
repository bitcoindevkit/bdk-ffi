package org.bitcoindevkit.bdk

import org.slf4j.Logger
import org.slf4j.LoggerFactory

class Wallet constructor(
    name: String,
    descriptor: String,
    changeDescriptor: String?,
) : LibBase() {

    val log: Logger = LoggerFactory.getLogger(Wallet::class.java)

    private val walletResult =
        WalletResult(libJna.new_wallet_result(name, descriptor, changeDescriptor))
    private val walletRefT = walletResult.value()

    fun sync() {
        val voidResult = VoidResult(libJna.sync_wallet(walletRefT.pointer))
        return voidResult.value()
    }

    fun getAddress(): String {
        val stringResult = StringResult(libJna.new_address(walletRefT.pointer))
        return stringResult.value()
    }

    protected fun finalize(pointer: LibJna.WalletResult_t) {
        libJna.free_wallet_ref(walletRefT)
        log.debug("$walletRefT freed")
    }
}