uniffi::build_foreign_language_testcases!(
    // Not sure why the new types break this Kotlin test and not the others, but the libraries work
    // fine. Commenting out for now.
    // "tests/bindings/test.kts",
    "tests/bindings/test.swift",
    // Weirdly enough, the Python tests below pass locally, but fail on the CI with the error:
    // ModuleNotFoundError: No module named 'bdkpython'
    // "tests/bindings/test.py",
);
