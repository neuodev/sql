use serde_json::{json, Map, Value};
use std::{fs, io, path::Path};
use thiserror::Error;

use crate::{
    database::{Database, DatabaseError},
    utils::get_db_path,
    DB_DIR,
};

pub struct Table<'a> {
    pub db: &'a str,
    pub table_name: &'a str,
}

#[derive(Debug, Error)]
pub enum TableError {
    #[error("DB Error")]
    DBErr(#[from] DatabaseError),
    #[error("IO Error")]
    IoErr(#[from] io::Error),
    #[error("Invalid JSON")]
    SerializationErr(#[from] serde_json::Error),
}

// Todo: Add table builder
impl<'a> Table<'a> {
    pub fn new(db: &'a str, table_name: &'a str) -> Result<Self, TableError> {
        Database::exists_or_err(db)?;
        Ok(Self { db, table_name })
    }
    pub fn create(&self, cols: &Vec<(&str, &str)>) -> Result<(), TableError> {
        // Should create the [tablename].schema.json

        let mut map = Map::new();

        cols.iter().for_each(|(key, value)| {
            map.insert(key.to_string(), Value::String(value.to_string()));
        });

        let fields = json!({ "fields": Value::Object(map) });
        let fields = serde_json::to_string_pretty(&fields)?;

        Database::exists_or_err(self.db)?;

        let db_path = get_db_path(self.db);
        let schema_file = db_path.join(format!("{}.schema.json", self.table_name));

        fs::write(schema_file, fields.as_bytes())?;

        Ok(())
    }
}
