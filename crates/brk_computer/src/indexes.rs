use std::{ops::Deref, path::Path};

use brk_error::Result;
use brk_indexer::Indexer;
use brk_structs::{
    Date, DateIndex, DecadeIndex, DifficultyEpoch, EmptyOutputIndex, HalvingEpoch, Height,
    InputIndex, MonthIndex, OpReturnIndex, OutputIndex, P2AAddressIndex, P2ABytes, P2MSOutputIndex,
    P2PK33AddressIndex, P2PK33Bytes, P2PK65AddressIndex, P2PK65Bytes, P2PKHAddressIndex,
    P2PKHBytes, P2SHAddressIndex, P2SHBytes, P2TRAddressIndex, P2TRBytes, P2WPKHAddressIndex,
    P2WPKHBytes, P2WSHAddressIndex, P2WSHBytes, QuarterIndex, Sats, SemesterIndex, StoredU64,
    Timestamp, TxIndex, Txid, UnknownOutputIndex, Version, WeekIndex, YearIndex,
};
use vecdb::{
    AnyCloneableIterableVec, AnyCollectableVec, Database, EagerVec, Exit, LazyVecFrom1,
    LazyVecFrom2, PAGE_SIZE, StoredIndex, VecIterator,
};

const VERSION: Version = Version::ZERO;

#[derive(Clone)]
pub struct Vecs {
    db: Database,

    pub dateindex_to_date: EagerVec<DateIndex, Date>,
    pub dateindex_to_dateindex: EagerVec<DateIndex, DateIndex>,
    pub dateindex_to_first_height: EagerVec<DateIndex, Height>,
    pub dateindex_to_height_count: EagerVec<DateIndex, StoredU64>,
    pub dateindex_to_monthindex: EagerVec<DateIndex, MonthIndex>,
    pub dateindex_to_weekindex: EagerVec<DateIndex, WeekIndex>,
    pub decadeindex_to_decadeindex: EagerVec<DecadeIndex, DecadeIndex>,
    pub decadeindex_to_first_yearindex: EagerVec<DecadeIndex, YearIndex>,
    pub decadeindex_to_yearindex_count: EagerVec<DecadeIndex, StoredU64>,
    pub difficultyepoch_to_difficultyepoch: EagerVec<DifficultyEpoch, DifficultyEpoch>,
    pub difficultyepoch_to_first_height: EagerVec<DifficultyEpoch, Height>,
    pub difficultyepoch_to_height_count: EagerVec<DifficultyEpoch, StoredU64>,
    pub emptyoutputindex_to_emptyoutputindex:
        LazyVecFrom1<EmptyOutputIndex, EmptyOutputIndex, EmptyOutputIndex, TxIndex>,
    pub halvingepoch_to_first_height: EagerVec<HalvingEpoch, Height>,
    pub halvingepoch_to_halvingepoch: EagerVec<HalvingEpoch, HalvingEpoch>,
    pub height_to_date: EagerVec<Height, Date>,
    pub height_to_date_fixed: EagerVec<Height, Date>,
    pub height_to_dateindex: EagerVec<Height, DateIndex>,
    pub height_to_difficultyepoch: EagerVec<Height, DifficultyEpoch>,
    pub height_to_halvingepoch: EagerVec<Height, HalvingEpoch>,
    pub height_to_height: EagerVec<Height, Height>,
    pub height_to_timestamp_fixed: EagerVec<Height, Timestamp>,
    pub height_to_txindex_count: EagerVec<Height, StoredU64>,
    pub inputindex_to_inputindex: LazyVecFrom1<InputIndex, InputIndex, InputIndex, OutputIndex>,
    pub monthindex_to_dateindex_count: EagerVec<MonthIndex, StoredU64>,
    pub monthindex_to_first_dateindex: EagerVec<MonthIndex, DateIndex>,
    pub monthindex_to_monthindex: EagerVec<MonthIndex, MonthIndex>,
    pub monthindex_to_quarterindex: EagerVec<MonthIndex, QuarterIndex>,
    pub monthindex_to_semesterindex: EagerVec<MonthIndex, SemesterIndex>,
    pub monthindex_to_yearindex: EagerVec<MonthIndex, YearIndex>,
    pub opreturnindex_to_opreturnindex:
        LazyVecFrom1<OpReturnIndex, OpReturnIndex, OpReturnIndex, TxIndex>,
    pub outputindex_to_outputindex: LazyVecFrom1<OutputIndex, OutputIndex, OutputIndex, Sats>,
    pub outputindex_to_txindex: EagerVec<OutputIndex, TxIndex>,
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
    pub quarterindex_to_first_monthindex: EagerVec<QuarterIndex, MonthIndex>,
    pub quarterindex_to_monthindex_count: EagerVec<QuarterIndex, StoredU64>,
    pub quarterindex_to_quarterindex: EagerVec<QuarterIndex, QuarterIndex>,
    pub semesterindex_to_first_monthindex: EagerVec<SemesterIndex, MonthIndex>,
    pub semesterindex_to_monthindex_count: EagerVec<SemesterIndex, StoredU64>,
    pub semesterindex_to_semesterindex: EagerVec<SemesterIndex, SemesterIndex>,
    pub txindex_to_height: EagerVec<TxIndex, Height>,
    pub txindex_to_input_count:
        LazyVecFrom2<TxIndex, StoredU64, TxIndex, InputIndex, InputIndex, OutputIndex>,
    pub txindex_to_output_count:
        LazyVecFrom2<TxIndex, StoredU64, TxIndex, OutputIndex, OutputIndex, Sats>,
    pub txindex_to_txindex: LazyVecFrom1<TxIndex, TxIndex, TxIndex, Txid>,
    pub unknownoutputindex_to_unknownoutputindex:
        LazyVecFrom1<UnknownOutputIndex, UnknownOutputIndex, UnknownOutputIndex, TxIndex>,
    pub weekindex_to_dateindex_count: EagerVec<WeekIndex, StoredU64>,
    pub weekindex_to_first_dateindex: EagerVec<WeekIndex, DateIndex>,
    pub weekindex_to_weekindex: EagerVec<WeekIndex, WeekIndex>,
    pub yearindex_to_decadeindex: EagerVec<YearIndex, DecadeIndex>,
    pub yearindex_to_first_monthindex: EagerVec<YearIndex, MonthIndex>,
    pub yearindex_to_monthindex_count: EagerVec<YearIndex, StoredU64>,
    pub yearindex_to_yearindex: EagerVec<YearIndex, YearIndex>,
}

