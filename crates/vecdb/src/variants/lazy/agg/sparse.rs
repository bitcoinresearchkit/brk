use crate::{AggFold, ReadableVec, VecIndex, VecValue};

/// Sparse aggregation: emits `Option<T>` per output index.
///
/// `Some(last_value)` when the range contains source elements,
/// `None` when the range is empty.
pub struct Sparse;

impl<T: VecValue, SI: VecIndex> AggFold<Option<T>, SI, SI, T> for Sparse {
    #[inline]
    fn try_fold<S: ReadableVec<SI, T> + ?Sized, B, E, F: FnMut(B, Option<T>) -> Result<B, E>>(
        source: &S,
        mapping: &[SI],
        from: usize,
        to: usize,
        init: B,
        mut f: F,
    ) -> Result<B, E> {
        let source_len = source.visible_len();

        let mut indices: Vec<usize> = Vec::with_capacity(to - from);
        let mut slot_map: Vec<Option<u32>> = Vec::with_capacity(to - from);

        (from..to).for_each(|idx| {
            let current_first = mapping[idx].to_usize();
            let next_first = mapping
                .get(idx + 1)
                .map(|h| h.to_usize())
                .unwrap_or(source_len)
                .min(source_len);

            if next_first == 0 || current_first >= next_first {
                slot_map.push(None);
            } else {
                slot_map.push(Some(indices.len() as u32));
                indices.push(next_first - 1);
            }
        });

        let values = source.read_sorted_at(&indices);

        slot_map.iter().try_fold(init, |acc, slot| match slot {
            None => f(acc, None),
            &Some(vi) => f(acc, Some(values[vi as usize].clone())),
        })
    }

    #[inline]
    fn collect_one<S: ReadableVec<SI, T> + ?Sized>(
        source: &S,
        mapping: &[SI],
        index: usize,
    ) -> Option<Option<T>> {
        let source_len = source.visible_len();
        let current_first = mapping[index].to_usize();
        let next_first = mapping
            .get(index + 1)
            .map(|h| h.to_usize())
            .unwrap_or(source_len)
            .min(source_len);

        if next_first == 0 || current_first >= next_first {
            return Some(None);
        }
        Some(source.collect_one_at(next_first - 1))
    }
}

#[cfg(test)]
mod tests {
    use super::Sparse;
    use crate::{AggFold, BytesVec, ImportableVec, ReadBounds, Version, WritableVec};

    #[test]
    fn clamps_partial_final_range_to_source_length() {
        let temp = tempfile::tempdir().unwrap();
        let db = crate::Database::open(temp.path()).unwrap();
        let mut source: BytesVec<usize, u64> =
            BytesVec::forced_import(&db, "source", Version::ONE).unwrap();

        for value in [10, 20, 30] {
            source.push(value);
        }

        let mapping = [0, 2, 4];
        let values = Sparse::fold(
            &source,
            &mapping,
            0,
            mapping.len(),
            Vec::new(),
            |mut values, value| {
                values.push(value);
                values
            },
        );

        assert_eq!(values, [Some(20), Some(30), None]);
        assert_eq!(Sparse::collect_one(&source, &mapping, 1), Some(Some(30)));
        assert_eq!(Sparse::collect_one(&source, &mapping, 2), Some(None));
    }

    #[test]
    fn final_range_uses_the_published_source_bound() {
        let temp = tempfile::tempdir().unwrap();
        let db = crate::Database::open(temp.path()).unwrap();
        let mut source: BytesVec<usize, u64> =
            BytesVec::forced_import(&db, "bounded_source", Version::ONE).unwrap();

        for value in [10, 20, 30] {
            source.push(value);
        }

        let mut bounds = ReadBounds::new();
        bounds.set("usize", 2);
        let values = bounds.scope(|| {
            Sparse::fold(&source, &[0], 0, 1, Vec::new(), |mut values, value| {
                values.push(value);
                values
            })
        });

        assert_eq!(values, [Some(20)]);
    }

    #[test]
    fn sparse_ranges_and_early_errors_match_reference_buckets() {
        let temp = tempfile::tempdir().unwrap();
        let db = crate::Database::open(temp.path()).unwrap();
        let mut source: BytesVec<usize, u64> =
            BytesVec::forced_import(&db, "buckets", Version::ONE).unwrap();
        let values = [10, 20, 30, 40];
        for value in values {
            source.push(value);
        }

        for mapping in [
            vec![],
            vec![0],
            vec![0, 0, 0, 0],
            vec![0, 1, 2, 3],
            vec![0, 2, 2, 3, 100],
            vec![0, 0, 4, 4, 4],
        ] {
            for source_len in [0, 2, values.len()] {
                let expected: Vec<_> = mapping
                    .iter()
                    .enumerate()
                    .map(|(i, &first)| {
                        let end = mapping
                            .get(i + 1)
                            .copied()
                            .unwrap_or(source_len)
                            .min(source_len);
                        (first..end).last().map(|index| values[index])
                    })
                    .collect();
                let mut bounds = ReadBounds::new();
                bounds.set("usize", source_len);
                bounds.scope(|| {
                    for (index, &expected) in expected.iter().enumerate() {
                        assert_eq!(
                            Sparse::collect_one(&source, &mapping, index),
                            Some(expected)
                        );
                    }
                    for from in 0..=mapping.len() {
                        for to in from..=mapping.len() {
                            let actual = Sparse::fold(
                                &source,
                                &mapping,
                                from,
                                to,
                                Vec::new(),
                                |mut acc, value| {
                                    acc.push(value);
                                    acc
                                },
                            );
                            assert_eq!(actual, expected[from..to]);
                            for stop in 0..=to - from {
                                let mut visited = Vec::new();
                                let result = Sparse::try_fold(
                                    &source,
                                    &mapping,
                                    from,
                                    to,
                                    (),
                                    |(), value| {
                                        visited.push(value);
                                        if visited.len() > stop {
                                            Err(stop)
                                        } else {
                                            Ok(())
                                        }
                                    },
                                );
                                assert_eq!(
                                    result,
                                    if stop < to - from { Err(stop) } else { Ok(()) }
                                );
                                assert_eq!(visited, expected[from..to.min(from + stop + 1)]);
                            }
                        }
                    }
                });
            }
        }
    }
}
