use std::path::Path;

#[derive(PartialEq, Eq)]
pub enum Extension {
    CSV,
    JSON,
}

impl Extension {
    pub fn from(path: &Path) -> Option<Self> {
        if let Some(extension) = path.extension() {
            let extension = extension.to_str().unwrap();

            if extension == Self::CSV.to_str() {
                Some(Self::CSV)
            } else if extension == Self::JSON.to_str() {
                Some(Self::JSON)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            Extension::CSV => "csv",
            Extension::JSON => "json",
        }
    }

    pub fn to_dot_str(&self) -> String {
        format!(".{}", self.to_str())
    }

    pub fn remove_extension(s: &str) -> String {
        s.replace(&Self::CSV.to_dot_str(), "")
            .replace(&Self::JSON.to_dot_str(), "")
    }
}
