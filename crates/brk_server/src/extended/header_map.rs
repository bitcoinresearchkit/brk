use std::{
    path::Path,
    time::{self, Duration},
};

use axum::http::{
    HeaderMap,
    header::{self, IF_MODIFIED_SINCE, IF_NONE_MATCH},
};
use brk_error::Result;
use jiff::{Timestamp, civil::DateTime, fmt::strtime, tz::TimeZone};

const MODIFIED_SINCE_FORMAT: &str = "%a, %d %b %Y %H:%M:%S GMT";

#[derive(PartialEq, Eq)]
pub enum ModifiedState {
    ModifiedSince,
    NotModifiedSince,
}

pub trait HeaderMapExtended {
    fn insert_cors(&mut self);

    fn get_if_none_match(&self) -> Option<&str>;

    fn get_if_modified_since(&self) -> Option<DateTime>;
    fn check_if_modified_since(&self, path: &Path) -> Result<(ModifiedState, DateTime)>;
    fn check_if_modified_since_(&self, duration: Duration) -> Result<(ModifiedState, DateTime)>;

    fn insert_cache_control_must_revalidate(&mut self);
    fn insert_cache_control_immutable(&mut self);
    fn insert_etag(&mut self, etag: &str);
    fn insert_last_modified(&mut self, date: DateTime);

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
    fn insert_content_type_text_tsv(&mut self);
    fn insert_content_type_text_html(&mut self);
    fn insert_content_type_text_plain(&mut self);
    fn insert_content_type_font_woff2(&mut self);
}

impl HeaderMapExtended for HeaderMap {
    fn insert_cors(&mut self) {
        self.insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*".parse().unwrap());
        self.insert(header::ACCESS_CONTROL_ALLOW_HEADERS, "*".parse().unwrap());
    }

    fn insert_cache_control_must_revalidate(&mut self) {
        self.insert(
            header::CACHE_CONTROL,
            "public, max-age=0, must-revalidate".parse().unwrap(),
        );
    }

    fn insert_cache_control_immutable(&mut self) {
        self.insert(
            header::CACHE_CONTROL,
            "public, max-age=31536000, immutable".parse().unwrap(),
        );
    }

    fn insert_content_disposition_attachment(&mut self) {
        self.insert(header::CONTENT_DISPOSITION, "attachment".parse().unwrap());
    }

    fn insert_last_modified(&mut self, date: DateTime) {
        let formatted = date
            .to_zoned(TimeZone::system())
            .unwrap()
            .strftime(MODIFIED_SINCE_FORMAT)
            .to_string();

        self.insert(header::LAST_MODIFIED, formatted.parse().unwrap());
    }

    fn insert_etag(&mut self, etag: &str) {
        self.insert(header::ETAG, etag.parse().unwrap());
    }

    fn check_if_modified_since(&self, path: &Path) -> Result<(ModifiedState, DateTime)> {
        self.check_if_modified_since_(
            path.metadata()?
                .modified()?
                .duration_since(time::UNIX_EPOCH)?,
        )
    }

    fn check_if_modified_since_(&self, duration: Duration) -> Result<(ModifiedState, DateTime)> {
        let date = Timestamp::new(duration.as_secs() as i64, 0)
            .unwrap()
            .to_zoned(TimeZone::UTC)
            .datetime();

        if let Some(if_modified_since) = self.get_if_modified_since()
            && if_modified_since == date
        {
            return Ok((ModifiedState::NotModifiedSince, date));
        }

        Ok((ModifiedState::ModifiedSince, date))
    }

    fn get_if_modified_since(&self) -> Option<DateTime> {
        if let Some(modified_since) = self.get(IF_MODIFIED_SINCE)
            && let Ok(modified_since) = modified_since.to_str()
        {
            return strtime::parse(MODIFIED_SINCE_FORMAT, modified_since)
                .unwrap()
                .to_datetime()
                .ok();
        }

        None
    }

    fn get_if_none_match(&self) -> Option<&str> {
        self.get(IF_NONE_MATCH).and_then(|v| v.to_str().ok())
    }

    // https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types/Common_types
    fn insert_content_type(&mut self, path: &Path) {
        match path
            .extension()
            .map(|s| s.to_str().unwrap_or_default())
            .unwrap_or_default()
        {
            "js" => self.insert_content_type_application_javascript(),
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

    fn insert_content_type_text_tsv(&mut self) {
        self.insert(
            header::CONTENT_TYPE,
            "text/tab-separated-values".parse().unwrap(),
        );
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
}
