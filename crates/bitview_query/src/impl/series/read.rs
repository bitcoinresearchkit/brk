use bitview_plugin::PluginReadGuard;
use brk_error::{Error, Result};
use brk_types::Lengths;
use vecdb::{AnyExportableVec, BoundedVec, ReadBounds};

use crate::{Query, vecs::SeriesEntry};

/// Selected series and their published read protection.
///
/// Column readers borrow this view, so they cannot outlive its plugin guards.
/// The underlying unbounded vectors and bounds are deliberately private.
pub struct SeriesRead {
    vecs: Vec<&'static dyn AnyExportableVec>,
    bounds: ReadBounds,
    safe: Lengths,
    is_mutable: bool,
    _guard: PluginReadGuard,
}

impl SeriesRead {
    pub(super) fn new(query: &Query, entries: Vec<SeriesEntry<'static>>) -> Result<Self> {
        let guard = query.series_guard(&entries)?;
        let safe = query.safe_lengths();
        let bounds = query.read_bounds(safe);
        let is_mutable = entries.iter().any(|entry| entry.is_mutable());
        let vecs = entries
            .into_iter()
            .map(SeriesEntry::vec)
            .collect::<Vec<_>>();
        for vec in &vecs {
            bounds
                .bind(*vec)
                .ok_or(Error::Internal("Missing published series bound"))?;
        }
        Ok(Self {
            vecs,
            bounds,
            safe,
            is_mutable,
            _guard: guard,
        })
    }

    pub(super) fn safe_lengths(&self) -> Lengths {
        self.safe
    }

    pub(super) fn is_mutable(&self) -> bool {
        self.is_mutable
    }

    pub(super) fn bind<'a>(&'a self, vec: &'a dyn AnyExportableVec) -> Result<BoundedVec<'a>> {
        self.bounds
            .bind(vec)
            .ok_or(Error::Internal("Missing published series bound"))
    }

    pub fn columns(&self) -> impl ExactSizeIterator<Item = BoundedVec<'_>> + '_ {
        self.vecs.iter().map(|vec| {
            self.bounds
                .bind(*vec)
                .expect("selected series bounds were checked at construction")
        })
    }
}
