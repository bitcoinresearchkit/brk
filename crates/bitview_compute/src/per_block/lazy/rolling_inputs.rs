use brk_types::Height;
use vecdb::{ReadableBoxedVec, VecIndex, VecValue};

/// Read overlapping windows once, and skip the unused gap for disjoint windows.
/// Complete source reads before invoking consumers, since source chunks can
/// hold non-reentrant publication locks.
pub(super) fn for_each_rolling_input<S: VecValue>(
    source: &ReadableBoxedVec<Height, S>,
    from: usize,
    to: usize,
    starts: &[Height],
    mut visit: impl FnMut(usize, &[S], usize, &[S]),
) {
    let previous = |start: &Height| start.to_usize().checked_sub(1);
    let first = starts.iter().find_map(previous);
    let last = starts.iter().rev().find_map(previous);
    if let Some((first, last)) = first.zip(last) {
        if last + 1 >= from {
            let read_from = first.min(from);
            let values = source.collect_range_dyn(read_from, to);
            visit(from, &values[from - read_from..], read_from, &values);
        } else {
            let values = source.collect_range_dyn(first, last + 1);
            let current = source.collect_range_dyn(from, to);
            visit(from, &current, first, &values);
        }
    } else {
        let current = source.collect_range_dyn(from, to);
        visit(from, &current, 0, &[]);
    }
}
