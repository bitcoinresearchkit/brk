use std::{fs, path::Path};

use brk_vec::{Compressed, DynamicVec, GenericVec, StoredVec, Version};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = fs::remove_dir_all("./vec");

    let version = Version::ZERO;
    let compressed = Compressed::YES;

    {
        let mut vec: StoredVec<usize, u32> =
            StoredVec::forced_import(Path::new("./vec"), version, compressed)?;

        (0..21_u32).for_each(|v| {
            vec.push(v);
        });
        dbg!(vec.get(0)?);
        dbg!(vec.get(20)?);
        dbg!(vec.get(21)?);

        vec.flush()?;
    }

    {
        let mut vec: StoredVec<usize, u32> =
            StoredVec::forced_import(Path::new("./vec"), version, compressed)?;

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
        let mut vec: StoredVec<usize, u32> =
            StoredVec::forced_import(Path::new("./vec"), version, compressed)?;

        vec.enable_large_cache_if_needed();

        dbg!(vec.get(0)?);
        dbg!(vec.get(20)?);
        dbg!(vec.get(21)?);
        dbg!(vec.get(22)?);

        vec.truncate_if_needed(14)?;

        dbg!(vec.get(0)?);
        dbg!(vec.get(5)?);
        dbg!(vec.get(20)?);

        dbg!(vec.collect_signed_range(Some(-5), None)?);

        vec.push(vec.len() as u32);
        dbg!(vec.iter().last());

        dbg!(vec.into_iter().collect::<Vec<_>>());
    }

    Ok(())
}
