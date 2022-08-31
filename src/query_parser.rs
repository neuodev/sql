use std::collections::HashMap;

use regex::Regex;

use crate::regex::*;

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
    pub fn parse(query: &str) -> Result<Query, &'static str> {
        let re_db = Regex::new(RE_DB).unwrap();
        if let Some(caps) = re_db.captures(query) {
            let name = caps["name"].to_string();
            let action = &caps["action"];

            let action = match action.to_lowercase().as_str() {
                "create" => DatabaseAction::Create,
                "drop" => DatabaseAction::Drop,
                "use" => DatabaseAction::Use,
                _ => return Err("Invalid database action"),
            };

            return Ok(Query::Database { name, action });
        }

        let re_create_table = Regex::new(RE_CREATE_TABLE).unwrap();
        if let Some(caps) = re_create_table.captures(query) {
            let table_name = caps["name"].to_string();
            let re_entries = Regex::new(RE_TABLE_ENTRIES).unwrap();
            let mut cols = HashMap::new();
            re_entries.captures_iter(&caps["entries"]).for_each(|caps| {
                cols.insert(caps["col_name"].into(), caps["col_type"].into());
            });

            return Ok(Query::Table(TableQuery::Create { table_name, cols }));
        }

        let re_table = Regex::new(RE_TABLE).unwrap();

        if let Some(caps) = re_table.captures(query) {
            let table_name = caps["name"].to_string();
            match caps["action"].to_lowercase().as_str() {
                "drop" => return Ok(Query::Table(TableQuery::Drop(table_name))),
                "truncate" => return Ok(Query::Table(TableQuery::Truncate(table_name))),
                _ => {}
            };
        }

        Err("Invalid query.")
    }
}

#[cfg(test)]
mod tests {
    use crate::query_parser::{DatabaseAction, Query, TableQuery};

    use super::QueryParser;

    #[test]
    fn create_database() {
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
    fn drop_database() {
        let query = QueryParser::parse("DROP DATABASE demo").unwrap();

        assert_eq!(
            query,
            Query::Database {
                name: "demo".to_string(),
                action: DatabaseAction::Drop
            }
        );
    }

    #[test]
    fn use_database() {
        let query = QueryParser::parse("USE DATABASE demo").unwrap();

        assert_eq!(
            query,
            Query::Database {
                name: "demo".to_string(),
                action: DatabaseAction::Use
            }
        );
    }

    #[test]
    fn create_table_one_line_query() {
        let query = QueryParser::parse("CREATE TABLE user(id int, name varchar, age int)").unwrap();
        if let Query::Table(TableQuery::Create { table_name, cols }) = query {
            assert_eq!(table_name, "user".to_string());
            assert_eq!(cols.get("age").unwrap(), "int");
            assert_eq!(cols.get("name").unwrap(), "varchar");
            assert_eq!(cols.get("id").unwrap(), "int");
        } else {
            panic!("Unexpted query");
        }
    }

    #[test]
    fn create_table_multi_line_query() {
        let query = QueryParser::parse(
            r#"CREATE TABLE t_name (
                column1 datatype,
                column2 datatype,
                column3 datatype,
               );"#,
        )
        .unwrap();
        if let Query::Table(TableQuery::Create { table_name, cols }) = query {
            assert_eq!(table_name, "t_name".to_string());
            assert_eq!(cols.get("column1").unwrap(), "datatype");
            assert_eq!(cols.get("column2").unwrap(), "datatype");
            assert_eq!(cols.get("column3").unwrap(), "datatype");
        } else {
            panic!("Unexpted query");
        }
    }

    #[test]
    fn drop_table() {
        let query = QueryParser::parse(r#"DROP TABLE demo"#).unwrap();
        if let Query::Table(TableQuery::Drop(t_name)) = query {
            assert_eq!(t_name, "demo".to_string());
        } else {
            panic!("Unexpted query")
        }
    }

    #[test]
    fn truncate_table() {
        let query = QueryParser::parse(r#"TRUNCATE TABLE demo"#).unwrap();
        if let Query::Table(TableQuery::Truncate(t_name)) = query {
            assert_eq!(t_name, "demo".to_string());
        } else {
            panic!("Unexpted query")
        }
    }
}
