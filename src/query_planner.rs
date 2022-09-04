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

        let query_suggester = |input: &str| {
            if input.is_empty() {
                return Ok(vec![]);
            };

            let input_tokens = input.split(" ").collect::<Vec<_>>();
            let num_of_tokens = input_tokens.len();

            if num_of_tokens == 0 || input_tokens[num_of_tokens - 1].is_empty() {
                return Ok(vec![]);
            }

            let q = input_tokens[num_of_tokens - 1];

            Ok(keywords
                .clone()
                .into_iter()
                .filter(|keyword| keyword.starts_with(&q.to_uppercase()))
                .take(4)
                .map(|k| {
                    let mut as_string = (&input_tokens[0..num_of_tokens - 1]).to_vec().join(" ");
                    as_string.push(' ');
                    as_string.push_str(&k);

                    as_string
                })
                .collect::<Vec<_>>())
        };

        loop {
            let query = Text::new("sql #>")
                .with_placeholder("SELECT * FROM ...")
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
        let curr_db = Database::get_curr_db()?;
        match query {
            Query::Database { name, action } => match action {
                DatabaseAction::Create => Database::new(&name)?,
                DatabaseAction::Drop => Database::drop(&name)?,
                DatabaseAction::Use => Database::use_db(&name)?,
            },
            Query::Table { name, query } => {
                let table = Table::new(&curr_db, &name)?;
                match query {
                    TableQuery::Create { cols } => table.create(&cols)?,
                    TableQuery::DropTable => table.drop()?,
                    TableQuery::Truncate => table.truncate()?,
                    TableQuery::DropCol(col) => table.remove_col(col)?,
                    TableQuery::AlterCol { col_name, datatype } => {
                        table.alter(&col_name, datatype)?
                    }
                    TableQuery::AddCol { col_name, datatype } => {
                        table.add_col(col_name, datatype)?
                    }
                    TableQuery::Select { cols, condition } => todo!(),
                    TableQuery::Insert { cols, values } => table.insert(cols, values)?,
                    TableQuery::Delete { condition } => todo!(),
                }
            }
            Query::ShowAllDBs => Database::get_dbs()?.iter().for_each(|db| {
                println!("{}", db);
            }),
            Query::ShowCurrDB => {
                println!("Current DB: {}", curr_db);
            }
            Query::ShowTables => Database::get_db_tables(&curr_db)?.iter().for_each(|t| {
                println!("{}", t);
            }),
        };

        Ok(())
    }
}
