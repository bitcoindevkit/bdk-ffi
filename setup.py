#!/usr/bin/env python

import os

from setuptools import setup
from setuptools_rust import Binding, RustExtension

LONG_DESCRIPTION = """# bdkpython
The Python language bindings for the [bitcoindevkit](https://github.com/bitcoindevkit).

## Install the package
```shell
pip install bdkpython
```

## Simple example
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

wallet = bdk.Wallet(
             descriptor=descriptor,
             change_descriptor=descriptor,
             network=bdk.Network.TESTNET,
             database_config=config,
             blockchain_config=client
         )

# print new receive address
address = wallet.get_new_address()
print(f"New BIP84 testnet address: {address}")


# print wallet balance
class LogProgress(bdk.BdkProgress):
    def update(self, progress, update):
        pass

wallet.sync(progress_update=LogProgress(), max_address_param=20)
balance = wallet.get_balance()
print(f"Wallet balance is: {balance}")
"""

rust_ext = RustExtension(
    "bdkpython.bdkffi",
    path="./bdk-ffi/Cargo.toml",
    binding=Binding.NoBinding,
)

setup(
    name = 'bdkpython',
    version = '0.0.6-SNAPSHOT',
    description="The Python language bindings for the bitcoindevkit",
    long_description=LONG_DESCRIPTION,
    long_description_content_type='text/markdown',
    rust_extensions=[rust_ext],
    zip_safe=False,
    packages=['bdkpython'],
    package_dir={ 'bdkpython': './src/bdkpython' },
    url="https://github.com/thunderbiscuit/bdk-python",
    author="Alekos Filini <alekos.filini@gmail.com>, Steve Myers <steve@notmandatory.org>",
    license="MIT or Apache 2.0",
)
