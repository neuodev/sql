// use crate::{utils::get_db_path, DB_DIR};
use std::{fs, io, path::Path};
use thiserror::Error;

use crate::{utils::get_db_path, DB_DIR};

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Duplicated database")]
    DuplicatedDB(String),
    #[error("IO Error")]
    IoError(#[from] io::Error),
    #[error("Database not found")]
    NotFound(String),
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

    pub fn drop_db(name: &str) -> Result<(), DatabaseError> {
        let base_dir = Path::new(DB_DIR);
        let db_dir = base_dir.join(name);

        if !db_dir.exists() {
            return Err(DatabaseError::NotFound(name.to_string()));
        }
        fs::remove_dir(db_dir)?;
        Ok(())
    }

    pub fn exists(name: &str) -> bool {
        let path = get_db_path(name);
        path.exists()
    }

    pub fn exists_or_err(name: &str) -> Result<(), DatabaseError> {
        if !Database::exists(name) {
            Err(DatabaseError::NotFound(name.to_string()))
        } else {
            Ok(())
        }
    }
}
