use std::path::Path;

use brk_vec::{StorableVec, Version};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    {
        let mut vec: StorableVec<usize, u32> = StorableVec::forced_import(Path::new("./v"), Version::from(1))?;

        vec.push(0);
        vec.push(1);
        vec.push(2);
        dbg!(vec.get(0)?); // Some(0)
        dbg!(vec.get(21)?); // None

        vec.flush()?;
    }

    {
        let mut vec: StorableVec<usize, u32> = StorableVec::forced_import(Path::new("./v"), Version::from(1))?;

        dbg!(vec.read(0)?); // 0
        dbg!(vec.read(1)?); // 0
        dbg!(vec.read(2)?); // 0
        dbg!(vec.read(0)?); // 0
    }

    Ok(())
}
