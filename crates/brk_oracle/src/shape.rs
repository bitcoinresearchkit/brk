use crate::{
    scale::HistogramEma,
    stencil::{N_ARMS, normalized_arms_at},
};

/// EMA rate for the adaptive shape template (~250-block time constant), slow
/// enough that a transient octave slide can't corrupt the profile before the
/// pick recovers.
const SHAPE_BETA: f64 = 0.004;

/// Adaptive shape-anchoring restoring force for the slow cold-start regime.
///
/// Holds a round-USD shape template (`profile`), re-estimated each block from the
/// arm vector at the pick, and adds a per-candidate score pulling the search
/// toward the octave whose payment shape looks real. This lets the slow EMA
/// resist round-USD octave aliasing in the thin pre-2018 output mix.
///
/// A zero `weight` makes it inert ([`score`](Self::score) returns 0,
/// [`update`](Self::update) is a no-op), so the fast regime carries it for free
/// without call sites special-casing the disabled path.
#[derive(Clone)]
pub(crate) struct ShapeAnchor {
    weight: f64,
    /// Seeded flat (every arm equal). The slow EMA learns the real payment shape
    /// within a few hundred blocks, so no hand-tuned starting guess is needed.
    profile: [f64; N_ARMS],
}

impl ShapeAnchor {
    pub(crate) fn new(weight: f64) -> Self {
        Self {
            weight,
            profile: [1.0 / N_ARMS as f64; N_ARMS],
        }
    }

    /// Restoring-force contribution to a candidate bin's score: `weight` times the
    /// shape match against the learned profile. 0 when inert or the bin is empty.
    pub(crate) fn score(&self, ema: &HistogramEma, bin: i64) -> f64 {
        if self.weight == 0.0 {
            return 0.0;
        }
        self.weight * self.shape_match(ema, bin)
    }

    /// Blend the L1-normalized arm shape at `pick` into the profile (slow EMA,
    /// [`SHAPE_BETA`]). No-op when inert or the pick is empty.
    pub(crate) fn update(&mut self, ema: &HistogramEma, pick: i64) {
        if self.weight == 0.0 {
            return;
        }
        if let Some(arms) = normalized_arms_at(ema, pick) {
            (0..N_ARMS).for_each(|i| {
                self.profile[i] = (1.0 - SHAPE_BETA) * self.profile[i] + SHAPE_BETA * arms[i];
            });
        }
    }

    /// Shape match `1 - L1distance` between the candidate's L1-normalized arm
    /// vector and the profile. 1.0 is an identical shape and it falls as mass
    /// shifts off the round-USD ladder. 0 for an empty (no-mass) center.
    fn shape_match(&self, ema: &HistogramEma, center: i64) -> f64 {
        match normalized_arms_at(ema, center) {
            Some(arms) => 1.0 - (0..N_ARMS).map(|i| (arms[i] - self.profile[i]).abs()).sum::<f64>(),
            None => 0.0,
        }
    }
}
