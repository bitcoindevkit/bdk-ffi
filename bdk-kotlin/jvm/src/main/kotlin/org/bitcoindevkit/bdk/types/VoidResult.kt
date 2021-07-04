package org.bitcoindevkit.bdk.types

import org.bitcoindevkit.bdk.JnaError
import org.bitcoindevkit.bdk.JnaException
import org.bitcoindevkit.bdk.LibBase
import org.bitcoindevkit.bdk.LibJna
import org.slf4j.Logger
import org.slf4j.LoggerFactory

class VoidResult constructor(private val ffiResultInt32T: LibJna.FfiResult_int32_t.ByValue) :
    LibBase() {

    private val log: Logger = LoggerFactory.getLogger(VoidResult::class.java)

    fun value(): Unit {
        val err = ffiResultInt32T.err
        //val ok = ffiResultInt32T.ok
        when {
            err.isNotEmpty() -> {
                log.error("JnaError: $err")
                throw JnaException(JnaError.valueOf(err))
            }
            else -> {
                return
            }
        }
    }

    protected fun finalize() {
        libJna.free_int_result(ffiResultInt32T)
        log.debug("$ffiResultInt32T freed")
    }
}