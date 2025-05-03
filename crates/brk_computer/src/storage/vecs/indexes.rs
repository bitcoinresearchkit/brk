use std::{fs, ops::Deref, path::Path};

use brk_core::{
    Date, DateIndex, DecadeIndex, DifficultyEpoch, EmptyOutputIndex, HalvingEpoch, Height,
    InputIndex, MonthIndex, OpReturnIndex, OutputIndex, P2AIndex, P2MSIndex, P2PK33Index,
    P2PK65Index, P2PKHIndex, P2SHIndex, P2TRIndex, P2WPKHIndex, P2WSHIndex, QuarterIndex,
    StoredUsize, Timestamp, TxIndex, UnknownOutputIndex, WeekIndex, YearIndex,
};
use brk_exit::Exit;
use brk_indexer::Indexer;
use brk_vec::{Compressed, VecIterator, Version};

use super::EagerVec;

#[derive(Clone)]
pub struct Vecs {
    pub dateindex_to_date: EagerVec<DateIndex, Date>,
    pub dateindex_to_dateindex: EagerVec<DateIndex, DateIndex>,
    pub dateindex_to_first_height: EagerVec<DateIndex, Height>,
    pub dateindex_to_height_count: EagerVec<DateIndex, StoredUsize>,
    pub dateindex_to_monthindex: EagerVec<DateIndex, MonthIndex>,
    pub dateindex_to_weekindex: EagerVec<DateIndex, WeekIndex>,
    pub decadeindex_to_decadeindex: EagerVec<DecadeIndex, DecadeIndex>,
    pub decadeindex_to_first_yearindex: EagerVec<DecadeIndex, YearIndex>,
    pub decadeindex_to_yearindex_count: EagerVec<DecadeIndex, StoredUsize>,
    pub difficultyepoch_to_difficultyepoch: EagerVec<DifficultyEpoch, DifficultyEpoch>,
    pub difficultyepoch_to_first_height: EagerVec<DifficultyEpoch, Height>,
    pub difficultyepoch_to_height_count: EagerVec<DifficultyEpoch, StoredUsize>,
    pub emptyoutputindex_to_emptyoutputindex: EagerVec<EmptyOutputIndex, EmptyOutputIndex>,
    pub halvingepoch_to_first_height: EagerVec<HalvingEpoch, Height>,
    pub halvingepoch_to_halvingepoch: EagerVec<HalvingEpoch, HalvingEpoch>,
    pub height_to_date: EagerVec<Height, Date>,
    pub height_to_date_fixed: EagerVec<Height, Date>,
    pub height_to_dateindex: EagerVec<Height, DateIndex>,
    pub height_to_difficultyepoch: EagerVec<Height, DifficultyEpoch>,
    pub height_to_halvingepoch: EagerVec<Height, HalvingEpoch>,
    pub height_to_height: EagerVec<Height, Height>,
    pub height_to_timestamp_fixed: EagerVec<Height, Timestamp>,
    pub height_to_txindex_count: EagerVec<Height, StoredUsize>,
    pub inputindex_to_inputindex: EagerVec<InputIndex, InputIndex>,
    pub monthindex_to_dateindex_count: EagerVec<MonthIndex, StoredUsize>,
    pub monthindex_to_first_dateindex: EagerVec<MonthIndex, DateIndex>,
    pub monthindex_to_monthindex: EagerVec<MonthIndex, MonthIndex>,
    pub monthindex_to_quarterindex: EagerVec<MonthIndex, QuarterIndex>,
    pub monthindex_to_yearindex: EagerVec<MonthIndex, YearIndex>,
    pub opreturnindex_to_opreturnindex: EagerVec<OpReturnIndex, OpReturnIndex>,
    pub outputindex_to_outputindex: EagerVec<OutputIndex, OutputIndex>,
    pub p2aindex_to_p2aindex: EagerVec<P2AIndex, P2AIndex>,
    pub p2msindex_to_p2msindex: EagerVec<P2MSIndex, P2MSIndex>,
    pub p2pk33index_to_p2pk33index: EagerVec<P2PK33Index, P2PK33Index>,
    pub p2pk65index_to_p2pk65index: EagerVec<P2PK65Index, P2PK65Index>,
    pub p2pkhindex_to_p2pkhindex: EagerVec<P2PKHIndex, P2PKHIndex>,
    pub p2shindex_to_p2shindex: EagerVec<P2SHIndex, P2SHIndex>,
    pub p2trindex_to_p2trindex: EagerVec<P2TRIndex, P2TRIndex>,
    pub p2wpkhindex_to_p2wpkhindex: EagerVec<P2WPKHIndex, P2WPKHIndex>,
    pub p2wshindex_to_p2wshindex: EagerVec<P2WSHIndex, P2WSHIndex>,
    pub quarterindex_to_first_monthindex: EagerVec<QuarterIndex, MonthIndex>,
    pub quarterindex_to_monthindex_count: EagerVec<QuarterIndex, StoredUsize>,
    pub quarterindex_to_quarterindex: EagerVec<QuarterIndex, QuarterIndex>,
    pub txindex_to_height: EagerVec<TxIndex, Height>,
    pub txindex_to_txindex: EagerVec<TxIndex, TxIndex>,
    pub unknownoutputindex_to_unknownoutputindex: EagerVec<UnknownOutputIndex, UnknownOutputIndex>,
    pub weekindex_to_dateindex_count: EagerVec<WeekIndex, StoredUsize>,
    pub weekindex_to_first_dateindex: EagerVec<WeekIndex, DateIndex>,
    pub weekindex_to_weekindex: EagerVec<WeekIndex, WeekIndex>,
    pub yearindex_to_decadeindex: EagerVec<YearIndex, DecadeIndex>,
    pub yearindex_to_first_monthindex: EagerVec<YearIndex, MonthIndex>,
    pub yearindex_to_monthindex_count: EagerVec<YearIndex, StoredUsize>,
    pub yearindex_to_yearindex: EagerVec<YearIndex, YearIndex>,
}

