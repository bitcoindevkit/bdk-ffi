package org.bitcoindevkit.bdk

import com.sun.jna.Pointer
import org.slf4j.Logger
import org.slf4j.LoggerFactory

class StringResult internal constructor(stringResultT: LibJna.StringResult_t) :
    ResultBase<LibJna.StringResult_t, String>(stringResultT) {

    override val log: Logger = LoggerFactory.getLogger(StringResult::class.java)

    override fun err(): Pointer? {
        return libJna.get_string_err(resultT)
    }

    override fun ok(): String {
        val okPointer = libJna.get_string_ok(resultT)
        val ok = okPointer!!.getString(0)
        libJna.free_string(okPointer)
        return ok
    }

    override fun free(pointer: LibJna.StringResult_t) {
        libJna.free_string_result(resultT)
    }
}