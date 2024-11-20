use std::fs;
use anyhow::Context;
use uniffi_udl;
use uniffi_meta::Metadata;

fn main() -> anyhow::Result<()> {
  let udl_file = "../bdk-ffi/src/bdk.udl";
  let udl = fs::read_to_string(udl_file)
    .with_context(|| format!("Failed to read UDL from {udl_file}"))?;
  let group = uniffi_udl::parse_udl(&udl, "bdk")?;
  println!("Namespace: {}", group.namespace.name);
  println!("\nItems:");

  println!("Objects in UDL:");
  for item in group.items.iter() {
      if let Metadata::Object(obj) = item {
          println!("  - {}", obj.name);
      }
  }
  Ok(())
}