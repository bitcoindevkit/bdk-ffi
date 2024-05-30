uniffi::build_foreign_language_testcases!(
    "tests/bindings/test.kts",
    "tests/bindings/test.swift",
    // Using types defined in an external library seems to break this test and we don't know how to
    // fix it yet, but the actual Python tests and the generated package work fine.
    // from .bitcoin import Script
    //     ImportError: attempted relative import with no known parent package
    // "tests/bindings/test.py",
);
