package org.bitcoindevkit.bdk.wallet

import com.sun.jna.Pointer
import org.bitcoindevkit.bdk.LibJna
import org.bitcoindevkit.bdk.types.Result

class WalletResult constructor(walletResultPtr: LibJna.FfiResult_OpaqueWallet_t.ByValue) :
    Result<LibJna.FfiResult_OpaqueWallet_t.ByValue, LibJna.OpaqueWallet_t>(walletResultPtr) {

    override fun getOkValue(pointer: Pointer): LibJna.OpaqueWallet_t {
        return LibJna.OpaqueWallet_t(pointer)
    }

    override fun freeResult(ffiResult: LibJna.FfiResult_OpaqueWallet_t.ByValue) {
        libJna.free_wallet_result(ffiResult)
    }
}