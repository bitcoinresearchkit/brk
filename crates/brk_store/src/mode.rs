#[derive(Debug, Clone, Copy)]
pub enum Mode {
    Any,
    PushOnly,
}

impl Mode {
    pub fn is_any(&self) -> bool {
        matches!(*self, Self::Any)
    }

    pub fn is_push_only(&self) -> bool {
        matches!(*self, Self::PushOnly)
    }
}
