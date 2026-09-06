use bitview_types::Format;

/// Series data output format
#[derive(Debug)]
pub enum Output {
    Json(Vec<u8>),
    CSV(String),
}

impl Output {
    #[allow(clippy::inherent_to_string)]
    pub fn to_string(self) -> String {
        match self {
            Output::CSV(s) => s,
            Output::Json(v) => unsafe { String::from_utf8_unchecked(v) },
        }
    }

    pub fn default(format: Format) -> Self {
        match format {
            Format::CSV => Output::CSV(String::new()),
            Format::JSON => {
                Output::Json(br#"{"version":0,"total":0,"start":0,"end":0,"data":[]}"#.to_vec())
            }
        }
    }
}
