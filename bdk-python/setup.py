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
from bdkpython import Wallet
```
"""

setup(
    name="bdkpython",
    version="1.1.0",
    description="The Python language bindings for the Bitcoin Development Kit",
    long_description=LONG_DESCRIPTION,
    long_description_content_type="text/markdown",
    include_package_data = True,
    zip_safe=False,
    packages=["bdkpython"],
    package_dir={"bdkpython": "./src/bdkpython"},
    url="https://github.com/bitcoindevkit/bdk-ffi",
    author="Bitcoin Dev Kit Developers <dev@bitcoindevkit.org>",
    license="MIT or Apache 2.0",
    # This is required to ensure the library name includes the python version, abi, and platform tags
    # See issue #350 for more information
    has_ext_modules=lambda: True,
)
