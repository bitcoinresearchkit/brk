use std::path::Path;

use storable_vec::{AnyStorableVec, StorableVec};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    {
        let mut vec: StorableVec<usize, u32> = StorableVec::import(Path::new("./v"))?;

        vec.push(21);
        dbg!(vec.get(0)?); // 21

        vec.flush()?;
    }

    {
        let vec: StorableVec<usize, u32> = StorableVec::import(Path::new("./v"))?;

        dbg!(vec.get(0)?); // 21
    }

    Ok(())
}
