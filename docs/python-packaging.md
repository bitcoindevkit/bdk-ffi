# Building the Python Package

I am able to package the native library in the tar ball that gets installed with pip, but the `bdk.py` module does not find it by itself. It looks for the native lib on the system path (and if you have it in one of those places it runs fine), but it doesn't seem to know about the path that's right beside it!

The current layout is the following:
```shell
❯ tree
.
└── bitcoindevkit
   ├── __init__.py
   ├── bdk.py
   └── libbdkffi.dylib
```

And I need a way to tell the package that part of its path should be the `./` path, _or_ I have to fix up the `bdk.py` file to point directly at its own directory when it searches for the native lib.

The following code fix for the loadIndirect() method makes the whole package work and the tests run:
```python
def loadIndirect():
    if sys.platform == "linux":
        # libname = "lib{}.so"
        libname = os.path.join(os.path.dirname(__file__), "lib{}.so")
    elif sys.platform == "darwin":
        # libname = "lib{}.dylib"
        libname = os.path.join(os.path.dirname(__file__), "lib{}.dylib")
    elif sys.platform.startswith("win"):
        # As of python3.8, ctypes does not seem to search $PATH when loading DLLs.
        # We could use `os.add_dll_directory` to configure the search path, but
        # it doesn't feel right to mess with application-wide settings. Let's
        # assume that the `.dll` is next to the `.py` file and load by full path.
        libname = os.path.join(
            os.path.dirname(__file__),
            "{}.dll",
        )
    return getattr(ctypes.cdll, libname.format("bdkffi"))
```
