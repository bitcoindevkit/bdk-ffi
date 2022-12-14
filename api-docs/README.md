# API documentation
The Bitcoin Dev Kit language bindings make use of the [uniffi-rs](https://github.com/mozilla/uniffi-rs) library to produce their bindings. While efforts are ongoing to allow inline documentation on the Rust side to be ported to the bindings code, this is not currently possible.

This directory contains our temporary solution to this problem. A set of files mimicking the bindings libraries in their function signatures, but without any implementation. This allows for documentation build tools to produce API docs similarly to how we would like them to be if they could be inlined.

You can find the resulting API documentation websites in the ["API Reference" section of the sidebar](https://bitcoindevkit.org/getting-started/) under the "Docs" tab on the bitcoindevkit official website.
