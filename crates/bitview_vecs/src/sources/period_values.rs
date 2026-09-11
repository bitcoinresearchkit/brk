use brk_types::Height;
use vecdb::{ReadableVec, VecIndex, VecValue};

/// Visit each requested period at its first source value, filling missing
/// periods from the next available value in the monotonic source.
pub(super) fn try_for_each_period<I, T, E>(
    source: &(impl ReadableVec<Height, T> + ?Sized),
    mut periods: impl ExactSizeIterator<Item = usize>,
    period_from_value: impl Fn(T) -> I,
    mut each: impl FnMut(I, Height, T) -> Result<(), E>,
) -> Result<(), E>
where
    I: VecIndex,
    T: VecValue + Copy,
{
    let Some(mut target) = periods.next() else {
        return Ok(());
    };

    let source_len = source.len();
    let seek = |mut from: usize, target: usize| {
        if target == 0 {
            return from;
        }
        let mut to = source_len;
        while from < to {
            let mid = from + (to - from) / 2;
            let value = source.collect_one_at(mid).unwrap();
            if period_from_value(value).to_usize() < target {
                from = mid + 1;
            } else {
                to = mid;
            }
        }
        from
    };
    let mut source_from = seek(0, target);
    if periods.len() == 0 {
        if let Some(value) = source.collect_one_at(source_from) {
            each(I::from(target), Height::from(source_from), value)?;
        }
        return Ok(());
    }

    let chunk_size = source.cursor_chunk_size();
    let mut buf = Vec::with_capacity(chunk_size);
    let mut previous_period = None;

    while source_from < source_len {
        let source_to = (source_from + chunk_size).min(source_len);
        buf.clear();
        source.read_into_at(source_from, source_to, &mut buf);
        let last_period = period_from_value(*buf.last().unwrap()).to_usize();
        // A whole chunk without a requested period marks a gap worth seeking
        // past. Otherwise nearby requests keep sharing the forward scan.
        if target > last_period {
            source_from = seek(source_to, target);
            continue;
        }

        for (offset, value) in buf.iter().copied().enumerate() {
            let period = period_from_value(value).to_usize();
            debug_assert!(previous_period.is_none_or(|previous| previous <= period));
            previous_period = Some(period);
            while target <= period {
                each(I::from(target), Height::from(source_from + offset), value)?;
                let Some(next) = periods.next() else {
                    return Ok(());
                };
                debug_assert!(next >= target);
                target = next;
            }
        }

        source_from = source_to;
    }

    Ok(())
}
