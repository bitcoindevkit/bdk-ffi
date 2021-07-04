package org.bitcoindevkit.bdk.wallet

import org.bitcoindevkit.bdk.FfiException
import org.bitcoindevkit.bdk.LibBase
import org.bitcoindevkit.bdk.LibJna
import org.slf4j.Logger
import org.slf4j.LoggerFactory

class VecLocalUtxoResult(private val ffiResultVecLocalUtxoT: LibJna.FfiResultVec_LocalUtxo_t.ByValue) :
    LibBase() {

    private val log: Logger = LoggerFactory.getLogger(VecLocalUtxoResult::class.java)

    fun value(): Array<LibJna.LocalUtxo_t.ByReference> {
        val err = ffiResultVecLocalUtxoT.err
        val ok = ffiResultVecLocalUtxoT.ok
        when {
            err > 0 -> {
                throw FfiException(err)
            }
            else -> {
                val first = ok.ptr!!
                return first.toArray(ok.len!!.toInt()) as Array<LibJna.LocalUtxo_t.ByReference>
            }
        }
    }

    protected fun finalize() {
        libJna.free_veclocalutxo_result(ffiResultVecLocalUtxoT)
        log.debug("$ffiResultVecLocalUtxoT freed")
    }
}