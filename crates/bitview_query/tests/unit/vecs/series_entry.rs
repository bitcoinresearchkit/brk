use std::mem::size_of;

use super::SeriesEntry;

#[test]
fn index_uses_existing_series_entry_padding() {
    assert_eq!(size_of::<SeriesEntry<'static>>(), 5 * size_of::<usize>());
}
