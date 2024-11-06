import bdkpython as bdk
import unittest


descriptor = bdk.Descriptor("wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/*)", bdk.Network.TESTNET)
db_config = bdk.DatabaseConfig.MEMORY()
blockchain_config = bdk.BlockchainConfig.ELECTRUM(
    bdk.ElectrumConfig(
        url = "ssl://electrum.blockstream.info:60002",
        socks5 = None,
        retry = 5,
        timeout = None,
        stop_gap = 100,
        validate_domain = True,
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
        address_info = wallet.get_address(bdk.AddressIndex.LAST_UNUSED())
        address = address_info.address.as_string()
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

    def test_output_address_from_script_pubkey(self):
        wallet = bdk.Wallet(
            descriptor=descriptor,
            change_descriptor=None,
            network=bdk.Network.TESTNET,
            database_config=db_config,
        )
        wallet.sync(blockchain, None)
        first_tx = list(wallet.list_transactions(True))[0]
        assert first_tx.txid == '35d3de8dd429ec4c9684168c1fbb9a4fb6db6f2ce89be214a024657a73ef4908'
        
        output1, output2 = list(first_tx.transaction.output())
        
        assert bdk.Address.from_script(output1.script_pubkey, bdk.Network.TESTNET).as_string() == 'tb1qw6ly2te8k9vy2mwj3g6gx82hj7hc8f5q3vry8t'
        assert bdk.Address.from_script(output2.script_pubkey, bdk.Network.TESTNET).as_string() == 'tb1qzsvpnmme78yl60j7ldh9aqvhvxr4mz7mjpmh22'
        
if __name__ == '__main__':
    unittest.main()
