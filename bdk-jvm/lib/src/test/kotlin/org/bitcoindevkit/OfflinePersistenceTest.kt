package org.bitcoindevkit

import kotlin.test.Test
import kotlin.test.assertEquals
import org.rustbitcoin.bitcoin.Network

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
}
