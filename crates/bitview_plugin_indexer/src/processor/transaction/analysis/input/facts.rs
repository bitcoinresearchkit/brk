use super::{ScriptSigFacts, WitnessFacts, redeem};

pub struct Facts<'a> {
    pub script_sig: ScriptSigFacts<'a>,
    pub redeem: redeem::Facts,
    pub witness: WitnessFacts<'a>,
}
