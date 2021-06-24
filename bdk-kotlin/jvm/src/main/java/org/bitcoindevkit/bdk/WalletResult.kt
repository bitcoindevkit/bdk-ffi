package org.bitcoindevkit.bdk

import com.sun.jna.Pointer
import org.slf4j.Logger
import org.slf4j.LoggerFactory

class WalletResult internal constructor(walletResultT: LibJna.WalletResult_t) :
    ResultBase<LibJna.WalletResult_t, LibJna.WalletRef_t>(walletResultT) {

    override val log: Logger = LoggerFactory.getLogger(WalletResult::class.java)

    override fun err(): Pointer? {
        return libJna.get_wallet_err(resultT)
    }

    override fun ok(): LibJna.WalletRef_t {
        return libJna.get_wallet_ok(resultT)!!
    }

    override fun free(pointer: LibJna.WalletResult_t) {
        libJna.free_wallet_result(resultT)
    }
}