use crate::query_parser::{QueryParser, QueryParserError};
use inquire::{Editor, InquireError};
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
        let query = Editor::new("SQL query")
            .with_help_message("Enter SQL query")
            .with_predefined_text("SELECT * FROM user")
            .with_file_extension("sql")
            .prompt()?;

        QueryPlanner::execute_query(&query);
        Ok(())
    }

    fn execute_query(raw_query: &str) -> Result<(), QueryPlannerError> {
        let query = QueryParser::parse(raw_query)?;

        Ok(())
    }
}
