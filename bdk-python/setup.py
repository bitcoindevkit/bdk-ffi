#!/usr/bin/env python

import os

from setuptools import setup
from setuptools_rust import Binding, RustExtension

LONG_DESCRIPTION = """# bdkpython
The Python language bindings for the [Bitcoin Dev Kit](https://github.com/bitcoindevkit).

## Install the package
```shell
pip install bdkpython
```

## Simple example
```python
import bdkpython as bdk


descriptor = bdk.Descriptor("wpkh(tprv8ZgxMBicQKsPcx5nBGsR63Pe8KnRUqmbJNENAfGftF3yuXoMMoVJJcYeUw5eVkm9WBPjWYt6HMWYJNesB5HaNVBaFc1M6dRjWSYnmewUMYy/84h/0h/0h/0/*)", bdk.Network.TESTNET) 
db_config = bdk.DatabaseConfig.MEMORY()
blockchain_config = bdk.BlockchainConfig.ELECTRUM(
    bdk.ElectrumConfig(
        "ssl://electrum.blockstream.info:60002",
        None,
        5,
        None,
        100,
        True,
    )
)
blockchain = bdk.Blockchain(blockchain_config)

wallet = bdk.Wallet(
             descriptor=descriptor,
             change_descriptor=None,
             network=bdk.Network.TESTNET,
             database_config=db_config,
         )

# print new receive address
address_info = wallet.get_address(bdk.AddressIndex.LAST_UNUSED())
address = address_info.address
index = address_info.index
print(f"New BIP84 testnet address: {address} at index {index}")


# print wallet balance
wallet.sync(blockchain, None)
balance = wallet.get_balance()
print(f"Wallet balance is: {balance.total}")
"""

rust_ext = RustExtension(
    target="bdkpython.bdkffi",
    path="../bdk-ffi/Cargo.toml",
    binding=Binding.NoBinding,
)

setup(
    name='bdkpython',
    version='0.28.0.dev0',
    description="The Python language bindings for the Bitcoin Development Kit",
    long_description=LONG_DESCRIPTION,
    long_description_content_type='text/markdown',
    rust_extensions=[rust_ext],
    zip_safe=False,
    packages=['bdkpython'],
    package_dir={'bdkpython': './src/bdkpython'},
    url="https://github.com/bitcoindevkit/bdk-ffi",
    author="Alekos Filini <alekos.filini@gmail.com>, Steve Myers <steve@notmandatory.org>",
    license="MIT or Apache 2.0",
)
