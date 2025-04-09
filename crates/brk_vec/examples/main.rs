use std::{fs, path::Path};

use brk_vec::{AnyVec, RawVec, Version};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = fs::remove_dir_all("./vec");

    {
        let mut vec: RawVec<usize, u32> = RawVec::forced_import(Path::new("./vec"), Version::ZERO)?;

        (0..21_u32).for_each(|v| {
            vec.push(v);
        });
        dbg!(vec.get(0)?);
        dbg!(vec.get(20)?);
        dbg!(vec.get(21)?);

        vec.flush()?;
    }

    {
        let mut vec: RawVec<usize, u32> = RawVec::forced_import(Path::new("./vec"), Version::ZERO)?;

        dbg!(vec.get(0)?);
        dbg!(vec.get(0)?);
        dbg!(vec.get(1)?);
        dbg!(vec.get(2)?);
        dbg!(vec.get(20)?);
        dbg!(vec.get(20)?);
        dbg!(vec.get(0)?);

        vec.push(21);
        vec.push(22);
        dbg!(vec.get(20)?);
        dbg!(vec.get(21)?);
        dbg!(vec.get(22)?);
        dbg!(vec.get(23)?);

        vec.flush()?;
    }

    {
        let mut vec: RawVec<usize, u32> = RawVec::forced_import(Path::new("./vec"), Version::ZERO)?;

        // vec.enable_large_cache_if_possible();

        dbg!(vec.get(0)?);
        dbg!(vec.get(20)?);
        dbg!(vec.get(21)?);
        dbg!(vec.get(22)?);

        vec.truncate_if_needed(14)?;

        dbg!(vec.get(0)?);
        dbg!(vec.get(5)?);
        dbg!(vec.get(20)?);

        vec.iter(|(_, v, ..)| {
            dbg!(v);
            Ok(())
        })?;

        vec.iter_from(5, |(_, v, ..)| {
            dbg!(v);
            Ok(())
        })?;

        dbg!(vec.collect_range(Some(-5), None)?);
    }

    Ok(())
}
