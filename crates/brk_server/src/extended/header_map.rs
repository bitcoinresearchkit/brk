use axum::http::{
    HeaderMap,
    header::{self, IF_NONE_MATCH},
};

pub trait HeaderMapExtended {
    fn has_etag(&self, etag: &str) -> bool;
    fn insert_etag(&mut self, etag: &str);

    fn insert_cache_control(&mut self, value: &str);
    fn insert_cache_control_must_revalidate(&mut self);

    fn insert_content_disposition_attachment(&mut self);

    fn insert_content_type_application_json(&mut self);
    fn insert_content_type_text_csv(&mut self);
    fn insert_content_type_text_plain(&mut self);
    fn insert_content_type_octet_stream(&mut self);
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

    fn insert_content_disposition_attachment(&mut self) {
        self.insert(header::CONTENT_DISPOSITION, "attachment".parse().unwrap());
    }

    fn insert_content_type_application_json(&mut self) {
        self.insert(header::CONTENT_TYPE, "application/json".parse().unwrap());
    }

    fn insert_content_type_text_csv(&mut self) {
        self.insert(header::CONTENT_TYPE, "text/csv".parse().unwrap());
    }

    fn insert_content_type_text_plain(&mut self) {
        self.insert(header::CONTENT_TYPE, "text/plain".parse().unwrap());
    }

    fn insert_content_type_octet_stream(&mut self) {
        self.insert(
            header::CONTENT_TYPE,
            "application/octet-stream".parse().unwrap(),
        );
    }
}
