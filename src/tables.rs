use serde_json::{json, Map, Value};
use std::{io, path::Path};
use thiserror::Error;

use crate::{
    database::{Database, DatabaseError},
    DB_DIR,
};

pub struct Table<'a> {
    pub db: &'a str,
    pub table_name: &'a str,
}

#[derive(Debug, Error)]
pub enum TableError {
    #[error("DB Error")]
    DBError(#[from] DatabaseError),
    #[error("IO Error")]
    IoError(#[from] io::Error),
}

// Todo: Add table builder
impl<'a> Table<'a> {
    pub fn new(db: &'a str, table_name: &'a str) -> Self {
        Self { db, table_name }
    }
    pub fn create(&self, cols: &Vec<(&str, &str)>) -> Result<(), TableError> {
        // Should create the [tablename].schema.json

        let mut map = Map::new();

        cols.iter().for_each(|(key, value)| {
            map.insert(key.to_string(), Value::String(value.to_string()));
        });

        let fields = json!({ "fields": Value::Object(map) }).to_string();

        Database::exists_or_err(self.db)?;

        Ok(())
    }
}
