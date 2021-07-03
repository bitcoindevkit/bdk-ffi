package org.bitcoindevkit.bdk.wallet

import org.bitcoindevkit.bdk.JnaError
import org.bitcoindevkit.bdk.JnaException
import org.bitcoindevkit.bdk.LibBase
import org.bitcoindevkit.bdk.LibJna
import org.slf4j.Logger
import org.slf4j.LoggerFactory

class VecLocalUtxoResult(private val ffiResult: LibJna.FfiResultVec_LocalUtxo_t.ByValue) :
    LibBase() {

    protected open val log: Logger = LoggerFactory.getLogger(VecLocalUtxoResult::class.java)

    fun value(): Array<LibJna.LocalUtxo_t.ByReference> {
        val err = ffiResult.err
        val ok = ffiResult.ok
        when {
            err != null -> {
                val errString = err.getPointer(0).getString(0)
                log.error("JnaError: $errString")
                throw JnaException(JnaError.valueOf(errString))
            }
            else -> {
                val first = ok.ptr!!
                return first.toArray(ok.len!!.toInt()) as Array<LibJna.LocalUtxo_t.ByReference>
            }
        }
    }

    protected fun finalize() {
        libJna.free_unspent_result(ffiResult)
        log.debug("$ffiResult freed")
    }
}