package org.bitcoindevkit

import androidx.test.ext.junit.runners.AndroidJUnit4
import org.junit.runner.RunWith
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFalse

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
    }
}
