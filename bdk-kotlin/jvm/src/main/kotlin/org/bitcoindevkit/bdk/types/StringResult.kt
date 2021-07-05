package org.bitcoindevkit.bdk.types

import org.bitcoindevkit.bdk.FfiException
import org.bitcoindevkit.bdk.LibBase
import org.bitcoindevkit.bdk.LibJna
import org.slf4j.Logger
import org.slf4j.LoggerFactory

class StringResult constructor(private val ffiResultCharPtrT: LibJna.FfiResult_char_ptr_t.ByValue) :
    LibBase() {

    private val log: Logger = LoggerFactory.getLogger(StringResult::class.java)

    fun value(): String {
        val err = ffiResultCharPtrT.err!!
        val ok = ffiResultCharPtrT.ok!!
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
        libJna.free_string_result(ffiResultCharPtrT)
        log.debug("$ffiResultCharPtrT freed")
    }
}