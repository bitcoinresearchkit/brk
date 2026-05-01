/// Parameter information.
#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub required: bool,
    pub param_type: String,
    pub description: Option<String>,
}
