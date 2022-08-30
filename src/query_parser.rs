use std::collections::HashMap;

pub type DBName = String;
pub type TableName = String;
pub type ColName = String;
pub type ColType = String; // Todo: Should be an enum

#[derive(Debug)]
pub enum DatabaseAction {
    Create,
    Drop,
    Use,
}

#[derive(Debug)]
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

#[derive(Debug)]
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
