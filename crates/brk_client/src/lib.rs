// Auto-generated BRK Rust client
// Do not edit manually

#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(clippy::useless_format)]
#![allow(clippy::unnecessary_to_owned)]

use std::sync::Arc;
use serde::de::DeserializeOwned;
pub use brk_cohort::*;
pub use brk_types::*;


/// Error type for BRK client operations.
#[derive(Debug)]
pub struct BrkError {
    pub message: String,
}

impl std::fmt::Display for BrkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for BrkError {}

/// Result type for BRK client operations.
pub type Result<T> = std::result::Result<T, BrkError>;

/// Options for configuring the BRK client.
#[derive(Debug, Clone)]
pub struct BrkClientOptions {
    pub base_url: String,
    pub timeout_secs: u64,
}

impl Default for BrkClientOptions {
    fn default() -> Self {
        Self {
            base_url: "http://localhost:3000".to_string(),
            timeout_secs: 30,
        }
    }
}

/// Base HTTP client for making requests.
#[derive(Debug, Clone)]
pub struct BrkClientBase {
    base_url: String,
    timeout_secs: u64,
}

impl BrkClientBase {
    /// Create a new client with the given base URL.
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            timeout_secs: 30,
        }
    }

    /// Create a new client with options.
    pub fn with_options(options: BrkClientOptions) -> Self {
        Self {
            base_url: options.base_url,
            timeout_secs: options.timeout_secs,
        }
    }

    /// Make a GET request.
    pub fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let base = self.base_url.trim_end_matches('/');
        let url = format!("{}{}", base, path);
        let response = minreq::get(&url)
            .with_timeout(self.timeout_secs)
            .send()
            .map_err(|e| BrkError { message: e.to_string() })?;

        if response.status_code >= 400 {
            return Err(BrkError {
                message: format!("HTTP {}", response.status_code),
            });
        }

        response
            .json()
            .map_err(|e| BrkError { message: e.to_string() })
    }
}

/// Build metric name with optional prefix.
#[inline]
fn _m(acc: &str, s: &str) -> String {
    if acc.is_empty() { s.to_string() } else { format!("{acc}_{s}") }
}


/// Non-generic trait for metric patterns (usable in collections).
pub trait AnyMetricPattern {
    /// Get the metric name.
    fn name(&self) -> &str;

    /// Get the list of available indexes for this metric.
    fn indexes(&self) -> &'static [Index];
}

/// Generic trait for metric patterns with endpoint access.
pub trait MetricPattern<T>: AnyMetricPattern {
    /// Get an endpoint for a specific index, if supported.
    fn get(&self, index: Index) -> Option<Endpoint<T>>;
}


/// An endpoint for a specific metric + index combination.
pub struct Endpoint<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    index: Index,
    _marker: std::marker::PhantomData<T>,
}

impl<T: DeserializeOwned> Endpoint<T> {
    pub fn new(client: Arc<BrkClientBase>, name: Arc<str>, index: Index) -> Self {
        Self {
            client,
            name,
            index,
            _marker: std::marker::PhantomData,
        }
    }

    /// Fetch all data points for this metric/index.
    pub fn get(&self) -> Result<Vec<T>> {
        self.client.get(&self.path())
    }

    /// Fetch data points within a range.
    pub fn range(&self, from: Option<i64>, to: Option<i64>) -> Result<Vec<T>> {
        let mut params = Vec::new();
        if let Some(f) = from { params.push(format!("from={}", f)); }
        if let Some(t) = to { params.push(format!("to={}", t)); }
        let p = self.path();
        let path = if params.is_empty() {
            p
        } else {
            format!("{}?{}", p, params.join("&"))
        };
        self.client.get(&path)
    }

    /// Get the endpoint path.
    pub fn path(&self) -> String {
        format!("/api/metric/{}/{}", self.name, self.index.serialize_long())
    }
}


// Index accessor structs

/// Container for index endpoint methods.
pub struct MetricPattern1By<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: DeserializeOwned> MetricPattern1By<T> {
    pub fn by_dateindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::DateIndex)
    }
    pub fn by_decadeindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::DecadeIndex)
    }
    pub fn by_difficultyepoch(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::DifficultyEpoch)
    }
    pub fn by_height(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::Height)
    }
    pub fn by_monthindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::MonthIndex)
    }
    pub fn by_quarterindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::QuarterIndex)
    }
    pub fn by_semesterindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::SemesterIndex)
    }
    pub fn by_weekindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::WeekIndex)
    }
    pub fn by_yearindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::YearIndex)
    }
}

/// Index accessor for metrics with 9 indexes.
pub struct MetricPattern1<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    pub by: MetricPattern1By<T>,
}

impl<T: DeserializeOwned> MetricPattern1<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self {
        let name: Arc<str> = name.into();
        Self {
            client: client.clone(),
            name: name.clone(),
            by: MetricPattern1By {
                client,
                name,
                _marker: std::marker::PhantomData,
            }
        }
    }

    /// Get the metric name.
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<T> AnyMetricPattern for MetricPattern1<T> {
    fn name(&self) -> &str {
        &self.name
    }

    fn indexes(&self) -> &'static [Index] {
        &[
            Index::DateIndex,
            Index::DecadeIndex,
            Index::DifficultyEpoch,
            Index::Height,
            Index::MonthIndex,
            Index::QuarterIndex,
            Index::SemesterIndex,
            Index::WeekIndex,
            Index::YearIndex,
        ]
    }
}

impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern1<T> {
    fn get(&self, index: Index) -> Option<Endpoint<T>> {
        match index {
            Index::DateIndex => Some(self.by.by_dateindex()),
            Index::DecadeIndex => Some(self.by.by_decadeindex()),
            Index::DifficultyEpoch => Some(self.by.by_difficultyepoch()),
            Index::Height => Some(self.by.by_height()),
            Index::MonthIndex => Some(self.by.by_monthindex()),
            Index::QuarterIndex => Some(self.by.by_quarterindex()),
            Index::SemesterIndex => Some(self.by.by_semesterindex()),
            Index::WeekIndex => Some(self.by.by_weekindex()),
            Index::YearIndex => Some(self.by.by_yearindex()),
            _ => None,
        }
    }
}

/// Container for index endpoint methods.
pub struct MetricPattern2By<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: DeserializeOwned> MetricPattern2By<T> {
    pub fn by_dateindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::DateIndex)
    }
    pub fn by_decadeindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::DecadeIndex)
    }
    pub fn by_difficultyepoch(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::DifficultyEpoch)
    }
    pub fn by_monthindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::MonthIndex)
    }
    pub fn by_quarterindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::QuarterIndex)
    }
    pub fn by_semesterindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::SemesterIndex)
    }
    pub fn by_weekindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::WeekIndex)
    }
    pub fn by_yearindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::YearIndex)
    }
}

/// Index accessor for metrics with 8 indexes.
pub struct MetricPattern2<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    pub by: MetricPattern2By<T>,
}

impl<T: DeserializeOwned> MetricPattern2<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self {
        let name: Arc<str> = name.into();
        Self {
            client: client.clone(),
            name: name.clone(),
            by: MetricPattern2By {
                client,
                name,
                _marker: std::marker::PhantomData,
            }
        }
    }

    /// Get the metric name.
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<T> AnyMetricPattern for MetricPattern2<T> {
    fn name(&self) -> &str {
        &self.name
    }

    fn indexes(&self) -> &'static [Index] {
        &[
            Index::DateIndex,
            Index::DecadeIndex,
            Index::DifficultyEpoch,
            Index::MonthIndex,
            Index::QuarterIndex,
            Index::SemesterIndex,
            Index::WeekIndex,
            Index::YearIndex,
        ]
    }
}

impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern2<T> {
    fn get(&self, index: Index) -> Option<Endpoint<T>> {
        match index {
            Index::DateIndex => Some(self.by.by_dateindex()),
            Index::DecadeIndex => Some(self.by.by_decadeindex()),
            Index::DifficultyEpoch => Some(self.by.by_difficultyepoch()),
            Index::MonthIndex => Some(self.by.by_monthindex()),
            Index::QuarterIndex => Some(self.by.by_quarterindex()),
            Index::SemesterIndex => Some(self.by.by_semesterindex()),
            Index::WeekIndex => Some(self.by.by_weekindex()),
            Index::YearIndex => Some(self.by.by_yearindex()),
            _ => None,
        }
    }
}

/// Container for index endpoint methods.
pub struct MetricPattern3By<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: DeserializeOwned> MetricPattern3By<T> {
    pub fn by_dateindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::DateIndex)
    }
    pub fn by_decadeindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::DecadeIndex)
    }
    pub fn by_height(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::Height)
    }
    pub fn by_monthindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::MonthIndex)
    }
    pub fn by_quarterindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::QuarterIndex)
    }
    pub fn by_semesterindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::SemesterIndex)
    }
    pub fn by_weekindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::WeekIndex)
    }
    pub fn by_yearindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::YearIndex)
    }
}

/// Index accessor for metrics with 8 indexes.
pub struct MetricPattern3<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    pub by: MetricPattern3By<T>,
}

impl<T: DeserializeOwned> MetricPattern3<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self {
        let name: Arc<str> = name.into();
        Self {
            client: client.clone(),
            name: name.clone(),
            by: MetricPattern3By {
                client,
                name,
                _marker: std::marker::PhantomData,
            }
        }
    }

    /// Get the metric name.
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<T> AnyMetricPattern for MetricPattern3<T> {
    fn name(&self) -> &str {
        &self.name
    }

    fn indexes(&self) -> &'static [Index] {
        &[
            Index::DateIndex,
            Index::DecadeIndex,
            Index::Height,
            Index::MonthIndex,
            Index::QuarterIndex,
            Index::SemesterIndex,
            Index::WeekIndex,
            Index::YearIndex,
        ]
    }
}

impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern3<T> {
    fn get(&self, index: Index) -> Option<Endpoint<T>> {
        match index {
            Index::DateIndex => Some(self.by.by_dateindex()),
            Index::DecadeIndex => Some(self.by.by_decadeindex()),
            Index::Height => Some(self.by.by_height()),
            Index::MonthIndex => Some(self.by.by_monthindex()),
            Index::QuarterIndex => Some(self.by.by_quarterindex()),
            Index::SemesterIndex => Some(self.by.by_semesterindex()),
            Index::WeekIndex => Some(self.by.by_weekindex()),
            Index::YearIndex => Some(self.by.by_yearindex()),
            _ => None,
        }
    }
}

/// Container for index endpoint methods.
pub struct MetricPattern4By<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: DeserializeOwned> MetricPattern4By<T> {
    pub fn by_dateindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::DateIndex)
    }
    pub fn by_decadeindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::DecadeIndex)
    }
    pub fn by_monthindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::MonthIndex)
    }
    pub fn by_quarterindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::QuarterIndex)
    }
    pub fn by_semesterindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::SemesterIndex)
    }
    pub fn by_weekindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::WeekIndex)
    }
    pub fn by_yearindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::YearIndex)
    }
}

/// Index accessor for metrics with 7 indexes.
pub struct MetricPattern4<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    pub by: MetricPattern4By<T>,
}

impl<T: DeserializeOwned> MetricPattern4<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self {
        let name: Arc<str> = name.into();
        Self {
            client: client.clone(),
            name: name.clone(),
            by: MetricPattern4By {
                client,
                name,
                _marker: std::marker::PhantomData,
            }
        }
    }

    /// Get the metric name.
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<T> AnyMetricPattern for MetricPattern4<T> {
    fn name(&self) -> &str {
        &self.name
    }

    fn indexes(&self) -> &'static [Index] {
        &[
            Index::DateIndex,
            Index::DecadeIndex,
            Index::MonthIndex,
            Index::QuarterIndex,
            Index::SemesterIndex,
            Index::WeekIndex,
            Index::YearIndex,
        ]
    }
}

impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern4<T> {
    fn get(&self, index: Index) -> Option<Endpoint<T>> {
        match index {
            Index::DateIndex => Some(self.by.by_dateindex()),
            Index::DecadeIndex => Some(self.by.by_decadeindex()),
            Index::MonthIndex => Some(self.by.by_monthindex()),
            Index::QuarterIndex => Some(self.by.by_quarterindex()),
            Index::SemesterIndex => Some(self.by.by_semesterindex()),
            Index::WeekIndex => Some(self.by.by_weekindex()),
            Index::YearIndex => Some(self.by.by_yearindex()),
            _ => None,
        }
    }
}

/// Container for index endpoint methods.
pub struct MetricPattern5By<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: DeserializeOwned> MetricPattern5By<T> {
    pub fn by_decadeindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::DecadeIndex)
    }
    pub fn by_monthindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::MonthIndex)
    }
    pub fn by_quarterindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::QuarterIndex)
    }
    pub fn by_semesterindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::SemesterIndex)
    }
    pub fn by_weekindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::WeekIndex)
    }
    pub fn by_yearindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::YearIndex)
    }
}

/// Index accessor for metrics with 6 indexes.
pub struct MetricPattern5<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    pub by: MetricPattern5By<T>,
}

impl<T: DeserializeOwned> MetricPattern5<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self {
        let name: Arc<str> = name.into();
        Self {
            client: client.clone(),
            name: name.clone(),
            by: MetricPattern5By {
                client,
                name,
                _marker: std::marker::PhantomData,
            }
        }
    }

    /// Get the metric name.
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<T> AnyMetricPattern for MetricPattern5<T> {
    fn name(&self) -> &str {
        &self.name
    }

    fn indexes(&self) -> &'static [Index] {
        &[
            Index::DecadeIndex,
            Index::MonthIndex,
            Index::QuarterIndex,
            Index::SemesterIndex,
            Index::WeekIndex,
            Index::YearIndex,
        ]
    }
}

impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern5<T> {
    fn get(&self, index: Index) -> Option<Endpoint<T>> {
        match index {
            Index::DecadeIndex => Some(self.by.by_decadeindex()),
            Index::MonthIndex => Some(self.by.by_monthindex()),
            Index::QuarterIndex => Some(self.by.by_quarterindex()),
            Index::SemesterIndex => Some(self.by.by_semesterindex()),
            Index::WeekIndex => Some(self.by.by_weekindex()),
            Index::YearIndex => Some(self.by.by_yearindex()),
            _ => None,
        }
    }
}

/// Container for index endpoint methods.
pub struct MetricPattern6By<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: DeserializeOwned> MetricPattern6By<T> {
    pub fn by_dateindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::DateIndex)
    }
    pub fn by_height(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::Height)
    }
}

/// Index accessor for metrics with 2 indexes.
pub struct MetricPattern6<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    pub by: MetricPattern6By<T>,
}

impl<T: DeserializeOwned> MetricPattern6<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self {
        let name: Arc<str> = name.into();
        Self {
            client: client.clone(),
            name: name.clone(),
            by: MetricPattern6By {
                client,
                name,
                _marker: std::marker::PhantomData,
            }
        }
    }

    /// Get the metric name.
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<T> AnyMetricPattern for MetricPattern6<T> {
    fn name(&self) -> &str {
        &self.name
    }

    fn indexes(&self) -> &'static [Index] {
        &[
            Index::DateIndex,
            Index::Height,
        ]
    }
}

impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern6<T> {
    fn get(&self, index: Index) -> Option<Endpoint<T>> {
        match index {
            Index::DateIndex => Some(self.by.by_dateindex()),
            Index::Height => Some(self.by.by_height()),
            _ => None,
        }
    }
}

/// Container for index endpoint methods.
pub struct MetricPattern7By<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: DeserializeOwned> MetricPattern7By<T> {
    pub fn by_dateindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::DateIndex)
    }
}

/// Index accessor for metrics with 1 indexes.
pub struct MetricPattern7<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    pub by: MetricPattern7By<T>,
}

impl<T: DeserializeOwned> MetricPattern7<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self {
        let name: Arc<str> = name.into();
        Self {
            client: client.clone(),
            name: name.clone(),
            by: MetricPattern7By {
                client,
                name,
                _marker: std::marker::PhantomData,
            }
        }
    }

    /// Get the metric name.
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<T> AnyMetricPattern for MetricPattern7<T> {
    fn name(&self) -> &str {
        &self.name
    }

    fn indexes(&self) -> &'static [Index] {
        &[
            Index::DateIndex,
        ]
    }
}

impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern7<T> {
    fn get(&self, index: Index) -> Option<Endpoint<T>> {
        match index {
            Index::DateIndex => Some(self.by.by_dateindex()),
            _ => None,
        }
    }
}

/// Container for index endpoint methods.
pub struct MetricPattern8By<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: DeserializeOwned> MetricPattern8By<T> {
    pub fn by_decadeindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::DecadeIndex)
    }
}

/// Index accessor for metrics with 1 indexes.
pub struct MetricPattern8<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    pub by: MetricPattern8By<T>,
}

impl<T: DeserializeOwned> MetricPattern8<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self {
        let name: Arc<str> = name.into();
        Self {
            client: client.clone(),
            name: name.clone(),
            by: MetricPattern8By {
                client,
                name,
                _marker: std::marker::PhantomData,
            }
        }
    }

    /// Get the metric name.
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<T> AnyMetricPattern for MetricPattern8<T> {
    fn name(&self) -> &str {
        &self.name
    }

    fn indexes(&self) -> &'static [Index] {
        &[
            Index::DecadeIndex,
        ]
    }
}

impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern8<T> {
    fn get(&self, index: Index) -> Option<Endpoint<T>> {
        match index {
            Index::DecadeIndex => Some(self.by.by_decadeindex()),
            _ => None,
        }
    }
}

/// Container for index endpoint methods.
pub struct MetricPattern9By<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: DeserializeOwned> MetricPattern9By<T> {
    pub fn by_difficultyepoch(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::DifficultyEpoch)
    }
}

/// Index accessor for metrics with 1 indexes.
pub struct MetricPattern9<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    pub by: MetricPattern9By<T>,
}

impl<T: DeserializeOwned> MetricPattern9<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self {
        let name: Arc<str> = name.into();
        Self {
            client: client.clone(),
            name: name.clone(),
            by: MetricPattern9By {
                client,
                name,
                _marker: std::marker::PhantomData,
            }
        }
    }

    /// Get the metric name.
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<T> AnyMetricPattern for MetricPattern9<T> {
    fn name(&self) -> &str {
        &self.name
    }

    fn indexes(&self) -> &'static [Index] {
        &[
            Index::DifficultyEpoch,
        ]
    }
}

impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern9<T> {
    fn get(&self, index: Index) -> Option<Endpoint<T>> {
        match index {
            Index::DifficultyEpoch => Some(self.by.by_difficultyepoch()),
            _ => None,
        }
    }
}

/// Container for index endpoint methods.
pub struct MetricPattern10By<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: DeserializeOwned> MetricPattern10By<T> {
    pub fn by_emptyoutputindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::EmptyOutputIndex)
    }
}

/// Index accessor for metrics with 1 indexes.
pub struct MetricPattern10<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    pub by: MetricPattern10By<T>,
}

impl<T: DeserializeOwned> MetricPattern10<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self {
        let name: Arc<str> = name.into();
        Self {
            client: client.clone(),
            name: name.clone(),
            by: MetricPattern10By {
                client,
                name,
                _marker: std::marker::PhantomData,
            }
        }
    }

    /// Get the metric name.
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<T> AnyMetricPattern for MetricPattern10<T> {
    fn name(&self) -> &str {
        &self.name
    }

    fn indexes(&self) -> &'static [Index] {
        &[
            Index::EmptyOutputIndex,
        ]
    }
}

impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern10<T> {
    fn get(&self, index: Index) -> Option<Endpoint<T>> {
        match index {
            Index::EmptyOutputIndex => Some(self.by.by_emptyoutputindex()),
            _ => None,
        }
    }
}

/// Container for index endpoint methods.
pub struct MetricPattern11By<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: DeserializeOwned> MetricPattern11By<T> {
    pub fn by_halvingepoch(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::HalvingEpoch)
    }
}

/// Index accessor for metrics with 1 indexes.
pub struct MetricPattern11<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    pub by: MetricPattern11By<T>,
}

impl<T: DeserializeOwned> MetricPattern11<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self {
        let name: Arc<str> = name.into();
        Self {
            client: client.clone(),
            name: name.clone(),
            by: MetricPattern11By {
                client,
                name,
                _marker: std::marker::PhantomData,
            }
        }
    }

    /// Get the metric name.
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<T> AnyMetricPattern for MetricPattern11<T> {
    fn name(&self) -> &str {
        &self.name
    }

    fn indexes(&self) -> &'static [Index] {
        &[
            Index::HalvingEpoch,
        ]
    }
}

impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern11<T> {
    fn get(&self, index: Index) -> Option<Endpoint<T>> {
        match index {
            Index::HalvingEpoch => Some(self.by.by_halvingepoch()),
            _ => None,
        }
    }
}

/// Container for index endpoint methods.
pub struct MetricPattern12By<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: DeserializeOwned> MetricPattern12By<T> {
    pub fn by_height(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::Height)
    }
}

/// Index accessor for metrics with 1 indexes.
pub struct MetricPattern12<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    pub by: MetricPattern12By<T>,
}

impl<T: DeserializeOwned> MetricPattern12<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self {
        let name: Arc<str> = name.into();
        Self {
            client: client.clone(),
            name: name.clone(),
            by: MetricPattern12By {
                client,
                name,
                _marker: std::marker::PhantomData,
            }
        }
    }

    /// Get the metric name.
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<T> AnyMetricPattern for MetricPattern12<T> {
    fn name(&self) -> &str {
        &self.name
    }

    fn indexes(&self) -> &'static [Index] {
        &[
            Index::Height,
        ]
    }
}

impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern12<T> {
    fn get(&self, index: Index) -> Option<Endpoint<T>> {
        match index {
            Index::Height => Some(self.by.by_height()),
            _ => None,
        }
    }
}

/// Container for index endpoint methods.
pub struct MetricPattern13By<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: DeserializeOwned> MetricPattern13By<T> {
    pub fn by_txinindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::TxInIndex)
    }
}

/// Index accessor for metrics with 1 indexes.
pub struct MetricPattern13<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    pub by: MetricPattern13By<T>,
}

impl<T: DeserializeOwned> MetricPattern13<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self {
        let name: Arc<str> = name.into();
        Self {
            client: client.clone(),
            name: name.clone(),
            by: MetricPattern13By {
                client,
                name,
                _marker: std::marker::PhantomData,
            }
        }
    }

    /// Get the metric name.
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<T> AnyMetricPattern for MetricPattern13<T> {
    fn name(&self) -> &str {
        &self.name
    }

    fn indexes(&self) -> &'static [Index] {
        &[
            Index::TxInIndex,
        ]
    }
}

impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern13<T> {
    fn get(&self, index: Index) -> Option<Endpoint<T>> {
        match index {
            Index::TxInIndex => Some(self.by.by_txinindex()),
            _ => None,
        }
    }
}

/// Container for index endpoint methods.
pub struct MetricPattern14By<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: DeserializeOwned> MetricPattern14By<T> {
    pub fn by_monthindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::MonthIndex)
    }
}

/// Index accessor for metrics with 1 indexes.
pub struct MetricPattern14<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    pub by: MetricPattern14By<T>,
}

impl<T: DeserializeOwned> MetricPattern14<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self {
        let name: Arc<str> = name.into();
        Self {
            client: client.clone(),
            name: name.clone(),
            by: MetricPattern14By {
                client,
                name,
                _marker: std::marker::PhantomData,
            }
        }
    }

    /// Get the metric name.
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<T> AnyMetricPattern for MetricPattern14<T> {
    fn name(&self) -> &str {
        &self.name
    }

    fn indexes(&self) -> &'static [Index] {
        &[
            Index::MonthIndex,
        ]
    }
}

impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern14<T> {
    fn get(&self, index: Index) -> Option<Endpoint<T>> {
        match index {
            Index::MonthIndex => Some(self.by.by_monthindex()),
            _ => None,
        }
    }
}

/// Container for index endpoint methods.
pub struct MetricPattern15By<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: DeserializeOwned> MetricPattern15By<T> {
    pub fn by_opreturnindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::OpReturnIndex)
    }
}

/// Index accessor for metrics with 1 indexes.
pub struct MetricPattern15<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    pub by: MetricPattern15By<T>,
}

impl<T: DeserializeOwned> MetricPattern15<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self {
        let name: Arc<str> = name.into();
        Self {
            client: client.clone(),
            name: name.clone(),
            by: MetricPattern15By {
                client,
                name,
                _marker: std::marker::PhantomData,
            }
        }
    }

    /// Get the metric name.
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<T> AnyMetricPattern for MetricPattern15<T> {
    fn name(&self) -> &str {
        &self.name
    }

    fn indexes(&self) -> &'static [Index] {
        &[
            Index::OpReturnIndex,
        ]
    }
}

impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern15<T> {
    fn get(&self, index: Index) -> Option<Endpoint<T>> {
        match index {
            Index::OpReturnIndex => Some(self.by.by_opreturnindex()),
            _ => None,
        }
    }
}

/// Container for index endpoint methods.
pub struct MetricPattern16By<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: DeserializeOwned> MetricPattern16By<T> {
    pub fn by_txoutindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::TxOutIndex)
    }
}

/// Index accessor for metrics with 1 indexes.
pub struct MetricPattern16<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    pub by: MetricPattern16By<T>,
}

impl<T: DeserializeOwned> MetricPattern16<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self {
        let name: Arc<str> = name.into();
        Self {
            client: client.clone(),
            name: name.clone(),
            by: MetricPattern16By {
                client,
                name,
                _marker: std::marker::PhantomData,
            }
        }
    }

    /// Get the metric name.
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<T> AnyMetricPattern for MetricPattern16<T> {
    fn name(&self) -> &str {
        &self.name
    }

    fn indexes(&self) -> &'static [Index] {
        &[
            Index::TxOutIndex,
        ]
    }
}

impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern16<T> {
    fn get(&self, index: Index) -> Option<Endpoint<T>> {
        match index {
            Index::TxOutIndex => Some(self.by.by_txoutindex()),
            _ => None,
        }
    }
}

/// Container for index endpoint methods.
pub struct MetricPattern17By<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: DeserializeOwned> MetricPattern17By<T> {
    pub fn by_p2aaddressindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::P2AAddressIndex)
    }
}

/// Index accessor for metrics with 1 indexes.
pub struct MetricPattern17<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    pub by: MetricPattern17By<T>,
}

impl<T: DeserializeOwned> MetricPattern17<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self {
        let name: Arc<str> = name.into();
        Self {
            client: client.clone(),
            name: name.clone(),
            by: MetricPattern17By {
                client,
                name,
                _marker: std::marker::PhantomData,
            }
        }
    }

    /// Get the metric name.
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<T> AnyMetricPattern for MetricPattern17<T> {
    fn name(&self) -> &str {
        &self.name
    }

    fn indexes(&self) -> &'static [Index] {
        &[
            Index::P2AAddressIndex,
        ]
    }
}

impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern17<T> {
    fn get(&self, index: Index) -> Option<Endpoint<T>> {
        match index {
            Index::P2AAddressIndex => Some(self.by.by_p2aaddressindex()),
            _ => None,
        }
    }
}

/// Container for index endpoint methods.
pub struct MetricPattern18By<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: DeserializeOwned> MetricPattern18By<T> {
    pub fn by_p2msoutputindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::P2MSOutputIndex)
    }
}

/// Index accessor for metrics with 1 indexes.
pub struct MetricPattern18<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    pub by: MetricPattern18By<T>,
}

impl<T: DeserializeOwned> MetricPattern18<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self {
        let name: Arc<str> = name.into();
        Self {
            client: client.clone(),
            name: name.clone(),
            by: MetricPattern18By {
                client,
                name,
                _marker: std::marker::PhantomData,
            }
        }
    }

    /// Get the metric name.
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<T> AnyMetricPattern for MetricPattern18<T> {
    fn name(&self) -> &str {
        &self.name
    }

    fn indexes(&self) -> &'static [Index] {
        &[
            Index::P2MSOutputIndex,
        ]
    }
}

impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern18<T> {
    fn get(&self, index: Index) -> Option<Endpoint<T>> {
        match index {
            Index::P2MSOutputIndex => Some(self.by.by_p2msoutputindex()),
            _ => None,
        }
    }
}

/// Container for index endpoint methods.
pub struct MetricPattern19By<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: DeserializeOwned> MetricPattern19By<T> {
    pub fn by_p2pk33addressindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::P2PK33AddressIndex)
    }
}

/// Index accessor for metrics with 1 indexes.
pub struct MetricPattern19<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    pub by: MetricPattern19By<T>,
}

impl<T: DeserializeOwned> MetricPattern19<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self {
        let name: Arc<str> = name.into();
        Self {
            client: client.clone(),
            name: name.clone(),
            by: MetricPattern19By {
                client,
                name,
                _marker: std::marker::PhantomData,
            }
        }
    }

    /// Get the metric name.
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<T> AnyMetricPattern for MetricPattern19<T> {
    fn name(&self) -> &str {
        &self.name
    }

    fn indexes(&self) -> &'static [Index] {
        &[
            Index::P2PK33AddressIndex,
        ]
    }
}

impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern19<T> {
    fn get(&self, index: Index) -> Option<Endpoint<T>> {
        match index {
            Index::P2PK33AddressIndex => Some(self.by.by_p2pk33addressindex()),
            _ => None,
        }
    }
}

/// Container for index endpoint methods.
pub struct MetricPattern20By<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: DeserializeOwned> MetricPattern20By<T> {
    pub fn by_p2pk65addressindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::P2PK65AddressIndex)
    }
}

/// Index accessor for metrics with 1 indexes.
pub struct MetricPattern20<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    pub by: MetricPattern20By<T>,
}

impl<T: DeserializeOwned> MetricPattern20<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self {
        let name: Arc<str> = name.into();
        Self {
            client: client.clone(),
            name: name.clone(),
            by: MetricPattern20By {
                client,
                name,
                _marker: std::marker::PhantomData,
            }
        }
    }

    /// Get the metric name.
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<T> AnyMetricPattern for MetricPattern20<T> {
    fn name(&self) -> &str {
        &self.name
    }

    fn indexes(&self) -> &'static [Index] {
        &[
            Index::P2PK65AddressIndex,
        ]
    }
}

impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern20<T> {
    fn get(&self, index: Index) -> Option<Endpoint<T>> {
        match index {
            Index::P2PK65AddressIndex => Some(self.by.by_p2pk65addressindex()),
            _ => None,
        }
    }
}

/// Container for index endpoint methods.
pub struct MetricPattern21By<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: DeserializeOwned> MetricPattern21By<T> {
    pub fn by_p2pkhaddressindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::P2PKHAddressIndex)
    }
}

/// Index accessor for metrics with 1 indexes.
pub struct MetricPattern21<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    pub by: MetricPattern21By<T>,
}

impl<T: DeserializeOwned> MetricPattern21<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self {
        let name: Arc<str> = name.into();
        Self {
            client: client.clone(),
            name: name.clone(),
            by: MetricPattern21By {
                client,
                name,
                _marker: std::marker::PhantomData,
            }
        }
    }

    /// Get the metric name.
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<T> AnyMetricPattern for MetricPattern21<T> {
    fn name(&self) -> &str {
        &self.name
    }

    fn indexes(&self) -> &'static [Index] {
        &[
            Index::P2PKHAddressIndex,
        ]
    }
}

impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern21<T> {
    fn get(&self, index: Index) -> Option<Endpoint<T>> {
        match index {
            Index::P2PKHAddressIndex => Some(self.by.by_p2pkhaddressindex()),
            _ => None,
        }
    }
}

/// Container for index endpoint methods.
pub struct MetricPattern22By<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: DeserializeOwned> MetricPattern22By<T> {
    pub fn by_p2shaddressindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::P2SHAddressIndex)
    }
}

/// Index accessor for metrics with 1 indexes.
pub struct MetricPattern22<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    pub by: MetricPattern22By<T>,
}

impl<T: DeserializeOwned> MetricPattern22<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self {
        let name: Arc<str> = name.into();
        Self {
            client: client.clone(),
            name: name.clone(),
            by: MetricPattern22By {
                client,
                name,
                _marker: std::marker::PhantomData,
            }
        }
    }

    /// Get the metric name.
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<T> AnyMetricPattern for MetricPattern22<T> {
    fn name(&self) -> &str {
        &self.name
    }

    fn indexes(&self) -> &'static [Index] {
        &[
            Index::P2SHAddressIndex,
        ]
    }
}

impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern22<T> {
    fn get(&self, index: Index) -> Option<Endpoint<T>> {
        match index {
            Index::P2SHAddressIndex => Some(self.by.by_p2shaddressindex()),
            _ => None,
        }
    }
}

/// Container for index endpoint methods.
pub struct MetricPattern23By<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: DeserializeOwned> MetricPattern23By<T> {
    pub fn by_p2traddressindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::P2TRAddressIndex)
    }
}

/// Index accessor for metrics with 1 indexes.
pub struct MetricPattern23<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    pub by: MetricPattern23By<T>,
}

impl<T: DeserializeOwned> MetricPattern23<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self {
        let name: Arc<str> = name.into();
        Self {
            client: client.clone(),
            name: name.clone(),
            by: MetricPattern23By {
                client,
                name,
                _marker: std::marker::PhantomData,
            }
        }
    }

    /// Get the metric name.
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<T> AnyMetricPattern for MetricPattern23<T> {
    fn name(&self) -> &str {
        &self.name
    }

    fn indexes(&self) -> &'static [Index] {
        &[
            Index::P2TRAddressIndex,
        ]
    }
}

impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern23<T> {
    fn get(&self, index: Index) -> Option<Endpoint<T>> {
        match index {
            Index::P2TRAddressIndex => Some(self.by.by_p2traddressindex()),
            _ => None,
        }
    }
}

/// Container for index endpoint methods.
pub struct MetricPattern24By<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: DeserializeOwned> MetricPattern24By<T> {
    pub fn by_p2wpkhaddressindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::P2WPKHAddressIndex)
    }
}

/// Index accessor for metrics with 1 indexes.
pub struct MetricPattern24<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    pub by: MetricPattern24By<T>,
}

impl<T: DeserializeOwned> MetricPattern24<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self {
        let name: Arc<str> = name.into();
        Self {
            client: client.clone(),
            name: name.clone(),
            by: MetricPattern24By {
                client,
                name,
                _marker: std::marker::PhantomData,
            }
        }
    }

    /// Get the metric name.
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<T> AnyMetricPattern for MetricPattern24<T> {
    fn name(&self) -> &str {
        &self.name
    }

    fn indexes(&self) -> &'static [Index] {
        &[
            Index::P2WPKHAddressIndex,
        ]
    }
}

impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern24<T> {
    fn get(&self, index: Index) -> Option<Endpoint<T>> {
        match index {
            Index::P2WPKHAddressIndex => Some(self.by.by_p2wpkhaddressindex()),
            _ => None,
        }
    }
}

