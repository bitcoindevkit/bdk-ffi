package org.bitcoindevkit.bdk

import com.sun.jna.*

interface LibJna : Library {

    // typedef struct {
    //
    //    char * ok;
    //
    //    FfiError_t err;
    //
    //} FfiResult_char_ptr_t;
    open class FfiResult_char_ptr_t : Structure() {
        class ByValue : FfiResult_char_ptr_t(), Structure.ByValue
        class ByReference : FfiResult_char_ptr_t(), Structure.ByReference

        @JvmField
        var ok: String? = null

        @JvmField
        var err: Short? = null

        override fun getFieldOrder() = listOf("ok", "err")
    }

    // void free_string_result (
    //    FfiResult_char_ptr_t string_result);
    fun free_string_result(string_result: FfiResult_char_ptr_t.ByValue)

    // typedef struct {
    //
    //    FfiError_t err;
    //
    //} FfiResultVoid_t;
    open class FfiResultVoid_t : Structure() {
        class ByValue : FfiResultVoid_t(), Structure.ByValue
        class ByReference : FfiResultVoid_t(), Structure.ByReference

        @JvmField
        var err: Short? = null
        
        override fun getFieldOrder() = listOf("err")
    }

    // void free_void_result (
    //    FfiResultVoid_t void_result);
    fun free_void_result(void_result: FfiResultVoid_t.ByValue)

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
    //    int16_t timeout,
    //    size_t stop_gap);
    fun new_electrum_config(
        url: String,
        socks5: String?,
        retry: Short,
        timeout: Short,
        stop_gap: Long,
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
    //    FfiError_t err;
    //
    // } FfiResult_OpaqueWallet_ptr_t;
    open class FfiResult_OpaqueWallet_ptr_t : Structure() {
        class ByValue : FfiResult_OpaqueWallet_ptr_t(), Structure.ByValue
        class ByReference : FfiResult_OpaqueWallet_ptr_t(), Structure.ByReference

        @JvmField
        var ok: OpaqueWallet_t? = null

        @JvmField
        var err: Short? = null
        
        override fun getFieldOrder() = listOf("ok", "err")
    }

    // FfiResult_OpaqueWallet_ptr_t new_wallet_result (
    //    char const * descriptor,
    //    char const * change_descriptor,
    //    char const * network,
    //    BlockchainConfig_t const * blockchain_config,
    //    DatabaseConfig_t const * database_config);
    fun new_wallet_result(
        descriptor: String,
        changeDescriptor: String?,
        network: String,
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
    //    FfiError_t err;
    //
    // } FfiResult_Vec_LocalUtxo_t;
    open class FfiResultVec_LocalUtxo_t : Structure() {
        
        class ByValue : FfiResultVec_LocalUtxo_t(), Structure.ByValue
        class ByReference : FfiResultVec_LocalUtxo_t(), Structure.ByReference

        @JvmField
        var ok: Vec_LocalUtxo_t? = null

        @JvmField
        var err: Short? = null

        override fun getFieldOrder() = listOf("ok", "err")
    }

    // void free_veclocalutxo_result (
    //    FfiResult_Vec_LocalUtxo_t unspent_result);
    fun free_veclocalutxo_result(unspent_result: FfiResultVec_LocalUtxo_t.ByValue)

    // typedef struct {
    //
    //    uint64_t ok;
    //
    //    FfiError_t err;
    //
    // } FfiResult_uint64_t;
    open class FfiResult_uint64_t : Structure() {

        class ByValue : FfiResult_uint64_t(), Structure.ByValue
        class ByReference : FfiResult_uint64_t(), Structure.ByReference
        
        @JvmField
        var ok: Long? = null

        @JvmField
        var err: Short? = null

        override fun getFieldOrder() = listOf("ok", "err")
    }

    // void free_uint64_result (
    //    FfiResult_uint64_t void_result);
    fun free_uint64_result(unspent_result: FfiResult_uint64_t.ByValue)
    
    // FfiResultVoid_t sync_wallet (
    //    OpaqueWallet_t const * opaque_wallet);
    fun sync_wallet(opaque_wallet: OpaqueWallet_t): FfiResultVoid_t.ByValue

    // FfiResult_char_ptr_t new_address (
    //    OpaqueWallet_t const * opaque_wallet);
    fun new_address(opaque_wallet: OpaqueWallet_t): FfiResult_char_ptr_t.ByValue

    // FfiResult_Vec_LocalUtxo_t list_unspent (
    //    OpaqueWallet_t const * opaque_wallet);
    fun list_unspent(opaque_wallet: OpaqueWallet_t): FfiResultVec_LocalUtxo_t.ByValue

    // FfiResult_uint64_t balance (
    //    OpaqueWallet_t const * opaque_wallet);
    fun balance(opaque_wallet: OpaqueWallet_t): FfiResult_uint64_t.ByValue
    
    // DatabaseConfig_t * new_memory_config (void);
    fun new_memory_config(): DatabaseConfig_t

    // DatabaseConfig_t * new_sled_config (
    //    char const * path,
    //    char const * tree_name);
    fun new_sled_config(path: String, tree_name: String): DatabaseConfig_t

    // void free_database_config (
    //    DatabaseConfig_t * database_config);
    fun free_database_config(database_config: DatabaseConfig_t)

    // typedef struct {
    //    uint32_t height;
    //    uint64_t timestamp;
    // } ConfirmationTime_t;
    open class ConfirmationTime_t : Structure() {
        
        class ByValue : ConfirmationTime_t(), Structure.ByValue
        class ByReference : ConfirmationTime_t(), Structure.ByReference
        
        @JvmField
        var height: Int? = null
        
        @JvmField
        var timestamp: Long? = null

        override fun getFieldOrder() = listOf("height", "timestamp")
    }

    // typedef struct {
    //    char * txid;
    //    uint64_t received;
    //    uint64_t sent;
    //    int64_t fee;
    //    bool is_confirmed;
    //    ConfirmationTime_t confirmation_time;
    //    bool verified;
    // } TransactionDetails_t;
    open class TransactionDetails_t : Structure() {

        class ByValue : TransactionDetails_t(), Structure.ByValue
        class ByReference : TransactionDetails_t(), Structure.ByReference

        @JvmField
        var txid: String? = null

        @JvmField
        var received: Long? = null

        @JvmField
        var sent: Long? = null

        @JvmField
        var fee: Long? = null
        
        @JvmField
        var is_confirmed: Boolean? = null

        @JvmField
        var confirmation_time: ConfirmationTime_t? = null

        @JvmField
        var verified: Boolean? = null
        
        override fun getFieldOrder() = listOf("txid", "received", "sent", "fee", "is_confirmed", "confirmation_time", "verified")
    }
    
    // typedef struct {
    //
    //    TransactionDetails_t * ptr;
    //
    //    size_t len;
    //
    //    size_t cap;
    //
    // } Vec_TransactionDetails_t;
    open class Vec_TransactionDetails_t : Structure() {

        class ByReference : Vec_TransactionDetails_t(), Structure.ByReference
        class ByValue : Vec_TransactionDetails_t(), Structure.ByValue

        @JvmField
        var ptr: TransactionDetails_t.ByReference? = null

        @JvmField
        var len: NativeLong? = null

        @JvmField
        var cap: NativeLong? = null

        override fun getFieldOrder() = listOf("ptr", "len", "cap")
    }
    
    // typedef struct {
    //
    //    Vec_TransactionDetails_t ok;
    //
    //    FfiError_t err;
    //
    // } FfiResult_Vec_TransactionDetails_t;
    open class FfiResult_Vec_TransactionDetails_t : Structure() {

        class ByValue : FfiResult_Vec_TransactionDetails_t(), Structure.ByValue
        class ByReference : FfiResult_Vec_TransactionDetails_t(), Structure.ByReference

        @JvmField
        var ok: Vec_TransactionDetails_t? = null

        @JvmField
        var err: Short? = null

        override fun getFieldOrder() = listOf("ok", "err")
    }
    
    // FfiResult_Vec_TransactionDetails_t list_transactions (
    //    OpaqueWallet_t const * opaque_wallet);
    fun list_transactions(opaque_wallet: OpaqueWallet_t): FfiResult_Vec_TransactionDetails_t.ByValue


    // void free_vectxdetails_result (
    //    FfiResult_Vec_TransactionDetails_t txdetails_result);
    fun free_vectxdetails_result(txdetails_result: FfiResult_Vec_TransactionDetails_t.ByValue)
}
