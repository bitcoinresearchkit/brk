use schemars::JsonSchema;
use serde::Deserialize;

/// Range parameters for slicing data
#[derive(Default, Debug, Deserialize, JsonSchema)]
pub struct DataRange {
    /// Inclusive starting index, if negative will be from the end
    #[serde(default, alias = "f")]
    from: Option<i64>,

    /// Exclusive ending index, if negative will be from the end, overrides 'count'
    #[serde(default, alias = "t")]
    to: Option<i64>,

    /// Number of values requested
    #[serde(default, alias = "c")]
    count: Option<usize>,
}

impl DataRange {
    pub fn set_from(mut self, from: i64) -> Self {
        self.from.replace(from);
        self
    }

    pub fn set_to(mut self, to: i64) -> Self {
        self.to.replace(to);
        self
    }

    pub fn set_count(mut self, count: usize) -> Self {
        self.count.replace(count);
        self
    }

    pub fn from(&self) -> Option<i64> {
        self.from
    }

    pub fn to(&self) -> Option<i64> {
        if self.to.is_none()
            && let Some(c) = self.count
        {
            let c = c as i64;
            if let Some(f) = self.from {
                if f >= 0 || f.abs() > c {
                    return Some(f + c);
                }
            } else {
                return Some(c);
            }
        }
        self.to
    }
}
