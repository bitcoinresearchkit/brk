use std::{fs, ops::Deref, path::Path};

use brk_core::{
    Date, Dateindex, Decadeindex, Difficultyepoch, Halvingepoch, Height, Monthindex, Quarterindex,
    Timestamp, Txindex, Txinindex, Txoutindex, Weekindex, Yearindex,
};
use brk_exit::Exit;
use brk_indexer::Indexer;
use brk_vec::{Compressed, Version};

use super::ComputedVec;

#[derive(Clone)]
pub struct Vecs {
    pub dateindex_to_date: ComputedVec<Dateindex, Date>,
    pub dateindex_to_dateindex: ComputedVec<Dateindex, Dateindex>,
    pub dateindex_to_first_height: ComputedVec<Dateindex, Height>,
    pub dateindex_to_last_height: ComputedVec<Dateindex, Height>,
    pub dateindex_to_monthindex: ComputedVec<Dateindex, Monthindex>,
    pub dateindex_to_timestamp: ComputedVec<Dateindex, Timestamp>,
    pub dateindex_to_weekindex: ComputedVec<Dateindex, Weekindex>,
    pub decadeindex_to_decadeindex: ComputedVec<Decadeindex, Decadeindex>,
    pub decadeindex_to_first_yearindex: ComputedVec<Decadeindex, Yearindex>,
    pub decadeindex_to_last_yearindex: ComputedVec<Decadeindex, Yearindex>,
    pub decadeindex_to_timestamp: ComputedVec<Decadeindex, Timestamp>,
    pub difficultyepoch_to_difficultyepoch: ComputedVec<Difficultyepoch, Difficultyepoch>,
    pub difficultyepoch_to_first_height: ComputedVec<Difficultyepoch, Height>,
    pub difficultyepoch_to_last_height: ComputedVec<Difficultyepoch, Height>,
    pub difficultyepoch_to_timestamp: ComputedVec<Difficultyepoch, Timestamp>,
    pub halvingepoch_to_first_height: ComputedVec<Halvingepoch, Height>,
    pub halvingepoch_to_halvingepoch: ComputedVec<Halvingepoch, Halvingepoch>,
    pub halvingepoch_to_last_height: ComputedVec<Halvingepoch, Height>,
    pub halvingepoch_to_timestamp: ComputedVec<Halvingepoch, Timestamp>,
    pub height_to_dateindex: ComputedVec<Height, Dateindex>,
    pub height_to_difficultyepoch: ComputedVec<Height, Difficultyepoch>,
    pub height_to_fixed_date: ComputedVec<Height, Date>,
    pub height_to_fixed_timestamp: ComputedVec<Height, Timestamp>,
    pub height_to_halvingepoch: ComputedVec<Height, Halvingepoch>,
    pub height_to_height: ComputedVec<Height, Height>,
    pub height_to_last_txindex: ComputedVec<Height, Txindex>,
    pub height_to_real_date: ComputedVec<Height, Date>,
    pub monthindex_to_first_dateindex: ComputedVec<Monthindex, Dateindex>,
    pub monthindex_to_last_dateindex: ComputedVec<Monthindex, Dateindex>,
    pub monthindex_to_monthindex: ComputedVec<Monthindex, Monthindex>,
    pub monthindex_to_quarterindex: ComputedVec<Monthindex, Quarterindex>,
    pub monthindex_to_timestamp: ComputedVec<Monthindex, Timestamp>,
    pub monthindex_to_yearindex: ComputedVec<Monthindex, Yearindex>,
    pub quarterindex_to_first_monthindex: ComputedVec<Quarterindex, Monthindex>,
    pub quarterindex_to_last_monthindex: ComputedVec<Quarterindex, Monthindex>,
    pub quarterindex_to_quarterindex: ComputedVec<Quarterindex, Quarterindex>,
    pub quarterindex_to_timestamp: ComputedVec<Quarterindex, Timestamp>,
    pub txindex_to_last_txinindex: ComputedVec<Txindex, Txinindex>,
    pub txindex_to_last_txoutindex: ComputedVec<Txindex, Txoutindex>,
    pub weekindex_to_first_dateindex: ComputedVec<Weekindex, Dateindex>,
    pub weekindex_to_last_dateindex: ComputedVec<Weekindex, Dateindex>,
    pub weekindex_to_timestamp: ComputedVec<Weekindex, Timestamp>,
    pub weekindex_to_weekindex: ComputedVec<Weekindex, Weekindex>,
    pub yearindex_to_decadeindex: ComputedVec<Yearindex, Decadeindex>,
    pub yearindex_to_first_monthindex: ComputedVec<Yearindex, Monthindex>,
    pub yearindex_to_last_monthindex: ComputedVec<Yearindex, Monthindex>,
    pub yearindex_to_timestamp: ComputedVec<Yearindex, Timestamp>,
    pub yearindex_to_yearindex: ComputedVec<Yearindex, Yearindex>,
}

