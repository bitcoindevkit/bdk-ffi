package org.bitcoindevkit.bdk.wallet

import org.bitcoindevkit.bdk.JnaError
import org.bitcoindevkit.bdk.JnaException
import org.bitcoindevkit.bdk.LibBase
import org.bitcoindevkit.bdk.LibJna
import org.slf4j.Logger
import org.slf4j.LoggerFactory

class WalletResult constructor(private val ffiResultOpaqueWalletPtrT: LibJna.FfiResult_OpaqueWallet_ptr_t.ByValue) :
    LibBase() {

    private val log: Logger = LoggerFactory.getLogger(WalletResult::class.java)

    fun value(): LibJna.OpaqueWallet_t {
        val err = ffiResultOpaqueWalletPtrT.err
        val ok = ffiResultOpaqueWalletPtrT.ok
        when {
            err.isNotEmpty() -> {
                log.error("JnaError: $err")
                throw JnaException(JnaError.valueOf(err))
            }
            else -> {
                return ok
            }
        }
    }

    protected fun finalize() {
        libJna.free_wallet_result(ffiResultOpaqueWalletPtrT)
        log.debug("$ffiResultOpaqueWalletPtrT freed")
    }
}