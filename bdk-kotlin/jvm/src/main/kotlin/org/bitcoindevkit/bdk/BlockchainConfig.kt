package org.bitcoindevkit.bdk

import org.slf4j.Logger
import org.slf4j.LoggerFactory

abstract class BlockchainConfig() : LibBase() {
    private val log: Logger = LoggerFactory.getLogger(BlockchainConfig::class.java)
    abstract val blockchainConfigT: LibJna.BlockchainConfig_t

    protected fun finalize() {
        libJna.free_blockchain_config(blockchainConfigT)
        log.debug("$blockchainConfigT freed")
    }
}

class ElectrumConfig(
    url: String,
    socks5: String?,
    retry: Short,
    timeout: Short
) : BlockchainConfig() {

    private val log: Logger = LoggerFactory.getLogger(ElectrumConfig::class.java)
    override val blockchainConfigT = libJna.new_electrum_config(url, socks5, retry, timeout)
}