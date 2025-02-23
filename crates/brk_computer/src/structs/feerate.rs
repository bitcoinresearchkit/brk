use derive_deref::Deref;

#[derive(Debug, Deref, Clone, Copy)]
pub struct Feerate(f32);