/// Container for index endpoint methods.
pub struct MetricPattern25By<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: DeserializeOwned> MetricPattern25By<T> {
    pub fn by_p2wshaddressindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::P2WSHAddressIndex)
    }
}

/// Index accessor for metrics with 1 indexes.
pub struct MetricPattern25<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    pub by: MetricPattern25By<T>,
}

impl<T: DeserializeOwned> MetricPattern25<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self {
        let name: Arc<str> = name.into();
        Self {
            client: client.clone(),
            name: name.clone(),
            by: MetricPattern25By {
                client,
                name,
                _marker: std::marker::PhantomData,
            }
        }
    }

    /// Get the metric name.
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<T> AnyMetricPattern for MetricPattern25<T> {
    fn name(&self) -> &str {
        &self.name
    }

    fn indexes(&self) -> &'static [Index] {
        &[
            Index::P2WSHAddressIndex,
        ]
    }
}

impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern25<T> {
    fn get(&self, index: Index) -> Option<Endpoint<T>> {
        match index {
            Index::P2WSHAddressIndex => Some(self.by.by_p2wshaddressindex()),
            _ => None,
        }
    }
}

/// Container for index endpoint methods.
pub struct MetricPattern26By<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: DeserializeOwned> MetricPattern26By<T> {
    pub fn by_quarterindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::QuarterIndex)
    }
}

/// Index accessor for metrics with 1 indexes.
pub struct MetricPattern26<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    pub by: MetricPattern26By<T>,
}

impl<T: DeserializeOwned> MetricPattern26<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self {
        let name: Arc<str> = name.into();
        Self {
            client: client.clone(),
            name: name.clone(),
            by: MetricPattern26By {
                client,
                name,
                _marker: std::marker::PhantomData,
            }
        }
    }

    /// Get the metric name.
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<T> AnyMetricPattern for MetricPattern26<T> {
    fn name(&self) -> &str {
        &self.name
    }

    fn indexes(&self) -> &'static [Index] {
        &[
            Index::QuarterIndex,
        ]
    }
}

impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern26<T> {
    fn get(&self, index: Index) -> Option<Endpoint<T>> {
        match index {
            Index::QuarterIndex => Some(self.by.by_quarterindex()),
            _ => None,
        }
    }
}

/// Container for index endpoint methods.
pub struct MetricPattern27By<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: DeserializeOwned> MetricPattern27By<T> {
    pub fn by_semesterindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::SemesterIndex)
    }
}

/// Index accessor for metrics with 1 indexes.
pub struct MetricPattern27<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    pub by: MetricPattern27By<T>,
}

impl<T: DeserializeOwned> MetricPattern27<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self {
        let name: Arc<str> = name.into();
        Self {
            client: client.clone(),
            name: name.clone(),
            by: MetricPattern27By {
                client,
                name,
                _marker: std::marker::PhantomData,
            }
        }
    }

    /// Get the metric name.
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<T> AnyMetricPattern for MetricPattern27<T> {
    fn name(&self) -> &str {
        &self.name
    }

    fn indexes(&self) -> &'static [Index] {
        &[
            Index::SemesterIndex,
        ]
    }
}

impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern27<T> {
    fn get(&self, index: Index) -> Option<Endpoint<T>> {
        match index {
            Index::SemesterIndex => Some(self.by.by_semesterindex()),
            _ => None,
        }
    }
}

/// Container for index endpoint methods.
pub struct MetricPattern28By<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: DeserializeOwned> MetricPattern28By<T> {
    pub fn by_txindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::TxIndex)
    }
}

/// Index accessor for metrics with 1 indexes.
pub struct MetricPattern28<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    pub by: MetricPattern28By<T>,
}

impl<T: DeserializeOwned> MetricPattern28<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self {
        let name: Arc<str> = name.into();
        Self {
            client: client.clone(),
            name: name.clone(),
            by: MetricPattern28By {
                client,
                name,
                _marker: std::marker::PhantomData,
            }
        }
    }

    /// Get the metric name.
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<T> AnyMetricPattern for MetricPattern28<T> {
    fn name(&self) -> &str {
        &self.name
    }

    fn indexes(&self) -> &'static [Index] {
        &[
            Index::TxIndex,
        ]
    }
}

impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern28<T> {
    fn get(&self, index: Index) -> Option<Endpoint<T>> {
        match index {
            Index::TxIndex => Some(self.by.by_txindex()),
            _ => None,
        }
    }
}

/// Container for index endpoint methods.
pub struct MetricPattern29By<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: DeserializeOwned> MetricPattern29By<T> {
    pub fn by_unknownoutputindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::UnknownOutputIndex)
    }
}

/// Index accessor for metrics with 1 indexes.
pub struct MetricPattern29<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    pub by: MetricPattern29By<T>,
}

impl<T: DeserializeOwned> MetricPattern29<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self {
        let name: Arc<str> = name.into();
        Self {
            client: client.clone(),
            name: name.clone(),
            by: MetricPattern29By {
                client,
                name,
                _marker: std::marker::PhantomData,
            }
        }
    }

    /// Get the metric name.
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<T> AnyMetricPattern for MetricPattern29<T> {
    fn name(&self) -> &str {
        &self.name
    }

    fn indexes(&self) -> &'static [Index] {
        &[
            Index::UnknownOutputIndex,
        ]
    }
}

impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern29<T> {
    fn get(&self, index: Index) -> Option<Endpoint<T>> {
        match index {
            Index::UnknownOutputIndex => Some(self.by.by_unknownoutputindex()),
            _ => None,
        }
    }
}

/// Container for index endpoint methods.
pub struct MetricPattern30By<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: DeserializeOwned> MetricPattern30By<T> {
    pub fn by_weekindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::WeekIndex)
    }
}

/// Index accessor for metrics with 1 indexes.
pub struct MetricPattern30<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    pub by: MetricPattern30By<T>,
}

impl<T: DeserializeOwned> MetricPattern30<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self {
        let name: Arc<str> = name.into();
        Self {
            client: client.clone(),
            name: name.clone(),
            by: MetricPattern30By {
                client,
                name,
                _marker: std::marker::PhantomData,
            }
        }
    }

    /// Get the metric name.
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<T> AnyMetricPattern for MetricPattern30<T> {
    fn name(&self) -> &str {
        &self.name
    }

    fn indexes(&self) -> &'static [Index] {
        &[
            Index::WeekIndex,
        ]
    }
}

impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern30<T> {
    fn get(&self, index: Index) -> Option<Endpoint<T>> {
        match index {
            Index::WeekIndex => Some(self.by.by_weekindex()),
            _ => None,
        }
    }
}

/// Container for index endpoint methods.
pub struct MetricPattern31By<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: DeserializeOwned> MetricPattern31By<T> {
    pub fn by_yearindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::YearIndex)
    }
}

/// Index accessor for metrics with 1 indexes.
pub struct MetricPattern31<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    pub by: MetricPattern31By<T>,
}

impl<T: DeserializeOwned> MetricPattern31<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self {
        let name: Arc<str> = name.into();
        Self {
            client: client.clone(),
            name: name.clone(),
            by: MetricPattern31By {
                client,
                name,
                _marker: std::marker::PhantomData,
            }
        }
    }

    /// Get the metric name.
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<T> AnyMetricPattern for MetricPattern31<T> {
    fn name(&self) -> &str {
        &self.name
    }

    fn indexes(&self) -> &'static [Index] {
        &[
            Index::YearIndex,
        ]
    }
}

impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern31<T> {
    fn get(&self, index: Index) -> Option<Endpoint<T>> {
        match index {
            Index::YearIndex => Some(self.by.by_yearindex()),
            _ => None,
        }
    }
}

/// Container for index endpoint methods.
pub struct MetricPattern32By<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: DeserializeOwned> MetricPattern32By<T> {
    pub fn by_loadedaddressindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::LoadedAddressIndex)
    }
}

/// Index accessor for metrics with 1 indexes.
pub struct MetricPattern32<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    pub by: MetricPattern32By<T>,
}

impl<T: DeserializeOwned> MetricPattern32<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self {
        let name: Arc<str> = name.into();
        Self {
            client: client.clone(),
            name: name.clone(),
            by: MetricPattern32By {
                client,
                name,
                _marker: std::marker::PhantomData,
            }
        }
    }

    /// Get the metric name.
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<T> AnyMetricPattern for MetricPattern32<T> {
    fn name(&self) -> &str {
        &self.name
    }

    fn indexes(&self) -> &'static [Index] {
        &[
            Index::LoadedAddressIndex,
        ]
    }
}

impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern32<T> {
    fn get(&self, index: Index) -> Option<Endpoint<T>> {
        match index {
            Index::LoadedAddressIndex => Some(self.by.by_loadedaddressindex()),
            _ => None,
        }
    }
}

/// Container for index endpoint methods.
pub struct MetricPattern33By<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: DeserializeOwned> MetricPattern33By<T> {
    pub fn by_emptyaddressindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::EmptyAddressIndex)
    }
}

/// Index accessor for metrics with 1 indexes.
pub struct MetricPattern33<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    pub by: MetricPattern33By<T>,
}

impl<T: DeserializeOwned> MetricPattern33<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self {
        let name: Arc<str> = name.into();
        Self {
            client: client.clone(),
            name: name.clone(),
            by: MetricPattern33By {
                client,
                name,
                _marker: std::marker::PhantomData,
            }
        }
    }

    /// Get the metric name.
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<T> AnyMetricPattern for MetricPattern33<T> {
    fn name(&self) -> &str {
        &self.name
    }

    fn indexes(&self) -> &'static [Index] {
        &[
            Index::EmptyAddressIndex,
        ]
    }
}

impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern33<T> {
    fn get(&self, index: Index) -> Option<Endpoint<T>> {
        match index {
            Index::EmptyAddressIndex => Some(self.by.by_emptyaddressindex()),
            _ => None,
        }
    }
}

// Reusable pattern structs

/// Pattern struct for repeated tree structure.
pub struct RealizedPattern3 {
    pub adjusted_sopr: MetricPattern7<StoredF64>,
    pub adjusted_sopr_30d_ema: MetricPattern7<StoredF64>,
    pub adjusted_sopr_7d_ema: MetricPattern7<StoredF64>,
    pub adjusted_value_created: MetricPattern1<Dollars>,
    pub adjusted_value_destroyed: MetricPattern1<Dollars>,
    pub mvrv: MetricPattern4<StoredF32>,
    pub neg_realized_loss: BlockCountPattern<Dollars>,
    pub net_realized_pnl: BlockCountPattern<Dollars>,
    pub net_realized_pnl_cumulative_30d_delta: MetricPattern4<Dollars>,
    pub net_realized_pnl_cumulative_30d_delta_rel_to_market_cap: MetricPattern4<StoredF32>,
    pub net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap: MetricPattern4<StoredF32>,
    pub net_realized_pnl_rel_to_realized_cap: BlockCountPattern<StoredF32>,
    pub realized_cap: MetricPattern1<Dollars>,
    pub realized_cap_30d_delta: MetricPattern4<Dollars>,
    pub realized_cap_rel_to_own_market_cap: MetricPattern1<StoredF32>,
    pub realized_loss: BlockCountPattern<Dollars>,
    pub realized_loss_rel_to_realized_cap: BlockCountPattern<StoredF32>,
    pub realized_price: MetricPattern1<Dollars>,
    pub realized_price_extra: ActivePriceRatioPattern,
    pub realized_profit: BlockCountPattern<Dollars>,
    pub realized_profit_rel_to_realized_cap: BlockCountPattern<StoredF32>,
    pub realized_profit_to_loss_ratio: MetricPattern7<StoredF64>,
    pub realized_value: MetricPattern1<Dollars>,
    pub sell_side_risk_ratio: MetricPattern7<StoredF32>,
    pub sell_side_risk_ratio_30d_ema: MetricPattern7<StoredF32>,
    pub sell_side_risk_ratio_7d_ema: MetricPattern7<StoredF32>,
    pub sopr: MetricPattern7<StoredF64>,
    pub sopr_30d_ema: MetricPattern7<StoredF64>,
    pub sopr_7d_ema: MetricPattern7<StoredF64>,
    pub total_realized_pnl: MetricPattern1<Dollars>,
    pub value_created: MetricPattern1<Dollars>,
    pub value_destroyed: MetricPattern1<Dollars>,
}

