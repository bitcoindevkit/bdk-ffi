from bdkpython import Descriptor
from bdkpython import Wallet
from bdkpython import KeychainKind
from bdkpython import Connection
from bdkpython import AddressInfo
from bdkpython import Network

import unittest
import os

descriptor: Descriptor = Descriptor(
    "wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/1h/0/*)",
    Network.TESTNET
)
change_descriptor: Descriptor = Descriptor(
    "wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/1h/1/*)",
    Network.TESTNET
)

class OfflineWalletTest(unittest.TestCase):

    def tearDown(self) -> None:
        if os.path.exists("./bdk_persistence.sqlite"):
            os.remove("./bdk_persistence.sqlite")
    
    def test_new_address(self):
        connection: Connection = Connection.new_in_memory()
        wallet: Wallet = Wallet(
            descriptor,
            change_descriptor,
            Network.TESTNET,
            connection
        )
        address_info: AddressInfo = wallet.reveal_next_address(KeychainKind.EXTERNAL)
    
        self.assertTrue(address_info.address.is_valid_for_network(Network.TESTNET), "Address is not valid for testnet network")
        self.assertTrue(address_info.address.is_valid_for_network(Network.SIGNET), "Address is not valid for signet network")
        self.assertFalse(address_info.address.is_valid_for_network(Network.REGTEST), "Address is valid for regtest network, but it shouldn't be")
        self.assertFalse(address_info.address.is_valid_for_network(Network.BITCOIN), "Address is valid for bitcoin network, but it shouldn't be")
    
        self.assertEqual("tb1qhjys9wxlfykmte7ftryptx975uqgd6kcm6a7z4", address_info.address.__str__())
    
    def test_balance(self):
        connection: Connection = Connection.new_in_memory()
        wallet: Wallet = Wallet(
            descriptor,
            change_descriptor,
            Network.TESTNET,
            connection
        )
    
        self.assertEqual(wallet.balance().total.to_sat(), 0)

if __name__ == '__main__':
    unittest.main()
