mod database;
mod tables;
mod utils;

use std::collections::HashMap;

use database::{Database, DatabaseError};

use tables::{Table, TableError};
use thiserror::Error;

pub const DB_DIR: &str = "./sql";

#[derive(Debug, Error)]
enum ErrorWrapper {
    #[error("DB Error")]
    DatabaseError(#[from] DatabaseError),
    #[error("TAble Error")]
    TableError(#[from] TableError),
}

fn main() -> Result<(), ErrorWrapper> {
    let db_name = "stats";
    // Database::new(db_name)?;
    let users_table = Table::new(db_name, "users")?;
    let mut cols = HashMap::new();
    cols.insert("name", "varchar");
    cols.insert("id", "int");
    cols.insert("age", "int");
    users_table.create(&cols)?;
    // Database::drop_db(db_name)?;

    Ok(())
}
