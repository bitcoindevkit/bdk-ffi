import bdkpython as bdk
import unittest

descriptor = "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/*)"
db_config = bdk.DatabaseConfig.MEMORY()
blockchain_config = bdk.BlockchainConfig.ELECTRUM(
    bdk.ElectrumConfig(
        "ssl://electrum.blockstream.info:60002",
        None,
        5,
        None,
        100
    )
)
blockchain = bdk.Blockchain(blockchain_config)


class TestSimpleBip84Wallet(unittest.TestCase):

    def test_address_bip84_testnet(self):
        wallet = bdk.Wallet(
            descriptor=descriptor,
            change_descriptor=None,
            network=bdk.Network.TESTNET,
            database_config=db_config
        )
        address_info = wallet.get_address(bdk.AddressIndex.LAST_UNUSED)
        address = address_info.address
        # print(f"New address is {address}")
        assert address == "tb1qzg4mckdh50nwdm9hkzq06528rsu73hjxxzem3e", f"Wrong address {address}, should be tb1qzg4mckdh50nwdm9hkzq06528rsu73hjxxzem3e"

    def test_wallet_balance(self):
        wallet = bdk.Wallet(
            descriptor=descriptor,
            change_descriptor=None,
            network=bdk.Network.TESTNET,
            database_config=db_config,
        )
        wallet.sync(blockchain, None)
        balance = wallet.get_balance()
        # print(f"Balance is {balance.total} sat")
        assert balance.total > 0, "Balance is 0, send testnet coins to tb1qzg4mckdh50nwdm9hkzq06528rsu73hjxxzem3e"


if __name__ == '__main__':
    unittest.main()
