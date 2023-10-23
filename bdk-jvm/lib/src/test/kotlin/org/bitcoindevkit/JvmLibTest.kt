package org.bitcoindevkit

import org.junit.Test

class WalletTest {
    @Test
    fun testNetwork() {
        val signetNetwork = Network.SIGNET
    }

    @Test
    fun testDescriptorBip86() {
        val mnemonic = Mnemonic(WordCount.WORDS12)
        val descriptorSecretKey = DescriptorSecretKey(Network.TESTNET, mnemonic, null)
        val descriptor = Descriptor.newBip86(descriptorSecretKey, KeychainKind.EXTERNAL, Network.TESTNET)
    }

    @Test
    fun testUsedWallet() {
        val descriptor = Descriptor("wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/0/*)", Network.TESTNET)
        val wallet = Wallet.newNoPersist(descriptor, null, Network.TESTNET)
        val (index, address, keychain) = wallet.getAddress(AddressIndex.LastUnused)
        println("Address ${address.asString()} at index $index")
    }

    @Test
    fun testBalance() {
        val descriptor = Descriptor("wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/0/*)", Network.TESTNET)
        val wallet = Wallet.newNoPersist(descriptor, null, Network.TESTNET)

        assert(wallet.getBalance().total() == 0uL)
    }

    @Test
    fun testSpks() {
        val descriptor = Descriptor("wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/0/*)", Network.TESTNET)
        val wallet = Wallet.newNoPersist(descriptor, null, Network.TESTNET)
        val (index, address, keychain) = wallet.getAddress(AddressIndex.LastUnused)
        //
        val spks: Map<KeychainKind, List<IndexedScript>> = wallet.getAllSpks()
        // println(spks)
    }

    // @Test
    // fun testSyncedBalance() {
    //     val descriptor = Descriptor("wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/0/*)", Network.TESTNET)
    //     val wallet = Wallet.newNoPersist(descriptor, null, Network.TESTNET, WalletType.MEMORY)
    //     val esploraClient = EsploraClient("https://mempool.space/testnet/api")
    //     // val esploraClient = EsploraClient("https://blockstream.info/testnet/api")
    //     val update = esploraClient.scan(wallet, 10uL, 1uL)
    //     wallet.applyUpdate(update)
    //     println("Balance: ${wallet.getBalance().total()}")
    // }
}
