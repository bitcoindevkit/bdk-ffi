package org.bitcoindevkit.bdk.types

import org.bitcoindevkit.bdk.FfiException
import org.bitcoindevkit.bdk.LibBase
import org.bitcoindevkit.bdk.LibJna
import org.slf4j.Logger
import org.slf4j.LoggerFactory

class UInt64Result constructor(private val ffiResultUint64T: LibJna.FfiResult_uint64_t.ByValue) :
    LibBase() {

    private val log: Logger = LoggerFactory.getLogger(UInt64Result::class.java)

    fun value(): Long {
        val err = ffiResultUint64T.err
        val ok = ffiResultUint64T.ok
        when {
            err > 0 -> {
                throw FfiException(err)
            }
            else -> {
                return ok
            }
        }
    }

    protected fun finalize() {
        libJna.free_uint64_result(ffiResultUint64T)
        log.debug("$ffiResultUint64T freed")
    }
}