impl Vecs {
    pub fn forced_import(path: &Path, compressed: Compressed) -> color_eyre::Result<Self> {
        fs::create_dir_all(path)?;

        Ok(Self {
            dateindex_to_date: EagerVec::forced_import(
                &path.join("dateindex_to_date"),
                Version::ZERO,
                compressed,
            )?,
            dateindex_to_dateindex: EagerVec::forced_import(
                &path.join("dateindex_to_dateindex"),
                Version::ZERO,
                compressed,
            )?,
            dateindex_to_first_height: EagerVec::forced_import(
                &path.join("dateindex_to_first_height"),
                Version::ZERO,
                compressed,
            )?,
            height_to_date: EagerVec::forced_import(
                &path.join("height_to_date"),
                Version::ZERO,
                compressed,
            )?,
            height_to_date_fixed: EagerVec::forced_import(
                &path.join("height_to_date_fixed"),
                Version::ZERO,
                compressed,
            )?,
            height_to_dateindex: EagerVec::forced_import(
                &path.join("height_to_dateindex"),
                Version::ZERO,
                compressed,
            )?,
            height_to_height: EagerVec::forced_import(
                &path.join("height_to_height"),
                Version::ZERO,
                compressed,
            )?,
            txindex_to_height: EagerVec::forced_import(
                &path.join("txindex_to_height"),
                Version::ZERO,
                compressed,
            )?,
            difficultyepoch_to_first_height: EagerVec::forced_import(
                &path.join("difficultyepoch_to_first_height"),
                Version::ZERO,
                compressed,
            )?,
            halvingepoch_to_first_height: EagerVec::forced_import(
                &path.join("halvingepoch_to_first_height"),
                Version::ZERO,
                compressed,
            )?,
            weekindex_to_first_dateindex: EagerVec::forced_import(
                &path.join("weekindex_to_first_dateindex"),
                Version::ZERO,
                compressed,
            )?,
            monthindex_to_first_dateindex: EagerVec::forced_import(
                &path.join("monthindex_to_first_dateindex"),
                Version::ZERO,
                compressed,
            )?,
            yearindex_to_first_monthindex: EagerVec::forced_import(
                &path.join("yearindex_to_first_monthindex"),
                Version::ZERO,
                compressed,
            )?,
            decadeindex_to_first_yearindex: EagerVec::forced_import(
                &path.join("decadeindex_to_first_yearindex"),
                Version::ZERO,
                compressed,
            )?,
            dateindex_to_weekindex: EagerVec::forced_import(
                &path.join("dateindex_to_weekindex"),
                Version::ZERO,
                compressed,
            )?,
            dateindex_to_monthindex: EagerVec::forced_import(
                &path.join("dateindex_to_monthindex"),
                Version::ZERO,
                compressed,
            )?,
            monthindex_to_yearindex: EagerVec::forced_import(
                &path.join("monthindex_to_yearindex"),
                Version::ZERO,
                compressed,
            )?,
            yearindex_to_decadeindex: EagerVec::forced_import(
                &path.join("yearindex_to_decadeindex"),
                Version::ZERO,
                compressed,
            )?,
            height_to_difficultyepoch: EagerVec::forced_import(
                &path.join("height_to_difficultyepoch"),
                Version::ZERO,
                compressed,
            )?,
            height_to_halvingepoch: EagerVec::forced_import(
                &path.join("height_to_halvingepoch"),
                Version::ZERO,
                compressed,
            )?,
            weekindex_to_weekindex: EagerVec::forced_import(
                &path.join("weekindex_to_weekindex"),
                Version::ZERO,
                compressed,
            )?,
            monthindex_to_monthindex: EagerVec::forced_import(
                &path.join("monthindex_to_monthindex"),
                Version::ZERO,
                compressed,
            )?,
            yearindex_to_yearindex: EagerVec::forced_import(
                &path.join("yearindex_to_yearindex"),
                Version::ZERO,
                compressed,
            )?,
            decadeindex_to_decadeindex: EagerVec::forced_import(
                &path.join("decadeindex_to_decadeindex"),
                Version::ZERO,
                compressed,
            )?,
            difficultyepoch_to_difficultyepoch: EagerVec::forced_import(
                &path.join("difficultyepoch_to_difficultyepoch"),
                Version::ZERO,
                compressed,
            )?,
            halvingepoch_to_halvingepoch: EagerVec::forced_import(
                &path.join("halvingepoch_to_halvingepoch"),
                Version::ZERO,
                compressed,
            )?,
            height_to_timestamp_fixed: EagerVec::forced_import(
                &path.join("height_to_timestamp_fixed"),
                Version::ZERO,
                compressed,
            )?,
            monthindex_to_quarterindex: EagerVec::forced_import(
                &path.join("monthindex_to_quarterindex"),
                Version::ZERO,
                compressed,
            )?,
            quarterindex_to_first_monthindex: EagerVec::forced_import(
                &path.join("quarterindex_to_first_monthindex"),
                Version::ZERO,
                compressed,
            )?,
            quarterindex_to_quarterindex: EagerVec::forced_import(
                &path.join("quarterindex_to_quarterindex"),
                Version::ZERO,
                compressed,
            )?,
            p2pk33index_to_p2pk33index: EagerVec::forced_import(
                &path.join("p2pk33index_to_p2pk33index"),
                Version::ZERO,
                compressed,
            )?,
            p2pk65index_to_p2pk65index: EagerVec::forced_import(
                &path.join("p2pk65index_to_p2pk65index"),
                Version::ZERO,
                compressed,
            )?,
            p2pkhindex_to_p2pkhindex: EagerVec::forced_import(
                &path.join("p2pkhindex_to_p2pkhindex"),
                Version::ZERO,
                compressed,
            )?,
            p2shindex_to_p2shindex: EagerVec::forced_import(
                &path.join("p2shindex_to_p2shindex"),
                Version::ZERO,
                compressed,
            )?,
            p2trindex_to_p2trindex: EagerVec::forced_import(
                &path.join("p2trindex_to_p2trindex"),
                Version::ZERO,
                compressed,
            )?,
            p2wpkhindex_to_p2wpkhindex: EagerVec::forced_import(
                &path.join("p2wpkhindex_to_p2wpkhindex"),
                Version::ZERO,
                compressed,
            )?,
            p2wshindex_to_p2wshindex: EagerVec::forced_import(
                &path.join("p2wshindex_to_p2wshindex"),
                Version::ZERO,
                compressed,
            )?,
            txindex_to_txindex: EagerVec::forced_import(
                &path.join("txindex_to_txindex"),
                Version::ZERO,
                compressed,
            )?,
            inputindex_to_inputindex: EagerVec::forced_import(
                &path.join("inputindex_to_inputindex"),
                Version::ZERO,
                compressed,
            )?,
            emptyoutputindex_to_emptyoutputindex: EagerVec::forced_import(
                &path.join("emptyoutputindex_to_emptyoutputindex"),
                Version::ZERO,
                compressed,
            )?,
            p2msindex_to_p2msindex: EagerVec::forced_import(
                &path.join("p2msindex_to_p2msindex"),
                Version::ZERO,
                compressed,
            )?,
            opreturnindex_to_opreturnindex: EagerVec::forced_import(
                &path.join("opreturnindex_to_opreturnindex"),
                Version::ZERO,
                compressed,
            )?,
            p2aindex_to_p2aindex: EagerVec::forced_import(
                &path.join("p2aindex_to_p2aindex"),
                Version::ZERO,
                compressed,
            )?,
            unknownoutputindex_to_unknownoutputindex: EagerVec::forced_import(
                &path.join("unknownoutputindex_to_unknownoutputindex"),
                Version::ZERO,
                compressed,
            )?,
            outputindex_to_outputindex: EagerVec::forced_import(
                &path.join("outputindex_to_outputindex"),
                Version::ZERO,
                compressed,
            )?,
            height_to_txindex_count: EagerVec::forced_import(
                &path.join("height_to_txindex_count"),
                Version::ZERO,
                compressed,
            )?,
            dateindex_to_height_count: EagerVec::forced_import(
                &path.join("dateindex_to_height_count"),
                Version::ZERO,
                compressed,
            )?,
            weekindex_to_dateindex_count: EagerVec::forced_import(
                &path.join("weekindex_to_dateindex_count"),
                Version::ZERO,
                compressed,
            )?,
            difficultyepoch_to_height_count: EagerVec::forced_import(
                &path.join("difficultyepoch_to_height_count"),
                Version::ZERO,
                compressed,
            )?,
            monthindex_to_dateindex_count: EagerVec::forced_import(
                &path.join("monthindex_to_dateindex_count"),
                Version::ZERO,
                compressed,
            )?,
            quarterindex_to_monthindex_count: EagerVec::forced_import(
                &path.join("quarterindex_to_monthindex_count"),
                Version::ZERO,
                compressed,
            )?,
            yearindex_to_monthindex_count: EagerVec::forced_import(
                &path.join("yearindex_to_monthindex_count"),
                Version::ZERO,
                compressed,
            )?,
            decadeindex_to_yearindex_count: EagerVec::forced_import(
                &path.join("decadeindex_to_yearindex_count"),
                Version::ZERO,
                compressed,
            )?,
        })
    }

