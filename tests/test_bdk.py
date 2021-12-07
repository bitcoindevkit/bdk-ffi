import bdkpython as bdk


# taken from bdk test suite @ https://github.com/bitcoindevkit/bdk/blob/master/src/descriptor/template.rs#L676
descriptor = "wpkh(tprv8ZgxMBicQKsPcx5nBGsR63Pe8KnRUqmbJNENAfGftF3yuXoMMoVJJcYeUw5eVkm9WBPjWYt6HMWYJNesB5HaNVBaFc1M6dRjWSYnmewUMYy/84h/0h/0h/0/*)"
config = bdk.DatabaseConfig.MEMORY("")
client = bdk.BlockchainConfig.ELECTRUM(
             bdk.ElectrumConfig(
                 "ssl://electrum.blockstream.info:60002",
                 None,
                 5,
                 None,
                 100
             )   
)


def test_address_BIP84_testnet():
    wallet = bdk.OfflineWallet(descriptor, bdk.Network.TESTNET, config)
    address = wallet.get_new_address()
    assert address == "tb1qkmvk2nadgplmd57ztld8nf8v2yxkzmdvvztyse"

# def test_wallet_balance():
#     wallet = bdk.OnlineWallet(
#                 descriptor=descriptor,
#                 change_descriptor=descriptor,
#                 network=bdk.Network.TESTNET,
#                 database_config=config,
#                 blockchain_config=client
#     )
#     wallet.sync()
#     balance = wallet.get_balance()
#     assert balance > 0
