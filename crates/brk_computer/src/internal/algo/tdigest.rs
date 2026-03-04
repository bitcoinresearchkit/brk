/// Streaming t-digest for approximate quantile estimation.
///
/// Uses the merging algorithm with scale function k₂: `q * (1 - q)`.
/// Compression parameter δ controls accuracy vs memory (default 100 → ~200 centroids max).
#[derive(Clone)]
pub(crate) struct TDigest {
    centroids: Vec<Centroid>,
    count: u64,
    min: f64,
    max: f64,
    compression: f64,
}

#[derive(Clone, Copy)]
struct Centroid {
    mean: f64,
    weight: f64,
}

impl Default for TDigest {
    fn default() -> Self {
        Self::new(100.0)
    }
}

impl TDigest {
    pub fn new(compression: f64) -> Self {
        Self {
            centroids: Vec::new(),
            count: 0,
            min: f64::INFINITY,
            max: f64::NEG_INFINITY,
            compression,
        }
    }

    pub fn count(&self) -> u64 {
        self.count
    }

    pub fn reset(&mut self) {
        self.centroids.clear();
        self.count = 0;
        self.min = f64::INFINITY;
        self.max = f64::NEG_INFINITY;
    }

    pub fn add(&mut self, value: f64) {
        if value.is_nan() {
            return;
        }

        self.count += 1;
        if value < self.min {
            self.min = value;
        }
        if value > self.max {
            self.max = value;
        }

        if self.centroids.is_empty() {
            self.centroids.push(Centroid {
                mean: value,
                weight: 1.0,
            });
            return;
        }

        // Single binary search: unclamped position doubles as insert point
        let search = self.centroids.binary_search_by(|c| {
            c.mean
                .partial_cmp(&value)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        let insert_pos = match search {
            Ok(i) | Err(i) => i,
        };

        // Find nearest centroid from insert_pos
        let nearest = if insert_pos >= self.centroids.len() {
            self.centroids.len() - 1
        } else if insert_pos == 0 {
            0
        } else if (value - self.centroids[insert_pos - 1].mean).abs()
            < (value - self.centroids[insert_pos].mean).abs()
        {
            insert_pos - 1
        } else {
            insert_pos
        };

        // Compute quantile of nearest centroid
        let cum_weight: f64 = self.centroids[..nearest]
            .iter()
            .map(|c| c.weight)
            .sum::<f64>()
            + self.centroids[nearest].weight / 2.0;
        let q = cum_weight / self.count as f64;
        let limit = (4.0 * self.compression * q * (1.0 - q)).floor().max(1.0);

        if self.centroids[nearest].weight + 1.0 <= limit {
            // Merge into nearest centroid
            let c = &mut self.centroids[nearest];
            c.mean = (c.mean * c.weight + value) / (c.weight + 1.0);
            c.weight += 1.0;
        } else {
            // Insert new centroid at sorted position (reuse insert_pos)
            self.centroids.insert(
                insert_pos,
                Centroid {
                    mean: value,
                    weight: 1.0,
                },
            );
        }

        // Compress if too many centroids
        let max_centroids = (2.0 * self.compression) as usize;
        if self.centroids.len() > max_centroids {
            self.compress();
        }
    }

    fn compress(&mut self) {
        if self.centroids.len() <= 1 {
            return;
        }

        let total = self.count as f64;
        let mut cum = 0.0;
        let mut write_idx = 0;

        for read_idx in 1..self.centroids.len() {
            let c = self.centroids[read_idx];
            let last = &mut self.centroids[write_idx];
            let q = (cum + last.weight / 2.0) / total;
            let limit = (4.0 * self.compression * q * (1.0 - q)).floor().max(1.0);
            if last.weight + c.weight <= limit {
                let new_weight = last.weight + c.weight;
                last.mean = (last.mean * last.weight + c.mean * c.weight) / new_weight;
                last.weight = new_weight;
            } else {
                cum += last.weight;
                write_idx += 1;
                self.centroids[write_idx] = c;
            }
        }
        self.centroids.truncate(write_idx + 1);
    }

    /// Batch quantile query in a single pass. `qs` must be sorted ascending.
    pub fn quantiles(&self, qs: &[f64], out: &mut [f64]) {
        if self.centroids.is_empty() {
            out.iter_mut().for_each(|o| *o = 0.0);
            return;
        }
        if self.centroids.len() == 1 {
            let mean = self.centroids[0].mean;
            for (i, &q) in qs.iter().enumerate() {
                out[i] = if q <= 0.0 {
                    self.min
                } else if q >= 1.0 {
                    self.max
                } else {
                    mean
                };
            }
            return;
        }

        let total = self.count as f64;
        let mut cum = 0.0;
        let mut ci = 0;

        for (qi, &q) in qs.iter().enumerate() {
            if q <= 0.0 {
                out[qi] = self.min;
                continue;
            }
            if q >= 1.0 {
                out[qi] = self.max;
                continue;
            }

            let target = q * total;

            // Advance centroids until the current centroid's midpoint exceeds target
            while ci < self.centroids.len() {
                let mid = cum + self.centroids[ci].weight / 2.0;
                if target < mid {
                    break;
                }
                cum += self.centroids[ci].weight;
                ci += 1;
            }

            if ci >= self.centroids.len() {
                // Past all centroids — interpolate between last centroid and max
                let last = self.centroids.last().unwrap();
                let last_mid = total - last.weight / 2.0;
                let remaining = total - last_mid;
                out[qi] = if remaining == 0.0 {
                    self.max
                } else {
                    last.mean + (self.max - last.mean) * ((target - last_mid) / remaining)
                };
            } else if ci == 0 {
                // Before first centroid — interpolate between min and first centroid
                let c = &self.centroids[0];
                let first_mid = c.weight / 2.0;
                out[qi] = if first_mid == 0.0 {
                    self.min
                } else {
                    self.min + (c.mean - self.min) * (target / first_mid)
                };
            } else {
                // Between centroid ci-1 and ci
                let c = &self.centroids[ci];
                let prev = &self.centroids[ci - 1];
                let mid = cum + c.weight / 2.0;
                let prev_center = cum - prev.weight / 2.0;
                let frac = if mid == prev_center {
                    0.5
                } else {
                    (target - prev_center) / (mid - prev_center)
                };
                out[qi] = prev.mean + (c.mean - prev.mean) * frac;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn quantile(td: &TDigest, q: f64) -> f64 {
        let mut out = [0.0];
        td.quantiles(&[q], &mut out);
        out[0]
    }

    #[test]
    fn basic_quantiles() {
        let mut td = TDigest::default();
        for i in 1..=1000 {
            td.add(i as f64);
        }
        assert_eq!(td.count(), 1000);

        let median = quantile(&td, 0.5);
        assert!((median - 500.0).abs() < 10.0, "median was {median}");

        let p99 = quantile(&td, 0.99);
        assert!((p99 - 990.0).abs() < 15.0, "p99 was {p99}");

        let p01 = quantile(&td, 0.01);
        assert!((p01 - 10.0).abs() < 15.0, "p01 was {p01}");
    }

    #[test]
    fn empty_digest() {
        let td = TDigest::default();
        assert_eq!(td.count(), 0);
        assert_eq!(quantile(&td, 0.5), 0.0);
    }

    #[test]
    fn single_value() {
        let mut td = TDigest::default();
        td.add(42.0);
        assert_eq!(quantile(&td, 0.0), 42.0);
        assert_eq!(quantile(&td, 0.5), 42.0);
        assert_eq!(quantile(&td, 1.0), 42.0);
    }

    #[test]
    fn reset_works() {
        let mut td = TDigest::default();
        for i in 0..100 {
            td.add(i as f64);
        }
        assert_eq!(td.count(), 100);
        td.reset();
        assert_eq!(td.count(), 0);
        assert_eq!(quantile(&td, 0.5), 0.0);
    }
}
