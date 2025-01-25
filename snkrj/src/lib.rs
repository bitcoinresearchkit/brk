// https://docs.rs/sanakirja/latest/sanakirja/index.html
// https://pijul.org/posts/2021-02-06-rethinking-sanakirja/

pub use sanakirja::*;

mod base;
mod multi;
mod traits;
mod unique;

pub use base::*;
pub use multi::*;
pub use traits::*;
pub use unique::*;
