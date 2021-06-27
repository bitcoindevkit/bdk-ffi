package org.bitcoindevkit.bdk

import com.sun.jna.Pointer
import org.slf4j.Logger
import org.slf4j.LoggerFactory

class StringResult internal constructor(stringResultT: LibJna.StringResult_t) :
    ResultBase<LibJna.StringResult_t, String>(stringResultT) {

    override val log: Logger = LoggerFactory.getLogger(StringResult::class.java)

    override fun err(pointerT: LibJna.StringResult_t): Pointer? {
        return libJna.get_string_err(pointerT)
    }

    override fun ok(pointerT: LibJna.StringResult_t): String {
        val okPointer = libJna.get_string_ok(pointerT)
        val ok = okPointer!!.getString(0)
        libJna.free_string(okPointer)
        return ok
    }

    override fun free(pointerT: LibJna.StringResult_t) {
        libJna.free_string_result(pointerT)
    }
}