package org.bitcoindevkit.bdk.wallet

import org.bitcoindevkit.bdk.FfiException
import org.bitcoindevkit.bdk.LibBase
import org.bitcoindevkit.bdk.LibJna
import org.slf4j.Logger
import org.slf4j.LoggerFactory

class VecTxDetailsResult(private val ffiResultVecTransactionDetailsT: LibJna.FfiResult_Vec_TransactionDetails_t.ByValue) :
    LibBase() {

    private val log: Logger = LoggerFactory.getLogger(VecTxDetailsResult::class.java)

    fun value(): Array<LibJna.TransactionDetails_t.ByReference> {
        val err = ffiResultVecTransactionDetailsT.err!!
        val ok = ffiResultVecTransactionDetailsT.ok!!
        when {
            err > 0 -> {
                throw FfiException(err)
            }
            else -> {
                val first = ok.ptr!!
                return first.toArray(ok.len!!.toInt()) as Array<LibJna.TransactionDetails_t.ByReference>
            }
        }
    }

    protected fun finalize() {
        libJna.free_vectxdetails_result(ffiResultVecTransactionDetailsT)
        log.debug("$ffiResultVecTransactionDetailsT freed")
    }
}