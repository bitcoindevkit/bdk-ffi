/*
 * This is a basic test swift program that does nothing but confirm that the swift bindings compile
 * and that a program that depends on them will run.
 */

import Foundation
import BitcoinDevKit

// A type from the bitcoin-ffi library
let network = Network.testnet

// A type from the bdk-ffi library
let blockId = BlockId(height: 32, hash: "abcd")
