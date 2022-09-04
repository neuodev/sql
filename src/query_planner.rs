use std::ffi::OsStr;

use crate::query_parser::{DatabaseAction, Query, QueryParser, QueryParserError, TableQuery};
use inquire::{
    validator::{StringValidator, Validation},
    Editor, InquireError,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum QueryPlannerError {
    #[error("Unabel to read from stdin")]
    InputError(#[from] InquireError),
    #[error("Error while parsing the query")]
    QueryError(#[from] QueryParserError),
}

pub struct QueryPlanner;
impl QueryPlanner {
    pub fn new() -> Result<(), QueryPlannerError> {
        loop {
            let query = Editor::new("SQL query")
                .with_help_message("Enter SQL query")
                .with_file_extension("sql")
                .with_validator(|q: &str| {
                    if q.is_empty() {
                        Ok(Validation::Invalid("Empty query".into()))
                    } else {
                        Ok(Validation::Valid)
                    }
                })
                .prompt();

            if let Err(e) = query {
                eprintln!("{:?}", e);
                continue;
            }

            if let Err(e) = QueryPlanner::execute_query(&query.unwrap()) {
                eprintln!("{:?}", e);
            }
        }
    }

    fn execute_query(raw_query: &str) -> Result<(), QueryPlannerError> {
        let query = QueryParser::parse(raw_query)?;

        println!("[Query] {:?}", query);

        match query {
            Query::Database { name, action } => match action {
                DatabaseAction::Create => {}
                DatabaseAction::Drop => {}
                DatabaseAction::Use => {}
            },
            Query::Table { name, query } => match query {
                TableQuery::Create { cols } => {}
                TableQuery::DropTable => {}
                TableQuery::Truncate => {}
                TableQuery::DropCol(col) => {}
                TableQuery::AlterCol { col_name, datatype } => {}
                TableQuery::AddCol { col_name, datatype } => {}
                TableQuery::Select { cols, condition } => {}
                TableQuery::Insert { cols, values } => {}
                TableQuery::Delete { condition } => {}
            },
        }

        Ok(())
    }
}
