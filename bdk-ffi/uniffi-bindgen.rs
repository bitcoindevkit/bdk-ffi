fn main() {
    uniffi_bindgen()
}

fn uniffi_bindgen() {
    // uniffi_bindgen_main parses command line arguments for officially supported languages,
    // but we need to parse them manually first to decide whether to use the uniffi_dart plugin.
    let args: Vec<String> = std::env::args().collect();
    let language = args
        .iter()
        .position(|arg| arg == "--language")
        .and_then(|idx| args.get(idx + 1));
    match language {
        #[cfg(feature = "dart")]
        Some(lang) if lang == "dart" => {
            use camino::Utf8Path;

            let library_path = args
                .iter()
                .find_map(|arg| {
                    if !arg.starts_with("--")
                        && (arg.ends_with(".dylib")
                            || arg.ends_with(".so")
                            || arg.ends_with(".dll"))
                    {
                        Some(arg.clone())
                    } else {
                        None
                    }
                })
                .expect("Library path not found - specify a .dylib, .so, or .dll file");
            let output_dir = args
                .iter()
                .position(|arg| arg == "--out-dir")
                .and_then(|idx| args.get(idx + 1))
                .expect("--out-dir is required when using --library");

            // For library mode with proc macros, we need to extract metadata from the built library
            // The UDL path is used for config lookup, we can use a placeholder
            let udl_path = Utf8Path::new("src/lib.rs");
            uniffi_dart::gen::generate_dart_bindings(
                udl_path,
                None,
                Some(Utf8Path::new(output_dir.as_str())),
                Utf8Path::new(library_path.as_str()),
                true,
            )
            .expect("Failed to generate dart bindings");
        }
        _ => uniffi::uniffi_bindgen_main(),
    }
}
