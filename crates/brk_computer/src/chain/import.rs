use std::path::Path;

use brk_error::Result;
use brk_indexer::Indexer;
use brk_traversable::Traversable;
use brk_types::Version;
use vecdb::{Database, PAGE_SIZE};

use crate::{indexes, price};

use super::{
    BlockVecs, CoinbaseVecs, EpochVecs, MiningVecs, OutputTypeVecs, TransactionVecs, Vecs,
    VolumeVecs,
};

impl Vecs {
    pub fn forced_import(
        parent_path: &Path,
        parent_version: Version,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
    ) -> Result<Self> {
        let db = Database::open(&parent_path.join(super::DB_NAME))?;
        db.set_min_len(PAGE_SIZE * 50_000_000)?;

        let version = parent_version + Version::ZERO;
        let compute_dollars = price.is_some();

        let block = BlockVecs::forced_import(&db, version, indexer, indexes)?;
        let epoch = EpochVecs::forced_import(&db, version, indexes)?;
        let mining = MiningVecs::forced_import(&db, version, indexer, indexes)?;
        let coinbase = CoinbaseVecs::forced_import(&db, version, indexes, compute_dollars)?;
        let transaction = TransactionVecs::forced_import(&db, version, indexer, indexes, price)?;
        let output_type = OutputTypeVecs::forced_import(&db, version, indexes)?;
        let volume = VolumeVecs::forced_import(&db, version, indexes, compute_dollars)?;

        let this = Self {
            db,
            block,
            epoch,
            mining,
            coinbase,
            transaction,
            output_type,
            volume,
        };

        this.db.retain_regions(
            this.iter_any_exportable()
                .flat_map(|v| v.region_names())
                .collect(),
        )?;
        this.db.compact()?;

        Ok(this)
    }
}
