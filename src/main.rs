mod database;
mod query_parser;
mod query_planner;
mod regex;
mod tables;
mod utils;

use query_planner::{QueryPlanner, QueryPlannerError};
use thiserror::Error;

pub const DB_DIR: &str = "./sql";

#[derive(Debug, Error)]
enum ErrorWrapper {
    #[error("Unable to excude the query")]
    QueryPlanner(#[from] QueryPlannerError),
}

fn main() -> Result<(), ErrorWrapper> {
    QueryPlanner::new()?;
    Ok(())
}
