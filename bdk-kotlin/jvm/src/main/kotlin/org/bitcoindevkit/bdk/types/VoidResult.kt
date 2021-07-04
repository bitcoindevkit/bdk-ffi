package org.bitcoindevkit.bdk.types

import org.bitcoindevkit.bdk.FfiException
import org.bitcoindevkit.bdk.LibBase
import org.bitcoindevkit.bdk.LibJna
import org.slf4j.Logger
import org.slf4j.LoggerFactory

class VoidResult constructor(private val ffiResultVoidT: LibJna.FfiResultVoid_t.ByValue) :
    LibBase() {

    private val log: Logger = LoggerFactory.getLogger(VoidResult::class.java)

    fun value(): Unit {
        val err = ffiResultVoidT.err

        when {
            err > 0 -> {
                throw FfiException(err)
            }
            else -> {
                return
            }
        }
    }

    protected fun finalize() {
        libJna.free_void_result(ffiResultVoidT)
        log.debug("$ffiResultVoidT freed")
    }
}