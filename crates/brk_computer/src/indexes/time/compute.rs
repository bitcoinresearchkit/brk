use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::{DateIndex, MonthIndex, WeekIndex};
use vecdb::{Exit, TypedVecIterator};

use super::{super::block, vecs::StartingTimeIndexes, Vecs};

impl Vecs {
    pub fn compute(
        &mut self,
        indexer: &Indexer,
        starting_indexes: &brk_indexer::Indexes,
        starting_dateindex: DateIndex,
        block_vecs: &block::Vecs,
        exit: &Exit,
    ) -> Result<StartingTimeIndexes> {
        self.dateindex_to_first_height.compute_coarser(
            starting_indexes.height,
            &block_vecs.height_to_dateindex,
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
            &indexer.vecs.block.height_to_weight,
            exit,
        )?;

        // Week
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

        self.weekindex_to_dateindex_count.compute_count_from_indexes(
            starting_weekindex,
            &self.weekindex_to_first_dateindex,
            &self.dateindex_to_date,
            exit,
        )?;

        // Month
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

        self.monthindex_to_dateindex_count.compute_count_from_indexes(
            starting_monthindex,
            &self.monthindex_to_first_dateindex,
            &self.dateindex_to_date,
            exit,
        )?;

        // Quarter
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

        self.quarterindex_to_quarterindex.compute_from_index(
            starting_quarterindex,
            &self.quarterindex_to_first_monthindex,
            exit,
        )?;

        self.quarterindex_to_monthindex_count.compute_count_from_indexes(
            starting_quarterindex,
            &self.quarterindex_to_first_monthindex,
            &self.monthindex_to_monthindex,
            exit,
        )?;

        // Semester
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

        self.semesterindex_to_semesterindex.compute_from_index(
            starting_semesterindex,
            &self.semesterindex_to_first_monthindex,
            exit,
        )?;

        self.semesterindex_to_monthindex_count.compute_count_from_indexes(
            starting_semesterindex,
            &self.semesterindex_to_first_monthindex,
            &self.monthindex_to_monthindex,
            exit,
        )?;

        // Year
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

        self.yearindex_to_monthindex_count.compute_count_from_indexes(
            starting_yearindex,
            &self.yearindex_to_first_monthindex,
            &self.monthindex_to_monthindex,
            exit,
        )?;

        // Decade
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

        self.decadeindex_to_yearindex_count.compute_count_from_indexes(
            starting_decadeindex,
            &self.decadeindex_to_first_yearindex,
            &self.yearindex_to_yearindex,
            exit,
        )?;

        Ok(StartingTimeIndexes {
            dateindex: starting_dateindex,
            weekindex: starting_weekindex,
            monthindex: starting_monthindex,
            quarterindex: starting_quarterindex,
            semesterindex: starting_semesterindex,
            yearindex: starting_yearindex,
            decadeindex: starting_decadeindex,
        })
    }
}
