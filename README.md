# bdk-python
The Python language bindings for the [bitcoindevkit](https://github.com/bitcoindevkit).

See the [package on PyPI](https://pypi.org/project/bdkpython/).

Currently supported architectures: 
- macOS `arm64`
- macOS `x86_64`
- linux `x86_64`

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
pip install -r requirements.txt
# Generate the bindings first
bash generate.sh
# Build the wheel
python -m build
```
<br/>

## Install locally
```shell
pip install ./dist/bdkpython-0.0.1-py3-none-any.whl
```
<br/>
