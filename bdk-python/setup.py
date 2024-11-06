#!/usr/bin/env python

from setuptools import setup

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
        url = "ssl://electrum.blockstream.info:60002",
        socks5 = None,
        retry = 5,
        timeout = None,
        stop_gap = 100,
        validate_domain = True,
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

setup(
    name="bdkpython",
    version="0.32.0",
    description="The Python language bindings for the Bitcoin Development Kit",
    long_description=LONG_DESCRIPTION,
    long_description_content_type="text/markdown",
    include_package_data = True,
    zip_safe=False,
    packages=["bdkpython"],
    package_dir={"bdkpython": "./src/bdkpython"},
    url="https://github.com/bitcoindevkit/bdk-ffi",
    author="Alekos Filini <alekos.filini@gmail.com>, Steve Myers <steve@notmandatory.org>",
    license="MIT or Apache 2.0",
    # This is required to ensure the library name includes the python version, abi, and platform tags
    # See issue #350 for more information
    has_ext_modules=lambda: True,
)
