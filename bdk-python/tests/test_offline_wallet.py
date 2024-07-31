import bdkpython as bdk
import unittest
import os

descriptor: bdk.Descriptor = bdk.Descriptor(
    "wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/0/*)",
    bdk.Network.TESTNET
)
change_descriptor: bdk.Descriptor = bdk.Descriptor(
    "wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/0h/1/*)",
    bdk.Network.TESTNET
)

class OfflineWalletTest(unittest.TestCase):

    def tearDown(self) -> None:
        if os.path.exists("./bdk_persistence.sqlite"):
            os.remove("./bdk_persistence.sqlite")
    
    def test_new_address(self):
        connection: bdk.Connection = bdk.Connection.new_in_memory()
        wallet: bdk.Wallet = bdk.Wallet(
            descriptor,
            change_descriptor,
            bdk.Network.TESTNET,
            connection
        )
        address_info: bdk.AddressInfo = wallet.reveal_next_address(bdk.KeychainKind.EXTERNAL)
    
        self.assertTrue(address_info.address.is_valid_for_network(bdk.Network.TESTNET), "Address is not valid for testnet network")
        self.assertTrue(address_info.address.is_valid_for_network(bdk.Network.SIGNET), "Address is not valid for signet network")
        self.assertFalse(address_info.address.is_valid_for_network(bdk.Network.REGTEST), "Address is valid for regtest network, but it shouldn't be")
        self.assertFalse(address_info.address.is_valid_for_network(bdk.Network.BITCOIN), "Address is valid for bitcoin network, but it shouldn't be")
    
        self.assertEqual("tb1qrnfslnrve9uncz9pzpvf83k3ukz22ljgees989", address_info.address.__str__())
    
    def test_balance(self):
        connection: bdk.Connection = bdk.Connection.new_in_memory()
        wallet: bdk.Wallet = bdk.Wallet(
            descriptor,
            change_descriptor,
            bdk.Network.TESTNET,
            connection
        )
    
        self.assertEqual(wallet.balance().total.to_sat(), 0)

if __name__ == '__main__':
    unittest.main()
