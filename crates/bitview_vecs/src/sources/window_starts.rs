use bitview_collections::Windows;
use derive_more::{Deref, DerefMut};

use crate::LazyWindowStartVec;

#[derive(Deref, DerefMut)]
pub struct WindowStarts<'a>(pub Windows<&'a LazyWindowStartVec>);
