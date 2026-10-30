package org.bitcoindevkit

import androidx.test.ext.junit.runners.AndroidJUnit4
import kotlinx.coroutines.delay
import kotlinx.coroutines.runBlocking
import org.junit.runner.RunWith
import org.kotlinbitcointools.regtesttoolbox.regenv.RegEnv
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFalse
import kotlin.test.assertIs
import kotlin.test.assertNotNull
import kotlin.test.assertTrue
import kotlin.time.Duration.Companion.milliseconds

@RunWith(AndroidJUnit4::class)
class FinalizePsbtTest {
    @Test
    fun tryFinalizePsbtReportsInputOutcomes() {
        val wallet = Wallet(
            descriptor = BIP84_DESCRIPTOR,
            changeDescriptor = BIP84_CHANGE_DESCRIPTOR,
            network = Network.REGTEST,
            persister = Persister.newInMemory()
        )
        // Two unknown inputs: the second already has a final witness.
        val psbt = Psbt("cHNidP8BAFwCAAAAAgAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAD/////AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABAAAAAP////8AAAAAAAAAAQgDAQEBAA==")
        val outcome = wallet.tryFinalizePsbt(psbt)

        assertFalse(outcome.isFinalized)
        assertEquals(2, outcome.outcomes.size)
        assertEquals(FinalizeInputOutcome.MissingDescriptor, outcome.outcomes[0uL])
        assertEquals(FinalizeInputOutcome.AlreadyFinalized, outcome.outcomes[1uL])
        assertFalse(outcome.outcomes.getValue(0uL).isFinalized())
        assertTrue(outcome.outcomes.getValue(1uL).isFinalized())
    }

    @Test
    fun tryFinalizePsbtTransitionsFromUnsignedToFinalized() {
        runBlocking {
            val regtestEnv = RegEnv.connectTo(
                host = "10.0.2.2",
                walletName = "faucet",
                username = "regtest",
                password = "password",
            )
            // Do not fund the shared descriptors: other tests expect them to have no history.
            val secretKey = DescriptorSecretKey(
                NetworkKind.TEST,
                Mnemonic(WordCount.WORDS12),
                null
            )
            val wallet = Wallet(
                descriptor = Descriptor.newBip84(secretKey, KeychainKind.EXTERNAL, NetworkKind.TEST),
                changeDescriptor = Descriptor.newBip84(secretKey, KeychainKind.INTERNAL, NetworkKind.TEST),
                network = Network.REGTEST,
                persister = Persister.newInMemory()
            )

            val address = wallet.revealNextAddress(KeychainKind.EXTERNAL).address
            val txid = Txid.fromString(regtestEnv.send(address.toString(), 0.001, 2.0))
            regtestEnv.mine(1)
            val esploraClient = EsploraClient(ESPLORA_REGTEST_URL)
            for (i in 0..99) {
                if (esploraClient.getTx(txid) != null) break
                delay(200.milliseconds)
            }
            assertNotNull(esploraClient.getTx(txid), "Funding transaction was not indexed in time")
            wallet.applyUpdate(esploraClient.fullScan(wallet.startFullScan().build(), 10uL, 1uL))

            val recipient = wallet.revealNextAddress(KeychainKind.EXTERNAL).address.scriptPubkey()
            val psbt = TxBuilder()
                .addRecipient(recipient, Amount.fromSat(10_000uL))
                .finish(wallet)

            val unsigned = wallet.tryFinalizePsbt(psbt)
            assertFalse(unsigned.isFinalized)
            assertTrue(unsigned.outcomes.isNotEmpty())
            unsigned.outcomes.values.forEach {
                assertIs<FinalizeInputOutcome.CouldNotSatisfy>(it)
                assertFalse(it.isFinalized())
            }

            wallet.sign(psbt, SignOptions(
                trustWitnessUtxo = false,
                assumeHeight = null,
                allowAllSighashes = false,
                tryFinalize = false,
                signWithTapInternalKey = true,
                allowGrinding = true,
            ))
            val signed = wallet.tryFinalizePsbt(psbt)
            assertTrue(signed.isFinalized)
            assertEquals(unsigned.outcomes.keys, signed.outcomes.keys)
            signed.outcomes.values.forEach {
                assertEquals(FinalizeInputOutcome.Finalized, it)
                assertTrue(it.isFinalized())
            }

            val again = wallet.tryFinalizePsbt(psbt)
            assertTrue(again.isFinalized)
            assertEquals(signed.outcomes.keys, again.outcomes.keys)
            again.outcomes.values.forEach {
                assertEquals(FinalizeInputOutcome.AlreadyFinalized, it)
                assertTrue(it.isFinalized())
            }
        }
    }
}
