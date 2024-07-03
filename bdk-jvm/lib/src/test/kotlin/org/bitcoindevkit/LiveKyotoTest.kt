package org.bitcoindevkit
import org.rustbitcoin.bitcoin.Network

import kotlinx.coroutines.runBlocking
import kotlin.test.Test
import kotlin.test.AfterTest
import kotlin.test.assertNotNull

import java.nio.file.Files
import java.nio.file.Paths
import kotlin.io.path.ExperimentalPathApi
import kotlin.io.path.deleteRecursively

class LiveKyotoTest {
    private val descriptor: Descriptor = Descriptor("wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/0/*)", Network.SIGNET)
    private val changeDescriptor: Descriptor = Descriptor("wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/1/*)", Network.SIGNET)
    private val ip: IpAddress = IpAddress.fromIpv4(68u, 47u, 229u, 218u)
    private val peer: Peer = Peer(ip, null, false)
    private val currentPath = Paths.get(".").toAbsolutePath().normalize()
    private val persistenceFilePath = Files.createTempDirectory(currentPath, "tempDirPrefix_")

    @OptIn(ExperimentalPathApi::class)
    @AfterTest
    fun cleanup() {
        persistenceFilePath.deleteRecursively()
    }

    @Test
    fun testKyoto() {
        val conn: Connection = Connection.newInMemory()
        val wallet: Wallet = Wallet(descriptor, changeDescriptor, Network.SIGNET, conn)
        val peers = listOf(peer)
        runBlocking {
            val nodePair = LightClientBuilder()
                .peers(peers)
                .connections(1u)
                .dataDir(persistenceFilePath.toString())
                .build(wallet)
            val client = nodePair.client
            val node = nodePair.node
            println("Node running")
            node.run()
            val updateOpt: Update? = client.update(null)
            val update = assertNotNull(updateOpt)
            wallet.applyUpdate(update)
            assert(wallet.balance().total.toSat() > 0uL) {
                "Wallet balance must be greater than 0! Please send funds to ${wallet.revealNextAddress(KeychainKind.EXTERNAL).address} and try again."
            }
            client.shutdown()
            println("Test completed successfully")
        }
    }
}
