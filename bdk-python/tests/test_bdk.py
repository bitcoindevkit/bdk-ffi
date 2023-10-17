import bdkpython as bdk
import unittest


descriptor = bdk.Descriptor("wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/*)", bdk.Network.TESTNET)

class TestSimpleBip84Wallet(unittest.TestCase):

    def test_address_bip84_testnet(self):
        wallet = bdk.Wallet.new_no_persist(
            descriptor=descriptor,
            change_descriptor=None,
            network=bdk.Network.TESTNET,
        )
        address_info = wallet.get_address(bdk.AddressIndex.LAST_UNUSED())
        address = address_info.address.as_string()
        # print(f"New address is {address}")
        assert address == "tb1qzg4mckdh50nwdm9hkzq06528rsu73hjxxzem3e", f"Wrong address {address}, should be tb1qzg4mckdh50nwdm9hkzq06528rsu73hjxxzem3e"

if __name__ == '__main__':
    unittest.main()
