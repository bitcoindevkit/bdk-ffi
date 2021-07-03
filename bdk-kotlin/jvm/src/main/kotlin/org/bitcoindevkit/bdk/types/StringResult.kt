package org.bitcoindevkit.bdk.types

import com.sun.jna.Pointer
import org.bitcoindevkit.bdk.LibJna

class StringResult constructor(stringResultPtr: LibJna.FfiResult_char_ptr_t.ByValue) :
    Result<LibJna.FfiResult_char_ptr_t.ByValue, String>(stringResultPtr) {

    override fun getOkValue(pointer: Pointer): String {
        return pointer.getPointer(0).getString(0)
    }

    override fun freeResult(ffiResult: LibJna.FfiResult_char_ptr_t.ByValue) {
        libJna.free_string_result(ffiResult)
    }
}