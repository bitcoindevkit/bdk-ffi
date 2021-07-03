package org.bitcoindevkit.bdk

import com.sun.jna.Pointer
import com.sun.jna.PointerType
import org.slf4j.Logger
import org.slf4j.LoggerFactory

abstract class ResultBase<PT : PointerType, RT : Any> internal constructor(private val pointerT: PT) :
    LibBase() {

    protected open val log: Logger = LoggerFactory.getLogger(ResultBase::class.java)

    protected abstract fun err(pointerT: PT): Pointer?

    protected abstract fun ok(pointerT: PT): RT

    protected abstract fun free(pointerT: PT)

    private fun checkErr(pointerT: PT) {
        val errPointer = err(pointerT)
        val err = errPointer?.getString(0)
        libJna.free_string(errPointer)
        if (err != null) {
            log.error("JnaError: $err")
            throw JnaException(JnaError.valueOf(err))
        }
    }

    fun value(): RT {
        checkErr(pointerT)
        return ok(pointerT)
    }

    protected fun finalize() {
        free(pointerT)
        log.debug("$pointerT freed")
    }
}