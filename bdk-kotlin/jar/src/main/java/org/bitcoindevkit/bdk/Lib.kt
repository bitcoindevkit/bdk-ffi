package org.bitcoindevkit.bdk

import com.sun.jna.*

interface Lib : Library {

    // typedef struct WalletPtr WalletPtr_t;
    class WalletPtr_t : PointerType {
        constructor(): super()
        constructor(pointer: Pointer): super(pointer)
    }

    // void free_string (
    //    char * string);
    fun free_string(string: Pointer)

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
    fun new_address(wallet: WalletPtr_t): Pointer

    // void free_wallet (
    //    WalletPtr_t * wallet);
    fun free_wallet(wallet: WalletPtr_t)
}
