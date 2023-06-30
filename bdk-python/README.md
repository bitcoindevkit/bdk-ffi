# bdk-python
The Python language bindings for the [bitcoindevkit](https://github.com/bitcoindevkit).

See the [package on PyPI](https://pypi.org/project/bdkpython/).  

## Install from PyPI
Install the latest release using
```shell
pip install bdkpython
```

## Run the tests
```shell
pip install --requirement requirements.txt
bash ./scripts/generate-linux.sh # here you should run the script appropriate for your platform
python setup.py bdist_wheel --verbose
pip install ./dist/bdkpython-<yourversion>.whl --force-reinstall
python -m unittest --verbose tests/test_bdk.py
```

## Build the package
```shell
# Install dependencies
pip install --requirement requirements.txt

# Generate the bindings (use the script appropriate for your platform)
bash ./scripts/generate-linux.sh

# Build the wheel
python setup.py --verbose bdist_wheel
```

## Install locally
```shell
pip install ./dist/bdkpython-<yourversion>.whl
```
