package org.bitcoindevkit.bdk

import org.slf4j.Logger
import org.slf4j.LoggerFactory

class Wallet internal constructor(
    private val walletRefT: LibJna.WalletRef_t,
) : LibBase() {
    
    private val log: Logger = LoggerFactory.getLogger(Wallet::class.java)
    
    fun sync(): VoidResult {
        return VoidResult(libJna.sync_wallet(walletRefT.pointer))
    }
    
    fun getAddress(): StringResult {
        return StringResult(libJna.new_address(walletRefT.pointer))
    }

    protected fun finalize() {
        libJna.free_wallet_ref(walletRefT)
        log.debug("WalletRef_t freed")
    }

}