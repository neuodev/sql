use crate::{
    database::{Database, DatabaseError},
    query_parser::{DatabaseAction, Query, QueryParser, QueryParserError, TableQuery},
    tables::{Table, TableError},
};
use inquire::{validator::Validation, InquireError, Text};
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
        let keywords = include_str!("../mysql5.0_keywords.txt")
            .split("\n")
            .map(|k| k.trim().to_string())
            .collect::<Vec<_>>();

        let query_suggester = |q: &str| {
            if q.is_empty() {
                return Ok(vec![]);
            };
            Ok(keywords
                .clone()
                .into_iter()
                .filter(|keyword| keyword.to_lowercase().starts_with(q))
                .take(4)
                .collect::<Vec<_>>())
        };

        loop {
            let query = Text::new("Enter SQL Query")
                .with_placeholder("SELECT * FROM user")
                .with_page_size(200)
                .with_suggester(&query_suggester)
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
                DatabaseAction::Use => Database::use_db(&name)?,
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
