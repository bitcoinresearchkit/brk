use std::{fs, ops::Deref, path::Path};

use brk_core::{
    Date, DateIndex, DecadeIndex, DifficultyEpoch, EmptyOutputIndex, HalvingEpoch, Height,
    InputIndex, MonthIndex, OpReturnIndex, OutputIndex, P2ABytes, P2AIndex, P2MSIndex, P2PK33Bytes,
    P2PK33Index, P2PK65Bytes, P2PK65Index, P2PKHBytes, P2PKHIndex, P2SHBytes, P2SHIndex, P2TRBytes,
    P2TRIndex, P2WPKHBytes, P2WPKHIndex, P2WSHBytes, P2WSHIndex, QuarterIndex, Sats, StoredUsize,
    Timestamp, TxIndex, Txid, UnknownOutputIndex, Version, WeekIndex, Weight, YearIndex,
};
use brk_exit::Exit;
use brk_indexer::Indexer;
use brk_vec::{
    AnyCollectableVec, CloneableAnyIterableVec, Computation, ComputedVec, ComputedVecFrom1,
    ComputedVecFrom2, EagerVec, Format, StoredIndex, VecIterator,
};

use crate::vecs::indexes;

const VERSION: Version = Version::ZERO;

#[derive(Clone)]
pub struct Vecs {
    pub dateindex_to_date: ComputedVecFrom1<DateIndex, Date, DateIndex, DateIndex>,
    pub dateindex_to_dateindex: ComputedVecFrom1<DateIndex, DateIndex, DateIndex, Height>,
    pub dateindex_to_first_height: EagerVec<DateIndex, Height>,
    pub dateindex_to_height_count: EagerVec<DateIndex, StoredUsize>,
    pub dateindex_to_monthindex: EagerVec<DateIndex, MonthIndex>,
    pub dateindex_to_weekindex: EagerVec<DateIndex, WeekIndex>,
    pub decadeindex_to_decadeindex:
        ComputedVecFrom1<DecadeIndex, DecadeIndex, DecadeIndex, YearIndex>,
    pub decadeindex_to_first_yearindex: EagerVec<DecadeIndex, YearIndex>,
    pub decadeindex_to_yearindex_count: EagerVec<DecadeIndex, StoredUsize>,
    pub difficultyepoch_to_difficultyepoch:
        ComputedVecFrom1<DifficultyEpoch, DifficultyEpoch, DifficultyEpoch, Height>,
    pub difficultyepoch_to_first_height: EagerVec<DifficultyEpoch, Height>,
    pub difficultyepoch_to_height_count: EagerVec<DifficultyEpoch, StoredUsize>,
    pub emptyoutputindex_to_emptyoutputindex:
        ComputedVecFrom1<EmptyOutputIndex, EmptyOutputIndex, EmptyOutputIndex, TxIndex>,
    pub halvingepoch_to_first_height: EagerVec<HalvingEpoch, Height>,
    pub halvingepoch_to_halvingepoch:
        ComputedVecFrom1<HalvingEpoch, HalvingEpoch, HalvingEpoch, Height>,
    pub height_to_date: EagerVec<Height, Date>,
    pub height_to_date_fixed: EagerVec<Height, Date>,
    pub height_to_dateindex: EagerVec<Height, DateIndex>,
    pub height_to_difficultyepoch: ComputedVecFrom1<Height, DifficultyEpoch, Height, Height>,
    pub height_to_halvingepoch: ComputedVecFrom1<Height, HalvingEpoch, Height, Height>,
    pub height_to_height: ComputedVecFrom1<Height, Height, Height, Weight>,
    pub height_to_timestamp_fixed: EagerVec<Height, Timestamp>,
    pub height_to_txindex_count: EagerVec<Height, StoredUsize>,
    pub inputindex_to_inputindex: ComputedVecFrom1<InputIndex, InputIndex, InputIndex, OutputIndex>,
    pub monthindex_to_dateindex_count: EagerVec<MonthIndex, StoredUsize>,
    pub monthindex_to_first_dateindex: EagerVec<MonthIndex, DateIndex>,
    pub monthindex_to_monthindex: ComputedVecFrom1<MonthIndex, MonthIndex, MonthIndex, DateIndex>,
    pub monthindex_to_quarterindex:
        ComputedVecFrom1<MonthIndex, QuarterIndex, MonthIndex, MonthIndex>,
    pub monthindex_to_yearindex: ComputedVecFrom1<MonthIndex, YearIndex, MonthIndex, MonthIndex>,
    pub opreturnindex_to_opreturnindex:
        ComputedVecFrom1<OpReturnIndex, OpReturnIndex, OpReturnIndex, TxIndex>,
    pub outputindex_to_outputindex: ComputedVecFrom1<OutputIndex, OutputIndex, OutputIndex, Sats>,
    pub outputindex_to_txindex: EagerVec<OutputIndex, TxIndex>,
    pub p2aindex_to_p2aindex: ComputedVecFrom1<P2AIndex, P2AIndex, P2AIndex, P2ABytes>,
    pub p2msindex_to_p2msindex: ComputedVecFrom1<P2MSIndex, P2MSIndex, P2MSIndex, TxIndex>,
    pub p2pk33index_to_p2pk33index:
        ComputedVecFrom1<P2PK33Index, P2PK33Index, P2PK33Index, P2PK33Bytes>,
    pub p2pk65index_to_p2pk65index:
        ComputedVecFrom1<P2PK65Index, P2PK65Index, P2PK65Index, P2PK65Bytes>,
    pub p2pkhindex_to_p2pkhindex: ComputedVecFrom1<P2PKHIndex, P2PKHIndex, P2PKHIndex, P2PKHBytes>,
    pub p2shindex_to_p2shindex: ComputedVecFrom1<P2SHIndex, P2SHIndex, P2SHIndex, P2SHBytes>,
    pub p2trindex_to_p2trindex: ComputedVecFrom1<P2TRIndex, P2TRIndex, P2TRIndex, P2TRBytes>,
    pub p2wpkhindex_to_p2wpkhindex:
        ComputedVecFrom1<P2WPKHIndex, P2WPKHIndex, P2WPKHIndex, P2WPKHBytes>,
    pub p2wshindex_to_p2wshindex: ComputedVecFrom1<P2WSHIndex, P2WSHIndex, P2WSHIndex, P2WSHBytes>,
    pub quarterindex_to_first_monthindex: EagerVec<QuarterIndex, MonthIndex>,
    pub quarterindex_to_monthindex_count: EagerVec<QuarterIndex, StoredUsize>,
    pub quarterindex_to_quarterindex:
        ComputedVecFrom1<QuarterIndex, QuarterIndex, QuarterIndex, MonthIndex>,
    pub txindex_to_height: EagerVec<TxIndex, Height>,
    pub txindex_to_input_count:
        ComputedVecFrom2<TxIndex, StoredUsize, TxIndex, InputIndex, InputIndex, OutputIndex>,
    pub txindex_to_output_count:
        ComputedVecFrom2<TxIndex, StoredUsize, TxIndex, OutputIndex, OutputIndex, Sats>,
    pub txindex_to_txindex: ComputedVecFrom1<TxIndex, TxIndex, TxIndex, Txid>,
    pub unknownoutputindex_to_unknownoutputindex:
        ComputedVecFrom1<UnknownOutputIndex, UnknownOutputIndex, UnknownOutputIndex, TxIndex>,
    pub weekindex_to_dateindex_count: EagerVec<WeekIndex, StoredUsize>,
    pub weekindex_to_first_dateindex: EagerVec<WeekIndex, DateIndex>,
    pub weekindex_to_weekindex: ComputedVecFrom1<WeekIndex, WeekIndex, WeekIndex, DateIndex>,
    pub yearindex_to_decadeindex: ComputedVecFrom1<YearIndex, DecadeIndex, YearIndex, YearIndex>,
    pub yearindex_to_first_monthindex: EagerVec<YearIndex, MonthIndex>,
    pub yearindex_to_monthindex_count: EagerVec<YearIndex, StoredUsize>,
    pub yearindex_to_yearindex: ComputedVecFrom1<YearIndex, YearIndex, YearIndex, MonthIndex>,
}

