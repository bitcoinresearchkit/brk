use std::ops::Range;

#[derive(Clone, Copy)]
pub(crate) enum Request<'a> {
    Range(usize, usize),
    Sorted(&'a [usize]),
}

impl<'a> Request<'a> {
    pub(crate) fn clamp(self, len: usize) -> Self {
        match self {
            Self::Range(from, to) => Self::Range(from.min(len), to.min(len)),
            Self::Sorted(indices) => {
                Self::Sorted(&indices[..indices.partition_point(|&i| i < len)])
            }
        }
    }

    pub(super) fn ranges(self) -> Vec<Range<usize>> {
        match self {
            Self::Range(from, to) => {
                if from < to {
                    vec![from..to]
                } else {
                    Vec::new()
                }
            }
            Self::Sorted(indices) => {
                let mut ranges: Vec<Range<usize>> = Vec::new();
                for &index in indices {
                    if let Some(last) = ranges.last_mut()
                        && index <= last.end
                    {
                        last.end = last.end.max(index + 1);
                    } else {
                        ranges.push(index..index + 1);
                    }
                }
                ranges
            }
        }
    }
}
