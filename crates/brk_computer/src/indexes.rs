use std::{ops::Deref, path::Path};

use brk_error::Result;
use brk_indexer::Indexer;
use brk_traversable::Traversable;
use brk_types::{
    Date, DateIndex, DecadeIndex, DifficultyEpoch, EmptyOutputIndex, HalvingEpoch, Height,
    MonthIndex, OpReturnIndex, OutPoint, P2AAddressIndex, P2ABytes, P2MSOutputIndex,
    P2PK33AddressIndex, P2PK33Bytes, P2PK65AddressIndex, P2PK65Bytes, P2PKHAddressIndex,
    P2PKHBytes, P2SHAddressIndex, P2SHBytes, P2TRAddressIndex, P2TRBytes, P2WPKHAddressIndex,
    P2WPKHBytes, P2WSHAddressIndex, P2WSHBytes, QuarterIndex, Sats, SemesterIndex, StoredU64,
    Timestamp, TxInIndex, TxIndex, TxOutIndex, Txid, UnknownOutputIndex, Version, WeekIndex,
    YearIndex,
};
use vecdb::{
    Database, EagerVec, Exit, GenericStoredVec, ImportableVec, IterableCloneableVec, LazyVecFrom1,
    PAGE_SIZE, PcoVec, TypedVecIterator, unlikely,
};

const VERSION: Version = Version::ZERO;

#[derive(Clone, Traversable)]
pub struct Vecs {
    db: Database,

