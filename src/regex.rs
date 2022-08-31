pub const RE_DB: &str = r"(?im)(?P<action>[^\s;]+) database (?P<name>[^;]+)";
pub const RE_CREATE_TABLE: &str = r"(?im)create table (?P<name>[^\(\s]+)(\s|)(?P<entries>[^;]+)";
