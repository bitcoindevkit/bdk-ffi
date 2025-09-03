package org.bitcoindevkit

// Test networks
const val TEST_EXTENDED_PRIVKEY = "tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B"
const val BIP84_TEST_RECEIVE_PATH = "84h/1h/0h/0"
const val BIP84_TEST_CHANGE_PATH = "84h/1h/0h/1"
const val BIP86_TEST_RECEIVE_PATH = "86h/1h/0h/0"
const val BIP86_TEST_CHANGE_PATH = "86h/1h/0h/1"

// Mainnet
const val MAINNET_EXTENDED_PRIVKEY = "xprv9s21ZrQH143K3LRcTnWpaCSYb75ic2rGuSgicmJhSVQSbfaKgPXfa8PhnYszgdcyWLoc8n1E2iHUnskjgGTAyCEpJYv7fqKxUcRNaVngA1V"
const val BIP84_MAINNET_RECEIVE_PATH = "84h/0h/0h/1"
const val BIP86_MAINNET_RECEIVE_PATH = "86h/0h/0h/1"

val BIP84_DESCRIPTOR: Descriptor = Descriptor(
    "wpkh($TEST_EXTENDED_PRIVKEY/$BIP84_TEST_RECEIVE_PATH/*)",
    Network.TESTNET
)
val BIP84_CHANGE_DESCRIPTOR: Descriptor = Descriptor(
    "wpkh($TEST_EXTENDED_PRIVKEY/$BIP84_TEST_CHANGE_PATH/*)",
    Network.TESTNET
)
val BIP86_DESCRIPTOR: Descriptor = Descriptor(
    "tr($TEST_EXTENDED_PRIVKEY/$BIP86_TEST_RECEIVE_PATH/*)",
    Network.TESTNET
)
val BIP86_CHANGE_DESCRIPTOR: Descriptor = Descriptor(
    "tr($TEST_EXTENDED_PRIVKEY/$BIP86_TEST_CHANGE_PATH/*)",
    Network.TESTNET
)
val NON_EXTENDED_DESCRIPTOR_0: Descriptor = Descriptor(
    descriptor = "wpkh($TEST_EXTENDED_PRIVKEY/$BIP84_TEST_RECEIVE_PATH/0)",
    network = Network.TESTNET
)
val NON_EXTENDED_DESCRIPTOR_1: Descriptor = Descriptor(
    descriptor = "wpkh($TEST_EXTENDED_PRIVKEY/$BIP84_TEST_RECEIVE_PATH/1)",
    network = Network.TESTNET
)
