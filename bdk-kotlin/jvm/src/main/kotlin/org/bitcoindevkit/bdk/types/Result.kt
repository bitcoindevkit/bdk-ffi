package org.bitcoindevkit.bdk.types

import com.sun.jna.Pointer
import org.bitcoindevkit.bdk.FfiResult
import org.bitcoindevkit.bdk.JnaError
import org.bitcoindevkit.bdk.JnaException
import org.bitcoindevkit.bdk.LibBase
import org.slf4j.Logger
import org.slf4j.LoggerFactory

abstract class Result<T : FfiResult, RT : Any>(private val ffiResult: T): LibBase() {

    protected open val log: Logger = LoggerFactory.getLogger(Result::class.java)

    protected abstract fun getOkValue(pointer: Pointer): RT
    
    protected abstract fun freeResult(ffiResult: T)

    fun value(): RT {
        val err = ffiResult.err
        val ok = ffiResult.ok
        when {
            err != null -> {
                val errString = err.getPointer(0).getString(0)
                log.error("JnaError: $errString")
                throw JnaException(JnaError.valueOf(errString))
            }
            ok != null -> {
                return getOkValue(ok)
            }
            else -> {
                throw JnaException(JnaError.Generic)
            }
        }
    }

    protected fun finalize() {
        freeResult(ffiResult)
        log.debug("$ffiResult freed")
    }
}