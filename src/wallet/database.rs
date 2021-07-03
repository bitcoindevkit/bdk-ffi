use ::safer_ffi::prelude::*;
use bdk::database::any::SledDbConfiguration;
use bdk::database::AnyDatabaseConfig;
use safer_ffi::boxed::Box;
use safer_ffi::char_p::char_p_ref;

#[derive_ReprC]
#[ReprC::opaque]
#[derive(Debug)]
pub struct DatabaseConfig {
    pub raw: AnyDatabaseConfig,
}

#[ffi_export]
fn new_memory_config() -> Box<DatabaseConfig> {
    let memory_config = AnyDatabaseConfig::Memory(());
    Box::new(DatabaseConfig { raw: memory_config })
}

#[ffi_export]
fn new_sled_config(path: char_p_ref, tree_name: char_p_ref) -> Box<DatabaseConfig> {
    let path = path.to_string();
    let tree_name = tree_name.to_string();

    let sled_config = AnyDatabaseConfig::Sled(SledDbConfiguration { path, tree_name });
    Box::new(DatabaseConfig { raw: sled_config })
}

#[ffi_export]
fn free_database_config(database_config: Box<DatabaseConfig>) {
    drop(database_config);
}
