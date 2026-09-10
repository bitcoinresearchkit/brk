use brk_types::Height;
use vecdb::{ReadableVec, VecIndex, VecValue};

/// Visit each requested period at its first source value, filling missing
/// periods from the next available value in the monotonic source.
pub(super) fn try_for_each_period<I, T, E>(
    source: &(impl ReadableVec<Height, T> + ?Sized),
    from: usize,
    to: usize,
    period_from_value: impl Fn(T) -> I,
    mut each: impl FnMut(I, Height, T) -> Result<(), E>,
) -> Result<(), E>
where
    I: VecIndex,
    T: VecValue + Copy,
{
    if from >= to {
        return Ok(());
    }

    let chunk_size = source.cursor_chunk_size();
    let source_len = source.len();
    let mut source_from = 0;
    let mut target = from;
    let mut buf = Vec::with_capacity(chunk_size);
    let mut previous_period = None;

    while source_from < source_len && target < to {
        let source_to = (source_from + chunk_size).min(source_len);
        buf.clear();
        source.read_into_at(source_from, source_to, &mut buf);

        for (offset, value) in buf.iter().copied().enumerate() {
            let period = period_from_value(value).to_usize();
            debug_assert!(previous_period.is_none_or(|previous| previous <= period));
            previous_period = Some(period);
            while target <= period && target < to {
                each(I::from(target), Height::from(source_from + offset), value)?;
                target += 1;
            }
        }

        source_from = source_to;
    }

    Ok(())
}
