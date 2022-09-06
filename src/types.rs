use std::num::ParseIntError;

use regex::Regex;
use thiserror::Error;

use crate::regex::{RE_ENUM, RE_ENUM_VALUES, RE_VARCHAR};

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DataTypesErr {
    #[error("Invalid Type")]
    InvalidType(String),
    #[error("Invalid varchar Type")]
    InvalidVarchar(#[from] ParseIntError),
}

#[derive(Debug, PartialEq, Eq)]
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
    fn parse(datatype: &str) -> Result<Self, DataTypesErr> {
        let re_varchar = Regex::new(RE_VARCHAR).unwrap();
        let re_enum = Regex::new(RE_ENUM).unwrap();
        let re_enum_values = Regex::new(RE_ENUM_VALUES).unwrap();
        let dt = datatype.trim();

        if let Some(caps) = re_varchar.captures(dt) {
            let size = match caps["size"].parse::<u32>() {
                Ok(s) => s,
                Err(e) => return Err(DataTypesErr::InvalidVarchar(e)),
            };
            return Ok(DataType::VARCHAR(size));
        }

        if let Some(caps) = re_enum.captures(dt) {
            let values = re_enum_values
                .captures_iter(&caps["values"])
                .map(|caps| caps["value"].to_string())
                .filter(|v| !v.is_empty())
                .collect::<Vec<_>>();
            return Ok(DataType::ENUM(values));
        }

        let dt = match dt {
            _ if DataType::INTEGER.as_str() == dt => DataType::INTEGER,
            _ if DataType::INT.as_str() == dt => DataType::INT,
            _ if DataType::FLOAT.as_str() == dt => DataType::FLOAT,
            _ if DataType::DEC.as_str() == dt => DataType::DEC,
            _ if DataType::TEXT.as_str() == dt => DataType::TEXT,
            _ if DataType::BOOLEAN.as_str() == dt => DataType::BOOLEAN,
            _ if DataType::BOOL.as_str() == dt => DataType::BOOL,

            _ => return Err(DataTypesErr::InvalidType(dt.to_string())),
        };

        return Ok(dt);
    }

    fn as_str(&self) -> String {
        format!("{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use crate::types::DataTypesErr;

    use super::DataType;

    #[test]
    fn should_convert_datatypes_as_str() {
        assert_eq!(DataType::BOOL.as_str(), "BOOL");
        assert_eq!(DataType::INTEGER.as_str(), "INTEGER");
        assert_eq!(DataType::VARCHAR(12).as_str(), "VARCHAR(12)");
    }

    #[test]
    fn parse_as_integer() {
        let dt = DataType::parse(" INTEGER ");
        assert!(dt.is_ok());
        assert_eq!(dt.unwrap(), DataType::INTEGER)
    }

    #[test]
    fn parse_as_bool() {
        let dt = DataType::parse(" BOOL ");
        assert!(dt.is_ok());
        assert_eq!(dt.unwrap(), DataType::BOOL)
    }

    #[test]
    fn parse_invalid_type() {
        let dt = DataType::parse(" Cool ");
        assert!(dt.is_err());
        assert_eq!(dt, Err(DataTypesErr::InvalidType("Cool".into())));
    }
}
