use std::{fs, path::Path};

use brk_core::{DateIndex, Height, Version};
use brk_vec::{AnyVec, CollectableVec, Format, GenericStoredVec, StoredVec, VecIterator};

type I = DateIndex;
#[allow(clippy::upper_case_acronyms)]
type VEC = StoredVec<I, u32>;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = fs::remove_dir_all("./vec");

    let version = Version::TWO;
    let format = Format::Raw;

    {
        let mut vec: VEC = StoredVec::forced_import(Path::new("."), "vec", version, format)?;

        (0..21_u32).for_each(|v| {
            vec.push(v);
        });

        let mut iter = vec.into_iter();
        dbg!(iter.get(0.into()));
        dbg!(iter.get(1.into()));
        dbg!(iter.get(2.into()));
        dbg!(iter.get(20.into()));
        dbg!(iter.get(21.into()));

        vec.flush()?;

        // dbg!(vec.header());
    }

    {
        let mut vec: VEC = StoredVec::forced_import(Path::new("."), "vec", version, format)?;

        vec.mut_header().update_height(Height::new(100));

        let mut iter = vec.into_iter();
        dbg!(iter.get(0.into()));
        dbg!(iter.get(1.into()));
        dbg!(iter.get(2.into()));
        dbg!(iter.get(3.into()));
        dbg!(iter.get(4.into()));
        dbg!(iter.get(5.into()));
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
        let mut vec: VEC = StoredVec::forced_import(Path::new("."), "vec", version, format)?;
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

    {
        let mut vec: VEC = StoredVec::forced_import(Path::new("."), "vec", version, format)?;

        vec.reset()?;

        dbg!(vec.header(), vec.pushed_len(), vec.stored_len(), vec.len());

        (0..21_u32).for_each(|v| {
            vec.push(v);
        });

        let mut iter = vec.into_iter();
        dbg!(iter.get(0.into()));
        dbg!(iter.get(20.into()));
        dbg!(iter.get(21.into()));

        let mmap = vec.create_mmap()?;
        dbg!(vec.take(10.into(), &mmap)?);
        dbg!(vec.get_or_read(10.into(), &mmap)?);
        dbg!(vec.holes());
        vec.flush()?;
        dbg!(vec.holes());
    }

    {
        let mut vec: VEC = StoredVec::forced_import(Path::new("."), "vec", version, format)?;

        let mmap = vec.create_mmap()?;

        dbg!(vec.holes());
        dbg!(vec.get_or_read(10.into(), &mmap)?);

        vec.update(10.into(), 10)?;
        vec.update(0.into(), 10)?;
        dbg!(
            vec.holes(),
            vec.get_or_read(0.into(), &mmap)?,
            vec.get_or_read(10.into(), &mmap)?
        );

        vec.flush()?;
    }

    {
        let vec: VEC = StoredVec::forced_import(Path::new("."), "vec", version, format)?;

        dbg!(vec.collect()?);
    }

    Ok(())
}
