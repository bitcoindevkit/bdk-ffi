package org.bitcoindevkit

import androidx.test.ext.junit.runners.AndroidJUnit4
import org.junit.runner.RunWith
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFailsWith

@RunWith(AndroidJUnit4::class)
class ErrorsTest {
    @Test
    fun bip39ErrorDisplaysBadWordCount() {
        val thirteenWordMnemonic = "awesome awesome awesome awesome awesome awesome awesome awesome awesome awesome awesome awesome awesome"

        // Building a Mnemonic fails with BadWordCount exception
        val exception = assertFailsWith<Bip39Exception.BadWordCount> {
            Mnemonic.fromString(thirteenWordMnemonic)
        }

        // The toString() method on the exception is the Display trait exported from Rust
        assertEquals(
            expected = "the word count 13 is not supported",
            actual = exception.toString()
        )

        // The exception contains a field `wordCount` correctly populated
        assertEquals(
            expected = 13uL,
            actual = exception.wordCount
        )

        // The `message` field on the exception is the concatenation of all fields on the type
        assertEquals(
            expected = "wordCount=13",
            actual = exception.message
        )
    }

    @Test
    fun bip32ErrorDisplaysInvalidChildNumberFormat() {
        // A derivation path cannot contain words and fails with an InvalidChildNumberFormat exception
        val exception = assertFailsWith<Bip32Exception.InvalidChildNumberFormat> {
            DerivationPath("invalid/path/string")
        }

        // The toString() method on the exception is the Display trait exported from Rust
        assertEquals(
            expected = "invalid format for child number",
            actual = exception.toString()
        )

        // The `message` field on the exception is the concatenation of all fields on the type (in this case none)
        assertEquals(
            expected = "",
            actual = exception.message
        )
    }
}
