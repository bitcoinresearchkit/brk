use std::{fs, ops::Deref, path::Path};

use brk_core::{
    Date, DateIndex, DecadeIndex, DifficultyEpoch, EmptyOutputIndex, HalvingEpoch, Height,
    InputIndex, MonthIndex, OpReturnIndex, OutputIndex, P2AIndex, P2MSIndex, P2PK33Index,
    P2PK65Index, P2PKHIndex, P2SHIndex, P2TRIndex, P2WPKHIndex, P2WSHIndex, QuarterIndex,
    Timestamp, TxIndex, UnknownOutputIndex, WeekIndex, YearIndex,
};
use brk_exit::Exit;
use brk_indexer::Indexer;
use brk_vec::{Compressed, Version};

use super::EagerVec;

#[derive(Clone)]
pub struct Vecs {
    pub dateindex_to_date: EagerVec<DateIndex, Date>,
    pub dateindex_to_dateindex: EagerVec<DateIndex, DateIndex>,
    pub dateindex_to_first_height: EagerVec<DateIndex, Height>,
    pub dateindex_to_last_height: EagerVec<DateIndex, Height>,
    pub dateindex_to_monthindex: EagerVec<DateIndex, MonthIndex>,
    pub dateindex_to_weekindex: EagerVec<DateIndex, WeekIndex>,
    pub decadeindex_to_decadeindex: EagerVec<DecadeIndex, DecadeIndex>,
    pub decadeindex_to_first_yearindex: EagerVec<DecadeIndex, YearIndex>,
    pub decadeindex_to_last_yearindex: EagerVec<DecadeIndex, YearIndex>,
    pub difficultyepoch_to_difficultyepoch: EagerVec<DifficultyEpoch, DifficultyEpoch>,
    pub difficultyepoch_to_first_height: EagerVec<DifficultyEpoch, Height>,
    pub difficultyepoch_to_last_height: EagerVec<DifficultyEpoch, Height>,
    pub emptyoutputindex_to_emptyoutputindex: EagerVec<EmptyOutputIndex, EmptyOutputIndex>,
    pub halvingepoch_to_first_height: EagerVec<HalvingEpoch, Height>,
    pub halvingepoch_to_halvingepoch: EagerVec<HalvingEpoch, HalvingEpoch>,
    pub halvingepoch_to_last_height: EagerVec<HalvingEpoch, Height>,
    pub height_to_date: EagerVec<Height, Date>,
    pub height_to_date_fixed: EagerVec<Height, Date>,
    pub height_to_dateindex: EagerVec<Height, DateIndex>,
    pub height_to_difficultyepoch: EagerVec<Height, DifficultyEpoch>,
    pub height_to_halvingepoch: EagerVec<Height, HalvingEpoch>,
    pub height_to_height: EagerVec<Height, Height>,
    pub height_to_last_emptyoutputindex: EagerVec<Height, EmptyOutputIndex>,
    pub height_to_last_opreturnindex: EagerVec<Height, OpReturnIndex>,
    pub height_to_last_p2aindex: EagerVec<Height, P2AIndex>,
    pub height_to_last_p2msindex: EagerVec<Height, P2MSIndex>,
    pub height_to_last_p2pk33index: EagerVec<Height, P2PK33Index>,
    pub height_to_last_p2pk65index: EagerVec<Height, P2PK65Index>,
    pub height_to_last_p2pkhindex: EagerVec<Height, P2PKHIndex>,
    pub height_to_last_p2shindex: EagerVec<Height, P2SHIndex>,
    pub height_to_last_p2trindex: EagerVec<Height, P2TRIndex>,
    pub height_to_last_p2wpkhindex: EagerVec<Height, P2WPKHIndex>,
    pub height_to_last_p2wshindex: EagerVec<Height, P2WSHIndex>,
    pub height_to_last_txindex: EagerVec<Height, TxIndex>,
    pub height_to_last_unknownoutputindex: EagerVec<Height, UnknownOutputIndex>,
    pub height_to_timestamp_fixed: EagerVec<Height, Timestamp>,
    pub inputindex_to_inputindex: EagerVec<InputIndex, InputIndex>,
    pub monthindex_to_first_dateindex: EagerVec<MonthIndex, DateIndex>,
    pub monthindex_to_last_dateindex: EagerVec<MonthIndex, DateIndex>,
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
    pub quarterindex_to_last_monthindex: EagerVec<QuarterIndex, MonthIndex>,
    pub quarterindex_to_quarterindex: EagerVec<QuarterIndex, QuarterIndex>,
    pub txindex_to_height: EagerVec<TxIndex, Height>,
    pub txindex_to_last_inputindex: EagerVec<TxIndex, InputIndex>,
    pub txindex_to_last_outputindex: EagerVec<TxIndex, OutputIndex>,
    pub txindex_to_txindex: EagerVec<TxIndex, TxIndex>,
    pub unknownoutputindex_to_unknownoutputindex: EagerVec<UnknownOutputIndex, UnknownOutputIndex>,
    pub weekindex_to_first_dateindex: EagerVec<WeekIndex, DateIndex>,
    pub weekindex_to_last_dateindex: EagerVec<WeekIndex, DateIndex>,
    pub weekindex_to_weekindex: EagerVec<WeekIndex, WeekIndex>,
    pub yearindex_to_decadeindex: EagerVec<YearIndex, DecadeIndex>,
    pub yearindex_to_first_monthindex: EagerVec<YearIndex, MonthIndex>,
    pub yearindex_to_last_monthindex: EagerVec<YearIndex, MonthIndex>,
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
            dateindex_to_last_height: EagerVec::forced_import(
                &path.join("dateindex_to_last_height"),
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
            height_to_last_txindex: EagerVec::forced_import(
                &path.join("height_to_last_txindex"),
                Version::ZERO,
                compressed,
            )?,
            txindex_to_height: EagerVec::forced_import(
                &path.join("txindex_to_height"),
                Version::ZERO,
                compressed,
            )?,
            txindex_to_last_inputindex: EagerVec::forced_import(
                &path.join("txindex_to_last_inputindex"),
                Version::ZERO,
                compressed,
            )?,
            txindex_to_last_outputindex: EagerVec::forced_import(
                &path.join("txindex_to_last_outputindex"),
                Version::ZERO,
                compressed,
            )?,
            difficultyepoch_to_first_height: EagerVec::forced_import(
                &path.join("difficultyepoch_to_first_height"),
                Version::ZERO,
                compressed,
            )?,
            difficultyepoch_to_last_height: EagerVec::forced_import(
                &path.join("difficultyepoch_to_last_height"),
                Version::ZERO,
                compressed,
            )?,
            halvingepoch_to_first_height: EagerVec::forced_import(
                &path.join("halvingepoch_to_first_height"),
                Version::ZERO,
                compressed,
            )?,
            halvingepoch_to_last_height: EagerVec::forced_import(
                &path.join("halvingepoch_to_last_height"),
                Version::ZERO,
                compressed,
            )?,
            weekindex_to_first_dateindex: EagerVec::forced_import(
                &path.join("weekindex_to_first_dateindex"),
                Version::ZERO,
                compressed,
            )?,
            weekindex_to_last_dateindex: EagerVec::forced_import(
                &path.join("weekindex_to_last_dateindex"),
                Version::ZERO,
                compressed,
            )?,
            monthindex_to_first_dateindex: EagerVec::forced_import(
                &path.join("monthindex_to_first_dateindex"),
                Version::ZERO,
                compressed,
            )?,
            monthindex_to_last_dateindex: EagerVec::forced_import(
                &path.join("monthindex_to_last_dateindex"),
                Version::ZERO,
                compressed,
            )?,
            yearindex_to_first_monthindex: EagerVec::forced_import(
                &path.join("yearindex_to_first_monthindex"),
                Version::ZERO,
                compressed,
            )?,
            yearindex_to_last_monthindex: EagerVec::forced_import(
                &path.join("yearindex_to_last_monthindex"),
                Version::ZERO,
                compressed,
            )?,
            decadeindex_to_first_yearindex: EagerVec::forced_import(
                &path.join("decadeindex_to_first_yearindex"),
                Version::ZERO,
                compressed,
            )?,
            decadeindex_to_last_yearindex: EagerVec::forced_import(
                &path.join("decadeindex_to_last_yearindex"),
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
            quarterindex_to_last_monthindex: EagerVec::forced_import(
                &path.join("quarterindex_to_last_monthindex"),
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
            height_to_last_p2aindex: EagerVec::forced_import(
                &path.join("height_to_last_p2aindex"),
                Version::ZERO,
                compressed,
            )?,
            height_to_last_p2msindex: EagerVec::forced_import(
                &path.join("height_to_last_p2msindex"),
                Version::ZERO,
                compressed,
            )?,
            height_to_last_p2pk33index: EagerVec::forced_import(
                &path.join("height_to_last_p2pk33index"),
                Version::ZERO,
                compressed,
            )?,
            height_to_last_p2pk65index: EagerVec::forced_import(
                &path.join("height_to_last_p2pk65index"),
                Version::ZERO,
                compressed,
            )?,
            height_to_last_p2pkhindex: EagerVec::forced_import(
                &path.join("height_to_last_p2pkhindex"),
                Version::ZERO,
                compressed,
            )?,
            height_to_last_p2shindex: EagerVec::forced_import(
                &path.join("height_to_last_p2shindex"),
                Version::ZERO,
                compressed,
            )?,
            height_to_last_p2trindex: EagerVec::forced_import(
                &path.join("height_to_last_p2trindex"),
                Version::ZERO,
                compressed,
            )?,
            height_to_last_p2wpkhindex: EagerVec::forced_import(
                &path.join("height_to_last_p2wpkhindex"),
                Version::ZERO,
                compressed,
            )?,
            height_to_last_p2wshindex: EagerVec::forced_import(
                &path.join("height_to_last_p2wshindex"),
                Version::ZERO,
                compressed,
            )?,
            height_to_last_opreturnindex: EagerVec::forced_import(
                &path.join("height_to_last_opreturnindex"),
                Version::ZERO,
                compressed,
            )?,
            height_to_last_unknownoutputindex: EagerVec::forced_import(
                &path.join("height_to_last_unknownoutputindex"),
                Version::ZERO,
                compressed,
            )?,
            height_to_last_emptyoutputindex: EagerVec::forced_import(
                &path.join("height_to_last_emptyoutputindex"),
                Version::ZERO,
                compressed,
            )?,
        })
    }

