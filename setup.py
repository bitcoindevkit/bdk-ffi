#!/usr/bin/env python

from setuptools import setup, find_packages

LONG_DESCRIPTION = """# bdkpython
The Python language bindings for the [bitcoindevkit](https://github.com/bitcoindevkit).

## Install the package
```shell
pip install bdkpython
```

## Simple examples
**Generate a testnet address**
```python
import bdkpython as bdk


descriptor = "wpkh(tprv8ZgxMBicQKsPcx5nBGsR63Pe8KnRUqmbJNENAfGftF3yuXoMMoVJJcYeUw5eVkm9WBPjWYt6HMWYJNesB5HaNVBaFc1M6dRjWSYnmewUMYy/84h/1h/0h/0/*)"
config = bdk.DatabaseConfig.MEMORY("")

wallet = bdk.OfflineWallet(descriptor, bdk.Network.TESTNET, config)
address = wallet.get_new_address()

print(f"New BIP84 testnet address: {address}")
```  

**Sync a wallet using Electrum**
```python
import bdkpython as bdk

descriptor = "wpkh(tprv8ZgxMBicQKsPcx5nBGsR63Pe8KnRUqmbJNENAfGftF3yuXoMMoVJJcYeUw5eVkm9WBPjWYt6HMWYJNesB5HaNVBaFc1M6dRjWSYnmewUMYy/84h/1h/0h/0/*)"
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

wallet = bdk.OnlineWallet(
             descriptor=descriptor,
             change_descriptor=descriptor,
             network=bdk.Network.TESTNET,
             database_config=config,
             blockchain_config=client
         )

class LogProgress(bdk.BdkProgress):
    def update(self, progress, update):
        pass
        
wallet.sync(progress_update=LogProgress(), max_address_param=20)
balance = wallet.get_balance()

print(f"Wallet balance is: {balance}")
```
"""

setup(
    name='bdkpython',
    version='0.0.3',
    packages=find_packages(where="src"),
    package_dir={"": "src"},
    package_data={"bdkpython": ["*.dylib"]},
    include_package_data=True,
    zip_safe=False,
    description="The Python language bindings for the bitcoindevkit",
    long_description=LONG_DESCRIPTION,
    long_description_content_type='text/markdown',
    url="https://github.com/bitcoindevkit/bdk-ffi",
    author="Alekos Filini <alekos.filini@gmail.com>, Steve Myers <steve@notmandatory.org>",
    license="MIT or Apache 2.0",
)
