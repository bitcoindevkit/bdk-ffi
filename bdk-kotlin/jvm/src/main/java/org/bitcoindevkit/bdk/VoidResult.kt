package org.bitcoindevkit.bdk

import com.sun.jna.Pointer
import org.slf4j.Logger
import org.slf4j.LoggerFactory

class VoidResult internal constructor(voidResultT: LibJna.VoidResult_t) :
    ResultBase<LibJna.VoidResult_t, Unit>(voidResultT) {

    override val log: Logger = LoggerFactory.getLogger(VoidResult::class.java)

    override fun err(): Pointer? {
        return libJna.get_void_err(resultT)
    }

    override fun ok() {
        // Void
    }

    override fun free(pointer: LibJna.VoidResult_t) {
        libJna.free_void_result(resultT)
    }
}