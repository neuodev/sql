use std::num::ParseIntError;

use regex::Regex;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::regex::{RE_ENUM, RE_ENUM_VALUES, RE_VARCHAR};

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DataTypesErr {
    #[error("Invalid Type")]
    InvalidType(String),
    #[error("Invalid varchar Type")]
    InvalidVarchar(#[from] ParseIntError),
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
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
    pub fn parse(datatype: &str) -> Result<Self, DataTypesErr> {
        let re_varchar = Regex::new(RE_VARCHAR).unwrap();
        let re_enum = Regex::new(RE_ENUM).unwrap();
        let re_enum_values = Regex::new(RE_ENUM_VALUES).unwrap();
        let dt = datatype.trim();

        if let Some(caps) = re_varchar.captures(dt) {
            let size = match caps.name("size") {
                Some(_) => match caps["size"].parse::<u32>() {
                    Ok(s) => s,
                    Err(e) => return Err(DataTypesErr::InvalidVarchar(e)),
                },
                None => 255,
            };

            return Ok(DataType::VARCHAR(size));
        }

        if let Some(caps) = re_enum.captures(dt) {
            let values = re_enum_values
                .captures_iter(&caps["values"])
                .map(|caps| caps["value"].trim().to_string())
                .filter(|v| !v.is_empty())
                .collect::<Vec<_>>();
            return Ok(DataType::ENUM(values));
        }

        let dt = dt.to_uppercase();
        let dt = match dt {
            _ if DataType::INTEGER.as_string() == dt => DataType::INTEGER,
            _ if DataType::INT.as_string() == dt => DataType::INT,
            _ if DataType::FLOAT.as_string() == dt => DataType::FLOAT,
            _ if DataType::DEC.as_string() == dt => DataType::DEC,
            _ if DataType::TEXT.as_string() == dt => DataType::TEXT,
            _ if DataType::BOOLEAN.as_string() == dt => DataType::BOOLEAN,
            _ if DataType::BOOL.as_string() == dt => DataType::BOOL,

            _ => return Err(DataTypesErr::InvalidType(datatype.trim().into())),
        };

        return Ok(dt);
    }

    pub fn as_string(&self) -> String {
        format!("{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use crate::types::DataTypesErr;

    use super::DataType;

    #[test]
    fn should_convert_datatypes_as_str() {
        assert_eq!(DataType::BOOL.as_string(), "BOOL");
        assert_eq!(DataType::INTEGER.as_string(), "INTEGER");
        assert_eq!(DataType::VARCHAR(12).as_string(), "VARCHAR(12)");
    }

    #[test]
    fn parse_as_integer() {
        let dt = DataType::parse(" INTEGER ");
        assert!(dt.is_ok());
        assert_eq!(dt.unwrap(), DataType::INTEGER);

        let dt = DataType::parse(" int ");
        assert!(dt.is_ok());
        assert_eq!(dt.unwrap(), DataType::INT)
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

    #[test]
    fn parse_varchar_with_size() {
        let dt = DataType::parse("varchar(12)").unwrap();
        assert_eq!(dt, DataType::VARCHAR(12));
    }

    #[test]
    fn parse_varchar_with_default_size() {
        let dt = DataType::parse("varchar").unwrap();
        assert_eq!(dt, DataType::VARCHAR(255));
    }

    #[test]
    fn parse_enum_values() {
        let dt = DataType::parse("ENUM(1, 2, 3)").unwrap();
        assert_eq!(dt, DataType::ENUM(vec!["1".into(), "2".into(), "3".into()]));
    }

    #[test]
    fn parse_enum_values_with_single_quotes() {
        let dt = DataType::parse("ENUM('HUMAND', 'ALIEN')").unwrap();
        assert_eq!(dt, DataType::ENUM(vec!["HUMAND".into(), "ALIEN".into(),]));
    }

    #[test]
    fn parse_enum_values_with_douple_quotes() {
        let dt = DataType::parse(r#"ENUM("HUMAND", "ALIEN")"#).unwrap();
        assert_eq!(dt, DataType::ENUM(vec!["HUMAND".into(), "ALIEN".into(),]));
    }

    #[test]
    fn parse_enum_values_with_no_quotes() {
        let dt = DataType::parse(r#"ENUM(HUMAND, ALIEN)"#).unwrap();
        assert_eq!(dt, DataType::ENUM(vec!["HUMAND".into(), "ALIEN".into(),]));
    }
}
