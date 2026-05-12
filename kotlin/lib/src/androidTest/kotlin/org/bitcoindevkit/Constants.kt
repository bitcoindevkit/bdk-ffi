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
    NetworkKind.TEST
)
val BIP84_CHANGE_DESCRIPTOR: Descriptor = Descriptor(
    "wpkh($TEST_EXTENDED_PRIVKEY/$BIP84_TEST_CHANGE_PATH/*)",
    NetworkKind.TEST
)
val BIP86_DESCRIPTOR: Descriptor = Descriptor(
    "tr($TEST_EXTENDED_PRIVKEY/$BIP86_TEST_RECEIVE_PATH/*)",
    NetworkKind.TEST
)
val BIP86_CHANGE_DESCRIPTOR: Descriptor = Descriptor(
    "tr($TEST_EXTENDED_PRIVKEY/$BIP86_TEST_CHANGE_PATH/*)",
    NetworkKind.TEST
)
val NON_EXTENDED_DESCRIPTOR_0: Descriptor = Descriptor(
    descriptor = "wpkh($TEST_EXTENDED_PRIVKEY/$BIP84_TEST_RECEIVE_PATH/0)",
    networkKind = NetworkKind.TEST
)
val NON_EXTENDED_DESCRIPTOR_1: Descriptor = Descriptor(
    descriptor = "wpkh($TEST_EXTENDED_PRIVKEY/$BIP84_TEST_RECEIVE_PATH/1)",
    networkKind = NetworkKind.TEST
)

// Using the MNEMONIC_AWESOME
const val TEST_EXTENDED_PRIVKEY_0 = "tprv8ZgxMBicQKsPdWAHbugK2tjtVtRjKGixYVZUdL7xLHMgXZS6BFbFi1UDb1CHT25Z5PU1F9j7wGxwUiRhqz9E3nZRztikGUV6HoRDYcqPhM4"
const val BIP84_TEST_RECEIVE_PATH_0 = "84h/1h/0h/0"
val TEST_BIP84_DESCRIPTOR_0 = Descriptor("wpkh($TEST_EXTENDED_PRIVKEY_0/$BIP84_TEST_RECEIVE_PATH_0/*)", NetworkKind.TEST)

const val ESPLORA_REGTEST_URL = "http://10.0.2.2:3002"
