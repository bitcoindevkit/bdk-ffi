package org.bitcoindevkit.bdkjni

import com.sun.jna.*
import com.sun.jna.ptr.PointerByReference

// typedef struct {
//
//    char * name;
//
//    int32_t count;
//
// } Config_t;
@Structure.FieldOrder("name", "count")
class Config_t : Structure() {
    @JvmField
    var name: String? = null
    @JvmField
    var count: NativeLong? = null
}

// typedef struct WalletPtr WalletPtr_t;
class WalletPtr_t : PointerType {
    constructor(): super()
    constructor(pointer: Pointer): super(pointer)
}

interface Lib : Library {

    // void print_string (
    //    char const * string);
    fun print_string(name: String)

    // char * concat_string (
    //    char const * fst,
    //    char const * snd);
    fun concat_string(fst: String, snd: String): String

    // void free_string (
    //    char * string);
    fun free_string(string: String)

    // void print_config (
    //    Config_t const * config);
    fun print_config(config: Config_t)

    // Config_t new_config (
    //    char * name,
    //    int32_t count);
    fun new_config(name: String, count: NativeLong): Config_t

    // void free_config (
    //    Config_t * config);
    fun free_config(config: Config_t)

    // WalletPtr_t * new_wallet (
    //    char const * name,
    //    char const * descriptor,
    //    char const * change_descriptor);
    fun new_wallet(name: String, descriptor: String, changeDescriptor: String?): WalletPtr_t

    // void sync_wallet (
    //    WalletPtr_t * const * wallet);
    //fun sync_wallet(wallet: WalletPtr_t) 
    fun sync_wallet(wallet: WalletPtr_t)

    // char * new_address (
    //    WalletPtr_t * const * wallet);
    fun new_address(wallet: WalletPtr_t): String

    // void free_wallet (
    //    WalletPtr_t * wallet);
    fun free_wallet(wallet: WalletPtr_t)
}
