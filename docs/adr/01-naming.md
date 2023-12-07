# Naming convention

Producing language bindings potentially requires renaming a number of types and methods, and this document outlines the approach we have decided to take when thinking through this problem for bdk-ffi libraries.

## Context and Problem Statement

The tool we use to produce language bindings for bdk-ffi libraries is [uniffi]. While the library is powerful, it also comes with some caveats. Some of those include the inability to expose to foreign bindings Rust-specific types like tuples, and the inability to expose generics. This means that at least _some_ wrapping and transforming of certain things are required between the pure Rust code coming from the bdk library and the final language bindings in Swift, Kotlin, and Python.

With wrapping comes (a) the requirement for naming potentially new types, and (b) the ability to "wrap" behaviour that could be useful for end users. This document addresses point (a).

## Decision Drivers

Our main goals are:
1. Keep the multiple language bindings libraries maintainable.
2. Help users of bdk help each other and working with a similarly shaped API across languages.

## Decision Outcome

We decided to try and keep the names of all types the same between the Rust libraries and the bindings, and in cases where new types had to be created, to keep them in the style and spirit of the bdk and rust-bitcoin libraries.

There is so far one exception to this rule, where we renamed the `ScriptBuf` type from rust-bitcoin to `Script`. This was done because the concept of owned vs. borrowed types is strictly a Rust concept, and is not passed onto the languages bindings in any way, and therefore keeping the script type as `Script` was our preferred option in this case.

[uniffi]: https://github.com/mozilla/uniffi-rs/
