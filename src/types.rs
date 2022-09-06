#[derive(Debug)]
pub enum DataType {
    // Numeric datatypes
    INTEGER,
    INT,
    FLOAT,
    DEC,
    // String datatypes
    TEXT,
    VARCHAR(u32),
    ENUM(Vec<String>),
    BOOLEAN,
    BOOL,
}

impl DataType {
    fn parse(datatype: &str) -> Self {
        let dt = datatype.trim();
        match dt {
            _ if DataType::INTEGER.as_str() == dt => DataType::INTEGER,
            _ if DataType::INT.as_str() == dt => DataType::INT,
            _ if DataType::FLOAT.as_str() == dt => DataType::FLOAT,
            _ if DataType::DEC.as_str() == dt => DataType::DEC,
            _ if DataType::TEXT.as_str() == dt => DataType::TEXT,
            _ if DataType::BOOLEAN.as_str() == dt => DataType::BOOLEAN,
            _ if DataType::BOOL.as_str() == dt => DataType::BOOL,
            _ => {}
        }
    }

    fn as_str(&self) -> String {
        format!("{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::DataType;

    #[test]
    fn should_convert_datatypes_as_str() {
        assert_eq!(DataType::BOOL.as_str(), "BOOL");
        assert_eq!(DataType::INTEGER.as_str(), "INTEGER");
        assert_eq!(DataType::VARCHAR(12).as_str(), "VARCHAR(12)");
    }
}
