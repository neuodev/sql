mod database;
mod tables;
mod utils;

use database::{Database, DatabaseError};

use thiserror::Error;

pub const DB_DIR: &str = ".sql";

#[derive(Debug, Error)]
enum ErrorWrapper {
    #[error("DB Error")]
    DatabaseError(#[from] DatabaseError),
}

fn main() -> Result<(), ErrorWrapper> {
    let db_name = "stats";
    Database::drop_db(db_name)?;
    Database::new(db_name)?;

    Ok(())
}
