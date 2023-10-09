package org.bitcoindevkit

import org.junit.Assert.*
import org.junit.Test
import org.slf4j.Logger
import org.slf4j.LoggerFactory
import java.io.File
import java.nio.file.Files

/**
 * Library test, which will execute on linux host.
 */
class JvmLibTest {

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
           val descriptor1 = Descriptor("wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/0/*)", Network.TESTNET)
           // val mnemonic = Mnemonic(WordCount.WORDS12)
           // val descriptorSecretKey = DescriptorSecretKey(Network.TESTNET, mnemonic, null)
           // val descriptor = Descriptor.newBip86(descriptorSecretKey, KeychainKind.EXTERNAL, Network.TESTNET)
           val wallet = Wallet(descriptor1, null, Network.TESTNET, WalletType.MEMORY)
           val (index, address, keychain)  = wallet.getAddress(AddressIndex.LastUnused)
           println("Address ${address.asString()} at index $index")
        }
}
