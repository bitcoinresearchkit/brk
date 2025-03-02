use std::fmt;

use serde::Serialize;
use tabled::Tabled as TabledTabled;

use crate::Format;

#[derive(Debug)]
pub enum Output {
    Json(Value),
    CSV(String),
    TSV(String),
    MD(String),
}

#[derive(Debug, Serialize, TabledTabled)]
#[serde(untagged)]
pub enum Value {
    Matrix(Vec<Vec<serde_json::Value>>),
    List(Vec<serde_json::Value>),
    Single(serde_json::Value),
}

impl Output {
    pub fn default(format: Option<Format>) -> Self {
        match format {
            Some(Format::CSV) => Output::CSV("".to_string()),
            Some(Format::TSV) => Output::TSV("".to_string()),
            _ => Output::Json(Value::Single(serde_json::Value::Null)),
        }
    }
}

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Json(value) => write!(f, "{}", serde_json::to_string_pretty(value).unwrap()),
            Self::CSV(string) => write!(f, "{}", string),
            Self::TSV(string) => write!(f, "{}", string),
            Self::MD(string) => write!(f, "{}", string),
        }
    }
}
