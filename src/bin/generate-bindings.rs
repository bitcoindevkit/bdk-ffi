use std::fmt;
use std::str::FromStr;
use structopt::StructOpt;
use uniffi_bindgen;

pub const BDK_UDL: &str = "src/bdk.udl";

#[derive(Debug, PartialEq)]
pub enum Language {
    KOTLIN,
    PYTHON,
    SWIFT,
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Language::KOTLIN => write!(f, "kotlin"),
            Language::SWIFT => write!(f, "swift"),
            Language::PYTHON => write!(f, "python"),
        }
    }
}

#[derive(Debug)]
pub enum Error {
    UnsupportedLanguage,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for Language {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "kotlin" => Ok(Language::KOTLIN),
            "python" => Ok(Language::PYTHON),
            "swift" => Ok(Language::SWIFT),
            _ => Err(Error::UnsupportedLanguage),
        }
    }
}

fn generate_bindings(opt: &Opt) -> anyhow::Result<(), anyhow::Error> {
    uniffi_bindgen::generate_bindings(
        &format!("{}/{}", env!("CARGO_MANIFEST_DIR"), BDK_UDL),
        None,
        vec![opt.language.to_string().as_str()],
        Some(&opt.out_dir),
        false,
    )?;

    Ok(())
}

fn fixup_python_lib_path<O: AsRef<std::path::Path>>(
    out_dir: O,
    lib_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    use std::fs;
    use std::io::Write;

    const LOAD_INDIRECT_DEF: &str = "def loadIndirect():";

    let bindings_file = out_dir.as_ref().join("bdk.py");
    let mut data = fs::read_to_string(&bindings_file)?;

    let pos = data.find(LOAD_INDIRECT_DEF).expect(&format!(
        "loadIndirect not found in `{}`",
        bindings_file.display()
    ));
    let range = pos..pos + LOAD_INDIRECT_DEF.len();

    let replacement = format!(
        r#"
def loadIndirect():
    import glob
    return getattr(ctypes.cdll, glob.glob(os.path.join(os.path.dirname(os.path.abspath(__file__)), '{}.*'))[0])

def _loadIndirectOld():"#,
        lib_name
    );
    data.replace_range(range, &replacement);

    let mut file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&bindings_file)?;
    file.write(data.as_bytes())?;

    Ok(())
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "generate-bindings",
    about = "A tool to generate bdk-ffi language bindings"
)]
struct Opt {
    /// Language to generate bindings for
    #[structopt(env = "UNIFFI_BINDGEN_LANGUAGE", short, long, possible_values(&["kotlin","swift","python"]), parse(try_from_str = Language::from_str))]
    language: Language,

    /// Output directory to put generated language bindings
    #[structopt(env = "UNIFFI_BINDGEN_OUTPUT_DIR", short, long)]
    out_dir: String,

    /// Python fix up lib path
    #[structopt(env = "UNIFFI_BINDGEN_PYTHON_FIXUP_PATH", short, long)]
    python_fixup_path: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();

    println!("Chosen language is {}", opt.language);
    println!("Output directory is {}", opt.out_dir);

    generate_bindings(&opt)?;

    if opt.language == Language::PYTHON {
        if let Some(path) = &opt.python_fixup_path {
            println!("Fixing up python lib path, {}", &path);
            fixup_python_lib_path(&opt.out_dir, &path)?;
        }
    }
    Ok(())
}
