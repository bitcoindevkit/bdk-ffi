package org.bitcoindevkit.bdk

import com.sun.jna.Native

abstract class LibBase {

    protected val libJna: LibJna
        get() = Native.load("bdk_ffi", LibJna::class.java)
}