package org.bitcoindevkit

import android.database.sqlite.SQLiteDatabase
import androidx.test.platform.app.InstrumentationRegistry
import java.io.File
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertTrue

const val MNEMONIC_ALL = "all all all all all all all all all all all all"

class DatabaseVersionCompatTest {
    private val context = InstrumentationRegistry.getInstrumentation().targetContext

    private fun resolveAsset(dbFileName: String, subDir: String? = null): String {
        val context = InstrumentationRegistry.getInstrumentation().targetContext
        val assetPath = if (subDir != null) "$subDir/$dbFileName" else dbFileName
        val destFile = File(context.getDatabasePath(dbFileName).path)
        context.assets.open(assetPath).use { input ->
            destFile.outputStream().use { output ->
                input.copyTo(output)
            }
        }
        return destFile.absolutePath
    }

    val descriptorSecretKey: DescriptorSecretKey = DescriptorSecretKey(
        NetworkKind.TEST,
        Mnemonic.fromString(MNEMONIC_ALL),
        null
    )
    val descriptor: Descriptor = Descriptor.newBip84(descriptorSecretKey, KeychainKind.EXTERNAL, NetworkKind.TEST)
    val changeDescriptor: Descriptor = Descriptor.newBip84(descriptorSecretKey, KeychainKind.INTERNAL, NetworkKind.TEST)

    // You can take a v1 Wallet database and use it to create a v3 Wallet.
    @Test
    fun loadV1WalletDBIntoV3Wallet() {
        val dbV1 = Persister.newSqlite(resolveAsset("db_v1.sqlite3", "old_databases"))

        val wallet = Wallet.load(
            descriptor = descriptor,
            changeDescriptor = changeDescriptor,
            persister = dbV1,
        )

        val addressInfo: AddressInfo = wallet.revealNextAddress(KeychainKind.EXTERNAL)
        val changeAddressInfo: AddressInfo = wallet.revealNextAddress(KeychainKind.INTERNAL)
        println("Address info: $addressInfo")
        println("Change address info: $changeAddressInfo")

        assertEquals(addressInfo.index, 7u)
        assertEquals(changeAddressInfo.index, 1u)
    }

    // You can take a v2 Wallet database and use it to create a v3 Wallet.
    @Test
    fun loadV2WalletDBIntoV3Wallet() {
        val dbV2 = Persister.newSqlite(resolveAsset("db_v2.sqlite3", "old_databases"))

        val wallet = Wallet.load(
            descriptor = descriptor,
            changeDescriptor = changeDescriptor,
            persister = dbV2,
        )

        val addressInfo: AddressInfo = wallet.revealNextAddress(KeychainKind.EXTERNAL)
        val changeAddressInfo: AddressInfo = wallet.revealNextAddress(KeychainKind.INTERNAL)
        // println("Address info: $addressInfo")
        // println("Change address info: $changeAddressInfo")

        assertEquals(addressInfo.index, 7u)
        assertEquals(changeAddressInfo.index, 1u)
    }

    // You can take a v1 Wallet database and use it to create a v3 Wallet, and the database migrates gracefully.
    // The v3 database adds the bdk_descriptor_derived_spks and bdk_wallet_locked_outpoints tables.
    @Test
    fun v3WalletWillAddRequiredFieldsToV1DB() {
        val dbPath = resolveAsset("db_v1.sqlite3", "old_databases")
        val oldV1DB = Persister.newSqlite(dbPath)

        val tablesBefore: List<String> = SQLiteDatabase.openDatabase(dbPath, null, SQLiteDatabase.OPEN_READONLY).use { db ->
            db.rawQuery("SELECT name FROM sqlite_master WHERE type='table'", null).use { cursor ->
                generateSequence { if (cursor.moveToNext()) cursor.getString(0) else null }.toList()
            }
        }
        // println("V1 Database Tables: $tablesBefore")
        // V1 Database Tables: [bdk_schemas, bdk_wallet, bdk_blocks, bdk_txs, bdk_txouts, bdk_anchors, bdk_descriptor_last_revealed]

        assertTrue(!tablesBefore.contains("bdk_descriptor_derived_spks"))
        assertTrue(!tablesBefore.contains("bdk_wallet_locked_outpoints"))

        val wallet1 = Wallet.load(
            descriptor = descriptor,
            changeDescriptor = changeDescriptor,
            persister = oldV1DB,
        )

        val addressInfoWallet1: AddressInfo = wallet1.revealNextAddress(KeychainKind.EXTERNAL)
        val changeAddressInfoWallet1: AddressInfo = wallet1.revealNextAddress(KeychainKind.INTERNAL)

        assertEquals(addressInfoWallet1.index, 7u)
        assertEquals(changeAddressInfoWallet1.index, 1u)

        wallet1.persist(oldV1DB)

        val tables: List<String> = SQLiteDatabase.openDatabase(dbPath, null, SQLiteDatabase.OPEN_READONLY).use { db ->
            db.rawQuery("SELECT name FROM sqlite_master WHERE type='table'", null).use { cursor ->
                generateSequence { if (cursor.moveToNext()) cursor.getString(0) else null }.toList()
            }
        }

        // println("V3 Database Tables: $tables")
        // V3 Database Tables: [bdk_schemas, bdk_wallet, bdk_blocks, bdk_txs, bdk_txouts, bdk_anchors, bdk_descriptor_last_revealed, bdk_wallet_locked_outpoints, bdk_descriptor_derived_spks]

        assertTrue(tables.contains("bdk_descriptor_derived_spks"))
        assertTrue(tables.contains("bdk_wallet_locked_outpoints"))
    }

