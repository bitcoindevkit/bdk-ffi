package org.bitcoindevkit

import kotlin.test.Test
import kotlin.test.assertEquals
import org.bitcoindevkit.bitcoinffi.Network

class OfflinePersistenceTest {
    private val persistenceFilePath = run {
        val currentDirectory = System.getProperty("user.dir")
        val dbFileName = "pre_existing_wallet_persistence_test.sqlite"
        "$currentDirectory/src/test/resources/$dbFileName"
    }
    private val descriptor: Descriptor = Descriptor(
        "wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/0/*)",
        Network.SIGNET
    )
    private val changeDescriptor: Descriptor = Descriptor(
        "wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/1/*)",
        Network.SIGNET
    )

    @Test
    fun testPersistence() {
        val connection = Connection(persistenceFilePath)

        val wallet: Wallet = Wallet.load(
            descriptor,
            changeDescriptor,
            connection
        )
        val addressInfo: AddressInfo = wallet.revealNextAddress(KeychainKind.EXTERNAL)
        println("Address: $addressInfo")

        assertEquals(
            expected = 7u,
            actual = addressInfo.index,
        )
        assertEquals(
            expected = "tb1qan3lldunh37ma6c0afeywgjyjgnyc8uz975zl2",
            actual = addressInfo.address.toString(),
        )
    }

    @Test
    fun testPersistenceWithDescriptor() {
        val connection = Connection(persistenceFilePath)

        val descriptorPub = Descriptor(
            "wpkh([9122d9e0/84'/1'/0']tpubDCYVtmaSaDzTxcgvoP5AHZNbZKZzrvoNH9KARep88vESc6MxRqAp4LmePc2eeGX6XUxBcdhAmkthWTDqygPz2wLAyHWisD299Lkdrj5egY6/0/*)#zpaanzgu",
            Network.SIGNET
        )
        val changeDescriptorPub = Descriptor(
            "wpkh([9122d9e0/84'/1'/0']tpubDCYVtmaSaDzTxcgvoP5AHZNbZKZzrvoNH9KARep88vESc6MxRqAp4LmePc2eeGX6XUxBcdhAmkthWTDqygPz2wLAyHWisD299Lkdrj5egY6/1/*)#n4cuwhcy",
            Network.SIGNET
        )

        val wallet: Wallet = Wallet.load(
            descriptorPub,
            changeDescriptorPub,
            connection
        )
        val addressInfo: AddressInfo = wallet.revealNextAddress(KeychainKind.EXTERNAL)
        println("Address: $addressInfo")

        assertEquals(
            expected = 7u,
            actual = addressInfo.index,
        )
        assertEquals(
            expected = "tb1qan3lldunh37ma6c0afeywgjyjgnyc8uz975zl2",
            actual = addressInfo.address.toString(),
        )
    }
}
