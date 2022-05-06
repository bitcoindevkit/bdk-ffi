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
python -m tox
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
pip install ./dist/bdkpython-0.0.1-py3-none-any.whl
```
