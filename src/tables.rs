pub struct Table<'a> {
    db: &'a str,
    table_name: &'a str,
}

// Todo: Add table builder
impl<'a> Table<'a> {
    pub fn new(db: &'a str, table_name: &'a str) -> Self {
        Self { db, table_name }
    }
    pub fn create(cols: Vec<(&str, &str)>) {
        // Should create the [tablename].schema.json
    }
}
