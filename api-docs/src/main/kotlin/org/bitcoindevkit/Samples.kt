package org.bitcoindevkit

class Samples {
    val blockchainConfig: BlockchainConfig = BlockchainConfig.Electrum(
        ElectrumConfig(
            url = "ssl://electrum.blockstream.info:60002",
            socks5 = null,
            retry = 5u,
            timeout = null,
            stopGap = 10u
        )
    )
}

