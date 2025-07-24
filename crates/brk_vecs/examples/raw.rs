use std::{fs, path::Path, sync::Arc};

use brk_core::{DateIndex, Version};
use brk_vecs::{AnyVec, CollectableVec, File, GenericStoredVec, RawVec, Stamp, VecIterator};

type I = DateIndex;
#[allow(clippy::upper_case_acronyms)]
type VEC = RawVec<I, u32>;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = fs::remove_dir_all("raw");

    let version = Version::TWO;
    // let format = Format::Raw;
    //
    let file = Arc::new(File::open(Path::new("raw"))?);

    {
        let mut vec: VEC = RawVec::forced_import(&file, "vec", version)?;

        (0..21_u32).for_each(|v| {
            vec.push(v);
        });

        let mut iter = vec.into_iter();
        dbg!(iter.get(0.into()));
        dbg!(iter.get(1.into()));
        dbg!(iter.get(2.into()));
        dbg!(iter.get(20.into()));
        dbg!(iter.get(21.into()));
        drop(iter);

        vec.flush()?;

        dbg!(vec.header());
    }

    {
        let mut vec: VEC = RawVec::forced_import(&file, "vec", version)?;

        vec.mut_header().update_stamp(Stamp::new(100));

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
        drop(iter);

        vec.push(21);
        vec.push(22);

        let mut iter = vec.into_iter();
        dbg!(iter.get(20.into()));
        dbg!(iter.get(21.into()));
        dbg!(iter.get(22.into()));
        dbg!(iter.get(23.into()));
        drop(iter);

        vec.flush()?;
    }

    {
        let mut vec: VEC = RawVec::forced_import(&file, "vec", version)?;

        let mut iter = vec.into_iter();
        dbg!(iter.get(0.into()));
        dbg!(iter.get(20.into()));
        dbg!(iter.get(21.into()));
        dbg!(iter.get(22.into()));
        drop(iter);

        vec.truncate_if_needed(14.into())?;

        let mut iter = vec.into_iter();
        dbg!(iter.get(0.into()));
        dbg!(iter.get(5.into()));
        dbg!(iter.get(20.into()));
        drop(iter);

        dbg!(vec.collect_signed_range(Some(-5), None)?);

        vec.push(vec.len() as u32);
        dbg!(VecIterator::last(vec.into_iter()));

        dbg!(vec.into_iter().collect::<Vec<_>>());
    }

    {
        let mut vec: VEC = RawVec::forced_import(&file, "vec", version)?;

        vec.reset()?;

        dbg!(vec.header(), vec.pushed_len(), vec.stored_len(), vec.len());

        (0..21_u32).for_each(|v| {
            vec.push(v);
        });

        let mut iter = vec.into_iter();
        dbg!(iter.get(0.into()));
        dbg!(iter.get(20.into()));
        dbg!(iter.get(21.into()));
        drop(iter);

        let reader = vec.create_static_reader();
        dbg!(vec.take(10.into(), &reader)?);
        dbg!(vec.get_or_read(10.into(), &reader)?);
        dbg!(vec.holes());
        drop(reader);

        vec.flush()?;
        dbg!(vec.holes());
    }

    {
        let mut vec: VEC = RawVec::forced_import(&file, "vec", version)?;

        dbg!(vec.holes());

        let reader = vec.create_static_reader();
        dbg!(vec.get_or_read(10.into(), &reader)?);
        drop(reader);

        vec.update(10.into(), 10)?;
        vec.update(0.into(), 10)?;

        let reader = vec.create_static_reader();
        dbg!(
            vec.holes(),
            vec.get_or_read(0.into(), &reader)?,
            vec.get_or_read(10.into(), &reader)?
        );
        drop(reader);

        vec.flush()?;
    }

    {
        let vec: VEC = RawVec::forced_import(&file, "vec", version)?;

        dbg!(vec.collect()?);
    }

    Ok(())
}
