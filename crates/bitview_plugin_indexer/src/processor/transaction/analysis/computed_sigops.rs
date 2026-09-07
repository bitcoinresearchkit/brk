use brk_types::SigOps;

#[derive(Clone, Copy, Default)]
pub struct ComputedSigOps {
    pub total: SigOps,
    pub executed_legacy: SigOps,
}
