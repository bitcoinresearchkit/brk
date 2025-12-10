#[derive(Debug, Clone, Copy)]
pub enum Kind {
    Recent,
    Random,
    Sequential,
    Vec,
}

impl Kind {
    pub fn is_sequential(&self) -> bool {
        matches!(*self, Self::Sequential)
    }

    pub fn is_recent(&self) -> bool {
        matches!(*self, Self::Recent)
    }

    pub fn is_random(&self) -> bool {
        matches!(*self, Self::Random)
    }

    pub fn is_vec(&self) -> bool {
        matches!(*self, Self::Vec)
    }
}
