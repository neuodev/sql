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
    Database::new("stats")?;

    Ok(())
}
