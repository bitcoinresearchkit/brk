use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub location: ParameterLocation,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ParameterLocation {
    Path,
    Query,
}
