package org.bitcoindevkit.bdk

import com.sun.jna.*

interface Lib : Library {

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

    // void free_string (
    //    char * string);
    fun free_string(string: Pointer?)

    // typedef struct WalletResult WalletResult_t;
    class WalletResult_t : PointerType {
        constructor() : super()
        constructor(pointer: Pointer) : super(pointer)
    }

    // WalletResult_t * new_wallet_result (
    //    char const * name,
    //    char const * descriptor,
    //    char const * change_descriptor);
    fun new_wallet_result(
        name: String,
        descriptor: String,
        changeDescriptor: String?
    ): WalletResult_t
    
    // char * get_wallet_err (
    //    WalletResult_t const * wallet_result);
    // TODO

    // VoidResult_t * sync_wallet (
    //    WalletResult_t const * wallet_result);
    fun sync_wallet(wallet_result: WalletResult_t): VoidResult_t

    // StringResult_t * new_address (
    //    WalletResult_t const * wallet_result);
    fun new_address(wallet_result: WalletResult_t): StringResult_t

    // void free_wallet_result (
    //    WalletResult_t * wallet_result);
    fun free_wallet_result(wallet_result: WalletResult_t)
}
