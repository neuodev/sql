mod database;
mod query_parser;
mod query_planner;
mod regex;
mod tables;
mod utils;

use database::DatabaseError;
use inquire::InquireError;
use query_planner::{QueryPlanner, QueryPlannerError};
use tables::TableError;
use thiserror::Error;

pub const DB_DIR: &str = "./sql";

#[derive(Debug, Error)]
enum ErrorWrapper {
    #[error("DB Error")]
    DatabaseError(#[from] DatabaseError),
    #[error("Table Error")]
    TableError(#[from] TableError),
    #[error("Error while getting std input")]
    InputError(#[from] InquireError),
    #[error("Unable to excude the query")]
    QueryPlanner(#[from] QueryPlannerError),
}

fn main() -> Result<(), ErrorWrapper> {
    QueryPlanner::new()?;
    Ok(())
}