    pub dateindex_to_date: EagerVec<PcoVec<DateIndex, Date>>,
    pub dateindex_to_dateindex: EagerVec<PcoVec<DateIndex, DateIndex>>,
    pub dateindex_to_first_height: EagerVec<PcoVec<DateIndex, Height>>,
    pub dateindex_to_height_count: EagerVec<PcoVec<DateIndex, StoredU64>>,
    pub dateindex_to_monthindex: EagerVec<PcoVec<DateIndex, MonthIndex>>,
    pub dateindex_to_weekindex: EagerVec<PcoVec<DateIndex, WeekIndex>>,
    pub decadeindex_to_decadeindex: EagerVec<PcoVec<DecadeIndex, DecadeIndex>>,
    pub decadeindex_to_first_yearindex: EagerVec<PcoVec<DecadeIndex, YearIndex>>,
    pub decadeindex_to_yearindex_count: EagerVec<PcoVec<DecadeIndex, StoredU64>>,
    pub difficultyepoch_to_difficultyepoch: EagerVec<PcoVec<DifficultyEpoch, DifficultyEpoch>>,
    pub difficultyepoch_to_first_height: EagerVec<PcoVec<DifficultyEpoch, Height>>,
    pub difficultyepoch_to_height_count: EagerVec<PcoVec<DifficultyEpoch, StoredU64>>,
    pub emptyoutputindex_to_emptyoutputindex:
        LazyVecFrom1<EmptyOutputIndex, EmptyOutputIndex, EmptyOutputIndex, TxIndex>,
    pub halvingepoch_to_first_height: EagerVec<PcoVec<HalvingEpoch, Height>>,
    pub halvingepoch_to_halvingepoch: EagerVec<PcoVec<HalvingEpoch, HalvingEpoch>>,
    pub height_to_date: EagerVec<PcoVec<Height, Date>>,
    pub height_to_date_fixed: EagerVec<PcoVec<Height, Date>>,
    pub height_to_dateindex: EagerVec<PcoVec<Height, DateIndex>>,
    pub height_to_difficultyepoch: EagerVec<PcoVec<Height, DifficultyEpoch>>,
    pub height_to_halvingepoch: EagerVec<PcoVec<Height, HalvingEpoch>>,
    pub height_to_height: EagerVec<PcoVec<Height, Height>>,
    pub height_to_timestamp_fixed: EagerVec<PcoVec<Height, Timestamp>>,
    pub height_to_txindex_count: EagerVec<PcoVec<Height, StoredU64>>,
    pub monthindex_to_dateindex_count: EagerVec<PcoVec<MonthIndex, StoredU64>>,
    pub monthindex_to_first_dateindex: EagerVec<PcoVec<MonthIndex, DateIndex>>,
    pub monthindex_to_monthindex: EagerVec<PcoVec<MonthIndex, MonthIndex>>,
    pub monthindex_to_quarterindex: EagerVec<PcoVec<MonthIndex, QuarterIndex>>,
    pub monthindex_to_semesterindex: EagerVec<PcoVec<MonthIndex, SemesterIndex>>,
    pub monthindex_to_yearindex: EagerVec<PcoVec<MonthIndex, YearIndex>>,
    pub opreturnindex_to_opreturnindex:
        LazyVecFrom1<OpReturnIndex, OpReturnIndex, OpReturnIndex, TxIndex>,
    pub p2aaddressindex_to_p2aaddressindex:
        LazyVecFrom1<P2AAddressIndex, P2AAddressIndex, P2AAddressIndex, P2ABytes>,
    pub p2msoutputindex_to_p2msoutputindex:
        LazyVecFrom1<P2MSOutputIndex, P2MSOutputIndex, P2MSOutputIndex, TxIndex>,
    pub p2pk33addressindex_to_p2pk33addressindex:
        LazyVecFrom1<P2PK33AddressIndex, P2PK33AddressIndex, P2PK33AddressIndex, P2PK33Bytes>,
    pub p2pk65addressindex_to_p2pk65addressindex:
        LazyVecFrom1<P2PK65AddressIndex, P2PK65AddressIndex, P2PK65AddressIndex, P2PK65Bytes>,
    pub p2pkhaddressindex_to_p2pkhaddressindex:
        LazyVecFrom1<P2PKHAddressIndex, P2PKHAddressIndex, P2PKHAddressIndex, P2PKHBytes>,
    pub p2shaddressindex_to_p2shaddressindex:
        LazyVecFrom1<P2SHAddressIndex, P2SHAddressIndex, P2SHAddressIndex, P2SHBytes>,
    pub p2traddressindex_to_p2traddressindex:
        LazyVecFrom1<P2TRAddressIndex, P2TRAddressIndex, P2TRAddressIndex, P2TRBytes>,
    pub p2wpkhaddressindex_to_p2wpkhaddressindex:
        LazyVecFrom1<P2WPKHAddressIndex, P2WPKHAddressIndex, P2WPKHAddressIndex, P2WPKHBytes>,
    pub p2wshaddressindex_to_p2wshaddressindex:
        LazyVecFrom1<P2WSHAddressIndex, P2WSHAddressIndex, P2WSHAddressIndex, P2WSHBytes>,
    pub quarterindex_to_first_monthindex: EagerVec<PcoVec<QuarterIndex, MonthIndex>>,
    pub quarterindex_to_monthindex_count: EagerVec<PcoVec<QuarterIndex, StoredU64>>,
    pub quarterindex_to_quarterindex: EagerVec<PcoVec<QuarterIndex, QuarterIndex>>,
    pub semesterindex_to_first_monthindex: EagerVec<PcoVec<SemesterIndex, MonthIndex>>,
    pub semesterindex_to_monthindex_count: EagerVec<PcoVec<SemesterIndex, StoredU64>>,
    pub semesterindex_to_semesterindex: EagerVec<PcoVec<SemesterIndex, SemesterIndex>>,
    pub txindex_to_input_count: EagerVec<PcoVec<TxIndex, StoredU64>>,
    pub txindex_to_output_count: EagerVec<PcoVec<TxIndex, StoredU64>>,
    pub txindex_to_txindex: LazyVecFrom1<TxIndex, TxIndex, TxIndex, Txid>,
    pub txinindex_to_txinindex: LazyVecFrom1<TxInIndex, TxInIndex, TxInIndex, OutPoint>,
    pub txinindex_to_txoutindex: EagerVec<PcoVec<TxInIndex, TxOutIndex>>,
    pub txoutindex_to_txoutindex: LazyVecFrom1<TxOutIndex, TxOutIndex, TxOutIndex, Sats>,
    pub unknownoutputindex_to_unknownoutputindex:
        LazyVecFrom1<UnknownOutputIndex, UnknownOutputIndex, UnknownOutputIndex, TxIndex>,
    pub weekindex_to_dateindex_count: EagerVec<PcoVec<WeekIndex, StoredU64>>,
    pub weekindex_to_first_dateindex: EagerVec<PcoVec<WeekIndex, DateIndex>>,
    pub weekindex_to_weekindex: EagerVec<PcoVec<WeekIndex, WeekIndex>>,
    pub yearindex_to_decadeindex: EagerVec<PcoVec<YearIndex, DecadeIndex>>,
    pub yearindex_to_first_monthindex: EagerVec<PcoVec<YearIndex, MonthIndex>>,
    pub yearindex_to_monthindex_count: EagerVec<PcoVec<YearIndex, StoredU64>>,
    pub yearindex_to_yearindex: EagerVec<PcoVec<YearIndex, YearIndex>>,
}

