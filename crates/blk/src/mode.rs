#[derive(Clone, Copy)]
pub enum Mode {
    Bare,
    Tsv,
    Json,
    Pretty,
}

impl Mode {
    pub fn pick(pretty: bool, compact: bool, n_fields: usize) -> Self {
        if pretty {
            Self::Pretty
        } else if n_fields == 0 {
            Self::Json
        } else if n_fields == 1 {
            Self::Bare
        } else if compact {
            Self::Tsv
        } else {
            Self::Json
        }
    }
}
