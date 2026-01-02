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

/// Index accessor for metrics with 7 indexes.
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
            Index::Height,
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
pub struct MetricPattern6By<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: DeserializeOwned> MetricPattern6By<T> {
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
            Index::DecadeIndex,
            Index::MonthIndex,
            Index::QuarterIndex,
            Index::SemesterIndex,
            Index::WeekIndex,
            Index::YearIndex,
        ]
    }
}

impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern6<T> {
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
pub struct MetricPattern7By<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: DeserializeOwned> MetricPattern7By<T> {
    pub fn by_emptyoutputindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::EmptyOutputIndex)
    }
    pub fn by_opreturnindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::OpReturnIndex)
    }
    pub fn by_p2msoutputindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::P2MSOutputIndex)
    }
    pub fn by_unknownoutputindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::UnknownOutputIndex)
    }
}

/// Index accessor for metrics with 4 indexes.
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
            Index::EmptyOutputIndex,
            Index::OpReturnIndex,
            Index::P2MSOutputIndex,
            Index::UnknownOutputIndex,
        ]
    }
}

impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern7<T> {
    fn get(&self, index: Index) -> Option<Endpoint<T>> {
        match index {
            Index::EmptyOutputIndex => Some(self.by.by_emptyoutputindex()),
            Index::OpReturnIndex => Some(self.by.by_opreturnindex()),
            Index::P2MSOutputIndex => Some(self.by.by_p2msoutputindex()),
            Index::UnknownOutputIndex => Some(self.by.by_unknownoutputindex()),
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
    pub fn by_quarterindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::QuarterIndex)
    }
    pub fn by_semesterindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::SemesterIndex)
    }
    pub fn by_yearindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::YearIndex)
    }
}

/// Index accessor for metrics with 3 indexes.
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
            Index::QuarterIndex,
            Index::SemesterIndex,
            Index::YearIndex,
        ]
    }
}

impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern8<T> {
    fn get(&self, index: Index) -> Option<Endpoint<T>> {
        match index {
            Index::QuarterIndex => Some(self.by.by_quarterindex()),
            Index::SemesterIndex => Some(self.by.by_semesterindex()),
            Index::YearIndex => Some(self.by.by_yearindex()),
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
    pub fn by_dateindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::DateIndex)
    }
    pub fn by_height(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::Height)
    }
}

/// Index accessor for metrics with 2 indexes.
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
            Index::DateIndex,
            Index::Height,
        ]
    }
}

impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern9<T> {
    fn get(&self, index: Index) -> Option<Endpoint<T>> {
        match index {
            Index::DateIndex => Some(self.by.by_dateindex()),
            Index::Height => Some(self.by.by_height()),
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
    pub fn by_dateindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::DateIndex)
    }
    pub fn by_monthindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::MonthIndex)
    }
}

/// Index accessor for metrics with 2 indexes.
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
            Index::DateIndex,
            Index::MonthIndex,
        ]
    }
}

impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern10<T> {
    fn get(&self, index: Index) -> Option<Endpoint<T>> {
        match index {
            Index::DateIndex => Some(self.by.by_dateindex()),
            Index::MonthIndex => Some(self.by.by_monthindex()),
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
    pub fn by_dateindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::DateIndex)
    }
    pub fn by_weekindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::WeekIndex)
    }
}

/// Index accessor for metrics with 2 indexes.
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
            Index::DateIndex,
            Index::WeekIndex,
        ]
    }
}

impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern11<T> {
    fn get(&self, index: Index) -> Option<Endpoint<T>> {
        match index {
            Index::DateIndex => Some(self.by.by_dateindex()),
            Index::WeekIndex => Some(self.by.by_weekindex()),
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
    pub fn by_decadeindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::DecadeIndex)
    }
    pub fn by_yearindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::YearIndex)
    }
}

/// Index accessor for metrics with 2 indexes.
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
            Index::DecadeIndex,
            Index::YearIndex,
        ]
    }
}

impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern12<T> {
    fn get(&self, index: Index) -> Option<Endpoint<T>> {
        match index {
            Index::DecadeIndex => Some(self.by.by_decadeindex()),
            Index::YearIndex => Some(self.by.by_yearindex()),
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
    pub fn by_difficultyepoch(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::DifficultyEpoch)
    }
    pub fn by_halvingepoch(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::HalvingEpoch)
    }
}

/// Index accessor for metrics with 2 indexes.
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
            Index::DifficultyEpoch,
            Index::HalvingEpoch,
        ]
    }
}

impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern13<T> {
    fn get(&self, index: Index) -> Option<Endpoint<T>> {
        match index {
            Index::DifficultyEpoch => Some(self.by.by_difficultyepoch()),
            Index::HalvingEpoch => Some(self.by.by_halvingepoch()),
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
    pub fn by_difficultyepoch(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::DifficultyEpoch)
    }
    pub fn by_height(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::Height)
    }
}

/// Index accessor for metrics with 2 indexes.
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
            Index::DifficultyEpoch,
            Index::Height,
        ]
    }
}

impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern14<T> {
    fn get(&self, index: Index) -> Option<Endpoint<T>> {
        match index {
            Index::DifficultyEpoch => Some(self.by.by_difficultyepoch()),
            Index::Height => Some(self.by.by_height()),
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
    pub fn by_halvingepoch(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::HalvingEpoch)
    }
    pub fn by_height(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::Height)
    }
}

/// Index accessor for metrics with 2 indexes.
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
            Index::HalvingEpoch,
            Index::Height,
        ]
    }
}

impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern15<T> {
    fn get(&self, index: Index) -> Option<Endpoint<T>> {
        match index {
            Index::HalvingEpoch => Some(self.by.by_halvingepoch()),
            Index::Height => Some(self.by.by_height()),
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
    pub fn by_height(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::Height)
    }
    pub fn by_txindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::TxIndex)
    }
}

/// Index accessor for metrics with 2 indexes.
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
            Index::Height,
            Index::TxIndex,
        ]
    }
}

impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern16<T> {
    fn get(&self, index: Index) -> Option<Endpoint<T>> {
        match index {
            Index::Height => Some(self.by.by_height()),
            Index::TxIndex => Some(self.by.by_txindex()),
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
    pub fn by_monthindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::MonthIndex)
    }
    pub fn by_quarterindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::QuarterIndex)
    }
}

/// Index accessor for metrics with 2 indexes.
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
            Index::MonthIndex,
            Index::QuarterIndex,
        ]
    }
}

impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern17<T> {
    fn get(&self, index: Index) -> Option<Endpoint<T>> {
        match index {
            Index::MonthIndex => Some(self.by.by_monthindex()),
            Index::QuarterIndex => Some(self.by.by_quarterindex()),
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
    pub fn by_monthindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::MonthIndex)
    }
    pub fn by_semesterindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::SemesterIndex)
    }
}

/// Index accessor for metrics with 2 indexes.
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
            Index::MonthIndex,
            Index::SemesterIndex,
        ]
    }
}

impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern18<T> {
    fn get(&self, index: Index) -> Option<Endpoint<T>> {
        match index {
            Index::MonthIndex => Some(self.by.by_monthindex()),
            Index::SemesterIndex => Some(self.by.by_semesterindex()),
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
    pub fn by_monthindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::MonthIndex)
    }
    pub fn by_weekindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::WeekIndex)
    }
}

/// Index accessor for metrics with 2 indexes.
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
            Index::MonthIndex,
            Index::WeekIndex,
        ]
    }
}

impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern19<T> {
    fn get(&self, index: Index) -> Option<Endpoint<T>> {
        match index {
            Index::MonthIndex => Some(self.by.by_monthindex()),
            Index::WeekIndex => Some(self.by.by_weekindex()),
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
    pub fn by_monthindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::MonthIndex)
    }
    pub fn by_yearindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::YearIndex)
    }
}

/// Index accessor for metrics with 2 indexes.
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
            Index::MonthIndex,
            Index::YearIndex,
        ]
    }
}

impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern20<T> {
    fn get(&self, index: Index) -> Option<Endpoint<T>> {
        match index {
            Index::MonthIndex => Some(self.by.by_monthindex()),
            Index::YearIndex => Some(self.by.by_yearindex()),
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
    pub fn by_dateindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::DateIndex)
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
            Index::DateIndex,
        ]
    }
}

impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern21<T> {
    fn get(&self, index: Index) -> Option<Endpoint<T>> {
        match index {
            Index::DateIndex => Some(self.by.by_dateindex()),
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
    pub fn by_decadeindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::DecadeIndex)
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
            Index::DecadeIndex,
        ]
    }
}

impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern22<T> {
    fn get(&self, index: Index) -> Option<Endpoint<T>> {
        match index {
            Index::DecadeIndex => Some(self.by.by_decadeindex()),
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
    pub fn by_difficultyepoch(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::DifficultyEpoch)
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
            Index::DifficultyEpoch,
        ]
    }
}

impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern23<T> {
    fn get(&self, index: Index) -> Option<Endpoint<T>> {
        match index {
            Index::DifficultyEpoch => Some(self.by.by_difficultyepoch()),
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
    pub fn by_emptyoutputindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::EmptyOutputIndex)
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
            Index::EmptyOutputIndex,
        ]
    }
}

impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern24<T> {
    fn get(&self, index: Index) -> Option<Endpoint<T>> {
        match index {
            Index::EmptyOutputIndex => Some(self.by.by_emptyoutputindex()),
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
    pub fn by_height(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::Height)
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
            Index::Height,
        ]
    }
}

impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern25<T> {
    fn get(&self, index: Index) -> Option<Endpoint<T>> {
        match index {
            Index::Height => Some(self.by.by_height()),
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
    pub fn by_txinindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::TxInIndex)
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
            Index::TxInIndex,
        ]
    }
}

impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern26<T> {
    fn get(&self, index: Index) -> Option<Endpoint<T>> {
        match index {
            Index::TxInIndex => Some(self.by.by_txinindex()),
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
    pub fn by_monthindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::MonthIndex)
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
            Index::MonthIndex,
        ]
    }
}

impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern27<T> {
    fn get(&self, index: Index) -> Option<Endpoint<T>> {
        match index {
            Index::MonthIndex => Some(self.by.by_monthindex()),
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
    pub fn by_opreturnindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::OpReturnIndex)
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
            Index::OpReturnIndex,
        ]
    }
}

impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern28<T> {
    fn get(&self, index: Index) -> Option<Endpoint<T>> {
        match index {
            Index::OpReturnIndex => Some(self.by.by_opreturnindex()),
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
    pub fn by_txoutindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::TxOutIndex)
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
            Index::TxOutIndex,
        ]
    }
}

impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern29<T> {
    fn get(&self, index: Index) -> Option<Endpoint<T>> {
        match index {
            Index::TxOutIndex => Some(self.by.by_txoutindex()),
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
    pub fn by_p2aaddressindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::P2AAddressIndex)
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
            Index::P2AAddressIndex,
        ]
    }
}

impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern30<T> {
    fn get(&self, index: Index) -> Option<Endpoint<T>> {
        match index {
            Index::P2AAddressIndex => Some(self.by.by_p2aaddressindex()),
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
    pub fn by_p2msoutputindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::P2MSOutputIndex)
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
            Index::P2MSOutputIndex,
        ]
    }
}

impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern31<T> {
    fn get(&self, index: Index) -> Option<Endpoint<T>> {
        match index {
            Index::P2MSOutputIndex => Some(self.by.by_p2msoutputindex()),
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
    pub fn by_p2pk33addressindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::P2PK33AddressIndex)
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
            Index::P2PK33AddressIndex,
        ]
    }
}

impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern32<T> {
    fn get(&self, index: Index) -> Option<Endpoint<T>> {
        match index {
            Index::P2PK33AddressIndex => Some(self.by.by_p2pk33addressindex()),
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
    pub fn by_p2pk65addressindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::P2PK65AddressIndex)
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
            Index::P2PK65AddressIndex,
        ]
    }
}

impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern33<T> {
    fn get(&self, index: Index) -> Option<Endpoint<T>> {
        match index {
            Index::P2PK65AddressIndex => Some(self.by.by_p2pk65addressindex()),
            _ => None,
        }
    }
}

/// Container for index endpoint methods.
pub struct MetricPattern34By<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: DeserializeOwned> MetricPattern34By<T> {
    pub fn by_p2pkhaddressindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::P2PKHAddressIndex)
    }
}

/// Index accessor for metrics with 1 indexes.
pub struct MetricPattern34<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    pub by: MetricPattern34By<T>,
}

impl<T: DeserializeOwned> MetricPattern34<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self {
        let name: Arc<str> = name.into();
        Self {
            client: client.clone(),
            name: name.clone(),
            by: MetricPattern34By {
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

impl<T> AnyMetricPattern for MetricPattern34<T> {
    fn name(&self) -> &str {
        &self.name
    }

    fn indexes(&self) -> &'static [Index] {
        &[
            Index::P2PKHAddressIndex,
        ]
    }
}

impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern34<T> {
    fn get(&self, index: Index) -> Option<Endpoint<T>> {
        match index {
            Index::P2PKHAddressIndex => Some(self.by.by_p2pkhaddressindex()),
            _ => None,
        }
    }
}

/// Container for index endpoint methods.
pub struct MetricPattern35By<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: DeserializeOwned> MetricPattern35By<T> {
    pub fn by_p2shaddressindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::P2SHAddressIndex)
    }
}

/// Index accessor for metrics with 1 indexes.
pub struct MetricPattern35<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    pub by: MetricPattern35By<T>,
}

impl<T: DeserializeOwned> MetricPattern35<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self {
        let name: Arc<str> = name.into();
        Self {
            client: client.clone(),
            name: name.clone(),
            by: MetricPattern35By {
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

impl<T> AnyMetricPattern for MetricPattern35<T> {
    fn name(&self) -> &str {
        &self.name
    }

    fn indexes(&self) -> &'static [Index] {
        &[
            Index::P2SHAddressIndex,
        ]
    }
}

impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern35<T> {
    fn get(&self, index: Index) -> Option<Endpoint<T>> {
        match index {
            Index::P2SHAddressIndex => Some(self.by.by_p2shaddressindex()),
            _ => None,
        }
    }
}

/// Container for index endpoint methods.
pub struct MetricPattern36By<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: DeserializeOwned> MetricPattern36By<T> {
    pub fn by_p2traddressindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::P2TRAddressIndex)
    }
}

/// Index accessor for metrics with 1 indexes.
pub struct MetricPattern36<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    pub by: MetricPattern36By<T>,
}

impl<T: DeserializeOwned> MetricPattern36<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self {
        let name: Arc<str> = name.into();
        Self {
            client: client.clone(),
            name: name.clone(),
            by: MetricPattern36By {
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

impl<T> AnyMetricPattern for MetricPattern36<T> {
    fn name(&self) -> &str {
        &self.name
    }

    fn indexes(&self) -> &'static [Index] {
        &[
            Index::P2TRAddressIndex,
        ]
    }
}

impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern36<T> {
    fn get(&self, index: Index) -> Option<Endpoint<T>> {
        match index {
            Index::P2TRAddressIndex => Some(self.by.by_p2traddressindex()),
            _ => None,
        }
    }
}

/// Container for index endpoint methods.
pub struct MetricPattern37By<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: DeserializeOwned> MetricPattern37By<T> {
    pub fn by_p2wpkhaddressindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::P2WPKHAddressIndex)
    }
}

/// Index accessor for metrics with 1 indexes.
pub struct MetricPattern37<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    pub by: MetricPattern37By<T>,
}

impl<T: DeserializeOwned> MetricPattern37<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self {
        let name: Arc<str> = name.into();
        Self {
            client: client.clone(),
            name: name.clone(),
            by: MetricPattern37By {
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

impl<T> AnyMetricPattern for MetricPattern37<T> {
    fn name(&self) -> &str {
        &self.name
    }

    fn indexes(&self) -> &'static [Index] {
        &[
            Index::P2WPKHAddressIndex,
        ]
    }
}

impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern37<T> {
    fn get(&self, index: Index) -> Option<Endpoint<T>> {
        match index {
            Index::P2WPKHAddressIndex => Some(self.by.by_p2wpkhaddressindex()),
            _ => None,
        }
    }
}

/// Container for index endpoint methods.
pub struct MetricPattern38By<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: DeserializeOwned> MetricPattern38By<T> {
    pub fn by_p2wshaddressindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::P2WSHAddressIndex)
    }
}

/// Index accessor for metrics with 1 indexes.
pub struct MetricPattern38<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    pub by: MetricPattern38By<T>,
}

impl<T: DeserializeOwned> MetricPattern38<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self {
        let name: Arc<str> = name.into();
        Self {
            client: client.clone(),
            name: name.clone(),
            by: MetricPattern38By {
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

impl<T> AnyMetricPattern for MetricPattern38<T> {
    fn name(&self) -> &str {
        &self.name
    }

    fn indexes(&self) -> &'static [Index] {
        &[
            Index::P2WSHAddressIndex,
        ]
    }
}

impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern38<T> {
    fn get(&self, index: Index) -> Option<Endpoint<T>> {
        match index {
            Index::P2WSHAddressIndex => Some(self.by.by_p2wshaddressindex()),
            _ => None,
        }
    }
}

/// Container for index endpoint methods.
pub struct MetricPattern39By<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: DeserializeOwned> MetricPattern39By<T> {
    pub fn by_quarterindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::QuarterIndex)
    }
}

/// Index accessor for metrics with 1 indexes.
pub struct MetricPattern39<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    pub by: MetricPattern39By<T>,
}

impl<T: DeserializeOwned> MetricPattern39<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self {
        let name: Arc<str> = name.into();
        Self {
            client: client.clone(),
            name: name.clone(),
            by: MetricPattern39By {
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

impl<T> AnyMetricPattern for MetricPattern39<T> {
    fn name(&self) -> &str {
        &self.name
    }

    fn indexes(&self) -> &'static [Index] {
        &[
            Index::QuarterIndex,
        ]
    }
}

impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern39<T> {
    fn get(&self, index: Index) -> Option<Endpoint<T>> {
        match index {
            Index::QuarterIndex => Some(self.by.by_quarterindex()),
            _ => None,
        }
    }
}

/// Container for index endpoint methods.
pub struct MetricPattern40By<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: DeserializeOwned> MetricPattern40By<T> {
    pub fn by_semesterindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::SemesterIndex)
    }
}

/// Index accessor for metrics with 1 indexes.
pub struct MetricPattern40<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    pub by: MetricPattern40By<T>,
}

impl<T: DeserializeOwned> MetricPattern40<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self {
        let name: Arc<str> = name.into();
        Self {
            client: client.clone(),
            name: name.clone(),
            by: MetricPattern40By {
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

impl<T> AnyMetricPattern for MetricPattern40<T> {
    fn name(&self) -> &str {
        &self.name
    }

    fn indexes(&self) -> &'static [Index] {
        &[
            Index::SemesterIndex,
        ]
    }
}

impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern40<T> {
    fn get(&self, index: Index) -> Option<Endpoint<T>> {
        match index {
            Index::SemesterIndex => Some(self.by.by_semesterindex()),
            _ => None,
        }
    }
}

/// Container for index endpoint methods.
pub struct MetricPattern41By<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: DeserializeOwned> MetricPattern41By<T> {
    pub fn by_txindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::TxIndex)
    }
}

/// Index accessor for metrics with 1 indexes.
pub struct MetricPattern41<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    pub by: MetricPattern41By<T>,
}

impl<T: DeserializeOwned> MetricPattern41<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self {
        let name: Arc<str> = name.into();
        Self {
            client: client.clone(),
            name: name.clone(),
            by: MetricPattern41By {
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

impl<T> AnyMetricPattern for MetricPattern41<T> {
    fn name(&self) -> &str {
        &self.name
    }

    fn indexes(&self) -> &'static [Index] {
        &[
            Index::TxIndex,
        ]
    }
}

impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern41<T> {
    fn get(&self, index: Index) -> Option<Endpoint<T>> {
        match index {
            Index::TxIndex => Some(self.by.by_txindex()),
            _ => None,
        }
    }
}

/// Container for index endpoint methods.
pub struct MetricPattern42By<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: DeserializeOwned> MetricPattern42By<T> {
    pub fn by_unknownoutputindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::UnknownOutputIndex)
    }
}

/// Index accessor for metrics with 1 indexes.
pub struct MetricPattern42<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    pub by: MetricPattern42By<T>,
}

impl<T: DeserializeOwned> MetricPattern42<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self {
        let name: Arc<str> = name.into();
        Self {
            client: client.clone(),
            name: name.clone(),
            by: MetricPattern42By {
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

impl<T> AnyMetricPattern for MetricPattern42<T> {
    fn name(&self) -> &str {
        &self.name
    }

    fn indexes(&self) -> &'static [Index] {
        &[
            Index::UnknownOutputIndex,
        ]
    }
}

impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern42<T> {
    fn get(&self, index: Index) -> Option<Endpoint<T>> {
        match index {
            Index::UnknownOutputIndex => Some(self.by.by_unknownoutputindex()),
            _ => None,
        }
    }
}

/// Container for index endpoint methods.
pub struct MetricPattern43By<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: DeserializeOwned> MetricPattern43By<T> {
    pub fn by_weekindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::WeekIndex)
    }
}

/// Index accessor for metrics with 1 indexes.
pub struct MetricPattern43<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    pub by: MetricPattern43By<T>,
}

impl<T: DeserializeOwned> MetricPattern43<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self {
        let name: Arc<str> = name.into();
        Self {
            client: client.clone(),
            name: name.clone(),
            by: MetricPattern43By {
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

impl<T> AnyMetricPattern for MetricPattern43<T> {
    fn name(&self) -> &str {
        &self.name
    }

    fn indexes(&self) -> &'static [Index] {
        &[
            Index::WeekIndex,
        ]
    }
}

impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern43<T> {
    fn get(&self, index: Index) -> Option<Endpoint<T>> {
        match index {
            Index::WeekIndex => Some(self.by.by_weekindex()),
            _ => None,
        }
    }
}

/// Container for index endpoint methods.
pub struct MetricPattern44By<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: DeserializeOwned> MetricPattern44By<T> {
    pub fn by_yearindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::YearIndex)
    }
}

/// Index accessor for metrics with 1 indexes.
pub struct MetricPattern44<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    pub by: MetricPattern44By<T>,
}

impl<T: DeserializeOwned> MetricPattern44<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self {
        let name: Arc<str> = name.into();
        Self {
            client: client.clone(),
            name: name.clone(),
            by: MetricPattern44By {
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

impl<T> AnyMetricPattern for MetricPattern44<T> {
    fn name(&self) -> &str {
        &self.name
    }

    fn indexes(&self) -> &'static [Index] {
        &[
            Index::YearIndex,
        ]
    }
}

impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern44<T> {
    fn get(&self, index: Index) -> Option<Endpoint<T>> {
        match index {
            Index::YearIndex => Some(self.by.by_yearindex()),
            _ => None,
        }
    }
}

/// Container for index endpoint methods.
pub struct MetricPattern45By<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: DeserializeOwned> MetricPattern45By<T> {
    pub fn by_loadedaddressindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::LoadedAddressIndex)
    }
}

/// Index accessor for metrics with 1 indexes.
pub struct MetricPattern45<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    pub by: MetricPattern45By<T>,
}

