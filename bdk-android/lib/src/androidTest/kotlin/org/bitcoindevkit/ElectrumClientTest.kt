package org.bitcoindevkit

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertTrue
import androidx.test.ext.junit.runners.AndroidJUnit4
import kotlinx.coroutines.delay
import kotlinx.coroutines.runBlocking
import org.junit.runner.RunWith
import org.kotlinbitcointools.regtesttoolbox.regenv.RegEnv
import kotlin.time.Duration.Companion.milliseconds

@RunWith(AndroidJUnit4::class)
class ElectrumClientTest {
    private val conn: Persister = Persister.newInMemory()

    @Test
    fun populateAnchorCacheFromWalletTransactions() {
        runBlocking {
            val regtestEnv = RegEnv.connectTo(
                host = "10.0.2.2",
                walletName = "faucet",
                username = "regtest",
                password = "password",
            )

            // A freshly generated key on every run, so the wallet has no history from earlier runs.
            val secretKey = DescriptorSecretKey(NetworkKind.TEST, Mnemonic(WordCount.WORDS12), null)
            val wallet: Wallet = Wallet.createSingle(
                descriptor = Descriptor.newBip86(secretKey, KeychainKind.EXTERNAL, NetworkKind.TEST),
                network = Network.REGTEST,
                persister = conn
            )
            val address = wallet.revealNextAddress(KeychainKind.EXTERNAL).address
            val txid = Txid.fromString(regtestEnv.send(address.toString(), 0.12345678, 2.0))
            regtestEnv.mine(1)

            // Full scan until the Electrum server has indexed our transaction as confirmed.
            val electrumClient = ElectrumClient(ELECTRUM_REGTEST_URL)
            for (i in 0..99) {
                val request = wallet.startFullScan().build()
                wallet.applyUpdate(electrumClient.fullScan(request, 10uL, 10uL, false))
                if (wallet.getTx(txid)?.chainPosition is ChainPosition.Confirmed) break
                delay(200.milliseconds)
            }

            // Build one TxAnchor per directly confirmed wallet transaction. Transactions confirmed
            // only `transitively` (through a confirmed descendant) are skipped, since their
            // ConfirmationBlockTime belongs to the descendant, not to them.
            val anchors: List<TxAnchor> = wallet.transactions().mapNotNull { canonicalTx ->
                val position = canonicalTx.chainPosition
                if (position is ChainPosition.Confirmed && position.transitively == null) {
                    TxAnchor(
                        txid = canonicalTx.transaction.computeTxid(),
                        anchor = position.confirmationBlockTime,
                    )
                } else {
                    null
                }
            }
            assertEquals(1, anchors.size, "The funding transaction should produce one anchor")
            assertEquals(txid, anchors.single().txid)

            // A fresh client seeded with the wallet's transactions and anchors does not need to
            // re-download those transactions or re-fetch their Merkle proofs during the next sync.
            val freshClient = ElectrumClient(ELECTRUM_REGTEST_URL)
            freshClient.populateTxCache(wallet.transactions().map { it.transaction })
            freshClient.populateAnchorCache(anchors)

            val syncRequest = wallet.startSyncWithRevealedSpks().build()
            wallet.applyUpdate(freshClient.sync(syncRequest, 10uL, false))

            assertEquals(12_345_678uL, wallet.balance().confirmed.toSat())
            assertTrue(wallet.getTx(txid)?.chainPosition is ChainPosition.Confirmed)
        }
    }
}
