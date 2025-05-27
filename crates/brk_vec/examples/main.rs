use std::{fs, path::Path};

use brk_core::Version;
use brk_vec::{AnyVec, CollectableVec, Compressed, GenericStoredVec, StoredVec, VecIterator};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = fs::remove_dir_all("./vec");

    let version = Version::ZERO;
    let compressed = Compressed::YES;

    {
        let mut vec: StoredVec<usize, u32> =
            StoredVec::forced_import(Path::new("."), "vec", version, compressed)?;

        (0..21_u32).for_each(|v| {
            vec.push(v);
        });

        let mut iter = vec.into_iter();
        dbg!(iter.get(0));
        dbg!(iter.get(20));
        dbg!(iter.get(21));

        vec.flush()?;
    }

    {
        let mut vec: StoredVec<usize, u32> =
            StoredVec::forced_import(Path::new("."), "vec", version, compressed)?;
        let mut iter = vec.into_iter();

        dbg!(iter.get(0));
        dbg!(iter.get(0));
        dbg!(iter.get(1));
        dbg!(iter.get(2));
        dbg!(iter.get(20));
        dbg!(iter.get(20));
        dbg!(iter.get(0));

        vec.push(21);
        vec.push(22);

        let mut iter = vec.into_iter();

        dbg!(iter.get(20));
        dbg!(iter.get(21));
        dbg!(iter.get(22));
        dbg!(iter.get(23));

        vec.flush()?;
    }

    {
        let mut vec: StoredVec<usize, u32> =
            StoredVec::forced_import(Path::new("."), "vec", version, compressed)?;
        let mut iter = vec.into_iter();

        dbg!(iter.get(0));
        dbg!(iter.get(20));
        dbg!(iter.get(21));
        dbg!(iter.get(22));

        vec.truncate_if_needed(14)?;

        let mut iter = vec.into_iter();

        iter.get(0);
        iter.get(5);
        dbg!(iter.get(20));

        dbg!(vec.collect_signed_range(Some(-5), None)?);

        vec.push(vec.len() as u32);
        dbg!(VecIterator::last(vec.into_iter()));

        dbg!(vec.into_iter().collect::<Vec<_>>());
    }

    Ok(())
}
