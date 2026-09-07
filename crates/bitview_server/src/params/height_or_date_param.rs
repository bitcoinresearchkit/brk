use brk_types::{Date, Height};
use schemars::JsonSchema;
use serde::Deserialize;

use crate::Error;

/// Path parameter accepting either a block height (`840000`) or a calendar date
/// (`YYYY-MM-DD`). The handler resolves it and dispatches to the per-height or
/// per-day variant, choosing the matching cache strategy.
#[derive(Deserialize, JsonSchema)]
pub struct HeightOrDateParam {
    /// Confirmed block height as decimal digits (`840000`) or calendar date in
    /// `YYYY-MM-DD` format.
    #[schemars(example = &"840000")]
    pub point: String,
}

/// A resolved [`HeightOrDateParam`]: a confirmed block height or a calendar day.
#[derive(Clone, Copy)]
pub enum HeightOrDate {
    Height(Height),
    Date(Date),
}

impl HeightOrDateParam {
    /// Parses the raw `point`: a `YYYY-MM-DD` string is a [`Date`], an all-digit
    /// string is a [`Height`], anything else is a 400. Dates are tried first
    /// because their dashes keep them from parsing as a height.
    pub fn resolve(&self) -> Result<HeightOrDate, Error> {
        if let Ok(date) = self.point.parse::<Date>() {
            Ok(HeightOrDate::Date(date))
        } else if let Ok(height) = self.point.parse::<u32>() {
            Ok(HeightOrDate::Height(Height::from(height)))
        } else {
            Err(Error::bad_request(format!(
                "expected a block height or YYYY-MM-DD date, got `{}`",
                self.point
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn height_parsing_uses_the_domain_width_without_truncation() {
        for point in ["0", "840000", "4294967295"] {
            let param = HeightOrDateParam {
                point: point.into(),
            };
            let Ok(HeightOrDate::Height(height)) = param.resolve() else {
                panic!("expected height");
            };
            assert_eq!(u32::from(height), point.parse::<u32>().unwrap());
        }
        for point in ["4294967296", "18446744073709551615", "-1", "invalid"] {
            assert!(
                HeightOrDateParam {
                    point: point.into()
                }
                .resolve()
                .is_err()
            );
        }
        assert!(matches!(
            HeightOrDateParam {
                point: "2024-01-01".into()
            }
            .resolve(),
            Ok(HeightOrDate::Date(_))
        ));
    }
}
