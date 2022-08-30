use std::path::{Path, PathBuf};

use crate::DB_DIR;

pub fn get_db_path(name: &str) -> PathBuf {
    let base_dir = Path::new(DB_DIR);
    base_dir.join(name)
}
