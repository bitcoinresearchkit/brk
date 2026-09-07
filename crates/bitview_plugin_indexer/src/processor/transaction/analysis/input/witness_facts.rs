use bitcoin::taproot::LeafVersion;

pub struct WitnessFacts<'a> {
    pub has_annex: bool,
    pub last: Option<&'a [u8]>,
    pub leaf_version: Option<LeafVersion>,
    pub max_argument_bytes: usize,
    pub stack_items: usize,
}
