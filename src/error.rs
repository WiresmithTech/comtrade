use std::num::NonZeroUsize;
use thiserror::Error;
use crate::ParseError;

#[derive(Debug, Error, PartialEq, Clone)]
pub enum ComtradeError {
    #[error("Missing elements on line. Context: {0}")]
    MissingLineElements(&'static str),
    #[error("Unable to parse value: {value} as {type_} for {field}.")]
    InvalidValue {
        value: String,
        type_: &'static str,
        field: &'static str,
    },
    #[error("Unexpected end of cfg file.")]
    UnexpectedEndOfCfgFile,
    #[error("Invalid version string: {0}")]
    BadRevisionFormat(String),
    #[error("The normal status for status channel index {0} is invalid. It must be 0 or 1.")]
    InvalidNormalStatus(NonZeroUsize),
    #[error("Parser Error: {0:?}")]
    ParserError(ParseError),
}


impl ComtradeError {
    pub fn add_context(self, msg: &'static str) -> Self {
        match  self {
            ComtradeError::MissingLineElements(_) => ComtradeError::MissingLineElements(msg),
            ComtradeError::InvalidValue { value, type_, field: _ } => { ComtradeError::InvalidValue { value, type_, field: msg }}
            _ => self,
        }
    }
}