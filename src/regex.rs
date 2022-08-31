//! A collection of a regular expressions used to parse raw querys
//!
//! **Note:** All regex are case insensitive.

/// A regex to extract the the information from a database query
///
/// This regex should match any query whith this format
/// `CREATE DATABASE <DB_NAME>;`, `DROP DATABASE <DB_NAME>;`, `USE DATABASE <DB_NAME>;`
/// both the action(create, drop, use) and the DB name will be extracted.
///
/// See a interactive example [here](https://regex101.com/r/Co6RIt/1)
pub const RE_DB: &str = r"(?im)(?P<action>[^\s;]+) database (?P<name>[^;]+)";
/// A regex to extract table name and table entries.
pub const RE_CREATE_TABLE: &str = r"(?im)create table (?P<name>[^\(\s]+)(\s|)(?P<entries>[^;]+)";
/// A regex to extract columns name and its types. intractive example [here](https://regex101.com/r/s6rTCW/1)
pub const RE_TABLE_ENTRIES: &str = r"(?im)(?P<col_name>[^\s,\(]+) (?P<col_type>[^,\n;\)]+)";
/// A regex to match `drop` or `truncate` table query. Example [here](https://regex101.com/r/9z6nW4/1)
pub const RE_TABLE: &str = r"(?im)(?P<action>drop|truncate) table (?P<name>[^;]+)";
