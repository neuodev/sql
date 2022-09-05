use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{collections::HashMap, f32::consts::E, fs, io, ops::Index};
use thiserror::Error;

use crate::{
    database::{Database, DatabaseError},
    query_parser::SelectCols,
    utils::{get_db_path, get_schema_path, get_table_path},
};

pub type TableEntries = Vec<HashMap<String, String>>;

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
    TableNotFond(String),
    #[error("Column not found")]
    ColNotFound(String),
    #[error("Column type  not found")]
    ColTypeNotFound(String),
}

type TableResult<T> = Result<T, TableError>;

// Todo: Add table builder
impl<'a> Table<'a> {
    pub fn new(db: &'a str, table_name: &'a str) -> TableResult<Self> {
        Database::exists_or_err(db)?;
        Ok(Self { db, table_name })
    }

    pub fn create(&self, cols: Vec<String>, types: Vec<String>) -> TableResult<()> {
        let schema = json!({ "cols": cols, "types": types });
        let schema = serde_json::to_string_pretty(&schema)?;

        Database::exists_or_err(self.db)?;

        let db_path = get_db_path(self.db);
        let schema_file = db_path.join(format!("{}.schema.json", self.table_name));
        let table_file = db_path.join(format!("{}.json", self.table_name));
        fs::write(schema_file, schema.as_bytes())?;
        fs::write(table_file, "[]")?;
        Ok(())
    }

    pub fn insert(&self, cols: SelectCols, values: Vec<Vec<String>>) -> TableResult<()> {
        Database::exists_or_err(self.db)?;

        let cols = match cols {
            SelectCols::Cols(cols) => cols,
            SelectCols::All => self.read_schema()?.cols,
        };

        let new_entries = values
            .into_iter()
            .map(|row| {
                assert_eq!(row.len(), cols.len());
                let mut map = HashMap::new();
                cols.iter().zip(row).for_each(|(k, v)| {
                    map.insert(k.clone(), v);
                });

                map
            })
            .collect::<Vec<HashMap<_, _>>>();

        let mut all_entries = self.read()?;
        all_entries.extend(new_entries);
        println!(
            "[{}@{}] {:?} entry",
            self.table_name,
            self.db,
            all_entries.len()
        );
        self.write(&all_entries)?;
        Ok(())
    }

    pub fn select(&self, cols: SelectCols, condition: Option<String>) -> TableResult<TableEntries> {
        dbg!(&condition);
        let all_entries = self.read()?;

        let entries = all_entries
            .into_iter()
            .map(|entry| match &cols {
                SelectCols::All => entry,
                SelectCols::Cols(selectd_cols) => {
                    let mut map = HashMap::new();
                    selectd_cols.into_iter().for_each(|col| {
                        map.insert(col.clone(), entry.get(col.trim()).unwrap().clone());
                    });
                    map
                }
            })
            .collect::<Vec<HashMap<_, _>>>();

        Ok(entries)
    }

    pub fn alter<N: Into<String> + Copy + PartialEq, T: Into<String>>(
        &self,
        col_name: N,
        datatype: T,
    ) -> TableResult<()> {
        // Todo: Update the actual table
        // Update schema
        self.exists_or_err()?;
        let mut schema = self.read_schema()?;
        let p = schema.cols.iter().position(|c| c == &col_name.into());

        match p {
            None => Err(TableError::ColNotFound(col_name.into())),
            Some(pos) => match schema.types.get(pos) {
                None => Err(TableError::ColTypeNotFound(col_name.into())),
                Some(_) => {
                    schema.types[pos] = datatype.into();
                    self.write_schema(schema)?;

                    Ok(())
                }
            },
        }
    }

    pub fn drop(&self) -> TableResult<()> {
        self.exists_or_err()?;

        let schema = get_schema_path(self);
        let table = get_table_path(self);

        fs::remove_file(schema)?;
        fs::remove_file(table)?;

        Ok(())
    }

    pub fn truncate(&self) -> Result<(), TableError> {
        self.write(&vec![])?;
        Ok(())
    }

    pub fn add_col<N: Into<String>, T: Into<String>>(
        &self,
        col_name: N,
        datatype: T,
    ) -> TableResult<()> {
        // todo: Every column should be unique
        // TODO: Add the new column to the data with the default value of this type
        let mut schema = self.read_schema()?;
        schema.cols.push(col_name.into());
        schema.types.push(datatype.into());

        debug_assert_eq!(schema.cols.len(), schema.types.len());
        self.write_schema(schema)?;
        Ok(())
    }

    pub fn remove_col<T: Into<String> + Copy>(&self, col_name: T) -> TableResult<()> {
        // Todo: Col should be removed from the table
        let mut schema = self.read_schema()?;
        let pos = schema.cols.iter().position(|c| c == &col_name.into());

        match pos {
            Some(pos) => {
                schema.cols.remove(pos);
                schema.types.remove(pos);

                debug_assert_eq!(schema.cols.len(), schema.types.len());
                self.write_schema(schema)?;
                Ok(())
            }
            None => Err(TableError::ColNotFound(col_name.into())),
        }
    }

    fn read(&self) -> Result<TableEntries, TableError> {
        self.exists_or_err()?;
        let table = get_table_path(self);

        let content = fs::read_to_string(table)?;

        Ok(serde_json::from_str(&content)?)
    }

    fn write(&self, entries: &TableEntries) -> TableResult<()> {
        self.exists_or_err()?;
        let table = get_table_path(self);
        let entries = json!(entries);
        fs::write(table, entries.to_string())?;
        Ok(())
    }

    fn read_schema(&self) -> TableResult<Schema> {
        self.exists_or_err()?;
        let schema = get_schema_path(self);

        let content = fs::read_to_string(schema)?;

        Ok(serde_json::from_str(&content)?)
    }

    fn write_schema(&self, schema: Schema) -> TableResult<()> {
        self.exists_or_err()?;
        let path = get_schema_path(self);
        let schema = json!(schema);
        fs::write(path, serde_json::to_string_pretty(&schema)?)?;
        Ok(())
    }

    fn exist(&self) -> bool {
        let schema = get_schema_path(self);
        let table = get_table_path(self);
        schema.exists() && table.exists()
    }

    fn exists_or_err(&self) -> TableResult<()> {
        Database::exists_or_err(self.db)?;

        if !self.exist() {
            Err(TableError::TableNotFond(self.table_name.to_string()))
        } else {
            Ok(())
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Schema {
    cols: Vec<String>,
    types: Vec<String>,
}
