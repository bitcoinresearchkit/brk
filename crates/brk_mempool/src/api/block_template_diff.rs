use std::sync::Arc;

use brk_types::{NextBlockHash, Transaction};

use super::BlockTemplateSource;

/// A validated historical template captured for one diff request.
pub struct ResolvedBlockTemplateDiff {
    since: NextBlockHash,
    past: Arc<[Arc<Transaction>]>,
    source: BlockTemplateSource,
}

impl ResolvedBlockTemplateDiff {
    #[must_use]
    pub fn since(&self) -> NextBlockHash {
        self.since
    }

    #[must_use]
    pub fn source(&self) -> &BlockTemplateSource {
        &self.source
    }
}
pub trait ApiBlockTemplateDiffResolvedBlockTemplateDiffInternal: Sized {
    fn new(
        since: NextBlockHash,
        past: Arc<[Arc<Transaction>]>,
        source: BlockTemplateSource,
    ) -> Self;
    fn into_parts(self) -> (NextBlockHash, Arc<[Arc<Transaction>]>, BlockTemplateSource);
}
impl ApiBlockTemplateDiffResolvedBlockTemplateDiffInternal for ResolvedBlockTemplateDiff {
    fn new(
        since: NextBlockHash,
        past: Arc<[Arc<Transaction>]>,
        source: BlockTemplateSource,
    ) -> Self {
        Self {
            since,
            past,
            source,
        }
    }
    fn into_parts(self) -> (NextBlockHash, Arc<[Arc<Transaction>]>, BlockTemplateSource) {
        (self.since, self.past, self.source)
    }
}
