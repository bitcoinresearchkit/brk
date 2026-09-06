use brk_types::Version;
use vecdb::{AnyStoredVec, Database, EagerVec, ImportableVec, PcoVec, WritableVec};

use super::*;

#[test]
fn persisted_epoch_windows_keep_the_first_retarget_ratio() {
    let directory = tempfile::tempdir().unwrap();
    let database = Database::open(directory.path()).unwrap();
    let mut heights: EagerVec<PcoVec<Epoch, Height>> =
        EagerVec::forced_import(&database, "heights", Version::ONE).unwrap();
    let mut timestamps: EagerVec<PcoVec<Epoch, Timestamp>> =
        EagerVec::forced_import(&database, "timestamps", Version::ONE).unwrap();
    let mut difficulties: EagerVec<PcoVec<Epoch, StoredF64>> =
        EagerVec::forced_import(&database, "difficulty", Version::ONE).unwrap();
    for (height, time, difficulty) in [(0u32, 100u32, 1.0), (2016, 200, 2.0), (4032, 300, 1.0)] {
        heights.push(height.into());
        timestamps.push(time.into());
        difficulties.push(difficulty.into());
    }
    heights.write().unwrap();
    timestamps.write().unwrap();
    difficulties.write().unwrap();
    for (start, first_epoch, expected) in [
        (0, 0, vec![(0u32, 0.0), (2016, 2.0), (4032, 0.5)]),
        (2016, 1, vec![(2016, 2.0), (4032, 0.5)]),
        (2017, 1, vec![(4032, 0.5)]),
        (4032, 2, vec![(4032, 0.5)]),
    ] {
        let rows =
            read_epoch_window(start, first_epoch, 2, &heights, &timestamps, &difficulties).unwrap();
        assert_eq!(
            rows.iter()
                .map(|row| (u32::from(row.height), row.change_percent))
                .collect::<Vec<_>>(),
            expected
        );
    }
    assert!(read_epoch_window(4032, 2, 1, &heights, &timestamps, &difficulties).is_err());
    let missing: EagerVec<PcoVec<Epoch, StoredF64>> =
        EagerVec::forced_import(&database, "missing", Version::ONE).unwrap();
    assert!(read_epoch_window(2016, 1, 2, &heights, &timestamps, &missing).is_err());
}
