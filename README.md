# bdk-python
The Python language bindings for the [bitcoindevkit](https://github.com/bitcoindevkit).

See the [package on PyPI](https://pypi.org/project/bdkpython/).  
<br/>

## Install from PyPI
Install the latest release using
```shell
pip install bdkpython
```
<br/>

## Run the tests
```shell
pip3 install --requirement requirements.txt
bash ./generate.sh
python3 setup.py --verbose bdist_wheel
pip3 install ./dist/bdkpython-<yourversion>-py3-none-any.whl
python3 ./tests/test_bdk.py
```
<br/>

## Build the package
```shell
# Install dependecies
pip install --requirement requirements.txt

# Generate the bindings first
bash generate.sh

# Build the wheel
python3 setup.py --verbose bdist_wheel
```
<br/>

## Install locally
```shell
pip install ./dist/bdkpython-<yourversion>-py3-none-any.whl
```
