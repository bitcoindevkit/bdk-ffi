package org.bitcoindevkit.bdk

import org.slf4j.Logger
import org.slf4j.LoggerFactory

class FfiException(val err: FfiError) : Exception() {
    private val log: Logger = LoggerFactory.getLogger(FfiException::class.java)

    init {
        log.error("JnaError: [{}] {}",err.ordinal, err.name)
    }

    internal constructor(err: Short) : this(FfiError.values()[err.toInt()])
}