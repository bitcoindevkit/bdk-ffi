package org.bitcoindevkit

import kotlin.test.Test

class LiveWalletTest {
    @Test
    fun testSyncedBalance() {
        val descriptor: Descriptor = Descriptor("wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/0/*)", Network.TESTNET)
        val wallet: Wallet = Wallet.newNoPersist(descriptor, null, Network.TESTNET)
        val esploraClient: EsploraClient = EsploraClient("https://mempool.space/testnet/api")
        // val esploraClient = EsploraClient("https://blockstream.info/testnet/api")
        val update = esploraClient.scan(wallet, 10uL, 1uL)
        wallet.applyUpdate(update)
        println("Balance: ${wallet.getBalance().total()}")

        assert(wallet.getBalance().total() > 0uL)
    }
}
