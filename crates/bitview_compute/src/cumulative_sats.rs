use brk_error::Result;
use brk_exit::Exit;
use brk_types::{Height, Sats};
use vecdb::{AnyVec, CachePolicy, EagerVec, PcoVec, ReadableVec, VecIndex, VecValue, WritableVec};

#[allow(clippy::too_many_arguments)]
pub fn compute_cumulative_sats_from_indexes<A, B, P: CachePolicy>(
    target: &mut EagerVec<PcoVec<Height, Sats, P>>,
    max_from: Height,
    first_indexes: &impl ReadableVec<Height, A>,
    indexes_count: &impl ReadableVec<Height, B>,
    source: &impl ReadableVec<A, Sats>,
    mut filter: impl FnMut(&Sats) -> bool,
    exit: &Exit,
) -> Result<()>
where
    A: VecIndex + VecValue,
    B: VecValue,
    usize: From<B>,
{
    target.validate_computed_version_or_reset(
        first_indexes.version() + indexes_count.version() + source.version(),
    )?;
    target.truncate_if_needed(max_from)?;
    target.repeat_until_complete(exit, |target| {
        let skip = target.len();
        let end = target.batch_end(indexes_count.len());
        if skip >= end || skip >= first_indexes.len() {
            return Ok(());
        }

        let source_start = first_indexes.collect_one_at(skip).unwrap().to_usize();
        let counts: Vec<usize> = indexes_count
            .collect_range_at(skip, end)
            .into_iter()
            .map(usize::from)
            .collect();
        let source_end = source_start + counts.iter().sum::<usize>();
        let mut cumulative = skip
            .checked_sub(1)
            .and_then(|index| target.collect_one_at(index))
            .unwrap_or_default();
        let mut group_index = 0;

        while group_index < counts.len() && counts[group_index] == 0 {
            target.push(cumulative);
            group_index += 1;
        }

        if group_index < counts.len() {
            let mut remaining = counts[group_index];
            source.fold_range_at(source_start, source_end, Sats::ZERO, |sum, value| {
                let sum = if filter(&value) { sum + value } else { sum };
                remaining -= 1;
                if remaining == 0 {
                    cumulative += sum;
                    target.push(cumulative);
                    group_index += 1;
                    while group_index < counts.len() && counts[group_index] == 0 {
                        target.push(cumulative);
                        group_index += 1;
                    }
                    if group_index < counts.len() {
                        remaining = counts[group_index];
                    }
                    Sats::ZERO
                } else {
                    sum
                }
            });
        }

        Ok(())
    })?;
    Ok(())
}