    pub fn compute(
        &mut self,
        indexer: &mut Indexer,
        starting_indexes: brk_indexer::Indexes,
        exit: &Exit,
    ) -> color_eyre::Result<Indexes> {
        let indexer_vecs = indexer.mut_vecs();

        let height_count = indexer_vecs.height_to_total_size.len();
        let txindexes_count = indexer_vecs.txindex_to_txid.len();
        let inputindexes_count = indexer_vecs.inputindex_to_outputindex.len();
        let outputindexes_count = indexer_vecs.outputindex_to_value.len();

        // ---
        // OutputIndex
        // ---

        self.outputindex_to_outputindex.compute_range(
            starting_indexes.outputindex,
            indexer_vecs.outputindex_to_value.mut_vec(),
            |i| (i, i),
            exit,
        )?;

        self.p2pk33index_to_p2pk33index.compute_range(
            starting_indexes.p2pk33index,
            indexer_vecs.p2pk33index_to_p2pk33bytes.mut_vec(),
            |i| (i, i),
            exit,
        )?;

        self.p2pk65index_to_p2pk65index.compute_range(
            starting_indexes.p2pk65index,
            indexer_vecs.p2pk65index_to_p2pk65bytes.mut_vec(),
            |i| (i, i),
            exit,
        )?;

        self.p2pkhindex_to_p2pkhindex.compute_range(
            starting_indexes.p2pkhindex,
            indexer_vecs.p2pkhindex_to_p2pkhbytes.mut_vec(),
            |i| (i, i),
            exit,
        )?;

        self.p2shindex_to_p2shindex.compute_range(
            starting_indexes.p2shindex,
            indexer_vecs.p2shindex_to_p2shbytes.mut_vec(),
            |i| (i, i),
            exit,
        )?;

        self.p2trindex_to_p2trindex.compute_range(
            starting_indexes.p2trindex,
            indexer_vecs.p2trindex_to_p2trbytes.mut_vec(),
            |i| (i, i),
            exit,
        )?;

        self.p2wpkhindex_to_p2wpkhindex.compute_range(
            starting_indexes.p2wpkhindex,
            indexer_vecs.p2wpkhindex_to_p2wpkhbytes.mut_vec(),
            |i| (i, i),
            exit,
        )?;

        self.p2wshindex_to_p2wshindex.compute_range(
            starting_indexes.p2wshindex,
            indexer_vecs.p2wshindex_to_p2wshbytes.mut_vec(),
            |i| (i, i),
            exit,
        )?;

        self.emptyoutputindex_to_emptyoutputindex.compute_range(
            starting_indexes.emptyoutputindex,
            indexer_vecs.emptyoutputindex_to_txindex.mut_vec(),
            |i| (i, i),
            exit,
        )?;

        self.p2msindex_to_p2msindex.compute_range(
            starting_indexes.p2msindex,
            indexer_vecs.p2msindex_to_txindex.mut_vec(),
            |i| (i, i),
            exit,
        )?;

        self.opreturnindex_to_opreturnindex.compute_range(
            starting_indexes.opreturnindex,
            indexer_vecs.opreturnindex_to_txindex.mut_vec(),
            |i| (i, i),
            exit,
        )?;

        self.p2aindex_to_p2aindex.compute_range(
            starting_indexes.p2aindex,
            indexer_vecs.p2aindex_to_p2abytes.mut_vec(),
            |i| (i, i),
            exit,
        )?;

        self.unknownoutputindex_to_unknownoutputindex
            .compute_range(
                starting_indexes.unknownoutputindex,
                indexer_vecs.unknownoutputindex_to_txindex.mut_vec(),
                |i| (i, i),
                exit,
            )?;

        self.height_to_last_p2aindex.compute_last_index_from_first(
            starting_indexes.height,
            indexer_vecs.height_to_first_p2aindex.mut_vec(),
            height_count,
            exit,
        )?;

        self.height_to_last_p2msindex
            .compute_last_index_from_first(
                starting_indexes.height,
                indexer_vecs.height_to_first_p2msindex.mut_vec(),
                height_count,
                exit,
            )?;

        self.height_to_last_p2pk33index
            .compute_last_index_from_first(
                starting_indexes.height,
                indexer_vecs.height_to_first_p2pk33index.mut_vec(),
                height_count,
                exit,
            )?;

        self.height_to_last_p2pk65index
            .compute_last_index_from_first(
                starting_indexes.height,
                indexer_vecs.height_to_first_p2pk65index.mut_vec(),
                height_count,
                exit,
            )?;

        self.height_to_last_p2pkhindex
            .compute_last_index_from_first(
                starting_indexes.height,
                indexer_vecs.height_to_first_p2pkhindex.mut_vec(),
                height_count,
                exit,
            )?;

        self.height_to_last_p2shindex
            .compute_last_index_from_first(
                starting_indexes.height,
                indexer_vecs.height_to_first_p2shindex.mut_vec(),
                height_count,
                exit,
            )?;

        self.height_to_last_p2trindex
            .compute_last_index_from_first(
                starting_indexes.height,
                indexer_vecs.height_to_first_p2trindex.mut_vec(),
                height_count,
                exit,
            )?;

        self.height_to_last_p2wpkhindex
            .compute_last_index_from_first(
                starting_indexes.height,
                indexer_vecs.height_to_first_p2wpkhindex.mut_vec(),
                height_count,
                exit,
            )?;

        self.height_to_last_p2wshindex
            .compute_last_index_from_first(
                starting_indexes.height,
                indexer_vecs.height_to_first_p2wshindex.mut_vec(),
                height_count,
                exit,
            )?;

        self.height_to_last_opreturnindex
            .compute_last_index_from_first(
                starting_indexes.height,
                indexer_vecs.height_to_first_opreturnindex.mut_vec(),
                height_count,
                exit,
            )?;

        self.height_to_last_unknownoutputindex
            .compute_last_index_from_first(
                starting_indexes.height,
                indexer_vecs.height_to_first_unknownoutputindex.mut_vec(),
                height_count,
                exit,
            )?;

        self.height_to_last_emptyoutputindex
            .compute_last_index_from_first(
                starting_indexes.height,
                indexer_vecs.height_to_first_emptyoutputindex.mut_vec(),
                height_count,
                exit,
            )?;

        // ---
        // InputIndex
        // ---

        self.inputindex_to_inputindex.compute_range(
            starting_indexes.inputindex,
            indexer_vecs.inputindex_to_outputindex.mut_vec(),
            |i| (i, i),
            exit,
        )?;

        // ---
        // TxIndex
        // ---

        self.txindex_to_last_inputindex
            .compute_last_index_from_first(
                starting_indexes.txindex,
                indexer_vecs.txindex_to_first_inputindex.mut_vec(),
                inputindexes_count,
                exit,
            )?;

        self.txindex_to_last_outputindex
            .compute_last_index_from_first(
                starting_indexes.txindex,
                indexer_vecs.txindex_to_first_outputindex.mut_vec(),
                outputindexes_count,
                exit,
            )?;

        self.txindex_to_txindex.compute_range(
            starting_indexes.txindex,
            self.txindex_to_last_inputindex.mut_vec(),
            |i| (i, i),
            exit,
        )?;

        self.height_to_last_txindex.compute_last_index_from_first(
            starting_indexes.height,
            indexer_vecs.height_to_first_txindex.mut_vec(),
            txindexes_count,
            exit,
        )?;

        self.txindex_to_height.compute_inverse_less_to_more(
            starting_indexes.height,
            indexer_vecs.height_to_first_txindex.mut_vec(),
            self.height_to_last_txindex.mut_vec(),
            exit,
        )?;

        // ---
        // Height
        // ---

        self.height_to_height.compute_range(
            starting_indexes.height,
            indexer_vecs.height_to_timestamp.mut_vec(),
            |h| (h, h),
            exit,
        )?;

        self.height_to_date.compute_transform(
            starting_indexes.height,
            indexer_vecs.height_to_timestamp.mut_vec(),
            |(h, t, ..)| (h, Date::from(t)),
            exit,
        )?;

        self.height_to_timestamp_fixed.compute_transform(
            starting_indexes.height,
            indexer_vecs.height_to_timestamp.mut_vec(),
            |(h, timestamp, s, ..)| {
                let timestamp = h
                    .decremented()
                    .and_then(|h| s.unwrap_cached_get(h))
                    .map_or(timestamp, |prev_d| prev_d.max(timestamp));
                (h, timestamp)
            },
            exit,
        )?;

        self.height_to_date_fixed.compute_transform(
            starting_indexes.height,
            self.height_to_timestamp_fixed.mut_vec(),
            |(h, t, ..)| (h, Date::from(t)),
            exit,
        )?;

        let decremented_starting_height = starting_indexes.height.decremented().unwrap_or_default();

        // ---
        // DateIndex
        // ---

        let starting_dateindex = self
            .height_to_dateindex
            .unwrap_cached_get(decremented_starting_height)
            .unwrap_or_default();

        self.height_to_dateindex.compute_transform(
            starting_indexes.height,
            self.height_to_date_fixed.mut_vec(),
            |(h, d, ..)| (h, DateIndex::try_from(d).unwrap()),
            exit,
        )?;

        let starting_dateindex = if let Some(dateindex) = self
            .height_to_dateindex
            .unwrap_cached_get(decremented_starting_height)
        {
            starting_dateindex.min(dateindex)
        } else {
            starting_dateindex
        };

        self.dateindex_to_first_height
            .compute_inverse_more_to_less(
                starting_indexes.height,
                self.height_to_dateindex.mut_vec(),
                exit,
            )?;

        let date_count = self.dateindex_to_first_height.len();

        self.dateindex_to_last_height
            .compute_last_index_from_first(
                starting_dateindex,
                self.dateindex_to_first_height.mut_vec(),
                height_count,
                exit,
            )?;

        self.dateindex_to_dateindex.compute_range(
            starting_dateindex,
            self.dateindex_to_first_height.mut_vec(),
            |di| (di, di),
            exit,
        )?;

        self.dateindex_to_date.compute_range(
            starting_dateindex,
            self.dateindex_to_dateindex.mut_vec(),
            |di| (di, Date::from(di)),
            exit,
        )?;

        // ---
        // WeekIndex
        // ---

        let starting_weekindex = self
            .dateindex_to_weekindex
            .unwrap_cached_get(starting_dateindex)
            .unwrap_or_default();

        self.dateindex_to_weekindex.compute_range(
            starting_dateindex,
            self.dateindex_to_dateindex.mut_vec(),
            |di| (di, WeekIndex::from(di)),
            exit,
        )?;

        self.weekindex_to_first_dateindex
            .compute_inverse_more_to_less(
                starting_dateindex,
                self.dateindex_to_weekindex.mut_vec(),
                exit,
            )?;

        self.weekindex_to_last_dateindex
            .compute_last_index_from_first(
                starting_weekindex,
                self.weekindex_to_first_dateindex.mut_vec(),
                date_count,
                exit,
            )?;

        self.weekindex_to_weekindex.compute_range(
            starting_weekindex,
            self.weekindex_to_first_dateindex.mut_vec(),
            |wi| (wi, wi),
            exit,
        )?;

        // ---
        // DifficultyEpoch
        // ---

        let starting_difficultyepoch = self
            .height_to_difficultyepoch
            .unwrap_cached_get(decremented_starting_height)
            .unwrap_or_default();

        self.height_to_difficultyepoch.compute_range(
            starting_indexes.height,
            self.height_to_height.mut_vec(),
            |h| (h, DifficultyEpoch::from(h)),
            exit,
        )?;

        self.difficultyepoch_to_first_height
            .compute_inverse_more_to_less(
                starting_indexes.height,
                self.height_to_difficultyepoch.mut_vec(),
                exit,
            )?;

        self.difficultyepoch_to_last_height
            .compute_last_index_from_first(
                starting_difficultyepoch,
                self.difficultyepoch_to_first_height.mut_vec(),
                height_count,
                exit,
            )?;

        self.difficultyepoch_to_difficultyepoch.compute_range(
            starting_difficultyepoch,
            self.difficultyepoch_to_first_height.mut_vec(),
            |i| (i, i),
            exit,
        )?;

        // ---
        // MonthIndex
        // ---

        let starting_monthindex = self
            .dateindex_to_monthindex
            .unwrap_cached_get(starting_dateindex)
            .unwrap_or_default();

        self.dateindex_to_monthindex.compute_range(
            starting_dateindex,
            self.dateindex_to_dateindex.mut_vec(),
            |di| (di, MonthIndex::from(di)),
            exit,
        )?;

        self.monthindex_to_first_dateindex
            .compute_inverse_more_to_less(
                starting_dateindex,
                self.dateindex_to_monthindex.mut_vec(),
                exit,
            )?;

        let month_count = self.monthindex_to_first_dateindex.len();

        self.monthindex_to_last_dateindex
            .compute_last_index_from_first(
                starting_monthindex,
                self.monthindex_to_first_dateindex.mut_vec(),
                date_count,
                exit,
            )?;

        self.monthindex_to_monthindex.compute_range(
            starting_monthindex,
            self.monthindex_to_first_dateindex.mut_vec(),
            |mi| (mi, mi),
            exit,
        )?;

        // ---
        // QuarterIndex
        // ---

        let starting_quarterindex = self
            .monthindex_to_quarterindex
            .unwrap_cached_get(starting_monthindex)
            .unwrap_or_default();

        self.monthindex_to_quarterindex.compute_range(
            starting_monthindex,
            self.monthindex_to_monthindex.mut_vec(),
            |mi| (mi, QuarterIndex::from(mi)),
            exit,
        )?;

        self.quarterindex_to_first_monthindex
            .compute_inverse_more_to_less(
                starting_monthindex,
                self.monthindex_to_quarterindex.mut_vec(),
                exit,
            )?;

        // let quarter_count = self.quarterindex_to_first_monthindex.len();

        self.quarterindex_to_last_monthindex
            .compute_last_index_from_first(
                starting_quarterindex,
                self.quarterindex_to_first_monthindex.mut_vec(),
                month_count,
                exit,
            )?;

        self.quarterindex_to_quarterindex.compute_range(
            starting_quarterindex,
            self.quarterindex_to_first_monthindex.mut_vec(),
            |i| (i, i),
            exit,
        )?;

        // ---
        // YearIndex
        // ---

        let starting_yearindex = self
            .monthindex_to_yearindex
            .unwrap_cached_get(starting_monthindex)
            .unwrap_or_default();

        self.monthindex_to_yearindex.compute_range(
            starting_monthindex,
            self.monthindex_to_monthindex.mut_vec(),
            |i| (i, YearIndex::from(i)),
            exit,
        )?;

        self.yearindex_to_first_monthindex
            .compute_inverse_more_to_less(
                starting_monthindex,
                self.monthindex_to_yearindex.mut_vec(),
                exit,
            )?;

        let year_count = self.yearindex_to_first_monthindex.len();

        self.yearindex_to_last_monthindex
            .compute_last_index_from_first(
                starting_yearindex,
                self.yearindex_to_first_monthindex.mut_vec(),
                month_count,
                exit,
            )?;

        self.yearindex_to_yearindex.compute_range(
            starting_yearindex,
            self.yearindex_to_first_monthindex.mut_vec(),
            |i| (i, i),
            exit,
        )?;

        // ---
        // HalvingEpoch
        // ---

        let starting_halvingepoch = self
            .height_to_halvingepoch
            .unwrap_cached_get(decremented_starting_height)
            .unwrap_or_default();

        self.height_to_halvingepoch.compute_range(
            starting_indexes.height,
            self.height_to_height.mut_vec(),
            |h| (h, HalvingEpoch::from(h)),
            exit,
        )?;

        self.halvingepoch_to_first_height
            .compute_inverse_more_to_less(
                starting_indexes.height,
                self.height_to_halvingepoch.mut_vec(),
                exit,
            )?;

        self.halvingepoch_to_last_height
            .compute_last_index_from_first(
                starting_halvingepoch,
                self.halvingepoch_to_first_height.mut_vec(),
                height_count,
                exit,
            )?;

        self.halvingepoch_to_halvingepoch.compute_range(
            starting_halvingepoch,
            self.halvingepoch_to_first_height.mut_vec(),
            |i| (i, i),
            exit,
        )?;

        // ---
        // DecadeIndex
        // ---

        let starting_decadeindex = self
            .yearindex_to_decadeindex
            .unwrap_cached_get(starting_yearindex)
            .unwrap_or_default();

        self.yearindex_to_decadeindex.compute_range(
            starting_yearindex,
            self.yearindex_to_yearindex.mut_vec(),
            |i| (i, DecadeIndex::from(i)),
            exit,
        )?;

        self.decadeindex_to_first_yearindex
            .compute_inverse_more_to_less(
                starting_yearindex,
                self.yearindex_to_decadeindex.mut_vec(),
                exit,
            )?;

        self.decadeindex_to_last_yearindex
            .compute_last_index_from_first(
                starting_decadeindex,
                self.decadeindex_to_first_yearindex.mut_vec(),
                year_count,
                exit,
            )?;

        self.decadeindex_to_decadeindex.compute_range(
            starting_decadeindex,
            self.decadeindex_to_first_yearindex.mut_vec(),
            |i| (i, i),
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

    pub fn as_any_vecs(&self) -> Vec<&dyn brk_vec::AnyStoredVec> {
        vec![
            self.dateindex_to_date.any_vec(),
            self.dateindex_to_dateindex.any_vec(),
            self.dateindex_to_first_height.any_vec(),
            self.dateindex_to_last_height.any_vec(),
            self.height_to_dateindex.any_vec(),
            self.height_to_date_fixed.any_vec(),
            self.height_to_height.any_vec(),
            self.height_to_last_txindex.any_vec(),
            self.height_to_date.any_vec(),
            self.txindex_to_last_inputindex.any_vec(),
            self.txindex_to_last_outputindex.any_vec(),
            self.difficultyepoch_to_first_height.any_vec(),
            self.difficultyepoch_to_last_height.any_vec(),
            self.halvingepoch_to_first_height.any_vec(),
            self.halvingepoch_to_last_height.any_vec(),
            self.weekindex_to_first_dateindex.any_vec(),
            self.weekindex_to_last_dateindex.any_vec(),
            self.monthindex_to_first_dateindex.any_vec(),
            self.monthindex_to_last_dateindex.any_vec(),
            self.yearindex_to_first_monthindex.any_vec(),
            self.yearindex_to_last_monthindex.any_vec(),
            self.decadeindex_to_first_yearindex.any_vec(),
            self.decadeindex_to_last_yearindex.any_vec(),
            self.dateindex_to_weekindex.any_vec(),
            self.dateindex_to_monthindex.any_vec(),
            self.monthindex_to_yearindex.any_vec(),
            self.yearindex_to_decadeindex.any_vec(),
            self.height_to_difficultyepoch.any_vec(),
            self.height_to_halvingepoch.any_vec(),
            self.weekindex_to_weekindex.any_vec(),
            self.monthindex_to_monthindex.any_vec(),
            self.yearindex_to_yearindex.any_vec(),
            self.decadeindex_to_decadeindex.any_vec(),
            self.difficultyepoch_to_difficultyepoch.any_vec(),
            self.halvingepoch_to_halvingepoch.any_vec(),
            self.height_to_timestamp_fixed.any_vec(),
            self.monthindex_to_quarterindex.any_vec(),
            self.quarterindex_to_first_monthindex.any_vec(),
            self.quarterindex_to_last_monthindex.any_vec(),
            self.quarterindex_to_quarterindex.any_vec(),
            self.p2pk33index_to_p2pk33index.any_vec(),
            self.p2pk65index_to_p2pk65index.any_vec(),
            self.p2pkhindex_to_p2pkhindex.any_vec(),
            self.p2shindex_to_p2shindex.any_vec(),
            self.p2trindex_to_p2trindex.any_vec(),
            self.p2wpkhindex_to_p2wpkhindex.any_vec(),
            self.p2wshindex_to_p2wshindex.any_vec(),
            self.txindex_to_txindex.any_vec(),
            self.inputindex_to_inputindex.any_vec(),
            self.emptyoutputindex_to_emptyoutputindex.any_vec(),
            self.p2msindex_to_p2msindex.any_vec(),
            self.opreturnindex_to_opreturnindex.any_vec(),
            self.p2aindex_to_p2aindex.any_vec(),
            self.unknownoutputindex_to_unknownoutputindex.any_vec(),
            self.outputindex_to_outputindex.any_vec(),
            self.height_to_last_p2aindex.any_vec(),
            self.height_to_last_p2msindex.any_vec(),
            self.height_to_last_p2pk33index.any_vec(),
            self.height_to_last_p2pk65index.any_vec(),
            self.height_to_last_p2pkhindex.any_vec(),
            self.height_to_last_p2shindex.any_vec(),
            self.height_to_last_p2trindex.any_vec(),
            self.height_to_last_p2wpkhindex.any_vec(),
            self.height_to_last_p2wshindex.any_vec(),
            self.height_to_last_opreturnindex.any_vec(),
            self.height_to_last_unknownoutputindex.any_vec(),
            self.height_to_last_emptyoutputindex.any_vec(),
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
