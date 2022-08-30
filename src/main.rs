mod database;
mod tables;
mod utils;

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
    Database::new(db_name)?;
    let users_table = Table::new(db_name, "users");
    let cols = vec![("name", "varchar"), ("age", "int")];
    users_table.create(&cols)?;
    Database::drop_db(db_name)?;

    Ok(())
}
