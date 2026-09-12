use brk_types::Cents;
use rangeindex::RangeMap;
use vecdb::{ReadableVec, VecIndex};

use super::AggFold;

/// First price in a populated period, or the preceding close in an empty one.
pub struct Open;

impl Open {
    fn source_index<SI: VecIndex>(
        mapping: &[SI],
        index: usize,
        source_len: usize,
    ) -> Option<usize> {
        let first = mapping[index].to_usize().min(source_len);
        let end = mapping
            .get(index + 1)
            .map_or(source_len, |i| i.to_usize().min(source_len));
        if first < end {
            Some(first)
        } else {
            first.checked_sub(1)
        }
    }

    fn read_into<SI: VecIndex, MI: VecIndex, S: ReadableVec<SI, Cents> + ?Sized>(
        source: &S,
        mapping: &RangeMap<SI, MI>,
        indices: impl Iterator<Item = usize>,
        out: &mut Vec<Cents>,
    ) {
        let source_len = source.visible_len();
        let mapping = mapping.as_slice();
        let mut requested = Vec::new();
        let slots: Vec<_> = indices
            .map(|index| {
                Self::source_index(mapping, index, source_len).map(|index| {
                    if requested.last() != Some(&index) {
                        requested.push(index);
                    }
                    requested.len() - 1
                })
            })
            .collect();
        let values = source.read_sorted_at(&requested);
        out.extend(
            slots
                .into_iter()
                .map(|slot| slot.map_or(Cents::ZERO, |slot| values[slot])),
        );
    }
}

impl<SI: VecIndex> AggFold<Cents, SI, Cents> for Open {
    fn try_fold<
        MI: VecIndex,
        S: ReadableVec<SI, Cents> + ?Sized,
        B,
        E,
        F: FnMut(B, Cents) -> Result<B, E>,
    >(
        source: &S,
        mapping: &RangeMap<SI, MI>,
        from: usize,
        to: usize,
        init: B,
        f: F,
    ) -> Result<B, E> {
        let mut values = Vec::with_capacity(to - from);
        Self::read_into(source, mapping, from..to, &mut values);
        values.into_iter().try_fold(init, f)
    }

    fn collect_one<MI: VecIndex, S: ReadableVec<SI, Cents> + ?Sized>(
        source: &S,
        mapping: &RangeMap<SI, MI>,
        index: usize,
    ) -> Option<Cents> {
        if index >= mapping.len() {
            return None;
        }
        Some(
            Self::source_index(mapping.as_slice(), index, source.visible_len())
                .and_then(|index| source.collect_one_at(index))
                .unwrap_or_default(),
        )
    }

    fn read_sorted_into<MI: VecIndex, S: ReadableVec<SI, Cents> + ?Sized>(
        source: &S,
        mapping: &RangeMap<SI, MI>,
        indices: &[usize],
        out: &mut Vec<Cents>,
    ) {
        if let &[index] = indices {
            out.push(Self::collect_one(source, mapping, index).unwrap());
            return;
        }
        Self::read_into(source, mapping, indices.iter().copied(), out);
    }
}
