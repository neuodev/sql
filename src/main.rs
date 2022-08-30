use database::{Database, DatabaseError};

mod database;
mod tables;

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