impl Vecs {
    pub fn forced_import(
        parent: &Path,
        parent_version: Version,
        indexer: &Indexer,
    ) -> Result<Self> {
        let db = Database::open(&parent.join("indexes"))?;
        db.set_min_len(PAGE_SIZE * 10_000_000)?;

        let version = parent_version + VERSION;

        let this = Self {
            txinindex_to_txoutindex: EagerVec::forced_import(&db, "txoutindex", version)?,
            txoutindex_to_txoutindex: LazyVecFrom1::init(
                "txoutindex",
                version + Version::ZERO,
                indexer.vecs.txoutindex_to_value.boxed_clone(),
                |index, _| Some(index),
            ),
            txinindex_to_txinindex: LazyVecFrom1::init(
                "txinindex",
                version + Version::ZERO,
                indexer.vecs.txinindex_to_outpoint.boxed_clone(),
                |index, _| Some(index),
            ),
            p2pk33addressindex_to_p2pk33addressindex: LazyVecFrom1::init(
                "p2pk33addressindex",
                version + Version::ZERO,
                indexer.vecs.p2pk33addressindex_to_p2pk33bytes.boxed_clone(),
                |index, _| Some(index),
            ),
            p2pk65addressindex_to_p2pk65addressindex: LazyVecFrom1::init(
                "p2pk65addressindex",
                version + Version::ZERO,
                indexer.vecs.p2pk65addressindex_to_p2pk65bytes.boxed_clone(),
                |index, _| Some(index),
            ),
            p2pkhaddressindex_to_p2pkhaddressindex: LazyVecFrom1::init(
                "p2pkhaddressindex",
                version + Version::ZERO,
                indexer.vecs.p2pkhaddressindex_to_p2pkhbytes.boxed_clone(),
                |index, _| Some(index),
            ),
            p2shaddressindex_to_p2shaddressindex: LazyVecFrom1::init(
                "p2shaddressindex",
                version + Version::ZERO,
                indexer.vecs.p2shaddressindex_to_p2shbytes.boxed_clone(),
                |index, _| Some(index),
            ),
            p2traddressindex_to_p2traddressindex: LazyVecFrom1::init(
                "p2traddressindex",
                version + Version::ZERO,
                indexer.vecs.p2traddressindex_to_p2trbytes.boxed_clone(),
                |index, _| Some(index),
            ),
            p2wpkhaddressindex_to_p2wpkhaddressindex: LazyVecFrom1::init(
                "p2wpkhaddressindex",
                version + Version::ZERO,
                indexer.vecs.p2wpkhaddressindex_to_p2wpkhbytes.boxed_clone(),
                |index, _| Some(index),
            ),
            p2wshaddressindex_to_p2wshaddressindex: LazyVecFrom1::init(
                "p2wshaddressindex",
                version + Version::ZERO,
                indexer.vecs.p2wshaddressindex_to_p2wshbytes.boxed_clone(),
                |index, _| Some(index),
            ),
            p2aaddressindex_to_p2aaddressindex: LazyVecFrom1::init(
                "p2aaddressindex",
                version + Version::ZERO,
                indexer.vecs.p2aaddressindex_to_p2abytes.boxed_clone(),
                |index, _| Some(index),
            ),
            p2msoutputindex_to_p2msoutputindex: LazyVecFrom1::init(
                "p2msoutputindex",
                version + Version::ZERO,
                indexer.vecs.p2msoutputindex_to_txindex.boxed_clone(),
                |index, _| Some(index),
            ),
            emptyoutputindex_to_emptyoutputindex: LazyVecFrom1::init(
                "emptyoutputindex",
                version + Version::ZERO,
                indexer.vecs.emptyoutputindex_to_txindex.boxed_clone(),
                |index, _| Some(index),
            ),
            unknownoutputindex_to_unknownoutputindex: LazyVecFrom1::init(
                "unknownoutputindex",
                version + Version::ZERO,
                indexer.vecs.unknownoutputindex_to_txindex.boxed_clone(),
                |index, _| Some(index),
            ),
            opreturnindex_to_opreturnindex: LazyVecFrom1::init(
                "opreturnindex",
                version + Version::ZERO,
                indexer.vecs.opreturnindex_to_txindex.boxed_clone(),
                |index, _| Some(index),
            ),
            txindex_to_txindex: LazyVecFrom1::init(
                "txindex",
                version + Version::ZERO,
                indexer.vecs.txindex_to_txid.boxed_clone(),
                |index, _| Some(index),
            ),
            txindex_to_input_count: EagerVec::forced_import(
                &db,
                "input_count",
                version + Version::ZERO,
            )?,
            txindex_to_output_count: EagerVec::forced_import(
                &db,
                "output_count",
                version + Version::ZERO,
            )?,
            dateindex_to_date: EagerVec::forced_import(&db, "date", version + Version::ZERO)?,
            dateindex_to_dateindex: EagerVec::forced_import(
                &db,
                "dateindex",
                version + Version::ZERO,
            )?,
            dateindex_to_first_height: EagerVec::forced_import(
                &db,
                "first_height",
                version + Version::ZERO,
            )?,
            dateindex_to_monthindex: EagerVec::forced_import(
                &db,
                "monthindex",
                version + Version::ZERO,
            )?,
            dateindex_to_weekindex: EagerVec::forced_import(
                &db,
                "weekindex",
                version + Version::ZERO,
            )?,
            decadeindex_to_decadeindex: EagerVec::forced_import(
                &db,
                "decadeindex",
                version + Version::ZERO,
            )?,
            decadeindex_to_first_yearindex: EagerVec::forced_import(
                &db,
                "first_yearindex",
                version + Version::ZERO,
            )?,
            difficultyepoch_to_difficultyepoch: EagerVec::forced_import(
                &db,
                "difficultyepoch",
                version + Version::ZERO,
            )?,
            difficultyepoch_to_first_height: EagerVec::forced_import(
                &db,
                "first_height",
                version + Version::ZERO,
            )?,
            halvingepoch_to_first_height: EagerVec::forced_import(
                &db,
                "first_height",
                version + Version::ZERO,
            )?,
            halvingepoch_to_halvingepoch: EagerVec::forced_import(
                &db,
                "halvingepoch",
                version + Version::ZERO,
            )?,
            height_to_date: EagerVec::forced_import(&db, "date", version + Version::ZERO)?,
            height_to_difficultyepoch: EagerVec::forced_import(
                &db,
                "difficultyepoch",
                version + Version::ZERO,
            )?,
            height_to_halvingepoch: EagerVec::forced_import(
                &db,
                "halvingepoch",
                version + Version::ZERO,
            )?,
            height_to_height: EagerVec::forced_import(&db, "height", version + Version::ZERO)?,
            monthindex_to_first_dateindex: EagerVec::forced_import(
                &db,
                "first_dateindex",
                version + Version::ZERO,
            )?,
            monthindex_to_monthindex: EagerVec::forced_import(
                &db,
                "monthindex",
                version + Version::ZERO,
            )?,
            monthindex_to_quarterindex: EagerVec::forced_import(
                &db,
                "quarterindex",
                version + Version::ZERO,
            )?,
            monthindex_to_semesterindex: EagerVec::forced_import(
                &db,
                "semesterindex",
                version + Version::ZERO,
            )?,
            monthindex_to_yearindex: EagerVec::forced_import(
                &db,
                "yearindex",
                version + Version::ZERO,
            )?,
            quarterindex_to_first_monthindex: EagerVec::forced_import(
                &db,
                "first_monthindex",
                version + Version::ZERO,
            )?,
            semesterindex_to_first_monthindex: EagerVec::forced_import(
                &db,
                "first_monthindex",
                version + Version::ZERO,
            )?,
            weekindex_to_first_dateindex: EagerVec::forced_import(
                &db,
                "first_dateindex",
                version + Version::ZERO,
            )?,
            yearindex_to_first_monthindex: EagerVec::forced_import(
                &db,
                "first_monthindex",
                version + Version::ZERO,
            )?,
            quarterindex_to_quarterindex: EagerVec::forced_import(
                &db,
                "quarterindex",
                version + Version::ZERO,
            )?,
            semesterindex_to_semesterindex: EagerVec::forced_import(
                &db,
                "semesterindex",
                version + Version::ZERO,
            )?,
            weekindex_to_weekindex: EagerVec::forced_import(
                &db,
                "weekindex",
                version + Version::ZERO,
            )?,
            yearindex_to_decadeindex: EagerVec::forced_import(
                &db,
                "decadeindex",
                version + Version::ZERO,
            )?,
            yearindex_to_yearindex: EagerVec::forced_import(
                &db,
                "yearindex",
                version + Version::ZERO,
            )?,
            height_to_date_fixed: EagerVec::forced_import(
                &db,
                "date_fixed",
                version + Version::ZERO,
            )?,
            height_to_dateindex: EagerVec::forced_import(
                &db,
                "dateindex",
                version + Version::ZERO,
            )?,
            height_to_timestamp_fixed: EagerVec::forced_import(
                &db,
                "timestamp_fixed",
                version + Version::ZERO,
            )?,
            height_to_txindex_count: EagerVec::forced_import(
                &db,
                "txindex_count",
                version + Version::ZERO,
            )?,
            dateindex_to_height_count: EagerVec::forced_import(
                &db,
                "height_count",
                version + Version::ZERO,
            )?,
            weekindex_to_dateindex_count: EagerVec::forced_import(
                &db,
                "dateindex_count",
                version + Version::ZERO,
            )?,
            difficultyepoch_to_height_count: EagerVec::forced_import(
                &db,
                "height_count",
                version + Version::ZERO,
            )?,
            monthindex_to_dateindex_count: EagerVec::forced_import(
                &db,
                "dateindex_count",
                version + Version::ZERO,
            )?,
            quarterindex_to_monthindex_count: EagerVec::forced_import(
                &db,
                "monthindex_count",
                version + Version::ZERO,
            )?,
            semesterindex_to_monthindex_count: EagerVec::forced_import(
                &db,
                "monthindex_count",
                version + Version::ZERO,
            )?,
            yearindex_to_monthindex_count: EagerVec::forced_import(
                &db,
                "monthindex_count",
                version + Version::ZERO,
            )?,
            decadeindex_to_yearindex_count: EagerVec::forced_import(
                &db,
                "yearindex_count",
                version + Version::ZERO,
            )?,
            db,
        };

        this.db.retain_regions(
            this.iter_any_exportable()
                .flat_map(|v| v.region_names())
                .collect(),
        )?;

        this.db.compact()?;

        Ok(this)
    }

