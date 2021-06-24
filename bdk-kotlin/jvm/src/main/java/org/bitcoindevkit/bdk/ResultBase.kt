package org.bitcoindevkit.bdk

import com.sun.jna.Pointer
import com.sun.jna.PointerType
import org.slf4j.Logger
import org.slf4j.LoggerFactory

abstract class ResultBase<PT : PointerType, RT : Any> internal constructor(protected val resultT: PT) :
    LibBase() {

    protected open val log: Logger = LoggerFactory.getLogger(ResultBase::class.java)

    protected abstract fun err(): Pointer?

    protected abstract fun ok(): RT

    protected abstract fun free(pointer: PT)

    private fun checkErr() {
        val errPointer = err()
        val err = errPointer?.getString(0)
        libJna.free_string(errPointer)
        if (err != null) {
            throw JnaException(JnaError.valueOf(err))
        }
    }

    fun value(): RT {
        checkErr()
        return ok()
    }

    protected fun finalize() {
        free(resultT)
        log.debug("$resultT freed")
    }
}