impl Vecs {
    pub fn forced_import(parent: &Path, version: Version, indexer: &Indexer) -> Result<Self> {
        let db = Database::open(&parent.join("indexes"))?;
        db.set_min_len(PAGE_SIZE * 10_000_000)?;

        let outputindex_to_outputindex = LazyVecFrom1::init(
            "outputindex",
            version + VERSION + Version::ZERO,
            indexer.vecs.outputindex_to_value.boxed_clone(),
            |index, _| Some(index),
        );

        let inputindex_to_inputindex = LazyVecFrom1::init(
            "inputindex",
            version + VERSION + Version::ZERO,
            indexer.vecs.inputindex_to_outputindex.boxed_clone(),
            |index, _| Some(index),
        );

        let txindex_to_txindex = LazyVecFrom1::init(
            "txindex",
            version + VERSION + Version::ZERO,
            indexer.vecs.txindex_to_txid.boxed_clone(),
            |index, _| Some(index),
        );

        let txindex_to_input_count = LazyVecFrom2::init(
            "input_count",
            version + VERSION + Version::ZERO,
            indexer.vecs.txindex_to_first_inputindex.boxed_clone(),
            indexer.vecs.inputindex_to_outputindex.boxed_clone(),
            |index: TxIndex, txindex_to_first_inputindex_iter, inputindex_to_outputindex_iter| {
                let txindex = index.unwrap_to_usize();
                txindex_to_first_inputindex_iter
                    .next_at(txindex)
                    .map(|(_, start)| {
                        let start = usize::from(start.into_owned());
                        let end = txindex_to_first_inputindex_iter
                            .next_at(txindex + 1)
                            .map(|(_, v)| usize::from(v.into_owned()))
                            .unwrap_or_else(|| inputindex_to_outputindex_iter.len());
                        StoredU64::from((start..end).count())
                    })
            },
        );

        let txindex_to_output_count = LazyVecFrom2::init(
            "output_count",
            version + VERSION + Version::ZERO,
            indexer.vecs.txindex_to_first_outputindex.boxed_clone(),
            indexer.vecs.outputindex_to_value.boxed_clone(),
            |index: TxIndex, txindex_to_first_outputindex_iter, outputindex_to_value_iter| {
                let txindex = index.unwrap_to_usize();
                txindex_to_first_outputindex_iter
                    .next_at(txindex)
                    .map(|(_, start)| {
                        let start = usize::from(start.into_owned());
                        let end = txindex_to_first_outputindex_iter
                            .next_at(txindex + 1)
                            .map(|(_, v)| usize::from(v.into_owned()))
                            .unwrap_or_else(|| outputindex_to_value_iter.len());
                        StoredU64::from((start..end).count())
                    })
            },
        );

        let p2pk33addressindex_to_p2pk33addressindex = LazyVecFrom1::init(
            "p2pk33addressindex",
            version + VERSION + Version::ZERO,
            indexer.vecs.p2pk33addressindex_to_p2pk33bytes.boxed_clone(),
            |index, _| Some(index),
        );
        let p2pk65addressindex_to_p2pk65addressindex = LazyVecFrom1::init(
            "p2pk65addressindex",
            version + VERSION + Version::ZERO,
            indexer.vecs.p2pk65addressindex_to_p2pk65bytes.boxed_clone(),
            |index, _| Some(index),
        );
        let p2pkhaddressindex_to_p2pkhaddressindex = LazyVecFrom1::init(
            "p2pkhaddressindex",
            version + VERSION + Version::ZERO,
            indexer.vecs.p2pkhaddressindex_to_p2pkhbytes.boxed_clone(),
            |index, _| Some(index),
        );
        let p2shaddressindex_to_p2shaddressindex = LazyVecFrom1::init(
            "p2shaddressindex",
            version + VERSION + Version::ZERO,
            indexer.vecs.p2shaddressindex_to_p2shbytes.boxed_clone(),
            |index, _| Some(index),
        );
        let p2traddressindex_to_p2traddressindex = LazyVecFrom1::init(
            "p2traddressindex",
            version + VERSION + Version::ZERO,
            indexer.vecs.p2traddressindex_to_p2trbytes.boxed_clone(),
            |index, _| Some(index),
        );
        let p2wpkhaddressindex_to_p2wpkhaddressindex = LazyVecFrom1::init(
            "p2wpkhaddressindex",
            version + VERSION + Version::ZERO,
            indexer.vecs.p2wpkhaddressindex_to_p2wpkhbytes.boxed_clone(),
            |index, _| Some(index),
        );
        let p2wshaddressindex_to_p2wshaddressindex = LazyVecFrom1::init(
            "p2wshaddressindex",
            version + VERSION + Version::ZERO,
            indexer.vecs.p2wshaddressindex_to_p2wshbytes.boxed_clone(),
            |index, _| Some(index),
        );
        let p2aaddressindex_to_p2aaddressindex = LazyVecFrom1::init(
            "p2aaddressindex",
            version + VERSION + Version::ZERO,
            indexer.vecs.p2aaddressindex_to_p2abytes.boxed_clone(),
            |index, _| Some(index),
        );
        let p2msoutputindex_to_p2msoutputindex = LazyVecFrom1::init(
            "p2msoutputindex",
            version + VERSION + Version::ZERO,
            indexer.vecs.p2msoutputindex_to_txindex.boxed_clone(),
            |index, _| Some(index),
        );
        let emptyoutputindex_to_emptyoutputindex = LazyVecFrom1::init(
            "emptyoutputindex",
            version + VERSION + Version::ZERO,
            indexer.vecs.emptyoutputindex_to_txindex.boxed_clone(),
            |index, _| Some(index),
        );
        let unknownoutputindex_to_unknownoutputindex = LazyVecFrom1::init(
            "unknownoutputindex",
            version + VERSION + Version::ZERO,
            indexer.vecs.unknownoutputindex_to_txindex.boxed_clone(),
            |index, _| Some(index),
        );
        let opreturnindex_to_opreturnindex = LazyVecFrom1::init(
            "opreturnindex",
            version + VERSION + Version::ZERO,
            indexer.vecs.opreturnindex_to_txindex.boxed_clone(),
            |index, _| Some(index),
        );

        let this = Self {
            emptyoutputindex_to_emptyoutputindex,
            inputindex_to_inputindex,
            opreturnindex_to_opreturnindex,
            outputindex_to_outputindex,
            p2aaddressindex_to_p2aaddressindex,
            p2msoutputindex_to_p2msoutputindex,
            p2pk33addressindex_to_p2pk33addressindex,
            p2pk65addressindex_to_p2pk65addressindex,
            p2pkhaddressindex_to_p2pkhaddressindex,
            p2shaddressindex_to_p2shaddressindex,
            p2traddressindex_to_p2traddressindex,
            p2wpkhaddressindex_to_p2wpkhaddressindex,
            p2wshaddressindex_to_p2wshaddressindex,
            txindex_to_input_count,
            txindex_to_output_count,
            txindex_to_txindex,
            unknownoutputindex_to_unknownoutputindex,

            dateindex_to_date: EagerVec::forced_import_compressed(
                &db,
                "date",
                version + VERSION + Version::ZERO,
            )?,
            dateindex_to_dateindex: EagerVec::forced_import_compressed(
                &db,
                "dateindex",
                version + VERSION + Version::ZERO,
            )?,
            dateindex_to_first_height: EagerVec::forced_import_compressed(
                &db,
                "first_height",
                version + VERSION + Version::ZERO,
            )?,
            dateindex_to_monthindex: EagerVec::forced_import_compressed(
                &db,
                "monthindex",
                version + VERSION + Version::ZERO,
            )?,
            dateindex_to_weekindex: EagerVec::forced_import_compressed(
                &db,
                "weekindex",
                version + VERSION + Version::ZERO,
            )?,
            decadeindex_to_decadeindex: EagerVec::forced_import_compressed(
                &db,
                "decadeindex",
                version + VERSION + Version::ZERO,
            )?,
            decadeindex_to_first_yearindex: EagerVec::forced_import_compressed(
                &db,
                "first_yearindex",
                version + VERSION + Version::ZERO,
            )?,
            difficultyepoch_to_difficultyepoch: EagerVec::forced_import_compressed(
                &db,
                "difficultyepoch",
                version + VERSION + Version::ZERO,
            )?,
            difficultyepoch_to_first_height: EagerVec::forced_import_compressed(
                &db,
                "first_height",
                version + VERSION + Version::ZERO,
            )?,
            halvingepoch_to_first_height: EagerVec::forced_import_compressed(
                &db,
                "first_height",
                version + VERSION + Version::ZERO,
            )?,
            halvingepoch_to_halvingepoch: EagerVec::forced_import_compressed(
                &db,
                "halvingepoch",
                version + VERSION + Version::ZERO,
            )?,
            height_to_date: EagerVec::forced_import_compressed(
                &db,
                "date",
                version + VERSION + Version::ZERO,
            )?,
            height_to_difficultyepoch: EagerVec::forced_import_compressed(
                &db,
                "difficultyepoch",
                version + VERSION + Version::ZERO,
            )?,
            height_to_halvingepoch: EagerVec::forced_import_compressed(
                &db,
                "halvingepoch",
                version + VERSION + Version::ZERO,
            )?,
            height_to_height: EagerVec::forced_import_compressed(
                &db,
                "height",
                version + VERSION + Version::ZERO,
            )?,
            monthindex_to_first_dateindex: EagerVec::forced_import_compressed(
                &db,
                "first_dateindex",
                version + VERSION + Version::ZERO,
            )?,
            monthindex_to_monthindex: EagerVec::forced_import_compressed(
                &db,
                "monthindex",
                version + VERSION + Version::ZERO,
            )?,
            monthindex_to_quarterindex: EagerVec::forced_import_compressed(
                &db,
                "quarterindex",
                version + VERSION + Version::ZERO,
            )?,
            monthindex_to_semesterindex: EagerVec::forced_import_compressed(
                &db,
                "semesterindex",
                version + VERSION + Version::ZERO,
            )?,
            monthindex_to_yearindex: EagerVec::forced_import_compressed(
                &db,
                "yearindex",
                version + VERSION + Version::ZERO,
            )?,
            quarterindex_to_first_monthindex: EagerVec::forced_import_compressed(
                &db,
                "first_monthindex",
                version + VERSION + Version::ZERO,
            )?,
            semesterindex_to_first_monthindex: EagerVec::forced_import_compressed(
                &db,
                "first_monthindex",
                version + VERSION + Version::ZERO,
            )?,
            weekindex_to_first_dateindex: EagerVec::forced_import_compressed(
                &db,
                "first_dateindex",
                version + VERSION + Version::ZERO,
            )?,
            yearindex_to_first_monthindex: EagerVec::forced_import_compressed(
                &db,
                "first_monthindex",
                version + VERSION + Version::ZERO,
            )?,
            quarterindex_to_quarterindex: EagerVec::forced_import_compressed(
                &db,
                "quarterindex",
                version + VERSION + Version::ZERO,
            )?,
            semesterindex_to_semesterindex: EagerVec::forced_import_compressed(
                &db,
                "semesterindex",
                version + VERSION + Version::ZERO,
            )?,
            weekindex_to_weekindex: EagerVec::forced_import_compressed(
                &db,
                "weekindex",
                version + VERSION + Version::ZERO,
            )?,
            yearindex_to_decadeindex: EagerVec::forced_import_compressed(
                &db,
                "decadeindex",
                version + VERSION + Version::ZERO,
            )?,
            yearindex_to_yearindex: EagerVec::forced_import_compressed(
                &db,
                "yearindex",
                version + VERSION + Version::ZERO,
            )?,
            height_to_date_fixed: EagerVec::forced_import_compressed(
                &db,
                "date_fixed",
                version + VERSION + Version::ZERO,
            )?,
            height_to_dateindex: EagerVec::forced_import_compressed(
                &db,
                "dateindex",
                version + VERSION + Version::ZERO,
            )?,
            txindex_to_height: EagerVec::forced_import_compressed(
                &db,
                "height",
                version + VERSION + Version::ZERO,
            )?,
            height_to_timestamp_fixed: EagerVec::forced_import_compressed(
                &db,
                "timestamp_fixed",
                version + VERSION + Version::ZERO,
            )?,
            height_to_txindex_count: EagerVec::forced_import_compressed(
                &db,
                "txindex_count",
                version + VERSION + Version::ZERO,
            )?,
            dateindex_to_height_count: EagerVec::forced_import_compressed(
                &db,
                "height_count",
                version + VERSION + Version::ZERO,
            )?,
            weekindex_to_dateindex_count: EagerVec::forced_import_compressed(
                &db,
                "dateindex_count",
                version + VERSION + Version::ZERO,
            )?,
            difficultyepoch_to_height_count: EagerVec::forced_import_compressed(
                &db,
                "height_count",
                version + VERSION + Version::ZERO,
            )?,
            monthindex_to_dateindex_count: EagerVec::forced_import_compressed(
                &db,
                "dateindex_count",
                version + VERSION + Version::ZERO,
            )?,
            quarterindex_to_monthindex_count: EagerVec::forced_import_compressed(
                &db,
                "monthindex_count",
                version + VERSION + Version::ZERO,
            )?,
            semesterindex_to_monthindex_count: EagerVec::forced_import_compressed(
                &db,
                "monthindex_count",
                version + VERSION + Version::ZERO,
            )?,
            yearindex_to_monthindex_count: EagerVec::forced_import_compressed(
                &db,
                "monthindex_count",
                version + VERSION + Version::ZERO,
            )?,
            decadeindex_to_yearindex_count: EagerVec::forced_import_compressed(
                &db,
                "yearindex_count",
                version + VERSION + Version::ZERO,
            )?,
            outputindex_to_txindex: EagerVec::forced_import_compressed(
                &db,
                "txindex",
                version + VERSION + Version::ZERO,
            )?,

            db,
        };

        this.db.retain_regions(
            this.iter_any_collectable()
                .flat_map(|v| v.region_names())
                .collect(),
        )?;

        Ok(this)
    }

