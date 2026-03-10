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

    fn insert_deprecation(&mut self, sunset: &'static str);
}

impl HeaderMapExtended for HeaderMap {
    fn has_etag(&self, etag: &str) -> bool {
        self.get(IF_NONE_MATCH)
            .is_some_and(|prev_etag| etag == prev_etag)
    }

    fn insert_etag(&mut self, etag: &str) {
        self.insert(header::ETAG, etag.parse().unwrap());
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
            format!("attachment; filename=\"{filename}\"").parse().unwrap(),
        );
    }

    fn insert_content_encoding(&mut self, encoding: ContentEncoding) {
        if let Some(value) = encoding.header_value() {
            self.insert(header::CONTENT_ENCODING, value);
        }
    }

    fn insert_content_type_application_json(&mut self) {
        self.insert(header::CONTENT_TYPE, "application/json".parse().unwrap());
    }

    fn insert_content_type_text_csv(&mut self) {
        self.insert(header::CONTENT_TYPE, "text/csv".parse().unwrap());
    }

    fn insert_deprecation(&mut self, sunset: &'static str) {
        self.insert("Deprecation", "true".parse().unwrap());
        self.insert("Sunset", sunset.parse().unwrap());
    }
}
