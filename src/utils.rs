use std::path::{Path, PathBuf};

use crate::DB_DIR;

pub fn get_db_path(name: &str) -> PathBuf {
    let base_dir = Path::new(DB_DIR);
    let db_dir = base_dir.join(name);

    db_dir
}

pub fn schema_file(file: &str) -> String {
    format!("{}.schema.json", file)
}

pub fn table_file(file: &str) -> String {
    format!("{}.json", file)
}

pub fn get_schema_path(db: &str, table: &str) -> PathBuf {
    let db_dir = get_db_path(db);
    db_dir.join(schema_file(table))
}

pub fn get_table_path(db: &str, table: &str) -> PathBuf {
    let db_dir = get_db_path(db);
    db_dir.join(table_file(table))
}
