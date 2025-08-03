use std::{fs, path::Path, sync::Arc};

use brk_vecs::{
    AnyStoredVec, AnyVec, CollectableVec, CompressedVec, File, GenericStoredVec, Stamp,
    VecIterator, Version,
};

#[allow(clippy::upper_case_acronyms)]
type VEC = CompressedVec<usize, u32>;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = fs::remove_dir_all("compressed");

    let version = Version::TWO;

    let file = Arc::new(File::open(Path::new("compressed"))?);

    {
        let mut vec: VEC = CompressedVec::forced_import(&file, "vec", version)?;

        (0..21_u32).for_each(|v| {
            vec.push(v);
        });

        let mut iter = vec.into_iter();
        dbg!(iter.get(0));
        dbg!(iter.get(1));
        dbg!(iter.get(2));
        dbg!(iter.get(20));
        dbg!(iter.get(21));
        drop(iter);

        dbg!("flush");
        vec.flush()?;
        dbg!("flushed");

        dbg!(vec.header());
    }

    {
        let mut vec: VEC = CompressedVec::forced_import(&file, "vec", version)?;

        vec.mut_header().update_stamp(Stamp::new(100));

        let mut iter = vec.into_iter();
        dbg!(iter.get(0));
        dbg!(iter.get(1));
        dbg!(iter.get(2));
        dbg!(iter.get(3));
        dbg!(iter.get(4));
        dbg!(iter.get(5));
        dbg!(iter.get(20));
        dbg!(iter.get(20));
        dbg!(iter.get(0));
        drop(iter);

        vec.push(21);
        vec.push(22);

        let mut iter = vec.into_iter();
        dbg!(iter.get(20));
        dbg!(iter.get(21));
        dbg!(iter.get(22));
        dbg!(iter.get(23));
        drop(iter);

        vec.flush()?;
    }

    {
        let mut vec: VEC = CompressedVec::forced_import(&file, "vec", version)?;

        let mut iter = vec.into_iter();
        dbg!(iter.get(0));
        dbg!(iter.get(20));
        dbg!(iter.get(21));
        dbg!(iter.get(22));
        drop(iter);

        vec.truncate_if_needed(14)?;

        let mut iter = vec.into_iter();
        dbg!(iter.get(0));
        dbg!(iter.get(5));
        dbg!(iter.get(20));
        drop(iter);

        dbg!(vec.collect_signed_range(Some(-5), None)?);

        vec.push(vec.len() as u32);
        dbg!(VecIterator::last(vec.into_iter()));

        dbg!(vec.into_iter().collect::<Vec<_>>());
    }

    {
        let mut vec: VEC = CompressedVec::forced_import(&file, "vec", version)?;

        vec.reset()?;

        dbg!(vec.header(), vec.pushed_len(), vec.stored_len(), vec.len());

        (0..21_u32).for_each(|v| {
            vec.push(v);
        });

        let mut iter = vec.into_iter();
        dbg!(iter.get(0));
        dbg!(iter.get(20));
        dbg!(iter.get(21));
        drop(iter);

        // let reader = vec.create_static_reader();
        // dbg!(vec.take(10, &reader)?);
        // dbg!(vec.get_or_read(10, &reader)?);
        // dbg!(vec.holes());
        // drop(reader);

        vec.flush()?;
        dbg!(vec.holes());
    }

    {
        let mut vec: VEC = CompressedVec::forced_import(&file, "vec", version)?;

        dbg!(vec.holes());

        let reader = vec.create_static_reader();
        dbg!(vec.get_or_read(10, &reader)?);
        drop(reader);

        // vec.update(10, 10)?;
        // vec.update(0, 10)?;

        let reader = vec.create_static_reader();
        dbg!(
            vec.holes(),
            vec.get_or_read(0, &reader)?,
            vec.get_or_read(10, &reader)?
        );
        drop(reader);

        vec.flush()?;
    }

    {
        let vec: VEC = CompressedVec::forced_import(&file, "vec", version)?;

        dbg!(vec.collect()?);
    }

    Ok(())
}
