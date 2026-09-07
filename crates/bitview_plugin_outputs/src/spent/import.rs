use brk_error::Result;

use brk_types::Version;
use vecdb::{BytesVec, Database, ImportableVec, MutableVec};

use super::Vecs;

pub fn forced_import(db: &Database, version: Version) -> Result<Vecs> {
    Ok(Vecs {
        txin_index: MutableVec::<BytesVec<_, _>>::forced_import(db, "txin_index", version)?,
    })
}
