use uniffi_bindgen;

pub const BDK_UDL: &str = "src/bdk.udl";

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Language {
    KOTLIN,
    // PYTHON,
    // SWIFT,
    UNSUPPORTED,
}

impl Language {
    fn to_string(self) -> &'static str {
        if self == Language::KOTLIN { "kotlin" }
        // else if self == Language::PYTHON { "python" }
        // else if self == Language::SWIFT { "swift" }
        else { panic!("Not a supported language") }
    }
}

fn parse_language(language_argument: &str) -> Language {
   match language_argument {
       "kotlin" => Language::KOTLIN,
       // "python" => Language::PYTHON,
       // "swift" => Language::SWIFT,
       _ => panic!("Unsupported language")
   }
}

fn generate_bindings() -> Result<(), Box<dyn std::error::Error>> {
    use std::env;
    let args: Vec<String> = env::args().collect();
    let language: Language;
    let output_directory: &String;

    if &args[1] != "--language" {
        panic!("Please provide the --language option");
    } else {
        language = parse_language(&args[2]);
    }

    if &args[3] != "--out-dir" {
        panic!("Please provide the --out-dir option");
    } else {
        output_directory = &args[4]
    }

    println!("Chosen language is {:?}", language);
    println!("Output directory is {:?}", output_directory);

    uniffi_bindgen::generate_bindings(
        &format!("{}/{}", env!("CARGO_MANIFEST_DIR"), BDK_UDL),
        None,
        vec![language.to_string()],
        Some(&output_directory),
        false,
    )?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    generate_bindings()?;
    Ok(())
}
