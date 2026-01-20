use brk_error::Result;
use brk_types::{Format, LegacyValue, MetricOutputLegacy, OutputLegacy};

use crate::{Query, ResolvedQuery};

impl Query {
    /// Deprecated - format a resolved query as legacy output (expensive).
    pub fn format_legacy(&self, resolved: ResolvedQuery) -> Result<MetricOutputLegacy> {
        let ResolvedQuery {
            vecs,
            format,
            version,
            total,
            start,
            end,
            ..
        } = resolved;

        if vecs.is_empty() {
            return Ok(MetricOutputLegacy {
                output: OutputLegacy::default(format),
                version: 0,
                total: 0,
                start: 0,
                end: 0,
            });
        }

        let from = Some(start as i64);
        let to = Some(end as i64);

        let output = match format {
            Format::CSV => OutputLegacy::CSV(Self::columns_to_csv(&vecs, start, end)?),
            Format::JSON => {
                if vecs.len() == 1 {
                    let metric = vecs[0];
                    let count = metric.range_count(from, to);
                    let mut buf = Vec::new();
                    if count == 1 {
                        metric.write_json_value(Some(start), &mut buf)?;
                        OutputLegacy::Json(LegacyValue::Value(buf))
                    } else {
                        metric.write_json(Some(start), Some(end), &mut buf)?;
                        OutputLegacy::Json(LegacyValue::List(buf))
                    }
                } else {
                    let mut values = Vec::with_capacity(vecs.len());
                    for vec in &vecs {
                        let mut buf = Vec::new();
                        vec.write_json(Some(start), Some(end), &mut buf)?;
                        values.push(buf);
                    }
                    OutputLegacy::Json(LegacyValue::Matrix(values))
                }
            }
        };

        Ok(MetricOutputLegacy {
            output,
            version,
            total,
            start,
            end,
        })
    }
}
