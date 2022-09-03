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
/// A regex to match drop column query. [Example](https://regex101.com/r/fM8Csp/1)
pub const RE_DROP_COL: &str =
    r"(?im)ALTER TABLE (?P<table_name>[^\s\n]+) drop column (?P<col_name>[^\s\n;]+)";
/// A regex to match alter  column query. [Example](https://regex101.com/r/KAcjsB/1)
pub const RE_ALTER_COL: &str = r"(?im)ALTER TABLE (?P<table_name>[^\s\n]+) alter column (?P<col_name>[^\s\n;]+) (?P<datatype>[^\n;]+)";
/// A regex to match add column query. [Example](https://regex101.com/r/jcpHYb/1)
pub const RE_ADD_COL: &str =
    r"(?im)ALTER TABLE (?P<table_name>[^\s\n]+) add (?P<col_name>[^\s\n]+) (?P<datatype>[^\s\n;]+)";

/// A regex to match basic select queries with conditions. [Example](https://regex101.com/r/FhdTBh/1)
pub const RE_SELECT: &str =
    r"(?im)select (?P<cols>.+) from (?P<table_name>[^\s;\n]+)( where (?P<condition>[^\n;]+))?";

/// A regex to match complex insert queries. [Example](https://regex101.com/r/uAZ6Uo/1)
pub const RE_INSERT: &str =
    r"(?im)INSERT INTO (?P<table_name>[^\s\n;]+)(?P<cols>.+)? values\s?(?P<values>\(.+\))";

/// A regex to match comma separated values. [Example](https://regex101.com/r/OiSrOW/1)
pub const RE_COMMA_SEPARATED_VALUES: &str = r"(?im)(?P<value>[^,\(\)\s]+)";

/// A regex to match insert query values like `(val1, val2), (val1, val2) (val1, val2);[`. [Example](https://regex101.com/r/mJUv6g/1)
pub const RE_INSERT_VALUES_VALUES: &str = r"(?im)(?P<row>\([^\);]+\))";
