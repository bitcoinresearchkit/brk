pub trait LevelPolicy<T> {
    fn at_level(&self, level: usize) -> T;
}

impl<T: Copy> LevelPolicy<T> for [T] {
    fn at_level(&self, level: usize) -> T {
        #[expect(clippy::expect_used, reason = "policy is expected not to be empty")]
        self.get(level)
            .copied()
            .unwrap_or_else(|| self.last().copied().expect("policy should not be empty"))
    }
}