    pub fn compute(
        &mut self,
        indexer: &Indexer,
        starting_indexes: brk_indexer::Indexes,
        exit: &Exit,
    ) -> Result<Indexes> {
        let idxs = self.compute_(indexer, starting_indexes, exit)?;
        self.db.flush_then_punch()?;
        Ok(idxs)
    }

    fn compute_(
        &mut self,
        indexer: &Indexer,
        starting_indexes: brk_indexer::Indexes,
        exit: &Exit,
    ) -> Result<Indexes> {
        // ---
        // OutputIndex
        // ---

        self.outputindex_to_txindex.compute_inverse_less_to_more(
            starting_indexes.txindex,
            &indexer.vecs.txindex_to_first_outputindex,
            &self.txindex_to_output_count,
            exit,
        )?;

        // ---
        // TxIndex
        // ---

        self.height_to_txindex_count.compute_count_from_indexes(
            starting_indexes.height,
            &indexer.vecs.height_to_first_txindex,
            &indexer.vecs.txindex_to_txid,
            exit,
        )?;

        self.txindex_to_height.compute_inverse_less_to_more(
            starting_indexes.height,
            &indexer.vecs.height_to_first_txindex,
            &self.height_to_txindex_count,
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
                            .unwrap_get_inner(prev_h),
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
            .get_inner(decremented_starting_height)
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
            .get_inner(decremented_starting_height)
        {
            starting_dateindex.min(dateindex)
        } else {
            starting_dateindex
        };

        self.dateindex_to_first_height
            .compute_inverse_more_to_less(
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
            .get_inner(starting_dateindex)
            .unwrap_or_default();

        self.dateindex_to_weekindex.compute_range(
            starting_dateindex,
            &self.dateindex_to_dateindex,
            |i| (i, WeekIndex::from(i)),
            exit,
        )?;

        self.weekindex_to_first_dateindex
            .compute_inverse_more_to_less(starting_dateindex, &self.dateindex_to_weekindex, exit)?;

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
            .get_inner(decremented_starting_height)
            .unwrap_or_default();

        self.height_to_difficultyepoch.compute_from_index(
            starting_indexes.height,
            &indexer.vecs.height_to_weight,
            exit,
        )?;

        self.difficultyepoch_to_first_height
            .compute_inverse_more_to_less(
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
            .get_inner(starting_dateindex)
            .unwrap_or_default();

        self.dateindex_to_monthindex.compute_range(
            starting_dateindex,
            &self.dateindex_to_dateindex,
            |i| (i, MonthIndex::from(i)),
            exit,
        )?;

        self.monthindex_to_first_dateindex
            .compute_inverse_more_to_less(
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
            .get_inner(starting_monthindex)
            .unwrap_or_default();

        self.monthindex_to_quarterindex.compute_from_index(
            starting_monthindex,
            &self.monthindex_to_first_dateindex,
            exit,
        )?;

        self.quarterindex_to_first_monthindex
            .compute_inverse_more_to_less(
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
            .get_inner(starting_monthindex)
            .unwrap_or_default();

        self.monthindex_to_semesterindex.compute_from_index(
            starting_monthindex,
            &self.monthindex_to_first_dateindex,
            exit,
        )?;

        self.semesterindex_to_first_monthindex
            .compute_inverse_more_to_less(
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
            .get_inner(starting_monthindex)
            .unwrap_or_default();

        self.monthindex_to_yearindex.compute_from_index(
            starting_monthindex,
            &self.monthindex_to_first_dateindex,
            exit,
        )?;

        self.yearindex_to_first_monthindex
            .compute_inverse_more_to_less(
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
            .get_inner(decremented_starting_height)
            .unwrap_or_default();

        self.height_to_halvingepoch.compute_from_index(
            starting_indexes.height,
            &indexer.vecs.height_to_weight,
            exit,
        )?;

        self.halvingepoch_to_first_height
            .compute_inverse_more_to_less(
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
            .get_inner(starting_yearindex)
            .unwrap_or_default();

        self.yearindex_to_decadeindex.compute_from_index(
            starting_yearindex,
            &self.yearindex_to_first_monthindex,
            exit,
        )?;

        self.decadeindex_to_first_yearindex
            .compute_inverse_more_to_less(
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

    pub fn iter_any_collectable(&self) -> impl Iterator<Item = &dyn AnyCollectableVec> {
        [
            &self.dateindex_to_date as &dyn AnyCollectableVec,
            &self.dateindex_to_dateindex,
            &self.dateindex_to_first_height,
            &self.dateindex_to_height_count,
            &self.dateindex_to_monthindex,
            &self.dateindex_to_weekindex,
            &self.decadeindex_to_decadeindex,
            &self.decadeindex_to_first_yearindex,
            &self.decadeindex_to_yearindex_count,
            &self.difficultyepoch_to_difficultyepoch,
            &self.difficultyepoch_to_first_height,
            &self.difficultyepoch_to_height_count,
            &self.emptyoutputindex_to_emptyoutputindex,
            &self.halvingepoch_to_first_height,
            &self.halvingepoch_to_halvingepoch,
            &self.height_to_date,
            &self.height_to_date_fixed,
            &self.height_to_dateindex,
            &self.height_to_difficultyepoch,
            &self.height_to_halvingepoch,
            &self.height_to_height,
            &self.height_to_timestamp_fixed,
            &self.height_to_txindex_count,
            &self.inputindex_to_inputindex,
            &self.monthindex_to_dateindex_count,
            &self.monthindex_to_first_dateindex,
            &self.monthindex_to_monthindex,
            &self.monthindex_to_quarterindex,
            &self.monthindex_to_semesterindex,
            &self.monthindex_to_yearindex,
            &self.opreturnindex_to_opreturnindex,
            &self.outputindex_to_outputindex,
            &self.p2aaddressindex_to_p2aaddressindex,
            &self.p2msoutputindex_to_p2msoutputindex,
            &self.p2pk33addressindex_to_p2pk33addressindex,
            &self.p2pk65addressindex_to_p2pk65addressindex,
            &self.p2pkhaddressindex_to_p2pkhaddressindex,
            &self.p2shaddressindex_to_p2shaddressindex,
            &self.p2traddressindex_to_p2traddressindex,
            &self.p2wpkhaddressindex_to_p2wpkhaddressindex,
            &self.p2wshaddressindex_to_p2wshaddressindex,
            &self.quarterindex_to_first_monthindex,
            &self.quarterindex_to_monthindex_count,
            &self.quarterindex_to_quarterindex,
            &self.semesterindex_to_first_monthindex,
            &self.semesterindex_to_monthindex_count,
            &self.semesterindex_to_semesterindex,
            &self.txindex_to_height,
            &self.txindex_to_txindex,
            &self.txindex_to_input_count,
            &self.txindex_to_output_count,
            &self.unknownoutputindex_to_unknownoutputindex,
            &self.weekindex_to_dateindex_count,
            &self.weekindex_to_first_dateindex,
            &self.weekindex_to_weekindex,
            &self.yearindex_to_decadeindex,
            &self.yearindex_to_first_monthindex,
            &self.yearindex_to_monthindex_count,
            &self.yearindex_to_yearindex,
            &self.outputindex_to_txindex,
        ]
        .into_iter()
    }
}

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
        self.dateindex = DateIndex::try_from(
            indexes
                .height_to_date_fixed
                .into_iter()
                .unwrap_get_inner(height),
        )
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
