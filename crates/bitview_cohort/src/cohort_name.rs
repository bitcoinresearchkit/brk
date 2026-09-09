use serde::Serialize;

/// Display names for a cohort with id (for storage/API), short (for charts), and long (for tooltips/labels)
#[derive(Clone, Copy, Serialize)]
pub struct CohortName {
    pub id: &'static str,
    pub short: &'static str,
    pub long: &'static str,
}

impl CohortName {
    pub const fn new(id: &'static str, short: &'static str, long: &'static str) -> Self {
        Self { id, short, long }
    }
}
