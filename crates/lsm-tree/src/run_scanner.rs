use std::sync::Arc;

use crate::{Error, InternalValue, Result, Table, table::Scanner, version::Run};

/// Scans through a disjoint run for compaction.
pub struct RunScanner {
    tables: Arc<Run<Table>>,
    lo: usize,
    hi: usize,
    lo_reader: Option<Scanner>,
}

impl RunScanner {
    pub fn culled(run: Arc<Run<Table>>, (lo, hi): (Option<usize>, Option<usize>)) -> Result<Self> {
        let lo = lo.unwrap_or_default();
        let hi = hi.unwrap_or(run.len() - 1);
        let lo_reader = run.get(lo).ok_or(Error::Unrecoverable)?.scan()?;

        Ok(Self {
            tables: run,
            lo,
            hi,
            lo_reader: Some(lo_reader),
        })
    }
}

impl Iterator for RunScanner {
    type Item = Result<InternalValue>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let reader = self.lo_reader.as_mut()?;
            if let Some(item) = reader.next() {
                return Some(item);
            }

            self.lo += 1;
            if self.lo > self.hi {
                self.lo_reader = None;
                return None;
            }

            self.lo_reader = Some(fail_iter!(
                self.tables
                    .get(self.lo)
                    .ok_or(Error::Unrecoverable)
                    .and_then(Table::scan)
            ));
        }
    }
}
