package org.bitcoindevkit.bdk

import org.slf4j.Logger
import org.slf4j.LoggerFactory

class WalletResult(
    name: String,
    descriptor: String,
    changeDescriptor: String?,
) : LibBase() {
    
    private val log: Logger = LoggerFactory.getLogger(WalletResult::class.java)
    private val walletResultT = libJna.new_wallet_result(name, descriptor, changeDescriptor)
    
    fun isErr(): Boolean {
        return libJna.get_wallet_err(walletResultT) != null
    }

    fun err(): String? {
        val errPointer = libJna.get_wallet_err(walletResultT)
        val err = errPointer?.getString(0)
        libJna.free_string(errPointer)
        return err
    }

    fun ok(): Wallet? {
        val okWalletRef = libJna.get_wallet_ok(walletResultT)
        return if (okWalletRef != null) Wallet(okWalletRef) else null
    }

    protected fun finalize() {
        libJna.free_wallet_result(walletResultT)
        log.debug("WalletResult_t freed")
    }

}