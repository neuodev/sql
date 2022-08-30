use crate::DB_DIR;
use std::{fs, io, path::Path};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Duplicated database")]
    DuplicatedDB(String),
    #[error("Failed to create new database")]
    IoError(#[from] io::Error),
}

pub struct Database;
impl Database {
    pub fn new(name: &str) -> Result<(), DatabaseError> {
        let base_dir = Path::new(DB_DIR);
        let db_dir = base_dir.join(name);

        if db_dir.exists() {
            return Err(DatabaseError::DuplicatedDB(name.to_string()));
        }
        fs::create_dir_all(db_dir)?;
        Ok(())
    }
}
