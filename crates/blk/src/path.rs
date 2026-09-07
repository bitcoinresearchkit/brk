use brk_error::Error;

pub use crate::step::Step;

pub struct Path {
    pub raw: String,
    pub steps: Vec<Step>,
}

impl Path {
    pub fn parse(s: &str) -> brk_error::Result<Self> {
        let mut parts = s.split('.').peekable();
        let mut steps = Vec::new();
        while let Some(name) = parts.next() {
            if name.is_empty() {
                return Err(Error::Parse(format!("bad path '{s}': empty segment")));
            }
            if name.parse::<usize>().is_ok() {
                return Err(Error::Parse(format!(
                    "bad path '{s}': '{name}' must follow a field name"
                )));
            }
            let index = parts.peek().and_then(|p| p.parse::<usize>().ok());
            if index.is_some() {
                parts.next();
            }
            steps.push(Step {
                name: name.to_string(),
                index,
            });
        }
        Ok(Self {
            raw: s.to_string(),
            steps,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn consumes_only_numeric_following_segments_as_indexes() {
        let path = Path::parse("tx.2.vout.0.value").unwrap();
        let steps: Vec<_> = path
            .steps
            .iter()
            .map(|step| (step.name.as_str(), step.index))
            .collect();
        assert_eq!(steps, [("tx", Some(2)), ("vout", Some(0)), ("value", None)]);
        assert_eq!(path.raw, "tx.2.vout.0.value");
        assert_eq!(Path::parse("tx.value").unwrap().steps.len(), 2);
        for invalid in ["", ".tx", "tx.", "tx..value", "0.tx", "tx.0.1"] {
            assert!(Path::parse(invalid).is_err(), "{invalid}");
        }
    }
}
