use brk_indexer::Indexer;
use brk_types::Version;
use vecdb::{IterableCloneableVec, LazyVecFrom1};

use super::Vecs;

impl Vecs {
    pub fn forced_import(version: Version, indexer: &Indexer) -> Self {
        Self {
            p2pk33addressindex_to_p2pk33addressindex: LazyVecFrom1::init(
                "p2pk33addressindex",
                version,
                indexer.vecs.address.p2pk33addressindex_to_p2pk33bytes.boxed_clone(),
                |index, _| Some(index),
            ),
            p2pk65addressindex_to_p2pk65addressindex: LazyVecFrom1::init(
                "p2pk65addressindex",
                version,
                indexer.vecs.address.p2pk65addressindex_to_p2pk65bytes.boxed_clone(),
                |index, _| Some(index),
            ),
            p2pkhaddressindex_to_p2pkhaddressindex: LazyVecFrom1::init(
                "p2pkhaddressindex",
                version,
                indexer.vecs.address.p2pkhaddressindex_to_p2pkhbytes.boxed_clone(),
                |index, _| Some(index),
            ),
            p2shaddressindex_to_p2shaddressindex: LazyVecFrom1::init(
                "p2shaddressindex",
                version,
                indexer.vecs.address.p2shaddressindex_to_p2shbytes.boxed_clone(),
                |index, _| Some(index),
            ),
            p2traddressindex_to_p2traddressindex: LazyVecFrom1::init(
                "p2traddressindex",
                version,
                indexer.vecs.address.p2traddressindex_to_p2trbytes.boxed_clone(),
                |index, _| Some(index),
            ),
            p2wpkhaddressindex_to_p2wpkhaddressindex: LazyVecFrom1::init(
                "p2wpkhaddressindex",
                version,
                indexer.vecs.address.p2wpkhaddressindex_to_p2wpkhbytes.boxed_clone(),
                |index, _| Some(index),
            ),
            p2wshaddressindex_to_p2wshaddressindex: LazyVecFrom1::init(
                "p2wshaddressindex",
                version,
                indexer.vecs.address.p2wshaddressindex_to_p2wshbytes.boxed_clone(),
                |index, _| Some(index),
            ),
            p2aaddressindex_to_p2aaddressindex: LazyVecFrom1::init(
                "p2aaddressindex",
                version,
                indexer.vecs.address.p2aaddressindex_to_p2abytes.boxed_clone(),
                |index, _| Some(index),
            ),
            p2msoutputindex_to_p2msoutputindex: LazyVecFrom1::init(
                "p2msoutputindex",
                version,
                indexer.vecs.output.p2msoutputindex_to_txindex.boxed_clone(),
                |index, _| Some(index),
            ),
            emptyoutputindex_to_emptyoutputindex: LazyVecFrom1::init(
                "emptyoutputindex",
                version,
                indexer.vecs.output.emptyoutputindex_to_txindex.boxed_clone(),
                |index, _| Some(index),
            ),
            unknownoutputindex_to_unknownoutputindex: LazyVecFrom1::init(
                "unknownoutputindex",
                version,
                indexer.vecs.output.unknownoutputindex_to_txindex.boxed_clone(),
                |index, _| Some(index),
            ),
            opreturnindex_to_opreturnindex: LazyVecFrom1::init(
                "opreturnindex",
                version,
                indexer.vecs.output.opreturnindex_to_txindex.boxed_clone(),
                |index, _| Some(index),
            ),
        }
    }
}