    pub fn compute(
        &mut self,
        indexer: &Indexer,
        starting_indexes: brk_indexer::Indexes,
        exit: &Exit,
    ) -> color_eyre::Result<Indexes> {
        let indexer_vecs = indexer.vecs();

        // ---
        // OutputIndex
        // ---

        self.outputindex_to_outputindex.compute_range(
            starting_indexes.outputindex,
            indexer_vecs.outputindex_to_value.vec(),
            |i| (i, i),
            exit,
        )?;

        self.p2pk33index_to_p2pk33index.compute_range(
            starting_indexes.p2pk33index,
            indexer_vecs.p2pk33index_to_p2pk33bytes.vec(),
            |i| (i, i),
            exit,
        )?;

        self.p2pk65index_to_p2pk65index.compute_range(
            starting_indexes.p2pk65index,
            indexer_vecs.p2pk65index_to_p2pk65bytes.vec(),
            |i| (i, i),
            exit,
        )?;

        self.p2pkhindex_to_p2pkhindex.compute_range(
            starting_indexes.p2pkhindex,
            indexer_vecs.p2pkhindex_to_p2pkhbytes.vec(),
            |i| (i, i),
            exit,
        )?;

        self.p2shindex_to_p2shindex.compute_range(
            starting_indexes.p2shindex,
            indexer_vecs.p2shindex_to_p2shbytes.vec(),
            |i| (i, i),
            exit,
        )?;

        self.p2trindex_to_p2trindex.compute_range(
            starting_indexes.p2trindex,
            indexer_vecs.p2trindex_to_p2trbytes.vec(),
            |i| (i, i),
            exit,
        )?;

        self.p2wpkhindex_to_p2wpkhindex.compute_range(
            starting_indexes.p2wpkhindex,
            indexer_vecs.p2wpkhindex_to_p2wpkhbytes.vec(),
            |i| (i, i),
            exit,
        )?;

        self.p2wshindex_to_p2wshindex.compute_range(
            starting_indexes.p2wshindex,
            indexer_vecs.p2wshindex_to_p2wshbytes.vec(),
            |i| (i, i),
            exit,
        )?;

        self.emptyoutputindex_to_emptyoutputindex.compute_range(
            starting_indexes.emptyoutputindex,
            indexer_vecs.emptyoutputindex_to_txindex.vec(),
            |i| (i, i),
            exit,
        )?;

        self.p2msindex_to_p2msindex.compute_range(
            starting_indexes.p2msindex,
            indexer_vecs.p2msindex_to_txindex.vec(),
            |i| (i, i),
            exit,
        )?;

        self.opreturnindex_to_opreturnindex.compute_range(
            starting_indexes.opreturnindex,
            indexer_vecs.opreturnindex_to_txindex.vec(),
            |i| (i, i),
            exit,
        )?;

        self.p2aindex_to_p2aindex.compute_range(
            starting_indexes.p2aindex,
            indexer_vecs.p2aindex_to_p2abytes.vec(),
            |i| (i, i),
            exit,
        )?;

        self.unknownoutputindex_to_unknownoutputindex
            .compute_range(
                starting_indexes.unknownoutputindex,
                indexer_vecs.unknownoutputindex_to_txindex.vec(),
                |i| (i, i),
                exit,
            )?;

        // ---
        // InputIndex
        // ---

        self.inputindex_to_inputindex.compute_range(
            starting_indexes.inputindex,
            indexer_vecs.inputindex_to_outputindex.vec(),
            |i| (i, i),
            exit,
        )?;

        // ---
        // TxIndex
        // ---

        self.txindex_to_txindex.compute_range(
            starting_indexes.txindex,
            indexer_vecs.txindex_to_txid.vec(),
            |i| (i, i),
            exit,
        )?;

        self.height_to_txindex_count.compute_count_from_indexes(
            starting_indexes.height,
            indexer_vecs.height_to_first_txindex.vec(),
            indexer_vecs.txindex_to_txid.vec(),
            exit,
        )?;

        self.txindex_to_height.compute_inverse_less_to_more(
            starting_indexes.height,
            indexer_vecs.height_to_first_txindex.vec(),
            self.height_to_txindex_count.vec(),
            exit,
        )?;

        // ---
        // Height
        // ---

        self.height_to_height.compute_range(
            starting_indexes.height,
            indexer_vecs.height_to_timestamp.vec(),
            |h| (h, h),
            exit,
        )?;

        self.height_to_date.compute_transform(
            starting_indexes.height,
            indexer_vecs.height_to_timestamp.vec(),
            |(h, t, ..)| (h, Date::from(t)),
            exit,
        )?;

        let mut prev_timestamp_fixed = None;
        self.height_to_timestamp_fixed.compute_transform(
            starting_indexes.height,
            indexer_vecs.height_to_timestamp.vec(),
            |(h, timestamp, height_to_timestamp_fixed_iter)| {
                if prev_timestamp_fixed.is_none() {
                    if let Some(prev_h) = h.decremented() {
                        prev_timestamp_fixed.replace(
                            height_to_timestamp_fixed_iter
                                .iter()
                                .unwrap_get_inner(prev_h),
                        );
                    }
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
            self.height_to_timestamp_fixed.vec(),
            |(h, t, ..)| (h, Date::from(t)),
            exit,
        )?;

        let decremented_starting_height = starting_indexes.height.decremented().unwrap_or_default();

        // ---
        // DateIndex
        // ---

        let starting_dateindex = self
            .height_to_dateindex
            .iter()
            .get_inner(decremented_starting_height)
            .unwrap_or_default();

        self.height_to_dateindex.compute_transform(
            starting_indexes.height,
            self.height_to_date_fixed.vec(),
            |(h, d, ..)| (h, DateIndex::try_from(d).unwrap()),
            exit,
        )?;

        let starting_dateindex = if let Some(dateindex) = self
            .height_to_dateindex
            .iter()
            .get_inner(decremented_starting_height)
        {
            starting_dateindex.min(dateindex)
        } else {
            starting_dateindex
        };

        self.dateindex_to_first_height
            .compute_inverse_more_to_less(
                starting_indexes.height,
                self.height_to_dateindex.vec(),
                exit,
            )?;

        self.dateindex_to_dateindex.compute_range(
            starting_dateindex,
            self.dateindex_to_first_height.vec(),
            |di| (di, di),
            exit,
        )?;

        self.dateindex_to_date.compute_range(
            starting_dateindex,
            self.dateindex_to_dateindex.vec(),
            |di| (di, Date::from(di)),
            exit,
        )?;

        self.dateindex_to_height_count.compute_count_from_indexes(
            starting_dateindex,
            self.dateindex_to_first_height.vec(),
            indexer_vecs.height_to_weight.vec(),
            exit,
        )?;

        // ---
        // WeekIndex
        // ---

        let starting_weekindex = self
            .dateindex_to_weekindex
            .iter()
            .get_inner(starting_dateindex)
            .unwrap_or_default();

        self.dateindex_to_weekindex.compute_range(
            starting_dateindex,
            self.dateindex_to_dateindex.vec(),
            |di| (di, WeekIndex::from(di)),
            exit,
        )?;

        self.weekindex_to_first_dateindex
            .compute_inverse_more_to_less(
                starting_dateindex,
                self.dateindex_to_weekindex.vec(),
                exit,
            )?;

        self.weekindex_to_weekindex.compute_range(
            starting_weekindex,
            self.weekindex_to_first_dateindex.vec(),
            |wi| (wi, wi),
            exit,
        )?;

        self.weekindex_to_dateindex_count
            .compute_count_from_indexes(
                starting_weekindex,
                self.weekindex_to_first_dateindex.vec(),
                self.dateindex_to_date.vec(),
                exit,
            )?;

        // ---
        // DifficultyEpoch
        // ---

        let starting_difficultyepoch = self
            .height_to_difficultyepoch
            .iter()
            .get_inner(decremented_starting_height)
            .unwrap_or_default();

        self.height_to_difficultyepoch.compute_range(
            starting_indexes.height,
            self.height_to_height.vec(),
            |h| (h, DifficultyEpoch::from(h)),
            exit,
        )?;

        self.difficultyepoch_to_first_height
            .compute_inverse_more_to_less(
                starting_indexes.height,
                self.height_to_difficultyepoch.vec(),
                exit,
            )?;

        self.difficultyepoch_to_difficultyepoch.compute_range(
            starting_difficultyepoch,
            self.difficultyepoch_to_first_height.vec(),
            |i| (i, i),
            exit,
        )?;

        self.difficultyepoch_to_height_count
            .compute_count_from_indexes(
                starting_difficultyepoch,
                self.difficultyepoch_to_first_height.vec(),
                self.height_to_date.vec(),
                exit,
            )?;

        // ---
        // MonthIndex
        // ---

        let starting_monthindex = self
            .dateindex_to_monthindex
            .iter()
            .get_inner(starting_dateindex)
            .unwrap_or_default();

        self.dateindex_to_monthindex.compute_range(
            starting_dateindex,
            self.dateindex_to_dateindex.vec(),
            |di| (di, MonthIndex::from(di)),
            exit,
        )?;

        self.monthindex_to_first_dateindex
            .compute_inverse_more_to_less(
                starting_dateindex,
                self.dateindex_to_monthindex.vec(),
                exit,
            )?;

        self.monthindex_to_monthindex.compute_range(
            starting_monthindex,
            self.monthindex_to_first_dateindex.vec(),
            |mi| (mi, mi),
            exit,
        )?;

        self.monthindex_to_dateindex_count
            .compute_count_from_indexes(
                starting_monthindex,
                self.monthindex_to_first_dateindex.vec(),
                self.dateindex_to_date.vec(),
                exit,
            )?;

        // ---
        // QuarterIndex
        // ---

        let starting_quarterindex = self
            .monthindex_to_quarterindex
            .iter()
            .get_inner(starting_monthindex)
            .unwrap_or_default();

        self.monthindex_to_quarterindex.compute_range(
            starting_monthindex,
            self.monthindex_to_monthindex.vec(),
            |mi| (mi, QuarterIndex::from(mi)),
            exit,
        )?;

        self.quarterindex_to_first_monthindex
            .compute_inverse_more_to_less(
                starting_monthindex,
                self.monthindex_to_quarterindex.vec(),
                exit,
            )?;

        // let quarter_count = self.quarterindex_to_first_monthindex.len();

        self.quarterindex_to_quarterindex.compute_range(
            starting_quarterindex,
            self.quarterindex_to_first_monthindex.vec(),
            |i| (i, i),
            exit,
        )?;

        self.quarterindex_to_monthindex_count
            .compute_count_from_indexes(
                starting_quarterindex,
                self.quarterindex_to_first_monthindex.vec(),
                self.monthindex_to_monthindex.vec(),
                exit,
            )?;

        // ---
        // YearIndex
        // ---

        let starting_yearindex = self
            .monthindex_to_yearindex
            .iter()
            .get_inner(starting_monthindex)
            .unwrap_or_default();

        self.monthindex_to_yearindex.compute_range(
            starting_monthindex,
            self.monthindex_to_monthindex.vec(),
            |i| (i, YearIndex::from(i)),
            exit,
        )?;

        self.yearindex_to_first_monthindex
            .compute_inverse_more_to_less(
                starting_monthindex,
                self.monthindex_to_yearindex.vec(),
                exit,
            )?;

        self.yearindex_to_yearindex.compute_range(
            starting_yearindex,
            self.yearindex_to_first_monthindex.vec(),
            |i| (i, i),
            exit,
        )?;

        self.yearindex_to_monthindex_count
            .compute_count_from_indexes(
                starting_yearindex,
                self.yearindex_to_first_monthindex.vec(),
                self.monthindex_to_monthindex.vec(),
                exit,
            )?;
        // ---
        // HalvingEpoch
        // ---

        let starting_halvingepoch = self
            .height_to_halvingepoch
            .iter()
            .get_inner(decremented_starting_height)
            .unwrap_or_default();

        self.height_to_halvingepoch.compute_range(
            starting_indexes.height,
            self.height_to_height.vec(),
            |h| (h, HalvingEpoch::from(h)),
            exit,
        )?;

        self.halvingepoch_to_first_height
            .compute_inverse_more_to_less(
                starting_indexes.height,
                self.height_to_halvingepoch.vec(),
                exit,
            )?;

        self.halvingepoch_to_halvingepoch.compute_range(
            starting_halvingepoch,
            self.halvingepoch_to_first_height.vec(),
            |i| (i, i),
            exit,
        )?;

        // ---
        // DecadeIndex
        // ---

        let starting_decadeindex = self
            .yearindex_to_decadeindex
            .iter()
            .get_inner(starting_yearindex)
            .unwrap_or_default();

        self.yearindex_to_decadeindex.compute_range(
            starting_yearindex,
            self.yearindex_to_yearindex.vec(),
            |i| (i, DecadeIndex::from(i)),
            exit,
        )?;

        self.decadeindex_to_first_yearindex
            .compute_inverse_more_to_less(
                starting_yearindex,
                self.yearindex_to_decadeindex.vec(),
                exit,
            )?;

        self.decadeindex_to_decadeindex.compute_range(
            starting_decadeindex,
            self.decadeindex_to_first_yearindex.vec(),
            |i| (i, i),
            exit,
        )?;

        self.decadeindex_to_yearindex_count
            .compute_count_from_indexes(
                starting_decadeindex,
                self.decadeindex_to_first_yearindex.vec(),
                self.yearindex_to_yearindex.vec(),
                exit,
            )?;

        Ok(Indexes {
            indexes: starting_indexes,
            dateindex: starting_dateindex,
            weekindex: starting_weekindex,
            monthindex: starting_monthindex,
            quarterindex: starting_quarterindex,
            yearindex: starting_yearindex,
            decadeindex: starting_decadeindex,
            difficultyepoch: starting_difficultyepoch,
            halvingepoch: starting_halvingepoch,
        })
    }

    pub fn any_vecs(&self) -> Vec<&dyn brk_vec::AnyStoredVec> {
        vec![
            self.dateindex_to_date.any_vec(),
            self.dateindex_to_dateindex.any_vec(),
            self.dateindex_to_first_height.any_vec(),
            self.dateindex_to_height_count.any_vec(),
            self.dateindex_to_monthindex.any_vec(),
            self.dateindex_to_weekindex.any_vec(),
            self.decadeindex_to_decadeindex.any_vec(),
            self.decadeindex_to_first_yearindex.any_vec(),
            self.decadeindex_to_yearindex_count.any_vec(),
            self.difficultyepoch_to_difficultyepoch.any_vec(),
            self.difficultyepoch_to_first_height.any_vec(),
            self.difficultyepoch_to_height_count.any_vec(),
            self.emptyoutputindex_to_emptyoutputindex.any_vec(),
            self.halvingepoch_to_first_height.any_vec(),
            self.halvingepoch_to_halvingepoch.any_vec(),
            self.height_to_date.any_vec(),
            self.height_to_date_fixed.any_vec(),
            self.height_to_dateindex.any_vec(),
            self.height_to_difficultyepoch.any_vec(),
            self.height_to_halvingepoch.any_vec(),
            self.height_to_height.any_vec(),
            self.height_to_timestamp_fixed.any_vec(),
            self.height_to_txindex_count.any_vec(),
            self.inputindex_to_inputindex.any_vec(),
            self.monthindex_to_dateindex_count.any_vec(),
            self.monthindex_to_first_dateindex.any_vec(),
            self.monthindex_to_monthindex.any_vec(),
            self.monthindex_to_quarterindex.any_vec(),
            self.monthindex_to_yearindex.any_vec(),
            self.opreturnindex_to_opreturnindex.any_vec(),
            self.outputindex_to_outputindex.any_vec(),
            self.p2aindex_to_p2aindex.any_vec(),
            self.p2msindex_to_p2msindex.any_vec(),
            self.p2pk33index_to_p2pk33index.any_vec(),
            self.p2pk65index_to_p2pk65index.any_vec(),
            self.p2pkhindex_to_p2pkhindex.any_vec(),
            self.p2shindex_to_p2shindex.any_vec(),
            self.p2trindex_to_p2trindex.any_vec(),
            self.p2wpkhindex_to_p2wpkhindex.any_vec(),
            self.p2wshindex_to_p2wshindex.any_vec(),
            self.quarterindex_to_first_monthindex.any_vec(),
            self.quarterindex_to_monthindex_count.any_vec(),
            self.quarterindex_to_quarterindex.any_vec(),
            self.txindex_to_height.any_vec(),
            self.txindex_to_txindex.any_vec(),
            self.unknownoutputindex_to_unknownoutputindex.any_vec(),
            self.weekindex_to_dateindex_count.any_vec(),
            self.weekindex_to_first_dateindex.any_vec(),
            self.weekindex_to_weekindex.any_vec(),
            self.yearindex_to_decadeindex.any_vec(),
            self.yearindex_to_first_monthindex.any_vec(),
            self.yearindex_to_monthindex_count.any_vec(),
            self.yearindex_to_yearindex.any_vec(),
        ]
    }
}

pub struct Indexes {
    indexes: brk_indexer::Indexes,
    pub dateindex: DateIndex,
    pub weekindex: WeekIndex,
    pub monthindex: MonthIndex,
    pub quarterindex: QuarterIndex,
    pub yearindex: YearIndex,
    pub decadeindex: DecadeIndex,
    pub difficultyepoch: DifficultyEpoch,
    pub halvingepoch: HalvingEpoch,
}

impl Deref for Indexes {
    type Target = brk_indexer::Indexes;
    fn deref(&self) -> &Self::Target {
        &self.indexes
    }
}
