use brk_traversable::Traversable;
use derive_more::{Deref, DerefMut};

use crate::internal::LazyValueBlockLast;

/// Circulating supply - lazy references to distribution's actual supply (KISS)
#[derive(Clone, Deref, DerefMut, Traversable)]
pub struct Vecs(pub LazyValueBlockLast);
