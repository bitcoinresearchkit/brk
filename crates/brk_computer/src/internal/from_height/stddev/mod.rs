mod base;
mod extended;

pub use base::*;
pub use extended::*;

fn period_suffix(period: &str) -> String {
    if period.is_empty() {
        String::new()
    } else {
        format!("_{period}")
    }
}
