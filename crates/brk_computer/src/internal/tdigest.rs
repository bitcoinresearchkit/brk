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

        // Find nearest centroid by mean
        let pos = self
            .centroids
            .binary_search_by(|c| c.mean.partial_cmp(&value).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or_else(|i| i.min(self.centroids.len() - 1));

        // Check neighbors for the actual nearest
        let nearest = if pos > 0
            && (value - self.centroids[pos - 1].mean).abs()
                < (value - self.centroids[pos].mean).abs()
        {
            pos - 1
        } else {
            pos
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
            // Insert new centroid at sorted position
            let insert_pos = self
                .centroids
                .binary_search_by(|c| {
                    c.mean
                        .partial_cmp(&value)
                        .unwrap_or(std::cmp::Ordering::Equal)
                })
                .unwrap_or_else(|i| i);
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

        let total: f64 = self.centroids.iter().map(|c| c.weight).sum();
        let mut merged: Vec<Centroid> = Vec::with_capacity(self.centroids.len());
        let mut cum = 0.0;

        for c in &self.centroids {
            if let Some(last) = merged.last_mut() {
                let q = (cum + last.weight / 2.0) / total;
                let limit = (4.0 * self.compression * q * (1.0 - q)).floor().max(1.0);
                if last.weight + c.weight <= limit {
                    let new_weight = last.weight + c.weight;
                    last.mean = (last.mean * last.weight + c.mean * c.weight) / new_weight;
                    last.weight = new_weight;
                    continue;
                }
                cum += last.weight;
            }
            merged.push(*c);
        }
        self.centroids = merged;
    }

    pub fn quantile(&self, q: f64) -> f64 {
        if self.centroids.is_empty() {
            return 0.0;
        }
        if q <= 0.0 {
            return self.min;
        }
        if q >= 1.0 {
            return self.max;
        }
        if self.centroids.len() == 1 {
            return self.centroids[0].mean;
        }

        let total: f64 = self.centroids.iter().map(|c| c.weight).sum();
        let target = q * total;
        let mut cum = 0.0;

        for i in 0..self.centroids.len() {
            let c = &self.centroids[i];
            let mid = cum + c.weight / 2.0;

            if target < mid {
                // Interpolate between previous centroid (or min) and this one
                if i == 0 {
                    // Between min and first centroid center
                    let first_mid = c.weight / 2.0;
                    if first_mid == 0.0 {
                        return self.min;
                    }
                    return self.min + (c.mean - self.min) * (target / first_mid);
                }
                let prev = &self.centroids[i - 1];
                let prev_center = cum - prev.weight / 2.0;
                let frac = if mid == prev_center {
                    0.5
                } else {
                    (target - prev_center) / (mid - prev_center)
                };
                return prev.mean + (c.mean - prev.mean) * frac;
            }

            cum += c.weight;
        }

        // Between last centroid center and max
        let last = self.centroids.last().unwrap();
        let last_mid = total - last.weight / 2.0;
        let remaining = total - last_mid;
        if remaining == 0.0 {
            return self.max;
        }
        last.mean + (self.max - last.mean) * ((target - last_mid) / remaining)
    }

    /// Batch quantile query. `qs` must be sorted ascending.
    pub fn quantiles(&self, qs: &[f64], out: &mut [f64]) {
        for (i, &q) in qs.iter().enumerate() {
            out[i] = self.quantile(q);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_quantiles() {
        let mut td = TDigest::default();
        for i in 1..=1000 {
            td.add(i as f64);
        }
        assert_eq!(td.count(), 1000);

        let median = td.quantile(0.5);
        assert!((median - 500.0).abs() < 10.0, "median was {median}");

        let p99 = td.quantile(0.99);
        assert!((p99 - 990.0).abs() < 15.0, "p99 was {p99}");

        let p01 = td.quantile(0.01);
        assert!((p01 - 10.0).abs() < 15.0, "p01 was {p01}");
    }

    #[test]
    fn empty_digest() {
        let td = TDigest::default();
        assert_eq!(td.count(), 0);
        assert_eq!(td.quantile(0.5), 0.0);
    }

    #[test]
    fn single_value() {
        let mut td = TDigest::default();
        td.add(42.0);
        assert_eq!(td.quantile(0.0), 42.0);
        assert_eq!(td.quantile(0.5), 42.0);
        assert_eq!(td.quantile(1.0), 42.0);
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
        assert_eq!(td.quantile(0.5), 0.0);
    }
}
