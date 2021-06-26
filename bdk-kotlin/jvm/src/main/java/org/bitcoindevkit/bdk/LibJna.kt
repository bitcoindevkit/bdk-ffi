package org.bitcoindevkit.bdk

import com.sun.jna.Library
import com.sun.jna.Pointer
import com.sun.jna.PointerType

interface LibJna : Library {

    // typedef struct VoidResult VoidResult_t;
    class VoidResult_t : PointerType {
        constructor() : super()
        constructor(pointer: Pointer) : super(pointer)
    }

    // char * get_void_err (
    //    VoidResult_t const * void_result);
    fun get_void_err(void_result: VoidResult_t): Pointer?

    // void free_void_result (
    //    VoidResult_t * void_result);
    fun free_void_result(void_result: VoidResult_t)

    // typedef struct StringResult StringResult_t;
    class StringResult_t : PointerType {
        constructor() : super()
        constructor(pointer: Pointer) : super(pointer)
    }

    // char * get_string_ok (
    //    StringResult_t const * string_result);
    fun get_string_ok(string_result: StringResult_t): Pointer?

    // char * get_string_err (
    //    StringResult_t const * string_result);
    fun get_string_err(string_result: StringResult_t): Pointer?

    // void free_string_result (
    //    StringResult_t * string_result);
    fun free_string_result(string_result: StringResult_t)

    // typedef struct WalletRef WalletRef_t;
    class WalletRef_t : PointerType {
        constructor() : super()
        constructor(pointer: Pointer) : super(pointer)
    }

    // void free_wallet_ref (
    //    WalletRef_t * wallet_ref);
    fun free_wallet_ref(wallet_ref: WalletRef_t)

    // typedef struct BlockchainConfig BlockchainConfig_t;
    class BlockchainConfig_t : PointerType {
        constructor() : super()
        constructor(pointer: Pointer) : super(pointer)
    }

    // typedef struct DatabaseConfig DatabaseConfig_t;
    class DatabaseConfig_t : PointerType {
        constructor() : super()
        constructor(pointer: Pointer) : super(pointer)
    }

    // typedef struct WalletResult WalletResult_t;
    class WalletResult_t : PointerType {
        constructor() : super()
        constructor(pointer: Pointer) : super(pointer)
    }

    // WalletResult_t * new_wallet_result (
    //    char const * descriptor,
    //    char const * change_descriptor,
    //    BlockchainConfig_t const * blockchain_config,
    //    DatabaseConfig_t const * database_config);
    fun new_wallet_result(
        descriptor: String,
        changeDescriptor: String?,
        blockchainConfig: BlockchainConfig_t,
        databaseConfig: DatabaseConfig_t,
    ): WalletResult_t

    // char * get_wallet_err (
    //    WalletResult_t const * wallet_result);
    fun get_wallet_err(wallet_result: WalletResult_t): Pointer?

    // WalletRef_t * get_wallet_ok (
    //    WalletResult_t const * wallet_result);
    fun get_wallet_ok(wallet_result: WalletResult_t): WalletRef_t?

    // VoidResult_t * sync_wallet (
    //    WalletRef_t const * wallet_ref);
    fun sync_wallet(wallet_ref: Pointer): VoidResult_t

    // StringResult_t * new_address (
    //    WalletRef_t const * wallet_ref);
    fun new_address(wallet_ref: Pointer): StringResult_t

    // void free_wallet_result (
    //    WalletResult_t * wallet_result);
    fun free_wallet_result(wallet_result: WalletResult_t)

    // void free_string (
    //    char * string);
    fun free_string(string: Pointer?)

    // BlockchainConfig_t * new_electrum_config (
    //    char const * url,
    //    char const * socks5,
    //    int16_t retry,
    //    int16_t timeout);
    fun new_electrum_config(
        url: String,
        socks5: String?,
        retry: Short,
        timeout: Short
    ): BlockchainConfig_t

    // void free_blockchain_config (
    //    BlockchainConfig_t * blockchain_config);
    fun free_blockchain_config( blockchain_config: BlockchainConfig_t) 

    // DatabaseConfig_t * new_memory_config (void);
    fun new_memory_config(): DatabaseConfig_t
    
    // DatabaseConfig_t * new_sled_config (
    //    char const * path,
    //    char const * tree_name);
    fun new_sled_config(path: String, tree_name: String): DatabaseConfig_t

    // void free_database_config (
    //    DatabaseConfig_t * database_config);
    fun free_database_config( database_config: DatabaseConfig_t)
}
