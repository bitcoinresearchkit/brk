use std::path::Path;

use storable_vec::StorableVec;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    {
        let mut vec: StorableVec<usize, u32> = StorableVec::import(Path::new("./v"))?;

        vec.push(0);
        vec.push(1);
        vec.push(2);
        dbg!(vec.get(0)?); // Some(0)
        dbg!(vec.get(21)?); // None

        vec.flush()?;
    }

    {
        let vec: StorableVec<usize, u32> = StorableVec::import(Path::new("./v"))?;

        dbg!(vec.get(0)?); // 0
    }

    Ok(())
}
