import bitcoindevkit

# taken from bdk test suite @ https://github.com/bitcoindevkit/bdk/blob/master/src/descriptor/template.rs#L676
descriptor = "wpkh(tprv8ZgxMBicQKsPcx5nBGsR63Pe8KnRUqmbJNENAfGftF3yuXoMMoVJJcYeUw5eVkm9WBPjWYt6HMWYJNesB5HaNVBaFc1M6dRjWSYnmewUMYy/84h/0h/0h/0/*)"
config = bitcoindevkit.DatabaseConfig.MEMORY("")
wallet = bitcoindevkit.OfflineWallet(descriptor, bitcoindevkit.Network.REGTEST, config)
address = wallet.get_new_address()

def test_address_BIP84():
    assert address == "bcrt1qkmvk2nadgplmd57ztld8nf8v2yxkzmdvwtjf8s"
