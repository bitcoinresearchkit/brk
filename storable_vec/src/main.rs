use std::path::Path;

use storable_vec::{StorableVec, Version, ASYNC_READ_ONLY, CACHED_GETS, SINGLE_THREAD};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    {
        let mut vec: StorableVec<usize, u32, CACHED_GETS> =
            StorableVec::forced_import(Path::new("./v"), Version::from(1))?;

        vec.push(0);
        vec.push(1);
        vec.push(2);
        dbg!(vec.get(0)?); // Some(0)
        dbg!(vec.get(21)?); // None

        vec.flush()?;
    }

    {
        let mut vec: StorableVec<usize, u32, SINGLE_THREAD> =
            StorableVec::forced_import(Path::new("./v"), Version::from(1))?;

        dbg!(vec.get(0)?); // 0
        dbg!(vec.get(1)?); // 0
        dbg!(vec.get(2)?); // 0
        dbg!(vec.get(0)?); // 0
    }

    Ok(())
}
