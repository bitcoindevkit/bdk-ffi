# BDK Kyoto

BDK-Kyoto is an extension of [Kyoto](https://github.com/2140-dev/kyoto), a client-side implementation of BIP157/BIP158.
These proposals define a way for users to fetch transactions privately, using _compact block filters_.
You may want to read the specification [here](https://github.com/bitcoin/bips/blob/master/bip-0158.mediawiki).
Kyoto runs as a psuedo-node, sending messages over the Bitcoin peer-to-peer layer, finding new peers to connect to, and managing a
light-weight database of Bitcoin block headers. As such, developing a wallet application using this crate is distinct from a typical
client/server relationship. Esplora and Electrum offer _proactive_ APIs, in that the servers will respond to events as they are requested.

In the case of running a node as a background process, the developer experience is far more _reactive_, in that the node may emit any number of events, and the application may respond to them. BDK-Kyoto curates these events into structures that are easily handled by BDK APIs, making integration of compact block filters easily understood.

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)

at your option.
