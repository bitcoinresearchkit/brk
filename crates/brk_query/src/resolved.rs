use brk_types::{Etag, Format};
use vecdb::AnyExportableVec;

/// A resolved metric query ready for formatting.
/// Contains the vecs and metadata needed to build an ETag or format the output.
pub struct ResolvedQuery {
    pub(crate) vecs: Vec<&'static dyn AnyExportableVec>,
    pub(crate) format: Format,
    pub(crate) version: u64,
    pub(crate) total: usize,
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) height: u32,
}

impl ResolvedQuery {
    pub fn etag(&self) -> Etag {
        Etag::from_metric(self.version, self.total, self.start, self.end, self.height)
    }

    pub fn format(&self) -> Format {
        self.format
    }
}
