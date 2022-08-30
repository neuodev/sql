use serde_json::json;
use std::{collections::HashMap, fs, io};
use thiserror::Error;

use crate::{
    database::{Database, DatabaseError},
    utils::{get_db_path, get_schema_path, get_table_path, schema_file, table_file},
};

type TableEntries = Vec<HashMap<String, String>>;

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
    #[error("Table not found")]
    NotFond(String),
}

// Todo: Add table builder
impl<'a> Table<'a> {
    pub fn new(db: &'a str, table_name: &'a str) -> Result<Self, TableError> {
        Database::exists_or_err(db)?;
        Ok(Self { db, table_name })
    }

    pub fn create(&self, cols: &HashMap<&str, &str>) -> Result<(), TableError> {
        let fields = json!({ "fields": cols });
        let fields = serde_json::to_string_pretty(&fields)?;

        Database::exists_or_err(self.db)?;

        let db_path = get_db_path(self.db);
        let schema_file = db_path.join(format!("{}.schema.json", self.table_name));
        let table_file = db_path.join(format!("{}.json", self.table_name));
        fs::write(schema_file, fields.as_bytes())?;
        fs::write(table_file, "[]")?;
        Ok(())
    }

    pub fn insert(&self, entries: &TableEntries) -> Result<(), TableError> {
        Database::exists_or_err(self.db)?;
        let mut all_entries = self.read()?;
        println!("[{}@{}] {:?}", self.table_name, self.db, all_entries.len());
        all_entries.extend(entries.clone());
        self.write(&all_entries)?;
        Ok(())
    }

    fn read(&self) -> Result<TableEntries, TableError> {
        self.exists_or_err()?;
        let table = get_table_path(self);

        let content = fs::read_to_string(table)?;

        Ok(serde_json::from_str(&content)?)
    }

    fn write(&self, entries: &TableEntries) -> Result<(), TableError> {
        self.exists_or_err()?;
        let table = get_table_path(self);
        let entries = json!(entries);
        fs::write(table, entries.to_string())?;
        Ok(())
    }

    fn exist(&self) -> bool {
        let schema = get_schema_path(self);
        let table = get_table_path(self);
        schema.exists() && table.exists()
    }

    fn exists_or_err(&self) -> Result<(), TableError> {
        Database::exists_or_err(self.db)?;

        if !self.exist() {
            Err(TableError::NotFond(self.table_name.to_string()))
        } else {
            Ok(())
        }
    }
}
