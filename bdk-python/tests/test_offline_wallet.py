import bdkpython as bdk
import unittest
import os

class OfflineWalletTest(unittest.TestCase):

    def tearDown(self) -> None:
        if os.path.exists("./bdk_persistence.sqlite"):
            os.remove("./bdk_persistence.sqlite")
    
    def test_new_address(self):
        descriptor: bdk.Descriptor = bdk.Descriptor(
            "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/*)",
            bdk.Network.TESTNET
        )
        wallet: Wallet = bdk.Wallet(
            descriptor,
            None,
            "./bdk_persistence.sqlite",
            bdk.Network.TESTNET
        )
        address_info: bdk.AddressInfo = wallet.reveal_next_address(bdk.KeychainKind.EXTERNAL)
    
        self.assertTrue(address_info.address.is_valid_for_network(bdk.Network.TESTNET), "Address is not valid for testnet network")
        self.assertTrue(address_info.address.is_valid_for_network(bdk.Network.SIGNET), "Address is not valid for signet network")
        self.assertFalse(address_info.address.is_valid_for_network(bdk.Network.REGTEST), "Address is valid for regtest network, but it shouldn't be")
        self.assertFalse(address_info.address.is_valid_for_network(bdk.Network.BITCOIN), "Address is valid for bitcoin network, but it shouldn't be")
    
        self.assertEqual("tb1qzg4mckdh50nwdm9hkzq06528rsu73hjxxzem3e", address_info.address.as_string())
    
    def test_balance(self):
        descriptor: bdk.Descriptor = bdk.Descriptor(
            "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/*)",
            bdk.Network.TESTNET
        )
        wallet: bdk.Wallet = bdk.Wallet(
            descriptor,
            None,
            "./bdk_persistence.sqlite",
            bdk.Network.TESTNET
        )
    
        self.assertEqual(wallet.get_balance().total.to_sat(), 0)

if __name__ == '__main__':
    unittest.main()
