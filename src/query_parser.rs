use std::collections::HashMap;

use regex::Regex;

use crate::{
    regex::*,
    utils::{getCols, get_comma_separated_values},
};

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
        cols: HashMap<String, String>,
    },
    DropTable,
    Truncate,
    AddCol {
        col_name: String,
        datatype: ColType,
    },
    AlterCol {
        col_name: String,
        datatype: ColType,
    },
    DropCol(ColName),
    Select {
        cols: SelectCols,
        condition: Option<String>,
    },
    Insert {
        cols: SelectCols,
        values: Vec<Vec<String>>,
    },
    Delete {
        condition: String,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub enum SelectCols {
    All,
    Cols(Vec<String>),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Query {
    Database {
        name: String,
        action: DatabaseAction,
    },
    Table {
        name: String,
        query: TableQuery,
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

            return Ok(Query::Table {
                name: table_name,
                query: TableQuery::Create { cols },
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
                _ => {}
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
                    datatype: caps["datatype"].to_string(),
                },
            });
        }

        let re_add_col = Regex::new(RE_ADD_COL).unwrap();
        if let Some(caps) = re_add_col.captures(query) {
            return Ok(Query::Table {
                name: caps["table_name"].to_string(),
                query: TableQuery::AddCol {
                    col_name: caps["col_name"].to_string(),
                    datatype: caps["datatype"].to_string(),
                },
            });
        }

        let re_select = Regex::new(RE_SELECT).unwrap();
        if let Some(caps) = re_select.captures(query) {
            let condition = caps
                .name("condition")
                .map(|_| caps["condition"].to_string());

            return Ok(Query::Table {
                name: caps["table_name"].to_string(),
                query: TableQuery::Select {
                    condition,
                    cols: getCols(&caps["cols"]),
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
            return Ok(Query::Table {
                name: caps["table_name"].to_string(),
                query: TableQuery::Delete {
                    condition: caps["condition"].to_string(),
                },
            });
        }

        Err("Invalid query.")
    }
}

#[cfg(test)]
mod tests {
    use crate::query_parser::{DatabaseAction, Query, SelectCols, TableQuery};

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
        if let Query::Table {
            name,
            query: TableQuery::Create { cols },
        } = query
        {
            assert_eq!(name, "user".to_string());
            assert_eq!(cols.get("age").unwrap(), "int");
            assert_eq!(cols.get("name").unwrap(), "varchar");
            assert_eq!(cols.get("id").unwrap(), "int");
        } else {
            panic!("Unexpected query");
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
        if let Query::Table {
            name,
            query: TableQuery::Create { cols },
        } = query
        {
            assert_eq!(name, "t_name".to_string());
            assert_eq!(cols.get("column1").unwrap(), "datatype");
            assert_eq!(cols.get("column2").unwrap(), "datatype");
            assert_eq!(cols.get("column3").unwrap(), "datatype");
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
        let query = QueryParser::parse("ALTER TABLE demo ALTER COLUMN id uuid").unwrap();

        if let Query::Table {
            name,
            query: TableQuery::AlterCol { col_name, datatype },
        } = query
        {
            assert_eq!(name, "demo".to_string());
            assert_eq!(col_name, "id".to_string());
            assert_eq!(datatype, "uuid".to_string());
        } else {
            panic!("Unexpted query")
        }
    }

    #[test]
    fn add_col() {
        let query = QueryParser::parse("ALTER TABLE demo ADD id uuid").unwrap();

        if let Query::Table {
            name,
            query: TableQuery::AddCol { col_name, datatype },
        } = query
        {
            assert_eq!(name, "demo".to_string());
            assert_eq!(col_name, "id".to_string());
            assert_eq!(datatype, "uuid".to_string());
        } else {
            panic!("Unexpted query")
        }
    }

    #[test]
    fn parse_select_statment_with_condition() {
        let query = QueryParser::parse("SELECT id,name FROM user WHERE age=12").unwrap();

        if let Query::Table {
            name,
            query: TableQuery::Select { cols, condition },
        } = query
        {
            assert_eq!(name, "user".to_string());
            assert_eq!(cols, SelectCols::Cols(vec!["id".into(), "name".into()]));
            assert!(condition.is_some());
            assert_eq!(condition.unwrap(), "age=12");
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
}
