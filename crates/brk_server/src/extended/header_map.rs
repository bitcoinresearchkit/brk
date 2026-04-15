use axum::http::{
    HeaderMap,
    header::{self, IF_NONE_MATCH},
};

use super::ContentEncoding;

pub trait HeaderMapExtended {
    fn has_etag(&self, etag: &str) -> bool;
    fn insert_etag(&mut self, etag: &str);

    fn insert_cache_control(&mut self, value: &str);
    fn insert_cache_control_must_revalidate(&mut self);

    fn insert_content_disposition_attachment(&mut self, filename: &str);

    fn insert_content_encoding(&mut self, encoding: ContentEncoding);

    fn insert_content_type_application_json(&mut self);
    fn insert_content_type_text_csv(&mut self);

    fn insert_vary_accept_encoding(&mut self);

    fn insert_deprecation(&mut self, sunset: &'static str);
}

impl HeaderMapExtended for HeaderMap {
    fn has_etag(&self, etag: &str) -> bool {
        self.get(IF_NONE_MATCH).is_some_and(|v| {
            let s = v.as_bytes();
            // Match both quoted and unquoted: "etag" or etag
            s == etag.as_bytes()
                || (s.len() == etag.len() + 2
                    && s[0] == b'"'
                    && s[s.len() - 1] == b'"'
                    && &s[1..s.len() - 1] == etag.as_bytes())
        })
    }

    fn insert_etag(&mut self, etag: &str) {
        self.insert(header::ETAG, format!("\"{etag}\"").parse().unwrap());
    }

    fn insert_cache_control(&mut self, value: &str) {
        self.insert(header::CACHE_CONTROL, value.parse().unwrap());
    }

    fn insert_cache_control_must_revalidate(&mut self) {
        self.insert_cache_control("public, max-age=1, must-revalidate");
    }

    fn insert_content_disposition_attachment(&mut self, filename: &str) {
        self.insert(
            header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{filename}\"")
                .parse()
                .unwrap(),
        );
    }

    fn insert_content_encoding(&mut self, encoding: ContentEncoding) {
        if let Some(value) = encoding.header_value() {
            self.insert(header::CONTENT_ENCODING, value);
            self.insert_vary_accept_encoding();
        }
    }

    fn insert_content_type_application_json(&mut self) {
        self.insert(header::CONTENT_TYPE, "application/json".parse().unwrap());
    }

    fn insert_content_type_text_csv(&mut self) {
        self.insert(header::CONTENT_TYPE, "text/csv".parse().unwrap());
    }

    fn insert_vary_accept_encoding(&mut self) {
        self.insert(header::VARY, "Accept-Encoding".parse().unwrap());
    }

    fn insert_deprecation(&mut self, sunset: &'static str) {
        self.insert("Deprecation", "true".parse().unwrap());
        self.insert("Sunset", sunset.parse().unwrap());
    }
}

#[cfg(test)]
mod tests {
    use axum::http::header::{ETAG, IF_NONE_MATCH};

    use super::HeaderMapExtended;

    #[test]
    fn has_etag_matches_quoted_and_unquoted_values() {
        let mut headers = axum::http::HeaderMap::new();
        headers.insert(IF_NONE_MATCH, "\"abc123\"".parse().unwrap());

        assert!(headers.has_etag("abc123"));
        assert!(!headers.has_etag("different"));
    }

    #[test]
    fn insert_etag_wraps_value_in_quotes() {
        let mut headers = axum::http::HeaderMap::new();
        headers.insert_etag("abc123");

        assert_eq!(headers.get(ETAG).unwrap(), "\"abc123\"");
    }

    #[test]
    fn insert_deprecation_sets_expected_headers() {
        let mut headers = axum::http::HeaderMap::new();
        headers.insert_deprecation("Wed, 01 Jan 2027 00:00:00 GMT");

        assert_eq!(headers.get("Deprecation").unwrap(), "true");
        assert_eq!(
            headers.get("Sunset").unwrap(),
            "Wed, 01 Jan 2027 00:00:00 GMT",
        );
    }
}
