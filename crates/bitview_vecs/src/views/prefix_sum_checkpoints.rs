use std::sync::Arc;

use parking_lot::RwLock;
use vecdb::{ReadableVec, VecIndex, VecValue};

pub(super) const INTERVAL: usize = 256;

/// Compact algorithmic state for prefix sums: one sum per 256 source values.
/// Source revisions survive eviction and leave unchanged prefixes valid on append.
#[derive(Clone, Default)]
pub(super) struct PrefixSumCheckpoints(Arc<RwLock<State>>);

#[derive(Default)]
struct State {
    revision: Option<u64>,
    sums: Vec<u64>,
}

impl PrefixSumCheckpoints {
    pub(super) fn sum_before<I: VecIndex, T: VecValue>(
        &self,
        source: &impl ReadableVec<I, T>,
        end: usize,
        value: impl Fn(&T) -> u64,
    ) -> u64 {
        let revision = source.data_revision();
        let checkpoint = end / INTERVAL;
        let from = checkpoint * INTERVAL;
        let sum = {
            let mut state = self.0.write();
            if revision.is_none() || state.revision != revision {
                state.revision = revision;
                state.sums.clear();
            }
            if state.sums.is_empty() {
                state.sums.push(0);
            }
            let mut index = (state.sums.len() - 1) * INTERVAL;
            let mut sum = *state.sums.last().unwrap();
            source.for_each_chunk_at(index, from, &mut |_, values| {
                for item in values {
                    sum = sum.checked_add(value(item)).expect("prefix sum overflow");
                    index += 1;
                    if index.is_multiple_of(INTERVAL) {
                        state.sums.push(sum);
                    }
                }
            });
            state.sums[checkpoint]
        };
        source.fold_range_at(from, end, sum, |sum, item| {
            sum.checked_add(value(&item)).expect("prefix sum overflow")
        })
    }
}
