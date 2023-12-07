# Wrapping BDK APIs

Producing language bindings potentially requires wrapping a number of APIs, and this document outlines the approach we have decided to take when thinking through this problem for bdk-ffi libraries.

## Context and Problem Statement

The tool we use to produce language bindings for bdk-ffi libraries is [uniffi]. While the library is powerful, it also comes with some caveats. Some of those include the inability to expose to foreign bindings Rust-specific types like tuples, and the inability to expose generics. This means that at least _some_ wrapping and transforming of certain things are required between the pure Rust code coming from the bdk library and the final language bindings in Swift, Kotlin, and Python.

With wrapping comes (a) the requirement for naming potentially new types, and (b) the ability to "wrap" behaviour that could be useful for end users. This document addresses point (b).

## Decision Drivers

Our main goals are:
1. Keep the multiple language bindings libraries maintainable.
2. Help users of bdk help each other and working with a similarly shaped API across languages.

## Decision Outcome

There are three potential reasons for wrapping Rust BDK APIs:
1. The Rust types are not available in the target language (e.g., a function returns a tuple, which can't be returned in Swift/Kotlin)
2. Some complex functionality is available in the Rust bitcoin/miniscript/bdk ecosystem, but exposing all underlying types required for this functionality is out of scope at the time a particular feature is required
3. Some extra feature/utility might be interesting for our end-users

Our approach with the bdk-ffi libraries is to only provide wrapping for cases (1) and (2) mentioned above. If extra functionality to the BDK API would be useful, we open issues upstream and merge those in Rust first, and then expose it in our bindings. This approach favors (a) keeping the bindings libraries as thin as possible, minimizing the potential for integrating bugs at the bindings layer, and (b) keeping the API as close as we can to Rust BDK, promoting collaboration between users of BDK across languages, including with teams that use BDK in bindings (mobile) and server-side (Rust).

[uniffi]: https://github.com/mozilla/uniffi-rs/
