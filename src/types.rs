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
    #[error("Invalid number")]
    InvalidInt(String),
    #[error("Invalid float")]
    InvalidFloat(String),
    #[error("Invalid float")]
    InvalidEnum(String),
    #[error("Invalid boolean")]
    InvalidBool(String),
    #[error("Invalid string")]
    InvalidStr(String),
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
    VARCHAR(usize),
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
                Some(_) => match caps["size"].parse::<usize>() {
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

    pub fn is_valid(&self, raw: &str) -> Result<(), DataTypesErr> {
        return match self {
            DataType::INTEGER | DataType::INT if raw.parse::<i64>().is_err() => Err(
                DataTypesErr::InvalidInt(format!("'{}' is not a valid {:?}", raw, self)),
            ),
            DataType::FLOAT | DataType::DEC if raw.parse::<f64>().is_err() => Err(
                DataTypesErr::InvalidFloat(format!("'{}' is not a valid {:?}", raw, self)),
            ),
            DataType::VARCHAR(max_len) if &raw.len() > max_len => Err(DataTypesErr::InvalidStr(
                format!("Max length exceed of `{}`. Max len = {}", raw, max_len),
            )),
            DataType::ENUM(values) if values.iter().position(|v| v == raw).is_none() => {
                Err(DataTypesErr::InvalidEnum(format!(
                    "`{}` is not valid enum. must be one of these {:?}",
                    raw, values
                )))
            }
            DataType::BOOLEAN | DataType::BOOL if raw.parse::<bool>().is_err() => Err(
                DataTypesErr::InvalidBool(format!("`{}` is not a valid boolean", raw)),
            ),
            _ => Ok(()),
        };
    }

    fn default(&self) {
        match self {
            DataType::INTEGER | DataType::INT => "0",
            DataType::FLOAT | DataType::DEC => "0.0",
            DataType::TEXT | DataType::VARCHAR(_) => "",
            DataType::ENUM(val) => val[0].as_str(),
            DataType::BOOLEAN | DataType::BOOL => "false",
        };
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

    #[test]
    fn validate_datatypes() {
        let datatypes = [
            (DataType::BOOLEAN, "true"),
            (DataType::FLOAT, "1.00"),
            (
                DataType::ENUM(vec!["HUMAN".into(), "ALIAN".into()]),
                "HUMAN",
            ),
            (DataType::INTEGER, "21"),
        ];

        datatypes
            .into_iter()
            .for_each(|(dtype, value)| assert!(dtype.is_valid(value).is_ok()))
    }

    #[test]
    fn check_invalid_datatypes() {
        let datatypes = [
            (
                DataType::BOOLEAN,
                "something",
                DataTypesErr::InvalidBool("`something` is not a valid boolean".into()),
            ),
            (
                DataType::FLOAT,
                "str",
                DataTypesErr::InvalidFloat("'str' is not a valid FLOAT".into()),
            ),
            (
                DataType::ENUM(vec!["HUMAN".into(), "ALIAN".into()]),
                "ANIMAL",
                DataTypesErr::InvalidEnum(
                    "`ANIMAL` is not valid enum. must be one of these [\"HUMAN\", \"ALIAN\"]"
                        .into(),
                ),
            ),
            (
                DataType::INTEGER,
                "int",
                DataTypesErr::InvalidInt("'int' is not a valid INTEGER".into()),
            ),
        ];

        datatypes
            .into_iter()
            .for_each(|(dtype, value, err_msg)| assert_eq!(dtype.is_valid(value), Err(err_msg)))
    }
}
