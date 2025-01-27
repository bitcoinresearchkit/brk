use axum::extract::Query;
use color_eyre::eyre::eyre;

use crate::server::api::handlers::DatasetParams;

pub enum DatasetRange {
    All,
    Chunk(DatasetRangeChunk),
}

impl TryFrom<&Query<DatasetParams>> for DatasetRange {
    type Error = color_eyre::Report;

    fn try_from(query: &Query<DatasetParams>) -> Result<Self, Self::Error> {
        if let Some(chunk) = query.chunk {
            if query.all.is_some() {
                Err(eyre!("chunk and all are exclusive"))
            } else {
                Ok(Self::Chunk(DatasetRangeChunk::Chunk(chunk)))
            }
        } else if query.all.is_some() {
            Ok(Self::All)
        } else {
            Ok(Self::Chunk(DatasetRangeChunk::Last))
        }
    }
}

pub enum DatasetRangeChunk {
    Chunk(usize),
    Last,
}
