package org.bitcoindevkit.bdk

import com.sun.jna.*

interface LibJna : Library {

    // typedef struct {
    //
    //    char * ok;
    //
    //    char * err;
    //
    //} FfiResult_char_ptr_t;
    open class FfiResult_char_ptr_t : Structure() {
        class ByValue : FfiResult_char_ptr_t(), Structure.ByValue
        class ByReference : FfiResult_char_ptr_t(), Structure.ByReference

        @JvmField
        var ok: String = ""

        @JvmField
        var err: String = ""

        override fun getFieldOrder() = listOf("ok", "err")
    }

    // void free_string_result (
    //    FfiResult_char_ptr_t string_result);
    fun free_string_result(string_result: FfiResult_char_ptr_t.ByValue)

    // typedef struct {
    //
    //    int32_t ok;
    //
    //    char * err;
    //
    //} FfiResult_int32_t;
    open class FfiResult_int32_t : Structure() {
        class ByValue : FfiResult_int32_t(), Structure.ByValue
        class ByReference : FfiResult_int32_t(), Structure.ByReference

        @JvmField
        var ok: Int = 0

        @JvmField
        var err: String = ""
        
        override fun getFieldOrder() = listOf("ok", "err")
    }

    // void free_int_result (
    //    FfiResult_int32_t int_result);
    fun free_int_result(void_result: FfiResult_int32_t.ByValue)

    // void free_string (
    //    char * string);
    fun free_string(string: Pointer?)

    // typedef struct BlockchainConfig BlockchainConfig_t;
    class BlockchainConfig_t : PointerType {
        constructor() : super()
        constructor(pointer: Pointer) : super(pointer)
    }

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
    fun free_blockchain_config(blockchain_config: BlockchainConfig_t)

    // typedef struct DatabaseConfig DatabaseConfig_t;
    class DatabaseConfig_t : PointerType {
        constructor() : super()
        constructor(pointer: Pointer) : super(pointer)
    }

    // typedef struct OpaqueWallet OpaqueWallet_t;
    class OpaqueWallet_t : PointerType {
        constructor() : super()
        constructor(pointer: Pointer) : super(pointer)
    }

    // typedef struct {
    //
    //    OpaqueWallet_t * ok;
    //
    //    char * err;
    //
    // } FfiResult_OpaqueWallet_ptr_t;
    open class FfiResult_OpaqueWallet_ptr_t : Structure() {
        class ByValue : FfiResult_OpaqueWallet_ptr_t(), Structure.ByValue
        class ByReference : FfiResult_OpaqueWallet_ptr_t(), Structure.ByReference

        @JvmField
        var ok: OpaqueWallet_t = OpaqueWallet_t()

        @JvmField
        var err: String = ""
        
        override fun getFieldOrder() = listOf("ok", "err")
    }

    // FfiResult_OpaqueWallet_ptr_t new_wallet_result (
    //    char const * descriptor,
    //    char const * change_descriptor,
    //    BlockchainConfig_t const * blockchain_config,
    //    DatabaseConfig_t const * database_config);
    fun new_wallet_result(
        descriptor: String,
        changeDescriptor: String?,
        blockchainConfig: BlockchainConfig_t,
        databaseConfig: DatabaseConfig_t,
    ): FfiResult_OpaqueWallet_ptr_t.ByValue

    // void free_wallet_result (
    //    FfiResult_OpaqueWallet_ptr_t wallet_result);
    fun free_wallet_result(wallet_result: FfiResult_OpaqueWallet_ptr_t.ByValue)

    // typedef struct {
    //
    //    char * txid;
    //
    //    uint32_t vout;
    //
    // } OutPoint_t;
    open class OutPoint_t : Structure() {
        class ByValue : OutPoint_t(), Structure.ByValue

        @JvmField
        var txid: String? = null

        @JvmField
        var vout: Int? = null

        override fun getFieldOrder() = listOf("txid", "vout")
    }

    // typedef struct {
    //
    //    uint64_t value;
    //
    //    char * script_pubkey;
    //
    // } TxOut_t;
    open class TxOut_t : Structure() {
        class ByValue : TxOut_t(), Structure.ByValue

        @JvmField
        var value: Long? = null

        @JvmField
        var script_pubkey: String? = null

        override fun getFieldOrder() = listOf("value", "script_pubkey")
    }

    // typedef struct {
    //
    //    OutPoint_t outpoint;
    //
    //    TxOut_t txout;
    //
    //    uint16_t keychain;
    //
    // } LocalUtxo_t;
    open class LocalUtxo_t : Structure() {
        
        class ByValue : LocalUtxo_t(), Structure.ByValue
        class ByReference : LocalUtxo_t(), Structure.ByReference

        @JvmField
        var outpoint: OutPoint_t? = null

        @JvmField
        var txout: TxOut_t? = null

        @JvmField
        var keychain: Short? = null

        override fun getFieldOrder() = listOf("outpoint", "txout", "keychain")
    }

    // typedef struct {
    //
    //    LocalUtxo_t * ptr;
    //
    //    size_t len;
    //
    //    size_t cap;
    //
    // } Vec_LocalUtxo_t;
    open class Vec_LocalUtxo_t : Structure() {
        
        class ByReference : Vec_LocalUtxo_t(), Structure.ByReference
        class ByValue : Vec_LocalUtxo_t(), Structure.ByValue
        
        @JvmField
        var ptr: LocalUtxo_t.ByReference? = null

        @JvmField
        var len: NativeLong? = null

        @JvmField
        var cap: NativeLong? = null

        override fun getFieldOrder() = listOf("ptr", "len", "cap")
    }

    // typedef struct {
    //
    //    Vec_LocalUtxo_t ok;
    //
    //    char * err;
    //
    // } FfiResult_Vec_LocalUtxo_t;
    open class FfiResultVec_LocalUtxo_t : Structure() {
        
        class ByValue : FfiResultVec_LocalUtxo_t(), Structure.ByValue
        class ByReference : FfiResultVec_LocalUtxo_t(), Structure.ByReference

        @JvmField
        var ok: Vec_LocalUtxo_t = Vec_LocalUtxo_t()

        @JvmField
        var err: String = ""

        override fun getFieldOrder() = listOf("ok", "err")
    }

    // void free_unspent_result (
    //    FfiResult_Vec_LocalUtxo_t unspent_result);
    fun free_unspent_result(unspent_result: FfiResultVec_LocalUtxo_t.ByValue)

    // FfiResult_int32_t sync_wallet (
    //    OpaqueWallet_t const * opaque_wallet);
    fun sync_wallet(opaque_wallet: OpaqueWallet_t): FfiResult_int32_t.ByValue

    // FfiResult_char_ptr_t new_address (
    //    OpaqueWallet_t const * opaque_wallet);
    fun new_address(opaque_wallet: OpaqueWallet_t): FfiResult_char_ptr_t.ByValue

    // FfiResult_Vec_LocalUtxo_t list_unspent (
    //    OpaqueWallet_t const * opaque_wallet);
    fun list_unspent(opaque_wallet: OpaqueWallet_t): FfiResultVec_LocalUtxo_t.ByValue

    // DatabaseConfig_t * new_memory_config (void);
    fun new_memory_config(): DatabaseConfig_t

    // DatabaseConfig_t * new_sled_config (
    //    char const * path,
    //    char const * tree_name);
    fun new_sled_config(path: String, tree_name: String): DatabaseConfig_t

    // void free_database_config (
    //    DatabaseConfig_t * database_config);
    fun free_database_config(database_config: DatabaseConfig_t)
}
