use regex::Regex;
use thiserror::Error;

use crate::{
    regex::*,
    types::{DataType, DataTypesErr},
    utils::{get_cols, get_comma_separated_values},
};

pub type ColName = String;

#[derive(Debug, PartialEq, Eq)]
pub enum DatabaseAction {
    Create,
    Drop,
    Use,
}

#[derive(Debug, PartialEq, Eq)]
pub enum TableQuery {
    Create {
        cols: Vec<String>,
        types: Vec<String>,
    },
    DropTable,
    Truncate,
    AddCol {
        col_name: String,
        datatype: DataType,
    },
    AlterCol {
        col_name: String,
        datatype: DataType,
    },
    DropCol(ColName),
    Select {
        cols: SelectCols,
        condition: Option<Condition>,
    },
    Insert {
        cols: SelectCols,
        values: Vec<Vec<String>>,
    },
    Delete {
        condition: Condition,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub enum SelectCols {
    All,
    Cols(Vec<String>),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Query {
    ShowAllDBs,
    ShowCurrDB,
    ShowTables,
    Database {
        name: String,
        action: DatabaseAction,
    },
    Table {
        name: String,
        query: TableQuery,
    },
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum QueryParserError {
    #[error("Failed to parse the query")]
    BadQuery(String),
    #[error("Invalid DB query")]
    InvalidDBAction(String),
    #[error("Invalid query")]
    InvalidTableAction(String),
    #[error("Invalid condition")]
    InvalidCondition(String),
    #[error("Invalid Operator")]
    InvalidOperator(String),
    #[error("Data type error")]
    DataTypeErr(#[from] DataTypesErr),
}

pub struct QueryParser;
impl QueryParser {
    pub fn parse(mut query: &str) -> Result<Query, QueryParserError> {
        query = query.trim();
        let re_show = Regex::new(RE_SHOW_QUERY).unwrap();

        if let Some(caps) = re_show.captures(query) {
            return match caps["query"].to_lowercase().as_str() {
                "databases" => Ok(Query::ShowAllDBs),
                "current database" => Ok(Query::ShowCurrDB),
                "tables" => Ok(Query::ShowTables),
                _ => Err(QueryParserError::BadQuery(query.to_string())),
            };
        }

        let re_db = Regex::new(RE_DB).unwrap();
        if let Some(caps) = re_db.captures(query) {
            let name = caps["name"].to_string();
            let action = &caps["action"];

            let action = match action.to_lowercase().as_str() {
                "create" => DatabaseAction::Create,
                "drop" => DatabaseAction::Drop,
                "use" => DatabaseAction::Use,
                _ => return Err(QueryParserError::InvalidDBAction(action.to_string())),
            };

            return Ok(Query::Database { name, action });
        }

        let re_create_table = Regex::new(RE_CREATE_TABLE).unwrap();
        if let Some(caps) = re_create_table.captures(query) {
            let table_name = caps["name"].to_string();
            let re_entries = Regex::new(RE_TABLE_ENTRIES).unwrap();
            let mut types = Vec::new();
            let mut cols = Vec::new();
            re_entries.captures_iter(&caps["entries"]).for_each(|caps| {
                types.push(caps["col_type"].to_string());
                cols.push(caps["col_name"].to_string())
            });

            return Ok(Query::Table {
                name: table_name,
                query: TableQuery::Create { cols, types },
            });
        }

        let re_table = Regex::new(RE_TABLE).unwrap();

        if let Some(caps) = re_table.captures(query) {
            let table_name = caps["name"].to_string();
            match caps["action"].to_lowercase().as_str() {
                "drop" => {
                    return Ok(Query::Table {
                        name: table_name,
                        query: TableQuery::DropTable,
                    })
                }
                "truncate" => {
                    return Ok(Query::Table {
                        name: table_name,
                        query: TableQuery::Truncate,
                    })
                }
                _ => {
                    return Err(QueryParserError::InvalidTableAction(
                        caps["action"].to_string(),
                    ))
                }
            };
        }

        let re_drop_col = Regex::new(RE_DROP_COL).unwrap();
        if let Some(caps) = re_drop_col.captures(query) {
            return Ok(Query::Table {
                name: caps["table_name"].to_string(),
                query: TableQuery::DropCol(caps["col_name"].to_string()),
            });
        }

        let re_alter_col = Regex::new(RE_ALTER_COL).unwrap();
        if let Some(caps) = re_alter_col.captures(query) {
            return Ok(Query::Table {
                name: caps["table_name"].to_string(),
                query: TableQuery::AlterCol {
                    col_name: caps["col_name"].to_string(),
                    datatype: DataType::parse(&caps["datatype"])?,
                },
            });
        }

        let re_add_col = Regex::new(RE_ADD_COL).unwrap();
        if let Some(caps) = re_add_col.captures(query) {
            return Ok(Query::Table {
                name: caps["table_name"].to_string(),
                query: TableQuery::AddCol {
                    col_name: caps["col_name"].to_string(),
                    datatype: DataType::parse(&caps["datatype"])?,
                },
            });
        }

        let re_select = Regex::new(RE_SELECT).unwrap();
        if let Some(caps) = re_select.captures(query) {
            let condition = caps.name("condition").map(|_| &caps["condition"]);

            return Ok(Query::Table {
                name: caps["table_name"].to_string(),
                query: TableQuery::Select {
                    condition: match condition {
                        None => None,
                        Some(c) => Some(Condition::parse(c)?),
                    },
                    cols: get_cols(&caps["cols"]),
                },
            });
        }

        let re_insert = Regex::new(RE_INSERT).unwrap();
        if let Some(caps) = re_insert.captures(query) {
            let cols = match caps.name("cols") {
                Some(_) => SelectCols::Cols(get_comma_separated_values(&caps["cols"])),
                None => SelectCols::All,
            };

            let re_values = Regex::new(RE_INSERT_VALUES_VALUES).unwrap();
            let values = re_values
                .captures_iter(&caps["values"])
                .map(|caps| get_comma_separated_values(&caps["row"]))
                .collect::<Vec<Vec<_>>>();

            return Ok(Query::Table {
                name: caps["table_name"].to_string(),
                query: TableQuery::Insert { cols, values },
            });
        }

        let re_delete = Regex::new(RE_DELETE_FROM_TABLE).unwrap();
        if let Some(caps) = re_delete.captures(query) {
            let condition = Condition::parse(&caps["condition"])?;
            return Ok(Query::Table {
                name: caps["table_name"].to_string(),
                query: TableQuery::Delete { condition },
            });
        }

        Err(QueryParserError::BadQuery(query.to_string()))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Operator {
    Eq,
    NotEq,
    Gt,
    Lt,
    GtEq,
    LtEq,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Condition {
    pub key: String,
    pub value: String,
    pub operator: Operator,
}

impl Condition {
    fn parse(query: &str) -> Result<Condition, QueryParserError> {
        let re = Regex::new(RE_KEY_VALUE).unwrap();

        match re.captures(query) {
            Some(caps) => {
                let operator = match &caps["operator"] {
                    "=" => Operator::Eq,
                    "!=" => Operator::NotEq,
                    ">" => Operator::Gt,
                    ">=" => Operator::GtEq,
                    "<" => Operator::Lt,
                    "<=" => Operator::LtEq,
                    _ => {
                        return Err(QueryParserError::InvalidOperator(
                            caps["operator"].to_string(),
                        ))
                    }
                };

                Ok(Condition {
                    key: caps["key"].to_string(),
                    value: caps["value"].to_string(),
                    operator,
                })
            }
            None => Err(QueryParserError::InvalidCondition(query.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        query_parser::{Condition, DatabaseAction, Operator, Query, SelectCols, TableQuery},
        types::DataType,
    };

    use super::{QueryParser, QueryParserError};

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
        if let Query::Table {
            name,
            query: TableQuery::Create { cols, types },
        } = query
        {
            assert_eq!(name, "user".to_string());
            assert_eq!(cols[0], "id".to_string());
            assert_eq!(cols[1], "name".to_string());
            assert_eq!(cols[2], "age".to_string());
            assert_eq!(types[0], "int".to_string());
            assert_eq!(types[1], "varchar".to_string());
            assert_eq!(types[2], "int".to_string());
        } else {
            panic!("Unexpected query");
        }
    }

    #[test]
    fn create_table_multi_line_query() {
        let query = QueryParser::parse(
            r#"CREATE TABLE t_name (
                column1 datatype1,
                column2 datatype2,
                column3 datatype3,
               );"#,
        )
        .unwrap();
        if let Query::Table {
            name,
            query: TableQuery::Create { cols, types },
        } = query
        {
            assert_eq!(name, "t_name".to_string());
            assert_eq!(cols[0], "column1".to_string());
            assert_eq!(cols[1], "column2".to_string());
            assert_eq!(cols[2], "column3".to_string());
            assert_eq!(types[0], "datatype1".to_string());
            assert_eq!(types[1], "datatype2".to_string());
            assert_eq!(types[2], "datatype3".to_string());
        } else {
            panic!("Unexpected query");
        }
    }

    #[test]
    fn drop_table() {
        let query = QueryParser::parse(r#"DROP TABLE demo"#).unwrap();
        if let Query::Table {
            name,
            query: TableQuery::DropTable,
        } = query
        {
            assert_eq!(name, "demo".to_string());
        } else {
            panic!("Unexpected query")
        }
    }

    #[test]
    fn truncate_table() {
        let query = QueryParser::parse(r#"TRUNCATE TABLE demo"#).unwrap();
        if let Query::Table {
            name,
            query: TableQuery::Truncate,
        } = query
        {
            assert_eq!(name, "demo".to_string());
        } else {
            panic!("Unexpted query")
        }
    }

    #[test]
    fn drop_col() {
        let query = QueryParser::parse("ALTER TABLE demo DROP COLUMN id").unwrap();

        if let Query::Table {
            name,
            query: TableQuery::DropCol(col),
        } = query
        {
            assert_eq!(name, "demo".to_string());
            assert_eq!(col, "id".to_string());
        } else {
            panic!("Unexpted query")
        }
    }

    #[test]
    fn alter_col() {
        let query = QueryParser::parse("ALTER TABLE demo ALTER COLUMN id int").unwrap();

        if let Query::Table {
            name,
            query: TableQuery::AlterCol { col_name, datatype },
        } = query
        {
            assert_eq!(name, "demo".to_string());
            assert_eq!(col_name, "id".to_string());
            assert_eq!(datatype, DataType::INT);
        } else {
            panic!("Unexpted query")
        }
    }

    #[test]
    fn add_col() {
        let query = QueryParser::parse("ALTER TABLE demo ADD id TEXT").unwrap();

        if let Query::Table {
            name,
            query: TableQuery::AddCol { col_name, datatype },
        } = query
        {
            assert_eq!(name, "demo".to_string());
            assert_eq!(col_name, "id".to_string());
            assert_eq!(datatype, DataType::TEXT);
        } else {
            panic!("Unexpted query")
        }
    }

    #[test]
    fn parse_select_statment_with_condition() {
        let query = QueryParser::parse("SELECT id,name FROM user WHERE age >= 12").unwrap();

        if let Query::Table {
            name,
            query: TableQuery::Select { cols, condition },
        } = query
        {
            assert_eq!(name, "user".to_string());
            assert_eq!(cols, SelectCols::Cols(vec!["id".into(), "name".into()]));
            assert!(condition.is_some());
            assert_eq!(
                condition.unwrap(),
                Condition {
                    key: "age".into(),
                    value: "12".into(),
                    operator: Operator::GtEq
                }
            );
        } else {
            panic!("Unexpected query")
        }
    }

    #[test]
    fn parse_select_statment_with_all_cols_and_condition() {
        let query = QueryParser::parse("SELECT * FROM user WHERE age=12").unwrap();

        if let Query::Table {
            name,
            query: TableQuery::Select { cols, condition },
        } = query
        {
            assert_eq!(name, "user".to_string());
            assert_eq!(cols, SelectCols::All);
            assert!(condition.is_some());
            assert_eq!(
                condition.unwrap(),
                Condition {
                    key: "age".into(),
                    value: "12".into(),
                    operator: Operator::Eq
                }
            );
        } else {
            panic!("Unexpected query")
        }
    }

    #[test]
    fn parse_select_statment() {
        let query = QueryParser::parse("SELECT id,name FROM user").unwrap();

        if let Query::Table {
            name,
            query: TableQuery::Select { cols, condition },
        } = query
        {
            assert_eq!(name, "user".to_string());
            assert_eq!(cols, SelectCols::Cols(vec!["id".into(), "name".into()]));
            assert!(condition.is_none());
        } else {
            panic!("Unexpected query")
        }
    }

    #[test]
    fn insert_statment_with_no_cols_and_one_value() {
        let query = QueryParser::parse("INSERT INTO table_name VALUES (value1, value2);").unwrap();

        if let Query::Table {
            name,
            query: TableQuery::Insert { cols, values },
        } = query
        {
            assert_eq!(name, "table_name".to_string());
            assert_eq!(cols, SelectCols::All);
            assert_eq!(
                values,
                vec![vec!["value1".to_string(), "value2".to_string()]]
            );
        } else {
            panic!("Unexpected query")
        }
    }

    #[test]
    fn insert_statment_with_no_cols_and_many_value() {
        let query = QueryParser::parse(
            "INSERT INTO user VALUES (val1, val2), (val3, val4) (val5, val6);
        ",
        )
        .unwrap();

        let expected_values = vec![
            vec!["val1".to_string(), "val2".to_string()],
            vec!["val3".to_string(), "val4".to_string()],
            vec!["val5".to_string(), "val6".to_string()],
        ];

        if let Query::Table {
            name,
            query: TableQuery::Insert { cols, values },
        } = query
        {
            assert_eq!(name, "user".to_string());
            assert_eq!(cols, SelectCols::All);
            assert_eq!(values, expected_values);
        } else {
            panic!("Unexpected query")
        }
    }

    #[test]
    fn insert_statment_with_cols_and_many_value() {
        let query = QueryParser::parse(
            "INSERT INTO table_name (a,b) VALUES(1,2),(3, 4),(5, 6);
        ",
        )
        .unwrap();

        let expected_values = vec![
            vec!["1".to_string(), "2".to_string()],
            vec!["3".to_string(), "4".to_string()],
            vec!["5".to_string(), "6".to_string()],
        ];

        if let Query::Table {
            name,
            query: TableQuery::Insert { cols, values },
        } = query
        {
            assert_eq!(name, "table_name".to_string());
            assert_eq!(
                cols,
                SelectCols::Cols(vec!["a".to_string(), "b".to_string()])
            );
            assert_eq!(values, expected_values);
        } else {
            panic!("Unexpected query")
        }
    }

    #[test]
    fn delete_from_table() {
        let query = QueryParser::parse(
            "DELETE FROM table_name WHERE name = jone;
        ",
        )
        .unwrap();

        if let Query::Table {
            name,
            query: TableQuery::Delete { condition },
        } = query
        {
            assert_eq!(name, "table_name".to_string());
            assert_eq!(
                condition,
                Condition {
                    key: "name".into(),
                    value: "jone".into(),
                    operator: Operator::Eq
                }
            );
        } else {
            panic!("Unexpected query")
        }
    }

    #[test]
    fn show_queries() {
        let show_dbs = QueryParser::parse("SHOW DATABASES").unwrap();
        let show_curr_db = QueryParser::parse("SHOW CURRENT DATABASE").unwrap();
        let show_tables = QueryParser::parse("SHOW TABLES").unwrap();

        assert_eq!(show_dbs, Query::ShowAllDBs);
        assert_eq!(show_curr_db, Query::ShowCurrDB);
        assert_eq!(show_tables, Query::ShowTables);
    }

    #[test]
    fn parse_eq_condition() {
        let con = Condition::parse("name = jone").unwrap();

        assert_eq!(
            con,
            Condition {
                key: "name".into(),
                value: "jone".into(),
                operator: Operator::Eq
            }
        )
    }

    #[test]
    fn parse_less_than_or_equal_condition() {
        let con = Condition::parse("age <= 21").unwrap();
        assert_eq!(
            con,
            Condition {
                key: "age".into(),
                value: "21".into(),
                operator: Operator::LtEq
            }
        )
    }

    #[test]
    fn parse_invalid_condition() {
        let con = Condition::parse("age !! 21");
        assert_eq!(con, Err(QueryParserError::InvalidOperator("!!".into())));
    }
}