impl Vecs {
    pub fn forced_import(path: &Path, compressed: Compressed) -> color_eyre::Result<Self> {
        fs::create_dir_all(path)?;

        Ok(Self {
            dateindex_to_date: ComputedVec::forced_import(
                &path.join("dateindex_to_date"),
                Version::ZERO,
                compressed,
            )?,
            dateindex_to_dateindex: ComputedVec::forced_import(
                &path.join("dateindex_to_dateindex"),
                Version::ZERO,
                compressed,
            )?,
            dateindex_to_first_height: ComputedVec::forced_import(
                &path.join("dateindex_to_first_height"),
                Version::ZERO,
                compressed,
            )?,
            dateindex_to_last_height: ComputedVec::forced_import(
                &path.join("dateindex_to_last_height"),
                Version::ZERO,
                compressed,
            )?,
            height_to_real_date: ComputedVec::forced_import(
                &path.join("height_to_real_date"),
                Version::ZERO,
                compressed,
            )?,
            height_to_fixed_date: ComputedVec::forced_import(
                &path.join("height_to_fixed_date"),
                Version::ZERO,
                compressed,
            )?,
            height_to_dateindex: ComputedVec::forced_import(
                &path.join("height_to_dateindex"),
                Version::ZERO,
                compressed,
            )?,
            height_to_height: ComputedVec::forced_import(
                &path.join("height_to_height"),
                Version::ZERO,
                compressed,
            )?,
            height_to_last_txindex: ComputedVec::forced_import(
                &path.join("height_to_last_txindex"),
                Version::ZERO,
                compressed,
            )?,
            txindex_to_last_txinindex: ComputedVec::forced_import(
                &path.join("txindex_to_last_txinindex"),
                Version::ZERO,
                compressed,
            )?,
            txindex_to_last_txoutindex: ComputedVec::forced_import(
                &path.join("txindex_to_last_txoutindex"),
                Version::ZERO,
                compressed,
            )?,
            difficultyepoch_to_first_height: ComputedVec::forced_import(
                &path.join("difficultyepoch_to_first_height"),
                Version::ZERO,
                compressed,
            )?,
            difficultyepoch_to_last_height: ComputedVec::forced_import(
                &path.join("difficultyepoch_to_last_height"),
                Version::ZERO,
                compressed,
            )?,
            halvingepoch_to_first_height: ComputedVec::forced_import(
                &path.join("halvingepoch_to_first_height"),
                Version::ZERO,
                compressed,
            )?,
            halvingepoch_to_last_height: ComputedVec::forced_import(
                &path.join("halvingepoch_to_last_height"),
                Version::ZERO,
                compressed,
            )?,
            weekindex_to_first_dateindex: ComputedVec::forced_import(
                &path.join("weekindex_to_first_dateindex"),
                Version::ZERO,
                compressed,
            )?,
            weekindex_to_last_dateindex: ComputedVec::forced_import(
                &path.join("weekindex_to_last_dateindex"),
                Version::ZERO,
                compressed,
            )?,
            monthindex_to_first_dateindex: ComputedVec::forced_import(
                &path.join("monthindex_to_first_dateindex"),
                Version::ZERO,
                compressed,
            )?,
            monthindex_to_last_dateindex: ComputedVec::forced_import(
                &path.join("monthindex_to_last_dateindex"),
                Version::ZERO,
                compressed,
            )?,
            yearindex_to_first_monthindex: ComputedVec::forced_import(
                &path.join("yearindex_to_first_monthindex"),
                Version::ZERO,
                compressed,
            )?,
            yearindex_to_last_monthindex: ComputedVec::forced_import(
                &path.join("yearindex_to_last_monthindex"),
                Version::ZERO,
                compressed,
            )?,
            decadeindex_to_first_yearindex: ComputedVec::forced_import(
                &path.join("decadeindex_to_first_yearindex"),
                Version::ZERO,
                compressed,
            )?,
            decadeindex_to_last_yearindex: ComputedVec::forced_import(
                &path.join("decadeindex_to_last_yearindex"),
                Version::ZERO,
                compressed,
            )?,
            dateindex_to_weekindex: ComputedVec::forced_import(
                &path.join("dateindex_to_weekindex"),
                Version::ZERO,
                compressed,
            )?,
            dateindex_to_monthindex: ComputedVec::forced_import(
                &path.join("dateindex_to_monthindex"),
                Version::ZERO,
                compressed,
            )?,
            monthindex_to_yearindex: ComputedVec::forced_import(
                &path.join("monthindex_to_yearindex"),
                Version::ZERO,
                compressed,
            )?,
            yearindex_to_decadeindex: ComputedVec::forced_import(
                &path.join("yearindex_to_decadeindex"),
                Version::ZERO,
                compressed,
            )?,
            height_to_difficultyepoch: ComputedVec::forced_import(
                &path.join("height_to_difficultyepoch"),
                Version::ZERO,
                compressed,
            )?,
            height_to_halvingepoch: ComputedVec::forced_import(
                &path.join("height_to_halvingepoch"),
                Version::ZERO,
                compressed,
            )?,
            weekindex_to_weekindex: ComputedVec::forced_import(
                &path.join("weekindex_to_weekindex"),
                Version::ZERO,
                compressed,
            )?,
            monthindex_to_monthindex: ComputedVec::forced_import(
                &path.join("monthindex_to_monthindex"),
                Version::ZERO,
                compressed,
            )?,
            yearindex_to_yearindex: ComputedVec::forced_import(
                &path.join("yearindex_to_yearindex"),
                Version::ZERO,
                compressed,
            )?,
            decadeindex_to_decadeindex: ComputedVec::forced_import(
                &path.join("decadeindex_to_decadeindex"),
                Version::ZERO,
                compressed,
            )?,
            difficultyepoch_to_difficultyepoch: ComputedVec::forced_import(
                &path.join("difficultyepoch_to_difficultyepoch"),
                Version::ZERO,
                compressed,
            )?,
            halvingepoch_to_halvingepoch: ComputedVec::forced_import(
                &path.join("halvingepoch_to_halvingepoch"),
                Version::ZERO,
                compressed,
            )?,
            dateindex_to_timestamp: ComputedVec::forced_import(
                &path.join("dateindex_to_timestamp"),
                Version::ZERO,
                compressed,
            )?,
            decadeindex_to_timestamp: ComputedVec::forced_import(
                &path.join("decadeindex_to_timestamp"),
                Version::ZERO,
                compressed,
            )?,
            difficultyepoch_to_timestamp: ComputedVec::forced_import(
                &path.join("difficultyepoch_to_timestamp"),
                Version::ZERO,
                compressed,
            )?,
            halvingepoch_to_timestamp: ComputedVec::forced_import(
                &path.join("halvingepoch_to_timestamp"),
                Version::ZERO,
                compressed,
            )?,
            monthindex_to_timestamp: ComputedVec::forced_import(
                &path.join("monthindex_to_timestamp"),
                Version::ZERO,
                compressed,
            )?,
            weekindex_to_timestamp: ComputedVec::forced_import(
                &path.join("weekindex_to_timestamp"),
                Version::ZERO,
                compressed,
            )?,
            yearindex_to_timestamp: ComputedVec::forced_import(
                &path.join("yearindex_to_timestamp"),
                Version::ZERO,
                compressed,
            )?,
            height_to_fixed_timestamp: ComputedVec::forced_import(
                &path.join("height_to_fixed_timestamp"),
                Version::ZERO,
                compressed,
            )?,
            monthindex_to_quarterindex: ComputedVec::forced_import(
                &path.join("monthindex_to_quarterindex"),
                Version::ZERO,
                compressed,
            )?,
            quarterindex_to_first_monthindex: ComputedVec::forced_import(
                &path.join("quarterindex_to_first_monthindex"),
                Version::ZERO,
                compressed,
            )?,
            quarterindex_to_last_monthindex: ComputedVec::forced_import(
                &path.join("quarterindex_to_last_monthindex"),
                Version::ZERO,
                compressed,
            )?,
            quarterindex_to_quarterindex: ComputedVec::forced_import(
                &path.join("quarterindex_to_quarterindex"),
                Version::ZERO,
                compressed,
            )?,
            quarterindex_to_timestamp: ComputedVec::forced_import(
                &path.join("quarterindex_to_timestamp"),
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
        let txinindexes_count = indexer_vecs.txinindex_to_txoutindex.len();
        let txoutindexes_count = indexer_vecs.txoutindex_to_addressindex.len();

        self.height_to_height.compute_transform(
            starting_indexes.height,
            indexer_vecs.height_to_timestamp.mut_vec(),
            |(h, ..)| (h, h),
            exit,
        )?;

        self.height_to_real_date.compute_transform(
            starting_indexes.height,
            indexer_vecs.height_to_timestamp.mut_vec(),
            |(h, t, ..)| (h, Date::from(t)),
            exit,
        )?;

        self.height_to_fixed_timestamp.compute_transform(
            starting_indexes.height,
            indexer_vecs.height_to_timestamp.mut_vec(),
            |(h, d, s, ..)| {
                let d = h
                    .decremented()
                    .and_then(|h| s.unwrap_cached_get(h))
                    .map_or(d, |prev_d| prev_d.max(d));
                (h, d)
            },
            exit,
        )?;

        self.height_to_fixed_date.compute_transform(
            starting_indexes.height,
            self.height_to_fixed_timestamp.mut_vec(),
            |(h, t, ..)| (h, Date::from(t)),
            exit,
        )?;

        let decremented_starting_height = starting_indexes.height.decremented().unwrap_or_default();

        let starting_dateindex = self
            .height_to_dateindex
            .unwrap_cached_get(decremented_starting_height)
            .unwrap_or_default();

        self.height_to_dateindex.compute_transform(
            starting_indexes.height,
            self.height_to_fixed_date.mut_vec(),
            |(h, d, ..)| (h, Dateindex::try_from(d).unwrap()),
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

        self.dateindex_to_dateindex.compute_transform(
            starting_dateindex,
            self.dateindex_to_first_height.mut_vec(),
            |(di, ..)| (di, di),
            exit,
        )?;

        self.dateindex_to_date.compute_transform(
            starting_dateindex,
            self.dateindex_to_dateindex.mut_vec(),
            |(di, ..)| (di, Date::from(di)),
            exit,
        )?;

        self.dateindex_to_timestamp.compute_transform(
            starting_dateindex,
            self.dateindex_to_date.mut_vec(),
            |(di, d, ..)| (di, Timestamp::from(d)),
            exit,
        )?;

        self.txindex_to_last_txinindex
            .compute_last_index_from_first(
                starting_indexes.txindex,
                indexer_vecs.txindex_to_first_txinindex.mut_vec(),
                txinindexes_count,
                exit,
            )?;

        self.txindex_to_last_txoutindex
            .compute_last_index_from_first(
                starting_indexes.txindex,
                indexer_vecs.txindex_to_first_txoutindex.mut_vec(),
                txoutindexes_count,
                exit,
            )?;

        self.height_to_last_txindex.compute_last_index_from_first(
            starting_indexes.height,
            indexer_vecs.height_to_first_txindex.mut_vec(),
            txindexes_count,
            exit,
        )?;

        // ---

        let starting_weekindex = self
            .dateindex_to_weekindex
            .unwrap_cached_get(starting_dateindex)
            .unwrap_or_default();

        self.dateindex_to_weekindex.compute_transform(
            starting_dateindex,
            self.dateindex_to_dateindex.mut_vec(),
            |(di, ..)| (di, Weekindex::from(di)),
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

        self.weekindex_to_weekindex.compute_transform(
            starting_weekindex,
            self.weekindex_to_first_dateindex.mut_vec(),
            |(wi, ..)| (wi, wi),
            exit,
        )?;

        self.weekindex_to_timestamp.compute_transform(
            starting_weekindex,
            self.weekindex_to_first_dateindex.mut_vec(),
            |(i, d, ..)| (i, self.dateindex_to_timestamp.double_unwrap_cached_get(d)),
            exit,
        )?;

        // ---

        let starting_monthindex = self
            .dateindex_to_monthindex
            .unwrap_cached_get(starting_dateindex)
            .unwrap_or_default();

        self.dateindex_to_monthindex.compute_transform(
            starting_dateindex,
            self.dateindex_to_dateindex.mut_vec(),
            |(di, ..)| (di, Monthindex::from(di)),
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

        self.monthindex_to_monthindex.compute_transform(
            starting_monthindex,
            self.monthindex_to_first_dateindex.mut_vec(),
            |(mi, ..)| (mi, mi),
            exit,
        )?;

        self.monthindex_to_timestamp.compute_transform(
            starting_monthindex,
            self.monthindex_to_first_dateindex.mut_vec(),
            |(i, d, ..)| (i, self.dateindex_to_timestamp.double_unwrap_cached_get(d)),
            exit,
        )?;

        // ---

        let starting_quarterindex = self
            .monthindex_to_quarterindex
            .unwrap_cached_get(starting_monthindex)
            .unwrap_or_default();

        self.monthindex_to_quarterindex.compute_transform(
            starting_monthindex,
            self.monthindex_to_monthindex.mut_vec(),
            |(mi, ..)| (mi, Quarterindex::from(mi)),
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

        self.quarterindex_to_quarterindex.compute_transform(
            starting_quarterindex,
            self.quarterindex_to_first_monthindex.mut_vec(),
            |(yi, ..)| (yi, yi),
            exit,
        )?;

        self.quarterindex_to_timestamp.compute_transform(
            starting_quarterindex,
            self.quarterindex_to_first_monthindex.mut_vec(),
            |(i, m, ..)| (i, self.monthindex_to_timestamp.double_unwrap_cached_get(m)),
            exit,
        )?;

        // ---

        let starting_yearindex = self
            .monthindex_to_yearindex
            .unwrap_cached_get(starting_monthindex)
            .unwrap_or_default();

        self.monthindex_to_yearindex.compute_transform(
            starting_monthindex,
            self.monthindex_to_monthindex.mut_vec(),
            |(mi, ..)| (mi, Yearindex::from(mi)),
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

        self.yearindex_to_yearindex.compute_transform(
            starting_yearindex,
            self.yearindex_to_first_monthindex.mut_vec(),
            |(yi, ..)| (yi, yi),
            exit,
        )?;

        self.yearindex_to_timestamp.compute_transform(
            starting_yearindex,
            self.yearindex_to_first_monthindex.mut_vec(),
            |(i, m, ..)| (i, self.monthindex_to_timestamp.double_unwrap_cached_get(m)),
            exit,
        )?;

        // ---

        let starting_decadeindex = self
            .yearindex_to_decadeindex
            .unwrap_cached_get(starting_yearindex)
            .unwrap_or_default();

        self.yearindex_to_decadeindex.compute_transform(
            starting_yearindex,
            self.yearindex_to_yearindex.mut_vec(),
            |(yi, ..)| (yi, Decadeindex::from(yi)),
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

        self.decadeindex_to_decadeindex.compute_transform(
            starting_decadeindex,
            self.decadeindex_to_first_yearindex.mut_vec(),
            |(di, ..)| (di, di),
            exit,
        )?;

        self.decadeindex_to_timestamp.compute_transform(
            starting_decadeindex,
            self.decadeindex_to_first_yearindex.mut_vec(),
            |(i, y, ..)| (i, self.yearindex_to_timestamp.double_unwrap_cached_get(y)),
            exit,
        )?;

        // ---

        let starting_difficultyepoch = self
            .height_to_difficultyepoch
            .unwrap_cached_get(decremented_starting_height)
            .unwrap_or_default();

        self.height_to_difficultyepoch.compute_transform(
            starting_indexes.height,
            self.height_to_height.mut_vec(),
            |(h, ..)| (h, Difficultyepoch::from(h)),
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

        self.difficultyepoch_to_difficultyepoch.compute_transform(
            starting_difficultyepoch,
            self.difficultyepoch_to_first_height.mut_vec(),
            |(de, ..)| (de, de),
            exit,
        )?;

        self.difficultyepoch_to_timestamp.compute_transform(
            starting_difficultyepoch,
            self.difficultyepoch_to_first_height.mut_vec(),
            |(i, h, ..)| {
                (
                    i,
                    indexer_vecs.height_to_timestamp.double_unwrap_cached_get(h),
                )
            },
            exit,
        )?;

        // ---

        let starting_halvingepoch = self
            .height_to_halvingepoch
            .unwrap_cached_get(decremented_starting_height)
            .unwrap_or_default();

        self.height_to_halvingepoch.compute_transform(
            starting_indexes.height,
            self.height_to_height.mut_vec(),
            |(h, ..)| (h, Halvingepoch::from(h)),
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

        self.halvingepoch_to_halvingepoch.compute_transform(
            starting_halvingepoch,
            self.halvingepoch_to_first_height.mut_vec(),
            |(he, ..)| (he, he),
            exit,
        )?;

        // self.difficultyepoch_to_timestamp.compute_transform(
        //     starting_difficultyepoch,
        //     self.difficultyepoch_to_first_height.mut_vec(),
        //     |(i, h, ..)| {
        //         (
        //             i,
        //             *indexer_vecs.height_to_timestamp.unwraped_cached_get(h).unwrap().unwrap(),
        //         )
        //     },
        //     exit,
        // )?;

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
            self.height_to_fixed_date.any_vec(),
            self.height_to_height.any_vec(),
            self.height_to_last_txindex.any_vec(),
            self.height_to_real_date.any_vec(),
            self.txindex_to_last_txinindex.any_vec(),
            self.txindex_to_last_txoutindex.any_vec(),
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
            self.dateindex_to_timestamp.any_vec(),
            self.decadeindex_to_timestamp.any_vec(),
            self.difficultyepoch_to_timestamp.any_vec(),
            self.halvingepoch_to_timestamp.any_vec(),
            self.monthindex_to_timestamp.any_vec(),
            self.weekindex_to_timestamp.any_vec(),
            self.yearindex_to_timestamp.any_vec(),
            self.height_to_fixed_timestamp.any_vec(),
            self.monthindex_to_quarterindex.any_vec(),
            self.quarterindex_to_first_monthindex.any_vec(),
            self.quarterindex_to_last_monthindex.any_vec(),
            self.quarterindex_to_quarterindex.any_vec(),
            self.quarterindex_to_timestamp.any_vec(),
        ]
    }
}

pub struct Indexes {
    indexes: brk_indexer::Indexes,
    pub dateindex: Dateindex,
    pub weekindex: Weekindex,
    pub monthindex: Monthindex,
    pub quarterindex: Quarterindex,
    pub yearindex: Yearindex,
    pub decadeindex: Decadeindex,
    pub difficultyepoch: Difficultyepoch,
    pub halvingepoch: Halvingepoch,
}

impl Deref for Indexes {
    type Target = brk_indexer::Indexes;
    fn deref(&self) -> &Self::Target {
        &self.indexes
    }
}
