use inquire::{Editor, InquireError};

pub struct QueryPlanner;

impl QueryPlanner {
    pub fn init() -> Result<(), InquireError> {
        let res = Editor::new("SQL query")
            .with_help_message("Enter SQL query")
            .with_predefined_text("SELECT * FROM user")
            .with_file_extension("sql")
            .prompt()?;

        Ok(())
    }
}
