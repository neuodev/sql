use regex::Regex;
use std::{
    io::Write,
    path::{Path, PathBuf},
};
use tabwriter::TabWriter;

use crate::{
    database::DB_DIR,
    query_parser::SelectCols,
    regex::RE_COMMA_SEPARATED_VALUES,
    tables::{Table, TableEntries},
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

pub fn get_cols(query: &str) -> SelectCols {
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

pub fn display_entries(entries: TableEntries) {
    let mut tw = TabWriter::new(vec![]);

    if let Some(entry) = entries.get(0) {
        let mut header = String::new();
        entry
            .keys()
            .into_iter()
            .for_each(|k| header.push_str(&format!("{k}\t")));

        tw.write_all(header.as_bytes()).unwrap();
        tw.flush().unwrap();
        let written = String::from_utf8(tw.into_inner().unwrap()).unwrap();
        println!("{}", written);
    }
}
