# BRK Vec

<p align="left">
  <a href="https://github.com/bitcoinresearchkit/brk">
    <img alt="GitHub Repo stars" src="https://img.shields.io/github/stars/bitcoinresearchkit/brk?style=social">
  </a>
  <a href="https://github.com/bitcoinresearchkit/brk/blob/main/LICENSE.md">
    <img src="https://img.shields.io/crates/l/brk" alt="License" />
  </a>
  <a href="https://crates.io/crates/brk_vec">
    <img src="https://img.shields.io/crates/v/brk_vec" alt="Version" />
  </a>
  <a href="https://docs.rs/brk_vec">
    <img src="https://img.shields.io/docsrs/brk_vec" alt="Documentation" />
  </a>
  <img src="https://img.shields.io/crates/size/brk_vec" alt="Size" />
  <a href="https://deps.rs/crate/brk_vec">
    <img src="https://deps.rs/crate/brk_vec/latest/status.svg" alt="Dependency status">
  </a>
  <a href="https://discord.gg/Cvrwpv3zEG">
    <img src="https://img.shields.io/discord/1350431684562124850" alt="Chat" />
  </a>
</p>

A `Vec` (an array) that is stored on disk and thus which can be much larger than the available RAM.

Compared to a key/value store, the data stored is raw byte interpretation of the Vec's values without any overhead which is very efficient. Additionally it uses close to no RAM when caching isn't active and up to 100 MB when it is.

Compression is also available and built on top [`zstd`](https://crates.io/crates/zstd) to save even more space (from 0 to 75%). The tradeoff being slower reading speeds, especially random reading speeds. This is due to the data being stored in compressed pages of 16 KB, which means that if you to read even one value in that page you have to uncompress the whole page.

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
