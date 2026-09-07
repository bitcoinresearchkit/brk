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
            Output::Json(v) => String::from_utf8(v).expect("JSON output is not valid UTF-8"),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_both_formats_and_defaults() {
        assert_eq!(Output::CSV("é,1\n".into()).to_string(), "é,1\n");
        assert_eq!(
            Output::Json(br#"{"data":[]}"#.to_vec()).to_string(),
            r#"{"data":[]}"#
        );
        assert!(Output::default(Format::CSV).to_string().is_empty());
        assert_eq!(
            Output::default(Format::JSON).to_string(),
            r#"{"version":0,"total":0,"start":0,"end":0,"data":[]}"#
        );
    }

    #[test]
    #[should_panic(expected = "JSON output is not valid UTF-8")]
    fn invalid_public_json_bytes_do_not_construct_an_invalid_string() {
        Output::Json(vec![255]).to_string();
    }
}
