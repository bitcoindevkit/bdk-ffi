pub const BDK_UDL: &str = "src/bdk.udl";

#[cfg(feature = "generate-python")]
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

#[cfg(feature = "generate-python")]
fn generate_python() -> Result<(), Box<dyn std::error::Error>> {
    use std::env;

    let out_path = env::var("GENERATE_PYTHON_BINDINGS_OUT")
        .map_err(|_| String::from("`GENERATE_PYTHON_BINDINGS_OUT` env variable missing"))?;
    uniffi_bindgen::generate_bindings(
        &format!("{}/{}", env!("CARGO_MANIFEST_DIR"), BDK_UDL),
        None,
        vec!["python"],
        Some(&out_path),
        false,
    )?;

    if let Some(name) = env::var("GENERATE_PYTHON_BINDINGS_FIXUP_LIB_PATH").ok() {
        fixup_python_lib_path(&out_path, &name)?;
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "generate-python")]
    generate_python()?;
    Ok(())
}
