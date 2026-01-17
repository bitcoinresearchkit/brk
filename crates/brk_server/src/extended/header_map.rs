use std::path::Path;

use axum::http::{
    HeaderMap,
    header::{self, IF_NONE_MATCH},
};

pub trait HeaderMapExtended {
    fn has_etag(&self, etag: &str) -> bool;

    fn insert_cache_control(&mut self, value: &str);
    fn insert_cache_control_must_revalidate(&mut self);
    fn insert_cache_control_immutable(&mut self);
    fn insert_etag(&mut self, etag: &str);

    fn insert_content_disposition_attachment(&mut self);

    fn insert_content_type(&mut self, path: &Path);
    fn insert_content_type_image_icon(&mut self);
    fn insert_content_type_image_jpeg(&mut self);
    fn insert_content_type_image_png(&mut self);
    fn insert_content_type_application_javascript(&mut self);
    fn insert_content_type_application_json(&mut self);
    fn insert_content_type_application_manifest_json(&mut self);
    fn insert_content_type_application_pdf(&mut self);
    fn insert_content_type_text_css(&mut self);
    fn insert_content_type_text_csv(&mut self);
    fn insert_content_type_text_html(&mut self);
    fn insert_content_type_text_plain(&mut self);
    fn insert_content_type_font_woff2(&mut self);
    fn insert_content_type_octet_stream(&mut self);
}

impl HeaderMapExtended for HeaderMap {
    fn insert_cache_control(&mut self, value: &str) {
        self.insert(header::CACHE_CONTROL, value.parse().unwrap());
    }

    fn insert_cache_control_must_revalidate(&mut self) {
        self.insert_cache_control("public, max-age=1, must-revalidate");
    }

    fn insert_cache_control_immutable(&mut self) {
        self.insert_cache_control("public, max-age=31536000, immutable");
    }

    fn insert_content_disposition_attachment(&mut self) {
        self.insert(header::CONTENT_DISPOSITION, "attachment".parse().unwrap());
    }

    fn insert_etag(&mut self, etag: &str) {
        self.insert(header::ETAG, etag.parse().unwrap());
    }

    fn has_etag(&self, etag: &str) -> bool {
        self.get(IF_NONE_MATCH)
            .is_some_and(|prev_etag| etag == prev_etag)
    }

    // https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types/Common_types
    fn insert_content_type(&mut self, path: &Path) {
        match path
            .extension()
            .map(|s| s.to_str().unwrap_or_default())
            .unwrap_or_default()
        {
            "js" | "mjs" => self.insert_content_type_application_javascript(),
            "json" | "map" => self.insert_content_type_application_json(),
            "html" => self.insert_content_type_text_html(),
            "css" => self.insert_content_type_text_css(),
            "toml" | "txt" => self.insert_content_type_text_plain(),
            "pdf" => self.insert_content_type_application_pdf(),
            "woff2" => self.insert_content_type_font_woff2(),
            "ico" => self.insert_content_type_image_icon(),
            "jpg" | "jpeg" => self.insert_content_type_image_jpeg(),
            "png" => self.insert_content_type_image_png(),
            "webmanifest" => self.insert_content_type_application_manifest_json(),
            _ => {}
        }
    }

    fn insert_content_type_image_icon(&mut self) {
        self.insert(header::CONTENT_TYPE, "image/x-icon".parse().unwrap());
    }

    fn insert_content_type_image_jpeg(&mut self) {
        self.insert(header::CONTENT_TYPE, "image/jpeg".parse().unwrap());
    }

    fn insert_content_type_image_png(&mut self) {
        self.insert(header::CONTENT_TYPE, "image/png".parse().unwrap());
    }

    fn insert_content_type_application_javascript(&mut self) {
        self.insert(
            header::CONTENT_TYPE,
            "application/javascript".parse().unwrap(),
        );
    }

    fn insert_content_type_application_json(&mut self) {
        self.insert(header::CONTENT_TYPE, "application/json".parse().unwrap());
    }

    fn insert_content_type_application_manifest_json(&mut self) {
        self.insert(
            header::CONTENT_TYPE,
            "application/manifest+json".parse().unwrap(),
        );
    }

    fn insert_content_type_application_pdf(&mut self) {
        self.insert(header::CONTENT_TYPE, "application/pdf".parse().unwrap());
    }

    fn insert_content_type_text_css(&mut self) {
        self.insert(header::CONTENT_TYPE, "text/css".parse().unwrap());
    }

    fn insert_content_type_text_csv(&mut self) {
        self.insert(header::CONTENT_TYPE, "text/csv".parse().unwrap());
    }

    fn insert_content_type_text_html(&mut self) {
        self.insert(header::CONTENT_TYPE, "text/html".parse().unwrap());
    }

    fn insert_content_type_text_plain(&mut self) {
        self.insert(header::CONTENT_TYPE, "text/plain".parse().unwrap());
    }

    fn insert_content_type_font_woff2(&mut self) {
        self.insert(header::CONTENT_TYPE, "font/woff2".parse().unwrap());
    }

    fn insert_content_type_octet_stream(&mut self) {
        self.insert(
            header::CONTENT_TYPE,
            "application/octet-stream".parse().unwrap(),
        );
    }
}
