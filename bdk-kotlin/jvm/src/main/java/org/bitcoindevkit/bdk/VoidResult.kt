package org.bitcoindevkit.bdk

import com.sun.jna.Pointer
import org.slf4j.Logger
import org.slf4j.LoggerFactory

class VoidResult internal constructor(voidResultT: LibJna.VoidResult_t) :
    ResultBase<LibJna.VoidResult_t, Unit>(voidResultT) {

    override val log: Logger = LoggerFactory.getLogger(VoidResult::class.java)

    override fun err(pointerT: LibJna.VoidResult_t): Pointer? {
        return libJna.get_void_err(pointerT)
    }

    override fun ok(pointerT: LibJna.VoidResult_t) {
        // Void
    }

    override fun free(pointerT: LibJna.VoidResult_t) {
        libJna.free_void_result(pointerT)
    }
}