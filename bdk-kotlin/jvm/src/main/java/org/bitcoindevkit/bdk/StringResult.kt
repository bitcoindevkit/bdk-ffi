package org.bitcoindevkit.bdk

import org.slf4j.Logger
import org.slf4j.LoggerFactory

class StringResult internal constructor(private val stringResultT: LibJna.StringResult_t) : LibBase() {

    private val log: Logger = LoggerFactory.getLogger(StringResult::class.java)

    fun isErr(): Boolean {
        return libJna.get_string_err(stringResultT) != null
    }
    
    fun err(): String? {
        val errPointer = libJna.get_string_err(stringResultT)
        val err = errPointer?.getString(0)
        libJna.free_string(errPointer)
        return err
    }
    
    fun ok(): String? {
        val okPointer = libJna.get_string_ok(stringResultT)
        val ok = okPointer?.getString(0)
        libJna.free_string(okPointer)
        return ok
    }
    
    protected fun finalize() {
        libJna.free_string_result(stringResultT)
        log.debug("StringResult_t freed")
    }
}