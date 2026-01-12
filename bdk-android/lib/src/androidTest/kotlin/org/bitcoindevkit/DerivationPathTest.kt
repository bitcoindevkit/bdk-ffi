package org.bitcoindevkit

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertTrue
import kotlin.test.assertFalse
import androidx.test.ext.junit.runners.AndroidJUnit4
import org.junit.runner.RunWith
import kotlin.test.assertFailsWith

@RunWith(AndroidJUnit4::class)
class DerivationPathTest {
    @Test
    fun derivationPathFromString() {
        val pathString = "m/84h/0h/0h/0/0"
        val expectedPath = "84'/0'/0'/0/0"
        val derivationPath = DerivationPath(pathString)
        assertEquals(
            expected = expectedPath,
            actual = derivationPath.toString()
        )
    }

    @Test
    fun derivationPathtoString() {
        val pathString = "m/44h/1h/0h/1/5"
        val expectedPath = "44'/1'/0'/1/5"
        val derivationPath = DerivationPath(pathString)
        assertEquals(
            expected = expectedPath,
            actual = derivationPath.toString()
        )
    }

    @Test
    fun correctlyIdentifiesMaster() {
        val pathString = "m"
        val derivationPath = DerivationPath(pathString)
        assertTrue(derivationPath.isMaster())

        val nonMasterPathString = "m/0h/1/2"
        val nonMasterDerivationPath = DerivationPath(nonMasterPathString)
        assertFalse(nonMasterDerivationPath.isMaster())

        val masterPathString = DerivationPath.master()
        assertTrue(masterPathString.isMaster())
    }

    @Test
    fun invalidDerivationPath(){
        val invalidPathString = "invalid/path/string"
        assertFailsWith<Bip32Exception.InvalidChildNumberFormat> { DerivationPath(invalidPathString) }
    }

    @Test
    fun createChildDerivationPath() {
        val parentPathString = "m/44h/0h"
        val childIndex = 5u
        val expectedNormalChildPath = "44'/0'/5"
        val parentDerivationPath = DerivationPath(parentPathString)
        val childDerivationPath = parentDerivationPath.child(ChildNumber.Normal(childIndex))

        assertEquals(
            expected = expectedNormalChildPath,
            actual = childDerivationPath.toString()
        )

        val expectedHardenedChildPath = "44'/0'/5'"
        val hardenedChildDerivationPath = parentDerivationPath.child(ChildNumber.Hardened(childIndex))

        assertEquals(
            expected = expectedHardenedChildPath,
            actual = hardenedChildDerivationPath.toString()
        )
    }

    @Test
    fun checkInvalidNormalChildNumberFails(){
        val parentPathString = "m/44h/0h"
        val aHardenedchildIndex = 2147483648u
        val parentDerivationPath = DerivationPath(parentPathString)

        assertFailsWith<Bip32Exception.InvalidChildNumber> { parentDerivationPath.child(ChildNumber.Normal(aHardenedchildIndex)) }
    }

    @Test
    fun checkInvalidHardenedChildNumberFails(){
        val parentPathString = "m/44h/0h"
        val alreadyHardenedIndex = 2147483649u
        val parentDerivationPath = DerivationPath(parentPathString)

        assertFailsWith<Bip32Exception.InvalidChildNumber> { parentDerivationPath.child(ChildNumber.Hardened(alreadyHardenedIndex)) }
    }

    @Test
    fun extendDerivationPath(){
        val basePathString = "m/84h/0h"
        val extensions = "0h/1/2"

        val expectedExtendedPath = "84'/0'/0'/1/2"

        val baseDerivationPath = DerivationPath(basePathString)
        val extensionDerivationPath = DerivationPath(extensions)

        val extendedDerivationPath = baseDerivationPath.extend(extensionDerivationPath)
        assertEquals(
            expected = expectedExtendedPath,
            actual = extendedDerivationPath.toString()
        )
    }

    @Test
    fun conversionToList() {
        val pathString = "m/49h/1h/0h/0/10"
        val derivationPathFromString = DerivationPath(pathString)

        val derivationList = derivationPathFromString.toU32Vec()

        // BIP32 standard (0x80000000 or 2147483648) converted to decimal and added to each hardened index:

        // 49h = 49 + 2147483648 = 2147483697
        // 1h  = 1  + 2147483648 = 2147483649
        // 0h  = 0  + 2147483648 = 2147483648
        val expectedListString = "[2147483697, 2147483649, 2147483648, 0, 10]"

        assertEquals(
            expected = expectedListString,
            actual = derivationList.toString()
        )
    }
}