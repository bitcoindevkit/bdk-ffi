package org.bitcoindevkit.bdk.types

import com.sun.jna.Pointer
import org.bitcoindevkit.bdk.LibJna

class VoidResult constructor(voidResultPtr: LibJna.FfiResult_void_t.ByValue) :
    Result<LibJna.FfiResult_void_t.ByValue, Unit>(voidResultPtr) {

    override fun getOkValue(pointer: Pointer) {
        // No value
    }

    override fun freeResult(ffiResult: LibJna.FfiResult_void_t.ByValue) {
        libJna.free_void_result(ffiResult)
    }
}