# Errors

Returning meaningful errors is important for users of our libraries. Libraries return errors and applications decide if and how those errors are formatted and displayed to users. Our goal as a library is to produce meaningful and structured error types which allows applications to easily differentiate various error cases. The Rust bitcoin ecosystem uses descriptive and data-driven errors, and we would like to stay as close to them as possible when building language bindings for these libraries.

## Context and Problem Statement

The tool we use to produce language bindings for bdk-ffi libraries is [uniffi]. While the library is powerful, it also comes with some caveats. Those come into play when attempting to expose Rust errors; we must choose between simple enums that have variants but not data associated with them (for example, a `TxidError` would not be able to return the specific txid that triggered the error), or more complex objects that do have data fields, but no ability to control the error message returned to the user. What's more, while Kotlin and Java users expect a `message` field on an `Exception` type, this field becomes an empty string if the object has no data, and just a strignified version of the fields if it does have any. Swift, in contrast, can optionally leverage something like the `LocalizedError` protocol to provide custom and localized descriptions.

## Decision Drivers

Some options were considered and explored in detail in [issue #509]. Some aspects of this decision include:
- Expectations from devs using the libraries in different languages (for example, Kotlin users expect a `message` field on the exception, whereas Swift user expect a `localizedDescription`)
- Ease of maintaining the errors as they evolve and we expose more and more of the Rust BDK ecosystem
- Important vs nice-to-have features (some fields can be stringified without loss of information, some cannot)

## Decision Outcome

We have decided to leverage two different approaches that uniffi offers for exposing errors, and using the most appropriate one for each situation.

In the case where errors do not require passing back data, we opt for the simpler to maintain option, which provides better, customized error message with a `message` field provided by the `thiserror` library. Those errors cannot have fields. For example:

### UDL

```txt
[Error]
enum FeeRateError {
  "ArithmeticOverflow"
};
```

#### Rust
```rust
#[derive(Debug, thiserror::Error)]
pub enum FeeRateError {
    #[error("arithmetic overflow on feerate {fee_rate}")]
    ArithmeticOverflow { fee_rate: u64 },
}
```

In the case where complex types should be returned as part of the error, we use the more complex interface UDL type and return the data, at the cost of more explicit messages. For example:

### UDL

```txt
[Error]
interface CalculateFeeError {
  MissingTxOut(sequence<OutPoint> out_points);
};
```

### Rust

```rust
#[derive(Debug, thiserror::Error)]
pub enum CalculateFeeError {
    #[error("missing transaction output: {out_points:?}")]
    MissingTxOut { out_points: Vec<OutPoint> },
}
```

[uniffi]: https://github.com/mozilla/uniffi-rs/
[issue #509]: https://github.com/bitcoindevkit/bdk-ffi/issues/509
