//! Deprecated metrics formatting without MetricData wrapper.

use brk_error::{Error, Result};
use brk_types::Format;
use vecdb::AnyExportableVec;

use crate::{DataRangeFormat, LegacyValue, MetricSelection, OutputLegacy, Query};

impl Query {
    /// Deprecated - raw data without MetricData wrapper
    pub fn format_legacy(&self, metrics: &[&dyn AnyExportableVec], params: &DataRangeFormat) -> Result<OutputLegacy> {
        let min_len = metrics.iter().map(|v| v.len()).min().unwrap_or(0);

        let from = params
            .from()
            .map(|from| metrics.iter().map(|v| v.i64_to_usize(from)).min().unwrap_or_default());

        let to = params
            .to_for_len(min_len)
            .map(|to| metrics.iter().map(|v| v.i64_to_usize(to)).min().unwrap_or_default());

        let format = params.format();

        Ok(match format {
            Format::CSV => OutputLegacy::CSV(Self::columns_to_csv(metrics, from.map(|v| v as i64), to.map(|v| v as i64))?),
            Format::JSON => {
                if metrics.is_empty() {
                    return Ok(OutputLegacy::default(format));
                }

                if metrics.len() == 1 {
                    let metric = metrics[0];
                    let count = metric.range_count(from.map(|v| v as i64), to.map(|v| v as i64));
                    let mut buf = Vec::new();
                    if count == 1 {
                        metric.write_json_value(from, &mut buf)?;
                        OutputLegacy::Json(LegacyValue::Value(buf))
                    } else {
                        metric.write_json(from, to, &mut buf)?;
                        OutputLegacy::Json(LegacyValue::List(buf))
                    }
                } else {
                    let mut values = Vec::with_capacity(metrics.len());
                    for vec in metrics {
                        let mut buf = Vec::new();
                        vec.write_json(from, to, &mut buf)?;
                        values.push(buf);
                    }
                    OutputLegacy::Json(LegacyValue::Matrix(values))
                }
            }
        })
    }

    /// Deprecated - use search_and_format instead
    pub fn search_and_format_legacy(&self, params: MetricSelection) -> Result<OutputLegacy> {
        self.search_and_format_legacy_checked(params, usize::MAX)
    }

    /// Deprecated - use search_and_format_checked instead
    pub fn search_and_format_legacy_checked(&self, params: MetricSelection, max_weight: usize) -> Result<OutputLegacy> {
        let vecs = self.search(&params)?;

        let min_len = vecs.iter().map(|v| v.len()).min().expect("search guarantees non-empty");
        let weight = Self::weight(&vecs, params.from(), params.to_for_len(min_len));
        if weight > max_weight {
            return Err(Error::String(format!(
                "Request too heavy: {weight} bytes exceeds limit of {max_weight} bytes"
            )));
        }

        self.format_legacy(&vecs, &params.range)
    }
}
