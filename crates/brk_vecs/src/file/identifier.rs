#[derive(Debug, Clone)]
pub enum Identifier {
    Number(usize),
    String(String),
}

impl<'a> From<&'a str> for Identifier {
    fn from(value: &'a str) -> Self {
        Self::String(value.to_owned())
    }
}

impl From<String> for Identifier {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<usize> for Identifier {
    fn from(value: usize) -> Self {
        Self::Number(value)
    }
}
