use brk_error::{Error, Result};
use brk_types::RecommendedFees;

use crate::Query;

pub fn get_recommended_fees(query: &Query) -> Result<RecommendedFees> {
    query
        .mempool()
        .map(|mempool| mempool.get_fees())
        .ok_or(Error::MempoolNotAvailable)
}
