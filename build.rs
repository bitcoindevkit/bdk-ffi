use std::fs;
use std::env;
use std::io::Write;
use std::path::Path;

const BDK_UDL: &str = "src/bdk.udl";

fn fixup_python_lib_path<O: AsRef<Path>>(out_dir: O, lib_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    const LOAD_INDIRECT_DEF: &str = "def loadIndirect():";

    let bindings_file = out_dir.as_ref().join("bdk.py");
    let mut data = fs::read_to_string(&bindings_file)?;

    let pos = data.find(LOAD_INDIRECT_DEF).expect(&format!("loadIndirect not found in `{}`", bindings_file.display()));
    let range = pos..pos + LOAD_INDIRECT_DEF.len();

    let replacement = format!(r#"
def loadIndirect():
    import glob
    return getattr(ctypes.cdll, glob.glob(os.path.join(os.path.dirname(os.path.abspath(__file__)), '{}.*'))[0])

def _loadIndirectOld():"#, lib_name);
    data.replace_range(range, &replacement);

    let mut file = fs::OpenOptions::new().write(true).truncate(true).open(&bindings_file)?;
    file.write(data.as_bytes())?;

    Ok(())
}

fn main() {
    uniffi_build::generate_scaffolding(BDK_UDL).unwrap();

    if let Some(lang) = env::var("GENERATE_BINDINGS_LANG").ok() {
        let out_path = env::var("GENERATE_BINDINGS_OUT").expect("GENERATE_BINDINGS_OUT must be set when GENERATE_BINDINGS_LANG is");
        uniffi_bindgen::generate_bindings(BDK_UDL, None, vec![&lang], Some(&out_path), false).unwrap();

        match (lang.as_ref(), env::var("GENERATE_BINDINGS_FIXUP_LIB_PATH").ok()) {
            ("python", Some(name)) => fixup_python_lib_path(&out_path, &name).unwrap(),
            _ => {},
        }
    }

    println!("cargo:rerun-if-changed=GENERATE_BINDINGS_LANG");
    println!("cargo:rerun-if-changed=GENERATE_BINDINGS_OUT");
    println!("cargo:rerun-if-changed=GENERATE_BINDINGS_FIXUP_LIB_PATH");
}
