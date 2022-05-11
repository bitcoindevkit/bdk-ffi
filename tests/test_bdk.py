import bdkpython as bdk
import unittest

# taken from bdk test suite @ https://github.com/bitcoindevkit/bdk/blob/master/src/descriptor/template.rs#L676
descriptor = "wpkh(tprv8ZgxMBicQKsPcx5nBGsR63Pe8KnRUqmbJNENAfGftF3yuXoMMoVJJcYeUw5eVkm9WBPjWYt6HMWYJNesB5HaNVBaFc1M6dRjWSYnmewUMYy/84h/0h/0h/0/*)"
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
        address = wallet.get_new_address()
        # print(f"New address is {address}")
        assert address == "tb1qkmvk2nadgplmd57ztld8nf8v2yxkzmdvvztyse", f"Wrong address {address}, should be tb1qkmvk2nadgplmd57ztld8nf8v2yxkzmdvvztyse"

    def test_wallet_balance(self):
        wallet = bdk.Wallet(
            descriptor=descriptor,
            change_descriptor=None,
            network=bdk.Network.TESTNET,
            database_config=db_config,
        )
        wallet.sync(blockchain, None)
        balance = wallet.get_balance()
        # print(f"Balance is {balance} sat")
        assert balance > 0, "Balance is 0, send testnet coins to tb1qkmvk2nadgplmd57ztld8nf8v2yxkzmdvvztyse"


if __name__ == '__main__':
    unittest.main()
