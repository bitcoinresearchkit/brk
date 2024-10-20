use std::path::Path;

use axum::{
    body::Body,
    http::{header, HeaderMap, Response},
    response::IntoResponse,
};
use chrono::{DateTime, Timelike, Utc};
use parser::log;
use reqwest::{
    header::{HOST, IF_MODIFIED_SINCE},
    StatusCode,
};

const STALE_IF_ERROR: u64 = 30_000_000; // 1 Year ish
const MODIFIED_SINCE_FORMAT: &str = "%a, %d %b %Y %H:%M:%S GMT";

pub trait HeaderMapUtils {
    fn get_scheme(&self) -> &str;
    fn get_host(&self) -> &str;
    fn check_if_host_is_any_local(&self) -> bool;
    fn check_if_host_is_0000(&self) -> bool;
    fn check_if_host_is_localhost(&self) -> bool;

    fn insert_cors(&mut self);

    fn get_if_modified_since(&self) -> Option<DateTime<Utc>>;
    fn check_if_modified_since(
        &self,
        path: &Path,
    ) -> color_eyre::Result<(DateTime<Utc>, Option<Response<Body>>)>;

    fn insert_cache_control_immutable(&mut self);
    fn insert_cache_control_revalidate(&mut self, max_age: u64, stale_while_revalidate: u64);
    fn insert_last_modified(&mut self, date: DateTime<Utc>);

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
}

impl HeaderMapUtils for HeaderMap {
    fn get_scheme(&self) -> &str {
        if self.check_if_host_is_any_local() {
            "http"
        } else {
            "https"
        }
    }

    fn get_host(&self) -> &str {
        self[HOST].to_str().unwrap()
    }

    fn check_if_host_is_any_local(&self) -> bool {
        self.check_if_host_is_localhost() || self.check_if_host_is_0000()
    }

    fn check_if_host_is_0000(&self) -> bool {
        self.get_host().contains("0.0.0.0")
    }

    fn check_if_host_is_localhost(&self) -> bool {
        self.get_host().contains("localhost")
    }

    fn insert_cors(&mut self) {
        self.insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*".parse().unwrap());
        self.insert(header::ACCESS_CONTROL_ALLOW_HEADERS, "*".parse().unwrap());
    }

    fn insert_cache_control_immutable(&mut self) {
        self.insert(
            header::CACHE_CONTROL,
            format!("public, max-age=604800, immutable, stale-if-error={STALE_IF_ERROR}")
                .parse()
                .unwrap(),
        );
    }

    fn insert_cache_control_revalidate(&mut self, max_age: u64, stale_while_revalidate: u64) {
        self.insert(
        header::CACHE_CONTROL,
        format!(
            "public, max-age={max_age}, stale-while-revalidate={stale_while_revalidate}, stale-if-error={STALE_IF_ERROR}")
        .parse()
        .unwrap(),
    );
    }

    fn insert_last_modified(&mut self, date: DateTime<Utc>) {
        let formatted = date.format(MODIFIED_SINCE_FORMAT).to_string();

        self.insert(header::LAST_MODIFIED, formatted.parse().unwrap());
    }

    fn check_if_modified_since(
        &self,
        path: &Path,
    ) -> color_eyre::Result<(DateTime<Utc>, Option<Response<Body>>)> {
        let time = path.metadata()?.modified()?;
        let date: DateTime<Utc> = time.into();
        let date = date.with_nanosecond(0).unwrap();
        let mut response_opt = None;

        if let Some(if_modified_since) = self.get_if_modified_since() {
            if if_modified_since == date {
                let mut response = (StatusCode::NOT_MODIFIED, "").into_response();
                let headers = response.headers_mut();
                headers.insert_cors();
                response_opt.replace(response);
            }
        }

        Ok((date, response_opt))
    }

    fn get_if_modified_since(&self) -> Option<DateTime<Utc>> {
        if let Some(modified_since) = self.get(IF_MODIFIED_SINCE) {
            if let Ok(modified_since) = modified_since.to_str() {
                let date = DateTime::parse_from_str(
                    &format!("{modified_since} +00:00"),
                    &format!("{MODIFIED_SINCE_FORMAT} %z"),
                );

                if let Ok(x) = date {
                    return Some(x.to_utc());
                }
            }
        }

        None
    }

    // https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types/Common_types
    fn insert_content_type(&mut self, path: &Path) {
        match path.extension().unwrap().to_str().unwrap() {
            "js" => self.insert_content_type_application_javascript(),
            "json" => self.insert_content_type_application_json(),
            "html" => self.insert_content_type_text_html(),
            "css" => self.insert_content_type_text_css(),
            "toml" | "txt" => self.insert_content_type_text_plain(),
            "pdf" => self.insert_content_type_application_pdf(),
            "woff2" => self.insert_content_type_font_woff2(),
            "ico" => self.insert_content_type_image_icon(),
            "jpg" | "jpeg" => self.insert_content_type_image_jpeg(),
            "png" => self.insert_content_type_image_png(),
            "webmanifest" => self.insert_content_type_application_manifest_json(),
            extension => {
                log(&format!("Extension unsupported: {extension}"));
                panic!()
            }
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
}
