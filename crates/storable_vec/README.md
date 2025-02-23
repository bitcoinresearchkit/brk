# storable_vec

A very small, fast, efficient and simple storable `vec` which uses `mmap2` for speed.

## Features

- [x] Get (Rayon compatible)
- [x] Push
- [ ] Update
- [ ] Insert
- [ ] Remove

## Example

```rust
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
```

## Disclaimer

Portability will depend on the type of values.

Non bytes/slices types (`u8`, `u16`, ...) will be read as slice in an unsafe manner (using `std::slice::from_raw_parts`) and thus have the endianness of the system. On the other hand, `&[u8]` should be inserted as is.

If portability is important to you, just create a wrapper struct which has custom `get`, `push`, ... methods and does something like:

```rust
impl StorableVecU64 {
    pub fn push(&mut self, value: u64) {
        self.push(&value.to_be_bytes())
    }
}
```
