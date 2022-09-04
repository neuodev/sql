use std::ffi::OsStr;

use crate::{
    database::{Database, DatabaseError},
    query_parser::{DatabaseAction, Query, QueryParser, QueryParserError, TableQuery},
    tables::{Table, TableError},
};
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
    #[error("DB Error")]
    DatabaseError(#[from] DatabaseError),
    #[error("Table Error")]
    TableError(#[from] TableError),
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
                DatabaseAction::Create => Database::new(&name)?,
                DatabaseAction::Drop => Database::drop(&name)?,
                DatabaseAction::Use => todo!(),
            },
            Query::Table { name, query } => {
                let table = Table::new("stats", &name)?;
                match query {
                    TableQuery::Create { cols } => table.create(&cols)?,
                    TableQuery::DropTable => table.drop()?,
                    TableQuery::Truncate => table.truncate()?,
                    TableQuery::DropCol(col) => todo!(),
                    TableQuery::AlterCol { col_name, datatype } => todo!(),
                    TableQuery::AddCol { col_name, datatype } => todo!(),
                    TableQuery::Select { cols, condition } => todo!(),
                    TableQuery::Insert { cols, values } => todo!(),
                    TableQuery::Delete { condition } => todo!(),
                }
            }
        };

        Ok(())
    }
}