    // You can take a v2 Wallet database and use it to create a v3 Wallet, and the database migrates gracefully.
    // The v3 database adds the bdk_wallet_locked_outpoints table.
    @Test
    fun v3WalletWillAddRequiredFieldsToV2DB() {
        val dbPath = resolveAsset("db_v2.sqlite3", "old_databases")
        val oldV1DB = Persister.newSqlite(dbPath)

        val tablesBefore: List<String> = SQLiteDatabase.openDatabase(dbPath, null, SQLiteDatabase.OPEN_READONLY).use { db ->
            db.rawQuery("SELECT name FROM sqlite_master WHERE type='table'", null).use { cursor ->
                generateSequence { if (cursor.moveToNext()) cursor.getString(0) else null }.toList()
            }
        }
        println("V2 Database Tables: $tablesBefore")
        // V2 Database Tables: [bdk_schemas, bdk_wallet, bdk_blocks, bdk_txs, bdk_txouts, bdk_anchors, bdk_descriptor_last_revealed, bdk_descriptor_derived_spks]
        assertTrue(!tablesBefore.contains("bdk_wallet_locked_outpoints"))

        val wallet1 = Wallet.load(
            descriptor = descriptor,
            changeDescriptor = changeDescriptor,
            persister = oldV1DB,
        )

        val addressInfoWallet1: AddressInfo = wallet1.revealNextAddress(KeychainKind.EXTERNAL)
        val changeAddressInfoWallet1: AddressInfo = wallet1.revealNextAddress(KeychainKind.INTERNAL)

        assertEquals(addressInfoWallet1.index, 7u)
        assertEquals(changeAddressInfoWallet1.index, 1u)

        wallet1.persist(oldV1DB)

        val tables: List<String> = SQLiteDatabase.openDatabase(dbPath, null, SQLiteDatabase.OPEN_READONLY).use { db ->
            db.rawQuery("SELECT name FROM sqlite_master WHERE type='table'", null).use { cursor ->
                generateSequence { if (cursor.moveToNext()) cursor.getString(0) else null }.toList()
            }
        }

        println("V3 Database Tables: $tables")
        // V3 Database Tables: [bdk_schemas, bdk_wallet, bdk_blocks, bdk_txs, bdk_txouts, bdk_anchors, bdk_descriptor_last_revealed, bdk_wallet_locked_outpoints, bdk_descriptor_derived_spks]

        assertTrue(tables.contains("bdk_wallet_locked_outpoints"))
    }

    // The v3 wallet can load a v3 database
    @Test
    fun loadV3Wallet() {
        val v3DB = Persister.newSqlite(resolveAsset("db_v3.sqlite3"))

        val wallet = Wallet.load(
            descriptor = descriptor,
            changeDescriptor = changeDescriptor,
            persister = v3DB,
        )

        val addressInfo: AddressInfo = wallet.revealNextAddress(KeychainKind.EXTERNAL)
        val changeAddressInfo: AddressInfo = wallet.revealNextAddress(KeychainKind.INTERNAL)
        println("Address info: $addressInfo")
        println("Change address info: $changeAddressInfo")

        assertEquals(addressInfo.index, 7u)
        assertEquals(changeAddressInfo.index, 1u)
    }

    // You can correctly migrate a v0.32 Wallet into a v3 Wallet
    @Test
    fun migrateToV3From032() {
        val oldDB = Persister.newSqlite(resolveAsset("db_v032.sqlite3", "old_databases"))
        val preV1Keychains: List<PreV1WalletKeychain> = oldDB.getPreV1WalletKeychains()

        val externalPreV1Keychain = preV1Keychains.single { it.keychain == KeychainKind.EXTERNAL }
        val internalPreV1Keychain = preV1Keychains.single { it.keychain == KeychainKind.INTERNAL }

        assertEquals(2, preV1Keychains.size)
        assertEquals(KeychainKind.EXTERNAL, externalPreV1Keychain.keychain)
        assertEquals(KeychainKind.INTERNAL, internalPreV1Keychain.keychain)
        assertEquals(6u, externalPreV1Keychain.lastDerivationIndex)
        assertEquals(0u, internalPreV1Keychain.lastDerivationIndex)
        assertEquals("rn0zejch", externalPreV1Keychain.checksum)
        assertEquals("j82ry8g0", internalPreV1Keychain.checksum)

        val newV3DBFilePath = context.getDatabasePath("new_v3_wallet.sqlite3")
        newV3DBFilePath.parentFile?.mkdirs()
        // This ensures local tests always create a new DB and don't reuse an old one leftover from prior tests
        if (newV3DBFilePath.exists()) newV3DBFilePath.delete()
        val newV3DB = Persister.newSqlite(newV3DBFilePath.absolutePath)

        val wallet = Wallet(
            descriptor = descriptor,
            changeDescriptor = changeDescriptor,
            network = Network.REGTEST,
            persister = newV3DB,
        )

        wallet.revealAddressesTo(KeychainKind.EXTERNAL, externalPreV1Keychain.lastDerivationIndex)
        wallet.revealAddressesTo(KeychainKind.INTERNAL, internalPreV1Keychain.lastDerivationIndex)
        wallet.persist(newV3DB)

        val reloadedWallet = Wallet.load(
            descriptor = descriptor,
            changeDescriptor = changeDescriptor,
            persister = newV3DB,
        )

        val addressInfo: AddressInfo = reloadedWallet.revealNextAddress(KeychainKind.EXTERNAL)
        val changeAddressInfo: AddressInfo = reloadedWallet.revealNextAddress(KeychainKind.INTERNAL)

        assertEquals(addressInfo.index, 7u)
        assertEquals(changeAddressInfo.index, 1u)
    }
}
