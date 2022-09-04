use std::path::{Path, PathBuf};

use regex::Regex;

use crate::{
    database::DB_DIR, query_parser::SelectCols, regex::RE_COMMA_SEPARATED_VALUES, tables::Table,
};

pub fn get_db_path(name: &str) -> PathBuf {
    let base_dir = Path::new(DB_DIR);
    let db_dir = base_dir.join(name);

    db_dir
}

pub fn schema_file(file: &str) -> String {
    format!("{}.schema.json", file)
}

pub fn table_file(file: &str) -> String {
    format!("{}.json", file)
}

pub fn get_schema_path(table: &Table) -> PathBuf {
    let db_dir = get_db_path(table.db);
    db_dir.join(schema_file(table.table_name))
}

pub fn get_table_path(table: &Table) -> PathBuf {
    let db_dir = get_db_path(table.db);
    db_dir.join(table_file(table.table_name))
}

pub fn getCols(query: &str) -> SelectCols {
    let query = query.trim();

    if query == "*" {
        SelectCols::All
    } else {
        let cols = query
            .split(",")
            .map(|c| c.trim().to_string())
            .collect::<Vec<_>>();

        SelectCols::Cols(cols)
    }
}

pub fn get_comma_separated_values(query: &str) -> Vec<String> {
    let re = Regex::new(RE_COMMA_SEPARATED_VALUES).unwrap();

    re.captures_iter(query)
        .map(|caps| caps["value"].to_string())
        .collect::<Vec<_>>()
}
