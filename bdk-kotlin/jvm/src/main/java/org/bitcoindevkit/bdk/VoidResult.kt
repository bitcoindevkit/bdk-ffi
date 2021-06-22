package org.bitcoindevkit.bdk

import org.slf4j.Logger
import org.slf4j.LoggerFactory

class VoidResult internal constructor(private val voidResultT: LibJna.VoidResult_t) : LibBase() {

    private val log: Logger = LoggerFactory.getLogger(VoidResult::class.java)

    fun isErr(): Boolean {
        return libJna.get_void_err(voidResultT) != null
    }
    
    fun err(): String? {
        val errPointer = libJna.get_void_err(voidResultT)
        val err = errPointer?.getString(0)
        libJna.free_string(errPointer)
        return err
    }
    
    protected fun finalize() {
        libJna.free_void_result(voidResultT)
        log.debug("VoidResult_t freed")
    }
}