    pub fn compute(
        &mut self,
        indexer: &Indexer,
        starting_indexes: brk_indexer::Indexes,
        exit: &Exit,
    ) -> Result<Indexes> {
        let indexes = self.compute_(indexer, starting_indexes, exit)?;
        self.db.compact()?;
        Ok(indexes)
    }

    fn compute_(
        &mut self,
        indexer: &Indexer,
        starting_indexes: brk_indexer::Indexes,
        exit: &Exit,
    ) -> Result<Indexes> {
        // ---
        // TxInIndex
        // ---

        let txindex_to_first_txoutindex = &indexer.vecs.txindex_to_first_txoutindex;
        let txindex_to_first_txoutindex_reader = txindex_to_first_txoutindex.create_reader();
        self.txinindex_to_txoutindex.compute_transform(
            starting_indexes.txinindex,
            &indexer.vecs.txinindex_to_outpoint,
            |(txinindex, outpoint, ..)| {
                if unlikely(outpoint.is_coinbase()) {
                    return (txinindex, TxOutIndex::COINBASE);
                }
                let txoutindex = txindex_to_first_txoutindex
                    .read_unwrap(outpoint.txindex(), &txindex_to_first_txoutindex_reader)
                    + outpoint.vout();
                (txinindex, txoutindex)
            },
            exit,
        )?;

        // ---
        // TxIndex
        // ---

        self.txindex_to_input_count.compute_count_from_indexes(
            starting_indexes.txindex,
            &indexer.vecs.txindex_to_first_txinindex,
            &self.txinindex_to_txoutindex,
            exit,
        )?;

        self.txindex_to_output_count.compute_count_from_indexes(
            starting_indexes.txindex,
            &indexer.vecs.txindex_to_first_txoutindex,
            &indexer.vecs.txoutindex_to_value,
            exit,
        )?;

        self.height_to_txindex_count.compute_count_from_indexes(
            starting_indexes.height,
            &indexer.vecs.height_to_first_txindex,
            &indexer.vecs.txindex_to_txid,
            exit,
        )?;

        // ---
        // Height
        // ---

        self.height_to_height.compute_from_index(
            starting_indexes.height,
            &indexer.vecs.height_to_weight,
            exit,
        )?;

        self.height_to_date.compute_transform(
            starting_indexes.height,
            &indexer.vecs.height_to_timestamp,
            |(h, t, ..)| (h, Date::from(t)),
            exit,
        )?;

        let mut prev_timestamp_fixed = None;
        self.height_to_timestamp_fixed.compute_transform(
            starting_indexes.height,
            &indexer.vecs.height_to_timestamp,
            |(h, timestamp, height_to_timestamp_fixed_iter)| {
                if prev_timestamp_fixed.is_none()
                    && let Some(prev_h) = h.decremented()
                {
                    prev_timestamp_fixed.replace(
                        height_to_timestamp_fixed_iter
                            .into_iter()
                            .get_unwrap(prev_h),
                    );
                }
                let timestamp_fixed =
                    prev_timestamp_fixed.map_or(timestamp, |prev_d| prev_d.max(timestamp));
                prev_timestamp_fixed.replace(timestamp_fixed);
                (h, timestamp_fixed)
            },
            exit,
        )?;

        self.height_to_date_fixed.compute_transform(
            starting_indexes.height,
            &self.height_to_timestamp_fixed,
            |(h, t, ..)| (h, Date::from(t)),
            exit,
        )?;

        let decremented_starting_height = starting_indexes.height.decremented().unwrap_or_default();

        // ---
        // DateIndex
        // ---

        let starting_dateindex = self
            .height_to_dateindex
            .into_iter()
            .get(decremented_starting_height)
            .unwrap_or_default();

        self.height_to_dateindex.compute_transform(
            starting_indexes.height,
            &self.height_to_date_fixed,
            |(h, d, ..)| (h, DateIndex::try_from(d).unwrap()),
            exit,
        )?;

        let starting_dateindex = if let Some(dateindex) = self
            .height_to_dateindex
            .into_iter()
            .get(decremented_starting_height)
        {
            starting_dateindex.min(dateindex)
        } else {
            starting_dateindex
        };

        self.dateindex_to_first_height.compute_coarser(
            starting_indexes.height,
            &self.height_to_dateindex,
            exit,
        )?;

        self.dateindex_to_dateindex.compute_from_index(
            starting_dateindex,
            &self.dateindex_to_first_height,
            exit,
        )?;

        self.dateindex_to_date.compute_from_index(
            starting_dateindex,
            &self.dateindex_to_first_height,
            exit,
        )?;

        self.dateindex_to_height_count.compute_count_from_indexes(
            starting_dateindex,
            &self.dateindex_to_first_height,
            &indexer.vecs.height_to_weight,
            exit,
        )?;

        // ---
        // WeekIndex
        // ---

        let starting_weekindex = self
            .dateindex_to_weekindex
            .into_iter()
            .get(starting_dateindex)
            .unwrap_or_default();

        self.dateindex_to_weekindex.compute_range(
            starting_dateindex,
            &self.dateindex_to_dateindex,
            |i| (i, WeekIndex::from(i)),
            exit,
        )?;

        self.weekindex_to_first_dateindex.compute_coarser(
            starting_dateindex,
            &self.dateindex_to_weekindex,
            exit,
        )?;

        self.weekindex_to_weekindex.compute_from_index(
            starting_weekindex,
            &self.weekindex_to_first_dateindex,
            exit,
        )?;

        self.weekindex_to_dateindex_count
            .compute_count_from_indexes(
                starting_weekindex,
                &self.weekindex_to_first_dateindex,
                &self.dateindex_to_date,
                exit,
            )?;

        // ---
        // DifficultyEpoch
        // ---

        let starting_difficultyepoch = self
            .height_to_difficultyepoch
            .into_iter()
            .get(decremented_starting_height)
            .unwrap_or_default();

        self.height_to_difficultyepoch.compute_from_index(
            starting_indexes.height,
            &indexer.vecs.height_to_weight,
            exit,
        )?;

        self.difficultyepoch_to_first_height.compute_coarser(
            starting_indexes.height,
            &self.height_to_difficultyepoch,
            exit,
        )?;

        self.difficultyepoch_to_difficultyepoch.compute_from_index(
            starting_difficultyepoch,
            &self.difficultyepoch_to_first_height,
            exit,
        )?;

        self.difficultyepoch_to_height_count
            .compute_count_from_indexes(
                starting_difficultyepoch,
                &self.difficultyepoch_to_first_height,
                &self.height_to_date,
                exit,
            )?;

        // ---
        // MonthIndex
        // ---

        let starting_monthindex = self
            .dateindex_to_monthindex
            .into_iter()
            .get(starting_dateindex)
            .unwrap_or_default();

        self.dateindex_to_monthindex.compute_range(
            starting_dateindex,
            &self.dateindex_to_dateindex,
            |i| (i, MonthIndex::from(i)),
            exit,
        )?;

        self.monthindex_to_first_dateindex.compute_coarser(
            starting_dateindex,
            &self.dateindex_to_monthindex,
            exit,
        )?;

        self.monthindex_to_monthindex.compute_from_index(
            starting_monthindex,
            &self.monthindex_to_first_dateindex,
            exit,
        )?;

        self.monthindex_to_dateindex_count
            .compute_count_from_indexes(
                starting_monthindex,
                &self.monthindex_to_first_dateindex,
                &self.dateindex_to_date,
                exit,
            )?;

        // ---
        // QuarterIndex
        // ---

        let starting_quarterindex = self
            .monthindex_to_quarterindex
            .into_iter()
            .get(starting_monthindex)
            .unwrap_or_default();

        self.monthindex_to_quarterindex.compute_from_index(
            starting_monthindex,
            &self.monthindex_to_first_dateindex,
            exit,
        )?;

        self.quarterindex_to_first_monthindex.compute_coarser(
            starting_monthindex,
            &self.monthindex_to_quarterindex,
            exit,
        )?;

        // let quarter_count = self.quarterindex_to_first_monthindex.len();

        self.quarterindex_to_quarterindex.compute_from_index(
            starting_quarterindex,
            &self.quarterindex_to_first_monthindex,
            exit,
        )?;

        self.quarterindex_to_monthindex_count
            .compute_count_from_indexes(
                starting_quarterindex,
                &self.quarterindex_to_first_monthindex,
                &self.monthindex_to_monthindex,
                exit,
            )?;

        // ---
        // SemesterIndex
        // ---

        let starting_semesterindex = self
            .monthindex_to_semesterindex
            .into_iter()
            .get(starting_monthindex)
            .unwrap_or_default();

        self.monthindex_to_semesterindex.compute_from_index(
            starting_monthindex,
            &self.monthindex_to_first_dateindex,
            exit,
        )?;

        self.semesterindex_to_first_monthindex.compute_coarser(
            starting_monthindex,
            &self.monthindex_to_semesterindex,
            exit,
        )?;

        // let semester_count = self.semesterindex_to_first_monthindex.len();

        self.semesterindex_to_semesterindex.compute_from_index(
            starting_semesterindex,
            &self.semesterindex_to_first_monthindex,
            exit,
        )?;

        self.semesterindex_to_monthindex_count
            .compute_count_from_indexes(
                starting_semesterindex,
                &self.semesterindex_to_first_monthindex,
                &self.monthindex_to_monthindex,
                exit,
            )?;

        // ---
        // YearIndex
        // ---

        let starting_yearindex = self
            .monthindex_to_yearindex
            .into_iter()
            .get(starting_monthindex)
            .unwrap_or_default();

        self.monthindex_to_yearindex.compute_from_index(
            starting_monthindex,
            &self.monthindex_to_first_dateindex,
            exit,
        )?;

        self.yearindex_to_first_monthindex.compute_coarser(
            starting_monthindex,
            &self.monthindex_to_yearindex,
            exit,
        )?;

        self.yearindex_to_yearindex.compute_from_index(
            starting_yearindex,
            &self.yearindex_to_first_monthindex,
            exit,
        )?;

        self.yearindex_to_monthindex_count
            .compute_count_from_indexes(
                starting_yearindex,
                &self.yearindex_to_first_monthindex,
                &self.monthindex_to_monthindex,
                exit,
            )?;
        // ---
        // HalvingEpoch
        // ---

        let starting_halvingepoch = self
            .height_to_halvingepoch
            .into_iter()
            .get(decremented_starting_height)
            .unwrap_or_default();

        self.height_to_halvingepoch.compute_from_index(
            starting_indexes.height,
            &indexer.vecs.height_to_weight,
            exit,
        )?;

        self.halvingepoch_to_first_height.compute_coarser(
            starting_indexes.height,
            &self.height_to_halvingepoch,
            exit,
        )?;

        self.halvingepoch_to_halvingepoch.compute_from_index(
            starting_halvingepoch,
            &self.halvingepoch_to_first_height,
            exit,
        )?;

        // ---
        // DecadeIndex
        // ---

        let starting_decadeindex = self
            .yearindex_to_decadeindex
            .into_iter()
            .get(starting_yearindex)
            .unwrap_or_default();

        self.yearindex_to_decadeindex.compute_from_index(
            starting_yearindex,
            &self.yearindex_to_first_monthindex,
            exit,
        )?;

        self.decadeindex_to_first_yearindex.compute_coarser(
            starting_yearindex,
            &self.yearindex_to_decadeindex,
            exit,
        )?;

        self.decadeindex_to_decadeindex.compute_from_index(
            starting_decadeindex,
            &self.decadeindex_to_first_yearindex,
            exit,
        )?;

        self.decadeindex_to_yearindex_count
            .compute_count_from_indexes(
                starting_decadeindex,
                &self.decadeindex_to_first_yearindex,
                &self.yearindex_to_yearindex,
                exit,
            )?;

        Ok(Indexes {
            indexes: starting_indexes,
            dateindex: starting_dateindex,
            weekindex: starting_weekindex,
            monthindex: starting_monthindex,
            quarterindex: starting_quarterindex,
            semesterindex: starting_semesterindex,
            yearindex: starting_yearindex,
            decadeindex: starting_decadeindex,
            difficultyepoch: starting_difficultyepoch,
            halvingepoch: starting_halvingepoch,
        })
    }
}

