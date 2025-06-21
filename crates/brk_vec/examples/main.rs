use std::{fs, path::Path};

use brk_core::{DateIndex, Version};
use brk_vec::{AnyVec, CollectableVec, Format, GenericStoredVec, StoredVec, VecIterator};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = fs::remove_dir_all("./vec");

    let version = Version::ZERO;
    let format = Format::Compressed;

    {
        let mut vec: StoredVec<DateIndex, u32> =
            StoredVec::forced_import(Path::new("."), "vec", version, format)?;

        (0..21_u32).for_each(|v| {
            vec.push(v);
        });

        let mut iter = vec.into_iter();
        dbg!(iter.get(0.into()));
        dbg!(iter.get(20.into()));
        dbg!(iter.get(21.into()));

        vec.flush()?;
    }

    {
        let mut vec: StoredVec<DateIndex, u32> =
            StoredVec::forced_import(Path::new("."), "vec", version, format)?;
        let mut iter = vec.into_iter();

        dbg!(iter.get(0.into()));
        dbg!(iter.get(0.into()));
        dbg!(iter.get(1.into()));
        dbg!(iter.get(2.into()));
        dbg!(iter.get(20.into()));
        dbg!(iter.get(20.into()));
        dbg!(iter.get(0.into()));

        vec.push(21);
        vec.push(22);

        let mut iter = vec.into_iter();

        dbg!(iter.get(20.into()));
        dbg!(iter.get(21.into()));
        dbg!(iter.get(22.into()));
        dbg!(iter.get(23.into()));

        vec.flush()?;
    }

    {
        let mut vec: StoredVec<DateIndex, u32> =
            StoredVec::forced_import(Path::new("."), "vec", version, format)?;
        let mut iter = vec.into_iter();

        dbg!(iter.get(0.into()));
        dbg!(iter.get(20.into()));
        dbg!(iter.get(21.into()));
        dbg!(iter.get(22.into()));

        vec.truncate_if_needed(14.into())?;

        let mut iter = vec.into_iter();

        iter.get(0.into());
        iter.get(5.into());
        dbg!(iter.get(20.into()));

        dbg!(vec.collect_signed_range(Some(-5), None)?);

        vec.push(vec.len() as u32);
        dbg!(VecIterator::last(vec.into_iter()));

        dbg!(vec.into_iter().collect::<Vec<_>>());
    }

    Ok(())
}
