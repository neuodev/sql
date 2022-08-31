use std::collections::HashMap;

use regex::Regex;

use crate::regex::DB_REGEX;

pub type TableName = String;
pub type ColName = String;
pub type ColType = String; // Todo: Should be an enum

#[derive(Debug, PartialEq, Eq)]
pub enum DatabaseAction {
    Create,
    Drop,
    Use,
}

#[derive(Debug, PartialEq, Eq)]
pub enum TableQuery {
    Create {
        table_name: String,
        cols: HashMap<String, String>,
    },
    Drop(TableName),
    Truncate(TableName),
    AddCol {
        col_name: String,
        col_type: ColType,
    },
    DropCol(ColName),
    AlterCol {
        col_name: String,
        col_type: ColType,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub enum Query {
    Database {
        name: String,
        action: DatabaseAction,
    },
    Table(TableQuery),
    Select {
        table_name: String,
        cols: Option<Vec<String>>,
    },
    Insert {
        table_name: String,
        cols: Option<Vec<String>>,
        values: Vec<String>,
    },
}

pub struct QueryParser;

impl QueryParser {
    pub fn parse(raw: &str) -> Result<Query, &'static str> {
        let re_db = Regex::new(DB_REGEX).unwrap();
        if let Some(caps) = re_db.captures(raw) {
            let db_name = caps["name"].to_string();
            let action = &caps["action"];

            let action = match action.to_lowercase().as_str() {
                "create" => DatabaseAction::Create,
                "drop" => DatabaseAction::Drop,
                "use" => DatabaseAction::Use,
                _ => return Err("Invalid database action"),
            };

            return Ok(Query::Database {
                name: caps["name"].to_string(),
                action,
            });
        }

        Err("Invalid query.")
    }
}

#[cfg(test)]
mod tests {
    use crate::query_parser::{DatabaseAction, Query};

    use super::QueryParser;

    #[test]
    fn create_table() {
        let all_caps = QueryParser::parse("CREATE DATABASE demo").unwrap();
        let all_lowercase = QueryParser::parse("create database demo").unwrap();

        assert_eq!(
            all_caps,
            Query::Database {
                name: "demo".to_string(),
                action: DatabaseAction::Create
            }
        );

        assert_eq!(
            all_lowercase,
            Query::Database {
                name: "demo".to_string(),
                action: DatabaseAction::Create
            }
        );
    }

    #[test]
    fn drop_table() {
        let query = QueryParser::parse("DROP DATABASE demo").unwrap();

        assert_eq!(
            query,
            Query::Database {
                name: "demo".to_string(),
                action: DatabaseAction::Drop
            }
        );
    }
}
