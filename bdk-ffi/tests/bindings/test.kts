/*
 * This is a basic test kotlin program that does nothing but confirm that the kotlin bindings compile
 * and that a program that depends on them will run.
 */

import org.bitcoindevkit.bitcoin.Network
import org.bitcoindevkit.BlockId

// A type from bitcoin-ffi
val network = Network.TESTNET

// A type from bdk-ffi
val blockId = BlockId(0uL, "abcd")
