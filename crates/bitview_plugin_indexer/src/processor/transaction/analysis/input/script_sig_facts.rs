pub struct ScriptSigFacts<'a> {
    pub accurate_sigops: usize,
    pub last_push: Option<&'a [u8]>,
    pub legacy_sigops: usize,
    pub push_only: bool,
}

impl ScriptSigFacts<'_> {
    pub const EMPTY: Self = Self {
        accurate_sigops: 0,
        last_push: None,
        legacy_sigops: 0,
        push_only: true,
    };
}