impl RealizedPattern3 {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            adjusted_sopr: MetricPattern7::new(client.clone(), _m(&acc, "adjusted_sopr")),
            adjusted_sopr_30d_ema: MetricPattern7::new(client.clone(), _m(&acc, "adjusted_sopr_30d_ema")),
            adjusted_sopr_7d_ema: MetricPattern7::new(client.clone(), _m(&acc, "adjusted_sopr_7d_ema")),
            adjusted_value_created: MetricPattern1::new(client.clone(), _m(&acc, "adjusted_value_created")),
            adjusted_value_destroyed: MetricPattern1::new(client.clone(), _m(&acc, "adjusted_value_destroyed")),
            mvrv: MetricPattern4::new(client.clone(), _m(&acc, "mvrv")),
            neg_realized_loss: BlockCountPattern::new(client.clone(), _m(&acc, "neg_realized_loss")),
            net_realized_pnl: BlockCountPattern::new(client.clone(), _m(&acc, "net_realized_pnl")),
            net_realized_pnl_cumulative_30d_delta: MetricPattern4::new(client.clone(), _m(&acc, "net_realized_pnl_cumulative_30d_delta")),
            net_realized_pnl_cumulative_30d_delta_rel_to_market_cap: MetricPattern4::new(client.clone(), _m(&acc, "net_realized_pnl_cumulative_30d_delta_rel_to_market_cap")),
            net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap: MetricPattern4::new(client.clone(), _m(&acc, "net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap")),
            net_realized_pnl_rel_to_realized_cap: BlockCountPattern::new(client.clone(), _m(&acc, "net_realized_pnl_rel_to_realized_cap")),
            realized_cap: MetricPattern1::new(client.clone(), _m(&acc, "realized_cap")),
            realized_cap_30d_delta: MetricPattern4::new(client.clone(), _m(&acc, "realized_cap_30d_delta")),
            realized_cap_rel_to_own_market_cap: MetricPattern1::new(client.clone(), _m(&acc, "realized_cap_rel_to_own_market_cap")),
            realized_loss: BlockCountPattern::new(client.clone(), _m(&acc, "realized_loss")),
            realized_loss_rel_to_realized_cap: BlockCountPattern::new(client.clone(), _m(&acc, "realized_loss_rel_to_realized_cap")),
            realized_price: MetricPattern1::new(client.clone(), _m(&acc, "realized_price")),
            realized_price_extra: ActivePriceRatioPattern::new(client.clone(), _m(&acc, "realized_price_ratio")),
            realized_profit: BlockCountPattern::new(client.clone(), _m(&acc, "realized_profit")),
            realized_profit_rel_to_realized_cap: BlockCountPattern::new(client.clone(), _m(&acc, "realized_profit_rel_to_realized_cap")),
            realized_profit_to_loss_ratio: MetricPattern7::new(client.clone(), _m(&acc, "realized_profit_to_loss_ratio")),
            realized_value: MetricPattern1::new(client.clone(), _m(&acc, "realized_value")),
            sell_side_risk_ratio: MetricPattern7::new(client.clone(), _m(&acc, "sell_side_risk_ratio")),
            sell_side_risk_ratio_30d_ema: MetricPattern7::new(client.clone(), _m(&acc, "sell_side_risk_ratio_30d_ema")),
            sell_side_risk_ratio_7d_ema: MetricPattern7::new(client.clone(), _m(&acc, "sell_side_risk_ratio_7d_ema")),
            sopr: MetricPattern7::new(client.clone(), _m(&acc, "sopr")),
            sopr_30d_ema: MetricPattern7::new(client.clone(), _m(&acc, "sopr_30d_ema")),
            sopr_7d_ema: MetricPattern7::new(client.clone(), _m(&acc, "sopr_7d_ema")),
            total_realized_pnl: MetricPattern1::new(client.clone(), _m(&acc, "total_realized_pnl")),
            value_created: MetricPattern1::new(client.clone(), _m(&acc, "value_created")),
            value_destroyed: MetricPattern1::new(client.clone(), _m(&acc, "value_destroyed")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct RealizedPattern4 {
    pub adjusted_sopr: MetricPattern7<StoredF64>,
    pub adjusted_sopr_30d_ema: MetricPattern7<StoredF64>,
    pub adjusted_sopr_7d_ema: MetricPattern7<StoredF64>,
    pub adjusted_value_created: MetricPattern1<Dollars>,
    pub adjusted_value_destroyed: MetricPattern1<Dollars>,
    pub mvrv: MetricPattern4<StoredF32>,
    pub neg_realized_loss: BlockCountPattern<Dollars>,
    pub net_realized_pnl: BlockCountPattern<Dollars>,
    pub net_realized_pnl_cumulative_30d_delta: MetricPattern4<Dollars>,
    pub net_realized_pnl_cumulative_30d_delta_rel_to_market_cap: MetricPattern4<StoredF32>,
    pub net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap: MetricPattern4<StoredF32>,
    pub net_realized_pnl_rel_to_realized_cap: BlockCountPattern<StoredF32>,
    pub realized_cap: MetricPattern1<Dollars>,
    pub realized_cap_30d_delta: MetricPattern4<Dollars>,
    pub realized_loss: BlockCountPattern<Dollars>,
    pub realized_loss_rel_to_realized_cap: BlockCountPattern<StoredF32>,
    pub realized_price: MetricPattern1<Dollars>,
    pub realized_price_extra: RealizedPriceExtraPattern,
    pub realized_profit: BlockCountPattern<Dollars>,
    pub realized_profit_rel_to_realized_cap: BlockCountPattern<StoredF32>,
    pub realized_value: MetricPattern1<Dollars>,
    pub sell_side_risk_ratio: MetricPattern7<StoredF32>,
    pub sell_side_risk_ratio_30d_ema: MetricPattern7<StoredF32>,
    pub sell_side_risk_ratio_7d_ema: MetricPattern7<StoredF32>,
    pub sopr: MetricPattern7<StoredF64>,
    pub sopr_30d_ema: MetricPattern7<StoredF64>,
    pub sopr_7d_ema: MetricPattern7<StoredF64>,
    pub total_realized_pnl: MetricPattern1<Dollars>,
    pub value_created: MetricPattern1<Dollars>,
    pub value_destroyed: MetricPattern1<Dollars>,
}

impl RealizedPattern4 {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            adjusted_sopr: MetricPattern7::new(client.clone(), _m(&acc, "adjusted_sopr")),
            adjusted_sopr_30d_ema: MetricPattern7::new(client.clone(), _m(&acc, "adjusted_sopr_30d_ema")),
            adjusted_sopr_7d_ema: MetricPattern7::new(client.clone(), _m(&acc, "adjusted_sopr_7d_ema")),
            adjusted_value_created: MetricPattern1::new(client.clone(), _m(&acc, "adjusted_value_created")),
            adjusted_value_destroyed: MetricPattern1::new(client.clone(), _m(&acc, "adjusted_value_destroyed")),
            mvrv: MetricPattern4::new(client.clone(), _m(&acc, "mvrv")),
            neg_realized_loss: BlockCountPattern::new(client.clone(), _m(&acc, "neg_realized_loss")),
            net_realized_pnl: BlockCountPattern::new(client.clone(), _m(&acc, "net_realized_pnl")),
            net_realized_pnl_cumulative_30d_delta: MetricPattern4::new(client.clone(), _m(&acc, "net_realized_pnl_cumulative_30d_delta")),
            net_realized_pnl_cumulative_30d_delta_rel_to_market_cap: MetricPattern4::new(client.clone(), _m(&acc, "net_realized_pnl_cumulative_30d_delta_rel_to_market_cap")),
            net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap: MetricPattern4::new(client.clone(), _m(&acc, "net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap")),
            net_realized_pnl_rel_to_realized_cap: BlockCountPattern::new(client.clone(), _m(&acc, "net_realized_pnl_rel_to_realized_cap")),
            realized_cap: MetricPattern1::new(client.clone(), _m(&acc, "realized_cap")),
            realized_cap_30d_delta: MetricPattern4::new(client.clone(), _m(&acc, "realized_cap_30d_delta")),
            realized_loss: BlockCountPattern::new(client.clone(), _m(&acc, "realized_loss")),
            realized_loss_rel_to_realized_cap: BlockCountPattern::new(client.clone(), _m(&acc, "realized_loss_rel_to_realized_cap")),
            realized_price: MetricPattern1::new(client.clone(), _m(&acc, "realized_price")),
            realized_price_extra: RealizedPriceExtraPattern::new(client.clone(), _m(&acc, "realized_price")),
            realized_profit: BlockCountPattern::new(client.clone(), _m(&acc, "realized_profit")),
            realized_profit_rel_to_realized_cap: BlockCountPattern::new(client.clone(), _m(&acc, "realized_profit_rel_to_realized_cap")),
            realized_value: MetricPattern1::new(client.clone(), _m(&acc, "realized_value")),
            sell_side_risk_ratio: MetricPattern7::new(client.clone(), _m(&acc, "sell_side_risk_ratio")),
            sell_side_risk_ratio_30d_ema: MetricPattern7::new(client.clone(), _m(&acc, "sell_side_risk_ratio_30d_ema")),
            sell_side_risk_ratio_7d_ema: MetricPattern7::new(client.clone(), _m(&acc, "sell_side_risk_ratio_7d_ema")),
            sopr: MetricPattern7::new(client.clone(), _m(&acc, "sopr")),
            sopr_30d_ema: MetricPattern7::new(client.clone(), _m(&acc, "sopr_30d_ema")),
            sopr_7d_ema: MetricPattern7::new(client.clone(), _m(&acc, "sopr_7d_ema")),
            total_realized_pnl: MetricPattern1::new(client.clone(), _m(&acc, "total_realized_pnl")),
            value_created: MetricPattern1::new(client.clone(), _m(&acc, "value_created")),
            value_destroyed: MetricPattern1::new(client.clone(), _m(&acc, "value_destroyed")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct Ratio1ySdPattern {
    pub _0sd_usd: MetricPattern4<Dollars>,
    pub m0_5sd: MetricPattern4<StoredF32>,
    pub m0_5sd_usd: MetricPattern4<Dollars>,
    pub m1_5sd: MetricPattern4<StoredF32>,
    pub m1_5sd_usd: MetricPattern4<Dollars>,
    pub m1sd: MetricPattern4<StoredF32>,
    pub m1sd_usd: MetricPattern4<Dollars>,
    pub m2_5sd: MetricPattern4<StoredF32>,
    pub m2_5sd_usd: MetricPattern4<Dollars>,
    pub m2sd: MetricPattern4<StoredF32>,
    pub m2sd_usd: MetricPattern4<Dollars>,
    pub m3sd: MetricPattern4<StoredF32>,
    pub m3sd_usd: MetricPattern4<Dollars>,
    pub p0_5sd: MetricPattern4<StoredF32>,
    pub p0_5sd_usd: MetricPattern4<Dollars>,
    pub p1_5sd: MetricPattern4<StoredF32>,
    pub p1_5sd_usd: MetricPattern4<Dollars>,
    pub p1sd: MetricPattern4<StoredF32>,
    pub p1sd_usd: MetricPattern4<Dollars>,
    pub p2_5sd: MetricPattern4<StoredF32>,
    pub p2_5sd_usd: MetricPattern4<Dollars>,
    pub p2sd: MetricPattern4<StoredF32>,
    pub p2sd_usd: MetricPattern4<Dollars>,
    pub p3sd: MetricPattern4<StoredF32>,
    pub p3sd_usd: MetricPattern4<Dollars>,
    pub sd: MetricPattern4<StoredF32>,
    pub sma: MetricPattern4<StoredF32>,
    pub zscore: MetricPattern4<StoredF32>,
}

impl Ratio1ySdPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            _0sd_usd: MetricPattern4::new(client.clone(), _m(&acc, "0sd_usd")),
            m0_5sd: MetricPattern4::new(client.clone(), _m(&acc, "m0_5sd")),
            m0_5sd_usd: MetricPattern4::new(client.clone(), _m(&acc, "m0_5sd_usd")),
            m1_5sd: MetricPattern4::new(client.clone(), _m(&acc, "m1_5sd")),
            m1_5sd_usd: MetricPattern4::new(client.clone(), _m(&acc, "m1_5sd_usd")),
            m1sd: MetricPattern4::new(client.clone(), _m(&acc, "m1sd")),
            m1sd_usd: MetricPattern4::new(client.clone(), _m(&acc, "m1sd_usd")),
            m2_5sd: MetricPattern4::new(client.clone(), _m(&acc, "m2_5sd")),
            m2_5sd_usd: MetricPattern4::new(client.clone(), _m(&acc, "m2_5sd_usd")),
            m2sd: MetricPattern4::new(client.clone(), _m(&acc, "m2sd")),
            m2sd_usd: MetricPattern4::new(client.clone(), _m(&acc, "m2sd_usd")),
            m3sd: MetricPattern4::new(client.clone(), _m(&acc, "m3sd")),
            m3sd_usd: MetricPattern4::new(client.clone(), _m(&acc, "m3sd_usd")),
            p0_5sd: MetricPattern4::new(client.clone(), _m(&acc, "p0_5sd")),
            p0_5sd_usd: MetricPattern4::new(client.clone(), _m(&acc, "p0_5sd_usd")),
            p1_5sd: MetricPattern4::new(client.clone(), _m(&acc, "p1_5sd")),
            p1_5sd_usd: MetricPattern4::new(client.clone(), _m(&acc, "p1_5sd_usd")),
            p1sd: MetricPattern4::new(client.clone(), _m(&acc, "p1sd")),
            p1sd_usd: MetricPattern4::new(client.clone(), _m(&acc, "p1sd_usd")),
            p2_5sd: MetricPattern4::new(client.clone(), _m(&acc, "p2_5sd")),
            p2_5sd_usd: MetricPattern4::new(client.clone(), _m(&acc, "p2_5sd_usd")),
            p2sd: MetricPattern4::new(client.clone(), _m(&acc, "p2sd")),
            p2sd_usd: MetricPattern4::new(client.clone(), _m(&acc, "p2sd_usd")),
            p3sd: MetricPattern4::new(client.clone(), _m(&acc, "p3sd")),
            p3sd_usd: MetricPattern4::new(client.clone(), _m(&acc, "p3sd_usd")),
            sd: MetricPattern4::new(client.clone(), _m(&acc, "sd")),
            sma: MetricPattern4::new(client.clone(), _m(&acc, "sma")),
            zscore: MetricPattern4::new(client.clone(), _m(&acc, "zscore")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct RealizedPattern2 {
    pub mvrv: MetricPattern4<StoredF32>,
    pub neg_realized_loss: BlockCountPattern<Dollars>,
    pub net_realized_pnl: BlockCountPattern<Dollars>,
    pub net_realized_pnl_cumulative_30d_delta: MetricPattern4<Dollars>,
    pub net_realized_pnl_cumulative_30d_delta_rel_to_market_cap: MetricPattern4<StoredF32>,
    pub net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap: MetricPattern4<StoredF32>,
    pub net_realized_pnl_rel_to_realized_cap: BlockCountPattern<StoredF32>,
    pub realized_cap: MetricPattern1<Dollars>,
    pub realized_cap_30d_delta: MetricPattern4<Dollars>,
    pub realized_cap_rel_to_own_market_cap: MetricPattern1<StoredF32>,
    pub realized_loss: BlockCountPattern<Dollars>,
    pub realized_loss_rel_to_realized_cap: BlockCountPattern<StoredF32>,
    pub realized_price: MetricPattern1<Dollars>,
    pub realized_price_extra: ActivePriceRatioPattern,
    pub realized_profit: BlockCountPattern<Dollars>,
    pub realized_profit_rel_to_realized_cap: BlockCountPattern<StoredF32>,
    pub realized_profit_to_loss_ratio: MetricPattern7<StoredF64>,
    pub realized_value: MetricPattern1<Dollars>,
    pub sell_side_risk_ratio: MetricPattern7<StoredF32>,
    pub sell_side_risk_ratio_30d_ema: MetricPattern7<StoredF32>,
    pub sell_side_risk_ratio_7d_ema: MetricPattern7<StoredF32>,
    pub sopr: MetricPattern7<StoredF64>,
    pub sopr_30d_ema: MetricPattern7<StoredF64>,
    pub sopr_7d_ema: MetricPattern7<StoredF64>,
    pub total_realized_pnl: MetricPattern1<Dollars>,
    pub value_created: MetricPattern1<Dollars>,
    pub value_destroyed: MetricPattern1<Dollars>,
}

impl RealizedPattern2 {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            mvrv: MetricPattern4::new(client.clone(), _m(&acc, "mvrv")),
            neg_realized_loss: BlockCountPattern::new(client.clone(), _m(&acc, "neg_realized_loss")),
            net_realized_pnl: BlockCountPattern::new(client.clone(), _m(&acc, "net_realized_pnl")),
            net_realized_pnl_cumulative_30d_delta: MetricPattern4::new(client.clone(), _m(&acc, "net_realized_pnl_cumulative_30d_delta")),
            net_realized_pnl_cumulative_30d_delta_rel_to_market_cap: MetricPattern4::new(client.clone(), _m(&acc, "net_realized_pnl_cumulative_30d_delta_rel_to_market_cap")),
            net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap: MetricPattern4::new(client.clone(), _m(&acc, "net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap")),
            net_realized_pnl_rel_to_realized_cap: BlockCountPattern::new(client.clone(), _m(&acc, "net_realized_pnl_rel_to_realized_cap")),
            realized_cap: MetricPattern1::new(client.clone(), _m(&acc, "realized_cap")),
            realized_cap_30d_delta: MetricPattern4::new(client.clone(), _m(&acc, "realized_cap_30d_delta")),
            realized_cap_rel_to_own_market_cap: MetricPattern1::new(client.clone(), _m(&acc, "realized_cap_rel_to_own_market_cap")),
            realized_loss: BlockCountPattern::new(client.clone(), _m(&acc, "realized_loss")),
            realized_loss_rel_to_realized_cap: BlockCountPattern::new(client.clone(), _m(&acc, "realized_loss_rel_to_realized_cap")),
            realized_price: MetricPattern1::new(client.clone(), _m(&acc, "realized_price")),
            realized_price_extra: ActivePriceRatioPattern::new(client.clone(), _m(&acc, "realized_price_ratio")),
            realized_profit: BlockCountPattern::new(client.clone(), _m(&acc, "realized_profit")),
            realized_profit_rel_to_realized_cap: BlockCountPattern::new(client.clone(), _m(&acc, "realized_profit_rel_to_realized_cap")),
            realized_profit_to_loss_ratio: MetricPattern7::new(client.clone(), _m(&acc, "realized_profit_to_loss_ratio")),
            realized_value: MetricPattern1::new(client.clone(), _m(&acc, "realized_value")),
            sell_side_risk_ratio: MetricPattern7::new(client.clone(), _m(&acc, "sell_side_risk_ratio")),
            sell_side_risk_ratio_30d_ema: MetricPattern7::new(client.clone(), _m(&acc, "sell_side_risk_ratio_30d_ema")),
            sell_side_risk_ratio_7d_ema: MetricPattern7::new(client.clone(), _m(&acc, "sell_side_risk_ratio_7d_ema")),
            sopr: MetricPattern7::new(client.clone(), _m(&acc, "sopr")),
            sopr_30d_ema: MetricPattern7::new(client.clone(), _m(&acc, "sopr_30d_ema")),
            sopr_7d_ema: MetricPattern7::new(client.clone(), _m(&acc, "sopr_7d_ema")),
            total_realized_pnl: MetricPattern1::new(client.clone(), _m(&acc, "total_realized_pnl")),
            value_created: MetricPattern1::new(client.clone(), _m(&acc, "value_created")),
            value_destroyed: MetricPattern1::new(client.clone(), _m(&acc, "value_destroyed")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct RealizedPattern {
    pub mvrv: MetricPattern4<StoredF32>,
    pub neg_realized_loss: BlockCountPattern<Dollars>,
    pub net_realized_pnl: BlockCountPattern<Dollars>,
    pub net_realized_pnl_cumulative_30d_delta: MetricPattern4<Dollars>,
    pub net_realized_pnl_cumulative_30d_delta_rel_to_market_cap: MetricPattern4<StoredF32>,
    pub net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap: MetricPattern4<StoredF32>,
    pub net_realized_pnl_rel_to_realized_cap: BlockCountPattern<StoredF32>,
    pub realized_cap: MetricPattern1<Dollars>,
    pub realized_cap_30d_delta: MetricPattern4<Dollars>,
    pub realized_loss: BlockCountPattern<Dollars>,
    pub realized_loss_rel_to_realized_cap: BlockCountPattern<StoredF32>,
    pub realized_price: MetricPattern1<Dollars>,
    pub realized_price_extra: RealizedPriceExtraPattern,
    pub realized_profit: BlockCountPattern<Dollars>,
    pub realized_profit_rel_to_realized_cap: BlockCountPattern<StoredF32>,
    pub realized_value: MetricPattern1<Dollars>,
    pub sell_side_risk_ratio: MetricPattern7<StoredF32>,
    pub sell_side_risk_ratio_30d_ema: MetricPattern7<StoredF32>,
    pub sell_side_risk_ratio_7d_ema: MetricPattern7<StoredF32>,
    pub sopr: MetricPattern7<StoredF64>,
    pub sopr_30d_ema: MetricPattern7<StoredF64>,
    pub sopr_7d_ema: MetricPattern7<StoredF64>,
    pub total_realized_pnl: MetricPattern1<Dollars>,
    pub value_created: MetricPattern1<Dollars>,
    pub value_destroyed: MetricPattern1<Dollars>,
}

impl RealizedPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            mvrv: MetricPattern4::new(client.clone(), _m(&acc, "mvrv")),
            neg_realized_loss: BlockCountPattern::new(client.clone(), _m(&acc, "neg_realized_loss")),
            net_realized_pnl: BlockCountPattern::new(client.clone(), _m(&acc, "net_realized_pnl")),
            net_realized_pnl_cumulative_30d_delta: MetricPattern4::new(client.clone(), _m(&acc, "net_realized_pnl_cumulative_30d_delta")),
            net_realized_pnl_cumulative_30d_delta_rel_to_market_cap: MetricPattern4::new(client.clone(), _m(&acc, "net_realized_pnl_cumulative_30d_delta_rel_to_market_cap")),
            net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap: MetricPattern4::new(client.clone(), _m(&acc, "net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap")),
            net_realized_pnl_rel_to_realized_cap: BlockCountPattern::new(client.clone(), _m(&acc, "net_realized_pnl_rel_to_realized_cap")),
            realized_cap: MetricPattern1::new(client.clone(), _m(&acc, "realized_cap")),
            realized_cap_30d_delta: MetricPattern4::new(client.clone(), _m(&acc, "realized_cap_30d_delta")),
            realized_loss: BlockCountPattern::new(client.clone(), _m(&acc, "realized_loss")),
            realized_loss_rel_to_realized_cap: BlockCountPattern::new(client.clone(), _m(&acc, "realized_loss_rel_to_realized_cap")),
            realized_price: MetricPattern1::new(client.clone(), _m(&acc, "realized_price")),
            realized_price_extra: RealizedPriceExtraPattern::new(client.clone(), _m(&acc, "realized_price")),
            realized_profit: BlockCountPattern::new(client.clone(), _m(&acc, "realized_profit")),
            realized_profit_rel_to_realized_cap: BlockCountPattern::new(client.clone(), _m(&acc, "realized_profit_rel_to_realized_cap")),
            realized_value: MetricPattern1::new(client.clone(), _m(&acc, "realized_value")),
            sell_side_risk_ratio: MetricPattern7::new(client.clone(), _m(&acc, "sell_side_risk_ratio")),
            sell_side_risk_ratio_30d_ema: MetricPattern7::new(client.clone(), _m(&acc, "sell_side_risk_ratio_30d_ema")),
            sell_side_risk_ratio_7d_ema: MetricPattern7::new(client.clone(), _m(&acc, "sell_side_risk_ratio_7d_ema")),
            sopr: MetricPattern7::new(client.clone(), _m(&acc, "sopr")),
            sopr_30d_ema: MetricPattern7::new(client.clone(), _m(&acc, "sopr_30d_ema")),
            sopr_7d_ema: MetricPattern7::new(client.clone(), _m(&acc, "sopr_7d_ema")),
            total_realized_pnl: MetricPattern1::new(client.clone(), _m(&acc, "total_realized_pnl")),
            value_created: MetricPattern1::new(client.clone(), _m(&acc, "value_created")),
            value_destroyed: MetricPattern1::new(client.clone(), _m(&acc, "value_destroyed")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct Price111dSmaPattern {
    pub price: MetricPattern4<Dollars>,
    pub ratio: MetricPattern4<StoredF32>,
    pub ratio_1m_sma: MetricPattern4<StoredF32>,
    pub ratio_1w_sma: MetricPattern4<StoredF32>,
    pub ratio_1y_sd: Ratio1ySdPattern,
    pub ratio_2y_sd: Ratio1ySdPattern,
    pub ratio_4y_sd: Ratio1ySdPattern,
    pub ratio_pct1: MetricPattern4<StoredF32>,
    pub ratio_pct1_usd: MetricPattern4<Dollars>,
    pub ratio_pct2: MetricPattern4<StoredF32>,
    pub ratio_pct2_usd: MetricPattern4<Dollars>,
    pub ratio_pct5: MetricPattern4<StoredF32>,
    pub ratio_pct5_usd: MetricPattern4<Dollars>,
    pub ratio_pct95: MetricPattern4<StoredF32>,
    pub ratio_pct95_usd: MetricPattern4<Dollars>,
    pub ratio_pct98: MetricPattern4<StoredF32>,
    pub ratio_pct98_usd: MetricPattern4<Dollars>,
    pub ratio_pct99: MetricPattern4<StoredF32>,
    pub ratio_pct99_usd: MetricPattern4<Dollars>,
    pub ratio_sd: Ratio1ySdPattern,
}

impl Price111dSmaPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            price: MetricPattern4::new(client.clone(), acc.clone()),
            ratio: MetricPattern4::new(client.clone(), _m(&acc, "ratio")),
            ratio_1m_sma: MetricPattern4::new(client.clone(), _m(&acc, "ratio_1m_sma")),
            ratio_1w_sma: MetricPattern4::new(client.clone(), _m(&acc, "ratio_1w_sma")),
            ratio_1y_sd: Ratio1ySdPattern::new(client.clone(), _m(&acc, "ratio_1y")),
            ratio_2y_sd: Ratio1ySdPattern::new(client.clone(), _m(&acc, "ratio_2y")),
            ratio_4y_sd: Ratio1ySdPattern::new(client.clone(), _m(&acc, "ratio_4y")),
            ratio_pct1: MetricPattern4::new(client.clone(), _m(&acc, "ratio_pct1")),
            ratio_pct1_usd: MetricPattern4::new(client.clone(), _m(&acc, "ratio_pct1_usd")),
            ratio_pct2: MetricPattern4::new(client.clone(), _m(&acc, "ratio_pct2")),
            ratio_pct2_usd: MetricPattern4::new(client.clone(), _m(&acc, "ratio_pct2_usd")),
            ratio_pct5: MetricPattern4::new(client.clone(), _m(&acc, "ratio_pct5")),
            ratio_pct5_usd: MetricPattern4::new(client.clone(), _m(&acc, "ratio_pct5_usd")),
            ratio_pct95: MetricPattern4::new(client.clone(), _m(&acc, "ratio_pct95")),
            ratio_pct95_usd: MetricPattern4::new(client.clone(), _m(&acc, "ratio_pct95_usd")),
            ratio_pct98: MetricPattern4::new(client.clone(), _m(&acc, "ratio_pct98")),
            ratio_pct98_usd: MetricPattern4::new(client.clone(), _m(&acc, "ratio_pct98_usd")),
            ratio_pct99: MetricPattern4::new(client.clone(), _m(&acc, "ratio_pct99")),
            ratio_pct99_usd: MetricPattern4::new(client.clone(), _m(&acc, "ratio_pct99_usd")),
            ratio_sd: Ratio1ySdPattern::new(client.clone(), _m(&acc, "ratio")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct ActivePriceRatioPattern {
    pub ratio: MetricPattern4<StoredF32>,
    pub ratio_1m_sma: MetricPattern4<StoredF32>,
    pub ratio_1w_sma: MetricPattern4<StoredF32>,
    pub ratio_1y_sd: Ratio1ySdPattern,
    pub ratio_2y_sd: Ratio1ySdPattern,
    pub ratio_4y_sd: Ratio1ySdPattern,
    pub ratio_pct1: MetricPattern4<StoredF32>,
    pub ratio_pct1_usd: MetricPattern4<Dollars>,
    pub ratio_pct2: MetricPattern4<StoredF32>,
    pub ratio_pct2_usd: MetricPattern4<Dollars>,
    pub ratio_pct5: MetricPattern4<StoredF32>,
    pub ratio_pct5_usd: MetricPattern4<Dollars>,
    pub ratio_pct95: MetricPattern4<StoredF32>,
    pub ratio_pct95_usd: MetricPattern4<Dollars>,
    pub ratio_pct98: MetricPattern4<StoredF32>,
    pub ratio_pct98_usd: MetricPattern4<Dollars>,
    pub ratio_pct99: MetricPattern4<StoredF32>,
    pub ratio_pct99_usd: MetricPattern4<Dollars>,
    pub ratio_sd: Ratio1ySdPattern,
}

impl ActivePriceRatioPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            ratio: MetricPattern4::new(client.clone(), acc.clone()),
            ratio_1m_sma: MetricPattern4::new(client.clone(), _m(&acc, "1m_sma")),
            ratio_1w_sma: MetricPattern4::new(client.clone(), _m(&acc, "1w_sma")),
            ratio_1y_sd: Ratio1ySdPattern::new(client.clone(), _m(&acc, "1y")),
            ratio_2y_sd: Ratio1ySdPattern::new(client.clone(), _m(&acc, "2y")),
            ratio_4y_sd: Ratio1ySdPattern::new(client.clone(), _m(&acc, "4y")),
            ratio_pct1: MetricPattern4::new(client.clone(), _m(&acc, "pct1")),
            ratio_pct1_usd: MetricPattern4::new(client.clone(), _m(&acc, "pct1_usd")),
            ratio_pct2: MetricPattern4::new(client.clone(), _m(&acc, "pct2")),
            ratio_pct2_usd: MetricPattern4::new(client.clone(), _m(&acc, "pct2_usd")),
            ratio_pct5: MetricPattern4::new(client.clone(), _m(&acc, "pct5")),
            ratio_pct5_usd: MetricPattern4::new(client.clone(), _m(&acc, "pct5_usd")),
            ratio_pct95: MetricPattern4::new(client.clone(), _m(&acc, "pct95")),
            ratio_pct95_usd: MetricPattern4::new(client.clone(), _m(&acc, "pct95_usd")),
            ratio_pct98: MetricPattern4::new(client.clone(), _m(&acc, "pct98")),
            ratio_pct98_usd: MetricPattern4::new(client.clone(), _m(&acc, "pct98_usd")),
            ratio_pct99: MetricPattern4::new(client.clone(), _m(&acc, "pct99")),
            ratio_pct99_usd: MetricPattern4::new(client.clone(), _m(&acc, "pct99_usd")),
            ratio_sd: Ratio1ySdPattern::new(client.clone(), acc.clone()),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct PercentilesPattern2 {
    pub cost_basis_pct05: MetricPattern4<Dollars>,
    pub cost_basis_pct10: MetricPattern4<Dollars>,
    pub cost_basis_pct15: MetricPattern4<Dollars>,
    pub cost_basis_pct20: MetricPattern4<Dollars>,
    pub cost_basis_pct25: MetricPattern4<Dollars>,
    pub cost_basis_pct30: MetricPattern4<Dollars>,
    pub cost_basis_pct35: MetricPattern4<Dollars>,
    pub cost_basis_pct40: MetricPattern4<Dollars>,
    pub cost_basis_pct45: MetricPattern4<Dollars>,
    pub cost_basis_pct50: MetricPattern4<Dollars>,
    pub cost_basis_pct55: MetricPattern4<Dollars>,
    pub cost_basis_pct60: MetricPattern4<Dollars>,
    pub cost_basis_pct65: MetricPattern4<Dollars>,
    pub cost_basis_pct70: MetricPattern4<Dollars>,
    pub cost_basis_pct75: MetricPattern4<Dollars>,
    pub cost_basis_pct80: MetricPattern4<Dollars>,
    pub cost_basis_pct85: MetricPattern4<Dollars>,
    pub cost_basis_pct90: MetricPattern4<Dollars>,
    pub cost_basis_pct95: MetricPattern4<Dollars>,
}

impl PercentilesPattern2 {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            cost_basis_pct05: MetricPattern4::new(client.clone(), _m(&acc, "pct05")),
            cost_basis_pct10: MetricPattern4::new(client.clone(), _m(&acc, "pct10")),
            cost_basis_pct15: MetricPattern4::new(client.clone(), _m(&acc, "pct15")),
            cost_basis_pct20: MetricPattern4::new(client.clone(), _m(&acc, "pct20")),
            cost_basis_pct25: MetricPattern4::new(client.clone(), _m(&acc, "pct25")),
            cost_basis_pct30: MetricPattern4::new(client.clone(), _m(&acc, "pct30")),
            cost_basis_pct35: MetricPattern4::new(client.clone(), _m(&acc, "pct35")),
            cost_basis_pct40: MetricPattern4::new(client.clone(), _m(&acc, "pct40")),
            cost_basis_pct45: MetricPattern4::new(client.clone(), _m(&acc, "pct45")),
            cost_basis_pct50: MetricPattern4::new(client.clone(), _m(&acc, "pct50")),
            cost_basis_pct55: MetricPattern4::new(client.clone(), _m(&acc, "pct55")),
            cost_basis_pct60: MetricPattern4::new(client.clone(), _m(&acc, "pct60")),
            cost_basis_pct65: MetricPattern4::new(client.clone(), _m(&acc, "pct65")),
            cost_basis_pct70: MetricPattern4::new(client.clone(), _m(&acc, "pct70")),
            cost_basis_pct75: MetricPattern4::new(client.clone(), _m(&acc, "pct75")),
            cost_basis_pct80: MetricPattern4::new(client.clone(), _m(&acc, "pct80")),
            cost_basis_pct85: MetricPattern4::new(client.clone(), _m(&acc, "pct85")),
            cost_basis_pct90: MetricPattern4::new(client.clone(), _m(&acc, "pct90")),
            cost_basis_pct95: MetricPattern4::new(client.clone(), _m(&acc, "pct95")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct RelativePattern5 {
    pub neg_unrealized_loss_rel_to_market_cap: MetricPattern1<StoredF32>,
    pub neg_unrealized_loss_rel_to_own_market_cap: MetricPattern1<StoredF32>,
    pub neg_unrealized_loss_rel_to_own_total_unrealized_pnl: MetricPattern1<StoredF32>,
    pub net_unrealized_pnl_rel_to_market_cap: MetricPattern1<StoredF32>,
    pub net_unrealized_pnl_rel_to_own_market_cap: MetricPattern1<StoredF32>,
    pub net_unrealized_pnl_rel_to_own_total_unrealized_pnl: MetricPattern1<StoredF32>,
    pub nupl: MetricPattern1<StoredF32>,
    pub supply_in_loss_rel_to_circulating_supply: MetricPattern1<StoredF64>,
    pub supply_in_loss_rel_to_own_supply: MetricPattern1<StoredF64>,
    pub supply_in_profit_rel_to_circulating_supply: MetricPattern1<StoredF64>,
    pub supply_in_profit_rel_to_own_supply: MetricPattern1<StoredF64>,
    pub supply_rel_to_circulating_supply: MetricPattern4<StoredF64>,
    pub unrealized_loss_rel_to_market_cap: MetricPattern1<StoredF32>,
    pub unrealized_loss_rel_to_own_market_cap: MetricPattern1<StoredF32>,
    pub unrealized_loss_rel_to_own_total_unrealized_pnl: MetricPattern1<StoredF32>,
    pub unrealized_profit_rel_to_market_cap: MetricPattern1<StoredF32>,
    pub unrealized_profit_rel_to_own_market_cap: MetricPattern1<StoredF32>,
    pub unrealized_profit_rel_to_own_total_unrealized_pnl: MetricPattern1<StoredF32>,
}

impl RelativePattern5 {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            neg_unrealized_loss_rel_to_market_cap: MetricPattern1::new(client.clone(), _m(&acc, "neg_unrealized_loss_rel_to_market_cap")),
            neg_unrealized_loss_rel_to_own_market_cap: MetricPattern1::new(client.clone(), _m(&acc, "neg_unrealized_loss_rel_to_own_market_cap")),
            neg_unrealized_loss_rel_to_own_total_unrealized_pnl: MetricPattern1::new(client.clone(), _m(&acc, "neg_unrealized_loss_rel_to_own_total_unrealized_pnl")),
            net_unrealized_pnl_rel_to_market_cap: MetricPattern1::new(client.clone(), _m(&acc, "net_unrealized_pnl_rel_to_market_cap")),
            net_unrealized_pnl_rel_to_own_market_cap: MetricPattern1::new(client.clone(), _m(&acc, "net_unrealized_pnl_rel_to_own_market_cap")),
            net_unrealized_pnl_rel_to_own_total_unrealized_pnl: MetricPattern1::new(client.clone(), _m(&acc, "net_unrealized_pnl_rel_to_own_total_unrealized_pnl")),
            nupl: MetricPattern1::new(client.clone(), _m(&acc, "nupl")),
            supply_in_loss_rel_to_circulating_supply: MetricPattern1::new(client.clone(), _m(&acc, "supply_in_loss_rel_to_circulating_supply")),
            supply_in_loss_rel_to_own_supply: MetricPattern1::new(client.clone(), _m(&acc, "supply_in_loss_rel_to_own_supply")),
            supply_in_profit_rel_to_circulating_supply: MetricPattern1::new(client.clone(), _m(&acc, "supply_in_profit_rel_to_circulating_supply")),
            supply_in_profit_rel_to_own_supply: MetricPattern1::new(client.clone(), _m(&acc, "supply_in_profit_rel_to_own_supply")),
            supply_rel_to_circulating_supply: MetricPattern4::new(client.clone(), _m(&acc, "supply_rel_to_circulating_supply")),
            unrealized_loss_rel_to_market_cap: MetricPattern1::new(client.clone(), _m(&acc, "unrealized_loss_rel_to_market_cap")),
            unrealized_loss_rel_to_own_market_cap: MetricPattern1::new(client.clone(), _m(&acc, "unrealized_loss_rel_to_own_market_cap")),
            unrealized_loss_rel_to_own_total_unrealized_pnl: MetricPattern1::new(client.clone(), _m(&acc, "unrealized_loss_rel_to_own_total_unrealized_pnl")),
            unrealized_profit_rel_to_market_cap: MetricPattern1::new(client.clone(), _m(&acc, "unrealized_profit_rel_to_market_cap")),
            unrealized_profit_rel_to_own_market_cap: MetricPattern1::new(client.clone(), _m(&acc, "unrealized_profit_rel_to_own_market_cap")),
            unrealized_profit_rel_to_own_total_unrealized_pnl: MetricPattern1::new(client.clone(), _m(&acc, "unrealized_profit_rel_to_own_total_unrealized_pnl")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct AaopoolPattern {
    pub _1m_blocks_mined: MetricPattern1<StoredU32>,
    pub _1m_dominance: MetricPattern1<StoredF32>,
    pub _1w_blocks_mined: MetricPattern1<StoredU32>,
    pub _1w_dominance: MetricPattern1<StoredF32>,
    pub _1y_blocks_mined: MetricPattern1<StoredU32>,
    pub _1y_dominance: MetricPattern1<StoredF32>,
    pub _24h_blocks_mined: MetricPattern1<StoredU32>,
    pub _24h_dominance: MetricPattern1<StoredF32>,
    pub blocks_mined: BlockCountPattern<StoredU32>,
    pub coinbase: UnclaimedRewardsPattern,
    pub days_since_block: MetricPattern4<StoredU16>,
    pub dominance: MetricPattern1<StoredF32>,
    pub fee: UnclaimedRewardsPattern,
    pub subsidy: UnclaimedRewardsPattern,
}

impl AaopoolPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            _1m_blocks_mined: MetricPattern1::new(client.clone(), _m(&acc, "1m_blocks_mined")),
            _1m_dominance: MetricPattern1::new(client.clone(), _m(&acc, "1m_dominance")),
            _1w_blocks_mined: MetricPattern1::new(client.clone(), _m(&acc, "1w_blocks_mined")),
            _1w_dominance: MetricPattern1::new(client.clone(), _m(&acc, "1w_dominance")),
            _1y_blocks_mined: MetricPattern1::new(client.clone(), _m(&acc, "1y_blocks_mined")),
            _1y_dominance: MetricPattern1::new(client.clone(), _m(&acc, "1y_dominance")),
            _24h_blocks_mined: MetricPattern1::new(client.clone(), _m(&acc, "24h_blocks_mined")),
            _24h_dominance: MetricPattern1::new(client.clone(), _m(&acc, "24h_dominance")),
            blocks_mined: BlockCountPattern::new(client.clone(), _m(&acc, "blocks_mined")),
            coinbase: UnclaimedRewardsPattern::new(client.clone(), _m(&acc, "coinbase")),
            days_since_block: MetricPattern4::new(client.clone(), _m(&acc, "days_since_block")),
            dominance: MetricPattern1::new(client.clone(), _m(&acc, "dominance")),
            fee: UnclaimedRewardsPattern::new(client.clone(), _m(&acc, "fee")),
            subsidy: UnclaimedRewardsPattern::new(client.clone(), _m(&acc, "subsidy")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct PriceAgoPattern<T> {
    pub _10y: MetricPattern4<T>,
    pub _1d: MetricPattern4<T>,
    pub _1m: MetricPattern4<T>,
    pub _1w: MetricPattern4<T>,
    pub _1y: MetricPattern4<T>,
    pub _2y: MetricPattern4<T>,
    pub _3m: MetricPattern4<T>,
    pub _3y: MetricPattern4<T>,
    pub _4y: MetricPattern4<T>,
    pub _5y: MetricPattern4<T>,
    pub _6m: MetricPattern4<T>,
    pub _6y: MetricPattern4<T>,
    pub _8y: MetricPattern4<T>,
}

impl<T: DeserializeOwned> PriceAgoPattern<T> {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            _10y: MetricPattern4::new(client.clone(), _m(&acc, "10y_ago")),
            _1d: MetricPattern4::new(client.clone(), _m(&acc, "1d_ago")),
            _1m: MetricPattern4::new(client.clone(), _m(&acc, "1m_ago")),
            _1w: MetricPattern4::new(client.clone(), _m(&acc, "1w_ago")),
            _1y: MetricPattern4::new(client.clone(), _m(&acc, "1y_ago")),
            _2y: MetricPattern4::new(client.clone(), _m(&acc, "2y_ago")),
            _3m: MetricPattern4::new(client.clone(), _m(&acc, "3m_ago")),
            _3y: MetricPattern4::new(client.clone(), _m(&acc, "3y_ago")),
            _4y: MetricPattern4::new(client.clone(), _m(&acc, "4y_ago")),
            _5y: MetricPattern4::new(client.clone(), _m(&acc, "5y_ago")),
            _6m: MetricPattern4::new(client.clone(), _m(&acc, "6m_ago")),
            _6y: MetricPattern4::new(client.clone(), _m(&acc, "6y_ago")),
            _8y: MetricPattern4::new(client.clone(), _m(&acc, "8y_ago")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct PeriodLumpSumStackPattern {
    pub _10y: _24hCoinbaseSumPattern,
    pub _1m: _24hCoinbaseSumPattern,
    pub _1w: _24hCoinbaseSumPattern,
    pub _1y: _24hCoinbaseSumPattern,
    pub _2y: _24hCoinbaseSumPattern,
    pub _3m: _24hCoinbaseSumPattern,
    pub _3y: _24hCoinbaseSumPattern,
    pub _4y: _24hCoinbaseSumPattern,
    pub _5y: _24hCoinbaseSumPattern,
    pub _6m: _24hCoinbaseSumPattern,
    pub _6y: _24hCoinbaseSumPattern,
    pub _8y: _24hCoinbaseSumPattern,
}

impl PeriodLumpSumStackPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            _10y: _24hCoinbaseSumPattern::new(client.clone(), if acc.is_empty() { "10y".to_string() } else { format!("10y_{acc}") }),
            _1m: _24hCoinbaseSumPattern::new(client.clone(), if acc.is_empty() { "1m".to_string() } else { format!("1m_{acc}") }),
            _1w: _24hCoinbaseSumPattern::new(client.clone(), if acc.is_empty() { "1w".to_string() } else { format!("1w_{acc}") }),
            _1y: _24hCoinbaseSumPattern::new(client.clone(), if acc.is_empty() { "1y".to_string() } else { format!("1y_{acc}") }),
            _2y: _24hCoinbaseSumPattern::new(client.clone(), if acc.is_empty() { "2y".to_string() } else { format!("2y_{acc}") }),
            _3m: _24hCoinbaseSumPattern::new(client.clone(), if acc.is_empty() { "3m".to_string() } else { format!("3m_{acc}") }),
            _3y: _24hCoinbaseSumPattern::new(client.clone(), if acc.is_empty() { "3y".to_string() } else { format!("3y_{acc}") }),
            _4y: _24hCoinbaseSumPattern::new(client.clone(), if acc.is_empty() { "4y".to_string() } else { format!("4y_{acc}") }),
            _5y: _24hCoinbaseSumPattern::new(client.clone(), if acc.is_empty() { "5y".to_string() } else { format!("5y_{acc}") }),
            _6m: _24hCoinbaseSumPattern::new(client.clone(), if acc.is_empty() { "6m".to_string() } else { format!("6m_{acc}") }),
            _6y: _24hCoinbaseSumPattern::new(client.clone(), if acc.is_empty() { "6y".to_string() } else { format!("6y_{acc}") }),
            _8y: _24hCoinbaseSumPattern::new(client.clone(), if acc.is_empty() { "8y".to_string() } else { format!("8y_{acc}") }),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct PeriodAveragePricePattern<T> {
    pub _10y: MetricPattern4<T>,
    pub _1m: MetricPattern4<T>,
    pub _1w: MetricPattern4<T>,
    pub _1y: MetricPattern4<T>,
    pub _2y: MetricPattern4<T>,
    pub _3m: MetricPattern4<T>,
    pub _3y: MetricPattern4<T>,
    pub _4y: MetricPattern4<T>,
    pub _5y: MetricPattern4<T>,
    pub _6m: MetricPattern4<T>,
    pub _6y: MetricPattern4<T>,
    pub _8y: MetricPattern4<T>,
}

impl<T: DeserializeOwned> PeriodAveragePricePattern<T> {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            _10y: MetricPattern4::new(client.clone(), if acc.is_empty() { "10y".to_string() } else { format!("10y_{acc}") }),
            _1m: MetricPattern4::new(client.clone(), if acc.is_empty() { "1m".to_string() } else { format!("1m_{acc}") }),
            _1w: MetricPattern4::new(client.clone(), if acc.is_empty() { "1w".to_string() } else { format!("1w_{acc}") }),
            _1y: MetricPattern4::new(client.clone(), if acc.is_empty() { "1y".to_string() } else { format!("1y_{acc}") }),
            _2y: MetricPattern4::new(client.clone(), if acc.is_empty() { "2y".to_string() } else { format!("2y_{acc}") }),
            _3m: MetricPattern4::new(client.clone(), if acc.is_empty() { "3m".to_string() } else { format!("3m_{acc}") }),
            _3y: MetricPattern4::new(client.clone(), if acc.is_empty() { "3y".to_string() } else { format!("3y_{acc}") }),
            _4y: MetricPattern4::new(client.clone(), if acc.is_empty() { "4y".to_string() } else { format!("4y_{acc}") }),
            _5y: MetricPattern4::new(client.clone(), if acc.is_empty() { "5y".to_string() } else { format!("5y_{acc}") }),
            _6m: MetricPattern4::new(client.clone(), if acc.is_empty() { "6m".to_string() } else { format!("6m_{acc}") }),
            _6y: MetricPattern4::new(client.clone(), if acc.is_empty() { "6y".to_string() } else { format!("6y_{acc}") }),
            _8y: MetricPattern4::new(client.clone(), if acc.is_empty() { "8y".to_string() } else { format!("8y_{acc}") }),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct ClassAveragePricePattern<T> {
    pub _2015: MetricPattern4<T>,
    pub _2016: MetricPattern4<T>,
    pub _2017: MetricPattern4<T>,
    pub _2018: MetricPattern4<T>,
    pub _2019: MetricPattern4<T>,
    pub _2020: MetricPattern4<T>,
    pub _2021: MetricPattern4<T>,
    pub _2022: MetricPattern4<T>,
    pub _2023: MetricPattern4<T>,
    pub _2024: MetricPattern4<T>,
    pub _2025: MetricPattern4<T>,
}

impl<T: DeserializeOwned> ClassAveragePricePattern<T> {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            _2015: MetricPattern4::new(client.clone(), _m(&acc, "2015_average_price")),
            _2016: MetricPattern4::new(client.clone(), _m(&acc, "2016_average_price")),
            _2017: MetricPattern4::new(client.clone(), _m(&acc, "2017_average_price")),
            _2018: MetricPattern4::new(client.clone(), _m(&acc, "2018_average_price")),
            _2019: MetricPattern4::new(client.clone(), _m(&acc, "2019_average_price")),
            _2020: MetricPattern4::new(client.clone(), _m(&acc, "2020_average_price")),
            _2021: MetricPattern4::new(client.clone(), _m(&acc, "2021_average_price")),
            _2022: MetricPattern4::new(client.clone(), _m(&acc, "2022_average_price")),
            _2023: MetricPattern4::new(client.clone(), _m(&acc, "2023_average_price")),
            _2024: MetricPattern4::new(client.clone(), _m(&acc, "2024_average_price")),
            _2025: MetricPattern4::new(client.clone(), _m(&acc, "2025_average_price")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct RelativePattern2 {
    pub neg_unrealized_loss_rel_to_own_market_cap: MetricPattern1<StoredF32>,
    pub neg_unrealized_loss_rel_to_own_total_unrealized_pnl: MetricPattern1<StoredF32>,
    pub net_unrealized_pnl_rel_to_own_market_cap: MetricPattern1<StoredF32>,
    pub net_unrealized_pnl_rel_to_own_total_unrealized_pnl: MetricPattern1<StoredF32>,
    pub supply_in_loss_rel_to_own_supply: MetricPattern1<StoredF64>,
    pub supply_in_profit_rel_to_own_supply: MetricPattern1<StoredF64>,
    pub unrealized_loss_rel_to_own_market_cap: MetricPattern1<StoredF32>,
    pub unrealized_loss_rel_to_own_total_unrealized_pnl: MetricPattern1<StoredF32>,
    pub unrealized_profit_rel_to_own_market_cap: MetricPattern1<StoredF32>,
    pub unrealized_profit_rel_to_own_total_unrealized_pnl: MetricPattern1<StoredF32>,
}

impl RelativePattern2 {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            neg_unrealized_loss_rel_to_own_market_cap: MetricPattern1::new(client.clone(), _m(&acc, "neg_unrealized_loss_rel_to_own_market_cap")),
            neg_unrealized_loss_rel_to_own_total_unrealized_pnl: MetricPattern1::new(client.clone(), _m(&acc, "neg_unrealized_loss_rel_to_own_total_unrealized_pnl")),
            net_unrealized_pnl_rel_to_own_market_cap: MetricPattern1::new(client.clone(), _m(&acc, "net_unrealized_pnl_rel_to_own_market_cap")),
            net_unrealized_pnl_rel_to_own_total_unrealized_pnl: MetricPattern1::new(client.clone(), _m(&acc, "net_unrealized_pnl_rel_to_own_total_unrealized_pnl")),
            supply_in_loss_rel_to_own_supply: MetricPattern1::new(client.clone(), _m(&acc, "supply_in_loss_rel_to_own_supply")),
            supply_in_profit_rel_to_own_supply: MetricPattern1::new(client.clone(), _m(&acc, "supply_in_profit_rel_to_own_supply")),
            unrealized_loss_rel_to_own_market_cap: MetricPattern1::new(client.clone(), _m(&acc, "unrealized_loss_rel_to_own_market_cap")),
            unrealized_loss_rel_to_own_total_unrealized_pnl: MetricPattern1::new(client.clone(), _m(&acc, "unrealized_loss_rel_to_own_total_unrealized_pnl")),
            unrealized_profit_rel_to_own_market_cap: MetricPattern1::new(client.clone(), _m(&acc, "unrealized_profit_rel_to_own_market_cap")),
            unrealized_profit_rel_to_own_total_unrealized_pnl: MetricPattern1::new(client.clone(), _m(&acc, "unrealized_profit_rel_to_own_total_unrealized_pnl")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct RelativePattern {
    pub neg_unrealized_loss_rel_to_market_cap: MetricPattern1<StoredF32>,
    pub net_unrealized_pnl_rel_to_market_cap: MetricPattern1<StoredF32>,
    pub nupl: MetricPattern1<StoredF32>,
    pub supply_in_loss_rel_to_circulating_supply: MetricPattern1<StoredF64>,
    pub supply_in_loss_rel_to_own_supply: MetricPattern1<StoredF64>,
    pub supply_in_profit_rel_to_circulating_supply: MetricPattern1<StoredF64>,
    pub supply_in_profit_rel_to_own_supply: MetricPattern1<StoredF64>,
    pub supply_rel_to_circulating_supply: MetricPattern4<StoredF64>,
    pub unrealized_loss_rel_to_market_cap: MetricPattern1<StoredF32>,
    pub unrealized_profit_rel_to_market_cap: MetricPattern1<StoredF32>,
}

impl RelativePattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            neg_unrealized_loss_rel_to_market_cap: MetricPattern1::new(client.clone(), _m(&acc, "neg_unrealized_loss_rel_to_market_cap")),
            net_unrealized_pnl_rel_to_market_cap: MetricPattern1::new(client.clone(), _m(&acc, "net_unrealized_pnl_rel_to_market_cap")),
            nupl: MetricPattern1::new(client.clone(), _m(&acc, "nupl")),
            supply_in_loss_rel_to_circulating_supply: MetricPattern1::new(client.clone(), _m(&acc, "supply_in_loss_rel_to_circulating_supply")),
            supply_in_loss_rel_to_own_supply: MetricPattern1::new(client.clone(), _m(&acc, "supply_in_loss_rel_to_own_supply")),
            supply_in_profit_rel_to_circulating_supply: MetricPattern1::new(client.clone(), _m(&acc, "supply_in_profit_rel_to_circulating_supply")),
            supply_in_profit_rel_to_own_supply: MetricPattern1::new(client.clone(), _m(&acc, "supply_in_profit_rel_to_own_supply")),
            supply_rel_to_circulating_supply: MetricPattern4::new(client.clone(), _m(&acc, "supply_rel_to_circulating_supply")),
            unrealized_loss_rel_to_market_cap: MetricPattern1::new(client.clone(), _m(&acc, "unrealized_loss_rel_to_market_cap")),
            unrealized_profit_rel_to_market_cap: MetricPattern1::new(client.clone(), _m(&acc, "unrealized_profit_rel_to_market_cap")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct AddrCountPattern {
    pub all: MetricPattern1<StoredU64>,
    pub p2a: MetricPattern1<StoredU64>,
    pub p2pk33: MetricPattern1<StoredU64>,
    pub p2pk65: MetricPattern1<StoredU64>,
    pub p2pkh: MetricPattern1<StoredU64>,
    pub p2sh: MetricPattern1<StoredU64>,
    pub p2tr: MetricPattern1<StoredU64>,
    pub p2wpkh: MetricPattern1<StoredU64>,
    pub p2wsh: MetricPattern1<StoredU64>,
}

impl AddrCountPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            all: MetricPattern1::new(client.clone(), if acc.is_empty() { "addr".to_string() } else { format!("addr_{acc}") }),
            p2a: MetricPattern1::new(client.clone(), if acc.is_empty() { "p2a_addr".to_string() } else { format!("p2a_addr_{acc}") }),
            p2pk33: MetricPattern1::new(client.clone(), if acc.is_empty() { "p2pk33_addr".to_string() } else { format!("p2pk33_addr_{acc}") }),
            p2pk65: MetricPattern1::new(client.clone(), if acc.is_empty() { "p2pk65_addr".to_string() } else { format!("p2pk65_addr_{acc}") }),
            p2pkh: MetricPattern1::new(client.clone(), if acc.is_empty() { "p2pkh_addr".to_string() } else { format!("p2pkh_addr_{acc}") }),
            p2sh: MetricPattern1::new(client.clone(), if acc.is_empty() { "p2sh_addr".to_string() } else { format!("p2sh_addr_{acc}") }),
            p2tr: MetricPattern1::new(client.clone(), if acc.is_empty() { "p2tr_addr".to_string() } else { format!("p2tr_addr_{acc}") }),
            p2wpkh: MetricPattern1::new(client.clone(), if acc.is_empty() { "p2wpkh_addr".to_string() } else { format!("p2wpkh_addr_{acc}") }),
            p2wsh: MetricPattern1::new(client.clone(), if acc.is_empty() { "p2wsh_addr".to_string() } else { format!("p2wsh_addr_{acc}") }),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct FeeRatePattern<T> {
    pub average: MetricPattern1<T>,
    pub max: MetricPattern1<T>,
    pub median: MetricPattern12<T>,
    pub min: MetricPattern1<T>,
    pub pct10: MetricPattern12<T>,
    pub pct25: MetricPattern12<T>,
    pub pct75: MetricPattern12<T>,
    pub pct90: MetricPattern12<T>,
    pub txindex: MetricPattern28<T>,
}

impl<T: DeserializeOwned> FeeRatePattern<T> {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            average: MetricPattern1::new(client.clone(), _m(&acc, "average")),
            max: MetricPattern1::new(client.clone(), _m(&acc, "max")),
            median: MetricPattern12::new(client.clone(), _m(&acc, "median")),
            min: MetricPattern1::new(client.clone(), _m(&acc, "min")),
            pct10: MetricPattern12::new(client.clone(), _m(&acc, "pct10")),
            pct25: MetricPattern12::new(client.clone(), _m(&acc, "pct25")),
            pct75: MetricPattern12::new(client.clone(), _m(&acc, "pct75")),
            pct90: MetricPattern12::new(client.clone(), _m(&acc, "pct90")),
            txindex: MetricPattern28::new(client.clone(), acc.clone()),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct OhlcPattern<T> {
    pub dateindex: MetricPattern7<T>,
    pub decade: MetricPattern8<T>,
    pub difficultyepoch: MetricPattern9<T>,
    pub height: MetricPattern12<T>,
    pub month: MetricPattern14<T>,
    pub quarter: MetricPattern26<T>,
    pub semester: MetricPattern27<T>,
    pub week: MetricPattern30<T>,
    pub year: MetricPattern31<T>,
}

impl<T: DeserializeOwned> OhlcPattern<T> {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            dateindex: MetricPattern7::new(client.clone(), acc.clone()),
            decade: MetricPattern8::new(client.clone(), acc.clone()),
            difficultyepoch: MetricPattern9::new(client.clone(), acc.clone()),
            height: MetricPattern12::new(client.clone(), acc.clone()),
            month: MetricPattern14::new(client.clone(), acc.clone()),
            quarter: MetricPattern26::new(client.clone(), acc.clone()),
            semester: MetricPattern27::new(client.clone(), acc.clone()),
            week: MetricPattern30::new(client.clone(), acc.clone()),
            year: MetricPattern31::new(client.clone(), acc.clone()),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct _0satsPattern {
    pub activity: ActivityPattern2,
    pub addr_count: MetricPattern1<StoredU64>,
    pub cost_basis: CostBasisPattern,
    pub outputs: OutputsPattern,
    pub realized: RealizedPattern,
    pub relative: RelativePattern,
    pub supply: SupplyPattern2,
    pub unrealized: UnrealizedPattern,
}

impl _0satsPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            activity: ActivityPattern2::new(client.clone(), acc.clone()),
            addr_count: MetricPattern1::new(client.clone(), _m(&acc, "addr_count")),
            cost_basis: CostBasisPattern::new(client.clone(), acc.clone()),
            outputs: OutputsPattern::new(client.clone(), acc.clone()),
            realized: RealizedPattern::new(client.clone(), acc.clone()),
            relative: RelativePattern::new(client.clone(), acc.clone()),
            supply: SupplyPattern2::new(client.clone(), _m(&acc, "supply")),
            unrealized: UnrealizedPattern::new(client.clone(), acc.clone()),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct _0satsPattern2 {
    pub activity: ActivityPattern2,
    pub cost_basis: CostBasisPattern,
    pub outputs: OutputsPattern,
    pub realized: RealizedPattern,
    pub relative: RelativePattern4,
    pub supply: SupplyPattern2,
    pub unrealized: UnrealizedPattern,
}

impl _0satsPattern2 {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            activity: ActivityPattern2::new(client.clone(), acc.clone()),
            cost_basis: CostBasisPattern::new(client.clone(), acc.clone()),
            outputs: OutputsPattern::new(client.clone(), acc.clone()),
            realized: RealizedPattern::new(client.clone(), acc.clone()),
            relative: RelativePattern4::new(client.clone(), _m(&acc, "supply_in")),
            supply: SupplyPattern2::new(client.clone(), _m(&acc, "supply")),
            unrealized: UnrealizedPattern::new(client.clone(), acc.clone()),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct UnrealizedPattern {
    pub neg_unrealized_loss: MetricPattern1<Dollars>,
    pub net_unrealized_pnl: MetricPattern1<Dollars>,
    pub supply_in_loss: _24hCoinbaseSumPattern,
    pub supply_in_profit: _24hCoinbaseSumPattern,
    pub total_unrealized_pnl: MetricPattern1<Dollars>,
    pub unrealized_loss: MetricPattern1<Dollars>,
    pub unrealized_profit: MetricPattern1<Dollars>,
}

impl UnrealizedPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            neg_unrealized_loss: MetricPattern1::new(client.clone(), _m(&acc, "neg_unrealized_loss")),
            net_unrealized_pnl: MetricPattern1::new(client.clone(), _m(&acc, "net_unrealized_pnl")),
            supply_in_loss: _24hCoinbaseSumPattern::new(client.clone(), _m(&acc, "supply_in_loss")),
            supply_in_profit: _24hCoinbaseSumPattern::new(client.clone(), _m(&acc, "supply_in_profit")),
            total_unrealized_pnl: MetricPattern1::new(client.clone(), _m(&acc, "total_unrealized_pnl")),
            unrealized_loss: MetricPattern1::new(client.clone(), _m(&acc, "unrealized_loss")),
            unrealized_profit: MetricPattern1::new(client.clone(), _m(&acc, "unrealized_profit")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct _100btcPattern {
    pub activity: ActivityPattern2,
    pub cost_basis: CostBasisPattern,
    pub outputs: OutputsPattern,
    pub realized: RealizedPattern,
    pub relative: RelativePattern,
    pub supply: SupplyPattern2,
    pub unrealized: UnrealizedPattern,
}

impl _100btcPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            activity: ActivityPattern2::new(client.clone(), acc.clone()),
            cost_basis: CostBasisPattern::new(client.clone(), acc.clone()),
            outputs: OutputsPattern::new(client.clone(), acc.clone()),
            realized: RealizedPattern::new(client.clone(), acc.clone()),
            relative: RelativePattern::new(client.clone(), acc.clone()),
            supply: SupplyPattern2::new(client.clone(), _m(&acc, "supply")),
            unrealized: UnrealizedPattern::new(client.clone(), acc.clone()),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct _10yTo12yPattern {
    pub activity: ActivityPattern2,
    pub cost_basis: CostBasisPattern2,
    pub outputs: OutputsPattern,
    pub realized: RealizedPattern2,
    pub relative: RelativePattern2,
    pub supply: SupplyPattern2,
    pub unrealized: UnrealizedPattern,
}

impl _10yTo12yPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            activity: ActivityPattern2::new(client.clone(), acc.clone()),
            cost_basis: CostBasisPattern2::new(client.clone(), acc.clone()),
            outputs: OutputsPattern::new(client.clone(), acc.clone()),
            realized: RealizedPattern2::new(client.clone(), acc.clone()),
            relative: RelativePattern2::new(client.clone(), acc.clone()),
            supply: SupplyPattern2::new(client.clone(), _m(&acc, "supply")),
            unrealized: UnrealizedPattern::new(client.clone(), acc.clone()),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct _10yPattern {
    pub activity: ActivityPattern2,
    pub cost_basis: CostBasisPattern,
    pub outputs: OutputsPattern,
    pub realized: RealizedPattern4,
    pub relative: RelativePattern,
    pub supply: SupplyPattern2,
    pub unrealized: UnrealizedPattern,
}

impl _10yPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            activity: ActivityPattern2::new(client.clone(), acc.clone()),
            cost_basis: CostBasisPattern::new(client.clone(), acc.clone()),
            outputs: OutputsPattern::new(client.clone(), acc.clone()),
            realized: RealizedPattern4::new(client.clone(), acc.clone()),
            relative: RelativePattern::new(client.clone(), acc.clone()),
            supply: SupplyPattern2::new(client.clone(), _m(&acc, "supply")),
            unrealized: UnrealizedPattern::new(client.clone(), acc.clone()),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct PeriodCagrPattern {
    pub _10y: MetricPattern4<StoredF32>,
    pub _2y: MetricPattern4<StoredF32>,
    pub _3y: MetricPattern4<StoredF32>,
    pub _4y: MetricPattern4<StoredF32>,
    pub _5y: MetricPattern4<StoredF32>,
    pub _6y: MetricPattern4<StoredF32>,
    pub _8y: MetricPattern4<StoredF32>,
}

impl PeriodCagrPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            _10y: MetricPattern4::new(client.clone(), if acc.is_empty() { "10y".to_string() } else { format!("10y_{acc}") }),
            _2y: MetricPattern4::new(client.clone(), if acc.is_empty() { "2y".to_string() } else { format!("2y_{acc}") }),
            _3y: MetricPattern4::new(client.clone(), if acc.is_empty() { "3y".to_string() } else { format!("3y_{acc}") }),
            _4y: MetricPattern4::new(client.clone(), if acc.is_empty() { "4y".to_string() } else { format!("4y_{acc}") }),
            _5y: MetricPattern4::new(client.clone(), if acc.is_empty() { "5y".to_string() } else { format!("5y_{acc}") }),
            _6y: MetricPattern4::new(client.clone(), if acc.is_empty() { "6y".to_string() } else { format!("6y_{acc}") }),
            _8y: MetricPattern4::new(client.clone(), if acc.is_empty() { "8y".to_string() } else { format!("8y_{acc}") }),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct BitcoinPattern<T> {
    pub average: MetricPattern2<T>,
    pub cumulative: MetricPattern2<T>,
    pub height: MetricPattern12<T>,
    pub max: MetricPattern2<T>,
    pub min: MetricPattern2<T>,
    pub percentiles: PercentilesPattern<T>,
    pub sum: MetricPattern2<T>,
}

impl<T: DeserializeOwned> BitcoinPattern<T> {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            average: MetricPattern2::new(client.clone(), _m(&acc, "average")),
            cumulative: MetricPattern2::new(client.clone(), _m(&acc, "cumulative")),
            height: MetricPattern12::new(client.clone(), acc.clone()),
            max: MetricPattern2::new(client.clone(), _m(&acc, "max")),
            min: MetricPattern2::new(client.clone(), _m(&acc, "min")),
            percentiles: PercentilesPattern::new(client.clone(), acc.clone()),
            sum: MetricPattern2::new(client.clone(), _m(&acc, "sum")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct SizePattern<T> {
    pub average: MetricPattern1<T>,
    pub cumulative: MetricPattern1<T>,
    pub max: MetricPattern1<T>,
    pub min: MetricPattern1<T>,
    pub percentiles: PercentilesPattern<T>,
    pub sum: MetricPattern1<T>,
}

impl<T: DeserializeOwned> SizePattern<T> {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            average: MetricPattern1::new(client.clone(), _m(&acc, "average")),
            cumulative: MetricPattern1::new(client.clone(), _m(&acc, "cumulative")),
            max: MetricPattern1::new(client.clone(), _m(&acc, "max")),
            min: MetricPattern1::new(client.clone(), _m(&acc, "min")),
            percentiles: PercentilesPattern::new(client.clone(), acc.clone()),
            sum: MetricPattern1::new(client.clone(), _m(&acc, "sum")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct ActivityPattern2 {
    pub coinblocks_destroyed: BlockCountPattern<StoredF64>,
    pub coindays_destroyed: BlockCountPattern<StoredF64>,
    pub satblocks_destroyed: MetricPattern12<Sats>,
    pub satdays_destroyed: MetricPattern12<Sats>,
    pub sent: UnclaimedRewardsPattern,
}

impl ActivityPattern2 {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            coinblocks_destroyed: BlockCountPattern::new(client.clone(), _m(&acc, "coinblocks_destroyed")),
            coindays_destroyed: BlockCountPattern::new(client.clone(), _m(&acc, "coindays_destroyed")),
            satblocks_destroyed: MetricPattern12::new(client.clone(), _m(&acc, "satblocks_destroyed")),
            satdays_destroyed: MetricPattern12::new(client.clone(), _m(&acc, "satdays_destroyed")),
            sent: UnclaimedRewardsPattern::new(client.clone(), _m(&acc, "sent")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct PercentilesPattern<T> {
    pub median: MetricPattern7<T>,
    pub pct10: MetricPattern7<T>,
    pub pct25: MetricPattern7<T>,
    pub pct75: MetricPattern7<T>,
    pub pct90: MetricPattern7<T>,
}

impl<T: DeserializeOwned> PercentilesPattern<T> {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            median: MetricPattern7::new(client.clone(), _m(&acc, "median")),
            pct10: MetricPattern7::new(client.clone(), _m(&acc, "pct10")),
            pct25: MetricPattern7::new(client.clone(), _m(&acc, "pct25")),
            pct75: MetricPattern7::new(client.clone(), _m(&acc, "pct75")),
            pct90: MetricPattern7::new(client.clone(), _m(&acc, "pct90")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct DateindexPattern2 {
    pub close: MetricPattern7<Cents>,
    pub high: MetricPattern7<Cents>,
    pub low: MetricPattern7<Cents>,
    pub open: MetricPattern7<Cents>,
}

impl DateindexPattern2 {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            close: MetricPattern7::new(client.clone(), _m(&acc, "close_cents")),
            high: MetricPattern7::new(client.clone(), _m(&acc, "high_cents")),
            low: MetricPattern7::new(client.clone(), _m(&acc, "low_cents")),
            open: MetricPattern7::new(client.clone(), _m(&acc, "open_cents")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct HighPattern<T> {
    pub dateindex: MetricPattern7<T>,
    pub difficultyepoch: MetricPattern9<T>,
    pub height: MetricPattern12<T>,
    pub rest: MetricPattern5<T>,
}

impl<T: DeserializeOwned> HighPattern<T> {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            dateindex: MetricPattern7::new(client.clone(), acc.clone()),
            difficultyepoch: MetricPattern9::new(client.clone(), _m(&acc, "max")),
            height: MetricPattern12::new(client.clone(), acc.clone()),
            rest: MetricPattern5::new(client.clone(), _m(&acc, "max")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct SplitPattern2<T> {
    pub close: MetricPattern1<T>,
    pub high: HighPattern<T>,
    pub low: HighPattern<T>,
    pub open: MetricPattern1<T>,
}

impl<T: DeserializeOwned> SplitPattern2<T> {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            close: MetricPattern1::new(client.clone(), _m(&acc, "close")),
            high: HighPattern::new(client.clone(), _m(&acc, "high")),
            low: HighPattern::new(client.clone(), _m(&acc, "low")),
            open: MetricPattern1::new(client.clone(), _m(&acc, "open")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct _24hCoinbaseSumPattern {
    pub bitcoin: MetricPattern12<Bitcoin>,
    pub dollars: MetricPattern12<Dollars>,
    pub sats: MetricPattern12<Sats>,
}

impl _24hCoinbaseSumPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            bitcoin: MetricPattern12::new(client.clone(), _m(&acc, "btc")),
            dollars: MetricPattern12::new(client.clone(), _m(&acc, "usd")),
            sats: MetricPattern12::new(client.clone(), acc.clone()),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct CoinbasePattern {
    pub bitcoin: BitcoinPattern<Bitcoin>,
    pub dollars: BitcoinPattern<Dollars>,
    pub sats: BitcoinPattern<Sats>,
}

impl CoinbasePattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            bitcoin: BitcoinPattern::new(client.clone(), _m(&acc, "btc")),
            dollars: BitcoinPattern::new(client.clone(), _m(&acc, "usd")),
            sats: BitcoinPattern::new(client.clone(), acc.clone()),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct UnclaimedRewardsPattern {
    pub bitcoin: BlockCountPattern<Bitcoin>,
    pub dollars: BlockCountPattern<Dollars>,
    pub sats: BlockCountPattern<Sats>,
}

impl UnclaimedRewardsPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            bitcoin: BlockCountPattern::new(client.clone(), _m(&acc, "btc")),
            dollars: BlockCountPattern::new(client.clone(), _m(&acc, "usd")),
            sats: BlockCountPattern::new(client.clone(), acc.clone()),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct CostBasisPattern2 {
    pub max: MetricPattern1<Dollars>,
    pub min: MetricPattern1<Dollars>,
    pub percentiles: PercentilesPattern2,
}

impl CostBasisPattern2 {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            max: MetricPattern1::new(client.clone(), _m(&acc, "max_cost_basis")),
            min: MetricPattern1::new(client.clone(), _m(&acc, "min_cost_basis")),
            percentiles: PercentilesPattern2::new(client.clone(), _m(&acc, "cost_basis")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct SegwitAdoptionPattern {
    pub cumulative: MetricPattern2<StoredF32>,
    pub height: MetricPattern12<StoredF32>,
    pub sum: MetricPattern2<StoredF32>,
}

impl SegwitAdoptionPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            cumulative: MetricPattern2::new(client.clone(), _m(&acc, "cumulative")),
            height: MetricPattern12::new(client.clone(), acc.clone()),
            sum: MetricPattern2::new(client.clone(), _m(&acc, "sum")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct SatsPattern {
    pub ohlc: OhlcPattern<OHLCSats>,
    pub split: SplitPattern2<Sats>,
}

impl SatsPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            ohlc: OhlcPattern::new(client.clone(), _m(&acc, "ohlc_sats")),
            split: SplitPattern2::new(client.clone(), _m(&acc, "sats")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct SupplyPattern2 {
    pub halved: _24hCoinbaseSumPattern,
    pub total: _24hCoinbaseSumPattern,
}

impl SupplyPattern2 {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            halved: _24hCoinbaseSumPattern::new(client.clone(), _m(&acc, "half")),
            total: _24hCoinbaseSumPattern::new(client.clone(), acc.clone()),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct CostBasisPattern {
    pub max: MetricPattern1<Dollars>,
    pub min: MetricPattern1<Dollars>,
}

impl CostBasisPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            max: MetricPattern1::new(client.clone(), _m(&acc, "max_cost_basis")),
            min: MetricPattern1::new(client.clone(), _m(&acc, "min_cost_basis")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct _1dReturns1mSdPattern {
    pub sd: MetricPattern4<StoredF32>,
    pub sma: MetricPattern4<StoredF32>,
}

impl _1dReturns1mSdPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            sd: MetricPattern4::new(client.clone(), _m(&acc, "sd")),
            sma: MetricPattern4::new(client.clone(), _m(&acc, "sma")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct RelativePattern4 {
    pub supply_in_loss_rel_to_own_supply: MetricPattern1<StoredF64>,
    pub supply_in_profit_rel_to_own_supply: MetricPattern1<StoredF64>,
}

impl RelativePattern4 {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            supply_in_loss_rel_to_own_supply: MetricPattern1::new(client.clone(), _m(&acc, "loss_rel_to_own_supply")),
            supply_in_profit_rel_to_own_supply: MetricPattern1::new(client.clone(), _m(&acc, "profit_rel_to_own_supply")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct BlockCountPattern<T> {
    pub cumulative: MetricPattern1<T>,
    pub sum: MetricPattern1<T>,
}

impl<T: DeserializeOwned> BlockCountPattern<T> {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            cumulative: MetricPattern1::new(client.clone(), _m(&acc, "cumulative")),
            sum: MetricPattern1::new(client.clone(), acc.clone()),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct OutputsPattern {
    pub utxo_count: MetricPattern1<StoredU64>,
}

impl OutputsPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            utxo_count: MetricPattern1::new(client.clone(), _m(&acc, "utxo_count")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct RealizedPriceExtraPattern {
    pub ratio: MetricPattern4<StoredF32>,
}

impl RealizedPriceExtraPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            ratio: MetricPattern4::new(client.clone(), _m(&acc, "ratio")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct EmptyPattern<T> {
    pub identity: MetricPattern25<T>,
}

impl<T: DeserializeOwned> EmptyPattern<T> {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            identity: MetricPattern25::new(client.clone(), acc.clone()),
        }
    }
}

// Catalog tree

/// Catalog tree node.
pub struct CatalogTree {
    pub addresses: CatalogTree_Addresses,
    pub blocks: CatalogTree_Blocks,
    pub cointime: CatalogTree_Cointime,
    pub constants: CatalogTree_Constants,
    pub distribution: CatalogTree_Distribution,
    pub indexes: CatalogTree_Indexes,
    pub inputs: CatalogTree_Inputs,
    pub market: CatalogTree_Market,
    pub outputs: CatalogTree_Outputs,
    pub pools: CatalogTree_Pools,
    pub positions: CatalogTree_Positions,
    pub price: CatalogTree_Price,
    pub scripts: CatalogTree_Scripts,
    pub supply: CatalogTree_Supply,
    pub transactions: CatalogTree_Transactions,
}

impl CatalogTree {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            addresses: CatalogTree_Addresses::new(client.clone(), format!("{base_path}_addresses")),
            blocks: CatalogTree_Blocks::new(client.clone(), format!("{base_path}_blocks")),
            cointime: CatalogTree_Cointime::new(client.clone(), format!("{base_path}_cointime")),
            constants: CatalogTree_Constants::new(client.clone(), format!("{base_path}_constants")),
            distribution: CatalogTree_Distribution::new(client.clone(), format!("{base_path}_distribution")),
            indexes: CatalogTree_Indexes::new(client.clone(), format!("{base_path}_indexes")),
            inputs: CatalogTree_Inputs::new(client.clone(), format!("{base_path}_inputs")),
            market: CatalogTree_Market::new(client.clone(), format!("{base_path}_market")),
            outputs: CatalogTree_Outputs::new(client.clone(), format!("{base_path}_outputs")),
            pools: CatalogTree_Pools::new(client.clone(), format!("{base_path}_pools")),
            positions: CatalogTree_Positions::new(client.clone(), format!("{base_path}_positions")),
            price: CatalogTree_Price::new(client.clone(), format!("{base_path}_price")),
            scripts: CatalogTree_Scripts::new(client.clone(), format!("{base_path}_scripts")),
            supply: CatalogTree_Supply::new(client.clone(), format!("{base_path}_supply")),
            transactions: CatalogTree_Transactions::new(client.clone(), format!("{base_path}_transactions")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Addresses {
    pub first_p2aaddressindex: MetricPattern12<P2AAddressIndex>,
    pub first_p2pk33addressindex: MetricPattern12<P2PK33AddressIndex>,
    pub first_p2pk65addressindex: MetricPattern12<P2PK65AddressIndex>,
    pub first_p2pkhaddressindex: MetricPattern12<P2PKHAddressIndex>,
    pub first_p2shaddressindex: MetricPattern12<P2SHAddressIndex>,
    pub first_p2traddressindex: MetricPattern12<P2TRAddressIndex>,
    pub first_p2wpkhaddressindex: MetricPattern12<P2WPKHAddressIndex>,
    pub first_p2wshaddressindex: MetricPattern12<P2WSHAddressIndex>,
    pub p2abytes: MetricPattern17<P2ABytes>,
    pub p2pk33bytes: MetricPattern19<P2PK33Bytes>,
    pub p2pk65bytes: MetricPattern20<P2PK65Bytes>,
    pub p2pkhbytes: MetricPattern21<P2PKHBytes>,
    pub p2shbytes: MetricPattern22<P2SHBytes>,
    pub p2trbytes: MetricPattern23<P2TRBytes>,
    pub p2wpkhbytes: MetricPattern24<P2WPKHBytes>,
    pub p2wshbytes: MetricPattern25<P2WSHBytes>,
}

impl CatalogTree_Addresses {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            first_p2aaddressindex: MetricPattern12::new(client.clone(), format!("{base_path}_first_p2aaddressindex")),
            first_p2pk33addressindex: MetricPattern12::new(client.clone(), format!("{base_path}_first_p2pk33addressindex")),
            first_p2pk65addressindex: MetricPattern12::new(client.clone(), format!("{base_path}_first_p2pk65addressindex")),
            first_p2pkhaddressindex: MetricPattern12::new(client.clone(), format!("{base_path}_first_p2pkhaddressindex")),
            first_p2shaddressindex: MetricPattern12::new(client.clone(), format!("{base_path}_first_p2shaddressindex")),
            first_p2traddressindex: MetricPattern12::new(client.clone(), format!("{base_path}_first_p2traddressindex")),
            first_p2wpkhaddressindex: MetricPattern12::new(client.clone(), format!("{base_path}_first_p2wpkhaddressindex")),
            first_p2wshaddressindex: MetricPattern12::new(client.clone(), format!("{base_path}_first_p2wshaddressindex")),
            p2abytes: MetricPattern17::new(client.clone(), format!("{base_path}_p2abytes")),
            p2pk33bytes: MetricPattern19::new(client.clone(), format!("{base_path}_p2pk33bytes")),
            p2pk65bytes: MetricPattern20::new(client.clone(), format!("{base_path}_p2pk65bytes")),
            p2pkhbytes: MetricPattern21::new(client.clone(), format!("{base_path}_p2pkhbytes")),
            p2shbytes: MetricPattern22::new(client.clone(), format!("{base_path}_p2shbytes")),
            p2trbytes: MetricPattern23::new(client.clone(), format!("{base_path}_p2trbytes")),
            p2wpkhbytes: MetricPattern24::new(client.clone(), format!("{base_path}_p2wpkhbytes")),
            p2wshbytes: MetricPattern25::new(client.clone(), format!("{base_path}_p2wshbytes")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Blocks {
    pub blockhash: MetricPattern12<BlockHash>,
    pub count: CatalogTree_Blocks_Count,
    pub difficulty: CatalogTree_Blocks_Difficulty,
    pub halving: CatalogTree_Blocks_Halving,
    pub interval: CatalogTree_Blocks_Interval,
    pub mining: CatalogTree_Blocks_Mining,
    pub rewards: CatalogTree_Blocks_Rewards,
    pub size: CatalogTree_Blocks_Size,
    pub time: CatalogTree_Blocks_Time,
    pub timestamp: MetricPattern12<Timestamp>,
    pub total_size: MetricPattern12<StoredU64>,
    pub weight: CatalogTree_Blocks_Weight,
}

impl CatalogTree_Blocks {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            blockhash: MetricPattern12::new(client.clone(), format!("{base_path}_blockhash")),
            count: CatalogTree_Blocks_Count::new(client.clone(), format!("{base_path}_count")),
            difficulty: CatalogTree_Blocks_Difficulty::new(client.clone(), format!("{base_path}_difficulty")),
            halving: CatalogTree_Blocks_Halving::new(client.clone(), format!("{base_path}_halving")),
            interval: CatalogTree_Blocks_Interval::new(client.clone(), format!("{base_path}_interval")),
            mining: CatalogTree_Blocks_Mining::new(client.clone(), format!("{base_path}_mining")),
            rewards: CatalogTree_Blocks_Rewards::new(client.clone(), format!("{base_path}_rewards")),
            size: CatalogTree_Blocks_Size::new(client.clone(), format!("{base_path}_size")),
            time: CatalogTree_Blocks_Time::new(client.clone(), format!("{base_path}_time")),
            timestamp: MetricPattern12::new(client.clone(), format!("{base_path}_timestamp")),
            total_size: MetricPattern12::new(client.clone(), format!("{base_path}_total_size")),
            weight: CatalogTree_Blocks_Weight::new(client.clone(), format!("{base_path}_weight")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Blocks_Count {
    pub _1m_block_count: MetricPattern1<StoredU32>,
    pub _1m_start: MetricPattern12<Height>,
    pub _1w_block_count: MetricPattern1<StoredU32>,
    pub _1w_start: MetricPattern12<Height>,
    pub _1y_block_count: MetricPattern1<StoredU32>,
    pub _1y_start: MetricPattern12<Height>,
    pub _24h_block_count: MetricPattern1<StoredU32>,
    pub _24h_start: MetricPattern12<Height>,
    pub block_count: BlockCountPattern<StoredU32>,
    pub block_count_target: MetricPattern4<StoredU64>,
}

impl CatalogTree_Blocks_Count {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _1m_block_count: MetricPattern1::new(client.clone(), format!("{base_path}_1m_block_count")),
            _1m_start: MetricPattern12::new(client.clone(), format!("{base_path}_1m_start")),
            _1w_block_count: MetricPattern1::new(client.clone(), format!("{base_path}_1w_block_count")),
            _1w_start: MetricPattern12::new(client.clone(), format!("{base_path}_1w_start")),
            _1y_block_count: MetricPattern1::new(client.clone(), format!("{base_path}_1y_block_count")),
            _1y_start: MetricPattern12::new(client.clone(), format!("{base_path}_1y_start")),
            _24h_block_count: MetricPattern1::new(client.clone(), format!("{base_path}_24h_block_count")),
            _24h_start: MetricPattern12::new(client.clone(), format!("{base_path}_24h_start")),
            block_count: BlockCountPattern::new(client.clone(), "block_count".to_string()),
            block_count_target: MetricPattern4::new(client.clone(), format!("{base_path}_block_count_target")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Blocks_Difficulty {
    pub base: MetricPattern12<StoredF64>,
    pub blocks_before_next_difficulty_adjustment: MetricPattern1<StoredU32>,
    pub days_before_next_difficulty_adjustment: MetricPattern1<StoredF32>,
    pub epoch: MetricPattern4<DifficultyEpoch>,
}

impl CatalogTree_Blocks_Difficulty {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            base: MetricPattern12::new(client.clone(), format!("{base_path}_base")),
            blocks_before_next_difficulty_adjustment: MetricPattern1::new(client.clone(), format!("{base_path}_blocks_before_next_difficulty_adjustment")),
            days_before_next_difficulty_adjustment: MetricPattern1::new(client.clone(), format!("{base_path}_days_before_next_difficulty_adjustment")),
            epoch: MetricPattern4::new(client.clone(), format!("{base_path}_epoch")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Blocks_Halving {
    pub blocks_before_next_halving: MetricPattern1<StoredU32>,
    pub days_before_next_halving: MetricPattern1<StoredF32>,
    pub epoch: MetricPattern4<HalvingEpoch>,
}

impl CatalogTree_Blocks_Halving {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            blocks_before_next_halving: MetricPattern1::new(client.clone(), format!("{base_path}_blocks_before_next_halving")),
            days_before_next_halving: MetricPattern1::new(client.clone(), format!("{base_path}_days_before_next_halving")),
            epoch: MetricPattern4::new(client.clone(), format!("{base_path}_epoch")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Blocks_Interval {
    pub average: MetricPattern2<Timestamp>,
    pub base: MetricPattern12<Timestamp>,
    pub max: MetricPattern2<Timestamp>,
    pub median: MetricPattern7<Timestamp>,
    pub min: MetricPattern2<Timestamp>,
    pub pct10: MetricPattern7<Timestamp>,
    pub pct25: MetricPattern7<Timestamp>,
    pub pct75: MetricPattern7<Timestamp>,
    pub pct90: MetricPattern7<Timestamp>,
}

impl CatalogTree_Blocks_Interval {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            average: MetricPattern2::new(client.clone(), format!("{base_path}_average")),
            base: MetricPattern12::new(client.clone(), format!("{base_path}_base")),
            max: MetricPattern2::new(client.clone(), format!("{base_path}_max")),
            median: MetricPattern7::new(client.clone(), format!("{base_path}_median")),
            min: MetricPattern2::new(client.clone(), format!("{base_path}_min")),
            pct10: MetricPattern7::new(client.clone(), format!("{base_path}_pct10")),
            pct25: MetricPattern7::new(client.clone(), format!("{base_path}_pct25")),
            pct75: MetricPattern7::new(client.clone(), format!("{base_path}_pct75")),
            pct90: MetricPattern7::new(client.clone(), format!("{base_path}_pct90")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Blocks_Mining {
    pub difficulty: MetricPattern2<StoredF64>,
    pub difficulty_adjustment: MetricPattern1<StoredF32>,
    pub difficulty_as_hash: MetricPattern1<StoredF32>,
    pub hash_price_phs: MetricPattern1<StoredF32>,
    pub hash_price_phs_min: MetricPattern1<StoredF32>,
    pub hash_price_rebound: MetricPattern1<StoredF32>,
    pub hash_price_ths: MetricPattern1<StoredF32>,
    pub hash_price_ths_min: MetricPattern1<StoredF32>,
    pub hash_rate: MetricPattern1<StoredF64>,
    pub hash_rate_1m_sma: MetricPattern4<StoredF32>,
    pub hash_rate_1w_sma: MetricPattern4<StoredF64>,
    pub hash_rate_1y_sma: MetricPattern4<StoredF32>,
    pub hash_rate_2m_sma: MetricPattern4<StoredF32>,
    pub hash_value_phs: MetricPattern1<StoredF32>,
    pub hash_value_phs_min: MetricPattern1<StoredF32>,
    pub hash_value_rebound: MetricPattern1<StoredF32>,
    pub hash_value_ths: MetricPattern1<StoredF32>,
    pub hash_value_ths_min: MetricPattern1<StoredF32>,
}

impl CatalogTree_Blocks_Mining {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            difficulty: MetricPattern2::new(client.clone(), format!("{base_path}_difficulty")),
            difficulty_adjustment: MetricPattern1::new(client.clone(), format!("{base_path}_difficulty_adjustment")),
            difficulty_as_hash: MetricPattern1::new(client.clone(), format!("{base_path}_difficulty_as_hash")),
            hash_price_phs: MetricPattern1::new(client.clone(), format!("{base_path}_hash_price_phs")),
            hash_price_phs_min: MetricPattern1::new(client.clone(), format!("{base_path}_hash_price_phs_min")),
            hash_price_rebound: MetricPattern1::new(client.clone(), format!("{base_path}_hash_price_rebound")),
            hash_price_ths: MetricPattern1::new(client.clone(), format!("{base_path}_hash_price_ths")),
            hash_price_ths_min: MetricPattern1::new(client.clone(), format!("{base_path}_hash_price_ths_min")),
            hash_rate: MetricPattern1::new(client.clone(), format!("{base_path}_hash_rate")),
            hash_rate_1m_sma: MetricPattern4::new(client.clone(), format!("{base_path}_hash_rate_1m_sma")),
            hash_rate_1w_sma: MetricPattern4::new(client.clone(), format!("{base_path}_hash_rate_1w_sma")),
            hash_rate_1y_sma: MetricPattern4::new(client.clone(), format!("{base_path}_hash_rate_1y_sma")),
            hash_rate_2m_sma: MetricPattern4::new(client.clone(), format!("{base_path}_hash_rate_2m_sma")),
            hash_value_phs: MetricPattern1::new(client.clone(), format!("{base_path}_hash_value_phs")),
            hash_value_phs_min: MetricPattern1::new(client.clone(), format!("{base_path}_hash_value_phs_min")),
            hash_value_rebound: MetricPattern1::new(client.clone(), format!("{base_path}_hash_value_rebound")),
            hash_value_ths: MetricPattern1::new(client.clone(), format!("{base_path}_hash_value_ths")),
            hash_value_ths_min: MetricPattern1::new(client.clone(), format!("{base_path}_hash_value_ths_min")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Blocks_Rewards {
    pub _24h_coinbase_sum: _24hCoinbaseSumPattern,
    pub coinbase: CoinbasePattern,
    pub fee_dominance: MetricPattern7<StoredF32>,
    pub subsidy: CoinbasePattern,
    pub subsidy_dominance: MetricPattern7<StoredF32>,
    pub subsidy_usd_1y_sma: MetricPattern4<Dollars>,
    pub unclaimed_rewards: UnclaimedRewardsPattern,
}

impl CatalogTree_Blocks_Rewards {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _24h_coinbase_sum: _24hCoinbaseSumPattern::new(client.clone(), "24h_coinbase_sum".to_string()),
            coinbase: CoinbasePattern::new(client.clone(), "coinbase".to_string()),
            fee_dominance: MetricPattern7::new(client.clone(), format!("{base_path}_fee_dominance")),
            subsidy: CoinbasePattern::new(client.clone(), "subsidy".to_string()),
            subsidy_dominance: MetricPattern7::new(client.clone(), format!("{base_path}_subsidy_dominance")),
            subsidy_usd_1y_sma: MetricPattern4::new(client.clone(), format!("{base_path}_subsidy_usd_1y_sma")),
            unclaimed_rewards: UnclaimedRewardsPattern::new(client.clone(), "unclaimed_rewards".to_string()),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Blocks_Size {
    pub size: SizePattern<StoredU64>,
    pub vbytes: CatalogTree_Blocks_Size_Vbytes,
}

impl CatalogTree_Blocks_Size {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            size: SizePattern::new(client.clone(), "block_size".to_string()),
            vbytes: CatalogTree_Blocks_Size_Vbytes::new(client.clone(), format!("{base_path}_vbytes")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Blocks_Size_Vbytes {
    pub average: MetricPattern2<StoredU64>,
    pub base: MetricPattern12<StoredU64>,
    pub cumulative: MetricPattern1<StoredU64>,
    pub max: MetricPattern2<StoredU64>,
    pub min: MetricPattern2<StoredU64>,
    pub percentiles: PercentilesPattern<StoredU64>,
    pub sum: MetricPattern2<StoredU64>,
}

impl CatalogTree_Blocks_Size_Vbytes {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            average: MetricPattern2::new(client.clone(), format!("{base_path}_average")),
            base: MetricPattern12::new(client.clone(), format!("{base_path}_base")),
            cumulative: MetricPattern1::new(client.clone(), format!("{base_path}_cumulative")),
            max: MetricPattern2::new(client.clone(), format!("{base_path}_max")),
            min: MetricPattern2::new(client.clone(), format!("{base_path}_min")),
            percentiles: PercentilesPattern::new(client.clone(), "block_vbytes".to_string()),
            sum: MetricPattern2::new(client.clone(), format!("{base_path}_sum")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Blocks_Time {
    pub date: MetricPattern12<Date>,
    pub date_fixed: MetricPattern12<Date>,
    pub timestamp: MetricPattern2<Timestamp>,
    pub timestamp_fixed: MetricPattern12<Timestamp>,
}

impl CatalogTree_Blocks_Time {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            date: MetricPattern12::new(client.clone(), format!("{base_path}_date")),
            date_fixed: MetricPattern12::new(client.clone(), format!("{base_path}_date_fixed")),
            timestamp: MetricPattern2::new(client.clone(), format!("{base_path}_timestamp")),
            timestamp_fixed: MetricPattern12::new(client.clone(), format!("{base_path}_timestamp_fixed")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Blocks_Weight {
    pub base: MetricPattern12<Weight>,
    pub fullness: BitcoinPattern<StoredF32>,
    pub weight: SizePattern<Weight>,
}

impl CatalogTree_Blocks_Weight {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            base: MetricPattern12::new(client.clone(), format!("{base_path}_base")),
            fullness: BitcoinPattern::new(client.clone(), "block_fullness".to_string()),
            weight: SizePattern::new(client.clone(), "block_weight".to_string()),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Cointime {
    pub activity: CatalogTree_Cointime_Activity,
    pub adjusted: CatalogTree_Cointime_Adjusted,
    pub cap: CatalogTree_Cointime_Cap,
    pub pricing: CatalogTree_Cointime_Pricing,
    pub supply: CatalogTree_Cointime_Supply,
    pub value: CatalogTree_Cointime_Value,
}

impl CatalogTree_Cointime {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            activity: CatalogTree_Cointime_Activity::new(client.clone(), format!("{base_path}_activity")),
            adjusted: CatalogTree_Cointime_Adjusted::new(client.clone(), format!("{base_path}_adjusted")),
            cap: CatalogTree_Cointime_Cap::new(client.clone(), format!("{base_path}_cap")),
            pricing: CatalogTree_Cointime_Pricing::new(client.clone(), format!("{base_path}_pricing")),
            supply: CatalogTree_Cointime_Supply::new(client.clone(), format!("{base_path}_supply")),
            value: CatalogTree_Cointime_Value::new(client.clone(), format!("{base_path}_value")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Cointime_Activity {
    pub activity_to_vaultedness_ratio: MetricPattern1<StoredF64>,
    pub coinblocks_created: BlockCountPattern<StoredF64>,
    pub coinblocks_stored: BlockCountPattern<StoredF64>,
    pub liveliness: MetricPattern1<StoredF64>,
    pub vaultedness: MetricPattern1<StoredF64>,
}

impl CatalogTree_Cointime_Activity {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            activity_to_vaultedness_ratio: MetricPattern1::new(client.clone(), format!("{base_path}_activity_to_vaultedness_ratio")),
            coinblocks_created: BlockCountPattern::new(client.clone(), "coinblocks_created".to_string()),
            coinblocks_stored: BlockCountPattern::new(client.clone(), "coinblocks_stored".to_string()),
            liveliness: MetricPattern1::new(client.clone(), format!("{base_path}_liveliness")),
            vaultedness: MetricPattern1::new(client.clone(), format!("{base_path}_vaultedness")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Cointime_Adjusted {
    pub cointime_adj_inflation_rate: MetricPattern4<StoredF32>,
    pub cointime_adj_tx_btc_velocity: MetricPattern4<StoredF64>,
    pub cointime_adj_tx_usd_velocity: MetricPattern4<StoredF64>,
}

impl CatalogTree_Cointime_Adjusted {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            cointime_adj_inflation_rate: MetricPattern4::new(client.clone(), format!("{base_path}_cointime_adj_inflation_rate")),
            cointime_adj_tx_btc_velocity: MetricPattern4::new(client.clone(), format!("{base_path}_cointime_adj_tx_btc_velocity")),
            cointime_adj_tx_usd_velocity: MetricPattern4::new(client.clone(), format!("{base_path}_cointime_adj_tx_usd_velocity")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Cointime_Cap {
    pub active_cap: MetricPattern1<Dollars>,
    pub cointime_cap: MetricPattern1<Dollars>,
    pub investor_cap: MetricPattern1<Dollars>,
    pub thermo_cap: MetricPattern1<Dollars>,
    pub vaulted_cap: MetricPattern1<Dollars>,
}

impl CatalogTree_Cointime_Cap {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            active_cap: MetricPattern1::new(client.clone(), format!("{base_path}_active_cap")),
            cointime_cap: MetricPattern1::new(client.clone(), format!("{base_path}_cointime_cap")),
            investor_cap: MetricPattern1::new(client.clone(), format!("{base_path}_investor_cap")),
            thermo_cap: MetricPattern1::new(client.clone(), format!("{base_path}_thermo_cap")),
            vaulted_cap: MetricPattern1::new(client.clone(), format!("{base_path}_vaulted_cap")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Cointime_Pricing {
    pub active_price: MetricPattern1<Dollars>,
    pub active_price_ratio: ActivePriceRatioPattern,
    pub cointime_price: MetricPattern1<Dollars>,
    pub cointime_price_ratio: ActivePriceRatioPattern,
    pub true_market_mean: MetricPattern1<Dollars>,
    pub true_market_mean_ratio: ActivePriceRatioPattern,
    pub vaulted_price: MetricPattern1<Dollars>,
    pub vaulted_price_ratio: ActivePriceRatioPattern,
}

impl CatalogTree_Cointime_Pricing {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            active_price: MetricPattern1::new(client.clone(), format!("{base_path}_active_price")),
            active_price_ratio: ActivePriceRatioPattern::new(client.clone(), "active_price_ratio".to_string()),
            cointime_price: MetricPattern1::new(client.clone(), format!("{base_path}_cointime_price")),
            cointime_price_ratio: ActivePriceRatioPattern::new(client.clone(), "cointime_price_ratio".to_string()),
            true_market_mean: MetricPattern1::new(client.clone(), format!("{base_path}_true_market_mean")),
            true_market_mean_ratio: ActivePriceRatioPattern::new(client.clone(), "true_market_mean_ratio".to_string()),
            vaulted_price: MetricPattern1::new(client.clone(), format!("{base_path}_vaulted_price")),
            vaulted_price_ratio: ActivePriceRatioPattern::new(client.clone(), "vaulted_price_ratio".to_string()),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Cointime_Supply {
    pub active_supply: _24hCoinbaseSumPattern,
    pub vaulted_supply: _24hCoinbaseSumPattern,
}

impl CatalogTree_Cointime_Supply {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            active_supply: _24hCoinbaseSumPattern::new(client.clone(), "active_supply".to_string()),
            vaulted_supply: _24hCoinbaseSumPattern::new(client.clone(), "vaulted_supply".to_string()),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Cointime_Value {
    pub cointime_value_created: BlockCountPattern<StoredF64>,
    pub cointime_value_destroyed: BlockCountPattern<StoredF64>,
    pub cointime_value_stored: BlockCountPattern<StoredF64>,
}

impl CatalogTree_Cointime_Value {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            cointime_value_created: BlockCountPattern::new(client.clone(), "cointime_value_created".to_string()),
            cointime_value_destroyed: BlockCountPattern::new(client.clone(), "cointime_value_destroyed".to_string()),
            cointime_value_stored: BlockCountPattern::new(client.clone(), "cointime_value_stored".to_string()),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Constants {
    pub constant_0: MetricPattern3<StoredU16>,
    pub constant_1: MetricPattern3<StoredU16>,
    pub constant_100: MetricPattern3<StoredU16>,
    pub constant_2: MetricPattern3<StoredU16>,
    pub constant_20: MetricPattern3<StoredU16>,
    pub constant_3: MetricPattern3<StoredU16>,
    pub constant_30: MetricPattern3<StoredU16>,
    pub constant_38_2: MetricPattern3<StoredF32>,
    pub constant_4: MetricPattern3<StoredU16>,
    pub constant_50: MetricPattern3<StoredU16>,
    pub constant_600: MetricPattern3<StoredU16>,
    pub constant_61_8: MetricPattern3<StoredF32>,
    pub constant_70: MetricPattern3<StoredU16>,
    pub constant_80: MetricPattern3<StoredU16>,
    pub constant_minus_1: MetricPattern3<StoredI16>,
    pub constant_minus_2: MetricPattern3<StoredI16>,
    pub constant_minus_3: MetricPattern3<StoredI16>,
    pub constant_minus_4: MetricPattern3<StoredI16>,
}

impl CatalogTree_Constants {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            constant_0: MetricPattern3::new(client.clone(), format!("{base_path}_constant_0")),
            constant_1: MetricPattern3::new(client.clone(), format!("{base_path}_constant_1")),
            constant_100: MetricPattern3::new(client.clone(), format!("{base_path}_constant_100")),
            constant_2: MetricPattern3::new(client.clone(), format!("{base_path}_constant_2")),
            constant_20: MetricPattern3::new(client.clone(), format!("{base_path}_constant_20")),
            constant_3: MetricPattern3::new(client.clone(), format!("{base_path}_constant_3")),
            constant_30: MetricPattern3::new(client.clone(), format!("{base_path}_constant_30")),
            constant_38_2: MetricPattern3::new(client.clone(), format!("{base_path}_constant_38_2")),
            constant_4: MetricPattern3::new(client.clone(), format!("{base_path}_constant_4")),
            constant_50: MetricPattern3::new(client.clone(), format!("{base_path}_constant_50")),
            constant_600: MetricPattern3::new(client.clone(), format!("{base_path}_constant_600")),
            constant_61_8: MetricPattern3::new(client.clone(), format!("{base_path}_constant_61_8")),
            constant_70: MetricPattern3::new(client.clone(), format!("{base_path}_constant_70")),
            constant_80: MetricPattern3::new(client.clone(), format!("{base_path}_constant_80")),
            constant_minus_1: MetricPattern3::new(client.clone(), format!("{base_path}_constant_minus_1")),
            constant_minus_2: MetricPattern3::new(client.clone(), format!("{base_path}_constant_minus_2")),
            constant_minus_3: MetricPattern3::new(client.clone(), format!("{base_path}_constant_minus_3")),
            constant_minus_4: MetricPattern3::new(client.clone(), format!("{base_path}_constant_minus_4")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Distribution {
    pub addr_count: AddrCountPattern,
    pub address_cohorts: CatalogTree_Distribution_AddressCohorts,
    pub addresses_data: CatalogTree_Distribution_AddressesData,
    pub any_address_indexes: CatalogTree_Distribution_AnyAddressIndexes,
    pub chain_state: MetricPattern12<SupplyState>,
    pub empty_addr_count: AddrCountPattern,
    pub emptyaddressindex: MetricPattern33<EmptyAddressIndex>,
    pub loadedaddressindex: MetricPattern32<LoadedAddressIndex>,
    pub utxo_cohorts: CatalogTree_Distribution_UtxoCohorts,
}

impl CatalogTree_Distribution {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            addr_count: AddrCountPattern::new(client.clone(), "addr_count".to_string()),
            address_cohorts: CatalogTree_Distribution_AddressCohorts::new(client.clone(), format!("{base_path}_address_cohorts")),
            addresses_data: CatalogTree_Distribution_AddressesData::new(client.clone(), format!("{base_path}_addresses_data")),
            any_address_indexes: CatalogTree_Distribution_AnyAddressIndexes::new(client.clone(), format!("{base_path}_any_address_indexes")),
            chain_state: MetricPattern12::new(client.clone(), format!("{base_path}_chain_state")),
            empty_addr_count: AddrCountPattern::new(client.clone(), "empty_addr_count".to_string()),
            emptyaddressindex: MetricPattern33::new(client.clone(), format!("{base_path}_emptyaddressindex")),
            loadedaddressindex: MetricPattern32::new(client.clone(), format!("{base_path}_loadedaddressindex")),
            utxo_cohorts: CatalogTree_Distribution_UtxoCohorts::new(client.clone(), format!("{base_path}_utxo_cohorts")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Distribution_AddressCohorts {
    pub amount_range: CatalogTree_Distribution_AddressCohorts_AmountRange,
    pub ge_amount: CatalogTree_Distribution_AddressCohorts_GeAmount,
    pub lt_amount: CatalogTree_Distribution_AddressCohorts_LtAmount,
}

impl CatalogTree_Distribution_AddressCohorts {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            amount_range: CatalogTree_Distribution_AddressCohorts_AmountRange::new(client.clone(), format!("{base_path}_amount_range")),
            ge_amount: CatalogTree_Distribution_AddressCohorts_GeAmount::new(client.clone(), format!("{base_path}_ge_amount")),
            lt_amount: CatalogTree_Distribution_AddressCohorts_LtAmount::new(client.clone(), format!("{base_path}_lt_amount")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Distribution_AddressCohorts_AmountRange {
    pub _0sats: _0satsPattern,
    pub _100btc_to_1k_btc: _0satsPattern,
    pub _100k_btc_or_more: _0satsPattern,
    pub _100k_sats_to_1m_sats: _0satsPattern,
    pub _100sats_to_1k_sats: _0satsPattern,
    pub _10btc_to_100btc: _0satsPattern,
    pub _10k_btc_to_100k_btc: _0satsPattern,
    pub _10k_sats_to_100k_sats: _0satsPattern,
    pub _10m_sats_to_1btc: _0satsPattern,
    pub _10sats_to_100sats: _0satsPattern,
    pub _1btc_to_10btc: _0satsPattern,
    pub _1k_btc_to_10k_btc: _0satsPattern,
    pub _1k_sats_to_10k_sats: _0satsPattern,
    pub _1m_sats_to_10m_sats: _0satsPattern,
    pub _1sat_to_10sats: _0satsPattern,
}

impl CatalogTree_Distribution_AddressCohorts_AmountRange {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _0sats: _0satsPattern::new(client.clone(), "addrs_with_0sats".to_string()),
            _100btc_to_1k_btc: _0satsPattern::new(client.clone(), "addrs_above_100btc_under_1k_btc".to_string()),
            _100k_btc_or_more: _0satsPattern::new(client.clone(), "addrs_above_100k_btc".to_string()),
            _100k_sats_to_1m_sats: _0satsPattern::new(client.clone(), "addrs_above_100k_sats_under_1m_sats".to_string()),
            _100sats_to_1k_sats: _0satsPattern::new(client.clone(), "addrs_above_100sats_under_1k_sats".to_string()),
            _10btc_to_100btc: _0satsPattern::new(client.clone(), "addrs_above_10btc_under_100btc".to_string()),
            _10k_btc_to_100k_btc: _0satsPattern::new(client.clone(), "addrs_above_10k_btc_under_100k_btc".to_string()),
            _10k_sats_to_100k_sats: _0satsPattern::new(client.clone(), "addrs_above_10k_sats_under_100k_sats".to_string()),
            _10m_sats_to_1btc: _0satsPattern::new(client.clone(), "addrs_above_10m_sats_under_1btc".to_string()),
            _10sats_to_100sats: _0satsPattern::new(client.clone(), "addrs_above_10sats_under_100sats".to_string()),
            _1btc_to_10btc: _0satsPattern::new(client.clone(), "addrs_above_1btc_under_10btc".to_string()),
            _1k_btc_to_10k_btc: _0satsPattern::new(client.clone(), "addrs_above_1k_btc_under_10k_btc".to_string()),
            _1k_sats_to_10k_sats: _0satsPattern::new(client.clone(), "addrs_above_1k_sats_under_10k_sats".to_string()),
            _1m_sats_to_10m_sats: _0satsPattern::new(client.clone(), "addrs_above_1m_sats_under_10m_sats".to_string()),
            _1sat_to_10sats: _0satsPattern::new(client.clone(), "addrs_above_1sat_under_10sats".to_string()),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Distribution_AddressCohorts_GeAmount {
    pub _100btc: _0satsPattern,
    pub _100k_sats: _0satsPattern,
    pub _100sats: _0satsPattern,
    pub _10btc: _0satsPattern,
    pub _10k_btc: _0satsPattern,
    pub _10k_sats: _0satsPattern,
    pub _10m_sats: _0satsPattern,
    pub _10sats: _0satsPattern,
    pub _1btc: _0satsPattern,
    pub _1k_btc: _0satsPattern,
    pub _1k_sats: _0satsPattern,
    pub _1m_sats: _0satsPattern,
    pub _1sat: _0satsPattern,
}

impl CatalogTree_Distribution_AddressCohorts_GeAmount {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _100btc: _0satsPattern::new(client.clone(), "addrs_above_100btc".to_string()),
            _100k_sats: _0satsPattern::new(client.clone(), "addrs_above_100k_sats".to_string()),
            _100sats: _0satsPattern::new(client.clone(), "addrs_above_100sats".to_string()),
            _10btc: _0satsPattern::new(client.clone(), "addrs_above_10btc".to_string()),
            _10k_btc: _0satsPattern::new(client.clone(), "addrs_above_10k_btc".to_string()),
            _10k_sats: _0satsPattern::new(client.clone(), "addrs_above_10k_sats".to_string()),
            _10m_sats: _0satsPattern::new(client.clone(), "addrs_above_10m_sats".to_string()),
            _10sats: _0satsPattern::new(client.clone(), "addrs_above_10sats".to_string()),
            _1btc: _0satsPattern::new(client.clone(), "addrs_above_1btc".to_string()),
            _1k_btc: _0satsPattern::new(client.clone(), "addrs_above_1k_btc".to_string()),
            _1k_sats: _0satsPattern::new(client.clone(), "addrs_above_1k_sats".to_string()),
            _1m_sats: _0satsPattern::new(client.clone(), "addrs_above_1m_sats".to_string()),
            _1sat: _0satsPattern::new(client.clone(), "addrs_above_1sat".to_string()),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Distribution_AddressCohorts_LtAmount {
    pub _100btc: _0satsPattern,
    pub _100k_btc: _0satsPattern,
    pub _100k_sats: _0satsPattern,
    pub _100sats: _0satsPattern,
    pub _10btc: _0satsPattern,
    pub _10k_btc: _0satsPattern,
    pub _10k_sats: _0satsPattern,
    pub _10m_sats: _0satsPattern,
    pub _10sats: _0satsPattern,
    pub _1btc: _0satsPattern,
    pub _1k_btc: _0satsPattern,
    pub _1k_sats: _0satsPattern,
    pub _1m_sats: _0satsPattern,
}

impl CatalogTree_Distribution_AddressCohorts_LtAmount {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _100btc: _0satsPattern::new(client.clone(), "addrs_under_100btc".to_string()),
            _100k_btc: _0satsPattern::new(client.clone(), "addrs_under_100k_btc".to_string()),
            _100k_sats: _0satsPattern::new(client.clone(), "addrs_under_100k_sats".to_string()),
            _100sats: _0satsPattern::new(client.clone(), "addrs_under_100sats".to_string()),
            _10btc: _0satsPattern::new(client.clone(), "addrs_under_10btc".to_string()),
            _10k_btc: _0satsPattern::new(client.clone(), "addrs_under_10k_btc".to_string()),
            _10k_sats: _0satsPattern::new(client.clone(), "addrs_under_10k_sats".to_string()),
            _10m_sats: _0satsPattern::new(client.clone(), "addrs_under_10m_sats".to_string()),
            _10sats: _0satsPattern::new(client.clone(), "addrs_under_10sats".to_string()),
            _1btc: _0satsPattern::new(client.clone(), "addrs_under_1btc".to_string()),
            _1k_btc: _0satsPattern::new(client.clone(), "addrs_under_1k_btc".to_string()),
            _1k_sats: _0satsPattern::new(client.clone(), "addrs_under_1k_sats".to_string()),
            _1m_sats: _0satsPattern::new(client.clone(), "addrs_under_1m_sats".to_string()),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Distribution_AddressesData {
    pub empty: MetricPattern33<EmptyAddressData>,
    pub loaded: MetricPattern32<LoadedAddressData>,
}

impl CatalogTree_Distribution_AddressesData {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            empty: MetricPattern33::new(client.clone(), format!("{base_path}_empty")),
            loaded: MetricPattern32::new(client.clone(), format!("{base_path}_loaded")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Distribution_AnyAddressIndexes {
    pub p2a: MetricPattern17<AnyAddressIndex>,
    pub p2pk33: MetricPattern19<AnyAddressIndex>,
    pub p2pk65: MetricPattern20<AnyAddressIndex>,
    pub p2pkh: MetricPattern21<AnyAddressIndex>,
    pub p2sh: MetricPattern22<AnyAddressIndex>,
    pub p2tr: MetricPattern23<AnyAddressIndex>,
    pub p2wpkh: MetricPattern24<AnyAddressIndex>,
    pub p2wsh: MetricPattern25<AnyAddressIndex>,
}

impl CatalogTree_Distribution_AnyAddressIndexes {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            p2a: MetricPattern17::new(client.clone(), format!("{base_path}_p2a")),
            p2pk33: MetricPattern19::new(client.clone(), format!("{base_path}_p2pk33")),
            p2pk65: MetricPattern20::new(client.clone(), format!("{base_path}_p2pk65")),
            p2pkh: MetricPattern21::new(client.clone(), format!("{base_path}_p2pkh")),
            p2sh: MetricPattern22::new(client.clone(), format!("{base_path}_p2sh")),
            p2tr: MetricPattern23::new(client.clone(), format!("{base_path}_p2tr")),
            p2wpkh: MetricPattern24::new(client.clone(), format!("{base_path}_p2wpkh")),
            p2wsh: MetricPattern25::new(client.clone(), format!("{base_path}_p2wsh")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Distribution_UtxoCohorts {
    pub age_range: CatalogTree_Distribution_UtxoCohorts_AgeRange,
    pub all: CatalogTree_Distribution_UtxoCohorts_All,
    pub amount_range: CatalogTree_Distribution_UtxoCohorts_AmountRange,
    pub epoch: CatalogTree_Distribution_UtxoCohorts_Epoch,
    pub ge_amount: CatalogTree_Distribution_UtxoCohorts_GeAmount,
    pub lt_amount: CatalogTree_Distribution_UtxoCohorts_LtAmount,
    pub max_age: CatalogTree_Distribution_UtxoCohorts_MaxAge,
    pub min_age: CatalogTree_Distribution_UtxoCohorts_MinAge,
    pub term: CatalogTree_Distribution_UtxoCohorts_Term,
    pub type_: CatalogTree_Distribution_UtxoCohorts_Type,
    pub year: CatalogTree_Distribution_UtxoCohorts_Year,
}

impl CatalogTree_Distribution_UtxoCohorts {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            age_range: CatalogTree_Distribution_UtxoCohorts_AgeRange::new(client.clone(), format!("{base_path}_age_range")),
            all: CatalogTree_Distribution_UtxoCohorts_All::new(client.clone(), format!("{base_path}_all")),
            amount_range: CatalogTree_Distribution_UtxoCohorts_AmountRange::new(client.clone(), format!("{base_path}_amount_range")),
            epoch: CatalogTree_Distribution_UtxoCohorts_Epoch::new(client.clone(), format!("{base_path}_epoch")),
            ge_amount: CatalogTree_Distribution_UtxoCohorts_GeAmount::new(client.clone(), format!("{base_path}_ge_amount")),
            lt_amount: CatalogTree_Distribution_UtxoCohorts_LtAmount::new(client.clone(), format!("{base_path}_lt_amount")),
            max_age: CatalogTree_Distribution_UtxoCohorts_MaxAge::new(client.clone(), format!("{base_path}_max_age")),
            min_age: CatalogTree_Distribution_UtxoCohorts_MinAge::new(client.clone(), format!("{base_path}_min_age")),
            term: CatalogTree_Distribution_UtxoCohorts_Term::new(client.clone(), format!("{base_path}_term")),
            type_: CatalogTree_Distribution_UtxoCohorts_Type::new(client.clone(), format!("{base_path}_type_")),
            year: CatalogTree_Distribution_UtxoCohorts_Year::new(client.clone(), format!("{base_path}_year")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Distribution_UtxoCohorts_AgeRange {
    pub _10y_to_12y: _10yTo12yPattern,
    pub _12y_to_15y: _10yTo12yPattern,
    pub _1d_to_1w: _10yTo12yPattern,
    pub _1h_to_1d: _10yTo12yPattern,
    pub _1m_to_2m: _10yTo12yPattern,
    pub _1w_to_1m: _10yTo12yPattern,
    pub _1y_to_2y: _10yTo12yPattern,
    pub _2m_to_3m: _10yTo12yPattern,
    pub _2y_to_3y: _10yTo12yPattern,
    pub _3m_to_4m: _10yTo12yPattern,
    pub _3y_to_4y: _10yTo12yPattern,
    pub _4m_to_5m: _10yTo12yPattern,
    pub _4y_to_5y: _10yTo12yPattern,
    pub _5m_to_6m: _10yTo12yPattern,
    pub _5y_to_6y: _10yTo12yPattern,
    pub _6m_to_1y: _10yTo12yPattern,
    pub _6y_to_7y: _10yTo12yPattern,
    pub _7y_to_8y: _10yTo12yPattern,
    pub _8y_to_10y: _10yTo12yPattern,
    pub from_15y: _10yTo12yPattern,
    pub up_to_1h: _10yTo12yPattern,
}

impl CatalogTree_Distribution_UtxoCohorts_AgeRange {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _10y_to_12y: _10yTo12yPattern::new(client.clone(), "utxos_at_least_10y_up_to_12y_old".to_string()),
            _12y_to_15y: _10yTo12yPattern::new(client.clone(), "utxos_at_least_12y_up_to_15y_old".to_string()),
            _1d_to_1w: _10yTo12yPattern::new(client.clone(), "utxos_at_least_1d_up_to_1w_old".to_string()),
            _1h_to_1d: _10yTo12yPattern::new(client.clone(), "utxos_at_least_1h_up_to_1d_old".to_string()),
            _1m_to_2m: _10yTo12yPattern::new(client.clone(), "utxos_at_least_1m_up_to_2m_old".to_string()),
            _1w_to_1m: _10yTo12yPattern::new(client.clone(), "utxos_at_least_1w_up_to_1m_old".to_string()),
            _1y_to_2y: _10yTo12yPattern::new(client.clone(), "utxos_at_least_1y_up_to_2y_old".to_string()),
            _2m_to_3m: _10yTo12yPattern::new(client.clone(), "utxos_at_least_2m_up_to_3m_old".to_string()),
            _2y_to_3y: _10yTo12yPattern::new(client.clone(), "utxos_at_least_2y_up_to_3y_old".to_string()),
            _3m_to_4m: _10yTo12yPattern::new(client.clone(), "utxos_at_least_3m_up_to_4m_old".to_string()),
            _3y_to_4y: _10yTo12yPattern::new(client.clone(), "utxos_at_least_3y_up_to_4y_old".to_string()),
            _4m_to_5m: _10yTo12yPattern::new(client.clone(), "utxos_at_least_4m_up_to_5m_old".to_string()),
            _4y_to_5y: _10yTo12yPattern::new(client.clone(), "utxos_at_least_4y_up_to_5y_old".to_string()),
            _5m_to_6m: _10yTo12yPattern::new(client.clone(), "utxos_at_least_5m_up_to_6m_old".to_string()),
            _5y_to_6y: _10yTo12yPattern::new(client.clone(), "utxos_at_least_5y_up_to_6y_old".to_string()),
            _6m_to_1y: _10yTo12yPattern::new(client.clone(), "utxos_at_least_6m_up_to_1y_old".to_string()),
            _6y_to_7y: _10yTo12yPattern::new(client.clone(), "utxos_at_least_6y_up_to_7y_old".to_string()),
            _7y_to_8y: _10yTo12yPattern::new(client.clone(), "utxos_at_least_7y_up_to_8y_old".to_string()),
            _8y_to_10y: _10yTo12yPattern::new(client.clone(), "utxos_at_least_8y_up_to_10y_old".to_string()),
            from_15y: _10yTo12yPattern::new(client.clone(), "utxos_at_least_15y_old".to_string()),
            up_to_1h: _10yTo12yPattern::new(client.clone(), "utxos_up_to_1h_old".to_string()),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Distribution_UtxoCohorts_All {
    pub activity: ActivityPattern2,
    pub cost_basis: CostBasisPattern2,
    pub outputs: OutputsPattern,
    pub realized: RealizedPattern3,
    pub relative: CatalogTree_Distribution_UtxoCohorts_All_Relative,
    pub supply: SupplyPattern2,
    pub unrealized: UnrealizedPattern,
}

impl CatalogTree_Distribution_UtxoCohorts_All {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            activity: ActivityPattern2::new(client.clone(), "".to_string()),
            cost_basis: CostBasisPattern2::new(client.clone(), "".to_string()),
            outputs: OutputsPattern::new(client.clone(), "utxo_count".to_string()),
            realized: RealizedPattern3::new(client.clone(), "".to_string()),
            relative: CatalogTree_Distribution_UtxoCohorts_All_Relative::new(client.clone(), format!("{base_path}_relative")),
            supply: SupplyPattern2::new(client.clone(), "supply".to_string()),
            unrealized: UnrealizedPattern::new(client.clone(), "".to_string()),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Distribution_UtxoCohorts_All_Relative {
    pub neg_unrealized_loss_rel_to_own_total_unrealized_pnl: MetricPattern1<StoredF32>,
    pub net_unrealized_pnl_rel_to_own_total_unrealized_pnl: MetricPattern1<StoredF32>,
    pub supply_in_loss_rel_to_own_supply: MetricPattern1<StoredF64>,
    pub supply_in_profit_rel_to_own_supply: MetricPattern1<StoredF64>,
    pub unrealized_loss_rel_to_own_total_unrealized_pnl: MetricPattern1<StoredF32>,
    pub unrealized_profit_rel_to_own_total_unrealized_pnl: MetricPattern1<StoredF32>,
}

impl CatalogTree_Distribution_UtxoCohorts_All_Relative {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            neg_unrealized_loss_rel_to_own_total_unrealized_pnl: MetricPattern1::new(client.clone(), format!("{base_path}_neg_unrealized_loss_rel_to_own_total_unrealized_pnl")),
            net_unrealized_pnl_rel_to_own_total_unrealized_pnl: MetricPattern1::new(client.clone(), format!("{base_path}_net_unrealized_pnl_rel_to_own_total_unrealized_pnl")),
            supply_in_loss_rel_to_own_supply: MetricPattern1::new(client.clone(), format!("{base_path}_supply_in_loss_rel_to_own_supply")),
            supply_in_profit_rel_to_own_supply: MetricPattern1::new(client.clone(), format!("{base_path}_supply_in_profit_rel_to_own_supply")),
            unrealized_loss_rel_to_own_total_unrealized_pnl: MetricPattern1::new(client.clone(), format!("{base_path}_unrealized_loss_rel_to_own_total_unrealized_pnl")),
            unrealized_profit_rel_to_own_total_unrealized_pnl: MetricPattern1::new(client.clone(), format!("{base_path}_unrealized_profit_rel_to_own_total_unrealized_pnl")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Distribution_UtxoCohorts_AmountRange {
    pub _0sats: _0satsPattern2,
    pub _100btc_to_1k_btc: _0satsPattern2,
    pub _100k_btc_or_more: _0satsPattern2,
    pub _100k_sats_to_1m_sats: _0satsPattern2,
    pub _100sats_to_1k_sats: _0satsPattern2,
    pub _10btc_to_100btc: _0satsPattern2,
    pub _10k_btc_to_100k_btc: _0satsPattern2,
    pub _10k_sats_to_100k_sats: _0satsPattern2,
    pub _10m_sats_to_1btc: _0satsPattern2,
    pub _10sats_to_100sats: _0satsPattern2,
    pub _1btc_to_10btc: _0satsPattern2,
    pub _1k_btc_to_10k_btc: _0satsPattern2,
    pub _1k_sats_to_10k_sats: _0satsPattern2,
    pub _1m_sats_to_10m_sats: _0satsPattern2,
    pub _1sat_to_10sats: _0satsPattern2,
}

impl CatalogTree_Distribution_UtxoCohorts_AmountRange {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _0sats: _0satsPattern2::new(client.clone(), "utxos_with_0sats".to_string()),
            _100btc_to_1k_btc: _0satsPattern2::new(client.clone(), "utxos_above_100btc_under_1k_btc".to_string()),
            _100k_btc_or_more: _0satsPattern2::new(client.clone(), "utxos_above_100k_btc".to_string()),
            _100k_sats_to_1m_sats: _0satsPattern2::new(client.clone(), "utxos_above_100k_sats_under_1m_sats".to_string()),
            _100sats_to_1k_sats: _0satsPattern2::new(client.clone(), "utxos_above_100sats_under_1k_sats".to_string()),
            _10btc_to_100btc: _0satsPattern2::new(client.clone(), "utxos_above_10btc_under_100btc".to_string()),
            _10k_btc_to_100k_btc: _0satsPattern2::new(client.clone(), "utxos_above_10k_btc_under_100k_btc".to_string()),
            _10k_sats_to_100k_sats: _0satsPattern2::new(client.clone(), "utxos_above_10k_sats_under_100k_sats".to_string()),
            _10m_sats_to_1btc: _0satsPattern2::new(client.clone(), "utxos_above_10m_sats_under_1btc".to_string()),
            _10sats_to_100sats: _0satsPattern2::new(client.clone(), "utxos_above_10sats_under_100sats".to_string()),
            _1btc_to_10btc: _0satsPattern2::new(client.clone(), "utxos_above_1btc_under_10btc".to_string()),
            _1k_btc_to_10k_btc: _0satsPattern2::new(client.clone(), "utxos_above_1k_btc_under_10k_btc".to_string()),
            _1k_sats_to_10k_sats: _0satsPattern2::new(client.clone(), "utxos_above_1k_sats_under_10k_sats".to_string()),
            _1m_sats_to_10m_sats: _0satsPattern2::new(client.clone(), "utxos_above_1m_sats_under_10m_sats".to_string()),
            _1sat_to_10sats: _0satsPattern2::new(client.clone(), "utxos_above_1sat_under_10sats".to_string()),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Distribution_UtxoCohorts_Epoch {
    pub _0: _0satsPattern2,
    pub _1: _0satsPattern2,
    pub _2: _0satsPattern2,
    pub _3: _0satsPattern2,
    pub _4: _0satsPattern2,
}

impl CatalogTree_Distribution_UtxoCohorts_Epoch {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _0: _0satsPattern2::new(client.clone(), "epoch_0".to_string()),
            _1: _0satsPattern2::new(client.clone(), "epoch_1".to_string()),
            _2: _0satsPattern2::new(client.clone(), "epoch_2".to_string()),
            _3: _0satsPattern2::new(client.clone(), "epoch_3".to_string()),
            _4: _0satsPattern2::new(client.clone(), "epoch_4".to_string()),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Distribution_UtxoCohorts_GeAmount {
    pub _100btc: _100btcPattern,
    pub _100k_sats: _100btcPattern,
    pub _100sats: _100btcPattern,
    pub _10btc: _100btcPattern,
    pub _10k_btc: _100btcPattern,
    pub _10k_sats: _100btcPattern,
    pub _10m_sats: _100btcPattern,
    pub _10sats: _100btcPattern,
    pub _1btc: _100btcPattern,
    pub _1k_btc: _100btcPattern,
    pub _1k_sats: _100btcPattern,
    pub _1m_sats: _100btcPattern,
    pub _1sat: _100btcPattern,
}

impl CatalogTree_Distribution_UtxoCohorts_GeAmount {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _100btc: _100btcPattern::new(client.clone(), "utxos_above_100btc".to_string()),
            _100k_sats: _100btcPattern::new(client.clone(), "utxos_above_100k_sats".to_string()),
            _100sats: _100btcPattern::new(client.clone(), "utxos_above_100sats".to_string()),
            _10btc: _100btcPattern::new(client.clone(), "utxos_above_10btc".to_string()),
            _10k_btc: _100btcPattern::new(client.clone(), "utxos_above_10k_btc".to_string()),
            _10k_sats: _100btcPattern::new(client.clone(), "utxos_above_10k_sats".to_string()),
            _10m_sats: _100btcPattern::new(client.clone(), "utxos_above_10m_sats".to_string()),
            _10sats: _100btcPattern::new(client.clone(), "utxos_above_10sats".to_string()),
            _1btc: _100btcPattern::new(client.clone(), "utxos_above_1btc".to_string()),
            _1k_btc: _100btcPattern::new(client.clone(), "utxos_above_1k_btc".to_string()),
            _1k_sats: _100btcPattern::new(client.clone(), "utxos_above_1k_sats".to_string()),
            _1m_sats: _100btcPattern::new(client.clone(), "utxos_above_1m_sats".to_string()),
            _1sat: _100btcPattern::new(client.clone(), "utxos_above_1sat".to_string()),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Distribution_UtxoCohorts_LtAmount {
    pub _100btc: _100btcPattern,
    pub _100k_btc: _100btcPattern,
    pub _100k_sats: _100btcPattern,
    pub _100sats: _100btcPattern,
    pub _10btc: _100btcPattern,
    pub _10k_btc: _100btcPattern,
    pub _10k_sats: _100btcPattern,
    pub _10m_sats: _100btcPattern,
    pub _10sats: _100btcPattern,
    pub _1btc: _100btcPattern,
    pub _1k_btc: _100btcPattern,
    pub _1k_sats: _100btcPattern,
    pub _1m_sats: _100btcPattern,
}

impl CatalogTree_Distribution_UtxoCohorts_LtAmount {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _100btc: _100btcPattern::new(client.clone(), "utxos_under_100btc".to_string()),
            _100k_btc: _100btcPattern::new(client.clone(), "utxos_under_100k_btc".to_string()),
            _100k_sats: _100btcPattern::new(client.clone(), "utxos_under_100k_sats".to_string()),
            _100sats: _100btcPattern::new(client.clone(), "utxos_under_100sats".to_string()),
            _10btc: _100btcPattern::new(client.clone(), "utxos_under_10btc".to_string()),
            _10k_btc: _100btcPattern::new(client.clone(), "utxos_under_10k_btc".to_string()),
            _10k_sats: _100btcPattern::new(client.clone(), "utxos_under_10k_sats".to_string()),
            _10m_sats: _100btcPattern::new(client.clone(), "utxos_under_10m_sats".to_string()),
            _10sats: _100btcPattern::new(client.clone(), "utxos_under_10sats".to_string()),
            _1btc: _100btcPattern::new(client.clone(), "utxos_under_1btc".to_string()),
            _1k_btc: _100btcPattern::new(client.clone(), "utxos_under_1k_btc".to_string()),
            _1k_sats: _100btcPattern::new(client.clone(), "utxos_under_1k_sats".to_string()),
            _1m_sats: _100btcPattern::new(client.clone(), "utxos_under_1m_sats".to_string()),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Distribution_UtxoCohorts_MaxAge {
    pub _10y: _10yPattern,
    pub _12y: _10yPattern,
    pub _15y: _10yPattern,
    pub _1m: _10yPattern,
    pub _1w: _10yPattern,
    pub _1y: _10yPattern,
    pub _2m: _10yPattern,
    pub _2y: _10yPattern,
    pub _3m: _10yPattern,
    pub _3y: _10yPattern,
    pub _4m: _10yPattern,
    pub _4y: _10yPattern,
    pub _5m: _10yPattern,
    pub _5y: _10yPattern,
    pub _6m: _10yPattern,
    pub _6y: _10yPattern,
    pub _7y: _10yPattern,
    pub _8y: _10yPattern,
}

impl CatalogTree_Distribution_UtxoCohorts_MaxAge {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _10y: _10yPattern::new(client.clone(), "utxos_up_to_10y_old".to_string()),
            _12y: _10yPattern::new(client.clone(), "utxos_up_to_12y_old".to_string()),
            _15y: _10yPattern::new(client.clone(), "utxos_up_to_15y_old".to_string()),
            _1m: _10yPattern::new(client.clone(), "utxos_up_to_1m_old".to_string()),
            _1w: _10yPattern::new(client.clone(), "utxos_up_to_1w_old".to_string()),
            _1y: _10yPattern::new(client.clone(), "utxos_up_to_1y_old".to_string()),
            _2m: _10yPattern::new(client.clone(), "utxos_up_to_2m_old".to_string()),
            _2y: _10yPattern::new(client.clone(), "utxos_up_to_2y_old".to_string()),
            _3m: _10yPattern::new(client.clone(), "utxos_up_to_3m_old".to_string()),
            _3y: _10yPattern::new(client.clone(), "utxos_up_to_3y_old".to_string()),
            _4m: _10yPattern::new(client.clone(), "utxos_up_to_4m_old".to_string()),
            _4y: _10yPattern::new(client.clone(), "utxos_up_to_4y_old".to_string()),
            _5m: _10yPattern::new(client.clone(), "utxos_up_to_5m_old".to_string()),
            _5y: _10yPattern::new(client.clone(), "utxos_up_to_5y_old".to_string()),
            _6m: _10yPattern::new(client.clone(), "utxos_up_to_6m_old".to_string()),
            _6y: _10yPattern::new(client.clone(), "utxos_up_to_6y_old".to_string()),
            _7y: _10yPattern::new(client.clone(), "utxos_up_to_7y_old".to_string()),
            _8y: _10yPattern::new(client.clone(), "utxos_up_to_8y_old".to_string()),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Distribution_UtxoCohorts_MinAge {
    pub _10y: _100btcPattern,
    pub _12y: _100btcPattern,
    pub _1d: _100btcPattern,
    pub _1m: _100btcPattern,
    pub _1w: _100btcPattern,
    pub _1y: _100btcPattern,
    pub _2m: _100btcPattern,
    pub _2y: _100btcPattern,
    pub _3m: _100btcPattern,
    pub _3y: _100btcPattern,
    pub _4m: _100btcPattern,
    pub _4y: _100btcPattern,
    pub _5m: _100btcPattern,
    pub _5y: _100btcPattern,
    pub _6m: _100btcPattern,
    pub _6y: _100btcPattern,
    pub _7y: _100btcPattern,
    pub _8y: _100btcPattern,
}

impl CatalogTree_Distribution_UtxoCohorts_MinAge {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _10y: _100btcPattern::new(client.clone(), "utxos_at_least_10y_old".to_string()),
            _12y: _100btcPattern::new(client.clone(), "utxos_at_least_12y_old".to_string()),
            _1d: _100btcPattern::new(client.clone(), "utxos_at_least_1d_old".to_string()),
            _1m: _100btcPattern::new(client.clone(), "utxos_at_least_1m_old".to_string()),
            _1w: _100btcPattern::new(client.clone(), "utxos_at_least_1w_old".to_string()),
            _1y: _100btcPattern::new(client.clone(), "utxos_at_least_1y_old".to_string()),
            _2m: _100btcPattern::new(client.clone(), "utxos_at_least_2m_old".to_string()),
            _2y: _100btcPattern::new(client.clone(), "utxos_at_least_2y_old".to_string()),
            _3m: _100btcPattern::new(client.clone(), "utxos_at_least_3m_old".to_string()),
            _3y: _100btcPattern::new(client.clone(), "utxos_at_least_3y_old".to_string()),
            _4m: _100btcPattern::new(client.clone(), "utxos_at_least_4m_old".to_string()),
            _4y: _100btcPattern::new(client.clone(), "utxos_at_least_4y_old".to_string()),
            _5m: _100btcPattern::new(client.clone(), "utxos_at_least_5m_old".to_string()),
            _5y: _100btcPattern::new(client.clone(), "utxos_at_least_5y_old".to_string()),
            _6m: _100btcPattern::new(client.clone(), "utxos_at_least_6m_old".to_string()),
            _6y: _100btcPattern::new(client.clone(), "utxos_at_least_6y_old".to_string()),
            _7y: _100btcPattern::new(client.clone(), "utxos_at_least_7y_old".to_string()),
            _8y: _100btcPattern::new(client.clone(), "utxos_at_least_8y_old".to_string()),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Distribution_UtxoCohorts_Term {
    pub long: CatalogTree_Distribution_UtxoCohorts_Term_Long,
    pub short: CatalogTree_Distribution_UtxoCohorts_Term_Short,
}

impl CatalogTree_Distribution_UtxoCohorts_Term {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            long: CatalogTree_Distribution_UtxoCohorts_Term_Long::new(client.clone(), format!("{base_path}_long")),
            short: CatalogTree_Distribution_UtxoCohorts_Term_Short::new(client.clone(), format!("{base_path}_short")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Distribution_UtxoCohorts_Term_Long {
    pub activity: ActivityPattern2,
    pub cost_basis: CostBasisPattern2,
    pub outputs: OutputsPattern,
    pub realized: RealizedPattern2,
    pub relative: RelativePattern5,
    pub supply: SupplyPattern2,
    pub unrealized: UnrealizedPattern,
}

impl CatalogTree_Distribution_UtxoCohorts_Term_Long {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            activity: ActivityPattern2::new(client.clone(), "lth".to_string()),
            cost_basis: CostBasisPattern2::new(client.clone(), "lth".to_string()),
            outputs: OutputsPattern::new(client.clone(), "lth_utxo_count".to_string()),
            realized: RealizedPattern2::new(client.clone(), "lth".to_string()),
            relative: RelativePattern5::new(client.clone(), "lth".to_string()),
            supply: SupplyPattern2::new(client.clone(), "lth_supply".to_string()),
            unrealized: UnrealizedPattern::new(client.clone(), "lth".to_string()),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Distribution_UtxoCohorts_Term_Short {
    pub activity: ActivityPattern2,
    pub cost_basis: CostBasisPattern2,
    pub outputs: OutputsPattern,
    pub realized: RealizedPattern3,
    pub relative: RelativePattern5,
    pub supply: SupplyPattern2,
    pub unrealized: UnrealizedPattern,
}

impl CatalogTree_Distribution_UtxoCohorts_Term_Short {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            activity: ActivityPattern2::new(client.clone(), "sth".to_string()),
            cost_basis: CostBasisPattern2::new(client.clone(), "sth".to_string()),
            outputs: OutputsPattern::new(client.clone(), "sth_utxo_count".to_string()),
            realized: RealizedPattern3::new(client.clone(), "sth".to_string()),
            relative: RelativePattern5::new(client.clone(), "sth".to_string()),
            supply: SupplyPattern2::new(client.clone(), "sth_supply".to_string()),
            unrealized: UnrealizedPattern::new(client.clone(), "sth".to_string()),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Distribution_UtxoCohorts_Type {
    pub empty: _0satsPattern2,
    pub p2a: _0satsPattern2,
    pub p2ms: _0satsPattern2,
    pub p2pk33: _0satsPattern2,
    pub p2pk65: _0satsPattern2,
    pub p2pkh: _0satsPattern2,
    pub p2sh: _0satsPattern2,
    pub p2tr: _0satsPattern2,
    pub p2wpkh: _0satsPattern2,
    pub p2wsh: _0satsPattern2,
    pub unknown: _0satsPattern2,
}

impl CatalogTree_Distribution_UtxoCohorts_Type {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            empty: _0satsPattern2::new(client.clone(), "empty_outputs".to_string()),
            p2a: _0satsPattern2::new(client.clone(), "p2a".to_string()),
            p2ms: _0satsPattern2::new(client.clone(), "p2ms".to_string()),
            p2pk33: _0satsPattern2::new(client.clone(), "p2pk33".to_string()),
            p2pk65: _0satsPattern2::new(client.clone(), "p2pk65".to_string()),
            p2pkh: _0satsPattern2::new(client.clone(), "p2pkh".to_string()),
            p2sh: _0satsPattern2::new(client.clone(), "p2sh".to_string()),
            p2tr: _0satsPattern2::new(client.clone(), "p2tr".to_string()),
            p2wpkh: _0satsPattern2::new(client.clone(), "p2wpkh".to_string()),
            p2wsh: _0satsPattern2::new(client.clone(), "p2wsh".to_string()),
            unknown: _0satsPattern2::new(client.clone(), "unknown_outputs".to_string()),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Distribution_UtxoCohorts_Year {
    pub _2009: _0satsPattern2,
    pub _2010: _0satsPattern2,
    pub _2011: _0satsPattern2,
    pub _2012: _0satsPattern2,
    pub _2013: _0satsPattern2,
    pub _2014: _0satsPattern2,
    pub _2015: _0satsPattern2,
    pub _2016: _0satsPattern2,
    pub _2017: _0satsPattern2,
    pub _2018: _0satsPattern2,
    pub _2019: _0satsPattern2,
    pub _2020: _0satsPattern2,
    pub _2021: _0satsPattern2,
    pub _2022: _0satsPattern2,
    pub _2023: _0satsPattern2,
    pub _2024: _0satsPattern2,
    pub _2025: _0satsPattern2,
    pub _2026: _0satsPattern2,
}

impl CatalogTree_Distribution_UtxoCohorts_Year {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _2009: _0satsPattern2::new(client.clone(), "year_2009".to_string()),
            _2010: _0satsPattern2::new(client.clone(), "year_2010".to_string()),
            _2011: _0satsPattern2::new(client.clone(), "year_2011".to_string()),
            _2012: _0satsPattern2::new(client.clone(), "year_2012".to_string()),
            _2013: _0satsPattern2::new(client.clone(), "year_2013".to_string()),
            _2014: _0satsPattern2::new(client.clone(), "year_2014".to_string()),
            _2015: _0satsPattern2::new(client.clone(), "year_2015".to_string()),
            _2016: _0satsPattern2::new(client.clone(), "year_2016".to_string()),
            _2017: _0satsPattern2::new(client.clone(), "year_2017".to_string()),
            _2018: _0satsPattern2::new(client.clone(), "year_2018".to_string()),
            _2019: _0satsPattern2::new(client.clone(), "year_2019".to_string()),
            _2020: _0satsPattern2::new(client.clone(), "year_2020".to_string()),
            _2021: _0satsPattern2::new(client.clone(), "year_2021".to_string()),
            _2022: _0satsPattern2::new(client.clone(), "year_2022".to_string()),
            _2023: _0satsPattern2::new(client.clone(), "year_2023".to_string()),
            _2024: _0satsPattern2::new(client.clone(), "year_2024".to_string()),
            _2025: _0satsPattern2::new(client.clone(), "year_2025".to_string()),
            _2026: _0satsPattern2::new(client.clone(), "year_2026".to_string()),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Indexes {
    pub address: CatalogTree_Indexes_Address,
    pub dateindex: CatalogTree_Indexes_Dateindex,
    pub decadeindex: CatalogTree_Indexes_Decadeindex,
    pub difficultyepoch: CatalogTree_Indexes_Difficultyepoch,
    pub halvingepoch: CatalogTree_Indexes_Halvingepoch,
    pub height: CatalogTree_Indexes_Height,
    pub monthindex: CatalogTree_Indexes_Monthindex,
    pub quarterindex: CatalogTree_Indexes_Quarterindex,
    pub semesterindex: CatalogTree_Indexes_Semesterindex,
    pub txindex: CatalogTree_Indexes_Txindex,
    pub txinindex: EmptyPattern<TxInIndex>,
    pub txoutindex: EmptyPattern<TxOutIndex>,
    pub weekindex: CatalogTree_Indexes_Weekindex,
    pub yearindex: CatalogTree_Indexes_Yearindex,
}

impl CatalogTree_Indexes {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            address: CatalogTree_Indexes_Address::new(client.clone(), format!("{base_path}_address")),
            dateindex: CatalogTree_Indexes_Dateindex::new(client.clone(), format!("{base_path}_dateindex")),
            decadeindex: CatalogTree_Indexes_Decadeindex::new(client.clone(), format!("{base_path}_decadeindex")),
            difficultyepoch: CatalogTree_Indexes_Difficultyepoch::new(client.clone(), format!("{base_path}_difficultyepoch")),
            halvingepoch: CatalogTree_Indexes_Halvingepoch::new(client.clone(), format!("{base_path}_halvingepoch")),
            height: CatalogTree_Indexes_Height::new(client.clone(), format!("{base_path}_height")),
            monthindex: CatalogTree_Indexes_Monthindex::new(client.clone(), format!("{base_path}_monthindex")),
            quarterindex: CatalogTree_Indexes_Quarterindex::new(client.clone(), format!("{base_path}_quarterindex")),
            semesterindex: CatalogTree_Indexes_Semesterindex::new(client.clone(), format!("{base_path}_semesterindex")),
            txindex: CatalogTree_Indexes_Txindex::new(client.clone(), format!("{base_path}_txindex")),
            txinindex: EmptyPattern::new(client.clone(), "txinindex".to_string()),
            txoutindex: EmptyPattern::new(client.clone(), "txoutindex".to_string()),
            weekindex: CatalogTree_Indexes_Weekindex::new(client.clone(), format!("{base_path}_weekindex")),
            yearindex: CatalogTree_Indexes_Yearindex::new(client.clone(), format!("{base_path}_yearindex")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Indexes_Address {
    pub empty: EmptyPattern<EmptyOutputIndex>,
    pub opreturn: EmptyPattern<OpReturnIndex>,
    pub p2a: EmptyPattern<P2AAddressIndex>,
    pub p2ms: EmptyPattern<P2MSOutputIndex>,
    pub p2pk33: EmptyPattern<P2PK33AddressIndex>,
    pub p2pk65: EmptyPattern<P2PK65AddressIndex>,
    pub p2pkh: EmptyPattern<P2PKHAddressIndex>,
    pub p2sh: EmptyPattern<P2SHAddressIndex>,
    pub p2tr: EmptyPattern<P2TRAddressIndex>,
    pub p2wpkh: EmptyPattern<P2WPKHAddressIndex>,
    pub p2wsh: EmptyPattern<P2WSHAddressIndex>,
    pub unknown: EmptyPattern<UnknownOutputIndex>,
}

impl CatalogTree_Indexes_Address {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            empty: EmptyPattern::new(client.clone(), "emptyoutputindex".to_string()),
            opreturn: EmptyPattern::new(client.clone(), "opreturnindex".to_string()),
            p2a: EmptyPattern::new(client.clone(), "p2aaddressindex".to_string()),
            p2ms: EmptyPattern::new(client.clone(), "p2msoutputindex".to_string()),
            p2pk33: EmptyPattern::new(client.clone(), "p2pk33addressindex".to_string()),
            p2pk65: EmptyPattern::new(client.clone(), "p2pk65addressindex".to_string()),
            p2pkh: EmptyPattern::new(client.clone(), "p2pkhaddressindex".to_string()),
            p2sh: EmptyPattern::new(client.clone(), "p2shaddressindex".to_string()),
            p2tr: EmptyPattern::new(client.clone(), "p2traddressindex".to_string()),
            p2wpkh: EmptyPattern::new(client.clone(), "p2wpkhaddressindex".to_string()),
            p2wsh: EmptyPattern::new(client.clone(), "p2wshaddressindex".to_string()),
            unknown: EmptyPattern::new(client.clone(), "unknownoutputindex".to_string()),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Indexes_Dateindex {
    pub date: MetricPattern7<Date>,
    pub first_height: MetricPattern7<Height>,
    pub height_count: MetricPattern7<StoredU64>,
    pub identity: MetricPattern7<DateIndex>,
    pub monthindex: MetricPattern7<MonthIndex>,
    pub weekindex: MetricPattern7<WeekIndex>,
}

impl CatalogTree_Indexes_Dateindex {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            date: MetricPattern7::new(client.clone(), format!("{base_path}_date")),
            first_height: MetricPattern7::new(client.clone(), format!("{base_path}_first_height")),
            height_count: MetricPattern7::new(client.clone(), format!("{base_path}_height_count")),
            identity: MetricPattern7::new(client.clone(), format!("{base_path}_identity")),
            monthindex: MetricPattern7::new(client.clone(), format!("{base_path}_monthindex")),
            weekindex: MetricPattern7::new(client.clone(), format!("{base_path}_weekindex")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Indexes_Decadeindex {
    pub first_yearindex: MetricPattern8<YearIndex>,
    pub identity: MetricPattern8<DecadeIndex>,
    pub yearindex_count: MetricPattern8<StoredU64>,
}

impl CatalogTree_Indexes_Decadeindex {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            first_yearindex: MetricPattern8::new(client.clone(), format!("{base_path}_first_yearindex")),
            identity: MetricPattern8::new(client.clone(), format!("{base_path}_identity")),
            yearindex_count: MetricPattern8::new(client.clone(), format!("{base_path}_yearindex_count")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Indexes_Difficultyepoch {
    pub first_height: MetricPattern9<Height>,
    pub height_count: MetricPattern9<StoredU64>,
    pub identity: MetricPattern9<DifficultyEpoch>,
}

impl CatalogTree_Indexes_Difficultyepoch {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            first_height: MetricPattern9::new(client.clone(), format!("{base_path}_first_height")),
            height_count: MetricPattern9::new(client.clone(), format!("{base_path}_height_count")),
            identity: MetricPattern9::new(client.clone(), format!("{base_path}_identity")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Indexes_Halvingepoch {
    pub first_height: MetricPattern11<Height>,
    pub identity: MetricPattern11<HalvingEpoch>,
}

impl CatalogTree_Indexes_Halvingepoch {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            first_height: MetricPattern11::new(client.clone(), format!("{base_path}_first_height")),
            identity: MetricPattern11::new(client.clone(), format!("{base_path}_identity")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Indexes_Height {
    pub dateindex: MetricPattern12<DateIndex>,
    pub difficultyepoch: MetricPattern12<DifficultyEpoch>,
    pub halvingepoch: MetricPattern12<HalvingEpoch>,
    pub identity: MetricPattern12<Height>,
    pub txindex_count: MetricPattern12<StoredU64>,
}

impl CatalogTree_Indexes_Height {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            dateindex: MetricPattern12::new(client.clone(), format!("{base_path}_dateindex")),
            difficultyepoch: MetricPattern12::new(client.clone(), format!("{base_path}_difficultyepoch")),
            halvingepoch: MetricPattern12::new(client.clone(), format!("{base_path}_halvingepoch")),
            identity: MetricPattern12::new(client.clone(), format!("{base_path}_identity")),
            txindex_count: MetricPattern12::new(client.clone(), format!("{base_path}_txindex_count")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Indexes_Monthindex {
    pub dateindex_count: MetricPattern14<StoredU64>,
    pub first_dateindex: MetricPattern14<DateIndex>,
    pub identity: MetricPattern14<MonthIndex>,
    pub quarterindex: MetricPattern14<QuarterIndex>,
    pub semesterindex: MetricPattern14<SemesterIndex>,
    pub yearindex: MetricPattern14<YearIndex>,
}

impl CatalogTree_Indexes_Monthindex {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            dateindex_count: MetricPattern14::new(client.clone(), format!("{base_path}_dateindex_count")),
            first_dateindex: MetricPattern14::new(client.clone(), format!("{base_path}_first_dateindex")),
            identity: MetricPattern14::new(client.clone(), format!("{base_path}_identity")),
            quarterindex: MetricPattern14::new(client.clone(), format!("{base_path}_quarterindex")),
            semesterindex: MetricPattern14::new(client.clone(), format!("{base_path}_semesterindex")),
            yearindex: MetricPattern14::new(client.clone(), format!("{base_path}_yearindex")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Indexes_Quarterindex {
    pub first_monthindex: MetricPattern26<MonthIndex>,
    pub identity: MetricPattern26<QuarterIndex>,
    pub monthindex_count: MetricPattern26<StoredU64>,
}

impl CatalogTree_Indexes_Quarterindex {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            first_monthindex: MetricPattern26::new(client.clone(), format!("{base_path}_first_monthindex")),
            identity: MetricPattern26::new(client.clone(), format!("{base_path}_identity")),
            monthindex_count: MetricPattern26::new(client.clone(), format!("{base_path}_monthindex_count")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Indexes_Semesterindex {
    pub first_monthindex: MetricPattern27<MonthIndex>,
    pub identity: MetricPattern27<SemesterIndex>,
    pub monthindex_count: MetricPattern27<StoredU64>,
}

impl CatalogTree_Indexes_Semesterindex {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            first_monthindex: MetricPattern27::new(client.clone(), format!("{base_path}_first_monthindex")),
            identity: MetricPattern27::new(client.clone(), format!("{base_path}_identity")),
            monthindex_count: MetricPattern27::new(client.clone(), format!("{base_path}_monthindex_count")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Indexes_Txindex {
    pub identity: MetricPattern28<TxIndex>,
    pub input_count: MetricPattern28<StoredU64>,
    pub output_count: MetricPattern28<StoredU64>,
}

impl CatalogTree_Indexes_Txindex {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern28::new(client.clone(), format!("{base_path}_identity")),
            input_count: MetricPattern28::new(client.clone(), format!("{base_path}_input_count")),
            output_count: MetricPattern28::new(client.clone(), format!("{base_path}_output_count")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Indexes_Weekindex {
    pub dateindex_count: MetricPattern30<StoredU64>,
    pub first_dateindex: MetricPattern30<DateIndex>,
    pub identity: MetricPattern30<WeekIndex>,
}

impl CatalogTree_Indexes_Weekindex {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            dateindex_count: MetricPattern30::new(client.clone(), format!("{base_path}_dateindex_count")),
            first_dateindex: MetricPattern30::new(client.clone(), format!("{base_path}_first_dateindex")),
            identity: MetricPattern30::new(client.clone(), format!("{base_path}_identity")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Indexes_Yearindex {
    pub decadeindex: MetricPattern31<DecadeIndex>,
    pub first_monthindex: MetricPattern31<MonthIndex>,
    pub identity: MetricPattern31<YearIndex>,
    pub monthindex_count: MetricPattern31<StoredU64>,
}

impl CatalogTree_Indexes_Yearindex {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            decadeindex: MetricPattern31::new(client.clone(), format!("{base_path}_decadeindex")),
            first_monthindex: MetricPattern31::new(client.clone(), format!("{base_path}_first_monthindex")),
            identity: MetricPattern31::new(client.clone(), format!("{base_path}_identity")),
            monthindex_count: MetricPattern31::new(client.clone(), format!("{base_path}_monthindex_count")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Inputs {
    pub count: SizePattern<StoredU64>,
    pub first_txinindex: MetricPattern12<TxInIndex>,
    pub outpoint: MetricPattern13<OutPoint>,
    pub outputtype: MetricPattern13<OutputType>,
    pub spent: CatalogTree_Inputs_Spent,
    pub txindex: MetricPattern13<TxIndex>,
    pub typeindex: MetricPattern13<TypeIndex>,
    pub witness_size: MetricPattern13<StoredU32>,
}

impl CatalogTree_Inputs {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            count: SizePattern::new(client.clone(), "input_count".to_string()),
            first_txinindex: MetricPattern12::new(client.clone(), format!("{base_path}_first_txinindex")),
            outpoint: MetricPattern13::new(client.clone(), format!("{base_path}_outpoint")),
            outputtype: MetricPattern13::new(client.clone(), format!("{base_path}_outputtype")),
            spent: CatalogTree_Inputs_Spent::new(client.clone(), format!("{base_path}_spent")),
            txindex: MetricPattern13::new(client.clone(), format!("{base_path}_txindex")),
            typeindex: MetricPattern13::new(client.clone(), format!("{base_path}_typeindex")),
            witness_size: MetricPattern13::new(client.clone(), format!("{base_path}_witness_size")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Inputs_Spent {
    pub txoutindex: MetricPattern13<TxOutIndex>,
    pub value: MetricPattern13<Sats>,
}

impl CatalogTree_Inputs_Spent {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            txoutindex: MetricPattern13::new(client.clone(), format!("{base_path}_txoutindex")),
            value: MetricPattern13::new(client.clone(), format!("{base_path}_value")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Market {
    pub ath: CatalogTree_Market_Ath,
    pub dca: CatalogTree_Market_Dca,
    pub indicators: CatalogTree_Market_Indicators,
    pub lookback: CatalogTree_Market_Lookback,
    pub moving_average: CatalogTree_Market_MovingAverage,
    pub range: CatalogTree_Market_Range,
    pub returns: CatalogTree_Market_Returns,
    pub volatility: CatalogTree_Market_Volatility,
}

impl CatalogTree_Market {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            ath: CatalogTree_Market_Ath::new(client.clone(), format!("{base_path}_ath")),
            dca: CatalogTree_Market_Dca::new(client.clone(), format!("{base_path}_dca")),
            indicators: CatalogTree_Market_Indicators::new(client.clone(), format!("{base_path}_indicators")),
            lookback: CatalogTree_Market_Lookback::new(client.clone(), format!("{base_path}_lookback")),
            moving_average: CatalogTree_Market_MovingAverage::new(client.clone(), format!("{base_path}_moving_average")),
            range: CatalogTree_Market_Range::new(client.clone(), format!("{base_path}_range")),
            returns: CatalogTree_Market_Returns::new(client.clone(), format!("{base_path}_returns")),
            volatility: CatalogTree_Market_Volatility::new(client.clone(), format!("{base_path}_volatility")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Market_Ath {
    pub days_since_price_ath: MetricPattern4<StoredU16>,
    pub max_days_between_price_aths: MetricPattern4<StoredU16>,
    pub max_years_between_price_aths: MetricPattern4<StoredF32>,
    pub price_ath: MetricPattern1<Dollars>,
    pub price_drawdown: MetricPattern3<StoredF32>,
    pub years_since_price_ath: MetricPattern4<StoredF32>,
}

impl CatalogTree_Market_Ath {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            days_since_price_ath: MetricPattern4::new(client.clone(), format!("{base_path}_days_since_price_ath")),
            max_days_between_price_aths: MetricPattern4::new(client.clone(), format!("{base_path}_max_days_between_price_aths")),
            max_years_between_price_aths: MetricPattern4::new(client.clone(), format!("{base_path}_max_years_between_price_aths")),
            price_ath: MetricPattern1::new(client.clone(), format!("{base_path}_price_ath")),
            price_drawdown: MetricPattern3::new(client.clone(), format!("{base_path}_price_drawdown")),
            years_since_price_ath: MetricPattern4::new(client.clone(), format!("{base_path}_years_since_price_ath")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Market_Dca {
    pub class_average_price: ClassAveragePricePattern<Dollars>,
    pub class_returns: ClassAveragePricePattern<StoredF32>,
    pub class_stack: CatalogTree_Market_Dca_ClassStack,
    pub period_average_price: PeriodAveragePricePattern<Dollars>,
    pub period_cagr: PeriodCagrPattern,
    pub period_lump_sum_stack: PeriodLumpSumStackPattern,
    pub period_returns: PeriodAveragePricePattern<StoredF32>,
    pub period_stack: PeriodLumpSumStackPattern,
}

impl CatalogTree_Market_Dca {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            class_average_price: ClassAveragePricePattern::new(client.clone(), "dca_class".to_string()),
            class_returns: ClassAveragePricePattern::new(client.clone(), "dca_class".to_string()),
            class_stack: CatalogTree_Market_Dca_ClassStack::new(client.clone(), format!("{base_path}_class_stack")),
            period_average_price: PeriodAveragePricePattern::new(client.clone(), "dca_average_price".to_string()),
            period_cagr: PeriodCagrPattern::new(client.clone(), "dca_cagr".to_string()),
            period_lump_sum_stack: PeriodLumpSumStackPattern::new(client.clone(), "".to_string()),
            period_returns: PeriodAveragePricePattern::new(client.clone(), "dca_returns".to_string()),
            period_stack: PeriodLumpSumStackPattern::new(client.clone(), "".to_string()),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Market_Dca_ClassStack {
    pub _2015: _24hCoinbaseSumPattern,
    pub _2016: _24hCoinbaseSumPattern,
    pub _2017: _24hCoinbaseSumPattern,
    pub _2018: _24hCoinbaseSumPattern,
    pub _2019: _24hCoinbaseSumPattern,
    pub _2020: _24hCoinbaseSumPattern,
    pub _2021: _24hCoinbaseSumPattern,
    pub _2022: _24hCoinbaseSumPattern,
    pub _2023: _24hCoinbaseSumPattern,
    pub _2024: _24hCoinbaseSumPattern,
    pub _2025: _24hCoinbaseSumPattern,
}

impl CatalogTree_Market_Dca_ClassStack {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _2015: _24hCoinbaseSumPattern::new(client.clone(), "dca_class_2015_stack".to_string()),
            _2016: _24hCoinbaseSumPattern::new(client.clone(), "dca_class_2016_stack".to_string()),
            _2017: _24hCoinbaseSumPattern::new(client.clone(), "dca_class_2017_stack".to_string()),
            _2018: _24hCoinbaseSumPattern::new(client.clone(), "dca_class_2018_stack".to_string()),
            _2019: _24hCoinbaseSumPattern::new(client.clone(), "dca_class_2019_stack".to_string()),
            _2020: _24hCoinbaseSumPattern::new(client.clone(), "dca_class_2020_stack".to_string()),
            _2021: _24hCoinbaseSumPattern::new(client.clone(), "dca_class_2021_stack".to_string()),
            _2022: _24hCoinbaseSumPattern::new(client.clone(), "dca_class_2022_stack".to_string()),
            _2023: _24hCoinbaseSumPattern::new(client.clone(), "dca_class_2023_stack".to_string()),
            _2024: _24hCoinbaseSumPattern::new(client.clone(), "dca_class_2024_stack".to_string()),
            _2025: _24hCoinbaseSumPattern::new(client.clone(), "dca_class_2025_stack".to_string()),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Market_Indicators {
    pub gini: MetricPattern7<StoredF32>,
    pub macd_histogram: MetricPattern7<StoredF32>,
    pub macd_line: MetricPattern7<StoredF32>,
    pub macd_signal: MetricPattern7<StoredF32>,
    pub nvt: MetricPattern4<StoredF32>,
    pub pi_cycle: MetricPattern7<StoredF32>,
    pub puell_multiple: MetricPattern4<StoredF32>,
    pub rsi_14d: MetricPattern7<StoredF32>,
    pub rsi_14d_max: MetricPattern7<StoredF32>,
    pub rsi_14d_min: MetricPattern7<StoredF32>,
    pub rsi_average_gain_14d: MetricPattern7<StoredF32>,
    pub rsi_average_loss_14d: MetricPattern7<StoredF32>,
    pub rsi_gains: MetricPattern7<StoredF32>,
    pub rsi_losses: MetricPattern7<StoredF32>,
    pub stoch_d: MetricPattern7<StoredF32>,
    pub stoch_k: MetricPattern7<StoredF32>,
    pub stoch_rsi: MetricPattern7<StoredF32>,
    pub stoch_rsi_d: MetricPattern7<StoredF32>,
    pub stoch_rsi_k: MetricPattern7<StoredF32>,
}

impl CatalogTree_Market_Indicators {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            gini: MetricPattern7::new(client.clone(), format!("{base_path}_gini")),
            macd_histogram: MetricPattern7::new(client.clone(), format!("{base_path}_macd_histogram")),
            macd_line: MetricPattern7::new(client.clone(), format!("{base_path}_macd_line")),
            macd_signal: MetricPattern7::new(client.clone(), format!("{base_path}_macd_signal")),
            nvt: MetricPattern4::new(client.clone(), format!("{base_path}_nvt")),
            pi_cycle: MetricPattern7::new(client.clone(), format!("{base_path}_pi_cycle")),
            puell_multiple: MetricPattern4::new(client.clone(), format!("{base_path}_puell_multiple")),
            rsi_14d: MetricPattern7::new(client.clone(), format!("{base_path}_rsi_14d")),
            rsi_14d_max: MetricPattern7::new(client.clone(), format!("{base_path}_rsi_14d_max")),
            rsi_14d_min: MetricPattern7::new(client.clone(), format!("{base_path}_rsi_14d_min")),
            rsi_average_gain_14d: MetricPattern7::new(client.clone(), format!("{base_path}_rsi_average_gain_14d")),
            rsi_average_loss_14d: MetricPattern7::new(client.clone(), format!("{base_path}_rsi_average_loss_14d")),
            rsi_gains: MetricPattern7::new(client.clone(), format!("{base_path}_rsi_gains")),
            rsi_losses: MetricPattern7::new(client.clone(), format!("{base_path}_rsi_losses")),
            stoch_d: MetricPattern7::new(client.clone(), format!("{base_path}_stoch_d")),
            stoch_k: MetricPattern7::new(client.clone(), format!("{base_path}_stoch_k")),
            stoch_rsi: MetricPattern7::new(client.clone(), format!("{base_path}_stoch_rsi")),
            stoch_rsi_d: MetricPattern7::new(client.clone(), format!("{base_path}_stoch_rsi_d")),
            stoch_rsi_k: MetricPattern7::new(client.clone(), format!("{base_path}_stoch_rsi_k")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Market_Lookback {
    pub price_ago: PriceAgoPattern<Dollars>,
}

impl CatalogTree_Market_Lookback {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            price_ago: PriceAgoPattern::new(client.clone(), "price".to_string()),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Market_MovingAverage {
    pub price_111d_sma: Price111dSmaPattern,
    pub price_12d_ema: Price111dSmaPattern,
    pub price_13d_ema: Price111dSmaPattern,
    pub price_13d_sma: Price111dSmaPattern,
    pub price_144d_ema: Price111dSmaPattern,
    pub price_144d_sma: Price111dSmaPattern,
    pub price_1m_ema: Price111dSmaPattern,
    pub price_1m_sma: Price111dSmaPattern,
    pub price_1w_ema: Price111dSmaPattern,
    pub price_1w_sma: Price111dSmaPattern,
    pub price_1y_ema: Price111dSmaPattern,
    pub price_1y_sma: Price111dSmaPattern,
    pub price_200d_ema: Price111dSmaPattern,
    pub price_200d_sma: Price111dSmaPattern,
    pub price_200d_sma_x0_8: MetricPattern4<Dollars>,
    pub price_200d_sma_x2_4: MetricPattern4<Dollars>,
    pub price_200w_ema: Price111dSmaPattern,
    pub price_200w_sma: Price111dSmaPattern,
    pub price_21d_ema: Price111dSmaPattern,
    pub price_21d_sma: Price111dSmaPattern,
    pub price_26d_ema: Price111dSmaPattern,
    pub price_2y_ema: Price111dSmaPattern,
    pub price_2y_sma: Price111dSmaPattern,
    pub price_34d_ema: Price111dSmaPattern,
    pub price_34d_sma: Price111dSmaPattern,
    pub price_350d_sma: Price111dSmaPattern,
    pub price_350d_sma_x2: MetricPattern4<Dollars>,
    pub price_4y_ema: Price111dSmaPattern,
    pub price_4y_sma: Price111dSmaPattern,
    pub price_55d_ema: Price111dSmaPattern,
    pub price_55d_sma: Price111dSmaPattern,
    pub price_89d_ema: Price111dSmaPattern,
    pub price_89d_sma: Price111dSmaPattern,
    pub price_8d_ema: Price111dSmaPattern,
    pub price_8d_sma: Price111dSmaPattern,
}

impl CatalogTree_Market_MovingAverage {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            price_111d_sma: Price111dSmaPattern::new(client.clone(), "price_111d_sma".to_string()),
            price_12d_ema: Price111dSmaPattern::new(client.clone(), "price_12d_ema".to_string()),
            price_13d_ema: Price111dSmaPattern::new(client.clone(), "price_13d_ema".to_string()),
            price_13d_sma: Price111dSmaPattern::new(client.clone(), "price_13d_sma".to_string()),
            price_144d_ema: Price111dSmaPattern::new(client.clone(), "price_144d_ema".to_string()),
            price_144d_sma: Price111dSmaPattern::new(client.clone(), "price_144d_sma".to_string()),
            price_1m_ema: Price111dSmaPattern::new(client.clone(), "price_1m_ema".to_string()),
            price_1m_sma: Price111dSmaPattern::new(client.clone(), "price_1m_sma".to_string()),
            price_1w_ema: Price111dSmaPattern::new(client.clone(), "price_1w_ema".to_string()),
            price_1w_sma: Price111dSmaPattern::new(client.clone(), "price_1w_sma".to_string()),
            price_1y_ema: Price111dSmaPattern::new(client.clone(), "price_1y_ema".to_string()),
            price_1y_sma: Price111dSmaPattern::new(client.clone(), "price_1y_sma".to_string()),
            price_200d_ema: Price111dSmaPattern::new(client.clone(), "price_200d_ema".to_string()),
            price_200d_sma: Price111dSmaPattern::new(client.clone(), "price_200d_sma".to_string()),
            price_200d_sma_x0_8: MetricPattern4::new(client.clone(), format!("{base_path}_price_200d_sma_x0_8")),
            price_200d_sma_x2_4: MetricPattern4::new(client.clone(), format!("{base_path}_price_200d_sma_x2_4")),
            price_200w_ema: Price111dSmaPattern::new(client.clone(), "price_200w_ema".to_string()),
            price_200w_sma: Price111dSmaPattern::new(client.clone(), "price_200w_sma".to_string()),
            price_21d_ema: Price111dSmaPattern::new(client.clone(), "price_21d_ema".to_string()),
            price_21d_sma: Price111dSmaPattern::new(client.clone(), "price_21d_sma".to_string()),
            price_26d_ema: Price111dSmaPattern::new(client.clone(), "price_26d_ema".to_string()),
            price_2y_ema: Price111dSmaPattern::new(client.clone(), "price_2y_ema".to_string()),
            price_2y_sma: Price111dSmaPattern::new(client.clone(), "price_2y_sma".to_string()),
            price_34d_ema: Price111dSmaPattern::new(client.clone(), "price_34d_ema".to_string()),
            price_34d_sma: Price111dSmaPattern::new(client.clone(), "price_34d_sma".to_string()),
            price_350d_sma: Price111dSmaPattern::new(client.clone(), "price_350d_sma".to_string()),
            price_350d_sma_x2: MetricPattern4::new(client.clone(), format!("{base_path}_price_350d_sma_x2")),
            price_4y_ema: Price111dSmaPattern::new(client.clone(), "price_4y_ema".to_string()),
            price_4y_sma: Price111dSmaPattern::new(client.clone(), "price_4y_sma".to_string()),
            price_55d_ema: Price111dSmaPattern::new(client.clone(), "price_55d_ema".to_string()),
            price_55d_sma: Price111dSmaPattern::new(client.clone(), "price_55d_sma".to_string()),
            price_89d_ema: Price111dSmaPattern::new(client.clone(), "price_89d_ema".to_string()),
            price_89d_sma: Price111dSmaPattern::new(client.clone(), "price_89d_sma".to_string()),
            price_8d_ema: Price111dSmaPattern::new(client.clone(), "price_8d_ema".to_string()),
            price_8d_sma: Price111dSmaPattern::new(client.clone(), "price_8d_sma".to_string()),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Market_Range {
    pub price_1m_max: MetricPattern4<Dollars>,
    pub price_1m_min: MetricPattern4<Dollars>,
    pub price_1w_max: MetricPattern4<Dollars>,
    pub price_1w_min: MetricPattern4<Dollars>,
    pub price_1y_max: MetricPattern4<Dollars>,
    pub price_1y_min: MetricPattern4<Dollars>,
    pub price_2w_choppiness_index: MetricPattern4<StoredF32>,
    pub price_2w_max: MetricPattern4<Dollars>,
    pub price_2w_min: MetricPattern4<Dollars>,
    pub price_true_range: MetricPattern7<StoredF32>,
    pub price_true_range_2w_sum: MetricPattern7<StoredF32>,
}

impl CatalogTree_Market_Range {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            price_1m_max: MetricPattern4::new(client.clone(), format!("{base_path}_price_1m_max")),
            price_1m_min: MetricPattern4::new(client.clone(), format!("{base_path}_price_1m_min")),
            price_1w_max: MetricPattern4::new(client.clone(), format!("{base_path}_price_1w_max")),
            price_1w_min: MetricPattern4::new(client.clone(), format!("{base_path}_price_1w_min")),
            price_1y_max: MetricPattern4::new(client.clone(), format!("{base_path}_price_1y_max")),
            price_1y_min: MetricPattern4::new(client.clone(), format!("{base_path}_price_1y_min")),
            price_2w_choppiness_index: MetricPattern4::new(client.clone(), format!("{base_path}_price_2w_choppiness_index")),
            price_2w_max: MetricPattern4::new(client.clone(), format!("{base_path}_price_2w_max")),
            price_2w_min: MetricPattern4::new(client.clone(), format!("{base_path}_price_2w_min")),
            price_true_range: MetricPattern7::new(client.clone(), format!("{base_path}_price_true_range")),
            price_true_range_2w_sum: MetricPattern7::new(client.clone(), format!("{base_path}_price_true_range_2w_sum")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Market_Returns {
    pub _1d_returns_1m_sd: _1dReturns1mSdPattern,
    pub _1d_returns_1w_sd: _1dReturns1mSdPattern,
    pub _1d_returns_1y_sd: _1dReturns1mSdPattern,
    pub cagr: PeriodCagrPattern,
    pub downside_1m_sd: _1dReturns1mSdPattern,
    pub downside_1w_sd: _1dReturns1mSdPattern,
    pub downside_1y_sd: _1dReturns1mSdPattern,
    pub downside_returns: MetricPattern7<StoredF32>,
    pub price_returns: PriceAgoPattern<StoredF32>,
}

impl CatalogTree_Market_Returns {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _1d_returns_1m_sd: _1dReturns1mSdPattern::new(client.clone(), "1d_returns_1m_sd".to_string()),
            _1d_returns_1w_sd: _1dReturns1mSdPattern::new(client.clone(), "1d_returns_1w_sd".to_string()),
            _1d_returns_1y_sd: _1dReturns1mSdPattern::new(client.clone(), "1d_returns_1y_sd".to_string()),
            cagr: PeriodCagrPattern::new(client.clone(), "cagr".to_string()),
            downside_1m_sd: _1dReturns1mSdPattern::new(client.clone(), "downside_1m_sd".to_string()),
            downside_1w_sd: _1dReturns1mSdPattern::new(client.clone(), "downside_1w_sd".to_string()),
            downside_1y_sd: _1dReturns1mSdPattern::new(client.clone(), "downside_1y_sd".to_string()),
            downside_returns: MetricPattern7::new(client.clone(), format!("{base_path}_downside_returns")),
            price_returns: PriceAgoPattern::new(client.clone(), "price_returns".to_string()),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Market_Volatility {
    pub price_1m_volatility: MetricPattern4<StoredF32>,
    pub price_1w_volatility: MetricPattern4<StoredF32>,
    pub price_1y_volatility: MetricPattern4<StoredF32>,
    pub sharpe_1m: MetricPattern7<StoredF32>,
    pub sharpe_1w: MetricPattern7<StoredF32>,
    pub sharpe_1y: MetricPattern7<StoredF32>,
    pub sortino_1m: MetricPattern7<StoredF32>,
    pub sortino_1w: MetricPattern7<StoredF32>,
    pub sortino_1y: MetricPattern7<StoredF32>,
}

impl CatalogTree_Market_Volatility {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            price_1m_volatility: MetricPattern4::new(client.clone(), format!("{base_path}_price_1m_volatility")),
            price_1w_volatility: MetricPattern4::new(client.clone(), format!("{base_path}_price_1w_volatility")),
            price_1y_volatility: MetricPattern4::new(client.clone(), format!("{base_path}_price_1y_volatility")),
            sharpe_1m: MetricPattern7::new(client.clone(), format!("{base_path}_sharpe_1m")),
            sharpe_1w: MetricPattern7::new(client.clone(), format!("{base_path}_sharpe_1w")),
            sharpe_1y: MetricPattern7::new(client.clone(), format!("{base_path}_sharpe_1y")),
            sortino_1m: MetricPattern7::new(client.clone(), format!("{base_path}_sortino_1m")),
            sortino_1w: MetricPattern7::new(client.clone(), format!("{base_path}_sortino_1w")),
            sortino_1y: MetricPattern7::new(client.clone(), format!("{base_path}_sortino_1y")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Outputs {
    pub count: CatalogTree_Outputs_Count,
    pub first_txoutindex: MetricPattern12<TxOutIndex>,
    pub outputtype: MetricPattern16<OutputType>,
    pub spent: CatalogTree_Outputs_Spent,
    pub txindex: MetricPattern16<TxIndex>,
    pub typeindex: MetricPattern16<TypeIndex>,
    pub value: MetricPattern16<Sats>,
}

impl CatalogTree_Outputs {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            count: CatalogTree_Outputs_Count::new(client.clone(), format!("{base_path}_count")),
            first_txoutindex: MetricPattern12::new(client.clone(), format!("{base_path}_first_txoutindex")),
            outputtype: MetricPattern16::new(client.clone(), format!("{base_path}_outputtype")),
            spent: CatalogTree_Outputs_Spent::new(client.clone(), format!("{base_path}_spent")),
            txindex: MetricPattern16::new(client.clone(), format!("{base_path}_txindex")),
            typeindex: MetricPattern16::new(client.clone(), format!("{base_path}_typeindex")),
            value: MetricPattern16::new(client.clone(), format!("{base_path}_value")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Outputs_Count {
    pub total_count: SizePattern<StoredU64>,
    pub utxo_count: BitcoinPattern<StoredU64>,
}

impl CatalogTree_Outputs_Count {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            total_count: SizePattern::new(client.clone(), "output_count".to_string()),
            utxo_count: BitcoinPattern::new(client.clone(), "exact_utxo_count".to_string()),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Outputs_Spent {
    pub txinindex: MetricPattern16<TxInIndex>,
}

impl CatalogTree_Outputs_Spent {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            txinindex: MetricPattern16::new(client.clone(), format!("{base_path}_txinindex")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Pools {
    pub pool: MetricPattern12<PoolSlug>,
    pub vecs: CatalogTree_Pools_Vecs,
}

impl CatalogTree_Pools {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            pool: MetricPattern12::new(client.clone(), format!("{base_path}_pool")),
            vecs: CatalogTree_Pools_Vecs::new(client.clone(), format!("{base_path}_vecs")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Pools_Vecs {
    pub aaopool: AaopoolPattern,
    pub antpool: AaopoolPattern,
    pub arkpool: AaopoolPattern,
    pub asicminer: AaopoolPattern,
    pub axbt: AaopoolPattern,
    pub batpool: AaopoolPattern,
    pub bcmonster: AaopoolPattern,
    pub bcpoolio: AaopoolPattern,
    pub binancepool: AaopoolPattern,
    pub bitalo: AaopoolPattern,
    pub bitclub: AaopoolPattern,
    pub bitcoinaffiliatenetwork: AaopoolPattern,
    pub bitcoincom: AaopoolPattern,
    pub bitcoinindia: AaopoolPattern,
    pub bitcoinrussia: AaopoolPattern,
    pub bitcoinukraine: AaopoolPattern,
    pub bitfarms: AaopoolPattern,
    pub bitfufupool: AaopoolPattern,
    pub bitfury: AaopoolPattern,
    pub bitminter: AaopoolPattern,
    pub bitparking: AaopoolPattern,
    pub bitsolo: AaopoolPattern,
    pub bixin: AaopoolPattern,
    pub blockfills: AaopoolPattern,
    pub braiinspool: AaopoolPattern,
    pub bravomining: AaopoolPattern,
    pub btcc: AaopoolPattern,
    pub btccom: AaopoolPattern,
    pub btcdig: AaopoolPattern,
    pub btcguild: AaopoolPattern,
    pub btclab: AaopoolPattern,
    pub btcmp: AaopoolPattern,
    pub btcnuggets: AaopoolPattern,
    pub btcpoolparty: AaopoolPattern,
    pub btcserv: AaopoolPattern,
    pub btctop: AaopoolPattern,
    pub btpool: AaopoolPattern,
    pub bwpool: AaopoolPattern,
    pub bytepool: AaopoolPattern,
    pub canoe: AaopoolPattern,
    pub canoepool: AaopoolPattern,
    pub carbonnegative: AaopoolPattern,
    pub ckpool: AaopoolPattern,
    pub cloudhashing: AaopoolPattern,
    pub coinlab: AaopoolPattern,
    pub cointerra: AaopoolPattern,
    pub connectbtc: AaopoolPattern,
    pub dcex: AaopoolPattern,
    pub dcexploration: AaopoolPattern,
    pub digitalbtc: AaopoolPattern,
    pub digitalxmintsy: AaopoolPattern,
    pub dpool: AaopoolPattern,
    pub eclipsemc: AaopoolPattern,
    pub eightbaochi: AaopoolPattern,
    pub ekanembtc: AaopoolPattern,
    pub eligius: AaopoolPattern,
    pub emcdpool: AaopoolPattern,
    pub entrustcharitypool: AaopoolPattern,
    pub eobot: AaopoolPattern,
    pub exxbw: AaopoolPattern,
    pub f2pool: AaopoolPattern,
    pub fiftyeightcoin: AaopoolPattern,
    pub foundryusa: AaopoolPattern,
    pub futurebitapollosolo: AaopoolPattern,
    pub gbminers: AaopoolPattern,
    pub ghashio: AaopoolPattern,
    pub givemecoins: AaopoolPattern,
    pub gogreenlight: AaopoolPattern,
    pub haominer: AaopoolPattern,
    pub haozhuzhu: AaopoolPattern,
    pub hashbx: AaopoolPattern,
    pub hashpool: AaopoolPattern,
    pub helix: AaopoolPattern,
    pub hhtt: AaopoolPattern,
    pub hotpool: AaopoolPattern,
    pub hummerpool: AaopoolPattern,
    pub huobipool: AaopoolPattern,
    pub innopolistech: AaopoolPattern,
    pub kanopool: AaopoolPattern,
    pub kncminer: AaopoolPattern,
    pub kucoinpool: AaopoolPattern,
    pub lubiancom: AaopoolPattern,
    pub luckypool: AaopoolPattern,
    pub luxor: AaopoolPattern,
    pub marapool: AaopoolPattern,
    pub maxbtc: AaopoolPattern,
    pub maxipool: AaopoolPattern,
    pub megabigpower: AaopoolPattern,
    pub minerium: AaopoolPattern,
    pub miningcity: AaopoolPattern,
    pub miningdutch: AaopoolPattern,
    pub miningkings: AaopoolPattern,
    pub miningsquared: AaopoolPattern,
    pub mmpool: AaopoolPattern,
    pub mtred: AaopoolPattern,
    pub multicoinco: AaopoolPattern,
    pub multipool: AaopoolPattern,
    pub mybtccoinpool: AaopoolPattern,
    pub neopool: AaopoolPattern,
    pub nexious: AaopoolPattern,
    pub nicehash: AaopoolPattern,
    pub nmcbit: AaopoolPattern,
    pub novablock: AaopoolPattern,
    pub ocean: AaopoolPattern,
    pub okexpool: AaopoolPattern,
    pub okkong: AaopoolPattern,
    pub okminer: AaopoolPattern,
    pub okpooltop: AaopoolPattern,
    pub onehash: AaopoolPattern,
    pub onem1x: AaopoolPattern,
    pub onethash: AaopoolPattern,
    pub ozcoin: AaopoolPattern,
    pub parasite: AaopoolPattern,
    pub patels: AaopoolPattern,
    pub pegapool: AaopoolPattern,
    pub phashio: AaopoolPattern,
    pub phoenix: AaopoolPattern,
    pub polmine: AaopoolPattern,
    pub pool175btc: AaopoolPattern,
    pub pool50btc: AaopoolPattern,
    pub poolin: AaopoolPattern,
    pub portlandhodl: AaopoolPattern,
    pub publicpool: AaopoolPattern,
    pub purebtccom: AaopoolPattern,
    pub rawpool: AaopoolPattern,
    pub rigpool: AaopoolPattern,
    pub sbicrypto: AaopoolPattern,
    pub secpool: AaopoolPattern,
    pub secretsuperstar: AaopoolPattern,
    pub sevenpool: AaopoolPattern,
    pub shawnp0wers: AaopoolPattern,
    pub sigmapoolcom: AaopoolPattern,
    pub simplecoinus: AaopoolPattern,
    pub solock: AaopoolPattern,
    pub spiderpool: AaopoolPattern,
    pub stminingcorp: AaopoolPattern,
    pub tangpool: AaopoolPattern,
    pub tatmaspool: AaopoolPattern,
    pub tbdice: AaopoolPattern,
    pub telco214: AaopoolPattern,
    pub terrapool: AaopoolPattern,
    pub tiger: AaopoolPattern,
    pub tigerpoolnet: AaopoolPattern,
    pub titan: AaopoolPattern,
    pub transactioncoinmining: AaopoolPattern,
    pub trickysbtcpool: AaopoolPattern,
    pub triplemining: AaopoolPattern,
    pub twentyoneinc: AaopoolPattern,
    pub ultimuspool: AaopoolPattern,
    pub unknown: AaopoolPattern,
    pub unomp: AaopoolPattern,
    pub viabtc: AaopoolPattern,
    pub waterhole: AaopoolPattern,
    pub wayicn: AaopoolPattern,
    pub whitepool: AaopoolPattern,
    pub wk057: AaopoolPattern,
    pub yourbtcnet: AaopoolPattern,
    pub zulupool: AaopoolPattern,
}

impl CatalogTree_Pools_Vecs {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            aaopool: AaopoolPattern::new(client.clone(), "aaopool".to_string()),
            antpool: AaopoolPattern::new(client.clone(), "antpool".to_string()),
            arkpool: AaopoolPattern::new(client.clone(), "arkpool".to_string()),
            asicminer: AaopoolPattern::new(client.clone(), "asicminer".to_string()),
            axbt: AaopoolPattern::new(client.clone(), "axbt".to_string()),
            batpool: AaopoolPattern::new(client.clone(), "batpool".to_string()),
            bcmonster: AaopoolPattern::new(client.clone(), "bcmonster".to_string()),
            bcpoolio: AaopoolPattern::new(client.clone(), "bcpoolio".to_string()),
            binancepool: AaopoolPattern::new(client.clone(), "binancepool".to_string()),
            bitalo: AaopoolPattern::new(client.clone(), "bitalo".to_string()),
            bitclub: AaopoolPattern::new(client.clone(), "bitclub".to_string()),
            bitcoinaffiliatenetwork: AaopoolPattern::new(client.clone(), "bitcoinaffiliatenetwork".to_string()),
            bitcoincom: AaopoolPattern::new(client.clone(), "bitcoincom".to_string()),
            bitcoinindia: AaopoolPattern::new(client.clone(), "bitcoinindia".to_string()),
            bitcoinrussia: AaopoolPattern::new(client.clone(), "bitcoinrussia".to_string()),
            bitcoinukraine: AaopoolPattern::new(client.clone(), "bitcoinukraine".to_string()),
            bitfarms: AaopoolPattern::new(client.clone(), "bitfarms".to_string()),
            bitfufupool: AaopoolPattern::new(client.clone(), "bitfufupool".to_string()),
            bitfury: AaopoolPattern::new(client.clone(), "bitfury".to_string()),
            bitminter: AaopoolPattern::new(client.clone(), "bitminter".to_string()),
            bitparking: AaopoolPattern::new(client.clone(), "bitparking".to_string()),
            bitsolo: AaopoolPattern::new(client.clone(), "bitsolo".to_string()),
            bixin: AaopoolPattern::new(client.clone(), "bixin".to_string()),
            blockfills: AaopoolPattern::new(client.clone(), "blockfills".to_string()),
            braiinspool: AaopoolPattern::new(client.clone(), "braiinspool".to_string()),
            bravomining: AaopoolPattern::new(client.clone(), "bravomining".to_string()),
            btcc: AaopoolPattern::new(client.clone(), "btcc".to_string()),
            btccom: AaopoolPattern::new(client.clone(), "btccom".to_string()),
            btcdig: AaopoolPattern::new(client.clone(), "btcdig".to_string()),
            btcguild: AaopoolPattern::new(client.clone(), "btcguild".to_string()),
            btclab: AaopoolPattern::new(client.clone(), "btclab".to_string()),
            btcmp: AaopoolPattern::new(client.clone(), "btcmp".to_string()),
            btcnuggets: AaopoolPattern::new(client.clone(), "btcnuggets".to_string()),
            btcpoolparty: AaopoolPattern::new(client.clone(), "btcpoolparty".to_string()),
            btcserv: AaopoolPattern::new(client.clone(), "btcserv".to_string()),
            btctop: AaopoolPattern::new(client.clone(), "btctop".to_string()),
            btpool: AaopoolPattern::new(client.clone(), "btpool".to_string()),
            bwpool: AaopoolPattern::new(client.clone(), "bwpool".to_string()),
            bytepool: AaopoolPattern::new(client.clone(), "bytepool".to_string()),
            canoe: AaopoolPattern::new(client.clone(), "canoe".to_string()),
            canoepool: AaopoolPattern::new(client.clone(), "canoepool".to_string()),
            carbonnegative: AaopoolPattern::new(client.clone(), "carbonnegative".to_string()),
            ckpool: AaopoolPattern::new(client.clone(), "ckpool".to_string()),
            cloudhashing: AaopoolPattern::new(client.clone(), "cloudhashing".to_string()),
            coinlab: AaopoolPattern::new(client.clone(), "coinlab".to_string()),
            cointerra: AaopoolPattern::new(client.clone(), "cointerra".to_string()),
            connectbtc: AaopoolPattern::new(client.clone(), "connectbtc".to_string()),
            dcex: AaopoolPattern::new(client.clone(), "dcex".to_string()),
            dcexploration: AaopoolPattern::new(client.clone(), "dcexploration".to_string()),
            digitalbtc: AaopoolPattern::new(client.clone(), "digitalbtc".to_string()),
            digitalxmintsy: AaopoolPattern::new(client.clone(), "digitalxmintsy".to_string()),
            dpool: AaopoolPattern::new(client.clone(), "dpool".to_string()),
            eclipsemc: AaopoolPattern::new(client.clone(), "eclipsemc".to_string()),
            eightbaochi: AaopoolPattern::new(client.clone(), "eightbaochi".to_string()),
            ekanembtc: AaopoolPattern::new(client.clone(), "ekanembtc".to_string()),
            eligius: AaopoolPattern::new(client.clone(), "eligius".to_string()),
            emcdpool: AaopoolPattern::new(client.clone(), "emcdpool".to_string()),
            entrustcharitypool: AaopoolPattern::new(client.clone(), "entrustcharitypool".to_string()),
            eobot: AaopoolPattern::new(client.clone(), "eobot".to_string()),
            exxbw: AaopoolPattern::new(client.clone(), "exxbw".to_string()),
            f2pool: AaopoolPattern::new(client.clone(), "f2pool".to_string()),
            fiftyeightcoin: AaopoolPattern::new(client.clone(), "fiftyeightcoin".to_string()),
            foundryusa: AaopoolPattern::new(client.clone(), "foundryusa".to_string()),
            futurebitapollosolo: AaopoolPattern::new(client.clone(), "futurebitapollosolo".to_string()),
            gbminers: AaopoolPattern::new(client.clone(), "gbminers".to_string()),
            ghashio: AaopoolPattern::new(client.clone(), "ghashio".to_string()),
            givemecoins: AaopoolPattern::new(client.clone(), "givemecoins".to_string()),
            gogreenlight: AaopoolPattern::new(client.clone(), "gogreenlight".to_string()),
            haominer: AaopoolPattern::new(client.clone(), "haominer".to_string()),
            haozhuzhu: AaopoolPattern::new(client.clone(), "haozhuzhu".to_string()),
            hashbx: AaopoolPattern::new(client.clone(), "hashbx".to_string()),
            hashpool: AaopoolPattern::new(client.clone(), "hashpool".to_string()),
            helix: AaopoolPattern::new(client.clone(), "helix".to_string()),
            hhtt: AaopoolPattern::new(client.clone(), "hhtt".to_string()),
            hotpool: AaopoolPattern::new(client.clone(), "hotpool".to_string()),
            hummerpool: AaopoolPattern::new(client.clone(), "hummerpool".to_string()),
            huobipool: AaopoolPattern::new(client.clone(), "huobipool".to_string()),
            innopolistech: AaopoolPattern::new(client.clone(), "innopolistech".to_string()),
            kanopool: AaopoolPattern::new(client.clone(), "kanopool".to_string()),
            kncminer: AaopoolPattern::new(client.clone(), "kncminer".to_string()),
            kucoinpool: AaopoolPattern::new(client.clone(), "kucoinpool".to_string()),
            lubiancom: AaopoolPattern::new(client.clone(), "lubiancom".to_string()),
            luckypool: AaopoolPattern::new(client.clone(), "luckypool".to_string()),
            luxor: AaopoolPattern::new(client.clone(), "luxor".to_string()),
            marapool: AaopoolPattern::new(client.clone(), "marapool".to_string()),
            maxbtc: AaopoolPattern::new(client.clone(), "maxbtc".to_string()),
            maxipool: AaopoolPattern::new(client.clone(), "maxipool".to_string()),
            megabigpower: AaopoolPattern::new(client.clone(), "megabigpower".to_string()),
            minerium: AaopoolPattern::new(client.clone(), "minerium".to_string()),
            miningcity: AaopoolPattern::new(client.clone(), "miningcity".to_string()),
            miningdutch: AaopoolPattern::new(client.clone(), "miningdutch".to_string()),
            miningkings: AaopoolPattern::new(client.clone(), "miningkings".to_string()),
            miningsquared: AaopoolPattern::new(client.clone(), "miningsquared".to_string()),
            mmpool: AaopoolPattern::new(client.clone(), "mmpool".to_string()),
            mtred: AaopoolPattern::new(client.clone(), "mtred".to_string()),
            multicoinco: AaopoolPattern::new(client.clone(), "multicoinco".to_string()),
            multipool: AaopoolPattern::new(client.clone(), "multipool".to_string()),
            mybtccoinpool: AaopoolPattern::new(client.clone(), "mybtccoinpool".to_string()),
            neopool: AaopoolPattern::new(client.clone(), "neopool".to_string()),
            nexious: AaopoolPattern::new(client.clone(), "nexious".to_string()),
            nicehash: AaopoolPattern::new(client.clone(), "nicehash".to_string()),
            nmcbit: AaopoolPattern::new(client.clone(), "nmcbit".to_string()),
            novablock: AaopoolPattern::new(client.clone(), "novablock".to_string()),
            ocean: AaopoolPattern::new(client.clone(), "ocean".to_string()),
            okexpool: AaopoolPattern::new(client.clone(), "okexpool".to_string()),
            okkong: AaopoolPattern::new(client.clone(), "okkong".to_string()),
            okminer: AaopoolPattern::new(client.clone(), "okminer".to_string()),
            okpooltop: AaopoolPattern::new(client.clone(), "okpooltop".to_string()),
            onehash: AaopoolPattern::new(client.clone(), "onehash".to_string()),
            onem1x: AaopoolPattern::new(client.clone(), "onem1x".to_string()),
            onethash: AaopoolPattern::new(client.clone(), "onethash".to_string()),
            ozcoin: AaopoolPattern::new(client.clone(), "ozcoin".to_string()),
            parasite: AaopoolPattern::new(client.clone(), "parasite".to_string()),
            patels: AaopoolPattern::new(client.clone(), "patels".to_string()),
            pegapool: AaopoolPattern::new(client.clone(), "pegapool".to_string()),
            phashio: AaopoolPattern::new(client.clone(), "phashio".to_string()),
            phoenix: AaopoolPattern::new(client.clone(), "phoenix".to_string()),
            polmine: AaopoolPattern::new(client.clone(), "polmine".to_string()),
            pool175btc: AaopoolPattern::new(client.clone(), "pool175btc".to_string()),
            pool50btc: AaopoolPattern::new(client.clone(), "pool50btc".to_string()),
            poolin: AaopoolPattern::new(client.clone(), "poolin".to_string()),
            portlandhodl: AaopoolPattern::new(client.clone(), "portlandhodl".to_string()),
            publicpool: AaopoolPattern::new(client.clone(), "publicpool".to_string()),
            purebtccom: AaopoolPattern::new(client.clone(), "purebtccom".to_string()),
            rawpool: AaopoolPattern::new(client.clone(), "rawpool".to_string()),
            rigpool: AaopoolPattern::new(client.clone(), "rigpool".to_string()),
            sbicrypto: AaopoolPattern::new(client.clone(), "sbicrypto".to_string()),
            secpool: AaopoolPattern::new(client.clone(), "secpool".to_string()),
            secretsuperstar: AaopoolPattern::new(client.clone(), "secretsuperstar".to_string()),
            sevenpool: AaopoolPattern::new(client.clone(), "sevenpool".to_string()),
            shawnp0wers: AaopoolPattern::new(client.clone(), "shawnp0wers".to_string()),
            sigmapoolcom: AaopoolPattern::new(client.clone(), "sigmapoolcom".to_string()),
            simplecoinus: AaopoolPattern::new(client.clone(), "simplecoinus".to_string()),
            solock: AaopoolPattern::new(client.clone(), "solock".to_string()),
            spiderpool: AaopoolPattern::new(client.clone(), "spiderpool".to_string()),
            stminingcorp: AaopoolPattern::new(client.clone(), "stminingcorp".to_string()),
            tangpool: AaopoolPattern::new(client.clone(), "tangpool".to_string()),
            tatmaspool: AaopoolPattern::new(client.clone(), "tatmaspool".to_string()),
            tbdice: AaopoolPattern::new(client.clone(), "tbdice".to_string()),
            telco214: AaopoolPattern::new(client.clone(), "telco214".to_string()),
            terrapool: AaopoolPattern::new(client.clone(), "terrapool".to_string()),
            tiger: AaopoolPattern::new(client.clone(), "tiger".to_string()),
            tigerpoolnet: AaopoolPattern::new(client.clone(), "tigerpoolnet".to_string()),
            titan: AaopoolPattern::new(client.clone(), "titan".to_string()),
            transactioncoinmining: AaopoolPattern::new(client.clone(), "transactioncoinmining".to_string()),
            trickysbtcpool: AaopoolPattern::new(client.clone(), "trickysbtcpool".to_string()),
            triplemining: AaopoolPattern::new(client.clone(), "triplemining".to_string()),
            twentyoneinc: AaopoolPattern::new(client.clone(), "twentyoneinc".to_string()),
            ultimuspool: AaopoolPattern::new(client.clone(), "ultimuspool".to_string()),
            unknown: AaopoolPattern::new(client.clone(), "unknown".to_string()),
            unomp: AaopoolPattern::new(client.clone(), "unomp".to_string()),
            viabtc: AaopoolPattern::new(client.clone(), "viabtc".to_string()),
            waterhole: AaopoolPattern::new(client.clone(), "waterhole".to_string()),
            wayicn: AaopoolPattern::new(client.clone(), "wayicn".to_string()),
            whitepool: AaopoolPattern::new(client.clone(), "whitepool".to_string()),
            wk057: AaopoolPattern::new(client.clone(), "wk057".to_string()),
            yourbtcnet: AaopoolPattern::new(client.clone(), "yourbtcnet".to_string()),
            zulupool: AaopoolPattern::new(client.clone(), "zulupool".to_string()),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Positions {
    pub block_position: MetricPattern12<BlkPosition>,
    pub tx_position: MetricPattern28<BlkPosition>,
}

impl CatalogTree_Positions {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            block_position: MetricPattern12::new(client.clone(), format!("{base_path}_block_position")),
            tx_position: MetricPattern28::new(client.clone(), format!("{base_path}_tx_position")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Price {
    pub cents: CatalogTree_Price_Cents,
    pub sats: SatsPattern,
    pub usd: SatsPattern,
}

impl CatalogTree_Price {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            cents: CatalogTree_Price_Cents::new(client.clone(), format!("{base_path}_cents")),
            sats: SatsPattern::new(client.clone(), "price".to_string()),
            usd: SatsPattern::new(client.clone(), "price".to_string()),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Price_Cents {
    pub ohlc: MetricPattern6<OHLCCents>,
    pub split: CatalogTree_Price_Cents_Split,
}

impl CatalogTree_Price_Cents {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            ohlc: MetricPattern6::new(client.clone(), format!("{base_path}_ohlc")),
            split: CatalogTree_Price_Cents_Split::new(client.clone(), format!("{base_path}_split")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Price_Cents_Split {
    pub dateindex: DateindexPattern2,
    pub height: DateindexPattern2,
}

impl CatalogTree_Price_Cents_Split {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            dateindex: DateindexPattern2::new(client.clone(), "price".to_string()),
            height: DateindexPattern2::new(client.clone(), "price".to_string()),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Scripts {
    pub count: CatalogTree_Scripts_Count,
    pub empty_to_txindex: MetricPattern10<TxIndex>,
    pub first_emptyoutputindex: MetricPattern12<EmptyOutputIndex>,
    pub first_opreturnindex: MetricPattern12<OpReturnIndex>,
    pub first_p2msoutputindex: MetricPattern12<P2MSOutputIndex>,
    pub first_unknownoutputindex: MetricPattern12<UnknownOutputIndex>,
    pub opreturn_to_txindex: MetricPattern15<TxIndex>,
    pub p2ms_to_txindex: MetricPattern18<TxIndex>,
    pub unknown_to_txindex: MetricPattern29<TxIndex>,
    pub value: CatalogTree_Scripts_Value,
}

impl CatalogTree_Scripts {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            count: CatalogTree_Scripts_Count::new(client.clone(), format!("{base_path}_count")),
            empty_to_txindex: MetricPattern10::new(client.clone(), format!("{base_path}_empty_to_txindex")),
            first_emptyoutputindex: MetricPattern12::new(client.clone(), format!("{base_path}_first_emptyoutputindex")),
            first_opreturnindex: MetricPattern12::new(client.clone(), format!("{base_path}_first_opreturnindex")),
            first_p2msoutputindex: MetricPattern12::new(client.clone(), format!("{base_path}_first_p2msoutputindex")),
            first_unknownoutputindex: MetricPattern12::new(client.clone(), format!("{base_path}_first_unknownoutputindex")),
            opreturn_to_txindex: MetricPattern15::new(client.clone(), format!("{base_path}_opreturn_to_txindex")),
            p2ms_to_txindex: MetricPattern18::new(client.clone(), format!("{base_path}_p2ms_to_txindex")),
            unknown_to_txindex: MetricPattern29::new(client.clone(), format!("{base_path}_unknown_to_txindex")),
            value: CatalogTree_Scripts_Value::new(client.clone(), format!("{base_path}_value")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Scripts_Count {
    pub emptyoutput: BitcoinPattern<StoredU64>,
    pub opreturn: BitcoinPattern<StoredU64>,
    pub p2a: BitcoinPattern<StoredU64>,
    pub p2ms: BitcoinPattern<StoredU64>,
    pub p2pk33: BitcoinPattern<StoredU64>,
    pub p2pk65: BitcoinPattern<StoredU64>,
    pub p2pkh: BitcoinPattern<StoredU64>,
    pub p2sh: BitcoinPattern<StoredU64>,
    pub p2tr: BitcoinPattern<StoredU64>,
    pub p2wpkh: BitcoinPattern<StoredU64>,
    pub p2wsh: BitcoinPattern<StoredU64>,
    pub segwit: BitcoinPattern<StoredU64>,
    pub segwit_adoption: SegwitAdoptionPattern,
    pub taproot_adoption: SegwitAdoptionPattern,
    pub unknownoutput: BitcoinPattern<StoredU64>,
}

impl CatalogTree_Scripts_Count {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            emptyoutput: BitcoinPattern::new(client.clone(), "emptyoutput_count".to_string()),
            opreturn: BitcoinPattern::new(client.clone(), "opreturn_count".to_string()),
            p2a: BitcoinPattern::new(client.clone(), "p2a_count".to_string()),
            p2ms: BitcoinPattern::new(client.clone(), "p2ms_count".to_string()),
            p2pk33: BitcoinPattern::new(client.clone(), "p2pk33_count".to_string()),
            p2pk65: BitcoinPattern::new(client.clone(), "p2pk65_count".to_string()),
            p2pkh: BitcoinPattern::new(client.clone(), "p2pkh_count".to_string()),
            p2sh: BitcoinPattern::new(client.clone(), "p2sh_count".to_string()),
            p2tr: BitcoinPattern::new(client.clone(), "p2tr_count".to_string()),
            p2wpkh: BitcoinPattern::new(client.clone(), "p2wpkh_count".to_string()),
            p2wsh: BitcoinPattern::new(client.clone(), "p2wsh_count".to_string()),
            segwit: BitcoinPattern::new(client.clone(), "segwit_count".to_string()),
            segwit_adoption: SegwitAdoptionPattern::new(client.clone(), "segwit_adoption".to_string()),
            taproot_adoption: SegwitAdoptionPattern::new(client.clone(), "taproot_adoption".to_string()),
            unknownoutput: BitcoinPattern::new(client.clone(), "unknownoutput_count".to_string()),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Scripts_Value {
    pub opreturn: CoinbasePattern,
}

impl CatalogTree_Scripts_Value {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            opreturn: CoinbasePattern::new(client.clone(), "opreturn_value".to_string()),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Supply {
    pub burned: CatalogTree_Supply_Burned,
    pub circulating: _24hCoinbaseSumPattern,
    pub inflation: MetricPattern4<StoredF32>,
    pub market_cap: MetricPattern1<Dollars>,
    pub velocity: CatalogTree_Supply_Velocity,
}

impl CatalogTree_Supply {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            burned: CatalogTree_Supply_Burned::new(client.clone(), format!("{base_path}_burned")),
            circulating: _24hCoinbaseSumPattern::new(client.clone(), "circulating_supply".to_string()),
            inflation: MetricPattern4::new(client.clone(), format!("{base_path}_inflation")),
            market_cap: MetricPattern1::new(client.clone(), format!("{base_path}_market_cap")),
            velocity: CatalogTree_Supply_Velocity::new(client.clone(), format!("{base_path}_velocity")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Supply_Burned {
    pub opreturn: UnclaimedRewardsPattern,
    pub unspendable: UnclaimedRewardsPattern,
}

impl CatalogTree_Supply_Burned {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            opreturn: UnclaimedRewardsPattern::new(client.clone(), "opreturn_supply".to_string()),
            unspendable: UnclaimedRewardsPattern::new(client.clone(), "unspendable_supply".to_string()),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Supply_Velocity {
    pub btc: MetricPattern4<StoredF64>,
    pub usd: MetricPattern4<StoredF64>,
}

impl CatalogTree_Supply_Velocity {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            btc: MetricPattern4::new(client.clone(), format!("{base_path}_btc")),
            usd: MetricPattern4::new(client.clone(), format!("{base_path}_usd")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Transactions {
    pub base_size: MetricPattern28<StoredU32>,
    pub count: CatalogTree_Transactions_Count,
    pub fees: CatalogTree_Transactions_Fees,
    pub first_txindex: MetricPattern12<TxIndex>,
    pub first_txinindex: MetricPattern28<TxInIndex>,
    pub first_txoutindex: MetricPattern28<TxOutIndex>,
    pub height: MetricPattern28<Height>,
    pub is_explicitly_rbf: MetricPattern28<StoredBool>,
    pub rawlocktime: MetricPattern28<RawLockTime>,
    pub size: CatalogTree_Transactions_Size,
    pub total_size: MetricPattern28<StoredU32>,
    pub txid: MetricPattern28<Txid>,
    pub txversion: MetricPattern28<TxVersion>,
    pub versions: CatalogTree_Transactions_Versions,
    pub volume: CatalogTree_Transactions_Volume,
}

impl CatalogTree_Transactions {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            base_size: MetricPattern28::new(client.clone(), format!("{base_path}_base_size")),
            count: CatalogTree_Transactions_Count::new(client.clone(), format!("{base_path}_count")),
            fees: CatalogTree_Transactions_Fees::new(client.clone(), format!("{base_path}_fees")),
            first_txindex: MetricPattern12::new(client.clone(), format!("{base_path}_first_txindex")),
            first_txinindex: MetricPattern28::new(client.clone(), format!("{base_path}_first_txinindex")),
            first_txoutindex: MetricPattern28::new(client.clone(), format!("{base_path}_first_txoutindex")),
            height: MetricPattern28::new(client.clone(), format!("{base_path}_height")),
            is_explicitly_rbf: MetricPattern28::new(client.clone(), format!("{base_path}_is_explicitly_rbf")),
            rawlocktime: MetricPattern28::new(client.clone(), format!("{base_path}_rawlocktime")),
            size: CatalogTree_Transactions_Size::new(client.clone(), format!("{base_path}_size")),
            total_size: MetricPattern28::new(client.clone(), format!("{base_path}_total_size")),
            txid: MetricPattern28::new(client.clone(), format!("{base_path}_txid")),
            txversion: MetricPattern28::new(client.clone(), format!("{base_path}_txversion")),
            versions: CatalogTree_Transactions_Versions::new(client.clone(), format!("{base_path}_versions")),
            volume: CatalogTree_Transactions_Volume::new(client.clone(), format!("{base_path}_volume")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Transactions_Count {
    pub is_coinbase: MetricPattern28<StoredBool>,
    pub tx_count: BitcoinPattern<StoredU64>,
}

impl CatalogTree_Transactions_Count {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            is_coinbase: MetricPattern28::new(client.clone(), format!("{base_path}_is_coinbase")),
            tx_count: BitcoinPattern::new(client.clone(), "tx_count".to_string()),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Transactions_Fees {
    pub fee: CatalogTree_Transactions_Fees_Fee,
    pub fee_rate: FeeRatePattern<FeeRate>,
    pub input_value: MetricPattern28<Sats>,
    pub output_value: MetricPattern28<Sats>,
}

impl CatalogTree_Transactions_Fees {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            fee: CatalogTree_Transactions_Fees_Fee::new(client.clone(), format!("{base_path}_fee")),
            fee_rate: FeeRatePattern::new(client.clone(), "fee_rate".to_string()),
            input_value: MetricPattern28::new(client.clone(), format!("{base_path}_input_value")),
            output_value: MetricPattern28::new(client.clone(), format!("{base_path}_output_value")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Transactions_Fees_Fee {
    pub bitcoin: SizePattern<Bitcoin>,
    pub dollars: SizePattern<Dollars>,
    pub sats: SizePattern<Sats>,
    pub txindex: MetricPattern28<Sats>,
}

impl CatalogTree_Transactions_Fees_Fee {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            bitcoin: SizePattern::new(client.clone(), "fee_btc".to_string()),
            dollars: SizePattern::new(client.clone(), "fee_usd".to_string()),
            sats: SizePattern::new(client.clone(), "fee".to_string()),
            txindex: MetricPattern28::new(client.clone(), format!("{base_path}_txindex")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Transactions_Size {
    pub vsize: FeeRatePattern<VSize>,
    pub weight: FeeRatePattern<Weight>,
}

impl CatalogTree_Transactions_Size {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            vsize: FeeRatePattern::new(client.clone(), "".to_string()),
            weight: FeeRatePattern::new(client.clone(), "".to_string()),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Transactions_Versions {
    pub v1: BlockCountPattern<StoredU64>,
    pub v2: BlockCountPattern<StoredU64>,
    pub v3: BlockCountPattern<StoredU64>,
}

impl CatalogTree_Transactions_Versions {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            v1: BlockCountPattern::new(client.clone(), "tx_v1".to_string()),
            v2: BlockCountPattern::new(client.clone(), "tx_v2".to_string()),
            v3: BlockCountPattern::new(client.clone(), "tx_v3".to_string()),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Transactions_Volume {
    pub annualized_volume: _24hCoinbaseSumPattern,
    pub inputs_per_sec: MetricPattern4<StoredF32>,
    pub outputs_per_sec: MetricPattern4<StoredF32>,
    pub sent_sum: _24hCoinbaseSumPattern,
    pub tx_per_sec: MetricPattern4<StoredF32>,
}

impl CatalogTree_Transactions_Volume {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            annualized_volume: _24hCoinbaseSumPattern::new(client.clone(), "annualized_volume".to_string()),
            inputs_per_sec: MetricPattern4::new(client.clone(), format!("{base_path}_inputs_per_sec")),
            outputs_per_sec: MetricPattern4::new(client.clone(), format!("{base_path}_outputs_per_sec")),
            sent_sum: _24hCoinbaseSumPattern::new(client.clone(), "sent_sum".to_string()),
            tx_per_sec: MetricPattern4::new(client.clone(), format!("{base_path}_tx_per_sec")),
        }
    }
}

/// Main BRK client with catalog tree and API methods.
pub struct BrkClient {
    base: Arc<BrkClientBase>,
    tree: CatalogTree,
}

impl BrkClient {
    /// Client version.
    pub const VERSION: &'static str = "v0.1.0-alpha.2";

    /// Create a new client with the given base URL.
    pub fn new(base_url: impl Into<String>) -> Self {
        let base = Arc::new(BrkClientBase::new(base_url));
        let tree = CatalogTree::new(base.clone(), String::new());
        Self { base, tree }
    }

    /// Create a new client with options.
    pub fn with_options(options: BrkClientOptions) -> Self {
        let base = Arc::new(BrkClientBase::with_options(options));
        let tree = CatalogTree::new(base.clone(), String::new());
        Self { base, tree }
    }

    /// Get the catalog tree for navigating metrics.
    pub fn tree(&self) -> &CatalogTree {
        &self.tree
    }

    /// Address information
    ///
    /// Retrieve comprehensive information about a Bitcoin address including balance, transaction history, UTXOs, and estimated investment metrics. Supports all standard Bitcoin address types (P2PKH, P2SH, P2WPKH, P2WSH, P2TR, etc.).
    pub fn get_address(&self, address: &str) -> Result<AddressStats> {
        self.base.get(&format!("/api/address/{address}"))
    }

    /// Address transaction IDs
    ///
    /// Get transaction IDs for an address, newest first. Use after_txid for pagination.
    pub fn get_address_txs(&self, address: &str, after_txid: Option<&str>, limit: Option<&str>) -> Result<Vec<Txid>> {
        let mut query = Vec::new();
        if let Some(v) = after_txid { query.push(format!("after_txid={}", v)); }
        if let Some(v) = limit { query.push(format!("limit={}", v)); }
        let query_str = if query.is_empty() { String::new() } else { format!("?{}", query.join("&")) };
        self.base.get(&format!("/api/address/{address}/txs{}", query_str))
    }

    /// Address confirmed transactions
    ///
    /// Get confirmed transaction IDs for an address, 25 per page. Use ?after_txid=<txid> for pagination.
    pub fn get_address_txs_chain(&self, address: &str, after_txid: Option<&str>, limit: Option<&str>) -> Result<Vec<Txid>> {
        let mut query = Vec::new();
        if let Some(v) = after_txid { query.push(format!("after_txid={}", v)); }
        if let Some(v) = limit { query.push(format!("limit={}", v)); }
        let query_str = if query.is_empty() { String::new() } else { format!("?{}", query.join("&")) };
        self.base.get(&format!("/api/address/{address}/txs/chain{}", query_str))
    }

    /// Address mempool transactions
    ///
    /// Get unconfirmed transaction IDs for an address from the mempool (up to 50).
    pub fn get_address_txs_mempool(&self, address: &str) -> Result<Vec<Txid>> {
        self.base.get(&format!("/api/address/{address}/txs/mempool"))
    }

    /// Address UTXOs
    ///
    /// Get unspent transaction outputs for an address.
    pub fn get_address_utxo(&self, address: &str) -> Result<Vec<Utxo>> {
        self.base.get(&format!("/api/address/{address}/utxo"))
    }

    /// Block by height
    ///
    /// Retrieve block information by block height. Returns block metadata including hash, timestamp, difficulty, size, weight, and transaction count.
    pub fn get_block_height(&self, height: &str) -> Result<BlockInfo> {
        self.base.get(&format!("/api/block-height/{height}"))
    }

    /// Block information
    ///
    /// Retrieve block information by block hash. Returns block metadata including height, timestamp, difficulty, size, weight, and transaction count.
    pub fn get_block_by_hash(&self, hash: &str) -> Result<BlockInfo> {
        self.base.get(&format!("/api/block/{hash}"))
    }

    /// Raw block
    ///
    /// Returns the raw block data in binary format.
    pub fn get_block_by_hash_raw(&self, hash: &str) -> Result<Vec<f64>> {
        self.base.get(&format!("/api/block/{hash}/raw"))
    }

    /// Block status
    ///
    /// Retrieve the status of a block. Returns whether the block is in the best chain and, if so, its height and the hash of the next block.
    pub fn get_block_by_hash_status(&self, hash: &str) -> Result<BlockStatus> {
        self.base.get(&format!("/api/block/{hash}/status"))
    }

    /// Transaction ID at index
    ///
    /// Retrieve a single transaction ID at a specific index within a block. Returns plain text txid.
    pub fn get_block_by_hash_txid_by_index(&self, hash: &str, index: &str) -> Result<Txid> {
        self.base.get(&format!("/api/block/{hash}/txid/{index}"))
    }

    /// Block transaction IDs
    ///
    /// Retrieve all transaction IDs in a block by block hash.
    pub fn get_block_by_hash_txids(&self, hash: &str) -> Result<Vec<Txid>> {
        self.base.get(&format!("/api/block/{hash}/txids"))
    }

    /// Block transactions (paginated)
    ///
    /// Retrieve transactions in a block by block hash, starting from the specified index. Returns up to 25 transactions at a time.
    pub fn get_block_by_hash_txs_by_start_index(&self, hash: &str, start_index: &str) -> Result<Vec<Transaction>> {
        self.base.get(&format!("/api/block/{hash}/txs/{start_index}"))
    }

    /// Recent blocks
    ///
    /// Retrieve the last 10 blocks. Returns block metadata for each block.
    pub fn get_blocks(&self) -> Result<Vec<BlockInfo>> {
        self.base.get(&format!("/api/blocks"))
    }

    /// Blocks from height
    ///
    /// Retrieve up to 10 blocks going backwards from the given height. For example, height=100 returns blocks 100, 99, 98, ..., 91. Height=0 returns only block 0.
    pub fn get_blocks_by_height(&self, height: &str) -> Result<Vec<BlockInfo>> {
        self.base.get(&format!("/api/blocks/{height}"))
    }

    /// Mempool statistics
    ///
    /// Get current mempool statistics including transaction count, total vsize, and total fees.
    pub fn get_mempool_info(&self) -> Result<MempoolInfo> {
        self.base.get(&format!("/api/mempool/info"))
    }

    /// Mempool transaction IDs
    ///
    /// Get all transaction IDs currently in the mempool.
    pub fn get_mempool_txids(&self) -> Result<Vec<Txid>> {
        self.base.get(&format!("/api/mempool/txids"))
    }

    /// Get supported indexes for a metric
    ///
    /// Returns the list of indexes are supported by the specified metric. For example, `realized_price` might be available on dateindex, weekindex, and monthindex.
    pub fn get_metric(&self, metric: &str) -> Result<Vec<Index>> {
        self.base.get(&format!("/api/metric/{metric}"))
    }

    /// Get metric data
    ///
    /// Fetch data for a specific metric at the given index. Use query parameters to filter by date range and format (json/csv).
    pub fn get_metric_by_index(&self, index: &str, metric: &str, count: Option<&str>, format: Option<&str>, from: Option<&str>, to: Option<&str>) -> Result<MetricData> {
        let mut query = Vec::new();
        if let Some(v) = count { query.push(format!("count={}", v)); }
        if let Some(v) = format { query.push(format!("format={}", v)); }
        if let Some(v) = from { query.push(format!("from={}", v)); }
        if let Some(v) = to { query.push(format!("to={}", v)); }
        let query_str = if query.is_empty() { String::new() } else { format!("?{}", query.join("&")) };
        self.base.get(&format!("/api/metric/{metric}/{index}{}", query_str))
    }

    /// Bulk metric data
    ///
    /// Fetch multiple metrics in a single request. Supports filtering by index and date range. Returns an array of MetricData objects.
    pub fn get_metrics_bulk(&self, count: Option<&str>, format: Option<&str>, from: Option<&str>, index: &str, metrics: &str, to: Option<&str>) -> Result<Vec<MetricData>> {
        let mut query = Vec::new();
        if let Some(v) = count { query.push(format!("count={}", v)); }
        if let Some(v) = format { query.push(format!("format={}", v)); }
        if let Some(v) = from { query.push(format!("from={}", v)); }
        query.push(format!("index={}", index));
        query.push(format!("metrics={}", metrics));
        if let Some(v) = to { query.push(format!("to={}", v)); }
        let query_str = if query.is_empty() { String::new() } else { format!("?{}", query.join("&")) };
        self.base.get(&format!("/api/metrics/bulk{}", query_str))
    }

    /// Metrics catalog
    ///
    /// Returns the complete hierarchical catalog of available metrics organized as a tree structure. Metrics are grouped by categories and subcategories. Best viewed in an interactive JSON viewer (e.g., Firefox's built-in JSON viewer) for easy navigation of the nested structure.
    pub fn get_metrics_catalog(&self) -> Result<TreeNode> {
        self.base.get(&format!("/api/metrics/catalog"))
    }

    /// Metric count
    ///
    /// Current metric count
    pub fn get_metrics_count(&self) -> Result<Vec<MetricCount>> {
        self.base.get(&format!("/api/metrics/count"))
    }

    /// List available indexes
    ///
    /// Returns all available indexes with their accepted query aliases. Use any alias when querying metrics.
    pub fn get_metrics_indexes(&self) -> Result<Vec<IndexInfo>> {
        self.base.get(&format!("/api/metrics/indexes"))
    }

    /// Metrics list
    ///
    /// Paginated list of available metrics
    pub fn get_metrics_list(&self, page: Option<&str>) -> Result<PaginatedMetrics> {
        let mut query = Vec::new();
        if let Some(v) = page { query.push(format!("page={}", v)); }
        let query_str = if query.is_empty() { String::new() } else { format!("?{}", query.join("&")) };
        self.base.get(&format!("/api/metrics/list{}", query_str))
    }

    /// Search metrics
    ///
    /// Fuzzy search for metrics by name. Supports partial matches and typos.
    pub fn get_metrics_search_by_metric(&self, metric: &str, limit: Option<&str>) -> Result<Vec<Metric>> {
        let mut query = Vec::new();
        if let Some(v) = limit { query.push(format!("limit={}", v)); }
        let query_str = if query.is_empty() { String::new() } else { format!("?{}", query.join("&")) };
        self.base.get(&format!("/api/metrics/search/{metric}{}", query_str))
    }

    /// Transaction information
    ///
    /// Retrieve complete transaction data by transaction ID (txid). Returns the full transaction details including inputs, outputs, and metadata. The transaction data is read directly from the blockchain data files.
    pub fn get_tx_by_txid(&self, txid: &str) -> Result<Transaction> {
        self.base.get(&format!("/api/tx/{txid}"))
    }

    /// Transaction hex
    ///
    /// Retrieve the raw transaction as a hex-encoded string. Returns the serialized transaction in hexadecimal format.
    pub fn get_tx_by_txid_hex(&self, txid: &str) -> Result<Hex> {
        self.base.get(&format!("/api/tx/{txid}/hex"))
    }

    /// Output spend status
    ///
    /// Get the spending status of a transaction output. Returns whether the output has been spent and, if so, the spending transaction details.
    pub fn get_tx_by_txid_outspend_by_vout(&self, txid: &str, vout: &str) -> Result<TxOutspend> {
        self.base.get(&format!("/api/tx/{txid}/outspend/{vout}"))
    }

    /// All output spend statuses
    ///
    /// Get the spending status of all outputs in a transaction. Returns an array with the spend status for each output.
    pub fn get_tx_by_txid_outspends(&self, txid: &str) -> Result<Vec<TxOutspend>> {
        self.base.get(&format!("/api/tx/{txid}/outspends"))
    }

    /// Transaction status
    ///
    /// Retrieve the confirmation status of a transaction. Returns whether the transaction is confirmed and, if so, the block height, hash, and timestamp.
    pub fn get_tx_by_txid_status(&self, txid: &str) -> Result<TxStatus> {
        self.base.get(&format!("/api/tx/{txid}/status"))
    }

    /// Difficulty adjustment
    ///
    /// Get current difficulty adjustment information including progress through the current epoch, estimated retarget date, and difficulty change prediction.
    pub fn get_v1_difficulty_adjustment(&self) -> Result<DifficultyAdjustment> {
        self.base.get(&format!("/api/v1/difficulty-adjustment"))
    }

    /// Projected mempool blocks
    ///
    /// Get projected blocks from the mempool for fee estimation. Each block contains statistics about transactions that would be included if a block were mined now.
    pub fn get_v1_fees_mempool_blocks(&self) -> Result<Vec<MempoolBlock>> {
        self.base.get(&format!("/api/v1/fees/mempool-blocks"))
    }

    /// Recommended fees
    ///
    /// Get recommended fee rates for different confirmation targets based on current mempool state.
    pub fn get_v1_fees_recommended(&self) -> Result<RecommendedFees> {
        self.base.get(&format!("/api/v1/fees/recommended"))
    }

    /// Block fees
    ///
    /// Get average block fees for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y
    pub fn get_v1_mining_blocks_fees_by_time_period(&self, time_period: &str) -> Result<Vec<BlockFeesEntry>> {
        self.base.get(&format!("/api/v1/mining/blocks/fees/{time_period}"))
    }

    /// Block rewards
    ///
    /// Get average block rewards (coinbase = subsidy + fees) for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y
    pub fn get_v1_mining_blocks_rewards_by_time_period(&self, time_period: &str) -> Result<Vec<BlockRewardsEntry>> {
        self.base.get(&format!("/api/v1/mining/blocks/rewards/{time_period}"))
    }

    /// Block sizes and weights
    ///
    /// Get average block sizes and weights for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y
    pub fn get_v1_mining_blocks_sizes_weights_by_time_period(&self, time_period: &str) -> Result<BlockSizesWeights> {
        self.base.get(&format!("/api/v1/mining/blocks/sizes-weights/{time_period}"))
    }

    /// Block by timestamp
    ///
    /// Find the block closest to a given UNIX timestamp.
    pub fn get_v1_mining_blocks_timestamp(&self, timestamp: &str) -> Result<BlockTimestamp> {
        self.base.get(&format!("/api/v1/mining/blocks/timestamp/{timestamp}"))
    }

    /// Difficulty adjustments (all time)
    ///
    /// Get historical difficulty adjustments. Returns array of [timestamp, height, difficulty, change_percent].
    pub fn get_v1_mining_difficulty_adjustments(&self) -> Result<Vec<DifficultyAdjustmentEntry>> {
        self.base.get(&format!("/api/v1/mining/difficulty-adjustments"))
    }

    /// Difficulty adjustments
    ///
    /// Get historical difficulty adjustments for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y. Returns array of [timestamp, height, difficulty, change_percent].
    pub fn get_v1_mining_difficulty_adjustments_by_time_period(&self, time_period: &str) -> Result<Vec<DifficultyAdjustmentEntry>> {
        self.base.get(&format!("/api/v1/mining/difficulty-adjustments/{time_period}"))
    }

    /// Network hashrate (all time)
    ///
    /// Get network hashrate and difficulty data for all time.
    pub fn get_v1_mining_hashrate(&self) -> Result<HashrateSummary> {
        self.base.get(&format!("/api/v1/mining/hashrate"))
    }

    /// Network hashrate
    ///
    /// Get network hashrate and difficulty data for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y
    pub fn get_v1_mining_hashrate_by_time_period(&self, time_period: &str) -> Result<HashrateSummary> {
        self.base.get(&format!("/api/v1/mining/hashrate/{time_period}"))
    }

    /// Mining pool details
    ///
    /// Get detailed information about a specific mining pool including block counts and shares for different time periods.
    pub fn get_v1_mining_pool_by_slug(&self, slug: &str) -> Result<PoolDetail> {
        self.base.get(&format!("/api/v1/mining/pool/{slug}"))
    }

    /// List all mining pools
    ///
    /// Get list of all known mining pools with their identifiers.
    pub fn get_v1_mining_pools(&self) -> Result<Vec<PoolInfo>> {
        self.base.get(&format!("/api/v1/mining/pools"))
    }

    /// Mining pool statistics
    ///
    /// Get mining pool statistics for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y
    pub fn get_v1_mining_pools_by_time_period(&self, time_period: &str) -> Result<PoolsSummary> {
        self.base.get(&format!("/api/v1/mining/pools/{time_period}"))
    }

    /// Mining reward statistics
    ///
    /// Get mining reward statistics for the last N blocks including total rewards, fees, and transaction count.
    pub fn get_v1_mining_reward_stats_by_block_count(&self, block_count: &str) -> Result<RewardStats> {
        self.base.get(&format!("/api/v1/mining/reward-stats/{block_count}"))
    }

    /// Validate address
    ///
    /// Validate a Bitcoin address and get information about its type and scriptPubKey.
    pub fn get_v1_validate_address(&self, address: &str) -> Result<AddressValidation> {
        self.base.get(&format!("/api/v1/validate-address/{address}"))
    }

    /// Health check
    ///
    /// Returns the health status of the API server
    pub fn get_health(&self) -> Result<Health> {
        self.base.get(&format!("/health"))
    }

    /// API version
    ///
    /// Returns the current version of the API server
    pub fn get_version(&self) -> Result<String> {
        self.base.get(&format!("/version"))
    }

}