#[derive(Debug)]
pub struct Indexes {
    indexes: brk_indexer::Indexes,
    pub dateindex: DateIndex,
    pub weekindex: WeekIndex,
    pub monthindex: MonthIndex,
    pub quarterindex: QuarterIndex,
    pub semesterindex: SemesterIndex,
    pub yearindex: YearIndex,
    pub decadeindex: DecadeIndex,
    pub difficultyepoch: DifficultyEpoch,
    pub halvingepoch: HalvingEpoch,
}

impl Indexes {
    pub fn update_from_height(&mut self, height: Height, indexes: &Vecs) {
        self.indexes.height = height;
        self.dateindex =
            DateIndex::try_from(indexes.height_to_date_fixed.into_iter().get_unwrap(height))
                .unwrap();
        self.weekindex = WeekIndex::from(self.dateindex);
        self.monthindex = MonthIndex::from(self.dateindex);
        self.quarterindex = QuarterIndex::from(self.monthindex);
        self.semesterindex = SemesterIndex::from(self.monthindex);
        self.yearindex = YearIndex::from(self.monthindex);
        self.decadeindex = DecadeIndex::from(self.dateindex);
        self.difficultyepoch = DifficultyEpoch::from(self.height);
        self.halvingepoch = HalvingEpoch::from(self.height);
    }
}

impl Deref for Indexes {
    type Target = brk_indexer::Indexes;
    fn deref(&self) -> &Self::Target {
        &self.indexes
    }
}
