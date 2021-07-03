package org.bitcoindevkit.bdk

class JnaException internal constructor(val err: JnaError) : Exception()