impl Vecs {
    pub fn forced_import(
        path: &Path,
        version: Version,
        indexer: &Indexer,
        computation: Computation,
        format: Format,
    ) -> color_eyre::Result<Self> {
        fs::create_dir_all(path)?;

        let outputindex_to_outputindex = ComputedVec::forced_import_or_init_from_1(
            computation,
            path,
            "outputindex",
            version + VERSION + Version::ZERO,
            format,
            indexer.vecs().outputindex_to_value.boxed_clone(),
            |index, _| Some(index),
        )?;

        let inputindex_to_inputindex = ComputedVec::forced_import_or_init_from_1(
            computation,
            path,
            "inputindex",
            version + VERSION + Version::ZERO,
            format,
            indexer.vecs().inputindex_to_outputindex.boxed_clone(),
            |index, _| Some(index),
        )?;

        let txindex_to_txindex = ComputedVec::forced_import_or_init_from_1(
            computation,
            path,
            "txindex",
            version + VERSION + Version::ZERO,
            format,
            indexer.vecs().txindex_to_txid.boxed_clone(),
            |index, _| Some(index),
        )?;

        let txindex_to_input_count = ComputedVec::forced_import_or_init_from_2(
            computation,
            path,
            "input_count",
            version + VERSION + Version::ZERO,
            format,
            indexer.vecs().txindex_to_first_inputindex.boxed_clone(),
            indexer.vecs().inputindex_to_outputindex.boxed_clone(),
            |index: TxIndex, txindex_to_first_inputindex_iter, inputindex_to_outputindex_iter| {
                let txindex = index.unwrap_to_usize();
                txindex_to_first_inputindex_iter
                    .next_at(txindex)
                    .map(|(_, start)| {
                        let start = usize::from(start.into_inner());
                        let end = txindex_to_first_inputindex_iter
                            .next_at(txindex + 1)
                            .map(|(_, v)| usize::from(v.into_inner()))
                            .unwrap_or_else(|| inputindex_to_outputindex_iter.len());
                        StoredUsize::from((start..end).count())
                    })
            },
        )?;

        let txindex_to_output_count = ComputedVec::forced_import_or_init_from_2(
            computation,
            path,
            "output_count",
            version + VERSION + Version::ZERO,
            format,
            indexer.vecs().txindex_to_first_outputindex.boxed_clone(),
            indexer.vecs().outputindex_to_value.boxed_clone(),
            |index: TxIndex, txindex_to_first_outputindex_iter, outputindex_to_value_iter| {
                let txindex = index.unwrap_to_usize();
                txindex_to_first_outputindex_iter
                    .next_at(txindex)
                    .map(|(_, start)| {
                        let start = usize::from(start.into_inner());
                        let end = txindex_to_first_outputindex_iter
                            .next_at(txindex + 1)
                            .map(|(_, v)| usize::from(v.into_inner()))
                            .unwrap_or_else(|| outputindex_to_value_iter.len());
                        StoredUsize::from((start..end).count())
                    })
            },
        )?;

        let p2pk33index_to_p2pk33index = ComputedVec::forced_import_or_init_from_1(
            computation,
            path,
            "p2pk33index",
            version + VERSION + Version::ZERO,
            format,
            indexer.vecs().p2pk33index_to_p2pk33bytes.boxed_clone(),
            |index, _| Some(index),
        )?;
        let p2pk65index_to_p2pk65index = ComputedVec::forced_import_or_init_from_1(
            computation,
            path,
            "p2pk65index",
            version + VERSION + Version::ZERO,
            format,
            indexer.vecs().p2pk65index_to_p2pk65bytes.boxed_clone(),
            |index, _| Some(index),
        )?;
        let p2pkhindex_to_p2pkhindex = ComputedVec::forced_import_or_init_from_1(
            computation,
            path,
            "p2pkhindex",
            version + VERSION + Version::ZERO,
            format,
            indexer.vecs().p2pkhindex_to_p2pkhbytes.boxed_clone(),
            |index, _| Some(index),
        )?;
        let p2shindex_to_p2shindex = ComputedVec::forced_import_or_init_from_1(
            computation,
            path,
            "p2shindex",
            version + VERSION + Version::ZERO,
            format,
            indexer.vecs().p2shindex_to_p2shbytes.boxed_clone(),
            |index, _| Some(index),
        )?;
        let p2trindex_to_p2trindex = ComputedVec::forced_import_or_init_from_1(
            computation,
            path,
            "p2trindex",
            version + VERSION + Version::ZERO,
            format,
            indexer.vecs().p2trindex_to_p2trbytes.boxed_clone(),
            |index, _| Some(index),
        )?;
        let p2wpkhindex_to_p2wpkhindex = ComputedVec::forced_import_or_init_from_1(
            computation,
            path,
            "p2wpkhindex",
            version + VERSION + Version::ZERO,
            format,
            indexer.vecs().p2wpkhindex_to_p2wpkhbytes.boxed_clone(),
            |index, _| Some(index),
        )?;
        let p2wshindex_to_p2wshindex = ComputedVec::forced_import_or_init_from_1(
            computation,
            path,
            "p2wshindex",
            version + VERSION + Version::ZERO,
            format,
            indexer.vecs().p2wshindex_to_p2wshbytes.boxed_clone(),
            |index, _| Some(index),
        )?;
        let p2aindex_to_p2aindex = ComputedVec::forced_import_or_init_from_1(
            computation,
            path,
            "p2aindex",
            version + VERSION + Version::ZERO,
            format,
            indexer.vecs().p2aindex_to_p2abytes.boxed_clone(),
            |index, _| Some(index),
        )?;
        let p2msindex_to_p2msindex = ComputedVec::forced_import_or_init_from_1(
            computation,
            path,
            "p2msindex",
            version + VERSION + Version::ZERO,
            format,
            indexer.vecs().p2msindex_to_txindex.boxed_clone(),
            |index, _| Some(index),
        )?;
        let emptyoutputindex_to_emptyoutputindex = ComputedVec::forced_import_or_init_from_1(
            computation,
            path,
            "emptyoutputindex",
            version + VERSION + Version::ZERO,
            format,
            indexer.vecs().emptyoutputindex_to_txindex.boxed_clone(),
            |index, _| Some(index),
        )?;
        let unknownoutputindex_to_unknownoutputindex = ComputedVec::forced_import_or_init_from_1(
            computation,
            path,
            "unknownoutputindex",
            version + VERSION + Version::ZERO,
            format,
            indexer.vecs().unknownoutputindex_to_txindex.boxed_clone(),
            |index, _| Some(index),
        )?;
        let opreturnindex_to_opreturnindex = ComputedVec::forced_import_or_init_from_1(
            computation,
            path,
            "opreturnindex",
            version + VERSION + Version::ZERO,
            format,
            indexer.vecs().opreturnindex_to_txindex.boxed_clone(),
            |index, _| Some(index),
        )?;

        let dateindex_to_first_height = EagerVec::forced_import(
            path,
            "first_height",
            version + VERSION + Version::ZERO,
            format,
        )?;

        let dateindex_to_dateindex = ComputedVec::forced_import_or_init_from_1(
            computation,
            path,
            "dateindex",
            version + VERSION + Version::ZERO,
            format,
            dateindex_to_first_height.boxed_clone(),
            |index, _| Some(index),
        )?;

        let dateindex_to_date = ComputedVec::forced_import_or_init_from_1(
            computation,
            path,
            "date",
            version + VERSION + Version::ZERO,
            format,
            dateindex_to_dateindex.boxed_clone(),
            |index, _| Some(Date::from(index)),
        )?;

        let height_to_date =
            EagerVec::forced_import(path, "date", version + VERSION + Version::ZERO, format)?;

        let height_to_height = ComputedVec::forced_import_or_init_from_1(
            computation,
            path,
            "height",
            version + VERSION + Version::ZERO,
            format,
            indexer.vecs().height_to_weight.boxed_clone(),
            |index, _| Some(index),
        )?;

        let height_to_difficultyepoch = ComputedVec::forced_import_or_init_from_1(
            computation,
            path,
            "difficultyepoch",
            version + VERSION + Version::ZERO,
            format,
            height_to_height.boxed_clone(),
            |index, _| Some(DifficultyEpoch::from(index)),
        )?;

        let difficultyepoch_to_first_height = EagerVec::forced_import(
            path,
            "first_height",
            version + VERSION + Version::ZERO,
            format,
        )?;

        let difficultyepoch_to_difficultyepoch = ComputedVec::forced_import_or_init_from_1(
            computation,
            path,
            "difficultyepoch",
            version + VERSION + Version::ZERO,
            format,
            difficultyepoch_to_first_height.boxed_clone(),
            |index, _| Some(index),
        )?;

        let height_to_halvingepoch = ComputedVec::forced_import_or_init_from_1(
            computation,
            path,
            "halvingepoch",
            version + VERSION + Version::ZERO,
            format,
            height_to_height.boxed_clone(),
            |index, _| Some(HalvingEpoch::from(index)),
        )?;

        let halvingepoch_to_first_height = EagerVec::forced_import(
            path,
            "first_height",
            version + VERSION + Version::ZERO,
            format,
        )?;

        let halvingepoch_to_halvingepoch = ComputedVec::forced_import_or_init_from_1(
            computation,
            path,
            "halvingepoch",
            version + VERSION + Version::ZERO,
            format,
            halvingepoch_to_first_height.boxed_clone(),
            |index, _| Some(index),
        )?;

        let dateindex_to_weekindex =
            EagerVec::forced_import(path, "weekindex", version + VERSION + Version::ZERO, format)?;

        let weekindex_to_first_dateindex = EagerVec::forced_import(
            path,
            "first_dateindex",
            version + VERSION + Version::ZERO,
            format,
        )?;

        let weekindex_to_weekindex = ComputedVec::forced_import_or_init_from_1(
            computation,
            path,
            "weekindex",
            version + VERSION + Version::ZERO,
            format,
            weekindex_to_first_dateindex.boxed_clone(),
            |index, _| Some(index),
        )?;

        let dateindex_to_monthindex = EagerVec::forced_import(
            path,
            "monthindex",
            version + VERSION + Version::ZERO,
            format,
        )?;

        let monthindex_to_first_dateindex = EagerVec::forced_import(
            path,
            "first_dateindex",
            version + VERSION + Version::ZERO,
            format,
        )?;

        let monthindex_to_monthindex = ComputedVec::forced_import_or_init_from_1(
            computation,
            path,
            "monthindex",
            version + VERSION + Version::ZERO,
            format,
            monthindex_to_first_dateindex.boxed_clone(),
            |index, _| Some(index),
        )?;

        let monthindex_to_quarterindex = ComputedVec::forced_import_or_init_from_1(
            computation,
            path,
            "quarterindex",
            version + VERSION + Version::ZERO,
            format,
            monthindex_to_monthindex.boxed_clone(),
            |index, _| Some(QuarterIndex::from(index)),
        )?;

        let quarterindex_to_first_monthindex = EagerVec::forced_import(
            path,
            "first_monthindex",
            version + VERSION + Version::ZERO,
            format,
        )?;

        let quarterindex_to_quarterindex = ComputedVec::forced_import_or_init_from_1(
            computation,
            path,
            "quarterindex",
            version + VERSION + Version::ZERO,
            format,
            quarterindex_to_first_monthindex.boxed_clone(),
            |index, _| Some(index),
        )?;

        let monthindex_to_yearindex = ComputedVec::forced_import_or_init_from_1(
            computation,
            path,
            "yearindex",
            version + VERSION + Version::ZERO,
            format,
            monthindex_to_monthindex.boxed_clone(),
            |index, _| Some(YearIndex::from(index)),
        )?;

        let yearindex_to_first_monthindex = EagerVec::forced_import(
            path,
            "first_monthindex",
            version + VERSION + Version::ZERO,
            format,
        )?;

        let yearindex_to_yearindex = ComputedVec::forced_import_or_init_from_1(
            computation,
            path,
            "yearindex",
            version + VERSION + Version::ZERO,
            format,
            yearindex_to_first_monthindex.boxed_clone(),
            |index, _| Some(index),
        )?;

        let yearindex_to_decadeindex = ComputedVec::forced_import_or_init_from_1(
            computation,
            path,
            "decadeindex",
            version + VERSION + Version::ZERO,
            format,
            yearindex_to_yearindex.boxed_clone(),
            |index, _| Some(DecadeIndex::from(index)),
        )?;

        let decadeindex_to_first_yearindex = EagerVec::forced_import(
            path,
            "first_yearindex",
            version + VERSION + Version::ZERO,
            format,
        )?;

        let decadeindex_to_decadeindex = ComputedVec::forced_import_or_init_from_1(
            computation,
            path,
            "decadeindex",
            version + VERSION + Version::ZERO,
            format,
            decadeindex_to_first_yearindex.boxed_clone(),
            |index, _| Some(index),
        )?;

        Ok(Self {
            dateindex_to_date,
            dateindex_to_dateindex,
            dateindex_to_first_height,
            dateindex_to_monthindex,
            dateindex_to_weekindex,
            decadeindex_to_decadeindex,
            decadeindex_to_first_yearindex,
            difficultyepoch_to_difficultyepoch,
            difficultyepoch_to_first_height,
            emptyoutputindex_to_emptyoutputindex,
            halvingepoch_to_first_height,
            halvingepoch_to_halvingepoch,
            height_to_date,
            height_to_difficultyepoch,
            height_to_halvingepoch,
            height_to_height,
            inputindex_to_inputindex,
            monthindex_to_first_dateindex,
            monthindex_to_monthindex,
            monthindex_to_quarterindex,
            monthindex_to_yearindex,
            opreturnindex_to_opreturnindex,
            outputindex_to_outputindex,
            p2aindex_to_p2aindex,
            p2msindex_to_p2msindex,
            p2pk33index_to_p2pk33index,
            p2pk65index_to_p2pk65index,
            p2pkhindex_to_p2pkhindex,
            p2shindex_to_p2shindex,
            p2trindex_to_p2trindex,
            p2wpkhindex_to_p2wpkhindex,
            p2wshindex_to_p2wshindex,
            quarterindex_to_first_monthindex,
            quarterindex_to_quarterindex,
            txindex_to_txindex,
            txindex_to_input_count,
            txindex_to_output_count,
            unknownoutputindex_to_unknownoutputindex,
            weekindex_to_first_dateindex,
            weekindex_to_weekindex,
            yearindex_to_decadeindex,
            yearindex_to_first_monthindex,
            yearindex_to_yearindex,
            height_to_date_fixed: EagerVec::forced_import(
                path,
                "date_fixed",
                version + VERSION + Version::ZERO,
                format,
            )?,
            height_to_dateindex: EagerVec::forced_import(
                path,
                "dateindex",
                version + VERSION + Version::ZERO,
                format,
            )?,
            txindex_to_height: EagerVec::forced_import(
                path,
                "height",
                version + VERSION + Version::ZERO,
                format,
            )?,
            height_to_timestamp_fixed: EagerVec::forced_import(
                path,
                "timestamp_fixed",
                version + VERSION + Version::ZERO,
                format,
            )?,
            height_to_txindex_count: EagerVec::forced_import(
                path,
                "txindex_count",
                version + VERSION + Version::ZERO,
                format,
            )?,
            dateindex_to_height_count: EagerVec::forced_import(
                path,
                "height_count",
                version + VERSION + Version::ZERO,
                format,
            )?,
            weekindex_to_dateindex_count: EagerVec::forced_import(
                path,
                "dateindex_count",
                version + VERSION + Version::ZERO,
                format,
            )?,
            difficultyepoch_to_height_count: EagerVec::forced_import(
                path,
                "height_count",
                version + VERSION + Version::ZERO,
                format,
            )?,
            monthindex_to_dateindex_count: EagerVec::forced_import(
                path,
                "dateindex_count",
                version + VERSION + Version::ZERO,
                format,
            )?,
            quarterindex_to_monthindex_count: EagerVec::forced_import(
                path,
                "monthindex_count",
                version + VERSION + Version::ZERO,
                format,
            )?,
            yearindex_to_monthindex_count: EagerVec::forced_import(
                path,
                "monthindex_count",
                version + VERSION + Version::ZERO,
                format,
            )?,
            decadeindex_to_yearindex_count: EagerVec::forced_import(
                path,
                "yearindex_count",
                version + VERSION + Version::ZERO,
                format,
            )?,
            outputindex_to_txindex: EagerVec::forced_import(
                path,
                "txindex",
                version + VERSION + Version::ZERO,
                format,
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

        self.outputindex_to_outputindex.compute_if_necessary(
            starting_indexes.outputindex,
            &indexer_vecs.outputindex_to_value,
            exit,
        )?;

        self.txindex_to_output_count.compute_if_necessary(
            starting_indexes.txindex,
            &indexer_vecs.txindex_to_txid,
            exit,
        )?;

        self.outputindex_to_txindex.compute_inverse_less_to_more(
            starting_indexes.txindex,
            &indexer_vecs.txindex_to_first_outputindex,
            &self.txindex_to_output_count,
            exit,
        )?;

        self.p2pk33index_to_p2pk33index.compute_if_necessary(
            starting_indexes.p2pk33index,
            &indexer_vecs.p2pk33index_to_p2pk33bytes,
            exit,
        )?;

        self.p2pk65index_to_p2pk65index.compute_if_necessary(
            starting_indexes.p2pk65index,
            &indexer_vecs.p2pk65index_to_p2pk65bytes,
            exit,
        )?;

        self.p2pkhindex_to_p2pkhindex.compute_if_necessary(
            starting_indexes.p2pkhindex,
            &indexer_vecs.p2pkhindex_to_p2pkhbytes,
            exit,
        )?;

        self.p2shindex_to_p2shindex.compute_if_necessary(
            starting_indexes.p2shindex,
            &indexer_vecs.p2shindex_to_p2shbytes,
            exit,
        )?;

        self.p2trindex_to_p2trindex.compute_if_necessary(
            starting_indexes.p2trindex,
            &indexer_vecs.p2trindex_to_p2trbytes,
            exit,
        )?;

        self.p2wpkhindex_to_p2wpkhindex.compute_if_necessary(
            starting_indexes.p2wpkhindex,
            &indexer_vecs.p2wpkhindex_to_p2wpkhbytes,
            exit,
        )?;

        self.p2wshindex_to_p2wshindex.compute_if_necessary(
            starting_indexes.p2wshindex,
            &indexer_vecs.p2wshindex_to_p2wshbytes,
            exit,
        )?;

        self.emptyoutputindex_to_emptyoutputindex
            .compute_if_necessary(
                starting_indexes.emptyoutputindex,
                &indexer_vecs.emptyoutputindex_to_txindex,
                exit,
            )?;

        self.p2msindex_to_p2msindex.compute_if_necessary(
            starting_indexes.p2msindex,
            &indexer_vecs.p2msindex_to_txindex,
            exit,
        )?;

        self.opreturnindex_to_opreturnindex.compute_if_necessary(
            starting_indexes.opreturnindex,
            &indexer_vecs.opreturnindex_to_txindex,
            exit,
        )?;

        self.p2aindex_to_p2aindex.compute_if_necessary(
            starting_indexes.p2aindex,
            &indexer_vecs.p2aindex_to_p2abytes,
            exit,
        )?;

        self.unknownoutputindex_to_unknownoutputindex
            .compute_if_necessary(
                starting_indexes.unknownoutputindex,
                &indexer_vecs.unknownoutputindex_to_txindex,
                exit,
            )?;

        // ---
        // InputIndex
        // ---

        self.inputindex_to_inputindex.compute_if_necessary(
            starting_indexes.inputindex,
            &indexer_vecs.inputindex_to_outputindex,
            exit,
        )?;

        // ---
        // TxIndex
        // ---

        self.txindex_to_txindex.compute_if_necessary(
            starting_indexes.txindex,
            &indexer_vecs.txindex_to_txid,
            exit,
        )?;

        self.height_to_txindex_count.compute_count_from_indexes(
            starting_indexes.height,
            &indexer_vecs.height_to_first_txindex,
            &indexer_vecs.txindex_to_txid,
            exit,
        )?;

        self.txindex_to_height.compute_inverse_less_to_more(
            starting_indexes.height,
            &indexer_vecs.height_to_first_txindex,
            &self.height_to_txindex_count,
            exit,
        )?;

        // ---
        // Height
        // ---

        self.height_to_height.compute_if_necessary(
            starting_indexes.height,
            &indexer_vecs.height_to_weight,
            exit,
        )?;

        self.height_to_date.compute_transform(
            starting_indexes.height,
            &indexer_vecs.height_to_timestamp,
            |(h, t, ..)| (h, Date::from(t)),
            exit,
        )?;

        let mut prev_timestamp_fixed = None;
        self.height_to_timestamp_fixed.compute_transform(
            starting_indexes.height,
            &indexer_vecs.height_to_timestamp,
            |(h, timestamp, height_to_timestamp_fixed_iter)| {
                if prev_timestamp_fixed.is_none() {
                    if let Some(prev_h) = h.decremented() {
                        prev_timestamp_fixed.replace(
                            height_to_timestamp_fixed_iter
                                .into_iter()
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

        self.dateindex_to_dateindex.compute_if_necessary(
            starting_dateindex,
            &self.dateindex_to_first_height,
            exit,
        )?;

        self.dateindex_to_date.compute_if_necessary(
            starting_dateindex,
            &self.dateindex_to_first_height,
            exit,
        )?;

        self.dateindex_to_height_count.compute_count_from_indexes(
            starting_dateindex,
            &self.dateindex_to_first_height,
            &indexer_vecs.height_to_weight,
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

        self.weekindex_to_weekindex.compute_if_necessary(
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

        self.height_to_difficultyepoch.compute_if_necessary(
            starting_indexes.height,
            &indexer_vecs.height_to_weight,
            exit,
        )?;

        self.difficultyepoch_to_first_height
            .compute_inverse_more_to_less(
                starting_indexes.height,
                &self.height_to_difficultyepoch,
                exit,
            )?;

        self.difficultyepoch_to_difficultyepoch
            .compute_if_necessary(
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

        self.monthindex_to_monthindex.compute_if_necessary(
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

        self.monthindex_to_quarterindex.compute_if_necessary(
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

        self.quarterindex_to_quarterindex.compute_if_necessary(
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
        // YearIndex
        // ---

        let starting_yearindex = self
            .monthindex_to_yearindex
            .into_iter()
            .get_inner(starting_monthindex)
            .unwrap_or_default();

        self.monthindex_to_yearindex.compute_if_necessary(
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

        self.yearindex_to_yearindex.compute_if_necessary(
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

        self.height_to_halvingepoch.compute_if_necessary(
            starting_indexes.height,
            &indexer_vecs.height_to_weight,
            exit,
        )?;

        self.halvingepoch_to_first_height
            .compute_inverse_more_to_less(
                starting_indexes.height,
                &self.height_to_halvingepoch,
                exit,
            )?;

        self.halvingepoch_to_halvingepoch.compute_if_necessary(
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

        self.yearindex_to_decadeindex.compute_if_necessary(
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

        self.decadeindex_to_decadeindex.compute_if_necessary(
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
            yearindex: starting_yearindex,
            decadeindex: starting_decadeindex,
            difficultyepoch: starting_difficultyepoch,
            halvingepoch: starting_halvingepoch,
        })
    }

    pub fn vecs(&self) -> Vec<&dyn AnyCollectableVec> {
        vec![
            &self.dateindex_to_date,
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
            &self.monthindex_to_yearindex,
            &self.opreturnindex_to_opreturnindex,
            &self.outputindex_to_outputindex,
            &self.p2aindex_to_p2aindex,
            &self.p2msindex_to_p2msindex,
            &self.p2pk33index_to_p2pk33index,
            &self.p2pk65index_to_p2pk65index,
            &self.p2pkhindex_to_p2pkhindex,
            &self.p2shindex_to_p2shindex,
            &self.p2trindex_to_p2trindex,
            &self.p2wpkhindex_to_p2wpkhindex,
            &self.p2wshindex_to_p2wshindex,
            &self.quarterindex_to_first_monthindex,
            &self.quarterindex_to_monthindex_count,
            &self.quarterindex_to_quarterindex,
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

impl Indexes {
    pub fn update_from_height(&mut self, height: Height, indexes: &indexes::Vecs) {
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