impl<T: DeserializeOwned> MetricPattern45<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self {
        let name: Arc<str> = name.into();
        Self {
            client: client.clone(),
            name: name.clone(),
            by: MetricPattern45By {
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

impl<T> AnyMetricPattern for MetricPattern45<T> {
    fn name(&self) -> &str {
        &self.name
    }

    fn indexes(&self) -> &'static [Index] {
        &[
            Index::LoadedAddressIndex,
        ]
    }
}

impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern45<T> {
    fn get(&self, index: Index) -> Option<Endpoint<T>> {
        match index {
            Index::LoadedAddressIndex => Some(self.by.by_loadedaddressindex()),
            _ => None,
        }
    }
}

/// Container for index endpoint methods.
pub struct MetricPattern46By<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: DeserializeOwned> MetricPattern46By<T> {
    pub fn by_emptyaddressindex(&self) -> Endpoint<T> {
        Endpoint::new(self.client.clone(), self.name.clone(), Index::EmptyAddressIndex)
    }
}

/// Index accessor for metrics with 1 indexes.
pub struct MetricPattern46<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    pub by: MetricPattern46By<T>,
}

impl<T: DeserializeOwned> MetricPattern46<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self {
        let name: Arc<str> = name.into();
        Self {
            client: client.clone(),
            name: name.clone(),
            by: MetricPattern46By {
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

impl<T> AnyMetricPattern for MetricPattern46<T> {
    fn name(&self) -> &str {
        &self.name
    }

    fn indexes(&self) -> &'static [Index] {
        &[
            Index::EmptyAddressIndex,
        ]
    }
}

impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern46<T> {
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
    pub adjusted_sopr: MetricPattern21<StoredF64>,
    pub adjusted_sopr_30d_ema: MetricPattern21<StoredF64>,
    pub adjusted_sopr_7d_ema: MetricPattern21<StoredF64>,
    pub adjusted_value_created: MetricPattern1<Dollars>,
    pub adjusted_value_destroyed: MetricPattern1<Dollars>,
    pub mvrv: MetricPattern4<StoredF32>,
    pub neg_realized_loss: BlockCountPattern<Dollars>,
    pub net_realized_pnl: BlockCountPattern<Dollars>,
    pub net_realized_pnl_cumulative_30d_delta: MetricPattern4<Dollars>,
    pub net_realized_pnl_cumulative_30d_delta_rel_to_market_cap: MetricPattern4<StoredF32>,
    pub net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap: MetricPattern4<StoredF32>,
    pub net_realized_pnl_rel_to_realized_cap: MetricPattern25<StoredF32>,
    pub realized_cap: MetricPattern1<Dollars>,
    pub realized_cap_30d_delta: MetricPattern4<Dollars>,
    pub realized_cap_rel_to_own_market_cap: MetricPattern1<StoredF32>,
    pub realized_loss: BlockCountPattern<Dollars>,
    pub realized_loss_rel_to_realized_cap: MetricPattern25<StoredF32>,
    pub realized_price: MetricPattern1<Dollars>,
    pub realized_price_extra: ActivePriceRatioPattern,
    pub realized_profit: BlockCountPattern<Dollars>,
    pub realized_profit_rel_to_realized_cap: MetricPattern25<StoredF32>,
    pub realized_profit_to_loss_ratio: MetricPattern21<StoredF64>,
    pub realized_value: MetricPattern1<Dollars>,
    pub sell_side_risk_ratio: MetricPattern21<StoredF32>,
    pub sell_side_risk_ratio_30d_ema: MetricPattern21<StoredF32>,
    pub sell_side_risk_ratio_7d_ema: MetricPattern21<StoredF32>,
    pub sopr: MetricPattern21<StoredF64>,
    pub sopr_30d_ema: MetricPattern21<StoredF64>,
    pub sopr_7d_ema: MetricPattern21<StoredF64>,
    pub total_realized_pnl: TotalRealizedPnlPattern<Dollars>,
    pub value_created: MetricPattern1<Dollars>,
    pub value_destroyed: MetricPattern1<Dollars>,
}

impl RealizedPattern3 {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            adjusted_sopr: MetricPattern21::new(client.clone(), _m(&acc, "adjusted_sopr")),
            adjusted_sopr_30d_ema: MetricPattern21::new(client.clone(), _m(&acc, "adjusted_sopr_30d_ema")),
            adjusted_sopr_7d_ema: MetricPattern21::new(client.clone(), _m(&acc, "adjusted_sopr_7d_ema")),
            adjusted_value_created: MetricPattern1::new(client.clone(), _m(&acc, "adjusted_value_created")),
            adjusted_value_destroyed: MetricPattern1::new(client.clone(), _m(&acc, "adjusted_value_destroyed")),
            mvrv: MetricPattern4::new(client.clone(), _m(&acc, "mvrv")),
            neg_realized_loss: BlockCountPattern::new(client.clone(), _m(&acc, "neg_realized_loss")),
            net_realized_pnl: BlockCountPattern::new(client.clone(), _m(&acc, "net_realized_pnl")),
            net_realized_pnl_cumulative_30d_delta: MetricPattern4::new(client.clone(), _m(&acc, "net_realized_pnl_cumulative_30d_delta")),
            net_realized_pnl_cumulative_30d_delta_rel_to_market_cap: MetricPattern4::new(client.clone(), _m(&acc, "net_realized_pnl_cumulative_30d_delta_rel_to_market_cap")),
            net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap: MetricPattern4::new(client.clone(), _m(&acc, "net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap")),
            net_realized_pnl_rel_to_realized_cap: MetricPattern25::new(client.clone(), _m(&acc, "net_realized_pnl_rel_to_realized_cap")),
            realized_cap: MetricPattern1::new(client.clone(), _m(&acc, "realized_cap")),
            realized_cap_30d_delta: MetricPattern4::new(client.clone(), _m(&acc, "realized_cap_30d_delta")),
            realized_cap_rel_to_own_market_cap: MetricPattern1::new(client.clone(), _m(&acc, "realized_cap_rel_to_own_market_cap")),
            realized_loss: BlockCountPattern::new(client.clone(), _m(&acc, "realized_loss")),
            realized_loss_rel_to_realized_cap: MetricPattern25::new(client.clone(), _m(&acc, "realized_loss_rel_to_realized_cap")),
            realized_price: MetricPattern1::new(client.clone(), _m(&acc, "realized_price")),
            realized_price_extra: ActivePriceRatioPattern::new(client.clone(), _m(&acc, "realized_price_ratio")),
            realized_profit: BlockCountPattern::new(client.clone(), _m(&acc, "realized_profit")),
            realized_profit_rel_to_realized_cap: MetricPattern25::new(client.clone(), _m(&acc, "realized_profit_rel_to_realized_cap")),
            realized_profit_to_loss_ratio: MetricPattern21::new(client.clone(), _m(&acc, "realized_profit_to_loss_ratio")),
            realized_value: MetricPattern1::new(client.clone(), _m(&acc, "realized_value")),
            sell_side_risk_ratio: MetricPattern21::new(client.clone(), _m(&acc, "sell_side_risk_ratio")),
            sell_side_risk_ratio_30d_ema: MetricPattern21::new(client.clone(), _m(&acc, "sell_side_risk_ratio_30d_ema")),
            sell_side_risk_ratio_7d_ema: MetricPattern21::new(client.clone(), _m(&acc, "sell_side_risk_ratio_7d_ema")),
            sopr: MetricPattern21::new(client.clone(), _m(&acc, "sopr")),
            sopr_30d_ema: MetricPattern21::new(client.clone(), _m(&acc, "sopr_30d_ema")),
            sopr_7d_ema: MetricPattern21::new(client.clone(), _m(&acc, "sopr_7d_ema")),
            total_realized_pnl: TotalRealizedPnlPattern::new(client.clone(), _m(&acc, "total_realized_pnl")),
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
    pub net_realized_pnl_rel_to_realized_cap: MetricPattern25<StoredF32>,
    pub realized_cap: MetricPattern1<Dollars>,
    pub realized_cap_30d_delta: MetricPattern4<Dollars>,
    pub realized_cap_rel_to_own_market_cap: MetricPattern1<StoredF32>,
    pub realized_loss: BlockCountPattern<Dollars>,
    pub realized_loss_rel_to_realized_cap: MetricPattern25<StoredF32>,
    pub realized_price: MetricPattern1<Dollars>,
    pub realized_price_extra: ActivePriceRatioPattern,
    pub realized_profit: BlockCountPattern<Dollars>,
    pub realized_profit_rel_to_realized_cap: MetricPattern25<StoredF32>,
    pub realized_profit_to_loss_ratio: MetricPattern21<StoredF64>,
    pub realized_value: MetricPattern1<Dollars>,
    pub sell_side_risk_ratio: MetricPattern21<StoredF32>,
    pub sell_side_risk_ratio_30d_ema: MetricPattern21<StoredF32>,
    pub sell_side_risk_ratio_7d_ema: MetricPattern21<StoredF32>,
    pub sopr: MetricPattern21<StoredF64>,
    pub sopr_30d_ema: MetricPattern21<StoredF64>,
    pub sopr_7d_ema: MetricPattern21<StoredF64>,
    pub total_realized_pnl: TotalRealizedPnlPattern<Dollars>,
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
            net_realized_pnl_rel_to_realized_cap: MetricPattern25::new(client.clone(), _m(&acc, "net_realized_pnl_rel_to_realized_cap")),
            realized_cap: MetricPattern1::new(client.clone(), _m(&acc, "realized_cap")),
            realized_cap_30d_delta: MetricPattern4::new(client.clone(), _m(&acc, "realized_cap_30d_delta")),
            realized_cap_rel_to_own_market_cap: MetricPattern1::new(client.clone(), _m(&acc, "realized_cap_rel_to_own_market_cap")),
            realized_loss: BlockCountPattern::new(client.clone(), _m(&acc, "realized_loss")),
            realized_loss_rel_to_realized_cap: MetricPattern25::new(client.clone(), _m(&acc, "realized_loss_rel_to_realized_cap")),
            realized_price: MetricPattern1::new(client.clone(), _m(&acc, "realized_price")),
            realized_price_extra: ActivePriceRatioPattern::new(client.clone(), _m(&acc, "realized_price_ratio")),
            realized_profit: BlockCountPattern::new(client.clone(), _m(&acc, "realized_profit")),
            realized_profit_rel_to_realized_cap: MetricPattern25::new(client.clone(), _m(&acc, "realized_profit_rel_to_realized_cap")),
            realized_profit_to_loss_ratio: MetricPattern21::new(client.clone(), _m(&acc, "realized_profit_to_loss_ratio")),
            realized_value: MetricPattern1::new(client.clone(), _m(&acc, "realized_value")),
            sell_side_risk_ratio: MetricPattern21::new(client.clone(), _m(&acc, "sell_side_risk_ratio")),
            sell_side_risk_ratio_30d_ema: MetricPattern21::new(client.clone(), _m(&acc, "sell_side_risk_ratio_30d_ema")),
            sell_side_risk_ratio_7d_ema: MetricPattern21::new(client.clone(), _m(&acc, "sell_side_risk_ratio_7d_ema")),
            sopr: MetricPattern21::new(client.clone(), _m(&acc, "sopr")),
            sopr_30d_ema: MetricPattern21::new(client.clone(), _m(&acc, "sopr_30d_ema")),
            sopr_7d_ema: MetricPattern21::new(client.clone(), _m(&acc, "sopr_7d_ema")),
            total_realized_pnl: TotalRealizedPnlPattern::new(client.clone(), _m(&acc, "total_realized_pnl")),
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
    pub net_realized_pnl_rel_to_realized_cap: MetricPattern25<StoredF32>,
    pub realized_cap: MetricPattern1<Dollars>,
    pub realized_cap_30d_delta: MetricPattern4<Dollars>,
    pub realized_loss: BlockCountPattern<Dollars>,
    pub realized_loss_rel_to_realized_cap: MetricPattern25<StoredF32>,
    pub realized_price: MetricPattern1<Dollars>,
    pub realized_price_extra: RealizedPriceExtraPattern,
    pub realized_profit: BlockCountPattern<Dollars>,
    pub realized_profit_rel_to_realized_cap: MetricPattern25<StoredF32>,
    pub realized_value: MetricPattern1<Dollars>,
    pub sell_side_risk_ratio: MetricPattern21<StoredF32>,
    pub sell_side_risk_ratio_30d_ema: MetricPattern21<StoredF32>,
    pub sell_side_risk_ratio_7d_ema: MetricPattern21<StoredF32>,
    pub sopr: MetricPattern21<StoredF64>,
    pub sopr_30d_ema: MetricPattern21<StoredF64>,
    pub sopr_7d_ema: MetricPattern21<StoredF64>,
    pub total_realized_pnl: TotalRealizedPnlPattern<Dollars>,
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
            net_realized_pnl_rel_to_realized_cap: MetricPattern25::new(client.clone(), _m(&acc, "net_realized_pnl_rel_to_realized_cap")),
            realized_cap: MetricPattern1::new(client.clone(), _m(&acc, "realized_cap")),
            realized_cap_30d_delta: MetricPattern4::new(client.clone(), _m(&acc, "realized_cap_30d_delta")),
            realized_loss: BlockCountPattern::new(client.clone(), _m(&acc, "realized_loss")),
            realized_loss_rel_to_realized_cap: MetricPattern25::new(client.clone(), _m(&acc, "realized_loss_rel_to_realized_cap")),
            realized_price: MetricPattern1::new(client.clone(), _m(&acc, "realized_price")),
            realized_price_extra: RealizedPriceExtraPattern::new(client.clone(), _m(&acc, "realized_price")),
            realized_profit: BlockCountPattern::new(client.clone(), _m(&acc, "realized_profit")),
            realized_profit_rel_to_realized_cap: MetricPattern25::new(client.clone(), _m(&acc, "realized_profit_rel_to_realized_cap")),
            realized_value: MetricPattern1::new(client.clone(), _m(&acc, "realized_value")),
            sell_side_risk_ratio: MetricPattern21::new(client.clone(), _m(&acc, "sell_side_risk_ratio")),
            sell_side_risk_ratio_30d_ema: MetricPattern21::new(client.clone(), _m(&acc, "sell_side_risk_ratio_30d_ema")),
            sell_side_risk_ratio_7d_ema: MetricPattern21::new(client.clone(), _m(&acc, "sell_side_risk_ratio_7d_ema")),
            sopr: MetricPattern21::new(client.clone(), _m(&acc, "sopr")),
            sopr_30d_ema: MetricPattern21::new(client.clone(), _m(&acc, "sopr_30d_ema")),
            sopr_7d_ema: MetricPattern21::new(client.clone(), _m(&acc, "sopr_7d_ema")),
            total_realized_pnl: TotalRealizedPnlPattern::new(client.clone(), _m(&acc, "total_realized_pnl")),
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
pub struct PercentilesPattern {
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

impl PercentilesPattern {
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
pub struct RelativePattern2 {
    pub neg_unrealized_loss_rel_to_market_cap: MetricPattern5<StoredF32>,
    pub neg_unrealized_loss_rel_to_own_market_cap: MetricPattern5<StoredF32>,
    pub neg_unrealized_loss_rel_to_own_total_unrealized_pnl: MetricPattern5<StoredF32>,
    pub net_unrealized_pnl_rel_to_market_cap: MetricPattern3<StoredF32>,
    pub net_unrealized_pnl_rel_to_own_market_cap: MetricPattern3<StoredF32>,
    pub net_unrealized_pnl_rel_to_own_total_unrealized_pnl: MetricPattern3<StoredF32>,
    pub nupl: MetricPattern4<StoredF32>,
    pub supply_in_loss_rel_to_circulating_supply: MetricPattern5<StoredF64>,
    pub supply_in_loss_rel_to_own_supply: MetricPattern5<StoredF64>,
    pub supply_in_profit_rel_to_circulating_supply: MetricPattern5<StoredF64>,
    pub supply_in_profit_rel_to_own_supply: MetricPattern5<StoredF64>,
    pub supply_rel_to_circulating_supply: MetricPattern4<StoredF64>,
    pub unrealized_loss_rel_to_market_cap: MetricPattern5<StoredF32>,
    pub unrealized_loss_rel_to_own_market_cap: MetricPattern5<StoredF32>,
    pub unrealized_loss_rel_to_own_total_unrealized_pnl: MetricPattern5<StoredF32>,
    pub unrealized_profit_rel_to_market_cap: MetricPattern5<StoredF32>,
    pub unrealized_profit_rel_to_own_market_cap: MetricPattern5<StoredF32>,
    pub unrealized_profit_rel_to_own_total_unrealized_pnl: MetricPattern5<StoredF32>,
}

impl RelativePattern2 {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            neg_unrealized_loss_rel_to_market_cap: MetricPattern5::new(client.clone(), _m(&acc, "neg_unrealized_loss_rel_to_market_cap")),
            neg_unrealized_loss_rel_to_own_market_cap: MetricPattern5::new(client.clone(), _m(&acc, "neg_unrealized_loss_rel_to_own_market_cap")),
            neg_unrealized_loss_rel_to_own_total_unrealized_pnl: MetricPattern5::new(client.clone(), _m(&acc, "neg_unrealized_loss_rel_to_own_total_unrealized_pnl")),
            net_unrealized_pnl_rel_to_market_cap: MetricPattern3::new(client.clone(), _m(&acc, "net_unrealized_pnl_rel_to_market_cap")),
            net_unrealized_pnl_rel_to_own_market_cap: MetricPattern3::new(client.clone(), _m(&acc, "net_unrealized_pnl_rel_to_own_market_cap")),
            net_unrealized_pnl_rel_to_own_total_unrealized_pnl: MetricPattern3::new(client.clone(), _m(&acc, "net_unrealized_pnl_rel_to_own_total_unrealized_pnl")),
            nupl: MetricPattern4::new(client.clone(), _m(&acc, "nupl")),
            supply_in_loss_rel_to_circulating_supply: MetricPattern5::new(client.clone(), _m(&acc, "supply_in_loss_rel_to_circulating_supply")),
            supply_in_loss_rel_to_own_supply: MetricPattern5::new(client.clone(), _m(&acc, "supply_in_loss_rel_to_own_supply")),
            supply_in_profit_rel_to_circulating_supply: MetricPattern5::new(client.clone(), _m(&acc, "supply_in_profit_rel_to_circulating_supply")),
            supply_in_profit_rel_to_own_supply: MetricPattern5::new(client.clone(), _m(&acc, "supply_in_profit_rel_to_own_supply")),
            supply_rel_to_circulating_supply: MetricPattern4::new(client.clone(), _m(&acc, "supply_rel_to_circulating_supply")),
            unrealized_loss_rel_to_market_cap: MetricPattern5::new(client.clone(), _m(&acc, "unrealized_loss_rel_to_market_cap")),
            unrealized_loss_rel_to_own_market_cap: MetricPattern5::new(client.clone(), _m(&acc, "unrealized_loss_rel_to_own_market_cap")),
            unrealized_loss_rel_to_own_total_unrealized_pnl: MetricPattern5::new(client.clone(), _m(&acc, "unrealized_loss_rel_to_own_total_unrealized_pnl")),
            unrealized_profit_rel_to_market_cap: MetricPattern5::new(client.clone(), _m(&acc, "unrealized_profit_rel_to_market_cap")),
            unrealized_profit_rel_to_own_market_cap: MetricPattern5::new(client.clone(), _m(&acc, "unrealized_profit_rel_to_own_market_cap")),
            unrealized_profit_rel_to_own_total_unrealized_pnl: MetricPattern5::new(client.clone(), _m(&acc, "unrealized_profit_rel_to_own_total_unrealized_pnl")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct AXbtPattern {
    pub _1d_dominance: BlockCountPattern<StoredF32>,
    pub _1m_blocks_mined: MetricPattern4<StoredU32>,
    pub _1m_dominance: MetricPattern4<StoredF32>,
    pub _1w_blocks_mined: MetricPattern4<StoredU32>,
    pub _1w_dominance: MetricPattern4<StoredF32>,
    pub _1y_blocks_mined: MetricPattern4<StoredU32>,
    pub _1y_dominance: MetricPattern4<StoredF32>,
    pub blocks_mined: BlockCountPattern<StoredU32>,
    pub coinbase: UnclaimedRewardsPattern,
    pub days_since_block: MetricPattern4<StoredU16>,
    pub dominance: BlockCountPattern<StoredF32>,
    pub fee: SentPattern,
    pub subsidy: SentPattern,
}

impl AXbtPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            _1d_dominance: BlockCountPattern::new(client.clone(), _m(&acc, "1d_dominance")),
            _1m_blocks_mined: MetricPattern4::new(client.clone(), _m(&acc, "1m_blocks_mined")),
            _1m_dominance: MetricPattern4::new(client.clone(), _m(&acc, "1m_dominance")),
            _1w_blocks_mined: MetricPattern4::new(client.clone(), _m(&acc, "1w_blocks_mined")),
            _1w_dominance: MetricPattern4::new(client.clone(), _m(&acc, "1w_dominance")),
            _1y_blocks_mined: MetricPattern4::new(client.clone(), _m(&acc, "1y_blocks_mined")),
            _1y_dominance: MetricPattern4::new(client.clone(), _m(&acc, "1y_dominance")),
            blocks_mined: BlockCountPattern::new(client.clone(), _m(&acc, "blocks_mined")),
            coinbase: UnclaimedRewardsPattern::new(client.clone(), _m(&acc, "coinbase")),
            days_since_block: MetricPattern4::new(client.clone(), _m(&acc, "days_since_block")),
            dominance: BlockCountPattern::new(client.clone(), _m(&acc, "dominance")),
            fee: SentPattern::new(client.clone(), _m(&acc, "fee")),
            subsidy: SentPattern::new(client.clone(), _m(&acc, "subsidy")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct BitcoinPattern<T> {
    pub average: MetricPattern2<T>,
    pub base: MetricPattern25<T>,
    pub cumulative: MetricPattern1<T>,
    pub max: MetricPattern2<T>,
    pub median: MetricPattern21<T>,
    pub min: MetricPattern2<T>,
    pub pct10: MetricPattern21<T>,
    pub pct25: MetricPattern21<T>,
    pub pct75: MetricPattern21<T>,
    pub pct90: MetricPattern21<T>,
    pub sum: MetricPattern2<T>,
}

impl<T: DeserializeOwned> BitcoinPattern<T> {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            average: MetricPattern2::new(client.clone(), _m(&acc, "avg")),
            base: MetricPattern25::new(client.clone(), acc.clone()),
            cumulative: MetricPattern1::new(client.clone(), _m(&acc, "cumulative")),
            max: MetricPattern2::new(client.clone(), _m(&acc, "max")),
            median: MetricPattern21::new(client.clone(), _m(&acc, "median")),
            min: MetricPattern2::new(client.clone(), _m(&acc, "min")),
            pct10: MetricPattern21::new(client.clone(), _m(&acc, "pct10")),
            pct25: MetricPattern21::new(client.clone(), _m(&acc, "pct25")),
            pct75: MetricPattern21::new(client.clone(), _m(&acc, "pct75")),
            pct90: MetricPattern21::new(client.clone(), _m(&acc, "pct90")),
            sum: MetricPattern2::new(client.clone(), _m(&acc, "sum")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct RelativePattern {
    pub neg_unrealized_loss_rel_to_market_cap: MetricPattern5<StoredF32>,
    pub net_unrealized_pnl_rel_to_market_cap: MetricPattern3<StoredF32>,
    pub nupl: MetricPattern4<StoredF32>,
    pub supply_in_loss_rel_to_circulating_supply: MetricPattern5<StoredF64>,
    pub supply_in_loss_rel_to_own_supply: MetricPattern5<StoredF64>,
    pub supply_in_profit_rel_to_circulating_supply: MetricPattern5<StoredF64>,
    pub supply_in_profit_rel_to_own_supply: MetricPattern5<StoredF64>,
    pub supply_rel_to_circulating_supply: MetricPattern4<StoredF64>,
    pub unrealized_loss_rel_to_market_cap: MetricPattern5<StoredF32>,
    pub unrealized_profit_rel_to_market_cap: MetricPattern5<StoredF32>,
}

impl RelativePattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            neg_unrealized_loss_rel_to_market_cap: MetricPattern5::new(client.clone(), _m(&acc, "neg_unrealized_loss_rel_to_market_cap")),
            net_unrealized_pnl_rel_to_market_cap: MetricPattern3::new(client.clone(), _m(&acc, "net_unrealized_pnl_rel_to_market_cap")),
            nupl: MetricPattern4::new(client.clone(), _m(&acc, "nupl")),
            supply_in_loss_rel_to_circulating_supply: MetricPattern5::new(client.clone(), _m(&acc, "supply_in_loss_rel_to_circulating_supply")),
            supply_in_loss_rel_to_own_supply: MetricPattern5::new(client.clone(), _m(&acc, "supply_in_loss_rel_to_own_supply")),
            supply_in_profit_rel_to_circulating_supply: MetricPattern5::new(client.clone(), _m(&acc, "supply_in_profit_rel_to_circulating_supply")),
            supply_in_profit_rel_to_own_supply: MetricPattern5::new(client.clone(), _m(&acc, "supply_in_profit_rel_to_own_supply")),
            supply_rel_to_circulating_supply: MetricPattern4::new(client.clone(), _m(&acc, "supply_rel_to_circulating_supply")),
            unrealized_loss_rel_to_market_cap: MetricPattern5::new(client.clone(), _m(&acc, "unrealized_loss_rel_to_market_cap")),
            unrealized_profit_rel_to_market_cap: MetricPattern5::new(client.clone(), _m(&acc, "unrealized_profit_rel_to_market_cap")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct BlockSizePattern<T> {
    pub average: MetricPattern1<T>,
    pub cumulative: MetricPattern1<T>,
    pub max: MetricPattern1<T>,
    pub median: MetricPattern25<T>,
    pub min: MetricPattern1<T>,
    pub pct10: MetricPattern25<T>,
    pub pct25: MetricPattern25<T>,
    pub pct75: MetricPattern25<T>,
    pub pct90: MetricPattern25<T>,
    pub sum: MetricPattern1<T>,
}

impl<T: DeserializeOwned> BlockSizePattern<T> {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            average: MetricPattern1::new(client.clone(), _m(&acc, "avg")),
            cumulative: MetricPattern1::new(client.clone(), _m(&acc, "cumulative")),
            max: MetricPattern1::new(client.clone(), _m(&acc, "max")),
            median: MetricPattern25::new(client.clone(), _m(&acc, "median")),
            min: MetricPattern1::new(client.clone(), _m(&acc, "min")),
            pct10: MetricPattern25::new(client.clone(), _m(&acc, "pct10")),
            pct25: MetricPattern25::new(client.clone(), _m(&acc, "pct25")),
            pct75: MetricPattern25::new(client.clone(), _m(&acc, "pct75")),
            pct90: MetricPattern25::new(client.clone(), _m(&acc, "pct90")),
            sum: MetricPattern1::new(client.clone(), _m(&acc, "sum")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct UnrealizedPattern {
    pub neg_unrealized_loss: MetricPattern3<Dollars>,
    pub net_unrealized_pnl: MetricPattern3<Dollars>,
    pub supply_in_loss: SupplyPattern2,
    pub supply_in_loss_value: SupplyValuePattern,
    pub supply_in_profit: SupplyPattern2,
    pub supply_in_profit_value: SupplyValuePattern,
    pub total_unrealized_pnl: MetricPattern3<Dollars>,
    pub unrealized_loss: MetricPattern3<Dollars>,
    pub unrealized_profit: MetricPattern3<Dollars>,
}

impl UnrealizedPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            neg_unrealized_loss: MetricPattern3::new(client.clone(), _m(&acc, "neg_unrealized_loss")),
            net_unrealized_pnl: MetricPattern3::new(client.clone(), _m(&acc, "net_unrealized_pnl")),
            supply_in_loss: SupplyPattern2::new(client.clone(), _m(&acc, "supply_in_loss")),
            supply_in_loss_value: SupplyValuePattern::new(client.clone(), _m(&acc, "supply_in_loss")),
            supply_in_profit: SupplyPattern2::new(client.clone(), _m(&acc, "supply_in_profit")),
            supply_in_profit_value: SupplyValuePattern::new(client.clone(), _m(&acc, "supply_in_profit")),
            total_unrealized_pnl: MetricPattern3::new(client.clone(), _m(&acc, "total_unrealized_pnl")),
            unrealized_loss: MetricPattern3::new(client.clone(), _m(&acc, "unrealized_loss")),
            unrealized_profit: MetricPattern3::new(client.clone(), _m(&acc, "unrealized_profit")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct Constant0Pattern<T> {
    pub dateindex: MetricPattern21<T>,
    pub decadeindex: MetricPattern22<T>,
    pub height: MetricPattern25<T>,
    pub monthindex: MetricPattern27<T>,
    pub quarterindex: MetricPattern39<T>,
    pub semesterindex: MetricPattern40<T>,
    pub weekindex: MetricPattern43<T>,
    pub yearindex: MetricPattern44<T>,
}

impl<T: DeserializeOwned> Constant0Pattern<T> {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            dateindex: MetricPattern21::new(client.clone(), acc.clone()),
            decadeindex: MetricPattern22::new(client.clone(), acc.clone()),
            height: MetricPattern25::new(client.clone(), acc.clone()),
            monthindex: MetricPattern27::new(client.clone(), acc.clone()),
            quarterindex: MetricPattern39::new(client.clone(), acc.clone()),
            semesterindex: MetricPattern40::new(client.clone(), acc.clone()),
            weekindex: MetricPattern43::new(client.clone(), acc.clone()),
            yearindex: MetricPattern44::new(client.clone(), acc.clone()),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct AddresstypeToHeightToAddrCountPattern<T> {
    pub p2a: MetricPattern25<T>,
    pub p2pk33: MetricPattern25<T>,
    pub p2pk65: MetricPattern25<T>,
    pub p2pkh: MetricPattern25<T>,
    pub p2sh: MetricPattern25<T>,
    pub p2tr: MetricPattern25<T>,
    pub p2wpkh: MetricPattern25<T>,
    pub p2wsh: MetricPattern25<T>,
}

impl<T: DeserializeOwned> AddresstypeToHeightToAddrCountPattern<T> {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            p2a: MetricPattern25::new(client.clone(), if acc.is_empty() { "p2a".to_string() } else { format!("p2a_{acc}") }),
            p2pk33: MetricPattern25::new(client.clone(), if acc.is_empty() { "p2pk33".to_string() } else { format!("p2pk33_{acc}") }),
            p2pk65: MetricPattern25::new(client.clone(), if acc.is_empty() { "p2pk65".to_string() } else { format!("p2pk65_{acc}") }),
            p2pkh: MetricPattern25::new(client.clone(), if acc.is_empty() { "p2pkh".to_string() } else { format!("p2pkh_{acc}") }),
            p2sh: MetricPattern25::new(client.clone(), if acc.is_empty() { "p2sh".to_string() } else { format!("p2sh_{acc}") }),
            p2tr: MetricPattern25::new(client.clone(), if acc.is_empty() { "p2tr".to_string() } else { format!("p2tr_{acc}") }),
            p2wpkh: MetricPattern25::new(client.clone(), if acc.is_empty() { "p2wpkh".to_string() } else { format!("p2wpkh_{acc}") }),
            p2wsh: MetricPattern25::new(client.clone(), if acc.is_empty() { "p2wsh".to_string() } else { format!("p2wsh_{acc}") }),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct BlockIntervalPattern<T> {
    pub average: MetricPattern1<T>,
    pub max: MetricPattern1<T>,
    pub median: MetricPattern25<T>,
    pub min: MetricPattern1<T>,
    pub pct10: MetricPattern25<T>,
    pub pct25: MetricPattern25<T>,
    pub pct75: MetricPattern25<T>,
    pub pct90: MetricPattern25<T>,
}

impl<T: DeserializeOwned> BlockIntervalPattern<T> {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            average: MetricPattern1::new(client.clone(), _m(&acc, "avg")),
            max: MetricPattern1::new(client.clone(), _m(&acc, "max")),
            median: MetricPattern25::new(client.clone(), _m(&acc, "median")),
            min: MetricPattern1::new(client.clone(), _m(&acc, "min")),
            pct10: MetricPattern25::new(client.clone(), _m(&acc, "pct10")),
            pct25: MetricPattern25::new(client.clone(), _m(&acc, "pct25")),
            pct75: MetricPattern25::new(client.clone(), _m(&acc, "pct75")),
            pct90: MetricPattern25::new(client.clone(), _m(&acc, "pct90")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct _0satsPattern {
    pub activity: ActivityPattern2,
    pub addr_count: MetricPattern1<StoredU64>,
    pub cost_basis: CostBasisPattern,
    pub realized: RealizedPattern,
    pub relative: RelativePattern,
    pub supply: SupplyPattern3,
    pub unrealized: UnrealizedPattern,
}

impl _0satsPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            activity: ActivityPattern2::new(client.clone(), acc.clone()),
            addr_count: MetricPattern1::new(client.clone(), _m(&acc, "addr_count")),
            cost_basis: CostBasisPattern::new(client.clone(), acc.clone()),
            realized: RealizedPattern::new(client.clone(), acc.clone()),
            relative: RelativePattern::new(client.clone(), acc.clone()),
            supply: SupplyPattern3::new(client.clone(), acc.clone()),
            unrealized: UnrealizedPattern::new(client.clone(), acc.clone()),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct _0satsPattern2 {
    pub activity: ActivityPattern2,
    pub cost_basis: CostBasisPattern,
    pub realized: RealizedPattern,
    pub relative: RelativePattern,
    pub supply: SupplyPattern3,
    pub unrealized: UnrealizedPattern,
}

impl _0satsPattern2 {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            activity: ActivityPattern2::new(client.clone(), acc.clone()),
            cost_basis: CostBasisPattern::new(client.clone(), acc.clone()),
            realized: RealizedPattern::new(client.clone(), acc.clone()),
            relative: RelativePattern::new(client.clone(), acc.clone()),
            supply: SupplyPattern3::new(client.clone(), acc.clone()),
            unrealized: UnrealizedPattern::new(client.clone(), acc.clone()),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct _10yTo12yPattern {
    pub activity: ActivityPattern2,
    pub cost_basis: CostBasisPattern2,
    pub realized: RealizedPattern2,
    pub relative: RelativePattern2,
    pub supply: SupplyPattern3,
    pub unrealized: UnrealizedPattern,
}

impl _10yTo12yPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            activity: ActivityPattern2::new(client.clone(), acc.clone()),
            cost_basis: CostBasisPattern2::new(client.clone(), acc.clone()),
            realized: RealizedPattern2::new(client.clone(), acc.clone()),
            relative: RelativePattern2::new(client.clone(), acc.clone()),
            supply: SupplyPattern3::new(client.clone(), acc.clone()),
            unrealized: UnrealizedPattern::new(client.clone(), acc.clone()),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct UpTo1dPattern {
    pub activity: ActivityPattern2,
    pub cost_basis: CostBasisPattern2,
    pub realized: RealizedPattern3,
    pub relative: RelativePattern2,
    pub supply: SupplyPattern3,
    pub unrealized: UnrealizedPattern,
}

impl UpTo1dPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            activity: ActivityPattern2::new(client.clone(), acc.clone()),
            cost_basis: CostBasisPattern2::new(client.clone(), acc.clone()),
            realized: RealizedPattern3::new(client.clone(), acc.clone()),
            relative: RelativePattern2::new(client.clone(), acc.clone()),
            supply: SupplyPattern3::new(client.clone(), acc.clone()),
            unrealized: UnrealizedPattern::new(client.clone(), acc.clone()),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct SegwitAdoptionPattern<T> {
    pub average: MetricPattern2<T>,
    pub base: MetricPattern25<T>,
    pub cumulative: MetricPattern1<T>,
    pub max: MetricPattern2<T>,
    pub min: MetricPattern2<T>,
    pub sum: MetricPattern2<T>,
}

impl<T: DeserializeOwned> SegwitAdoptionPattern<T> {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            average: MetricPattern2::new(client.clone(), _m(&acc, "avg")),
            base: MetricPattern25::new(client.clone(), acc.clone()),
            cumulative: MetricPattern1::new(client.clone(), _m(&acc, "cumulative")),
            max: MetricPattern2::new(client.clone(), _m(&acc, "max")),
            min: MetricPattern2::new(client.clone(), _m(&acc, "min")),
            sum: MetricPattern2::new(client.clone(), _m(&acc, "sum")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct ActivityPattern2 {
    pub coinblocks_destroyed: BlockCountPattern<StoredF64>,
    pub coindays_destroyed: BlockCountPattern<StoredF64>,
    pub satblocks_destroyed: MetricPattern25<Sats>,
    pub satdays_destroyed: MetricPattern25<Sats>,
    pub sent: SentPattern,
}

impl ActivityPattern2 {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            coinblocks_destroyed: BlockCountPattern::new(client.clone(), _m(&acc, "coinblocks_destroyed")),
            coindays_destroyed: BlockCountPattern::new(client.clone(), _m(&acc, "coindays_destroyed")),
            satblocks_destroyed: MetricPattern25::new(client.clone(), _m(&acc, "satblocks_destroyed")),
            satdays_destroyed: MetricPattern25::new(client.clone(), _m(&acc, "satdays_destroyed")),
            sent: SentPattern::new(client.clone(), _m(&acc, "sent")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct SupplyPattern3 {
    pub supply: SupplyPattern2,
    pub supply_half: ActiveSupplyPattern,
    pub supply_half_value: ActiveSupplyPattern,
    pub supply_value: SupplyValuePattern,
    pub utxo_count: MetricPattern1<StoredU64>,
}

impl SupplyPattern3 {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            supply: SupplyPattern2::new(client.clone(), _m(&acc, "supply")),
            supply_half: ActiveSupplyPattern::new(client.clone(), _m(&acc, "supply_half")),
            supply_half_value: ActiveSupplyPattern::new(client.clone(), _m(&acc, "supply_half")),
            supply_value: SupplyValuePattern::new(client.clone(), _m(&acc, "supply")),
            utxo_count: MetricPattern1::new(client.clone(), _m(&acc, "utxo_count")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct SentPattern {
    pub base: MetricPattern25<Sats>,
    pub bitcoin: BlockCountPattern<Bitcoin>,
    pub dollars: BlockCountPattern<Dollars>,
    pub sats: SatsPattern,
}

impl SentPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            base: MetricPattern25::new(client.clone(), acc.clone()),
            bitcoin: BlockCountPattern::new(client.clone(), _m(&acc, "btc")),
            dollars: BlockCountPattern::new(client.clone(), _m(&acc, "usd")),
            sats: SatsPattern::new(client.clone(), acc.clone()),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct OpreturnPattern {
    pub base: MetricPattern25<Sats>,
    pub bitcoin: BitcoinPattern2<Bitcoin>,
    pub dollars: BitcoinPattern2<Dollars>,
    pub sats: SatsPattern4,
}

impl OpreturnPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            base: MetricPattern25::new(client.clone(), acc.clone()),
            bitcoin: BitcoinPattern2::new(client.clone(), _m(&acc, "btc")),
            dollars: BitcoinPattern2::new(client.clone(), _m(&acc, "usd")),
            sats: SatsPattern4::new(client.clone(), acc.clone()),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct SupplyPattern2 {
    pub base: MetricPattern25<Sats>,
    pub bitcoin: MetricPattern4<Bitcoin>,
    pub dollars: MetricPattern4<Dollars>,
    pub sats: MetricPattern4<Sats>,
}

impl SupplyPattern2 {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            base: MetricPattern25::new(client.clone(), acc.clone()),
            bitcoin: MetricPattern4::new(client.clone(), _m(&acc, "btc")),
            dollars: MetricPattern4::new(client.clone(), _m(&acc, "usd")),
            sats: MetricPattern4::new(client.clone(), acc.clone()),
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
pub struct ActiveSupplyPattern {
    pub bitcoin: MetricPattern1<Bitcoin>,
    pub dollars: MetricPattern1<Dollars>,
    pub sats: MetricPattern1<Sats>,
}

impl ActiveSupplyPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            bitcoin: MetricPattern1::new(client.clone(), _m(&acc, "btc")),
            dollars: MetricPattern1::new(client.clone(), _m(&acc, "usd")),
            sats: MetricPattern1::new(client.clone(), acc.clone()),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct CostBasisPattern2 {
    pub max_cost_basis: MetricPattern1<Dollars>,
    pub min_cost_basis: MetricPattern1<Dollars>,
    pub percentiles: PercentilesPattern,
}

impl CostBasisPattern2 {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            max_cost_basis: MetricPattern1::new(client.clone(), _m(&acc, "max_cost_basis")),
            min_cost_basis: MetricPattern1::new(client.clone(), _m(&acc, "min_cost_basis")),
            percentiles: PercentilesPattern::new(client.clone(), _m(&acc, "cost_basis")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct BlockCountPattern<T> {
    pub base: MetricPattern25<T>,
    pub cumulative: MetricPattern1<T>,
    pub sum: MetricPattern2<T>,
}

impl<T: DeserializeOwned> BlockCountPattern<T> {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            base: MetricPattern25::new(client.clone(), acc.clone()),
            cumulative: MetricPattern1::new(client.clone(), _m(&acc, "cumulative")),
            sum: MetricPattern2::new(client.clone(), _m(&acc, "sum")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct BitcoinPattern2<T> {
    pub base: MetricPattern25<T>,
    pub cumulative: MetricPattern1<T>,
    pub last: MetricPattern2<T>,
}

impl<T: DeserializeOwned> BitcoinPattern2<T> {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            base: MetricPattern25::new(client.clone(), acc.clone()),
            cumulative: MetricPattern1::new(client.clone(), _m(&acc, "cumulative")),
            last: MetricPattern2::new(client.clone(), acc.clone()),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct SatsPattern4 {
    pub cumulative: MetricPattern1<Sats>,
    pub last: MetricPattern2<Sats>,
}

impl SatsPattern4 {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            cumulative: MetricPattern1::new(client.clone(), _m(&acc, "cumulative")),
            last: MetricPattern2::new(client.clone(), acc.clone()),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct CostBasisPattern {
    pub max_cost_basis: MetricPattern1<Dollars>,
    pub min_cost_basis: MetricPattern1<Dollars>,
}

impl CostBasisPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            max_cost_basis: MetricPattern1::new(client.clone(), _m(&acc, "max_cost_basis")),
            min_cost_basis: MetricPattern1::new(client.clone(), _m(&acc, "min_cost_basis")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct SatsPattern {
    pub cumulative: MetricPattern1<Sats>,
    pub sum: MetricPattern2<Sats>,
}

impl SatsPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            cumulative: MetricPattern1::new(client.clone(), _m(&acc, "cumulative")),
            sum: MetricPattern2::new(client.clone(), acc.clone()),
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
pub struct SupplyValuePattern {
    pub bitcoin: MetricPattern25<Bitcoin>,
    pub dollars: MetricPattern25<Dollars>,
}

impl SupplyValuePattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            bitcoin: MetricPattern25::new(client.clone(), _m(&acc, "btc")),
            dollars: MetricPattern25::new(client.clone(), _m(&acc, "usd")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct TotalRealizedPnlPattern<T> {
    pub base: MetricPattern25<T>,
    pub sum: MetricPattern2<T>,
}

impl<T: DeserializeOwned> TotalRealizedPnlPattern<T> {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            base: MetricPattern25::new(client.clone(), acc.clone()),
            sum: MetricPattern2::new(client.clone(), _m(&acc, "sum")),
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

// Catalog tree

/// Catalog tree node.
pub struct CatalogTree {
    pub computed: CatalogTree_Computed,
    pub indexed: CatalogTree_Indexed,
}

impl CatalogTree {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            computed: CatalogTree_Computed::new(client.clone(), format!("{base_path}_computed")),
            indexed: CatalogTree_Indexed::new(client.clone(), format!("{base_path}_indexed")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed {
    pub blocks: CatalogTree_Computed_Blocks,
    pub cointime: CatalogTree_Computed_Cointime,
    pub constants: CatalogTree_Computed_Constants,
    pub distribution: CatalogTree_Computed_Distribution,
    pub indexes: CatalogTree_Computed_Indexes,
    pub inputs: CatalogTree_Computed_Inputs,
    pub market: CatalogTree_Computed_Market,
    pub outputs: CatalogTree_Computed_Outputs,
    pub pools: CatalogTree_Computed_Pools,
    pub positions: CatalogTree_Computed_Positions,
    pub price: CatalogTree_Computed_Price,
    pub scripts: CatalogTree_Computed_Scripts,
    pub supply: CatalogTree_Computed_Supply,
    pub transactions: CatalogTree_Computed_Transactions,
}

impl CatalogTree_Computed {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            blocks: CatalogTree_Computed_Blocks::new(client.clone(), format!("{base_path}_blocks")),
            cointime: CatalogTree_Computed_Cointime::new(client.clone(), format!("{base_path}_cointime")),
            constants: CatalogTree_Computed_Constants::new(client.clone(), format!("{base_path}_constants")),
            distribution: CatalogTree_Computed_Distribution::new(client.clone(), format!("{base_path}_distribution")),
            indexes: CatalogTree_Computed_Indexes::new(client.clone(), format!("{base_path}_indexes")),
            inputs: CatalogTree_Computed_Inputs::new(client.clone(), format!("{base_path}_inputs")),
            market: CatalogTree_Computed_Market::new(client.clone(), format!("{base_path}_market")),
            outputs: CatalogTree_Computed_Outputs::new(client.clone(), format!("{base_path}_outputs")),
            pools: CatalogTree_Computed_Pools::new(client.clone(), format!("{base_path}_pools")),
            positions: CatalogTree_Computed_Positions::new(client.clone(), format!("{base_path}_positions")),
            price: CatalogTree_Computed_Price::new(client.clone(), format!("{base_path}_price")),
            scripts: CatalogTree_Computed_Scripts::new(client.clone(), format!("{base_path}_scripts")),
            supply: CatalogTree_Computed_Supply::new(client.clone(), format!("{base_path}_supply")),
            transactions: CatalogTree_Computed_Transactions::new(client.clone(), format!("{base_path}_transactions")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Blocks {
    pub count: CatalogTree_Computed_Blocks_Count,
    pub difficulty: CatalogTree_Computed_Blocks_Difficulty,
    pub halving: CatalogTree_Computed_Blocks_Halving,
    pub interval: CatalogTree_Computed_Blocks_Interval,
    pub mining: CatalogTree_Computed_Blocks_Mining,
    pub rewards: CatalogTree_Computed_Blocks_Rewards,
    pub size: CatalogTree_Computed_Blocks_Size,
    pub time: CatalogTree_Computed_Blocks_Time,
    pub weight: CatalogTree_Computed_Blocks_Weight,
}

impl CatalogTree_Computed_Blocks {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            count: CatalogTree_Computed_Blocks_Count::new(client.clone(), format!("{base_path}_count")),
            difficulty: CatalogTree_Computed_Blocks_Difficulty::new(client.clone(), format!("{base_path}_difficulty")),
            halving: CatalogTree_Computed_Blocks_Halving::new(client.clone(), format!("{base_path}_halving")),
            interval: CatalogTree_Computed_Blocks_Interval::new(client.clone(), format!("{base_path}_interval")),
            mining: CatalogTree_Computed_Blocks_Mining::new(client.clone(), format!("{base_path}_mining")),
            rewards: CatalogTree_Computed_Blocks_Rewards::new(client.clone(), format!("{base_path}_rewards")),
            size: CatalogTree_Computed_Blocks_Size::new(client.clone(), format!("{base_path}_size")),
            time: CatalogTree_Computed_Blocks_Time::new(client.clone(), format!("{base_path}_time")),
            weight: CatalogTree_Computed_Blocks_Weight::new(client.clone(), format!("{base_path}_weight")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Blocks_Count {
    pub _1m_block_count: MetricPattern4<StoredU32>,
    pub _1w_block_count: MetricPattern4<StoredU32>,
    pub _1y_block_count: MetricPattern4<StoredU32>,
    pub _24h_block_count: MetricPattern25<StoredU32>,
    pub block_count: BlockCountPattern<StoredU32>,
    pub block_count_target: MetricPattern4<StoredU64>,
}

impl CatalogTree_Computed_Blocks_Count {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _1m_block_count: MetricPattern4::new(client.clone(), format!("{base_path}_1m_block_count")),
            _1w_block_count: MetricPattern4::new(client.clone(), format!("{base_path}_1w_block_count")),
            _1y_block_count: MetricPattern4::new(client.clone(), format!("{base_path}_1y_block_count")),
            _24h_block_count: MetricPattern25::new(client.clone(), format!("{base_path}_24h_block_count")),
            block_count: BlockCountPattern::new(client.clone(), "block_count".to_string()),
            block_count_target: MetricPattern4::new(client.clone(), format!("{base_path}_block_count_target")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Blocks_Difficulty {
    pub blocks_before_next_difficulty_adjustment: MetricPattern1<StoredU32>,
    pub days_before_next_difficulty_adjustment: MetricPattern1<StoredF32>,
    pub difficultyepoch: MetricPattern4<DifficultyEpoch>,
}

impl CatalogTree_Computed_Blocks_Difficulty {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            blocks_before_next_difficulty_adjustment: MetricPattern1::new(client.clone(), format!("{base_path}_blocks_before_next_difficulty_adjustment")),
            days_before_next_difficulty_adjustment: MetricPattern1::new(client.clone(), format!("{base_path}_days_before_next_difficulty_adjustment")),
            difficultyepoch: MetricPattern4::new(client.clone(), format!("{base_path}_difficultyepoch")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Blocks_Halving {
    pub blocks_before_next_halving: MetricPattern1<StoredU32>,
    pub days_before_next_halving: MetricPattern1<StoredF32>,
    pub halvingepoch: MetricPattern4<HalvingEpoch>,
}

impl CatalogTree_Computed_Blocks_Halving {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            blocks_before_next_halving: MetricPattern1::new(client.clone(), format!("{base_path}_blocks_before_next_halving")),
            days_before_next_halving: MetricPattern1::new(client.clone(), format!("{base_path}_days_before_next_halving")),
            halvingepoch: MetricPattern4::new(client.clone(), format!("{base_path}_halvingepoch")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Blocks_Interval {
    pub block_interval: BlockIntervalPattern<Timestamp>,
    pub interval: MetricPattern25<Timestamp>,
}

impl CatalogTree_Computed_Blocks_Interval {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            block_interval: BlockIntervalPattern::new(client.clone(), "block_interval".to_string()),
            interval: MetricPattern25::new(client.clone(), format!("{base_path}_interval")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Blocks_Mining {
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

impl CatalogTree_Computed_Blocks_Mining {
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
pub struct CatalogTree_Computed_Blocks_Rewards {
    pub _24h_coinbase_sum: MetricPattern25<Sats>,
    pub _24h_coinbase_usd_sum: MetricPattern25<Dollars>,
    pub coinbase: CoinbasePattern,
    pub fee_dominance: MetricPattern21<StoredF32>,
    pub subsidy: CoinbasePattern,
    pub subsidy_dominance: MetricPattern21<StoredF32>,
    pub subsidy_usd_1y_sma: MetricPattern4<Dollars>,
    pub unclaimed_rewards: UnclaimedRewardsPattern,
}

impl CatalogTree_Computed_Blocks_Rewards {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _24h_coinbase_sum: MetricPattern25::new(client.clone(), format!("{base_path}_24h_coinbase_sum")),
            _24h_coinbase_usd_sum: MetricPattern25::new(client.clone(), format!("{base_path}_24h_coinbase_usd_sum")),
            coinbase: CoinbasePattern::new(client.clone(), "coinbase".to_string()),
            fee_dominance: MetricPattern21::new(client.clone(), format!("{base_path}_fee_dominance")),
            subsidy: CoinbasePattern::new(client.clone(), "subsidy".to_string()),
            subsidy_dominance: MetricPattern21::new(client.clone(), format!("{base_path}_subsidy_dominance")),
            subsidy_usd_1y_sma: MetricPattern4::new(client.clone(), format!("{base_path}_subsidy_usd_1y_sma")),
            unclaimed_rewards: UnclaimedRewardsPattern::new(client.clone(), "unclaimed_rewards".to_string()),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Blocks_Size {
    pub block_size: BlockSizePattern<StoredU64>,
    pub block_vbytes: BlockSizePattern<StoredU64>,
    pub vbytes: MetricPattern25<StoredU64>,
}

impl CatalogTree_Computed_Blocks_Size {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            block_size: BlockSizePattern::new(client.clone(), "block_size".to_string()),
            block_vbytes: BlockSizePattern::new(client.clone(), "block_vbytes".to_string()),
            vbytes: MetricPattern25::new(client.clone(), format!("{base_path}_vbytes")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Blocks_Time {
    pub date: MetricPattern25<Date>,
    pub date_fixed: MetricPattern25<Date>,
    pub timestamp: MetricPattern2<Timestamp>,
    pub timestamp_fixed: MetricPattern25<Timestamp>,
}

impl CatalogTree_Computed_Blocks_Time {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            date: MetricPattern25::new(client.clone(), format!("{base_path}_date")),
            date_fixed: MetricPattern25::new(client.clone(), format!("{base_path}_date_fixed")),
            timestamp: MetricPattern2::new(client.clone(), format!("{base_path}_timestamp")),
            timestamp_fixed: MetricPattern25::new(client.clone(), format!("{base_path}_timestamp_fixed")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Blocks_Weight {
    pub block_fullness: BitcoinPattern<StoredF32>,
    pub block_weight: BlockSizePattern<Weight>,
}

impl CatalogTree_Computed_Blocks_Weight {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            block_fullness: BitcoinPattern::new(client.clone(), "block_fullness".to_string()),
            block_weight: BlockSizePattern::new(client.clone(), "block_weight".to_string()),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Cointime {
    pub activity: CatalogTree_Computed_Cointime_Activity,
    pub adjusted: CatalogTree_Computed_Cointime_Adjusted,
    pub cap: CatalogTree_Computed_Cointime_Cap,
    pub pricing: CatalogTree_Computed_Cointime_Pricing,
    pub supply: CatalogTree_Computed_Cointime_Supply,
    pub value: CatalogTree_Computed_Cointime_Value,
}

impl CatalogTree_Computed_Cointime {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            activity: CatalogTree_Computed_Cointime_Activity::new(client.clone(), format!("{base_path}_activity")),
            adjusted: CatalogTree_Computed_Cointime_Adjusted::new(client.clone(), format!("{base_path}_adjusted")),
            cap: CatalogTree_Computed_Cointime_Cap::new(client.clone(), format!("{base_path}_cap")),
            pricing: CatalogTree_Computed_Cointime_Pricing::new(client.clone(), format!("{base_path}_pricing")),
            supply: CatalogTree_Computed_Cointime_Supply::new(client.clone(), format!("{base_path}_supply")),
            value: CatalogTree_Computed_Cointime_Value::new(client.clone(), format!("{base_path}_value")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Cointime_Activity {
    pub activity_to_vaultedness_ratio: MetricPattern1<StoredF64>,
    pub coinblocks_created: BlockCountPattern<StoredF64>,
    pub coinblocks_stored: BlockCountPattern<StoredF64>,
    pub liveliness: MetricPattern1<StoredF64>,
    pub vaultedness: MetricPattern1<StoredF64>,
}

impl CatalogTree_Computed_Cointime_Activity {
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
pub struct CatalogTree_Computed_Cointime_Adjusted {
    pub cointime_adj_inflation_rate: MetricPattern4<StoredF32>,
    pub cointime_adj_tx_btc_velocity: MetricPattern4<StoredF64>,
    pub cointime_adj_tx_usd_velocity: MetricPattern4<StoredF64>,
}

impl CatalogTree_Computed_Cointime_Adjusted {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            cointime_adj_inflation_rate: MetricPattern4::new(client.clone(), format!("{base_path}_cointime_adj_inflation_rate")),
            cointime_adj_tx_btc_velocity: MetricPattern4::new(client.clone(), format!("{base_path}_cointime_adj_tx_btc_velocity")),
            cointime_adj_tx_usd_velocity: MetricPattern4::new(client.clone(), format!("{base_path}_cointime_adj_tx_usd_velocity")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Cointime_Cap {
    pub active_cap: MetricPattern1<Dollars>,
    pub cointime_cap: MetricPattern1<Dollars>,
    pub investor_cap: MetricPattern1<Dollars>,
    pub thermo_cap: MetricPattern1<Dollars>,
    pub vaulted_cap: MetricPattern1<Dollars>,
}

impl CatalogTree_Computed_Cointime_Cap {
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
pub struct CatalogTree_Computed_Cointime_Pricing {
    pub active_price: MetricPattern1<Dollars>,
    pub active_price_ratio: ActivePriceRatioPattern,
    pub cointime_price: MetricPattern1<Dollars>,
    pub cointime_price_ratio: ActivePriceRatioPattern,
    pub true_market_mean: MetricPattern1<Dollars>,
    pub true_market_mean_ratio: ActivePriceRatioPattern,
    pub vaulted_price: MetricPattern1<Dollars>,
    pub vaulted_price_ratio: ActivePriceRatioPattern,
}

impl CatalogTree_Computed_Cointime_Pricing {
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
pub struct CatalogTree_Computed_Cointime_Supply {
    pub active_supply: ActiveSupplyPattern,
    pub vaulted_supply: ActiveSupplyPattern,
}

impl CatalogTree_Computed_Cointime_Supply {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            active_supply: ActiveSupplyPattern::new(client.clone(), "active_supply".to_string()),
            vaulted_supply: ActiveSupplyPattern::new(client.clone(), "vaulted_supply".to_string()),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Cointime_Value {
    pub cointime_value_created: BlockCountPattern<StoredF64>,
    pub cointime_value_destroyed: BlockCountPattern<StoredF64>,
    pub cointime_value_stored: BlockCountPattern<StoredF64>,
}

impl CatalogTree_Computed_Cointime_Value {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            cointime_value_created: BlockCountPattern::new(client.clone(), "cointime_value_created".to_string()),
            cointime_value_destroyed: BlockCountPattern::new(client.clone(), "cointime_value_destroyed".to_string()),
            cointime_value_stored: BlockCountPattern::new(client.clone(), "cointime_value_stored".to_string()),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Constants {
    pub constant_0: Constant0Pattern<StoredU16>,
    pub constant_1: Constant0Pattern<StoredU16>,
    pub constant_100: Constant0Pattern<StoredU16>,
    pub constant_2: Constant0Pattern<StoredU16>,
    pub constant_3: Constant0Pattern<StoredU16>,
    pub constant_38_2: Constant0Pattern<StoredF32>,
    pub constant_4: Constant0Pattern<StoredU16>,
    pub constant_50: Constant0Pattern<StoredU16>,
    pub constant_600: Constant0Pattern<StoredU16>,
    pub constant_61_8: Constant0Pattern<StoredF32>,
    pub constant_minus_1: Constant0Pattern<StoredI16>,
    pub constant_minus_2: Constant0Pattern<StoredI16>,
    pub constant_minus_3: Constant0Pattern<StoredI16>,
    pub constant_minus_4: Constant0Pattern<StoredI16>,
}

impl CatalogTree_Computed_Constants {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            constant_0: Constant0Pattern::new(client.clone(), "constant_0".to_string()),
            constant_1: Constant0Pattern::new(client.clone(), "constant_1".to_string()),
            constant_100: Constant0Pattern::new(client.clone(), "constant_100".to_string()),
            constant_2: Constant0Pattern::new(client.clone(), "constant_2".to_string()),
            constant_3: Constant0Pattern::new(client.clone(), "constant_3".to_string()),
            constant_38_2: Constant0Pattern::new(client.clone(), "constant_38_2".to_string()),
            constant_4: Constant0Pattern::new(client.clone(), "constant_4".to_string()),
            constant_50: Constant0Pattern::new(client.clone(), "constant_50".to_string()),
            constant_600: Constant0Pattern::new(client.clone(), "constant_600".to_string()),
            constant_61_8: Constant0Pattern::new(client.clone(), "constant_61_8".to_string()),
            constant_minus_1: Constant0Pattern::new(client.clone(), "constant_minus_1".to_string()),
            constant_minus_2: Constant0Pattern::new(client.clone(), "constant_minus_2".to_string()),
            constant_minus_3: Constant0Pattern::new(client.clone(), "constant_minus_3".to_string()),
            constant_minus_4: Constant0Pattern::new(client.clone(), "constant_minus_4".to_string()),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Distribution {
    pub addr_count: MetricPattern1<StoredU64>,
    pub address_cohorts: CatalogTree_Computed_Distribution_AddressCohorts,
    pub addresses_data: CatalogTree_Computed_Distribution_AddressesData,
    pub addresstype_to_height_to_addr_count: AddresstypeToHeightToAddrCountPattern<StoredU64>,
    pub addresstype_to_height_to_empty_addr_count: AddresstypeToHeightToAddrCountPattern<StoredU64>,
    pub addresstype_to_indexes_to_addr_count: AddresstypeToHeightToAddrCountPattern<StoredU64>,
    pub addresstype_to_indexes_to_empty_addr_count: AddresstypeToHeightToAddrCountPattern<StoredU64>,
    pub any_address_indexes: AddresstypeToHeightToAddrCountPattern<AnyAddressIndex>,
    pub chain_state: MetricPattern25<SupplyState>,
    pub empty_addr_count: MetricPattern1<StoredU64>,
    pub emptyaddressindex: MetricPattern46<EmptyAddressIndex>,
    pub loadedaddressindex: MetricPattern45<LoadedAddressIndex>,
    pub utxo_cohorts: CatalogTree_Computed_Distribution_UtxoCohorts,
}

impl CatalogTree_Computed_Distribution {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            addr_count: MetricPattern1::new(client.clone(), format!("{base_path}_addr_count")),
            address_cohorts: CatalogTree_Computed_Distribution_AddressCohorts::new(client.clone(), format!("{base_path}_address_cohorts")),
            addresses_data: CatalogTree_Computed_Distribution_AddressesData::new(client.clone(), format!("{base_path}_addresses_data")),
            addresstype_to_height_to_addr_count: AddresstypeToHeightToAddrCountPattern::new(client.clone(), "".to_string()),
            addresstype_to_height_to_empty_addr_count: AddresstypeToHeightToAddrCountPattern::new(client.clone(), "".to_string()),
            addresstype_to_indexes_to_addr_count: AddresstypeToHeightToAddrCountPattern::new(client.clone(), "".to_string()),
            addresstype_to_indexes_to_empty_addr_count: AddresstypeToHeightToAddrCountPattern::new(client.clone(), "".to_string()),
            any_address_indexes: AddresstypeToHeightToAddrCountPattern::new(client.clone(), "anyaddressindex".to_string()),
            chain_state: MetricPattern25::new(client.clone(), format!("{base_path}_chain_state")),
            empty_addr_count: MetricPattern1::new(client.clone(), format!("{base_path}_empty_addr_count")),
            emptyaddressindex: MetricPattern46::new(client.clone(), format!("{base_path}_emptyaddressindex")),
            loadedaddressindex: MetricPattern45::new(client.clone(), format!("{base_path}_loadedaddressindex")),
            utxo_cohorts: CatalogTree_Computed_Distribution_UtxoCohorts::new(client.clone(), format!("{base_path}_utxo_cohorts")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Distribution_AddressCohorts {
    pub amount_range: CatalogTree_Computed_Distribution_AddressCohorts_AmountRange,
    pub ge_amount: CatalogTree_Computed_Distribution_AddressCohorts_GeAmount,
    pub lt_amount: CatalogTree_Computed_Distribution_AddressCohorts_LtAmount,
}

impl CatalogTree_Computed_Distribution_AddressCohorts {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            amount_range: CatalogTree_Computed_Distribution_AddressCohorts_AmountRange::new(client.clone(), format!("{base_path}_amount_range")),
            ge_amount: CatalogTree_Computed_Distribution_AddressCohorts_GeAmount::new(client.clone(), format!("{base_path}_ge_amount")),
            lt_amount: CatalogTree_Computed_Distribution_AddressCohorts_LtAmount::new(client.clone(), format!("{base_path}_lt_amount")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Distribution_AddressCohorts_AmountRange {
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

impl CatalogTree_Computed_Distribution_AddressCohorts_AmountRange {
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
pub struct CatalogTree_Computed_Distribution_AddressCohorts_GeAmount {
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

impl CatalogTree_Computed_Distribution_AddressCohorts_GeAmount {
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
pub struct CatalogTree_Computed_Distribution_AddressCohorts_LtAmount {
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

impl CatalogTree_Computed_Distribution_AddressCohorts_LtAmount {
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
pub struct CatalogTree_Computed_Distribution_AddressesData {
    pub empty: MetricPattern46<EmptyAddressData>,
    pub loaded: MetricPattern45<LoadedAddressData>,
}

impl CatalogTree_Computed_Distribution_AddressesData {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            empty: MetricPattern46::new(client.clone(), format!("{base_path}_empty")),
            loaded: MetricPattern45::new(client.clone(), format!("{base_path}_loaded")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Distribution_UtxoCohorts {
    pub age_range: CatalogTree_Computed_Distribution_UtxoCohorts_AgeRange,
    pub all: CatalogTree_Computed_Distribution_UtxoCohorts_All,
    pub amount_range: CatalogTree_Computed_Distribution_UtxoCohorts_AmountRange,
    pub epoch: CatalogTree_Computed_Distribution_UtxoCohorts_Epoch,
    pub ge_amount: CatalogTree_Computed_Distribution_UtxoCohorts_GeAmount,
    pub lt_amount: CatalogTree_Computed_Distribution_UtxoCohorts_LtAmount,
    pub max_age: CatalogTree_Computed_Distribution_UtxoCohorts_MaxAge,
    pub min_age: CatalogTree_Computed_Distribution_UtxoCohorts_MinAge,
    pub term: CatalogTree_Computed_Distribution_UtxoCohorts_Term,
    pub type_: CatalogTree_Computed_Distribution_UtxoCohorts_Type,
    pub year: CatalogTree_Computed_Distribution_UtxoCohorts_Year,
}

impl CatalogTree_Computed_Distribution_UtxoCohorts {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            age_range: CatalogTree_Computed_Distribution_UtxoCohorts_AgeRange::new(client.clone(), format!("{base_path}_age_range")),
            all: CatalogTree_Computed_Distribution_UtxoCohorts_All::new(client.clone(), format!("{base_path}_all")),
            amount_range: CatalogTree_Computed_Distribution_UtxoCohorts_AmountRange::new(client.clone(), format!("{base_path}_amount_range")),
            epoch: CatalogTree_Computed_Distribution_UtxoCohorts_Epoch::new(client.clone(), format!("{base_path}_epoch")),
            ge_amount: CatalogTree_Computed_Distribution_UtxoCohorts_GeAmount::new(client.clone(), format!("{base_path}_ge_amount")),
            lt_amount: CatalogTree_Computed_Distribution_UtxoCohorts_LtAmount::new(client.clone(), format!("{base_path}_lt_amount")),
            max_age: CatalogTree_Computed_Distribution_UtxoCohorts_MaxAge::new(client.clone(), format!("{base_path}_max_age")),
            min_age: CatalogTree_Computed_Distribution_UtxoCohorts_MinAge::new(client.clone(), format!("{base_path}_min_age")),
            term: CatalogTree_Computed_Distribution_UtxoCohorts_Term::new(client.clone(), format!("{base_path}_term")),
            type_: CatalogTree_Computed_Distribution_UtxoCohorts_Type::new(client.clone(), format!("{base_path}_type_")),
            year: CatalogTree_Computed_Distribution_UtxoCohorts_Year::new(client.clone(), format!("{base_path}_year")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Distribution_UtxoCohorts_AgeRange {
    pub _10y_to_12y: _10yTo12yPattern,
    pub _12y_to_15y: _10yTo12yPattern,
    pub _1d_to_1w: _10yTo12yPattern,
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
    pub up_to_1d: UpTo1dPattern,
}

impl CatalogTree_Computed_Distribution_UtxoCohorts_AgeRange {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _10y_to_12y: _10yTo12yPattern::new(client.clone(), "utxos_at_least_10y_up_to_12y_old".to_string()),
            _12y_to_15y: _10yTo12yPattern::new(client.clone(), "utxos_at_least_12y_up_to_15y_old".to_string()),
            _1d_to_1w: _10yTo12yPattern::new(client.clone(), "utxos_at_least_1d_up_to_1w_old".to_string()),
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
            up_to_1d: UpTo1dPattern::new(client.clone(), "utxos_up_to_1d_old".to_string()),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Distribution_UtxoCohorts_All {
    pub activity: ActivityPattern2,
    pub cost_basis: CostBasisPattern2,
    pub realized: RealizedPattern3,
    pub relative: CatalogTree_Computed_Distribution_UtxoCohorts_All_Relative,
    pub supply: SupplyPattern3,
    pub unrealized: UnrealizedPattern,
}

impl CatalogTree_Computed_Distribution_UtxoCohorts_All {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            activity: ActivityPattern2::new(client.clone(), "".to_string()),
            cost_basis: CostBasisPattern2::new(client.clone(), "".to_string()),
            realized: RealizedPattern3::new(client.clone(), "".to_string()),
            relative: CatalogTree_Computed_Distribution_UtxoCohorts_All_Relative::new(client.clone(), format!("{base_path}_relative")),
            supply: SupplyPattern3::new(client.clone(), "".to_string()),
            unrealized: UnrealizedPattern::new(client.clone(), "".to_string()),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Distribution_UtxoCohorts_All_Relative {
    pub neg_unrealized_loss_rel_to_own_total_unrealized_pnl: MetricPattern5<StoredF32>,
    pub net_unrealized_pnl_rel_to_own_total_unrealized_pnl: MetricPattern3<StoredF32>,
    pub supply_in_loss_rel_to_own_supply: MetricPattern5<StoredF64>,
    pub supply_in_profit_rel_to_own_supply: MetricPattern5<StoredF64>,
    pub unrealized_loss_rel_to_own_total_unrealized_pnl: MetricPattern5<StoredF32>,
    pub unrealized_profit_rel_to_own_total_unrealized_pnl: MetricPattern5<StoredF32>,
}

impl CatalogTree_Computed_Distribution_UtxoCohorts_All_Relative {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            neg_unrealized_loss_rel_to_own_total_unrealized_pnl: MetricPattern5::new(client.clone(), format!("{base_path}_neg_unrealized_loss_rel_to_own_total_unrealized_pnl")),
            net_unrealized_pnl_rel_to_own_total_unrealized_pnl: MetricPattern3::new(client.clone(), format!("{base_path}_net_unrealized_pnl_rel_to_own_total_unrealized_pnl")),
            supply_in_loss_rel_to_own_supply: MetricPattern5::new(client.clone(), format!("{base_path}_supply_in_loss_rel_to_own_supply")),
            supply_in_profit_rel_to_own_supply: MetricPattern5::new(client.clone(), format!("{base_path}_supply_in_profit_rel_to_own_supply")),
            unrealized_loss_rel_to_own_total_unrealized_pnl: MetricPattern5::new(client.clone(), format!("{base_path}_unrealized_loss_rel_to_own_total_unrealized_pnl")),
            unrealized_profit_rel_to_own_total_unrealized_pnl: MetricPattern5::new(client.clone(), format!("{base_path}_unrealized_profit_rel_to_own_total_unrealized_pnl")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Distribution_UtxoCohorts_AmountRange {
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

impl CatalogTree_Computed_Distribution_UtxoCohorts_AmountRange {
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
pub struct CatalogTree_Computed_Distribution_UtxoCohorts_Epoch {
    pub _0: _10yTo12yPattern,
    pub _1: _10yTo12yPattern,
    pub _2: _10yTo12yPattern,
    pub _3: _10yTo12yPattern,
    pub _4: _10yTo12yPattern,
}

impl CatalogTree_Computed_Distribution_UtxoCohorts_Epoch {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _0: _10yTo12yPattern::new(client.clone(), "epoch_0".to_string()),
            _1: _10yTo12yPattern::new(client.clone(), "epoch_1".to_string()),
            _2: _10yTo12yPattern::new(client.clone(), "epoch_2".to_string()),
            _3: _10yTo12yPattern::new(client.clone(), "epoch_3".to_string()),
            _4: _10yTo12yPattern::new(client.clone(), "epoch_4".to_string()),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Distribution_UtxoCohorts_GeAmount {
    pub _100btc: _0satsPattern2,
    pub _100k_sats: _0satsPattern2,
    pub _100sats: _0satsPattern2,
    pub _10btc: _0satsPattern2,
    pub _10k_btc: _0satsPattern2,
    pub _10k_sats: _0satsPattern2,
    pub _10m_sats: _0satsPattern2,
    pub _10sats: _0satsPattern2,
    pub _1btc: _0satsPattern2,
    pub _1k_btc: _0satsPattern2,
    pub _1k_sats: _0satsPattern2,
    pub _1m_sats: _0satsPattern2,
    pub _1sat: _0satsPattern2,
}

impl CatalogTree_Computed_Distribution_UtxoCohorts_GeAmount {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _100btc: _0satsPattern2::new(client.clone(), "utxos_above_100btc".to_string()),
            _100k_sats: _0satsPattern2::new(client.clone(), "utxos_above_100k_sats".to_string()),
            _100sats: _0satsPattern2::new(client.clone(), "utxos_above_100sats".to_string()),
            _10btc: _0satsPattern2::new(client.clone(), "utxos_above_10btc".to_string()),
            _10k_btc: _0satsPattern2::new(client.clone(), "utxos_above_10k_btc".to_string()),
            _10k_sats: _0satsPattern2::new(client.clone(), "utxos_above_10k_sats".to_string()),
            _10m_sats: _0satsPattern2::new(client.clone(), "utxos_above_10m_sats".to_string()),
            _10sats: _0satsPattern2::new(client.clone(), "utxos_above_10sats".to_string()),
            _1btc: _0satsPattern2::new(client.clone(), "utxos_above_1btc".to_string()),
            _1k_btc: _0satsPattern2::new(client.clone(), "utxos_above_1k_btc".to_string()),
            _1k_sats: _0satsPattern2::new(client.clone(), "utxos_above_1k_sats".to_string()),
            _1m_sats: _0satsPattern2::new(client.clone(), "utxos_above_1m_sats".to_string()),
            _1sat: _0satsPattern2::new(client.clone(), "utxos_above_1sat".to_string()),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Distribution_UtxoCohorts_LtAmount {
    pub _100btc: _0satsPattern2,
    pub _100k_btc: _0satsPattern2,
    pub _100k_sats: _0satsPattern2,
    pub _100sats: _0satsPattern2,
    pub _10btc: _0satsPattern2,
    pub _10k_btc: _0satsPattern2,
    pub _10k_sats: _0satsPattern2,
    pub _10m_sats: _0satsPattern2,
    pub _10sats: _0satsPattern2,
    pub _1btc: _0satsPattern2,
    pub _1k_btc: _0satsPattern2,
    pub _1k_sats: _0satsPattern2,
    pub _1m_sats: _0satsPattern2,
}

impl CatalogTree_Computed_Distribution_UtxoCohorts_LtAmount {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _100btc: _0satsPattern2::new(client.clone(), "utxos_under_100btc".to_string()),
            _100k_btc: _0satsPattern2::new(client.clone(), "utxos_under_100k_btc".to_string()),
            _100k_sats: _0satsPattern2::new(client.clone(), "utxos_under_100k_sats".to_string()),
            _100sats: _0satsPattern2::new(client.clone(), "utxos_under_100sats".to_string()),
            _10btc: _0satsPattern2::new(client.clone(), "utxos_under_10btc".to_string()),
            _10k_btc: _0satsPattern2::new(client.clone(), "utxos_under_10k_btc".to_string()),
            _10k_sats: _0satsPattern2::new(client.clone(), "utxos_under_10k_sats".to_string()),
            _10m_sats: _0satsPattern2::new(client.clone(), "utxos_under_10m_sats".to_string()),
            _10sats: _0satsPattern2::new(client.clone(), "utxos_under_10sats".to_string()),
            _1btc: _0satsPattern2::new(client.clone(), "utxos_under_1btc".to_string()),
            _1k_btc: _0satsPattern2::new(client.clone(), "utxos_under_1k_btc".to_string()),
            _1k_sats: _0satsPattern2::new(client.clone(), "utxos_under_1k_sats".to_string()),
            _1m_sats: _0satsPattern2::new(client.clone(), "utxos_under_1m_sats".to_string()),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Distribution_UtxoCohorts_MaxAge {
    pub _10y: UpTo1dPattern,
    pub _12y: UpTo1dPattern,
    pub _15y: UpTo1dPattern,
    pub _1m: UpTo1dPattern,
    pub _1w: UpTo1dPattern,
    pub _1y: UpTo1dPattern,
    pub _2m: UpTo1dPattern,
    pub _2y: UpTo1dPattern,
    pub _3m: UpTo1dPattern,
    pub _3y: UpTo1dPattern,
    pub _4m: UpTo1dPattern,
    pub _4y: UpTo1dPattern,
    pub _5m: UpTo1dPattern,
    pub _5y: UpTo1dPattern,
    pub _6m: UpTo1dPattern,
    pub _6y: UpTo1dPattern,
    pub _7y: UpTo1dPattern,
    pub _8y: UpTo1dPattern,
}

impl CatalogTree_Computed_Distribution_UtxoCohorts_MaxAge {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _10y: UpTo1dPattern::new(client.clone(), "utxos_up_to_10y_old".to_string()),
            _12y: UpTo1dPattern::new(client.clone(), "utxos_up_to_12y_old".to_string()),
            _15y: UpTo1dPattern::new(client.clone(), "utxos_up_to_15y_old".to_string()),
            _1m: UpTo1dPattern::new(client.clone(), "utxos_up_to_1m_old".to_string()),
            _1w: UpTo1dPattern::new(client.clone(), "utxos_up_to_1w_old".to_string()),
            _1y: UpTo1dPattern::new(client.clone(), "utxos_up_to_1y_old".to_string()),
            _2m: UpTo1dPattern::new(client.clone(), "utxos_up_to_2m_old".to_string()),
            _2y: UpTo1dPattern::new(client.clone(), "utxos_up_to_2y_old".to_string()),
            _3m: UpTo1dPattern::new(client.clone(), "utxos_up_to_3m_old".to_string()),
            _3y: UpTo1dPattern::new(client.clone(), "utxos_up_to_3y_old".to_string()),
            _4m: UpTo1dPattern::new(client.clone(), "utxos_up_to_4m_old".to_string()),
            _4y: UpTo1dPattern::new(client.clone(), "utxos_up_to_4y_old".to_string()),
            _5m: UpTo1dPattern::new(client.clone(), "utxos_up_to_5m_old".to_string()),
            _5y: UpTo1dPattern::new(client.clone(), "utxos_up_to_5y_old".to_string()),
            _6m: UpTo1dPattern::new(client.clone(), "utxos_up_to_6m_old".to_string()),
            _6y: UpTo1dPattern::new(client.clone(), "utxos_up_to_6y_old".to_string()),
            _7y: UpTo1dPattern::new(client.clone(), "utxos_up_to_7y_old".to_string()),
            _8y: UpTo1dPattern::new(client.clone(), "utxos_up_to_8y_old".to_string()),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Distribution_UtxoCohorts_MinAge {
    pub _10y: _10yTo12yPattern,
    pub _12y: _10yTo12yPattern,
    pub _1d: _10yTo12yPattern,
    pub _1m: _10yTo12yPattern,
    pub _1w: _10yTo12yPattern,
    pub _1y: _10yTo12yPattern,
    pub _2m: _10yTo12yPattern,
    pub _2y: _10yTo12yPattern,
    pub _3m: _10yTo12yPattern,
    pub _3y: _10yTo12yPattern,
    pub _4m: _10yTo12yPattern,
    pub _4y: _10yTo12yPattern,
    pub _5m: _10yTo12yPattern,
    pub _5y: _10yTo12yPattern,
    pub _6m: _10yTo12yPattern,
    pub _6y: _10yTo12yPattern,
    pub _7y: _10yTo12yPattern,
    pub _8y: _10yTo12yPattern,
}

impl CatalogTree_Computed_Distribution_UtxoCohorts_MinAge {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _10y: _10yTo12yPattern::new(client.clone(), "utxos_at_least_10y_old".to_string()),
            _12y: _10yTo12yPattern::new(client.clone(), "utxos_at_least_12y_old".to_string()),
            _1d: _10yTo12yPattern::new(client.clone(), "utxos_at_least_1d_old".to_string()),
            _1m: _10yTo12yPattern::new(client.clone(), "utxos_at_least_1m_old".to_string()),
            _1w: _10yTo12yPattern::new(client.clone(), "utxos_at_least_1w_old".to_string()),
            _1y: _10yTo12yPattern::new(client.clone(), "utxos_at_least_1y_old".to_string()),
            _2m: _10yTo12yPattern::new(client.clone(), "utxos_at_least_2m_old".to_string()),
            _2y: _10yTo12yPattern::new(client.clone(), "utxos_at_least_2y_old".to_string()),
            _3m: _10yTo12yPattern::new(client.clone(), "utxos_at_least_3m_old".to_string()),
            _3y: _10yTo12yPattern::new(client.clone(), "utxos_at_least_3y_old".to_string()),
            _4m: _10yTo12yPattern::new(client.clone(), "utxos_at_least_4m_old".to_string()),
            _4y: _10yTo12yPattern::new(client.clone(), "utxos_at_least_4y_old".to_string()),
            _5m: _10yTo12yPattern::new(client.clone(), "utxos_at_least_5m_old".to_string()),
            _5y: _10yTo12yPattern::new(client.clone(), "utxos_at_least_5y_old".to_string()),
            _6m: _10yTo12yPattern::new(client.clone(), "utxos_at_least_6m_old".to_string()),
            _6y: _10yTo12yPattern::new(client.clone(), "utxos_at_least_6y_old".to_string()),
            _7y: _10yTo12yPattern::new(client.clone(), "utxos_at_least_7y_old".to_string()),
            _8y: _10yTo12yPattern::new(client.clone(), "utxos_at_least_8y_old".to_string()),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Distribution_UtxoCohorts_Term {
    pub long: UpTo1dPattern,
    pub short: UpTo1dPattern,
}

impl CatalogTree_Computed_Distribution_UtxoCohorts_Term {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            long: UpTo1dPattern::new(client.clone(), "lth".to_string()),
            short: UpTo1dPattern::new(client.clone(), "sth".to_string()),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Distribution_UtxoCohorts_Type {
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

impl CatalogTree_Computed_Distribution_UtxoCohorts_Type {
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
pub struct CatalogTree_Computed_Distribution_UtxoCohorts_Year {
    pub _2009: _10yTo12yPattern,
    pub _2010: _10yTo12yPattern,
    pub _2011: _10yTo12yPattern,
    pub _2012: _10yTo12yPattern,
    pub _2013: _10yTo12yPattern,
    pub _2014: _10yTo12yPattern,
    pub _2015: _10yTo12yPattern,
    pub _2016: _10yTo12yPattern,
    pub _2017: _10yTo12yPattern,
    pub _2018: _10yTo12yPattern,
    pub _2019: _10yTo12yPattern,
    pub _2020: _10yTo12yPattern,
    pub _2021: _10yTo12yPattern,
    pub _2022: _10yTo12yPattern,
    pub _2023: _10yTo12yPattern,
    pub _2024: _10yTo12yPattern,
    pub _2025: _10yTo12yPattern,
    pub _2026: _10yTo12yPattern,
}

impl CatalogTree_Computed_Distribution_UtxoCohorts_Year {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _2009: _10yTo12yPattern::new(client.clone(), "year_2009".to_string()),
            _2010: _10yTo12yPattern::new(client.clone(), "year_2010".to_string()),
            _2011: _10yTo12yPattern::new(client.clone(), "year_2011".to_string()),
            _2012: _10yTo12yPattern::new(client.clone(), "year_2012".to_string()),
            _2013: _10yTo12yPattern::new(client.clone(), "year_2013".to_string()),
            _2014: _10yTo12yPattern::new(client.clone(), "year_2014".to_string()),
            _2015: _10yTo12yPattern::new(client.clone(), "year_2015".to_string()),
            _2016: _10yTo12yPattern::new(client.clone(), "year_2016".to_string()),
            _2017: _10yTo12yPattern::new(client.clone(), "year_2017".to_string()),
            _2018: _10yTo12yPattern::new(client.clone(), "year_2018".to_string()),
            _2019: _10yTo12yPattern::new(client.clone(), "year_2019".to_string()),
            _2020: _10yTo12yPattern::new(client.clone(), "year_2020".to_string()),
            _2021: _10yTo12yPattern::new(client.clone(), "year_2021".to_string()),
            _2022: _10yTo12yPattern::new(client.clone(), "year_2022".to_string()),
            _2023: _10yTo12yPattern::new(client.clone(), "year_2023".to_string()),
            _2024: _10yTo12yPattern::new(client.clone(), "year_2024".to_string()),
            _2025: _10yTo12yPattern::new(client.clone(), "year_2025".to_string()),
            _2026: _10yTo12yPattern::new(client.clone(), "year_2026".to_string()),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Indexes {
    pub address: CatalogTree_Computed_Indexes_Address,
    pub block: CatalogTree_Computed_Indexes_Block,
    pub time: CatalogTree_Computed_Indexes_Time,
    pub transaction: CatalogTree_Computed_Indexes_Transaction,
}

impl CatalogTree_Computed_Indexes {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            address: CatalogTree_Computed_Indexes_Address::new(client.clone(), format!("{base_path}_address")),
            block: CatalogTree_Computed_Indexes_Block::new(client.clone(), format!("{base_path}_block")),
            time: CatalogTree_Computed_Indexes_Time::new(client.clone(), format!("{base_path}_time")),
            transaction: CatalogTree_Computed_Indexes_Transaction::new(client.clone(), format!("{base_path}_transaction")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Indexes_Address {
    pub emptyoutputindex: MetricPattern24<EmptyOutputIndex>,
    pub opreturnindex: MetricPattern28<OpReturnIndex>,
    pub p2aaddressindex: MetricPattern30<P2AAddressIndex>,
    pub p2msoutputindex: MetricPattern31<P2MSOutputIndex>,
    pub p2pk33addressindex: MetricPattern32<P2PK33AddressIndex>,
    pub p2pk65addressindex: MetricPattern33<P2PK65AddressIndex>,
    pub p2pkhaddressindex: MetricPattern34<P2PKHAddressIndex>,
    pub p2shaddressindex: MetricPattern35<P2SHAddressIndex>,
    pub p2traddressindex: MetricPattern36<P2TRAddressIndex>,
    pub p2wpkhaddressindex: MetricPattern37<P2WPKHAddressIndex>,
    pub p2wshaddressindex: MetricPattern38<P2WSHAddressIndex>,
    pub unknownoutputindex: MetricPattern42<UnknownOutputIndex>,
}

impl CatalogTree_Computed_Indexes_Address {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            emptyoutputindex: MetricPattern24::new(client.clone(), format!("{base_path}_emptyoutputindex")),
            opreturnindex: MetricPattern28::new(client.clone(), format!("{base_path}_opreturnindex")),
            p2aaddressindex: MetricPattern30::new(client.clone(), format!("{base_path}_p2aaddressindex")),
            p2msoutputindex: MetricPattern31::new(client.clone(), format!("{base_path}_p2msoutputindex")),
            p2pk33addressindex: MetricPattern32::new(client.clone(), format!("{base_path}_p2pk33addressindex")),
            p2pk65addressindex: MetricPattern33::new(client.clone(), format!("{base_path}_p2pk65addressindex")),
            p2pkhaddressindex: MetricPattern34::new(client.clone(), format!("{base_path}_p2pkhaddressindex")),
            p2shaddressindex: MetricPattern35::new(client.clone(), format!("{base_path}_p2shaddressindex")),
            p2traddressindex: MetricPattern36::new(client.clone(), format!("{base_path}_p2traddressindex")),
            p2wpkhaddressindex: MetricPattern37::new(client.clone(), format!("{base_path}_p2wpkhaddressindex")),
            p2wshaddressindex: MetricPattern38::new(client.clone(), format!("{base_path}_p2wshaddressindex")),
            unknownoutputindex: MetricPattern42::new(client.clone(), format!("{base_path}_unknownoutputindex")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Indexes_Block {
    pub dateindex: MetricPattern25<DateIndex>,
    pub difficultyepoch: MetricPattern14<DifficultyEpoch>,
    pub first_height: MetricPattern13<Height>,
    pub halvingepoch: MetricPattern15<HalvingEpoch>,
    pub height: MetricPattern25<Height>,
    pub height_count: MetricPattern23<StoredU64>,
    pub txindex_count: MetricPattern25<StoredU64>,
}

impl CatalogTree_Computed_Indexes_Block {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            dateindex: MetricPattern25::new(client.clone(), format!("{base_path}_dateindex")),
            difficultyepoch: MetricPattern14::new(client.clone(), format!("{base_path}_difficultyepoch")),
            first_height: MetricPattern13::new(client.clone(), format!("{base_path}_first_height")),
            halvingepoch: MetricPattern15::new(client.clone(), format!("{base_path}_halvingepoch")),
            height: MetricPattern25::new(client.clone(), format!("{base_path}_height")),
            height_count: MetricPattern23::new(client.clone(), format!("{base_path}_height_count")),
            txindex_count: MetricPattern25::new(client.clone(), format!("{base_path}_txindex_count")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Indexes_Time {
    pub date: MetricPattern21<Date>,
    pub dateindex: MetricPattern21<DateIndex>,
    pub dateindex_count: MetricPattern19<StoredU64>,
    pub decadeindex: MetricPattern12<DecadeIndex>,
    pub first_dateindex: MetricPattern19<DateIndex>,
    pub first_height: MetricPattern21<Height>,
    pub first_monthindex: MetricPattern8<MonthIndex>,
    pub first_yearindex: MetricPattern22<YearIndex>,
    pub height_count: MetricPattern21<StoredU64>,
    pub monthindex: MetricPattern10<MonthIndex>,
    pub monthindex_count: MetricPattern8<StoredU64>,
    pub quarterindex: MetricPattern17<QuarterIndex>,
    pub semesterindex: MetricPattern18<SemesterIndex>,
    pub weekindex: MetricPattern11<WeekIndex>,
    pub yearindex: MetricPattern20<YearIndex>,
    pub yearindex_count: MetricPattern22<StoredU64>,
}

impl CatalogTree_Computed_Indexes_Time {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            date: MetricPattern21::new(client.clone(), format!("{base_path}_date")),
            dateindex: MetricPattern21::new(client.clone(), format!("{base_path}_dateindex")),
            dateindex_count: MetricPattern19::new(client.clone(), format!("{base_path}_dateindex_count")),
            decadeindex: MetricPattern12::new(client.clone(), format!("{base_path}_decadeindex")),
            first_dateindex: MetricPattern19::new(client.clone(), format!("{base_path}_first_dateindex")),
            first_height: MetricPattern21::new(client.clone(), format!("{base_path}_first_height")),
            first_monthindex: MetricPattern8::new(client.clone(), format!("{base_path}_first_monthindex")),
            first_yearindex: MetricPattern22::new(client.clone(), format!("{base_path}_first_yearindex")),
            height_count: MetricPattern21::new(client.clone(), format!("{base_path}_height_count")),
            monthindex: MetricPattern10::new(client.clone(), format!("{base_path}_monthindex")),
            monthindex_count: MetricPattern8::new(client.clone(), format!("{base_path}_monthindex_count")),
            quarterindex: MetricPattern17::new(client.clone(), format!("{base_path}_quarterindex")),
            semesterindex: MetricPattern18::new(client.clone(), format!("{base_path}_semesterindex")),
            weekindex: MetricPattern11::new(client.clone(), format!("{base_path}_weekindex")),
            yearindex: MetricPattern20::new(client.clone(), format!("{base_path}_yearindex")),
            yearindex_count: MetricPattern22::new(client.clone(), format!("{base_path}_yearindex_count")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Indexes_Transaction {
    pub input_count: MetricPattern41<StoredU64>,
    pub output_count: MetricPattern41<StoredU64>,
    pub txindex: MetricPattern41<TxIndex>,
    pub txinindex: MetricPattern26<TxInIndex>,
    pub txoutindex: MetricPattern29<TxOutIndex>,
}

impl CatalogTree_Computed_Indexes_Transaction {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            input_count: MetricPattern41::new(client.clone(), format!("{base_path}_input_count")),
            output_count: MetricPattern41::new(client.clone(), format!("{base_path}_output_count")),
            txindex: MetricPattern41::new(client.clone(), format!("{base_path}_txindex")),
            txinindex: MetricPattern26::new(client.clone(), format!("{base_path}_txinindex")),
            txoutindex: MetricPattern29::new(client.clone(), format!("{base_path}_txoutindex")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Inputs {
    pub count: CatalogTree_Computed_Inputs_Count,
    pub spent: CatalogTree_Computed_Inputs_Spent,
}

impl CatalogTree_Computed_Inputs {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            count: CatalogTree_Computed_Inputs_Count::new(client.clone(), format!("{base_path}_count")),
            spent: CatalogTree_Computed_Inputs_Spent::new(client.clone(), format!("{base_path}_spent")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Inputs_Count {
    pub count: BlockSizePattern<StoredU64>,
}

impl CatalogTree_Computed_Inputs_Count {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            count: BlockSizePattern::new(client.clone(), "input_count".to_string()),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Inputs_Spent {
    pub txoutindex: MetricPattern26<TxOutIndex>,
    pub value: MetricPattern26<Sats>,
}

impl CatalogTree_Computed_Inputs_Spent {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            txoutindex: MetricPattern26::new(client.clone(), format!("{base_path}_txoutindex")),
            value: MetricPattern26::new(client.clone(), format!("{base_path}_value")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Market {
    pub ath: CatalogTree_Computed_Market_Ath,
    pub dca: CatalogTree_Computed_Market_Dca,
    pub indicators: CatalogTree_Computed_Market_Indicators,
    pub lookback: CatalogTree_Computed_Market_Lookback,
    pub moving_average: CatalogTree_Computed_Market_MovingAverage,
    pub range: CatalogTree_Computed_Market_Range,
    pub returns: CatalogTree_Computed_Market_Returns,
    pub volatility: CatalogTree_Computed_Market_Volatility,
}

impl CatalogTree_Computed_Market {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            ath: CatalogTree_Computed_Market_Ath::new(client.clone(), format!("{base_path}_ath")),
            dca: CatalogTree_Computed_Market_Dca::new(client.clone(), format!("{base_path}_dca")),
            indicators: CatalogTree_Computed_Market_Indicators::new(client.clone(), format!("{base_path}_indicators")),
            lookback: CatalogTree_Computed_Market_Lookback::new(client.clone(), format!("{base_path}_lookback")),
            moving_average: CatalogTree_Computed_Market_MovingAverage::new(client.clone(), format!("{base_path}_moving_average")),
            range: CatalogTree_Computed_Market_Range::new(client.clone(), format!("{base_path}_range")),
            returns: CatalogTree_Computed_Market_Returns::new(client.clone(), format!("{base_path}_returns")),
            volatility: CatalogTree_Computed_Market_Volatility::new(client.clone(), format!("{base_path}_volatility")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Market_Ath {
    pub days_since_price_ath: MetricPattern4<StoredU16>,
    pub max_days_between_price_aths: MetricPattern4<StoredU16>,
    pub max_years_between_price_aths: MetricPattern4<StoredF32>,
    pub price_ath: MetricPattern3<Dollars>,
    pub price_drawdown: MetricPattern3<StoredF32>,
}

impl CatalogTree_Computed_Market_Ath {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            days_since_price_ath: MetricPattern4::new(client.clone(), format!("{base_path}_days_since_price_ath")),
            max_days_between_price_aths: MetricPattern4::new(client.clone(), format!("{base_path}_max_days_between_price_aths")),
            max_years_between_price_aths: MetricPattern4::new(client.clone(), format!("{base_path}_max_years_between_price_aths")),
            price_ath: MetricPattern3::new(client.clone(), format!("{base_path}_price_ath")),
            price_drawdown: MetricPattern3::new(client.clone(), format!("{base_path}_price_drawdown")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Market_Dca {
    pub _10y_dca_avg_price: MetricPattern4<Dollars>,
    pub _10y_dca_cagr: MetricPattern4<StoredF32>,
    pub _10y_dca_returns: MetricPattern4<StoredF32>,
    pub _10y_dca_stack: MetricPattern4<Sats>,
    pub _1m_dca_avg_price: MetricPattern4<Dollars>,
    pub _1m_dca_returns: MetricPattern4<StoredF32>,
    pub _1m_dca_stack: MetricPattern4<Sats>,
    pub _1w_dca_avg_price: MetricPattern4<Dollars>,
    pub _1w_dca_returns: MetricPattern4<StoredF32>,
    pub _1w_dca_stack: MetricPattern4<Sats>,
    pub _1y_dca_avg_price: MetricPattern4<Dollars>,
    pub _1y_dca_returns: MetricPattern4<StoredF32>,
    pub _1y_dca_stack: MetricPattern4<Sats>,
    pub _2y_dca_avg_price: MetricPattern4<Dollars>,
    pub _2y_dca_cagr: MetricPattern4<StoredF32>,
    pub _2y_dca_returns: MetricPattern4<StoredF32>,
    pub _2y_dca_stack: MetricPattern4<Sats>,
    pub _3m_dca_avg_price: MetricPattern4<Dollars>,
    pub _3m_dca_returns: MetricPattern4<StoredF32>,
    pub _3m_dca_stack: MetricPattern4<Sats>,
    pub _3y_dca_avg_price: MetricPattern4<Dollars>,
    pub _3y_dca_cagr: MetricPattern4<StoredF32>,
    pub _3y_dca_returns: MetricPattern4<StoredF32>,
    pub _3y_dca_stack: MetricPattern4<Sats>,
    pub _4y_dca_avg_price: MetricPattern4<Dollars>,
    pub _4y_dca_cagr: MetricPattern4<StoredF32>,
    pub _4y_dca_returns: MetricPattern4<StoredF32>,
    pub _4y_dca_stack: MetricPattern4<Sats>,
    pub _5y_dca_avg_price: MetricPattern4<Dollars>,
    pub _5y_dca_cagr: MetricPattern4<StoredF32>,
    pub _5y_dca_returns: MetricPattern4<StoredF32>,
    pub _5y_dca_stack: MetricPattern4<Sats>,
    pub _6m_dca_avg_price: MetricPattern4<Dollars>,
    pub _6m_dca_returns: MetricPattern4<StoredF32>,
    pub _6m_dca_stack: MetricPattern4<Sats>,
    pub _6y_dca_avg_price: MetricPattern4<Dollars>,
    pub _6y_dca_cagr: MetricPattern4<StoredF32>,
    pub _6y_dca_returns: MetricPattern4<StoredF32>,
    pub _6y_dca_stack: MetricPattern4<Sats>,
    pub _8y_dca_avg_price: MetricPattern4<Dollars>,
    pub _8y_dca_cagr: MetricPattern4<StoredF32>,
    pub _8y_dca_returns: MetricPattern4<StoredF32>,
    pub _8y_dca_stack: MetricPattern4<Sats>,
    pub dca_class_2015_avg_price: MetricPattern4<Dollars>,
    pub dca_class_2015_returns: MetricPattern4<StoredF32>,
    pub dca_class_2015_stack: MetricPattern4<Sats>,
    pub dca_class_2016_avg_price: MetricPattern4<Dollars>,
    pub dca_class_2016_returns: MetricPattern4<StoredF32>,
    pub dca_class_2016_stack: MetricPattern4<Sats>,
    pub dca_class_2017_avg_price: MetricPattern4<Dollars>,
    pub dca_class_2017_returns: MetricPattern4<StoredF32>,
    pub dca_class_2017_stack: MetricPattern4<Sats>,
    pub dca_class_2018_avg_price: MetricPattern4<Dollars>,
    pub dca_class_2018_returns: MetricPattern4<StoredF32>,
    pub dca_class_2018_stack: MetricPattern4<Sats>,
    pub dca_class_2019_avg_price: MetricPattern4<Dollars>,
    pub dca_class_2019_returns: MetricPattern4<StoredF32>,
    pub dca_class_2019_stack: MetricPattern4<Sats>,
    pub dca_class_2020_avg_price: MetricPattern4<Dollars>,
    pub dca_class_2020_returns: MetricPattern4<StoredF32>,
    pub dca_class_2020_stack: MetricPattern4<Sats>,
    pub dca_class_2021_avg_price: MetricPattern4<Dollars>,
    pub dca_class_2021_returns: MetricPattern4<StoredF32>,
    pub dca_class_2021_stack: MetricPattern4<Sats>,
    pub dca_class_2022_avg_price: MetricPattern4<Dollars>,
    pub dca_class_2022_returns: MetricPattern4<StoredF32>,
    pub dca_class_2022_stack: MetricPattern4<Sats>,
    pub dca_class_2023_avg_price: MetricPattern4<Dollars>,
    pub dca_class_2023_returns: MetricPattern4<StoredF32>,
    pub dca_class_2023_stack: MetricPattern4<Sats>,
    pub dca_class_2024_avg_price: MetricPattern4<Dollars>,
    pub dca_class_2024_returns: MetricPattern4<StoredF32>,
    pub dca_class_2024_stack: MetricPattern4<Sats>,
    pub dca_class_2025_avg_price: MetricPattern4<Dollars>,
    pub dca_class_2025_returns: MetricPattern4<StoredF32>,
    pub dca_class_2025_stack: MetricPattern4<Sats>,
}

impl CatalogTree_Computed_Market_Dca {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _10y_dca_avg_price: MetricPattern4::new(client.clone(), format!("{base_path}_10y_dca_avg_price")),
            _10y_dca_cagr: MetricPattern4::new(client.clone(), format!("{base_path}_10y_dca_cagr")),
            _10y_dca_returns: MetricPattern4::new(client.clone(), format!("{base_path}_10y_dca_returns")),
            _10y_dca_stack: MetricPattern4::new(client.clone(), format!("{base_path}_10y_dca_stack")),
            _1m_dca_avg_price: MetricPattern4::new(client.clone(), format!("{base_path}_1m_dca_avg_price")),
            _1m_dca_returns: MetricPattern4::new(client.clone(), format!("{base_path}_1m_dca_returns")),
            _1m_dca_stack: MetricPattern4::new(client.clone(), format!("{base_path}_1m_dca_stack")),
            _1w_dca_avg_price: MetricPattern4::new(client.clone(), format!("{base_path}_1w_dca_avg_price")),
            _1w_dca_returns: MetricPattern4::new(client.clone(), format!("{base_path}_1w_dca_returns")),
            _1w_dca_stack: MetricPattern4::new(client.clone(), format!("{base_path}_1w_dca_stack")),
            _1y_dca_avg_price: MetricPattern4::new(client.clone(), format!("{base_path}_1y_dca_avg_price")),
            _1y_dca_returns: MetricPattern4::new(client.clone(), format!("{base_path}_1y_dca_returns")),
            _1y_dca_stack: MetricPattern4::new(client.clone(), format!("{base_path}_1y_dca_stack")),
            _2y_dca_avg_price: MetricPattern4::new(client.clone(), format!("{base_path}_2y_dca_avg_price")),
            _2y_dca_cagr: MetricPattern4::new(client.clone(), format!("{base_path}_2y_dca_cagr")),
            _2y_dca_returns: MetricPattern4::new(client.clone(), format!("{base_path}_2y_dca_returns")),
            _2y_dca_stack: MetricPattern4::new(client.clone(), format!("{base_path}_2y_dca_stack")),
            _3m_dca_avg_price: MetricPattern4::new(client.clone(), format!("{base_path}_3m_dca_avg_price")),
            _3m_dca_returns: MetricPattern4::new(client.clone(), format!("{base_path}_3m_dca_returns")),
            _3m_dca_stack: MetricPattern4::new(client.clone(), format!("{base_path}_3m_dca_stack")),
            _3y_dca_avg_price: MetricPattern4::new(client.clone(), format!("{base_path}_3y_dca_avg_price")),
            _3y_dca_cagr: MetricPattern4::new(client.clone(), format!("{base_path}_3y_dca_cagr")),
            _3y_dca_returns: MetricPattern4::new(client.clone(), format!("{base_path}_3y_dca_returns")),
            _3y_dca_stack: MetricPattern4::new(client.clone(), format!("{base_path}_3y_dca_stack")),
            _4y_dca_avg_price: MetricPattern4::new(client.clone(), format!("{base_path}_4y_dca_avg_price")),
            _4y_dca_cagr: MetricPattern4::new(client.clone(), format!("{base_path}_4y_dca_cagr")),
            _4y_dca_returns: MetricPattern4::new(client.clone(), format!("{base_path}_4y_dca_returns")),
            _4y_dca_stack: MetricPattern4::new(client.clone(), format!("{base_path}_4y_dca_stack")),
            _5y_dca_avg_price: MetricPattern4::new(client.clone(), format!("{base_path}_5y_dca_avg_price")),
            _5y_dca_cagr: MetricPattern4::new(client.clone(), format!("{base_path}_5y_dca_cagr")),
            _5y_dca_returns: MetricPattern4::new(client.clone(), format!("{base_path}_5y_dca_returns")),
            _5y_dca_stack: MetricPattern4::new(client.clone(), format!("{base_path}_5y_dca_stack")),
            _6m_dca_avg_price: MetricPattern4::new(client.clone(), format!("{base_path}_6m_dca_avg_price")),
            _6m_dca_returns: MetricPattern4::new(client.clone(), format!("{base_path}_6m_dca_returns")),
            _6m_dca_stack: MetricPattern4::new(client.clone(), format!("{base_path}_6m_dca_stack")),
            _6y_dca_avg_price: MetricPattern4::new(client.clone(), format!("{base_path}_6y_dca_avg_price")),
            _6y_dca_cagr: MetricPattern4::new(client.clone(), format!("{base_path}_6y_dca_cagr")),
            _6y_dca_returns: MetricPattern4::new(client.clone(), format!("{base_path}_6y_dca_returns")),
            _6y_dca_stack: MetricPattern4::new(client.clone(), format!("{base_path}_6y_dca_stack")),
            _8y_dca_avg_price: MetricPattern4::new(client.clone(), format!("{base_path}_8y_dca_avg_price")),
            _8y_dca_cagr: MetricPattern4::new(client.clone(), format!("{base_path}_8y_dca_cagr")),
            _8y_dca_returns: MetricPattern4::new(client.clone(), format!("{base_path}_8y_dca_returns")),
            _8y_dca_stack: MetricPattern4::new(client.clone(), format!("{base_path}_8y_dca_stack")),
            dca_class_2015_avg_price: MetricPattern4::new(client.clone(), format!("{base_path}_dca_class_2015_avg_price")),
            dca_class_2015_returns: MetricPattern4::new(client.clone(), format!("{base_path}_dca_class_2015_returns")),
            dca_class_2015_stack: MetricPattern4::new(client.clone(), format!("{base_path}_dca_class_2015_stack")),
            dca_class_2016_avg_price: MetricPattern4::new(client.clone(), format!("{base_path}_dca_class_2016_avg_price")),
            dca_class_2016_returns: MetricPattern4::new(client.clone(), format!("{base_path}_dca_class_2016_returns")),
            dca_class_2016_stack: MetricPattern4::new(client.clone(), format!("{base_path}_dca_class_2016_stack")),
            dca_class_2017_avg_price: MetricPattern4::new(client.clone(), format!("{base_path}_dca_class_2017_avg_price")),
            dca_class_2017_returns: MetricPattern4::new(client.clone(), format!("{base_path}_dca_class_2017_returns")),
            dca_class_2017_stack: MetricPattern4::new(client.clone(), format!("{base_path}_dca_class_2017_stack")),
            dca_class_2018_avg_price: MetricPattern4::new(client.clone(), format!("{base_path}_dca_class_2018_avg_price")),
            dca_class_2018_returns: MetricPattern4::new(client.clone(), format!("{base_path}_dca_class_2018_returns")),
            dca_class_2018_stack: MetricPattern4::new(client.clone(), format!("{base_path}_dca_class_2018_stack")),
            dca_class_2019_avg_price: MetricPattern4::new(client.clone(), format!("{base_path}_dca_class_2019_avg_price")),
            dca_class_2019_returns: MetricPattern4::new(client.clone(), format!("{base_path}_dca_class_2019_returns")),
            dca_class_2019_stack: MetricPattern4::new(client.clone(), format!("{base_path}_dca_class_2019_stack")),
            dca_class_2020_avg_price: MetricPattern4::new(client.clone(), format!("{base_path}_dca_class_2020_avg_price")),
            dca_class_2020_returns: MetricPattern4::new(client.clone(), format!("{base_path}_dca_class_2020_returns")),
            dca_class_2020_stack: MetricPattern4::new(client.clone(), format!("{base_path}_dca_class_2020_stack")),
            dca_class_2021_avg_price: MetricPattern4::new(client.clone(), format!("{base_path}_dca_class_2021_avg_price")),
            dca_class_2021_returns: MetricPattern4::new(client.clone(), format!("{base_path}_dca_class_2021_returns")),
            dca_class_2021_stack: MetricPattern4::new(client.clone(), format!("{base_path}_dca_class_2021_stack")),
            dca_class_2022_avg_price: MetricPattern4::new(client.clone(), format!("{base_path}_dca_class_2022_avg_price")),
            dca_class_2022_returns: MetricPattern4::new(client.clone(), format!("{base_path}_dca_class_2022_returns")),
            dca_class_2022_stack: MetricPattern4::new(client.clone(), format!("{base_path}_dca_class_2022_stack")),
            dca_class_2023_avg_price: MetricPattern4::new(client.clone(), format!("{base_path}_dca_class_2023_avg_price")),
            dca_class_2023_returns: MetricPattern4::new(client.clone(), format!("{base_path}_dca_class_2023_returns")),
            dca_class_2023_stack: MetricPattern4::new(client.clone(), format!("{base_path}_dca_class_2023_stack")),
            dca_class_2024_avg_price: MetricPattern4::new(client.clone(), format!("{base_path}_dca_class_2024_avg_price")),
            dca_class_2024_returns: MetricPattern4::new(client.clone(), format!("{base_path}_dca_class_2024_returns")),
            dca_class_2024_stack: MetricPattern4::new(client.clone(), format!("{base_path}_dca_class_2024_stack")),
            dca_class_2025_avg_price: MetricPattern4::new(client.clone(), format!("{base_path}_dca_class_2025_avg_price")),
            dca_class_2025_returns: MetricPattern4::new(client.clone(), format!("{base_path}_dca_class_2025_returns")),
            dca_class_2025_stack: MetricPattern4::new(client.clone(), format!("{base_path}_dca_class_2025_stack")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Market_Indicators {
    pub gini: MetricPattern21<StoredF32>,
    pub macd_histogram: MetricPattern21<StoredF32>,
    pub macd_line: MetricPattern21<StoredF32>,
    pub macd_signal: MetricPattern21<StoredF32>,
    pub nvt: MetricPattern21<StoredF32>,
    pub pi_cycle: MetricPattern21<StoredF32>,
    pub puell_multiple: MetricPattern4<StoredF32>,
    pub rsi_14d: MetricPattern21<StoredF32>,
    pub rsi_14d_max: MetricPattern21<StoredF32>,
    pub rsi_14d_min: MetricPattern21<StoredF32>,
    pub rsi_avg_gain_14d: MetricPattern21<StoredF32>,
    pub rsi_avg_loss_14d: MetricPattern21<StoredF32>,
    pub rsi_gains: MetricPattern21<StoredF32>,
    pub rsi_losses: MetricPattern21<StoredF32>,
    pub stoch_d: MetricPattern21<StoredF32>,
    pub stoch_k: MetricPattern21<StoredF32>,
    pub stoch_rsi: MetricPattern21<StoredF32>,
    pub stoch_rsi_d: MetricPattern21<StoredF32>,
    pub stoch_rsi_k: MetricPattern21<StoredF32>,
}

impl CatalogTree_Computed_Market_Indicators {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            gini: MetricPattern21::new(client.clone(), format!("{base_path}_gini")),
            macd_histogram: MetricPattern21::new(client.clone(), format!("{base_path}_macd_histogram")),
            macd_line: MetricPattern21::new(client.clone(), format!("{base_path}_macd_line")),
            macd_signal: MetricPattern21::new(client.clone(), format!("{base_path}_macd_signal")),
            nvt: MetricPattern21::new(client.clone(), format!("{base_path}_nvt")),
            pi_cycle: MetricPattern21::new(client.clone(), format!("{base_path}_pi_cycle")),
            puell_multiple: MetricPattern4::new(client.clone(), format!("{base_path}_puell_multiple")),
            rsi_14d: MetricPattern21::new(client.clone(), format!("{base_path}_rsi_14d")),
            rsi_14d_max: MetricPattern21::new(client.clone(), format!("{base_path}_rsi_14d_max")),
            rsi_14d_min: MetricPattern21::new(client.clone(), format!("{base_path}_rsi_14d_min")),
            rsi_avg_gain_14d: MetricPattern21::new(client.clone(), format!("{base_path}_rsi_avg_gain_14d")),
            rsi_avg_loss_14d: MetricPattern21::new(client.clone(), format!("{base_path}_rsi_avg_loss_14d")),
            rsi_gains: MetricPattern21::new(client.clone(), format!("{base_path}_rsi_gains")),
            rsi_losses: MetricPattern21::new(client.clone(), format!("{base_path}_rsi_losses")),
            stoch_d: MetricPattern21::new(client.clone(), format!("{base_path}_stoch_d")),
            stoch_k: MetricPattern21::new(client.clone(), format!("{base_path}_stoch_k")),
            stoch_rsi: MetricPattern21::new(client.clone(), format!("{base_path}_stoch_rsi")),
            stoch_rsi_d: MetricPattern21::new(client.clone(), format!("{base_path}_stoch_rsi_d")),
            stoch_rsi_k: MetricPattern21::new(client.clone(), format!("{base_path}_stoch_rsi_k")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Market_Lookback {
    pub price_10y_ago: MetricPattern4<Dollars>,
    pub price_1d_ago: MetricPattern4<Dollars>,
    pub price_1m_ago: MetricPattern4<Dollars>,
    pub price_1w_ago: MetricPattern4<Dollars>,
    pub price_1y_ago: MetricPattern4<Dollars>,
    pub price_2y_ago: MetricPattern4<Dollars>,
    pub price_3m_ago: MetricPattern4<Dollars>,
    pub price_3y_ago: MetricPattern4<Dollars>,
    pub price_4y_ago: MetricPattern4<Dollars>,
    pub price_5y_ago: MetricPattern4<Dollars>,
    pub price_6m_ago: MetricPattern4<Dollars>,
    pub price_6y_ago: MetricPattern4<Dollars>,
    pub price_8y_ago: MetricPattern4<Dollars>,
}

impl CatalogTree_Computed_Market_Lookback {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            price_10y_ago: MetricPattern4::new(client.clone(), format!("{base_path}_price_10y_ago")),
            price_1d_ago: MetricPattern4::new(client.clone(), format!("{base_path}_price_1d_ago")),
            price_1m_ago: MetricPattern4::new(client.clone(), format!("{base_path}_price_1m_ago")),
            price_1w_ago: MetricPattern4::new(client.clone(), format!("{base_path}_price_1w_ago")),
            price_1y_ago: MetricPattern4::new(client.clone(), format!("{base_path}_price_1y_ago")),
            price_2y_ago: MetricPattern4::new(client.clone(), format!("{base_path}_price_2y_ago")),
            price_3m_ago: MetricPattern4::new(client.clone(), format!("{base_path}_price_3m_ago")),
            price_3y_ago: MetricPattern4::new(client.clone(), format!("{base_path}_price_3y_ago")),
            price_4y_ago: MetricPattern4::new(client.clone(), format!("{base_path}_price_4y_ago")),
            price_5y_ago: MetricPattern4::new(client.clone(), format!("{base_path}_price_5y_ago")),
            price_6m_ago: MetricPattern4::new(client.clone(), format!("{base_path}_price_6m_ago")),
            price_6y_ago: MetricPattern4::new(client.clone(), format!("{base_path}_price_6y_ago")),
            price_8y_ago: MetricPattern4::new(client.clone(), format!("{base_path}_price_8y_ago")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Market_MovingAverage {
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

impl CatalogTree_Computed_Market_MovingAverage {
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
pub struct CatalogTree_Computed_Market_Range {
    pub price_1m_max: MetricPattern4<Dollars>,
    pub price_1m_min: MetricPattern4<Dollars>,
    pub price_1w_max: MetricPattern4<Dollars>,
    pub price_1w_min: MetricPattern4<Dollars>,
    pub price_1y_max: MetricPattern4<Dollars>,
    pub price_1y_min: MetricPattern4<Dollars>,
    pub price_2w_choppiness_index: MetricPattern4<StoredF32>,
    pub price_2w_max: MetricPattern4<Dollars>,
    pub price_2w_min: MetricPattern4<Dollars>,
    pub price_true_range: MetricPattern21<StoredF32>,
    pub price_true_range_2w_sum: MetricPattern21<StoredF32>,
}

impl CatalogTree_Computed_Market_Range {
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
            price_true_range: MetricPattern21::new(client.clone(), format!("{base_path}_price_true_range")),
            price_true_range_2w_sum: MetricPattern21::new(client.clone(), format!("{base_path}_price_true_range_2w_sum")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Market_Returns {
    pub _1d_returns_1m_sd: _1dReturns1mSdPattern,
    pub _1d_returns_1w_sd: _1dReturns1mSdPattern,
    pub _1d_returns_1y_sd: _1dReturns1mSdPattern,
    pub _10y_cagr: MetricPattern4<StoredF32>,
    pub _10y_price_returns: MetricPattern4<StoredF32>,
    pub _1d_price_returns: MetricPattern4<StoredF32>,
    pub _1m_price_returns: MetricPattern4<StoredF32>,
    pub _1w_price_returns: MetricPattern4<StoredF32>,
    pub _1y_price_returns: MetricPattern4<StoredF32>,
    pub _2y_cagr: MetricPattern4<StoredF32>,
    pub _2y_price_returns: MetricPattern4<StoredF32>,
    pub _3m_price_returns: MetricPattern4<StoredF32>,
    pub _3y_cagr: MetricPattern4<StoredF32>,
    pub _3y_price_returns: MetricPattern4<StoredF32>,
    pub _4y_cagr: MetricPattern4<StoredF32>,
    pub _4y_price_returns: MetricPattern4<StoredF32>,
    pub _5y_cagr: MetricPattern4<StoredF32>,
    pub _5y_price_returns: MetricPattern4<StoredF32>,
    pub _6m_price_returns: MetricPattern4<StoredF32>,
    pub _6y_cagr: MetricPattern4<StoredF32>,
    pub _6y_price_returns: MetricPattern4<StoredF32>,
    pub _8y_cagr: MetricPattern4<StoredF32>,
    pub _8y_price_returns: MetricPattern4<StoredF32>,
    pub downside_1m_sd: _1dReturns1mSdPattern,
    pub downside_1w_sd: _1dReturns1mSdPattern,
    pub downside_1y_sd: _1dReturns1mSdPattern,
    pub downside_returns: MetricPattern21<StoredF32>,
}

impl CatalogTree_Computed_Market_Returns {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _1d_returns_1m_sd: _1dReturns1mSdPattern::new(client.clone(), "1d_returns_1m_sd".to_string()),
            _1d_returns_1w_sd: _1dReturns1mSdPattern::new(client.clone(), "1d_returns_1w_sd".to_string()),
            _1d_returns_1y_sd: _1dReturns1mSdPattern::new(client.clone(), "1d_returns_1y_sd".to_string()),
            _10y_cagr: MetricPattern4::new(client.clone(), format!("{base_path}_10y_cagr")),
            _10y_price_returns: MetricPattern4::new(client.clone(), format!("{base_path}_10y_price_returns")),
            _1d_price_returns: MetricPattern4::new(client.clone(), format!("{base_path}_1d_price_returns")),
            _1m_price_returns: MetricPattern4::new(client.clone(), format!("{base_path}_1m_price_returns")),
            _1w_price_returns: MetricPattern4::new(client.clone(), format!("{base_path}_1w_price_returns")),
            _1y_price_returns: MetricPattern4::new(client.clone(), format!("{base_path}_1y_price_returns")),
            _2y_cagr: MetricPattern4::new(client.clone(), format!("{base_path}_2y_cagr")),
            _2y_price_returns: MetricPattern4::new(client.clone(), format!("{base_path}_2y_price_returns")),
            _3m_price_returns: MetricPattern4::new(client.clone(), format!("{base_path}_3m_price_returns")),
            _3y_cagr: MetricPattern4::new(client.clone(), format!("{base_path}_3y_cagr")),
            _3y_price_returns: MetricPattern4::new(client.clone(), format!("{base_path}_3y_price_returns")),
            _4y_cagr: MetricPattern4::new(client.clone(), format!("{base_path}_4y_cagr")),
            _4y_price_returns: MetricPattern4::new(client.clone(), format!("{base_path}_4y_price_returns")),
            _5y_cagr: MetricPattern4::new(client.clone(), format!("{base_path}_5y_cagr")),
            _5y_price_returns: MetricPattern4::new(client.clone(), format!("{base_path}_5y_price_returns")),
            _6m_price_returns: MetricPattern4::new(client.clone(), format!("{base_path}_6m_price_returns")),
            _6y_cagr: MetricPattern4::new(client.clone(), format!("{base_path}_6y_cagr")),
            _6y_price_returns: MetricPattern4::new(client.clone(), format!("{base_path}_6y_price_returns")),
            _8y_cagr: MetricPattern4::new(client.clone(), format!("{base_path}_8y_cagr")),
            _8y_price_returns: MetricPattern4::new(client.clone(), format!("{base_path}_8y_price_returns")),
            downside_1m_sd: _1dReturns1mSdPattern::new(client.clone(), "downside_1m_sd".to_string()),
            downside_1w_sd: _1dReturns1mSdPattern::new(client.clone(), "downside_1w_sd".to_string()),
            downside_1y_sd: _1dReturns1mSdPattern::new(client.clone(), "downside_1y_sd".to_string()),
            downside_returns: MetricPattern21::new(client.clone(), format!("{base_path}_downside_returns")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Market_Volatility {
    pub price_1m_volatility: MetricPattern4<StoredF32>,
    pub price_1w_volatility: MetricPattern4<StoredF32>,
    pub price_1y_volatility: MetricPattern4<StoredF32>,
    pub sharpe_1m: MetricPattern21<StoredF32>,
    pub sharpe_1w: MetricPattern21<StoredF32>,
    pub sharpe_1y: MetricPattern21<StoredF32>,
    pub sortino_1m: MetricPattern21<StoredF32>,
    pub sortino_1w: MetricPattern21<StoredF32>,
    pub sortino_1y: MetricPattern21<StoredF32>,
}

impl CatalogTree_Computed_Market_Volatility {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            price_1m_volatility: MetricPattern4::new(client.clone(), format!("{base_path}_price_1m_volatility")),
            price_1w_volatility: MetricPattern4::new(client.clone(), format!("{base_path}_price_1w_volatility")),
            price_1y_volatility: MetricPattern4::new(client.clone(), format!("{base_path}_price_1y_volatility")),
            sharpe_1m: MetricPattern21::new(client.clone(), format!("{base_path}_sharpe_1m")),
            sharpe_1w: MetricPattern21::new(client.clone(), format!("{base_path}_sharpe_1w")),
            sharpe_1y: MetricPattern21::new(client.clone(), format!("{base_path}_sharpe_1y")),
            sortino_1m: MetricPattern21::new(client.clone(), format!("{base_path}_sortino_1m")),
            sortino_1w: MetricPattern21::new(client.clone(), format!("{base_path}_sortino_1w")),
            sortino_1y: MetricPattern21::new(client.clone(), format!("{base_path}_sortino_1y")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Outputs {
    pub count: CatalogTree_Computed_Outputs_Count,
    pub spent: CatalogTree_Computed_Outputs_Spent,
}

impl CatalogTree_Computed_Outputs {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            count: CatalogTree_Computed_Outputs_Count::new(client.clone(), format!("{base_path}_count")),
            spent: CatalogTree_Computed_Outputs_Spent::new(client.clone(), format!("{base_path}_spent")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Outputs_Count {
    pub count: BlockSizePattern<StoredU64>,
    pub utxo_count: BitcoinPattern<StoredU64>,
}

impl CatalogTree_Computed_Outputs_Count {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            count: BlockSizePattern::new(client.clone(), "output_count".to_string()),
            utxo_count: BitcoinPattern::new(client.clone(), "exact_utxo_count".to_string()),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Outputs_Spent {
    pub txinindex: MetricPattern29<TxInIndex>,
}

impl CatalogTree_Computed_Outputs_Spent {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            txinindex: MetricPattern29::new(client.clone(), format!("{base_path}_txinindex")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Pools {
    pub pool: MetricPattern25<PoolSlug>,
    pub vecs: CatalogTree_Computed_Pools_Vecs,
}

impl CatalogTree_Computed_Pools {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            pool: MetricPattern25::new(client.clone(), format!("{base_path}_pool")),
            vecs: CatalogTree_Computed_Pools_Vecs::new(client.clone(), format!("{base_path}_vecs")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Pools_Vecs {
    pub axbt: AXbtPattern,
    pub aaopool: AXbtPattern,
    pub antpool: AXbtPattern,
    pub arkpool: AXbtPattern,
    pub asicminer: AXbtPattern,
    pub batpool: AXbtPattern,
    pub bcmonster: AXbtPattern,
    pub bcpoolio: AXbtPattern,
    pub binancepool: AXbtPattern,
    pub bitclub: AXbtPattern,
    pub bitfufupool: AXbtPattern,
    pub bitfury: AXbtPattern,
    pub bitminter: AXbtPattern,
    pub bitalo: AXbtPattern,
    pub bitcoinaffiliatenetwork: AXbtPattern,
    pub bitcoincom: AXbtPattern,
    pub bitcoinindia: AXbtPattern,
    pub bitcoinrussia: AXbtPattern,
    pub bitcoinukraine: AXbtPattern,
    pub bitfarms: AXbtPattern,
    pub bitparking: AXbtPattern,
    pub bitsolo: AXbtPattern,
    pub bixin: AXbtPattern,
    pub blockfills: AXbtPattern,
    pub braiinspool: AXbtPattern,
    pub bravomining: AXbtPattern,
    pub btpool: AXbtPattern,
    pub btccom: AXbtPattern,
    pub btcdig: AXbtPattern,
    pub btcguild: AXbtPattern,
    pub btclab: AXbtPattern,
    pub btcmp: AXbtPattern,
    pub btcnuggets: AXbtPattern,
    pub btcpoolparty: AXbtPattern,
    pub btcserv: AXbtPattern,
    pub btctop: AXbtPattern,
    pub btcc: AXbtPattern,
    pub bwpool: AXbtPattern,
    pub bytepool: AXbtPattern,
    pub canoe: AXbtPattern,
    pub canoepool: AXbtPattern,
    pub carbonnegative: AXbtPattern,
    pub ckpool: AXbtPattern,
    pub cloudhashing: AXbtPattern,
    pub coinlab: AXbtPattern,
    pub cointerra: AXbtPattern,
    pub connectbtc: AXbtPattern,
    pub dpool: AXbtPattern,
    pub dcexploration: AXbtPattern,
    pub dcex: AXbtPattern,
    pub digitalbtc: AXbtPattern,
    pub digitalxmintsy: AXbtPattern,
    pub eclipsemc: AXbtPattern,
    pub eightbaochi: AXbtPattern,
    pub ekanembtc: AXbtPattern,
    pub eligius: AXbtPattern,
    pub emcdpool: AXbtPattern,
    pub entrustcharitypool: AXbtPattern,
    pub eobot: AXbtPattern,
    pub exxbw: AXbtPattern,
    pub f2pool: AXbtPattern,
    pub fiftyeightcoin: AXbtPattern,
    pub foundryusa: AXbtPattern,
    pub futurebitapollosolo: AXbtPattern,
    pub gbminers: AXbtPattern,
    pub ghashio: AXbtPattern,
    pub givemecoins: AXbtPattern,
    pub gogreenlight: AXbtPattern,
    pub haozhuzhu: AXbtPattern,
    pub haominer: AXbtPattern,
    pub hashbx: AXbtPattern,
    pub hashpool: AXbtPattern,
    pub helix: AXbtPattern,
    pub hhtt: AXbtPattern,
    pub hotpool: AXbtPattern,
    pub hummerpool: AXbtPattern,
    pub huobipool: AXbtPattern,
    pub innopolistech: AXbtPattern,
    pub kanopool: AXbtPattern,
    pub kncminer: AXbtPattern,
    pub kucoinpool: AXbtPattern,
    pub lubiancom: AXbtPattern,
    pub luckypool: AXbtPattern,
    pub luxor: AXbtPattern,
    pub marapool: AXbtPattern,
    pub maxbtc: AXbtPattern,
    pub maxipool: AXbtPattern,
    pub megabigpower: AXbtPattern,
    pub minerium: AXbtPattern,
    pub miningcity: AXbtPattern,
    pub miningdutch: AXbtPattern,
    pub miningkings: AXbtPattern,
    pub miningsquared: AXbtPattern,
    pub mmpool: AXbtPattern,
    pub mtred: AXbtPattern,
    pub multicoinco: AXbtPattern,
    pub multipool: AXbtPattern,
    pub mybtccoinpool: AXbtPattern,
    pub neopool: AXbtPattern,
    pub nexious: AXbtPattern,
    pub nicehash: AXbtPattern,
    pub nmcbit: AXbtPattern,
    pub novablock: AXbtPattern,
    pub ocean: AXbtPattern,
    pub okexpool: AXbtPattern,
    pub okminer: AXbtPattern,
    pub okkong: AXbtPattern,
    pub okpooltop: AXbtPattern,
    pub onehash: AXbtPattern,
    pub onem1x: AXbtPattern,
    pub onethash: AXbtPattern,
    pub ozcoin: AXbtPattern,
    pub phashio: AXbtPattern,
    pub parasite: AXbtPattern,
    pub patels: AXbtPattern,
    pub pegapool: AXbtPattern,
    pub phoenix: AXbtPattern,
    pub polmine: AXbtPattern,
    pub pool175btc: AXbtPattern,
    pub pool50btc: AXbtPattern,
    pub poolin: AXbtPattern,
    pub portlandhodl: AXbtPattern,
    pub publicpool: AXbtPattern,
    pub purebtccom: AXbtPattern,
    pub rawpool: AXbtPattern,
    pub rigpool: AXbtPattern,
    pub sbicrypto: AXbtPattern,
    pub secpool: AXbtPattern,
    pub secretsuperstar: AXbtPattern,
    pub sevenpool: AXbtPattern,
    pub shawnp0wers: AXbtPattern,
    pub sigmapoolcom: AXbtPattern,
    pub simplecoinus: AXbtPattern,
    pub solock: AXbtPattern,
    pub spiderpool: AXbtPattern,
    pub stminingcorp: AXbtPattern,
    pub tangpool: AXbtPattern,
    pub tatmaspool: AXbtPattern,
    pub tbdice: AXbtPattern,
    pub telco214: AXbtPattern,
    pub terrapool: AXbtPattern,
    pub tiger: AXbtPattern,
    pub tigerpoolnet: AXbtPattern,
    pub titan: AXbtPattern,
    pub transactioncoinmining: AXbtPattern,
    pub trickysbtcpool: AXbtPattern,
    pub triplemining: AXbtPattern,
    pub twentyoneinc: AXbtPattern,
    pub ultimuspool: AXbtPattern,
    pub unknown: AXbtPattern,
    pub unomp: AXbtPattern,
    pub viabtc: AXbtPattern,
    pub waterhole: AXbtPattern,
    pub wayicn: AXbtPattern,
    pub whitepool: AXbtPattern,
    pub wk057: AXbtPattern,
    pub yourbtcnet: AXbtPattern,
    pub zulupool: AXbtPattern,
}

impl CatalogTree_Computed_Pools_Vecs {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            axbt: AXbtPattern::new(client.clone(), "axbt".to_string()),
            aaopool: AXbtPattern::new(client.clone(), "aaopool".to_string()),
            antpool: AXbtPattern::new(client.clone(), "antpool".to_string()),
            arkpool: AXbtPattern::new(client.clone(), "arkpool".to_string()),
            asicminer: AXbtPattern::new(client.clone(), "asicminer".to_string()),
            batpool: AXbtPattern::new(client.clone(), "batpool".to_string()),
            bcmonster: AXbtPattern::new(client.clone(), "bcmonster".to_string()),
            bcpoolio: AXbtPattern::new(client.clone(), "bcpoolio".to_string()),
            binancepool: AXbtPattern::new(client.clone(), "binancepool".to_string()),
            bitclub: AXbtPattern::new(client.clone(), "bitclub".to_string()),
            bitfufupool: AXbtPattern::new(client.clone(), "bitfufupool".to_string()),
            bitfury: AXbtPattern::new(client.clone(), "bitfury".to_string()),
            bitminter: AXbtPattern::new(client.clone(), "bitminter".to_string()),
            bitalo: AXbtPattern::new(client.clone(), "bitalo".to_string()),
            bitcoinaffiliatenetwork: AXbtPattern::new(client.clone(), "bitcoinaffiliatenetwork".to_string()),
            bitcoincom: AXbtPattern::new(client.clone(), "bitcoincom".to_string()),
            bitcoinindia: AXbtPattern::new(client.clone(), "bitcoinindia".to_string()),
            bitcoinrussia: AXbtPattern::new(client.clone(), "bitcoinrussia".to_string()),
            bitcoinukraine: AXbtPattern::new(client.clone(), "bitcoinukraine".to_string()),
            bitfarms: AXbtPattern::new(client.clone(), "bitfarms".to_string()),
            bitparking: AXbtPattern::new(client.clone(), "bitparking".to_string()),
            bitsolo: AXbtPattern::new(client.clone(), "bitsolo".to_string()),
            bixin: AXbtPattern::new(client.clone(), "bixin".to_string()),
            blockfills: AXbtPattern::new(client.clone(), "blockfills".to_string()),
            braiinspool: AXbtPattern::new(client.clone(), "braiinspool".to_string()),
            bravomining: AXbtPattern::new(client.clone(), "bravomining".to_string()),
            btpool: AXbtPattern::new(client.clone(), "btpool".to_string()),
            btccom: AXbtPattern::new(client.clone(), "btccom".to_string()),
            btcdig: AXbtPattern::new(client.clone(), "btcdig".to_string()),
            btcguild: AXbtPattern::new(client.clone(), "btcguild".to_string()),
            btclab: AXbtPattern::new(client.clone(), "btclab".to_string()),
            btcmp: AXbtPattern::new(client.clone(), "btcmp".to_string()),
            btcnuggets: AXbtPattern::new(client.clone(), "btcnuggets".to_string()),
            btcpoolparty: AXbtPattern::new(client.clone(), "btcpoolparty".to_string()),
            btcserv: AXbtPattern::new(client.clone(), "btcserv".to_string()),
            btctop: AXbtPattern::new(client.clone(), "btctop".to_string()),
            btcc: AXbtPattern::new(client.clone(), "btcc".to_string()),
            bwpool: AXbtPattern::new(client.clone(), "bwpool".to_string()),
            bytepool: AXbtPattern::new(client.clone(), "bytepool".to_string()),
            canoe: AXbtPattern::new(client.clone(), "canoe".to_string()),
            canoepool: AXbtPattern::new(client.clone(), "canoepool".to_string()),
            carbonnegative: AXbtPattern::new(client.clone(), "carbonnegative".to_string()),
            ckpool: AXbtPattern::new(client.clone(), "ckpool".to_string()),
            cloudhashing: AXbtPattern::new(client.clone(), "cloudhashing".to_string()),
            coinlab: AXbtPattern::new(client.clone(), "coinlab".to_string()),
            cointerra: AXbtPattern::new(client.clone(), "cointerra".to_string()),
            connectbtc: AXbtPattern::new(client.clone(), "connectbtc".to_string()),
            dpool: AXbtPattern::new(client.clone(), "dpool".to_string()),
            dcexploration: AXbtPattern::new(client.clone(), "dcexploration".to_string()),
            dcex: AXbtPattern::new(client.clone(), "dcex".to_string()),
            digitalbtc: AXbtPattern::new(client.clone(), "digitalbtc".to_string()),
            digitalxmintsy: AXbtPattern::new(client.clone(), "digitalxmintsy".to_string()),
            eclipsemc: AXbtPattern::new(client.clone(), "eclipsemc".to_string()),
            eightbaochi: AXbtPattern::new(client.clone(), "eightbaochi".to_string()),
            ekanembtc: AXbtPattern::new(client.clone(), "ekanembtc".to_string()),
            eligius: AXbtPattern::new(client.clone(), "eligius".to_string()),
            emcdpool: AXbtPattern::new(client.clone(), "emcdpool".to_string()),
            entrustcharitypool: AXbtPattern::new(client.clone(), "entrustcharitypool".to_string()),
            eobot: AXbtPattern::new(client.clone(), "eobot".to_string()),
            exxbw: AXbtPattern::new(client.clone(), "exxbw".to_string()),
            f2pool: AXbtPattern::new(client.clone(), "f2pool".to_string()),
            fiftyeightcoin: AXbtPattern::new(client.clone(), "fiftyeightcoin".to_string()),
            foundryusa: AXbtPattern::new(client.clone(), "foundryusa".to_string()),
            futurebitapollosolo: AXbtPattern::new(client.clone(), "futurebitapollosolo".to_string()),
            gbminers: AXbtPattern::new(client.clone(), "gbminers".to_string()),
            ghashio: AXbtPattern::new(client.clone(), "ghashio".to_string()),
            givemecoins: AXbtPattern::new(client.clone(), "givemecoins".to_string()),
            gogreenlight: AXbtPattern::new(client.clone(), "gogreenlight".to_string()),
            haozhuzhu: AXbtPattern::new(client.clone(), "haozhuzhu".to_string()),
            haominer: AXbtPattern::new(client.clone(), "haominer".to_string()),
            hashbx: AXbtPattern::new(client.clone(), "hashbx".to_string()),
            hashpool: AXbtPattern::new(client.clone(), "hashpool".to_string()),
            helix: AXbtPattern::new(client.clone(), "helix".to_string()),
            hhtt: AXbtPattern::new(client.clone(), "hhtt".to_string()),
            hotpool: AXbtPattern::new(client.clone(), "hotpool".to_string()),
            hummerpool: AXbtPattern::new(client.clone(), "hummerpool".to_string()),
            huobipool: AXbtPattern::new(client.clone(), "huobipool".to_string()),
            innopolistech: AXbtPattern::new(client.clone(), "innopolistech".to_string()),
            kanopool: AXbtPattern::new(client.clone(), "kanopool".to_string()),
            kncminer: AXbtPattern::new(client.clone(), "kncminer".to_string()),
            kucoinpool: AXbtPattern::new(client.clone(), "kucoinpool".to_string()),
            lubiancom: AXbtPattern::new(client.clone(), "lubiancom".to_string()),
            luckypool: AXbtPattern::new(client.clone(), "luckypool".to_string()),
            luxor: AXbtPattern::new(client.clone(), "luxor".to_string()),
            marapool: AXbtPattern::new(client.clone(), "marapool".to_string()),
            maxbtc: AXbtPattern::new(client.clone(), "maxbtc".to_string()),
            maxipool: AXbtPattern::new(client.clone(), "maxipool".to_string()),
            megabigpower: AXbtPattern::new(client.clone(), "megabigpower".to_string()),
            minerium: AXbtPattern::new(client.clone(), "minerium".to_string()),
            miningcity: AXbtPattern::new(client.clone(), "miningcity".to_string()),
            miningdutch: AXbtPattern::new(client.clone(), "miningdutch".to_string()),
            miningkings: AXbtPattern::new(client.clone(), "miningkings".to_string()),
            miningsquared: AXbtPattern::new(client.clone(), "miningsquared".to_string()),
            mmpool: AXbtPattern::new(client.clone(), "mmpool".to_string()),
            mtred: AXbtPattern::new(client.clone(), "mtred".to_string()),
            multicoinco: AXbtPattern::new(client.clone(), "multicoinco".to_string()),
            multipool: AXbtPattern::new(client.clone(), "multipool".to_string()),
            mybtccoinpool: AXbtPattern::new(client.clone(), "mybtccoinpool".to_string()),
            neopool: AXbtPattern::new(client.clone(), "neopool".to_string()),
            nexious: AXbtPattern::new(client.clone(), "nexious".to_string()),
            nicehash: AXbtPattern::new(client.clone(), "nicehash".to_string()),
            nmcbit: AXbtPattern::new(client.clone(), "nmcbit".to_string()),
            novablock: AXbtPattern::new(client.clone(), "novablock".to_string()),
            ocean: AXbtPattern::new(client.clone(), "ocean".to_string()),
            okexpool: AXbtPattern::new(client.clone(), "okexpool".to_string()),
            okminer: AXbtPattern::new(client.clone(), "okminer".to_string()),
            okkong: AXbtPattern::new(client.clone(), "okkong".to_string()),
            okpooltop: AXbtPattern::new(client.clone(), "okpooltop".to_string()),
            onehash: AXbtPattern::new(client.clone(), "onehash".to_string()),
            onem1x: AXbtPattern::new(client.clone(), "onem1x".to_string()),
            onethash: AXbtPattern::new(client.clone(), "onethash".to_string()),
            ozcoin: AXbtPattern::new(client.clone(), "ozcoin".to_string()),
            phashio: AXbtPattern::new(client.clone(), "phashio".to_string()),
            parasite: AXbtPattern::new(client.clone(), "parasite".to_string()),
            patels: AXbtPattern::new(client.clone(), "patels".to_string()),
            pegapool: AXbtPattern::new(client.clone(), "pegapool".to_string()),
            phoenix: AXbtPattern::new(client.clone(), "phoenix".to_string()),
            polmine: AXbtPattern::new(client.clone(), "polmine".to_string()),
            pool175btc: AXbtPattern::new(client.clone(), "pool175btc".to_string()),
            pool50btc: AXbtPattern::new(client.clone(), "pool50btc".to_string()),
            poolin: AXbtPattern::new(client.clone(), "poolin".to_string()),
            portlandhodl: AXbtPattern::new(client.clone(), "portlandhodl".to_string()),
            publicpool: AXbtPattern::new(client.clone(), "publicpool".to_string()),
            purebtccom: AXbtPattern::new(client.clone(), "purebtccom".to_string()),
            rawpool: AXbtPattern::new(client.clone(), "rawpool".to_string()),
            rigpool: AXbtPattern::new(client.clone(), "rigpool".to_string()),
            sbicrypto: AXbtPattern::new(client.clone(), "sbicrypto".to_string()),
            secpool: AXbtPattern::new(client.clone(), "secpool".to_string()),
            secretsuperstar: AXbtPattern::new(client.clone(), "secretsuperstar".to_string()),
            sevenpool: AXbtPattern::new(client.clone(), "sevenpool".to_string()),
            shawnp0wers: AXbtPattern::new(client.clone(), "shawnp0wers".to_string()),
            sigmapoolcom: AXbtPattern::new(client.clone(), "sigmapoolcom".to_string()),
            simplecoinus: AXbtPattern::new(client.clone(), "simplecoinus".to_string()),
            solock: AXbtPattern::new(client.clone(), "solock".to_string()),
            spiderpool: AXbtPattern::new(client.clone(), "spiderpool".to_string()),
            stminingcorp: AXbtPattern::new(client.clone(), "stminingcorp".to_string()),
            tangpool: AXbtPattern::new(client.clone(), "tangpool".to_string()),
            tatmaspool: AXbtPattern::new(client.clone(), "tatmaspool".to_string()),
            tbdice: AXbtPattern::new(client.clone(), "tbdice".to_string()),
            telco214: AXbtPattern::new(client.clone(), "telco214".to_string()),
            terrapool: AXbtPattern::new(client.clone(), "terrapool".to_string()),
            tiger: AXbtPattern::new(client.clone(), "tiger".to_string()),
            tigerpoolnet: AXbtPattern::new(client.clone(), "tigerpoolnet".to_string()),
            titan: AXbtPattern::new(client.clone(), "titan".to_string()),
            transactioncoinmining: AXbtPattern::new(client.clone(), "transactioncoinmining".to_string()),
            trickysbtcpool: AXbtPattern::new(client.clone(), "trickysbtcpool".to_string()),
            triplemining: AXbtPattern::new(client.clone(), "triplemining".to_string()),
            twentyoneinc: AXbtPattern::new(client.clone(), "twentyoneinc".to_string()),
            ultimuspool: AXbtPattern::new(client.clone(), "ultimuspool".to_string()),
            unknown: AXbtPattern::new(client.clone(), "unknown".to_string()),
            unomp: AXbtPattern::new(client.clone(), "unomp".to_string()),
            viabtc: AXbtPattern::new(client.clone(), "viabtc".to_string()),
            waterhole: AXbtPattern::new(client.clone(), "waterhole".to_string()),
            wayicn: AXbtPattern::new(client.clone(), "wayicn".to_string()),
            whitepool: AXbtPattern::new(client.clone(), "whitepool".to_string()),
            wk057: AXbtPattern::new(client.clone(), "wk057".to_string()),
            yourbtcnet: AXbtPattern::new(client.clone(), "yourbtcnet".to_string()),
            zulupool: AXbtPattern::new(client.clone(), "zulupool".to_string()),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Positions {
    pub position: MetricPattern16<BlkPosition>,
}

impl CatalogTree_Computed_Positions {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            position: MetricPattern16::new(client.clone(), format!("{base_path}_position")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Price {
    pub ohlc: CatalogTree_Computed_Price_Ohlc,
    pub sats: CatalogTree_Computed_Price_Sats,
    pub usd: CatalogTree_Computed_Price_Usd,
}

impl CatalogTree_Computed_Price {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            ohlc: CatalogTree_Computed_Price_Ohlc::new(client.clone(), format!("{base_path}_ohlc")),
            sats: CatalogTree_Computed_Price_Sats::new(client.clone(), format!("{base_path}_sats")),
            usd: CatalogTree_Computed_Price_Usd::new(client.clone(), format!("{base_path}_usd")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Price_Ohlc {
    pub ohlc_in_cents: MetricPattern9<OHLCCents>,
}

impl CatalogTree_Computed_Price_Ohlc {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            ohlc_in_cents: MetricPattern9::new(client.clone(), format!("{base_path}_ohlc_in_cents")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Price_Sats {
    pub price_close_in_sats: MetricPattern1<Sats>,
    pub price_high_in_sats: MetricPattern1<Sats>,
    pub price_low_in_sats: MetricPattern1<Sats>,
    pub price_ohlc_in_sats: MetricPattern1<OHLCSats>,
    pub price_open_in_sats: MetricPattern1<Sats>,
}

impl CatalogTree_Computed_Price_Sats {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            price_close_in_sats: MetricPattern1::new(client.clone(), format!("{base_path}_price_close_in_sats")),
            price_high_in_sats: MetricPattern1::new(client.clone(), format!("{base_path}_price_high_in_sats")),
            price_low_in_sats: MetricPattern1::new(client.clone(), format!("{base_path}_price_low_in_sats")),
            price_ohlc_in_sats: MetricPattern1::new(client.clone(), format!("{base_path}_price_ohlc_in_sats")),
            price_open_in_sats: MetricPattern1::new(client.clone(), format!("{base_path}_price_open_in_sats")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Price_Usd {
    pub price_close: MetricPattern1<Dollars>,
    pub price_close_in_cents: MetricPattern9<Cents>,
    pub price_high: MetricPattern1<Dollars>,
    pub price_high_in_cents: MetricPattern9<Cents>,
    pub price_low: MetricPattern1<Dollars>,
    pub price_low_in_cents: MetricPattern9<Cents>,
    pub price_ohlc: MetricPattern1<OHLCDollars>,
    pub price_open: MetricPattern1<Dollars>,
    pub price_open_in_cents: MetricPattern9<Cents>,
}

impl CatalogTree_Computed_Price_Usd {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            price_close: MetricPattern1::new(client.clone(), format!("{base_path}_price_close")),
            price_close_in_cents: MetricPattern9::new(client.clone(), format!("{base_path}_price_close_in_cents")),
            price_high: MetricPattern1::new(client.clone(), format!("{base_path}_price_high")),
            price_high_in_cents: MetricPattern9::new(client.clone(), format!("{base_path}_price_high_in_cents")),
            price_low: MetricPattern1::new(client.clone(), format!("{base_path}_price_low")),
            price_low_in_cents: MetricPattern9::new(client.clone(), format!("{base_path}_price_low_in_cents")),
            price_ohlc: MetricPattern1::new(client.clone(), format!("{base_path}_price_ohlc")),
            price_open: MetricPattern1::new(client.clone(), format!("{base_path}_price_open")),
            price_open_in_cents: MetricPattern9::new(client.clone(), format!("{base_path}_price_open_in_cents")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Scripts {
    pub count: CatalogTree_Computed_Scripts_Count,
    pub value: CatalogTree_Computed_Scripts_Value,
}

impl CatalogTree_Computed_Scripts {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            count: CatalogTree_Computed_Scripts_Count::new(client.clone(), format!("{base_path}_count")),
            value: CatalogTree_Computed_Scripts_Value::new(client.clone(), format!("{base_path}_value")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Scripts_Count {
    pub emptyoutput_count: BitcoinPattern<StoredU64>,
    pub opreturn_count: BitcoinPattern<StoredU64>,
    pub p2a_count: BitcoinPattern<StoredU64>,
    pub p2ms_count: BitcoinPattern<StoredU64>,
    pub p2pk33_count: BitcoinPattern<StoredU64>,
    pub p2pk65_count: BitcoinPattern<StoredU64>,
    pub p2pkh_count: BitcoinPattern<StoredU64>,
    pub p2sh_count: BitcoinPattern<StoredU64>,
    pub p2tr_count: BitcoinPattern<StoredU64>,
    pub p2wpkh_count: BitcoinPattern<StoredU64>,
    pub p2wsh_count: BitcoinPattern<StoredU64>,
    pub segwit_adoption: SegwitAdoptionPattern<StoredF32>,
    pub segwit_count: BitcoinPattern<StoredU64>,
    pub taproot_adoption: SegwitAdoptionPattern<StoredF32>,
    pub unknownoutput_count: BitcoinPattern<StoredU64>,
}

impl CatalogTree_Computed_Scripts_Count {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            emptyoutput_count: BitcoinPattern::new(client.clone(), "emptyoutput_count".to_string()),
            opreturn_count: BitcoinPattern::new(client.clone(), "opreturn_count".to_string()),
            p2a_count: BitcoinPattern::new(client.clone(), "p2a_count".to_string()),
            p2ms_count: BitcoinPattern::new(client.clone(), "p2ms_count".to_string()),
            p2pk33_count: BitcoinPattern::new(client.clone(), "p2pk33_count".to_string()),
            p2pk65_count: BitcoinPattern::new(client.clone(), "p2pk65_count".to_string()),
            p2pkh_count: BitcoinPattern::new(client.clone(), "p2pkh_count".to_string()),
            p2sh_count: BitcoinPattern::new(client.clone(), "p2sh_count".to_string()),
            p2tr_count: BitcoinPattern::new(client.clone(), "p2tr_count".to_string()),
            p2wpkh_count: BitcoinPattern::new(client.clone(), "p2wpkh_count".to_string()),
            p2wsh_count: BitcoinPattern::new(client.clone(), "p2wsh_count".to_string()),
            segwit_adoption: SegwitAdoptionPattern::new(client.clone(), "segwit_adoption".to_string()),
            segwit_count: BitcoinPattern::new(client.clone(), "segwit_count".to_string()),
            taproot_adoption: SegwitAdoptionPattern::new(client.clone(), "taproot_adoption".to_string()),
            unknownoutput_count: BitcoinPattern::new(client.clone(), "unknownoutput_count".to_string()),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Scripts_Value {
    pub opreturn_value: CatalogTree_Computed_Scripts_Value_OpreturnValue,
}

impl CatalogTree_Computed_Scripts_Value {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            opreturn_value: CatalogTree_Computed_Scripts_Value_OpreturnValue::new(client.clone(), format!("{base_path}_opreturn_value")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Scripts_Value_OpreturnValue {
    pub base: MetricPattern25<Sats>,
    pub bitcoin: SegwitAdoptionPattern<Bitcoin>,
    pub dollars: SegwitAdoptionPattern<Dollars>,
    pub sats: CatalogTree_Computed_Scripts_Value_OpreturnValue_Sats,
}

impl CatalogTree_Computed_Scripts_Value_OpreturnValue {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            base: MetricPattern25::new(client.clone(), format!("{base_path}_base")),
            bitcoin: SegwitAdoptionPattern::new(client.clone(), "opreturn_value_btc".to_string()),
            dollars: SegwitAdoptionPattern::new(client.clone(), "opreturn_value_usd".to_string()),
            sats: CatalogTree_Computed_Scripts_Value_OpreturnValue_Sats::new(client.clone(), format!("{base_path}_sats")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Scripts_Value_OpreturnValue_Sats {
    pub average: MetricPattern2<Sats>,
    pub cumulative: MetricPattern1<Sats>,
    pub max: MetricPattern2<Sats>,
    pub min: MetricPattern2<Sats>,
    pub sum: MetricPattern2<Sats>,
}

impl CatalogTree_Computed_Scripts_Value_OpreturnValue_Sats {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            average: MetricPattern2::new(client.clone(), format!("{base_path}_average")),
            cumulative: MetricPattern1::new(client.clone(), format!("{base_path}_cumulative")),
            max: MetricPattern2::new(client.clone(), format!("{base_path}_max")),
            min: MetricPattern2::new(client.clone(), format!("{base_path}_min")),
            sum: MetricPattern2::new(client.clone(), format!("{base_path}_sum")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Supply {
    pub burned: CatalogTree_Computed_Supply_Burned,
    pub circulating: CatalogTree_Computed_Supply_Circulating,
    pub inflation: CatalogTree_Computed_Supply_Inflation,
    pub market_cap: CatalogTree_Computed_Supply_MarketCap,
    pub velocity: CatalogTree_Computed_Supply_Velocity,
}

impl CatalogTree_Computed_Supply {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            burned: CatalogTree_Computed_Supply_Burned::new(client.clone(), format!("{base_path}_burned")),
            circulating: CatalogTree_Computed_Supply_Circulating::new(client.clone(), format!("{base_path}_circulating")),
            inflation: CatalogTree_Computed_Supply_Inflation::new(client.clone(), format!("{base_path}_inflation")),
            market_cap: CatalogTree_Computed_Supply_MarketCap::new(client.clone(), format!("{base_path}_market_cap")),
            velocity: CatalogTree_Computed_Supply_Velocity::new(client.clone(), format!("{base_path}_velocity")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Supply_Burned {
    pub opreturn: OpreturnPattern,
    pub unspendable: OpreturnPattern,
}

impl CatalogTree_Computed_Supply_Burned {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            opreturn: OpreturnPattern::new(client.clone(), "opreturn_supply".to_string()),
            unspendable: OpreturnPattern::new(client.clone(), "unspendable_supply".to_string()),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Supply_Circulating {
    pub btc: MetricPattern25<Bitcoin>,
    pub indexes: ActiveSupplyPattern,
    pub sats: MetricPattern25<Sats>,
    pub usd: MetricPattern25<Dollars>,
}

impl CatalogTree_Computed_Supply_Circulating {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            btc: MetricPattern25::new(client.clone(), format!("{base_path}_btc")),
            indexes: ActiveSupplyPattern::new(client.clone(), "circulating".to_string()),
            sats: MetricPattern25::new(client.clone(), format!("{base_path}_sats")),
            usd: MetricPattern25::new(client.clone(), format!("{base_path}_usd")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Supply_Inflation {
    pub indexes: MetricPattern4<StoredF32>,
}

impl CatalogTree_Computed_Supply_Inflation {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            indexes: MetricPattern4::new(client.clone(), format!("{base_path}_indexes")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Supply_MarketCap {
    pub height: MetricPattern25<Dollars>,
    pub indexes: MetricPattern4<Dollars>,
}

impl CatalogTree_Computed_Supply_MarketCap {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            height: MetricPattern25::new(client.clone(), format!("{base_path}_height")),
            indexes: MetricPattern4::new(client.clone(), format!("{base_path}_indexes")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Supply_Velocity {
    pub btc: MetricPattern4<StoredF64>,
    pub usd: MetricPattern4<StoredF64>,
}

impl CatalogTree_Computed_Supply_Velocity {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            btc: MetricPattern4::new(client.clone(), format!("{base_path}_btc")),
            usd: MetricPattern4::new(client.clone(), format!("{base_path}_usd")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Transactions {
    pub count: CatalogTree_Computed_Transactions_Count,
    pub fees: CatalogTree_Computed_Transactions_Fees,
    pub size: CatalogTree_Computed_Transactions_Size,
    pub versions: CatalogTree_Computed_Transactions_Versions,
    pub volume: CatalogTree_Computed_Transactions_Volume,
}

impl CatalogTree_Computed_Transactions {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            count: CatalogTree_Computed_Transactions_Count::new(client.clone(), format!("{base_path}_count")),
            fees: CatalogTree_Computed_Transactions_Fees::new(client.clone(), format!("{base_path}_fees")),
            size: CatalogTree_Computed_Transactions_Size::new(client.clone(), format!("{base_path}_size")),
            versions: CatalogTree_Computed_Transactions_Versions::new(client.clone(), format!("{base_path}_versions")),
            volume: CatalogTree_Computed_Transactions_Volume::new(client.clone(), format!("{base_path}_volume")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Transactions_Count {
    pub is_coinbase: MetricPattern41<StoredBool>,
    pub tx_count: BitcoinPattern<StoredU64>,
}

impl CatalogTree_Computed_Transactions_Count {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            is_coinbase: MetricPattern41::new(client.clone(), format!("{base_path}_is_coinbase")),
            tx_count: BitcoinPattern::new(client.clone(), "tx_count".to_string()),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Transactions_Fees {
    pub fee: CatalogTree_Computed_Transactions_Fees_Fee,
    pub fee_rate: CatalogTree_Computed_Transactions_Fees_FeeRate,
    pub input_value: MetricPattern41<Sats>,
    pub output_value: MetricPattern41<Sats>,
}

impl CatalogTree_Computed_Transactions_Fees {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            fee: CatalogTree_Computed_Transactions_Fees_Fee::new(client.clone(), format!("{base_path}_fee")),
            fee_rate: CatalogTree_Computed_Transactions_Fees_FeeRate::new(client.clone(), format!("{base_path}_fee_rate")),
            input_value: MetricPattern41::new(client.clone(), format!("{base_path}_input_value")),
            output_value: MetricPattern41::new(client.clone(), format!("{base_path}_output_value")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Transactions_Fees_Fee {
    pub base: MetricPattern41<Sats>,
    pub bitcoin: BlockSizePattern<Bitcoin>,
    pub bitcoin_txindex: MetricPattern41<Bitcoin>,
    pub dollars: BlockSizePattern<Dollars>,
    pub dollars_txindex: MetricPattern41<Dollars>,
    pub sats: BlockSizePattern<Sats>,
}

impl CatalogTree_Computed_Transactions_Fees_Fee {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            base: MetricPattern41::new(client.clone(), format!("{base_path}_base")),
            bitcoin: BlockSizePattern::new(client.clone(), "fee_btc".to_string()),
            bitcoin_txindex: MetricPattern41::new(client.clone(), format!("{base_path}_bitcoin_txindex")),
            dollars: BlockSizePattern::new(client.clone(), "fee_usd".to_string()),
            dollars_txindex: MetricPattern41::new(client.clone(), format!("{base_path}_dollars_txindex")),
            sats: BlockSizePattern::new(client.clone(), "fee".to_string()),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Transactions_Fees_FeeRate {
    pub average: MetricPattern1<FeeRate>,
    pub base: MetricPattern41<FeeRate>,
    pub max: MetricPattern1<FeeRate>,
    pub median: MetricPattern25<FeeRate>,
    pub min: MetricPattern1<FeeRate>,
    pub pct10: MetricPattern25<FeeRate>,
    pub pct25: MetricPattern25<FeeRate>,
    pub pct75: MetricPattern25<FeeRate>,
    pub pct90: MetricPattern25<FeeRate>,
}

impl CatalogTree_Computed_Transactions_Fees_FeeRate {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            average: MetricPattern1::new(client.clone(), format!("{base_path}_average")),
            base: MetricPattern41::new(client.clone(), format!("{base_path}_base")),
            max: MetricPattern1::new(client.clone(), format!("{base_path}_max")),
            median: MetricPattern25::new(client.clone(), format!("{base_path}_median")),
            min: MetricPattern1::new(client.clone(), format!("{base_path}_min")),
            pct10: MetricPattern25::new(client.clone(), format!("{base_path}_pct10")),
            pct25: MetricPattern25::new(client.clone(), format!("{base_path}_pct25")),
            pct75: MetricPattern25::new(client.clone(), format!("{base_path}_pct75")),
            pct90: MetricPattern25::new(client.clone(), format!("{base_path}_pct90")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Transactions_Size {
    pub tx_vsize: BlockIntervalPattern<VSize>,
    pub tx_weight: BlockIntervalPattern<Weight>,
    pub vsize: MetricPattern41<VSize>,
    pub weight: MetricPattern41<Weight>,
}

impl CatalogTree_Computed_Transactions_Size {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            tx_vsize: BlockIntervalPattern::new(client.clone(), "tx_vsize".to_string()),
            tx_weight: BlockIntervalPattern::new(client.clone(), "tx_weight".to_string()),
            vsize: MetricPattern41::new(client.clone(), format!("{base_path}_vsize")),
            weight: MetricPattern41::new(client.clone(), format!("{base_path}_weight")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Transactions_Versions {
    pub tx_v1: BlockCountPattern<StoredU64>,
    pub tx_v2: BlockCountPattern<StoredU64>,
    pub tx_v3: BlockCountPattern<StoredU64>,
}

impl CatalogTree_Computed_Transactions_Versions {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            tx_v1: BlockCountPattern::new(client.clone(), "tx_v1".to_string()),
            tx_v2: BlockCountPattern::new(client.clone(), "tx_v2".to_string()),
            tx_v3: BlockCountPattern::new(client.clone(), "tx_v3".to_string()),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Transactions_Volume {
    pub annualized_volume: MetricPattern4<Sats>,
    pub annualized_volume_btc: MetricPattern4<Bitcoin>,
    pub annualized_volume_usd: MetricPattern4<Dollars>,
    pub inputs_per_sec: MetricPattern4<StoredF32>,
    pub outputs_per_sec: MetricPattern4<StoredF32>,
    pub sent_sum: CatalogTree_Computed_Transactions_Volume_SentSum,
    pub tx_per_sec: MetricPattern4<StoredF32>,
}

impl CatalogTree_Computed_Transactions_Volume {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            annualized_volume: MetricPattern4::new(client.clone(), format!("{base_path}_annualized_volume")),
            annualized_volume_btc: MetricPattern4::new(client.clone(), format!("{base_path}_annualized_volume_btc")),
            annualized_volume_usd: MetricPattern4::new(client.clone(), format!("{base_path}_annualized_volume_usd")),
            inputs_per_sec: MetricPattern4::new(client.clone(), format!("{base_path}_inputs_per_sec")),
            outputs_per_sec: MetricPattern4::new(client.clone(), format!("{base_path}_outputs_per_sec")),
            sent_sum: CatalogTree_Computed_Transactions_Volume_SentSum::new(client.clone(), format!("{base_path}_sent_sum")),
            tx_per_sec: MetricPattern4::new(client.clone(), format!("{base_path}_tx_per_sec")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Transactions_Volume_SentSum {
    pub bitcoin: TotalRealizedPnlPattern<Bitcoin>,
    pub dollars: MetricPattern1<Dollars>,
    pub sats: MetricPattern1<Sats>,
}

impl CatalogTree_Computed_Transactions_Volume_SentSum {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            bitcoin: TotalRealizedPnlPattern::new(client.clone(), "sent_sum_btc".to_string()),
            dollars: MetricPattern1::new(client.clone(), format!("{base_path}_dollars")),
            sats: MetricPattern1::new(client.clone(), format!("{base_path}_sats")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Indexed {
    pub address: CatalogTree_Indexed_Address,
    pub block: CatalogTree_Indexed_Block,
    pub output: CatalogTree_Indexed_Output,
    pub tx: CatalogTree_Indexed_Tx,
    pub txin: CatalogTree_Indexed_Txin,
    pub txout: CatalogTree_Indexed_Txout,
}

impl CatalogTree_Indexed {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            address: CatalogTree_Indexed_Address::new(client.clone(), format!("{base_path}_address")),
            block: CatalogTree_Indexed_Block::new(client.clone(), format!("{base_path}_block")),
            output: CatalogTree_Indexed_Output::new(client.clone(), format!("{base_path}_output")),
            tx: CatalogTree_Indexed_Tx::new(client.clone(), format!("{base_path}_tx")),
            txin: CatalogTree_Indexed_Txin::new(client.clone(), format!("{base_path}_txin")),
            txout: CatalogTree_Indexed_Txout::new(client.clone(), format!("{base_path}_txout")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Indexed_Address {
    pub first_p2aaddressindex: MetricPattern25<P2AAddressIndex>,
    pub first_p2pk33addressindex: MetricPattern25<P2PK33AddressIndex>,
    pub first_p2pk65addressindex: MetricPattern25<P2PK65AddressIndex>,
    pub first_p2pkhaddressindex: MetricPattern25<P2PKHAddressIndex>,
    pub first_p2shaddressindex: MetricPattern25<P2SHAddressIndex>,
    pub first_p2traddressindex: MetricPattern25<P2TRAddressIndex>,
    pub first_p2wpkhaddressindex: MetricPattern25<P2WPKHAddressIndex>,
    pub first_p2wshaddressindex: MetricPattern25<P2WSHAddressIndex>,
    pub p2abytes: MetricPattern30<P2ABytes>,
    pub p2pk33bytes: MetricPattern32<P2PK33Bytes>,
    pub p2pk65bytes: MetricPattern33<P2PK65Bytes>,
    pub p2pkhbytes: MetricPattern34<P2PKHBytes>,
    pub p2shbytes: MetricPattern35<P2SHBytes>,
    pub p2trbytes: MetricPattern36<P2TRBytes>,
    pub p2wpkhbytes: MetricPattern37<P2WPKHBytes>,
    pub p2wshbytes: MetricPattern38<P2WSHBytes>,
}

impl CatalogTree_Indexed_Address {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            first_p2aaddressindex: MetricPattern25::new(client.clone(), format!("{base_path}_first_p2aaddressindex")),
            first_p2pk33addressindex: MetricPattern25::new(client.clone(), format!("{base_path}_first_p2pk33addressindex")),
            first_p2pk65addressindex: MetricPattern25::new(client.clone(), format!("{base_path}_first_p2pk65addressindex")),
            first_p2pkhaddressindex: MetricPattern25::new(client.clone(), format!("{base_path}_first_p2pkhaddressindex")),
            first_p2shaddressindex: MetricPattern25::new(client.clone(), format!("{base_path}_first_p2shaddressindex")),
            first_p2traddressindex: MetricPattern25::new(client.clone(), format!("{base_path}_first_p2traddressindex")),
            first_p2wpkhaddressindex: MetricPattern25::new(client.clone(), format!("{base_path}_first_p2wpkhaddressindex")),
            first_p2wshaddressindex: MetricPattern25::new(client.clone(), format!("{base_path}_first_p2wshaddressindex")),
            p2abytes: MetricPattern30::new(client.clone(), format!("{base_path}_p2abytes")),
            p2pk33bytes: MetricPattern32::new(client.clone(), format!("{base_path}_p2pk33bytes")),
            p2pk65bytes: MetricPattern33::new(client.clone(), format!("{base_path}_p2pk65bytes")),
            p2pkhbytes: MetricPattern34::new(client.clone(), format!("{base_path}_p2pkhbytes")),
            p2shbytes: MetricPattern35::new(client.clone(), format!("{base_path}_p2shbytes")),
            p2trbytes: MetricPattern36::new(client.clone(), format!("{base_path}_p2trbytes")),
            p2wpkhbytes: MetricPattern37::new(client.clone(), format!("{base_path}_p2wpkhbytes")),
            p2wshbytes: MetricPattern38::new(client.clone(), format!("{base_path}_p2wshbytes")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Indexed_Block {
    pub blockhash: MetricPattern25<BlockHash>,
    pub difficulty: MetricPattern25<StoredF64>,
    pub timestamp: MetricPattern25<Timestamp>,
    pub total_size: MetricPattern25<StoredU64>,
    pub weight: MetricPattern25<Weight>,
}

impl CatalogTree_Indexed_Block {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            blockhash: MetricPattern25::new(client.clone(), format!("{base_path}_blockhash")),
            difficulty: MetricPattern25::new(client.clone(), format!("{base_path}_difficulty")),
            timestamp: MetricPattern25::new(client.clone(), format!("{base_path}_timestamp")),
            total_size: MetricPattern25::new(client.clone(), format!("{base_path}_total_size")),
            weight: MetricPattern25::new(client.clone(), format!("{base_path}_weight")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Indexed_Output {
    pub first_emptyoutputindex: MetricPattern25<EmptyOutputIndex>,
    pub first_opreturnindex: MetricPattern25<OpReturnIndex>,
    pub first_p2msoutputindex: MetricPattern25<P2MSOutputIndex>,
    pub first_unknownoutputindex: MetricPattern25<UnknownOutputIndex>,
    pub txindex: MetricPattern7<TxIndex>,
}

impl CatalogTree_Indexed_Output {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            first_emptyoutputindex: MetricPattern25::new(client.clone(), format!("{base_path}_first_emptyoutputindex")),
            first_opreturnindex: MetricPattern25::new(client.clone(), format!("{base_path}_first_opreturnindex")),
            first_p2msoutputindex: MetricPattern25::new(client.clone(), format!("{base_path}_first_p2msoutputindex")),
            first_unknownoutputindex: MetricPattern25::new(client.clone(), format!("{base_path}_first_unknownoutputindex")),
            txindex: MetricPattern7::new(client.clone(), format!("{base_path}_txindex")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Indexed_Tx {
    pub base_size: MetricPattern41<StoredU32>,
    pub first_txindex: MetricPattern25<TxIndex>,
    pub first_txinindex: MetricPattern41<TxInIndex>,
    pub first_txoutindex: MetricPattern41<TxOutIndex>,
    pub height: MetricPattern41<Height>,
    pub is_explicitly_rbf: MetricPattern41<StoredBool>,
    pub rawlocktime: MetricPattern41<RawLockTime>,
    pub total_size: MetricPattern41<StoredU32>,
    pub txid: MetricPattern41<Txid>,
    pub txversion: MetricPattern41<TxVersion>,
}

impl CatalogTree_Indexed_Tx {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            base_size: MetricPattern41::new(client.clone(), format!("{base_path}_base_size")),
            first_txindex: MetricPattern25::new(client.clone(), format!("{base_path}_first_txindex")),
            first_txinindex: MetricPattern41::new(client.clone(), format!("{base_path}_first_txinindex")),
            first_txoutindex: MetricPattern41::new(client.clone(), format!("{base_path}_first_txoutindex")),
            height: MetricPattern41::new(client.clone(), format!("{base_path}_height")),
            is_explicitly_rbf: MetricPattern41::new(client.clone(), format!("{base_path}_is_explicitly_rbf")),
            rawlocktime: MetricPattern41::new(client.clone(), format!("{base_path}_rawlocktime")),
            total_size: MetricPattern41::new(client.clone(), format!("{base_path}_total_size")),
            txid: MetricPattern41::new(client.clone(), format!("{base_path}_txid")),
            txversion: MetricPattern41::new(client.clone(), format!("{base_path}_txversion")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Indexed_Txin {
    pub first_txinindex: MetricPattern25<TxInIndex>,
    pub outpoint: MetricPattern26<OutPoint>,
    pub outputtype: MetricPattern26<OutputType>,
    pub txindex: MetricPattern26<TxIndex>,
    pub typeindex: MetricPattern26<TypeIndex>,
}

impl CatalogTree_Indexed_Txin {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            first_txinindex: MetricPattern25::new(client.clone(), format!("{base_path}_first_txinindex")),
            outpoint: MetricPattern26::new(client.clone(), format!("{base_path}_outpoint")),
            outputtype: MetricPattern26::new(client.clone(), format!("{base_path}_outputtype")),
            txindex: MetricPattern26::new(client.clone(), format!("{base_path}_txindex")),
            typeindex: MetricPattern26::new(client.clone(), format!("{base_path}_typeindex")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Indexed_Txout {
    pub first_txoutindex: MetricPattern25<TxOutIndex>,
    pub outputtype: MetricPattern29<OutputType>,
    pub txindex: MetricPattern29<TxIndex>,
    pub typeindex: MetricPattern29<TypeIndex>,
    pub value: MetricPattern29<Sats>,
}

impl CatalogTree_Indexed_Txout {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            first_txoutindex: MetricPattern25::new(client.clone(), format!("{base_path}_first_txoutindex")),
            outputtype: MetricPattern29::new(client.clone(), format!("{base_path}_outputtype")),
            txindex: MetricPattern29::new(client.clone(), format!("{base_path}_txindex")),
            typeindex: MetricPattern29::new(client.clone(), format!("{base_path}_typeindex")),
            value: MetricPattern29::new(client.clone(), format!("{base_path}_value")),
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
    pub const VERSION: &'static str = "v0.1.0-alpha.1";

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
    pub fn get_metric_by_index(&self, metric: &str, index: &str, from: Option<&str>, to: Option<&str>, count: Option<&str>, format: Option<&str>) -> Result<MetricData> {
        let mut query = Vec::new();
        if let Some(v) = from { query.push(format!("from={}", v)); }
        if let Some(v) = to { query.push(format!("to={}", v)); }
        if let Some(v) = count { query.push(format!("count={}", v)); }
        if let Some(v) = format { query.push(format!("format={}", v)); }
        let query_str = if query.is_empty() { String::new() } else { format!("?{}", query.join("&")) };
        self.base.get(&format!("/api/metric/{metric}/{index}{}", query_str))
    }

    /// Bulk metric data
    ///
    /// Fetch multiple metrics in a single request. Supports filtering by index and date range. Returns an array of MetricData objects.
    pub fn get_metrics_bulk(&self, metrics: &str, index: &str, from: Option<&str>, to: Option<&str>, count: Option<&str>, format: Option<&str>) -> Result<Vec<MetricData>> {
        let mut query = Vec::new();
        query.push(format!("metrics={}", metrics));
        query.push(format!("index={}", index));
        if let Some(v) = from { query.push(format!("from={}", v)); }
        if let Some(v) = to { query.push(format!("to={}", v)); }
        if let Some(v) = count { query.push(format!("count={}", v)); }
        if let Some(v) = format { query.push(format!("format={}", v)); }
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
