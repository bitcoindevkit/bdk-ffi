import bdk



descriptor = "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/*)"
config = bdk.DatabaseConfig.MEMORY("")
wallet = bdk.OfflineWallet(descriptor, bdk.Network.REGTEST, config)

address = wallet.get_new_address()
print("Address 1 is:", address)



# db = bdk.DatabaseConfig.MEMORY("")
# client = bdk.BlockchainConfig.ELECTRUM(
#     bdk.ElectrumConfig("ssl://electrum.blockstream.info:60002", None, 5, None, 100)
# )
# wallet2 = bdk.OnlineWallet(descriptor, None, bdk.Network.TESTNET, db, client)
# wallet2.sync()
# balance = wallet2.get_balance()
# print(balance)

