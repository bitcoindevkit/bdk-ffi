uniffi_macros::build_foreign_language_testcases!(
    ["src/bdk.udl",],
    [
        "tests/bindings/test.kts",
        "tests/bindings/test.swift",
        "tests/bindings/test.py"
    ]
);
