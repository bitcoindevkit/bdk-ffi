package org.bitcoindevkit.bdk

import com.sun.jna.Pointer
import com.sun.jna.Structure

abstract class FfiResult : Structure {
    constructor() : super()
    constructor(pointer: Pointer) : super(pointer)

    @JvmField
    var ok: Pointer? = null

    @JvmField
    var err: Pointer? = null
}