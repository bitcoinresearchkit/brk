// Auto-generated BRK Rust client
// Do not edit manually

#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(clippy::useless_format)]
#![allow(clippy::unnecessary_to_owned)]

pub use brk_cohort::*;
pub use brk_types::*;
use serde::de::DeserializeOwned;
use std::ops::{Bound, RangeBounds};
use std::sync::Arc;

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

    fn get(&self, path: &str) -> Result<minreq::Response> {
        let base = self.base_url.trim_end_matches('/');
        let url = format!("{}{}", base, path);
        let response = minreq::get(&url)
            .with_timeout(self.timeout_secs)
            .send()
            .map_err(|e| BrkError {
                message: e.to_string(),
            })?;

        if response.status_code >= 400 {
            return Err(BrkError {
                message: format!("HTTP {}", response.status_code),
            });
        }

        Ok(response)
    }

    /// Make a GET request and deserialize JSON response.
    pub fn get_json<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        self.get(path)?.json().map_err(|e| BrkError {
            message: e.to_string(),
        })
    }

    /// Make a GET request and return raw text response.
    pub fn get_text(&self, path: &str) -> Result<String> {
        self.get(path)?
            .as_str()
            .map(|s| s.to_string())
            .map_err(|e| BrkError {
                message: e.to_string(),
            })
    }
}

/// Build metric name with suffix.
#[inline]
fn _m(acc: &str, s: &str) -> String {
    if s.is_empty() {
        acc.to_string()
    } else if acc.is_empty() {
        s.to_string()
    } else {
        format!("{acc}_{s}")
    }
}

/// Build metric name with prefix.
#[inline]
fn _p(prefix: &str, acc: &str) -> String {
    if acc.is_empty() {
        prefix.to_string()
    } else {
        format!("{prefix}_{acc}")
    }
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
    /// Get an endpoint builder for a specific index, if supported.
    fn get(&self, index: Index) -> Option<MetricEndpointBuilder<T>>;
}

/// Shared endpoint configuration.
#[derive(Clone)]
struct EndpointConfig {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    index: Index,
    start: Option<i64>,
    end: Option<i64>,
}

impl EndpointConfig {
    fn new(client: Arc<BrkClientBase>, name: Arc<str>, index: Index) -> Self {
        Self {
            client,
            name,
            index,
            start: None,
            end: None,
        }
    }

    fn path(&self) -> String {
        format!("/api/metric/{}/{}", self.name, self.index.serialize_long())
    }

    fn build_path(&self, format: Option<&str>) -> String {
        let mut params = Vec::new();
        if let Some(s) = self.start {
            params.push(format!("start={}", s));
        }
        if let Some(e) = self.end {
            params.push(format!("end={}", e));
        }
        if let Some(fmt) = format {
            params.push(format!("format={}", fmt));
        }
        let p = self.path();
        if params.is_empty() {
            p
        } else {
            format!("{}?{}", p, params.join("&"))
        }
    }

    fn get_json<T: DeserializeOwned>(&self, format: Option<&str>) -> Result<T> {
        self.client.get_json(&self.build_path(format))
    }

    fn get_text(&self, format: Option<&str>) -> Result<String> {
        self.client.get_text(&self.build_path(format))
    }
}

/// Initial builder for metric endpoint queries.
///
/// Use method chaining to specify the data range, then call `fetch()` or `fetch_csv()` to execute.
///
/// # Examples
/// ```ignore
/// // Fetch all data
/// let data = endpoint.fetch()?;
///
/// // Get single item at index 5
/// let data = endpoint.get(5).fetch()?;
///
/// // Get first 10 using range
/// let data = endpoint.range(..10).fetch()?;
///
/// // Get range [100, 200)
/// let data = endpoint.range(100..200).fetch()?;
///
/// // Get first 10 (convenience)
/// let data = endpoint.take(10).fetch()?;
///
/// // Get last 10
/// let data = endpoint.last(10).fetch()?;
///
/// // Iterator-style chaining
/// let data = endpoint.skip(100).take(10).fetch()?;
/// ```
pub struct MetricEndpointBuilder<T> {
    config: EndpointConfig,
    _marker: std::marker::PhantomData<T>,
}

impl<T: DeserializeOwned> MetricEndpointBuilder<T> {
    pub fn new(client: Arc<BrkClientBase>, name: Arc<str>, index: Index) -> Self {
        Self {
            config: EndpointConfig::new(client, name, index),
            _marker: std::marker::PhantomData,
        }
    }

    /// Select a specific index position.
    pub fn get(mut self, index: usize) -> SingleItemBuilder<T> {
        self.config.start = Some(index as i64);
        self.config.end = Some(index as i64 + 1);
        SingleItemBuilder {
            config: self.config,
            _marker: std::marker::PhantomData,
        }
    }

    /// Select a range using Rust range syntax.
    ///
    /// # Examples
    /// ```ignore
    /// endpoint.range(..10)      // first 10
    /// endpoint.range(100..110)  // indices 100-109
    /// endpoint.range(100..)     // from 100 to end
    /// ```
    pub fn range<R: RangeBounds<usize>>(mut self, range: R) -> RangeBuilder<T> {
        self.config.start = match range.start_bound() {
            Bound::Included(&n) => Some(n as i64),
            Bound::Excluded(&n) => Some(n as i64 + 1),
            Bound::Unbounded => None,
        };
        self.config.end = match range.end_bound() {
            Bound::Included(&n) => Some(n as i64 + 1),
            Bound::Excluded(&n) => Some(n as i64),
            Bound::Unbounded => None,
        };
        RangeBuilder {
            config: self.config,
            _marker: std::marker::PhantomData,
        }
    }

    /// Take the first n items.
    pub fn take(self, n: usize) -> RangeBuilder<T> {
        self.range(..n)
    }

    /// Take the last n items.
    pub fn last(mut self, n: usize) -> RangeBuilder<T> {
        if n == 0 {
            self.config.end = Some(0);
        } else {
            self.config.start = Some(-(n as i64));
        }
        RangeBuilder {
            config: self.config,
            _marker: std::marker::PhantomData,
        }
    }

    /// Skip the first n items. Chain with `take(n)` to get a range.
    pub fn skip(mut self, n: usize) -> SkippedBuilder<T> {
        self.config.start = Some(n as i64);
        SkippedBuilder {
            config: self.config,
            _marker: std::marker::PhantomData,
        }
    }

    /// Fetch all data as parsed JSON.
    pub fn fetch(self) -> Result<MetricData<T>> {
        self.config.get_json(None)
    }

    /// Fetch all data as CSV string.
    pub fn fetch_csv(self) -> Result<String> {
        self.config.get_text(Some("csv"))
    }

    /// Get the base endpoint path.
    pub fn path(&self) -> String {
        self.config.path()
    }
}

/// Builder for single item access.
pub struct SingleItemBuilder<T> {
    config: EndpointConfig,
    _marker: std::marker::PhantomData<T>,
}

impl<T: DeserializeOwned> SingleItemBuilder<T> {
    /// Fetch the single item.
    pub fn fetch(self) -> Result<MetricData<T>> {
        self.config.get_json(None)
    }

    /// Fetch the single item as CSV.
    pub fn fetch_csv(self) -> Result<String> {
        self.config.get_text(Some("csv"))
    }
}

/// Builder after calling `skip(n)`. Chain with `take(n)` to specify count.
pub struct SkippedBuilder<T> {
    config: EndpointConfig,
    _marker: std::marker::PhantomData<T>,
}

impl<T: DeserializeOwned> SkippedBuilder<T> {
    /// Take n items after the skipped position.
    pub fn take(mut self, n: usize) -> RangeBuilder<T> {
        let start = self.config.start.unwrap_or(0);
        self.config.end = Some(start + n as i64);
        RangeBuilder {
            config: self.config,
            _marker: std::marker::PhantomData,
        }
    }

    /// Fetch from the skipped position to the end.
    pub fn fetch(self) -> Result<MetricData<T>> {
        self.config.get_json(None)
    }

    /// Fetch from the skipped position to the end as CSV.
    pub fn fetch_csv(self) -> Result<String> {
        self.config.get_text(Some("csv"))
    }
}

/// Builder with range fully specified.
pub struct RangeBuilder<T> {
    config: EndpointConfig,
    _marker: std::marker::PhantomData<T>,
}

impl<T: DeserializeOwned> RangeBuilder<T> {
    /// Fetch the range as parsed JSON.
    pub fn fetch(self) -> Result<MetricData<T>> {
        self.config.get_json(None)
    }

    /// Fetch the range as CSV string.
    pub fn fetch_csv(self) -> Result<String> {
        self.config.get_text(Some("csv"))
    }
}

// Static index arrays
const _I1: &[Index] = &[
    Index::DateIndex,
    Index::DecadeIndex,
    Index::DifficultyEpoch,
    Index::Height,
    Index::MonthIndex,
    Index::QuarterIndex,
    Index::SemesterIndex,
    Index::WeekIndex,
    Index::YearIndex,
];
const _I2: &[Index] = &[
    Index::DateIndex,
    Index::DecadeIndex,
    Index::DifficultyEpoch,
    Index::MonthIndex,
    Index::QuarterIndex,
    Index::SemesterIndex,
    Index::WeekIndex,
    Index::YearIndex,
];
const _I3: &[Index] = &[
    Index::DateIndex,
    Index::DecadeIndex,
    Index::Height,
    Index::MonthIndex,
    Index::QuarterIndex,
    Index::SemesterIndex,
    Index::WeekIndex,
    Index::YearIndex,
];
const _I4: &[Index] = &[
    Index::DateIndex,
    Index::DecadeIndex,
    Index::MonthIndex,
    Index::QuarterIndex,
    Index::SemesterIndex,
    Index::WeekIndex,
    Index::YearIndex,
];
const _I5: &[Index] = &[Index::DateIndex, Index::Height];
const _I6: &[Index] = &[Index::DateIndex];
const _I7: &[Index] = &[Index::DecadeIndex];
const _I8: &[Index] = &[Index::DifficultyEpoch];
const _I9: &[Index] = &[Index::EmptyOutputIndex];
const _I10: &[Index] = &[Index::HalvingEpoch];
const _I11: &[Index] = &[Index::Height];
const _I12: &[Index] = &[Index::TxInIndex];
const _I13: &[Index] = &[Index::MonthIndex];
const _I14: &[Index] = &[Index::OpReturnIndex];
const _I15: &[Index] = &[Index::TxOutIndex];
const _I16: &[Index] = &[Index::P2AAddressIndex];
const _I17: &[Index] = &[Index::P2MSOutputIndex];
const _I18: &[Index] = &[Index::P2PK33AddressIndex];
const _I19: &[Index] = &[Index::P2PK65AddressIndex];
const _I20: &[Index] = &[Index::P2PKHAddressIndex];
const _I21: &[Index] = &[Index::P2SHAddressIndex];
const _I22: &[Index] = &[Index::P2TRAddressIndex];
const _I23: &[Index] = &[Index::P2WPKHAddressIndex];
const _I24: &[Index] = &[Index::P2WSHAddressIndex];
const _I25: &[Index] = &[Index::QuarterIndex];
const _I26: &[Index] = &[Index::SemesterIndex];
const _I27: &[Index] = &[Index::TxIndex];
const _I28: &[Index] = &[Index::UnknownOutputIndex];
const _I29: &[Index] = &[Index::WeekIndex];
const _I30: &[Index] = &[Index::YearIndex];
const _I31: &[Index] = &[Index::LoadedAddressIndex];
const _I32: &[Index] = &[Index::EmptyAddressIndex];
const _I33: &[Index] = &[Index::PairOutputIndex];

#[inline]
fn _ep<T: DeserializeOwned>(
    c: &Arc<BrkClientBase>,
    n: &Arc<str>,
    i: Index,
) -> MetricEndpointBuilder<T> {
    MetricEndpointBuilder::new(c.clone(), n.clone(), i)
}

// Index accessor structs

pub struct MetricPattern1By<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    _marker: std::marker::PhantomData<T>,
}
impl<T: DeserializeOwned> MetricPattern1By<T> {
    pub fn dateindex(&self) -> MetricEndpointBuilder<T> {
        _ep(&self.client, &self.name, Index::DateIndex)
    }
    pub fn decadeindex(&self) -> MetricEndpointBuilder<T> {
        _ep(&self.client, &self.name, Index::DecadeIndex)
    }
    pub fn difficultyepoch(&self) -> MetricEndpointBuilder<T> {
        _ep(&self.client, &self.name, Index::DifficultyEpoch)
    }
    pub fn height(&self) -> MetricEndpointBuilder<T> {
        _ep(&self.client, &self.name, Index::Height)
    }
    pub fn monthindex(&self) -> MetricEndpointBuilder<T> {
        _ep(&self.client, &self.name, Index::MonthIndex)
    }
    pub fn quarterindex(&self) -> MetricEndpointBuilder<T> {
        _ep(&self.client, &self.name, Index::QuarterIndex)
    }
    pub fn semesterindex(&self) -> MetricEndpointBuilder<T> {
        _ep(&self.client, &self.name, Index::SemesterIndex)
    }
    pub fn weekindex(&self) -> MetricEndpointBuilder<T> {
        _ep(&self.client, &self.name, Index::WeekIndex)
    }
    pub fn yearindex(&self) -> MetricEndpointBuilder<T> {
        _ep(&self.client, &self.name, Index::YearIndex)
    }
}

pub struct MetricPattern1<T> {
    name: Arc<str>,
    pub by: MetricPattern1By<T>,
}
impl<T: DeserializeOwned> MetricPattern1<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self {
        let name: Arc<str> = name.into();
        Self {
            name: name.clone(),
            by: MetricPattern1By {
                client,
                name,
                _marker: std::marker::PhantomData,
            },
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<T> AnyMetricPattern for MetricPattern1<T> {
    fn name(&self) -> &str {
        &self.name
    }
    fn indexes(&self) -> &'static [Index] {
        _I1
    }
}
impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern1<T> {
    fn get(&self, index: Index) -> Option<MetricEndpointBuilder<T>> {
        _I1.contains(&index)
            .then(|| _ep(&self.by.client, &self.by.name, index))
    }
}

pub struct MetricPattern2By<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    _marker: std::marker::PhantomData<T>,
}
impl<T: DeserializeOwned> MetricPattern2By<T> {
    pub fn dateindex(&self) -> MetricEndpointBuilder<T> {
        _ep(&self.client, &self.name, Index::DateIndex)
    }
    pub fn decadeindex(&self) -> MetricEndpointBuilder<T> {
        _ep(&self.client, &self.name, Index::DecadeIndex)
    }
    pub fn difficultyepoch(&self) -> MetricEndpointBuilder<T> {
        _ep(&self.client, &self.name, Index::DifficultyEpoch)
    }
    pub fn monthindex(&self) -> MetricEndpointBuilder<T> {
        _ep(&self.client, &self.name, Index::MonthIndex)
    }
    pub fn quarterindex(&self) -> MetricEndpointBuilder<T> {
        _ep(&self.client, &self.name, Index::QuarterIndex)
    }
    pub fn semesterindex(&self) -> MetricEndpointBuilder<T> {
        _ep(&self.client, &self.name, Index::SemesterIndex)
    }
    pub fn weekindex(&self) -> MetricEndpointBuilder<T> {
        _ep(&self.client, &self.name, Index::WeekIndex)
    }
    pub fn yearindex(&self) -> MetricEndpointBuilder<T> {
        _ep(&self.client, &self.name, Index::YearIndex)
    }
}

pub struct MetricPattern2<T> {
    name: Arc<str>,
    pub by: MetricPattern2By<T>,
}
impl<T: DeserializeOwned> MetricPattern2<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self {
        let name: Arc<str> = name.into();
        Self {
            name: name.clone(),
            by: MetricPattern2By {
                client,
                name,
                _marker: std::marker::PhantomData,
            },
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<T> AnyMetricPattern for MetricPattern2<T> {
    fn name(&self) -> &str {
        &self.name
    }
    fn indexes(&self) -> &'static [Index] {
        _I2
    }
}
impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern2<T> {
    fn get(&self, index: Index) -> Option<MetricEndpointBuilder<T>> {
        _I2.contains(&index)
            .then(|| _ep(&self.by.client, &self.by.name, index))
    }
}

pub struct MetricPattern3By<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    _marker: std::marker::PhantomData<T>,
}
impl<T: DeserializeOwned> MetricPattern3By<T> {
    pub fn dateindex(&self) -> MetricEndpointBuilder<T> {
        _ep(&self.client, &self.name, Index::DateIndex)
    }
    pub fn decadeindex(&self) -> MetricEndpointBuilder<T> {
        _ep(&self.client, &self.name, Index::DecadeIndex)
    }
    pub fn height(&self) -> MetricEndpointBuilder<T> {
        _ep(&self.client, &self.name, Index::Height)
    }
    pub fn monthindex(&self) -> MetricEndpointBuilder<T> {
        _ep(&self.client, &self.name, Index::MonthIndex)
    }
    pub fn quarterindex(&self) -> MetricEndpointBuilder<T> {
        _ep(&self.client, &self.name, Index::QuarterIndex)
    }
    pub fn semesterindex(&self) -> MetricEndpointBuilder<T> {
        _ep(&self.client, &self.name, Index::SemesterIndex)
    }
    pub fn weekindex(&self) -> MetricEndpointBuilder<T> {
        _ep(&self.client, &self.name, Index::WeekIndex)
    }
    pub fn yearindex(&self) -> MetricEndpointBuilder<T> {
        _ep(&self.client, &self.name, Index::YearIndex)
    }
}

pub struct MetricPattern3<T> {
    name: Arc<str>,
    pub by: MetricPattern3By<T>,
}
impl<T: DeserializeOwned> MetricPattern3<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self {
        let name: Arc<str> = name.into();
        Self {
            name: name.clone(),
            by: MetricPattern3By {
                client,
                name,
                _marker: std::marker::PhantomData,
            },
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<T> AnyMetricPattern for MetricPattern3<T> {
    fn name(&self) -> &str {
        &self.name
    }
    fn indexes(&self) -> &'static [Index] {
        _I3
    }
}
impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern3<T> {
    fn get(&self, index: Index) -> Option<MetricEndpointBuilder<T>> {
        _I3.contains(&index)
            .then(|| _ep(&self.by.client, &self.by.name, index))
    }
}

pub struct MetricPattern4By<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    _marker: std::marker::PhantomData<T>,
}
impl<T: DeserializeOwned> MetricPattern4By<T> {
    pub fn dateindex(&self) -> MetricEndpointBuilder<T> {
        _ep(&self.client, &self.name, Index::DateIndex)
    }
    pub fn decadeindex(&self) -> MetricEndpointBuilder<T> {
        _ep(&self.client, &self.name, Index::DecadeIndex)
    }
    pub fn monthindex(&self) -> MetricEndpointBuilder<T> {
        _ep(&self.client, &self.name, Index::MonthIndex)
    }
    pub fn quarterindex(&self) -> MetricEndpointBuilder<T> {
        _ep(&self.client, &self.name, Index::QuarterIndex)
    }
    pub fn semesterindex(&self) -> MetricEndpointBuilder<T> {
        _ep(&self.client, &self.name, Index::SemesterIndex)
    }
    pub fn weekindex(&self) -> MetricEndpointBuilder<T> {
        _ep(&self.client, &self.name, Index::WeekIndex)
    }
    pub fn yearindex(&self) -> MetricEndpointBuilder<T> {
        _ep(&self.client, &self.name, Index::YearIndex)
    }
}

pub struct MetricPattern4<T> {
    name: Arc<str>,
    pub by: MetricPattern4By<T>,
}
impl<T: DeserializeOwned> MetricPattern4<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self {
        let name: Arc<str> = name.into();
        Self {
            name: name.clone(),
            by: MetricPattern4By {
                client,
                name,
                _marker: std::marker::PhantomData,
            },
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<T> AnyMetricPattern for MetricPattern4<T> {
    fn name(&self) -> &str {
        &self.name
    }
    fn indexes(&self) -> &'static [Index] {
        _I4
    }
}
impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern4<T> {
    fn get(&self, index: Index) -> Option<MetricEndpointBuilder<T>> {
        _I4.contains(&index)
            .then(|| _ep(&self.by.client, &self.by.name, index))
    }
}

pub struct MetricPattern5By<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    _marker: std::marker::PhantomData<T>,
}
impl<T: DeserializeOwned> MetricPattern5By<T> {
    pub fn dateindex(&self) -> MetricEndpointBuilder<T> {
        _ep(&self.client, &self.name, Index::DateIndex)
    }
    pub fn height(&self) -> MetricEndpointBuilder<T> {
        _ep(&self.client, &self.name, Index::Height)
    }
}

pub struct MetricPattern5<T> {
    name: Arc<str>,
    pub by: MetricPattern5By<T>,
}
impl<T: DeserializeOwned> MetricPattern5<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self {
        let name: Arc<str> = name.into();
        Self {
            name: name.clone(),
            by: MetricPattern5By {
                client,
                name,
                _marker: std::marker::PhantomData,
            },
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<T> AnyMetricPattern for MetricPattern5<T> {
    fn name(&self) -> &str {
        &self.name
    }
    fn indexes(&self) -> &'static [Index] {
        _I5
    }
}
impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern5<T> {
    fn get(&self, index: Index) -> Option<MetricEndpointBuilder<T>> {
        _I5.contains(&index)
            .then(|| _ep(&self.by.client, &self.by.name, index))
    }
}

pub struct MetricPattern6By<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    _marker: std::marker::PhantomData<T>,
}
impl<T: DeserializeOwned> MetricPattern6By<T> {
    pub fn dateindex(&self) -> MetricEndpointBuilder<T> {
        _ep(&self.client, &self.name, Index::DateIndex)
    }
}

pub struct MetricPattern6<T> {
    name: Arc<str>,
    pub by: MetricPattern6By<T>,
}
impl<T: DeserializeOwned> MetricPattern6<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self {
        let name: Arc<str> = name.into();
        Self {
            name: name.clone(),
            by: MetricPattern6By {
                client,
                name,
                _marker: std::marker::PhantomData,
            },
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<T> AnyMetricPattern for MetricPattern6<T> {
    fn name(&self) -> &str {
        &self.name
    }
    fn indexes(&self) -> &'static [Index] {
        _I6
    }
}
impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern6<T> {
    fn get(&self, index: Index) -> Option<MetricEndpointBuilder<T>> {
        _I6.contains(&index)
            .then(|| _ep(&self.by.client, &self.by.name, index))
    }
}

pub struct MetricPattern7By<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    _marker: std::marker::PhantomData<T>,
}
impl<T: DeserializeOwned> MetricPattern7By<T> {
    pub fn decadeindex(&self) -> MetricEndpointBuilder<T> {
        _ep(&self.client, &self.name, Index::DecadeIndex)
    }
}

pub struct MetricPattern7<T> {
    name: Arc<str>,
    pub by: MetricPattern7By<T>,
}
impl<T: DeserializeOwned> MetricPattern7<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self {
        let name: Arc<str> = name.into();
        Self {
            name: name.clone(),
            by: MetricPattern7By {
                client,
                name,
                _marker: std::marker::PhantomData,
            },
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<T> AnyMetricPattern for MetricPattern7<T> {
    fn name(&self) -> &str {
        &self.name
    }
    fn indexes(&self) -> &'static [Index] {
        _I7
    }
}
impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern7<T> {
    fn get(&self, index: Index) -> Option<MetricEndpointBuilder<T>> {
        _I7.contains(&index)
            .then(|| _ep(&self.by.client, &self.by.name, index))
    }
}

pub struct MetricPattern8By<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    _marker: std::marker::PhantomData<T>,
}
impl<T: DeserializeOwned> MetricPattern8By<T> {
    pub fn difficultyepoch(&self) -> MetricEndpointBuilder<T> {
        _ep(&self.client, &self.name, Index::DifficultyEpoch)
    }
}

pub struct MetricPattern8<T> {
    name: Arc<str>,
    pub by: MetricPattern8By<T>,
}
impl<T: DeserializeOwned> MetricPattern8<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self {
        let name: Arc<str> = name.into();
        Self {
            name: name.clone(),
            by: MetricPattern8By {
                client,
                name,
                _marker: std::marker::PhantomData,
            },
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<T> AnyMetricPattern for MetricPattern8<T> {
    fn name(&self) -> &str {
        &self.name
    }
    fn indexes(&self) -> &'static [Index] {
        _I8
    }
}
impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern8<T> {
    fn get(&self, index: Index) -> Option<MetricEndpointBuilder<T>> {
        _I8.contains(&index)
            .then(|| _ep(&self.by.client, &self.by.name, index))
    }
}

pub struct MetricPattern9By<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    _marker: std::marker::PhantomData<T>,
}
impl<T: DeserializeOwned> MetricPattern9By<T> {
    pub fn emptyoutputindex(&self) -> MetricEndpointBuilder<T> {
        _ep(&self.client, &self.name, Index::EmptyOutputIndex)
    }
}

pub struct MetricPattern9<T> {
    name: Arc<str>,
    pub by: MetricPattern9By<T>,
}
impl<T: DeserializeOwned> MetricPattern9<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self {
        let name: Arc<str> = name.into();
        Self {
            name: name.clone(),
            by: MetricPattern9By {
                client,
                name,
                _marker: std::marker::PhantomData,
            },
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<T> AnyMetricPattern for MetricPattern9<T> {
    fn name(&self) -> &str {
        &self.name
    }
    fn indexes(&self) -> &'static [Index] {
        _I9
    }
}
impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern9<T> {
    fn get(&self, index: Index) -> Option<MetricEndpointBuilder<T>> {
        _I9.contains(&index)
            .then(|| _ep(&self.by.client, &self.by.name, index))
    }
}

pub struct MetricPattern10By<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    _marker: std::marker::PhantomData<T>,
}
impl<T: DeserializeOwned> MetricPattern10By<T> {
    pub fn halvingepoch(&self) -> MetricEndpointBuilder<T> {
        _ep(&self.client, &self.name, Index::HalvingEpoch)
    }
}

pub struct MetricPattern10<T> {
    name: Arc<str>,
    pub by: MetricPattern10By<T>,
}
impl<T: DeserializeOwned> MetricPattern10<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self {
        let name: Arc<str> = name.into();
        Self {
            name: name.clone(),
            by: MetricPattern10By {
                client,
                name,
                _marker: std::marker::PhantomData,
            },
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<T> AnyMetricPattern for MetricPattern10<T> {
    fn name(&self) -> &str {
        &self.name
    }
    fn indexes(&self) -> &'static [Index] {
        _I10
    }
}
impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern10<T> {
    fn get(&self, index: Index) -> Option<MetricEndpointBuilder<T>> {
        _I10.contains(&index)
            .then(|| _ep(&self.by.client, &self.by.name, index))
    }
}

pub struct MetricPattern11By<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    _marker: std::marker::PhantomData<T>,
}
impl<T: DeserializeOwned> MetricPattern11By<T> {
    pub fn height(&self) -> MetricEndpointBuilder<T> {
        _ep(&self.client, &self.name, Index::Height)
    }
}

pub struct MetricPattern11<T> {
    name: Arc<str>,
    pub by: MetricPattern11By<T>,
}
impl<T: DeserializeOwned> MetricPattern11<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self {
        let name: Arc<str> = name.into();
        Self {
            name: name.clone(),
            by: MetricPattern11By {
                client,
                name,
                _marker: std::marker::PhantomData,
            },
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<T> AnyMetricPattern for MetricPattern11<T> {
    fn name(&self) -> &str {
        &self.name
    }
    fn indexes(&self) -> &'static [Index] {
        _I11
    }
}
impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern11<T> {
    fn get(&self, index: Index) -> Option<MetricEndpointBuilder<T>> {
        _I11.contains(&index)
            .then(|| _ep(&self.by.client, &self.by.name, index))
    }
}

pub struct MetricPattern12By<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    _marker: std::marker::PhantomData<T>,
}
impl<T: DeserializeOwned> MetricPattern12By<T> {
    pub fn txinindex(&self) -> MetricEndpointBuilder<T> {
        _ep(&self.client, &self.name, Index::TxInIndex)
    }
}

pub struct MetricPattern12<T> {
    name: Arc<str>,
    pub by: MetricPattern12By<T>,
}
impl<T: DeserializeOwned> MetricPattern12<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self {
        let name: Arc<str> = name.into();
        Self {
            name: name.clone(),
            by: MetricPattern12By {
                client,
                name,
                _marker: std::marker::PhantomData,
            },
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<T> AnyMetricPattern for MetricPattern12<T> {
    fn name(&self) -> &str {
        &self.name
    }
    fn indexes(&self) -> &'static [Index] {
        _I12
    }
}
impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern12<T> {
    fn get(&self, index: Index) -> Option<MetricEndpointBuilder<T>> {
        _I12.contains(&index)
            .then(|| _ep(&self.by.client, &self.by.name, index))
    }
}

pub struct MetricPattern13By<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    _marker: std::marker::PhantomData<T>,
}
impl<T: DeserializeOwned> MetricPattern13By<T> {
    pub fn monthindex(&self) -> MetricEndpointBuilder<T> {
        _ep(&self.client, &self.name, Index::MonthIndex)
    }
}

pub struct MetricPattern13<T> {
    name: Arc<str>,
    pub by: MetricPattern13By<T>,
}
impl<T: DeserializeOwned> MetricPattern13<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self {
        let name: Arc<str> = name.into();
        Self {
            name: name.clone(),
            by: MetricPattern13By {
                client,
                name,
                _marker: std::marker::PhantomData,
            },
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<T> AnyMetricPattern for MetricPattern13<T> {
    fn name(&self) -> &str {
        &self.name
    }
    fn indexes(&self) -> &'static [Index] {
        _I13
    }
}
impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern13<T> {
    fn get(&self, index: Index) -> Option<MetricEndpointBuilder<T>> {
        _I13.contains(&index)
            .then(|| _ep(&self.by.client, &self.by.name, index))
    }
}

pub struct MetricPattern14By<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    _marker: std::marker::PhantomData<T>,
}
impl<T: DeserializeOwned> MetricPattern14By<T> {
    pub fn opreturnindex(&self) -> MetricEndpointBuilder<T> {
        _ep(&self.client, &self.name, Index::OpReturnIndex)
    }
}

pub struct MetricPattern14<T> {
    name: Arc<str>,
    pub by: MetricPattern14By<T>,
}
impl<T: DeserializeOwned> MetricPattern14<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self {
        let name: Arc<str> = name.into();
        Self {
            name: name.clone(),
            by: MetricPattern14By {
                client,
                name,
                _marker: std::marker::PhantomData,
            },
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<T> AnyMetricPattern for MetricPattern14<T> {
    fn name(&self) -> &str {
        &self.name
    }
    fn indexes(&self) -> &'static [Index] {
        _I14
    }
}
impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern14<T> {
    fn get(&self, index: Index) -> Option<MetricEndpointBuilder<T>> {
        _I14.contains(&index)
            .then(|| _ep(&self.by.client, &self.by.name, index))
    }
}

pub struct MetricPattern15By<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    _marker: std::marker::PhantomData<T>,
}
impl<T: DeserializeOwned> MetricPattern15By<T> {
    pub fn txoutindex(&self) -> MetricEndpointBuilder<T> {
        _ep(&self.client, &self.name, Index::TxOutIndex)
    }
}

pub struct MetricPattern15<T> {
    name: Arc<str>,
    pub by: MetricPattern15By<T>,
}
impl<T: DeserializeOwned> MetricPattern15<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self {
        let name: Arc<str> = name.into();
        Self {
            name: name.clone(),
            by: MetricPattern15By {
                client,
                name,
                _marker: std::marker::PhantomData,
            },
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<T> AnyMetricPattern for MetricPattern15<T> {
    fn name(&self) -> &str {
        &self.name
    }
    fn indexes(&self) -> &'static [Index] {
        _I15
    }
}
impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern15<T> {
    fn get(&self, index: Index) -> Option<MetricEndpointBuilder<T>> {
        _I15.contains(&index)
            .then(|| _ep(&self.by.client, &self.by.name, index))
    }
}

pub struct MetricPattern16By<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    _marker: std::marker::PhantomData<T>,
}
impl<T: DeserializeOwned> MetricPattern16By<T> {
    pub fn p2aaddressindex(&self) -> MetricEndpointBuilder<T> {
        _ep(&self.client, &self.name, Index::P2AAddressIndex)
    }
}

pub struct MetricPattern16<T> {
    name: Arc<str>,
    pub by: MetricPattern16By<T>,
}
impl<T: DeserializeOwned> MetricPattern16<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self {
        let name: Arc<str> = name.into();
        Self {
            name: name.clone(),
            by: MetricPattern16By {
                client,
                name,
                _marker: std::marker::PhantomData,
            },
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<T> AnyMetricPattern for MetricPattern16<T> {
    fn name(&self) -> &str {
        &self.name
    }
    fn indexes(&self) -> &'static [Index] {
        _I16
    }
}
impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern16<T> {
    fn get(&self, index: Index) -> Option<MetricEndpointBuilder<T>> {
        _I16.contains(&index)
            .then(|| _ep(&self.by.client, &self.by.name, index))
    }
}

pub struct MetricPattern17By<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    _marker: std::marker::PhantomData<T>,
}
impl<T: DeserializeOwned> MetricPattern17By<T> {
    pub fn p2msoutputindex(&self) -> MetricEndpointBuilder<T> {
        _ep(&self.client, &self.name, Index::P2MSOutputIndex)
    }
}

pub struct MetricPattern17<T> {
    name: Arc<str>,
    pub by: MetricPattern17By<T>,
}
impl<T: DeserializeOwned> MetricPattern17<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self {
        let name: Arc<str> = name.into();
        Self {
            name: name.clone(),
            by: MetricPattern17By {
                client,
                name,
                _marker: std::marker::PhantomData,
            },
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<T> AnyMetricPattern for MetricPattern17<T> {
    fn name(&self) -> &str {
        &self.name
    }
    fn indexes(&self) -> &'static [Index] {
        _I17
    }
}
impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern17<T> {
    fn get(&self, index: Index) -> Option<MetricEndpointBuilder<T>> {
        _I17.contains(&index)
            .then(|| _ep(&self.by.client, &self.by.name, index))
    }
}

pub struct MetricPattern18By<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    _marker: std::marker::PhantomData<T>,
}
impl<T: DeserializeOwned> MetricPattern18By<T> {
    pub fn p2pk33addressindex(&self) -> MetricEndpointBuilder<T> {
        _ep(&self.client, &self.name, Index::P2PK33AddressIndex)
    }
}

pub struct MetricPattern18<T> {
    name: Arc<str>,
    pub by: MetricPattern18By<T>,
}
impl<T: DeserializeOwned> MetricPattern18<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self {
        let name: Arc<str> = name.into();
        Self {
            name: name.clone(),
            by: MetricPattern18By {
                client,
                name,
                _marker: std::marker::PhantomData,
            },
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<T> AnyMetricPattern for MetricPattern18<T> {
    fn name(&self) -> &str {
        &self.name
    }
    fn indexes(&self) -> &'static [Index] {
        _I18
    }
}
impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern18<T> {
    fn get(&self, index: Index) -> Option<MetricEndpointBuilder<T>> {
        _I18.contains(&index)
            .then(|| _ep(&self.by.client, &self.by.name, index))
    }
}

pub struct MetricPattern19By<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    _marker: std::marker::PhantomData<T>,
}
impl<T: DeserializeOwned> MetricPattern19By<T> {
    pub fn p2pk65addressindex(&self) -> MetricEndpointBuilder<T> {
        _ep(&self.client, &self.name, Index::P2PK65AddressIndex)
    }
}

pub struct MetricPattern19<T> {
    name: Arc<str>,
    pub by: MetricPattern19By<T>,
}
impl<T: DeserializeOwned> MetricPattern19<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self {
        let name: Arc<str> = name.into();
        Self {
            name: name.clone(),
            by: MetricPattern19By {
                client,
                name,
                _marker: std::marker::PhantomData,
            },
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<T> AnyMetricPattern for MetricPattern19<T> {
    fn name(&self) -> &str {
        &self.name
    }
    fn indexes(&self) -> &'static [Index] {
        _I19
    }
}
impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern19<T> {
    fn get(&self, index: Index) -> Option<MetricEndpointBuilder<T>> {
        _I19.contains(&index)
            .then(|| _ep(&self.by.client, &self.by.name, index))
    }
}

pub struct MetricPattern20By<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    _marker: std::marker::PhantomData<T>,
}
impl<T: DeserializeOwned> MetricPattern20By<T> {
    pub fn p2pkhaddressindex(&self) -> MetricEndpointBuilder<T> {
        _ep(&self.client, &self.name, Index::P2PKHAddressIndex)
    }
}

pub struct MetricPattern20<T> {
    name: Arc<str>,
    pub by: MetricPattern20By<T>,
}
impl<T: DeserializeOwned> MetricPattern20<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self {
        let name: Arc<str> = name.into();
        Self {
            name: name.clone(),
            by: MetricPattern20By {
                client,
                name,
                _marker: std::marker::PhantomData,
            },
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<T> AnyMetricPattern for MetricPattern20<T> {
    fn name(&self) -> &str {
        &self.name
    }
    fn indexes(&self) -> &'static [Index] {
        _I20
    }
}
impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern20<T> {
    fn get(&self, index: Index) -> Option<MetricEndpointBuilder<T>> {
        _I20.contains(&index)
            .then(|| _ep(&self.by.client, &self.by.name, index))
    }
}

pub struct MetricPattern21By<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    _marker: std::marker::PhantomData<T>,
}
impl<T: DeserializeOwned> MetricPattern21By<T> {
    pub fn p2shaddressindex(&self) -> MetricEndpointBuilder<T> {
        _ep(&self.client, &self.name, Index::P2SHAddressIndex)
    }
}

pub struct MetricPattern21<T> {
    name: Arc<str>,
    pub by: MetricPattern21By<T>,
}
impl<T: DeserializeOwned> MetricPattern21<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self {
        let name: Arc<str> = name.into();
        Self {
            name: name.clone(),
            by: MetricPattern21By {
                client,
                name,
                _marker: std::marker::PhantomData,
            },
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<T> AnyMetricPattern for MetricPattern21<T> {
    fn name(&self) -> &str {
        &self.name
    }
    fn indexes(&self) -> &'static [Index] {
        _I21
    }
}
impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern21<T> {
    fn get(&self, index: Index) -> Option<MetricEndpointBuilder<T>> {
        _I21.contains(&index)
            .then(|| _ep(&self.by.client, &self.by.name, index))
    }
}

pub struct MetricPattern22By<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    _marker: std::marker::PhantomData<T>,
}
impl<T: DeserializeOwned> MetricPattern22By<T> {
    pub fn p2traddressindex(&self) -> MetricEndpointBuilder<T> {
        _ep(&self.client, &self.name, Index::P2TRAddressIndex)
    }
}

pub struct MetricPattern22<T> {
    name: Arc<str>,
    pub by: MetricPattern22By<T>,
}
impl<T: DeserializeOwned> MetricPattern22<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self {
        let name: Arc<str> = name.into();
        Self {
            name: name.clone(),
            by: MetricPattern22By {
                client,
                name,
                _marker: std::marker::PhantomData,
            },
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<T> AnyMetricPattern for MetricPattern22<T> {
    fn name(&self) -> &str {
        &self.name
    }
    fn indexes(&self) -> &'static [Index] {
        _I22
    }
}
impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern22<T> {
    fn get(&self, index: Index) -> Option<MetricEndpointBuilder<T>> {
        _I22.contains(&index)
            .then(|| _ep(&self.by.client, &self.by.name, index))
    }
}

pub struct MetricPattern23By<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    _marker: std::marker::PhantomData<T>,
}
impl<T: DeserializeOwned> MetricPattern23By<T> {
    pub fn p2wpkhaddressindex(&self) -> MetricEndpointBuilder<T> {
        _ep(&self.client, &self.name, Index::P2WPKHAddressIndex)
    }
}

pub struct MetricPattern23<T> {
    name: Arc<str>,
    pub by: MetricPattern23By<T>,
}
impl<T: DeserializeOwned> MetricPattern23<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self {
        let name: Arc<str> = name.into();
        Self {
            name: name.clone(),
            by: MetricPattern23By {
                client,
                name,
                _marker: std::marker::PhantomData,
            },
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<T> AnyMetricPattern for MetricPattern23<T> {
    fn name(&self) -> &str {
        &self.name
    }
    fn indexes(&self) -> &'static [Index] {
        _I23
    }
}
impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern23<T> {
    fn get(&self, index: Index) -> Option<MetricEndpointBuilder<T>> {
        _I23.contains(&index)
            .then(|| _ep(&self.by.client, &self.by.name, index))
    }
}

pub struct MetricPattern24By<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    _marker: std::marker::PhantomData<T>,
}
impl<T: DeserializeOwned> MetricPattern24By<T> {
    pub fn p2wshaddressindex(&self) -> MetricEndpointBuilder<T> {
        _ep(&self.client, &self.name, Index::P2WSHAddressIndex)
    }
}

pub struct MetricPattern24<T> {
    name: Arc<str>,
    pub by: MetricPattern24By<T>,
}
impl<T: DeserializeOwned> MetricPattern24<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self {
        let name: Arc<str> = name.into();
        Self {
            name: name.clone(),
            by: MetricPattern24By {
                client,
                name,
                _marker: std::marker::PhantomData,
            },
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<T> AnyMetricPattern for MetricPattern24<T> {
    fn name(&self) -> &str {
        &self.name
    }
    fn indexes(&self) -> &'static [Index] {
        _I24
    }
}
impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern24<T> {
    fn get(&self, index: Index) -> Option<MetricEndpointBuilder<T>> {
        _I24.contains(&index)
            .then(|| _ep(&self.by.client, &self.by.name, index))
    }
}

pub struct MetricPattern25By<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    _marker: std::marker::PhantomData<T>,
}
impl<T: DeserializeOwned> MetricPattern25By<T> {
    pub fn quarterindex(&self) -> MetricEndpointBuilder<T> {
        _ep(&self.client, &self.name, Index::QuarterIndex)
    }
}

pub struct MetricPattern25<T> {
    name: Arc<str>,
    pub by: MetricPattern25By<T>,
}
impl<T: DeserializeOwned> MetricPattern25<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self {
        let name: Arc<str> = name.into();
        Self {
            name: name.clone(),
            by: MetricPattern25By {
                client,
                name,
                _marker: std::marker::PhantomData,
            },
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<T> AnyMetricPattern for MetricPattern25<T> {
    fn name(&self) -> &str {
        &self.name
    }
    fn indexes(&self) -> &'static [Index] {
        _I25
    }
}
impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern25<T> {
    fn get(&self, index: Index) -> Option<MetricEndpointBuilder<T>> {
        _I25.contains(&index)
            .then(|| _ep(&self.by.client, &self.by.name, index))
    }
}

pub struct MetricPattern26By<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    _marker: std::marker::PhantomData<T>,
}
impl<T: DeserializeOwned> MetricPattern26By<T> {
    pub fn semesterindex(&self) -> MetricEndpointBuilder<T> {
        _ep(&self.client, &self.name, Index::SemesterIndex)
    }
}

pub struct MetricPattern26<T> {
    name: Arc<str>,
    pub by: MetricPattern26By<T>,
}
impl<T: DeserializeOwned> MetricPattern26<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self {
        let name: Arc<str> = name.into();
        Self {
            name: name.clone(),
            by: MetricPattern26By {
                client,
                name,
                _marker: std::marker::PhantomData,
            },
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<T> AnyMetricPattern for MetricPattern26<T> {
    fn name(&self) -> &str {
        &self.name
    }
    fn indexes(&self) -> &'static [Index] {
        _I26
    }
}
impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern26<T> {
    fn get(&self, index: Index) -> Option<MetricEndpointBuilder<T>> {
        _I26.contains(&index)
            .then(|| _ep(&self.by.client, &self.by.name, index))
    }
}

pub struct MetricPattern27By<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    _marker: std::marker::PhantomData<T>,
}
impl<T: DeserializeOwned> MetricPattern27By<T> {
    pub fn txindex(&self) -> MetricEndpointBuilder<T> {
        _ep(&self.client, &self.name, Index::TxIndex)
    }
}

pub struct MetricPattern27<T> {
    name: Arc<str>,
    pub by: MetricPattern27By<T>,
}
impl<T: DeserializeOwned> MetricPattern27<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self {
        let name: Arc<str> = name.into();
        Self {
            name: name.clone(),
            by: MetricPattern27By {
                client,
                name,
                _marker: std::marker::PhantomData,
            },
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<T> AnyMetricPattern for MetricPattern27<T> {
    fn name(&self) -> &str {
        &self.name
    }
    fn indexes(&self) -> &'static [Index] {
        _I27
    }
}
impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern27<T> {
    fn get(&self, index: Index) -> Option<MetricEndpointBuilder<T>> {
        _I27.contains(&index)
            .then(|| _ep(&self.by.client, &self.by.name, index))
    }
}

pub struct MetricPattern28By<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    _marker: std::marker::PhantomData<T>,
}
impl<T: DeserializeOwned> MetricPattern28By<T> {
    pub fn unknownoutputindex(&self) -> MetricEndpointBuilder<T> {
        _ep(&self.client, &self.name, Index::UnknownOutputIndex)
    }
}

pub struct MetricPattern28<T> {
    name: Arc<str>,
    pub by: MetricPattern28By<T>,
}
impl<T: DeserializeOwned> MetricPattern28<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self {
        let name: Arc<str> = name.into();
        Self {
            name: name.clone(),
            by: MetricPattern28By {
                client,
                name,
                _marker: std::marker::PhantomData,
            },
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<T> AnyMetricPattern for MetricPattern28<T> {
    fn name(&self) -> &str {
        &self.name
    }
    fn indexes(&self) -> &'static [Index] {
        _I28
    }
}
impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern28<T> {
    fn get(&self, index: Index) -> Option<MetricEndpointBuilder<T>> {
        _I28.contains(&index)
            .then(|| _ep(&self.by.client, &self.by.name, index))
    }
}

pub struct MetricPattern29By<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    _marker: std::marker::PhantomData<T>,
}
impl<T: DeserializeOwned> MetricPattern29By<T> {
    pub fn weekindex(&self) -> MetricEndpointBuilder<T> {
        _ep(&self.client, &self.name, Index::WeekIndex)
    }
}

pub struct MetricPattern29<T> {
    name: Arc<str>,
    pub by: MetricPattern29By<T>,
}
impl<T: DeserializeOwned> MetricPattern29<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self {
        let name: Arc<str> = name.into();
        Self {
            name: name.clone(),
            by: MetricPattern29By {
                client,
                name,
                _marker: std::marker::PhantomData,
            },
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<T> AnyMetricPattern for MetricPattern29<T> {
    fn name(&self) -> &str {
        &self.name
    }
    fn indexes(&self) -> &'static [Index] {
        _I29
    }
}
impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern29<T> {
    fn get(&self, index: Index) -> Option<MetricEndpointBuilder<T>> {
        _I29.contains(&index)
            .then(|| _ep(&self.by.client, &self.by.name, index))
    }
}

pub struct MetricPattern30By<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    _marker: std::marker::PhantomData<T>,
}
impl<T: DeserializeOwned> MetricPattern30By<T> {
    pub fn yearindex(&self) -> MetricEndpointBuilder<T> {
        _ep(&self.client, &self.name, Index::YearIndex)
    }
}

pub struct MetricPattern30<T> {
    name: Arc<str>,
    pub by: MetricPattern30By<T>,
}
impl<T: DeserializeOwned> MetricPattern30<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self {
        let name: Arc<str> = name.into();
        Self {
            name: name.clone(),
            by: MetricPattern30By {
                client,
                name,
                _marker: std::marker::PhantomData,
            },
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<T> AnyMetricPattern for MetricPattern30<T> {
    fn name(&self) -> &str {
        &self.name
    }
    fn indexes(&self) -> &'static [Index] {
        _I30
    }
}
impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern30<T> {
    fn get(&self, index: Index) -> Option<MetricEndpointBuilder<T>> {
        _I30.contains(&index)
            .then(|| _ep(&self.by.client, &self.by.name, index))
    }
}

pub struct MetricPattern31By<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    _marker: std::marker::PhantomData<T>,
}
impl<T: DeserializeOwned> MetricPattern31By<T> {
    pub fn loadedaddressindex(&self) -> MetricEndpointBuilder<T> {
        _ep(&self.client, &self.name, Index::LoadedAddressIndex)
    }
}

pub struct MetricPattern31<T> {
    name: Arc<str>,
    pub by: MetricPattern31By<T>,
}
impl<T: DeserializeOwned> MetricPattern31<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self {
        let name: Arc<str> = name.into();
        Self {
            name: name.clone(),
            by: MetricPattern31By {
                client,
                name,
                _marker: std::marker::PhantomData,
            },
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<T> AnyMetricPattern for MetricPattern31<T> {
    fn name(&self) -> &str {
        &self.name
    }
    fn indexes(&self) -> &'static [Index] {
        _I31
    }
}
impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern31<T> {
    fn get(&self, index: Index) -> Option<MetricEndpointBuilder<T>> {
        _I31.contains(&index)
            .then(|| _ep(&self.by.client, &self.by.name, index))
    }
}

pub struct MetricPattern32By<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    _marker: std::marker::PhantomData<T>,
}
impl<T: DeserializeOwned> MetricPattern32By<T> {
    pub fn emptyaddressindex(&self) -> MetricEndpointBuilder<T> {
        _ep(&self.client, &self.name, Index::EmptyAddressIndex)
    }
}

pub struct MetricPattern32<T> {
    name: Arc<str>,
    pub by: MetricPattern32By<T>,
}
impl<T: DeserializeOwned> MetricPattern32<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self {
        let name: Arc<str> = name.into();
        Self {
            name: name.clone(),
            by: MetricPattern32By {
                client,
                name,
                _marker: std::marker::PhantomData,
            },
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<T> AnyMetricPattern for MetricPattern32<T> {
    fn name(&self) -> &str {
        &self.name
    }
    fn indexes(&self) -> &'static [Index] {
        _I32
    }
}
impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern32<T> {
    fn get(&self, index: Index) -> Option<MetricEndpointBuilder<T>> {
        _I32.contains(&index)
            .then(|| _ep(&self.by.client, &self.by.name, index))
    }
}

pub struct MetricPattern33By<T> {
    client: Arc<BrkClientBase>,
    name: Arc<str>,
    _marker: std::marker::PhantomData<T>,
}
impl<T: DeserializeOwned> MetricPattern33By<T> {
    pub fn pairoutputindex(&self) -> MetricEndpointBuilder<T> {
        _ep(&self.client, &self.name, Index::PairOutputIndex)
    }
}

pub struct MetricPattern33<T> {
    name: Arc<str>,
    pub by: MetricPattern33By<T>,
}
impl<T: DeserializeOwned> MetricPattern33<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self {
        let name: Arc<str> = name.into();
        Self {
            name: name.clone(),
            by: MetricPattern33By {
                client,
                name,
                _marker: std::marker::PhantomData,
            },
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<T> AnyMetricPattern for MetricPattern33<T> {
    fn name(&self) -> &str {
        &self.name
    }
    fn indexes(&self) -> &'static [Index] {
        _I33
    }
}
impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern33<T> {
    fn get(&self, index: Index) -> Option<MetricEndpointBuilder<T>> {
        _I33.contains(&index)
            .then(|| _ep(&self.by.client, &self.by.name, index))
    }
}

// Reusable pattern structs

/// Pattern struct for repeated tree structure.
pub struct RealizedPattern3 {
    pub adjusted_sopr: MetricPattern6<StoredF64>,
    pub adjusted_sopr_30d_ema: MetricPattern6<StoredF64>,
    pub adjusted_sopr_7d_ema: MetricPattern6<StoredF64>,
    pub adjusted_value_created: MetricPattern1<Dollars>,
    pub adjusted_value_destroyed: MetricPattern1<Dollars>,
    pub mvrv: MetricPattern4<StoredF32>,
    pub neg_realized_loss: BitcoinPattern2<Dollars>,
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
    pub realized_profit_to_loss_ratio: MetricPattern6<StoredF64>,
    pub realized_value: MetricPattern1<Dollars>,
    pub sell_side_risk_ratio: MetricPattern6<StoredF32>,
    pub sell_side_risk_ratio_30d_ema: MetricPattern6<StoredF32>,
    pub sell_side_risk_ratio_7d_ema: MetricPattern6<StoredF32>,
    pub sopr: MetricPattern6<StoredF64>,
    pub sopr_30d_ema: MetricPattern6<StoredF64>,
    pub sopr_7d_ema: MetricPattern6<StoredF64>,
    pub total_realized_pnl: MetricPattern1<Dollars>,
    pub value_created: MetricPattern1<Dollars>,
    pub value_destroyed: MetricPattern1<Dollars>,
}

impl RealizedPattern3 {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            adjusted_sopr: MetricPattern6::new(client.clone(), _m(&acc, "adjusted_sopr")),
            adjusted_sopr_30d_ema: MetricPattern6::new(
                client.clone(),
                _m(&acc, "adjusted_sopr_30d_ema"),
            ),
            adjusted_sopr_7d_ema: MetricPattern6::new(
                client.clone(),
                _m(&acc, "adjusted_sopr_7d_ema"),
            ),
            adjusted_value_created: MetricPattern1::new(
                client.clone(),
                _m(&acc, "adjusted_value_created"),
            ),
            adjusted_value_destroyed: MetricPattern1::new(
                client.clone(),
                _m(&acc, "adjusted_value_destroyed"),
            ),
            mvrv: MetricPattern4::new(client.clone(), _m(&acc, "mvrv")),
            neg_realized_loss: BitcoinPattern2::new(client.clone(), _m(&acc, "neg_realized_loss")),
            net_realized_pnl: BlockCountPattern::new(client.clone(), _m(&acc, "net_realized_pnl")),
            net_realized_pnl_cumulative_30d_delta: MetricPattern4::new(
                client.clone(),
                _m(&acc, "net_realized_pnl_cumulative_30d_delta"),
            ),
            net_realized_pnl_cumulative_30d_delta_rel_to_market_cap: MetricPattern4::new(
                client.clone(),
                _m(
                    &acc,
                    "net_realized_pnl_cumulative_30d_delta_rel_to_market_cap",
                ),
            ),
            net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap: MetricPattern4::new(
                client.clone(),
                _m(
                    &acc,
                    "net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap",
                ),
            ),
            net_realized_pnl_rel_to_realized_cap: BlockCountPattern::new(
                client.clone(),
                _m(&acc, "net_realized_pnl_rel_to_realized_cap"),
            ),
            realized_cap: MetricPattern1::new(client.clone(), _m(&acc, "realized_cap")),
            realized_cap_30d_delta: MetricPattern4::new(
                client.clone(),
                _m(&acc, "realized_cap_30d_delta"),
            ),
            realized_cap_rel_to_own_market_cap: MetricPattern1::new(
                client.clone(),
                _m(&acc, "realized_cap_rel_to_own_market_cap"),
            ),
            realized_loss: BlockCountPattern::new(client.clone(), _m(&acc, "realized_loss")),
            realized_loss_rel_to_realized_cap: BlockCountPattern::new(
                client.clone(),
                _m(&acc, "realized_loss_rel_to_realized_cap"),
            ),
            realized_price: MetricPattern1::new(client.clone(), _m(&acc, "realized_price")),
            realized_price_extra: ActivePriceRatioPattern::new(
                client.clone(),
                _m(&acc, "realized_price_ratio"),
            ),
            realized_profit: BlockCountPattern::new(client.clone(), _m(&acc, "realized_profit")),
            realized_profit_rel_to_realized_cap: BlockCountPattern::new(
                client.clone(),
                _m(&acc, "realized_profit_rel_to_realized_cap"),
            ),
            realized_profit_to_loss_ratio: MetricPattern6::new(
                client.clone(),
                _m(&acc, "realized_profit_to_loss_ratio"),
            ),
            realized_value: MetricPattern1::new(client.clone(), _m(&acc, "realized_value")),
            sell_side_risk_ratio: MetricPattern6::new(
                client.clone(),
                _m(&acc, "sell_side_risk_ratio"),
            ),
            sell_side_risk_ratio_30d_ema: MetricPattern6::new(
                client.clone(),
                _m(&acc, "sell_side_risk_ratio_30d_ema"),
            ),
            sell_side_risk_ratio_7d_ema: MetricPattern6::new(
                client.clone(),
                _m(&acc, "sell_side_risk_ratio_7d_ema"),
            ),
            sopr: MetricPattern6::new(client.clone(), _m(&acc, "sopr")),
            sopr_30d_ema: MetricPattern6::new(client.clone(), _m(&acc, "sopr_30d_ema")),
            sopr_7d_ema: MetricPattern6::new(client.clone(), _m(&acc, "sopr_7d_ema")),
            total_realized_pnl: MetricPattern1::new(client.clone(), _m(&acc, "total_realized_pnl")),
            value_created: MetricPattern1::new(client.clone(), _m(&acc, "value_created")),
            value_destroyed: MetricPattern1::new(client.clone(), _m(&acc, "value_destroyed")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct RealizedPattern4 {
    pub adjusted_sopr: MetricPattern6<StoredF64>,
    pub adjusted_sopr_30d_ema: MetricPattern6<StoredF64>,
    pub adjusted_sopr_7d_ema: MetricPattern6<StoredF64>,
    pub adjusted_value_created: MetricPattern1<Dollars>,
    pub adjusted_value_destroyed: MetricPattern1<Dollars>,
    pub mvrv: MetricPattern4<StoredF32>,
    pub neg_realized_loss: BitcoinPattern2<Dollars>,
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
    pub sell_side_risk_ratio: MetricPattern6<StoredF32>,
    pub sell_side_risk_ratio_30d_ema: MetricPattern6<StoredF32>,
    pub sell_side_risk_ratio_7d_ema: MetricPattern6<StoredF32>,
    pub sopr: MetricPattern6<StoredF64>,
    pub sopr_30d_ema: MetricPattern6<StoredF64>,
    pub sopr_7d_ema: MetricPattern6<StoredF64>,
    pub total_realized_pnl: MetricPattern1<Dollars>,
    pub value_created: MetricPattern1<Dollars>,
    pub value_destroyed: MetricPattern1<Dollars>,
}

impl RealizedPattern4 {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            adjusted_sopr: MetricPattern6::new(client.clone(), _m(&acc, "adjusted_sopr")),
            adjusted_sopr_30d_ema: MetricPattern6::new(
                client.clone(),
                _m(&acc, "adjusted_sopr_30d_ema"),
            ),
            adjusted_sopr_7d_ema: MetricPattern6::new(
                client.clone(),
                _m(&acc, "adjusted_sopr_7d_ema"),
            ),
            adjusted_value_created: MetricPattern1::new(
                client.clone(),
                _m(&acc, "adjusted_value_created"),
            ),
            adjusted_value_destroyed: MetricPattern1::new(
                client.clone(),
                _m(&acc, "adjusted_value_destroyed"),
            ),
            mvrv: MetricPattern4::new(client.clone(), _m(&acc, "mvrv")),
            neg_realized_loss: BitcoinPattern2::new(client.clone(), _m(&acc, "neg_realized_loss")),
            net_realized_pnl: BlockCountPattern::new(client.clone(), _m(&acc, "net_realized_pnl")),
            net_realized_pnl_cumulative_30d_delta: MetricPattern4::new(
                client.clone(),
                _m(&acc, "net_realized_pnl_cumulative_30d_delta"),
            ),
            net_realized_pnl_cumulative_30d_delta_rel_to_market_cap: MetricPattern4::new(
                client.clone(),
                _m(
                    &acc,
                    "net_realized_pnl_cumulative_30d_delta_rel_to_market_cap",
                ),
            ),
            net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap: MetricPattern4::new(
                client.clone(),
                _m(
                    &acc,
                    "net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap",
                ),
            ),
            net_realized_pnl_rel_to_realized_cap: BlockCountPattern::new(
                client.clone(),
                _m(&acc, "net_realized_pnl_rel_to_realized_cap"),
            ),
            realized_cap: MetricPattern1::new(client.clone(), _m(&acc, "realized_cap")),
            realized_cap_30d_delta: MetricPattern4::new(
                client.clone(),
                _m(&acc, "realized_cap_30d_delta"),
            ),
            realized_loss: BlockCountPattern::new(client.clone(), _m(&acc, "realized_loss")),
            realized_loss_rel_to_realized_cap: BlockCountPattern::new(
                client.clone(),
                _m(&acc, "realized_loss_rel_to_realized_cap"),
            ),
            realized_price: MetricPattern1::new(client.clone(), _m(&acc, "realized_price")),
            realized_price_extra: RealizedPriceExtraPattern::new(
                client.clone(),
                _m(&acc, "realized_price_ratio"),
            ),
            realized_profit: BlockCountPattern::new(client.clone(), _m(&acc, "realized_profit")),
            realized_profit_rel_to_realized_cap: BlockCountPattern::new(
                client.clone(),
                _m(&acc, "realized_profit_rel_to_realized_cap"),
            ),
            realized_value: MetricPattern1::new(client.clone(), _m(&acc, "realized_value")),
            sell_side_risk_ratio: MetricPattern6::new(
                client.clone(),
                _m(&acc, "sell_side_risk_ratio"),
            ),
            sell_side_risk_ratio_30d_ema: MetricPattern6::new(
                client.clone(),
                _m(&acc, "sell_side_risk_ratio_30d_ema"),
            ),
            sell_side_risk_ratio_7d_ema: MetricPattern6::new(
                client.clone(),
                _m(&acc, "sell_side_risk_ratio_7d_ema"),
            ),
            sopr: MetricPattern6::new(client.clone(), _m(&acc, "sopr")),
            sopr_30d_ema: MetricPattern6::new(client.clone(), _m(&acc, "sopr_30d_ema")),
            sopr_7d_ema: MetricPattern6::new(client.clone(), _m(&acc, "sopr_7d_ema")),
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
    pub neg_realized_loss: BitcoinPattern2<Dollars>,
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
    pub realized_profit_to_loss_ratio: MetricPattern6<StoredF64>,
    pub realized_value: MetricPattern1<Dollars>,
    pub sell_side_risk_ratio: MetricPattern6<StoredF32>,
    pub sell_side_risk_ratio_30d_ema: MetricPattern6<StoredF32>,
    pub sell_side_risk_ratio_7d_ema: MetricPattern6<StoredF32>,
    pub sopr: MetricPattern6<StoredF64>,
    pub sopr_30d_ema: MetricPattern6<StoredF64>,
    pub sopr_7d_ema: MetricPattern6<StoredF64>,
    pub total_realized_pnl: MetricPattern1<Dollars>,
    pub value_created: MetricPattern1<Dollars>,
    pub value_destroyed: MetricPattern1<Dollars>,
}

impl RealizedPattern2 {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            mvrv: MetricPattern4::new(client.clone(), _m(&acc, "mvrv")),
            neg_realized_loss: BitcoinPattern2::new(client.clone(), _m(&acc, "neg_realized_loss")),
            net_realized_pnl: BlockCountPattern::new(client.clone(), _m(&acc, "net_realized_pnl")),
            net_realized_pnl_cumulative_30d_delta: MetricPattern4::new(
                client.clone(),
                _m(&acc, "net_realized_pnl_cumulative_30d_delta"),
            ),
            net_realized_pnl_cumulative_30d_delta_rel_to_market_cap: MetricPattern4::new(
                client.clone(),
                _m(
                    &acc,
                    "net_realized_pnl_cumulative_30d_delta_rel_to_market_cap",
                ),
            ),
            net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap: MetricPattern4::new(
                client.clone(),
                _m(
                    &acc,
                    "net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap",
                ),
            ),
            net_realized_pnl_rel_to_realized_cap: BlockCountPattern::new(
                client.clone(),
                _m(&acc, "net_realized_pnl_rel_to_realized_cap"),
            ),
            realized_cap: MetricPattern1::new(client.clone(), _m(&acc, "realized_cap")),
            realized_cap_30d_delta: MetricPattern4::new(
                client.clone(),
                _m(&acc, "realized_cap_30d_delta"),
            ),
            realized_cap_rel_to_own_market_cap: MetricPattern1::new(
                client.clone(),
                _m(&acc, "realized_cap_rel_to_own_market_cap"),
            ),
            realized_loss: BlockCountPattern::new(client.clone(), _m(&acc, "realized_loss")),
            realized_loss_rel_to_realized_cap: BlockCountPattern::new(
                client.clone(),
                _m(&acc, "realized_loss_rel_to_realized_cap"),
            ),
            realized_price: MetricPattern1::new(client.clone(), _m(&acc, "realized_price")),
            realized_price_extra: ActivePriceRatioPattern::new(
                client.clone(),
                _m(&acc, "realized_price_ratio"),
            ),
            realized_profit: BlockCountPattern::new(client.clone(), _m(&acc, "realized_profit")),
            realized_profit_rel_to_realized_cap: BlockCountPattern::new(
                client.clone(),
                _m(&acc, "realized_profit_rel_to_realized_cap"),
            ),
            realized_profit_to_loss_ratio: MetricPattern6::new(
                client.clone(),
                _m(&acc, "realized_profit_to_loss_ratio"),
            ),
            realized_value: MetricPattern1::new(client.clone(), _m(&acc, "realized_value")),
            sell_side_risk_ratio: MetricPattern6::new(
                client.clone(),
                _m(&acc, "sell_side_risk_ratio"),
            ),
            sell_side_risk_ratio_30d_ema: MetricPattern6::new(
                client.clone(),
                _m(&acc, "sell_side_risk_ratio_30d_ema"),
            ),
            sell_side_risk_ratio_7d_ema: MetricPattern6::new(
                client.clone(),
                _m(&acc, "sell_side_risk_ratio_7d_ema"),
            ),
            sopr: MetricPattern6::new(client.clone(), _m(&acc, "sopr")),
            sopr_30d_ema: MetricPattern6::new(client.clone(), _m(&acc, "sopr_30d_ema")),
            sopr_7d_ema: MetricPattern6::new(client.clone(), _m(&acc, "sopr_7d_ema")),
            total_realized_pnl: MetricPattern1::new(client.clone(), _m(&acc, "total_realized_pnl")),
            value_created: MetricPattern1::new(client.clone(), _m(&acc, "value_created")),
            value_destroyed: MetricPattern1::new(client.clone(), _m(&acc, "value_destroyed")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct RealizedPattern {
    pub mvrv: MetricPattern4<StoredF32>,
    pub neg_realized_loss: BitcoinPattern2<Dollars>,
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
    pub sell_side_risk_ratio: MetricPattern6<StoredF32>,
    pub sell_side_risk_ratio_30d_ema: MetricPattern6<StoredF32>,
    pub sell_side_risk_ratio_7d_ema: MetricPattern6<StoredF32>,
    pub sopr: MetricPattern6<StoredF64>,
    pub sopr_30d_ema: MetricPattern6<StoredF64>,
    pub sopr_7d_ema: MetricPattern6<StoredF64>,
    pub total_realized_pnl: MetricPattern1<Dollars>,
    pub value_created: MetricPattern1<Dollars>,
    pub value_destroyed: MetricPattern1<Dollars>,
}

impl RealizedPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            mvrv: MetricPattern4::new(client.clone(), _m(&acc, "mvrv")),
            neg_realized_loss: BitcoinPattern2::new(client.clone(), _m(&acc, "neg_realized_loss")),
            net_realized_pnl: BlockCountPattern::new(client.clone(), _m(&acc, "net_realized_pnl")),
            net_realized_pnl_cumulative_30d_delta: MetricPattern4::new(
                client.clone(),
                _m(&acc, "net_realized_pnl_cumulative_30d_delta"),
            ),
            net_realized_pnl_cumulative_30d_delta_rel_to_market_cap: MetricPattern4::new(
                client.clone(),
                _m(
                    &acc,
                    "net_realized_pnl_cumulative_30d_delta_rel_to_market_cap",
                ),
            ),
            net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap: MetricPattern4::new(
                client.clone(),
                _m(
                    &acc,
                    "net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap",
                ),
            ),
            net_realized_pnl_rel_to_realized_cap: BlockCountPattern::new(
                client.clone(),
                _m(&acc, "net_realized_pnl_rel_to_realized_cap"),
            ),
            realized_cap: MetricPattern1::new(client.clone(), _m(&acc, "realized_cap")),
            realized_cap_30d_delta: MetricPattern4::new(
                client.clone(),
                _m(&acc, "realized_cap_30d_delta"),
            ),
            realized_loss: BlockCountPattern::new(client.clone(), _m(&acc, "realized_loss")),
            realized_loss_rel_to_realized_cap: BlockCountPattern::new(
                client.clone(),
                _m(&acc, "realized_loss_rel_to_realized_cap"),
            ),
            realized_price: MetricPattern1::new(client.clone(), _m(&acc, "realized_price")),
            realized_price_extra: RealizedPriceExtraPattern::new(
                client.clone(),
                _m(&acc, "realized_price_ratio"),
            ),
            realized_profit: BlockCountPattern::new(client.clone(), _m(&acc, "realized_profit")),
            realized_profit_rel_to_realized_cap: BlockCountPattern::new(
                client.clone(),
                _m(&acc, "realized_profit_rel_to_realized_cap"),
            ),
            realized_value: MetricPattern1::new(client.clone(), _m(&acc, "realized_value")),
            sell_side_risk_ratio: MetricPattern6::new(
                client.clone(),
                _m(&acc, "sell_side_risk_ratio"),
            ),
            sell_side_risk_ratio_30d_ema: MetricPattern6::new(
                client.clone(),
                _m(&acc, "sell_side_risk_ratio_30d_ema"),
            ),
            sell_side_risk_ratio_7d_ema: MetricPattern6::new(
                client.clone(),
                _m(&acc, "sell_side_risk_ratio_7d_ema"),
            ),
            sopr: MetricPattern6::new(client.clone(), _m(&acc, "sopr")),
            sopr_30d_ema: MetricPattern6::new(client.clone(), _m(&acc, "sopr_30d_ema")),
            sopr_7d_ema: MetricPattern6::new(client.clone(), _m(&acc, "sopr_7d_ema")),
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
pub struct PercentilesPattern {
    pub pct05: MetricPattern4<Dollars>,
    pub pct10: MetricPattern4<Dollars>,
    pub pct15: MetricPattern4<Dollars>,
    pub pct20: MetricPattern4<Dollars>,
    pub pct25: MetricPattern4<Dollars>,
    pub pct30: MetricPattern4<Dollars>,
    pub pct35: MetricPattern4<Dollars>,
    pub pct40: MetricPattern4<Dollars>,
    pub pct45: MetricPattern4<Dollars>,
    pub pct50: MetricPattern4<Dollars>,
    pub pct55: MetricPattern4<Dollars>,
    pub pct60: MetricPattern4<Dollars>,
    pub pct65: MetricPattern4<Dollars>,
    pub pct70: MetricPattern4<Dollars>,
    pub pct75: MetricPattern4<Dollars>,
    pub pct80: MetricPattern4<Dollars>,
    pub pct85: MetricPattern4<Dollars>,
    pub pct90: MetricPattern4<Dollars>,
    pub pct95: MetricPattern4<Dollars>,
}

impl PercentilesPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            pct05: MetricPattern4::new(client.clone(), _m(&acc, "pct05")),
            pct10: MetricPattern4::new(client.clone(), _m(&acc, "pct10")),
            pct15: MetricPattern4::new(client.clone(), _m(&acc, "pct15")),
            pct20: MetricPattern4::new(client.clone(), _m(&acc, "pct20")),
            pct25: MetricPattern4::new(client.clone(), _m(&acc, "pct25")),
            pct30: MetricPattern4::new(client.clone(), _m(&acc, "pct30")),
            pct35: MetricPattern4::new(client.clone(), _m(&acc, "pct35")),
            pct40: MetricPattern4::new(client.clone(), _m(&acc, "pct40")),
            pct45: MetricPattern4::new(client.clone(), _m(&acc, "pct45")),
            pct50: MetricPattern4::new(client.clone(), _m(&acc, "pct50")),
            pct55: MetricPattern4::new(client.clone(), _m(&acc, "pct55")),
            pct60: MetricPattern4::new(client.clone(), _m(&acc, "pct60")),
            pct65: MetricPattern4::new(client.clone(), _m(&acc, "pct65")),
            pct70: MetricPattern4::new(client.clone(), _m(&acc, "pct70")),
            pct75: MetricPattern4::new(client.clone(), _m(&acc, "pct75")),
            pct80: MetricPattern4::new(client.clone(), _m(&acc, "pct80")),
            pct85: MetricPattern4::new(client.clone(), _m(&acc, "pct85")),
            pct90: MetricPattern4::new(client.clone(), _m(&acc, "pct90")),
            pct95: MetricPattern4::new(client.clone(), _m(&acc, "pct95")),
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
            neg_unrealized_loss_rel_to_market_cap: MetricPattern1::new(
                client.clone(),
                _m(&acc, "neg_unrealized_loss_rel_to_market_cap"),
            ),
            neg_unrealized_loss_rel_to_own_market_cap: MetricPattern1::new(
                client.clone(),
                _m(&acc, "neg_unrealized_loss_rel_to_own_market_cap"),
            ),
            neg_unrealized_loss_rel_to_own_total_unrealized_pnl: MetricPattern1::new(
                client.clone(),
                _m(&acc, "neg_unrealized_loss_rel_to_own_total_unrealized_pnl"),
            ),
            net_unrealized_pnl_rel_to_market_cap: MetricPattern1::new(
                client.clone(),
                _m(&acc, "net_unrealized_pnl_rel_to_market_cap"),
            ),
            net_unrealized_pnl_rel_to_own_market_cap: MetricPattern1::new(
                client.clone(),
                _m(&acc, "net_unrealized_pnl_rel_to_own_market_cap"),
            ),
            net_unrealized_pnl_rel_to_own_total_unrealized_pnl: MetricPattern1::new(
                client.clone(),
                _m(&acc, "net_unrealized_pnl_rel_to_own_total_unrealized_pnl"),
            ),
            nupl: MetricPattern1::new(client.clone(), _m(&acc, "nupl")),
            supply_in_loss_rel_to_circulating_supply: MetricPattern1::new(
                client.clone(),
                _m(&acc, "supply_in_loss_rel_to_circulating_supply"),
            ),
            supply_in_loss_rel_to_own_supply: MetricPattern1::new(
                client.clone(),
                _m(&acc, "supply_in_loss_rel_to_own_supply"),
            ),
            supply_in_profit_rel_to_circulating_supply: MetricPattern1::new(
                client.clone(),
                _m(&acc, "supply_in_profit_rel_to_circulating_supply"),
            ),
            supply_in_profit_rel_to_own_supply: MetricPattern1::new(
                client.clone(),
                _m(&acc, "supply_in_profit_rel_to_own_supply"),
            ),
            supply_rel_to_circulating_supply: MetricPattern4::new(
                client.clone(),
                _m(&acc, "supply_rel_to_circulating_supply"),
            ),
            unrealized_loss_rel_to_market_cap: MetricPattern1::new(
                client.clone(),
                _m(&acc, "unrealized_loss_rel_to_market_cap"),
            ),
            unrealized_loss_rel_to_own_market_cap: MetricPattern1::new(
                client.clone(),
                _m(&acc, "unrealized_loss_rel_to_own_market_cap"),
            ),
            unrealized_loss_rel_to_own_total_unrealized_pnl: MetricPattern1::new(
                client.clone(),
                _m(&acc, "unrealized_loss_rel_to_own_total_unrealized_pnl"),
            ),
            unrealized_profit_rel_to_market_cap: MetricPattern1::new(
                client.clone(),
                _m(&acc, "unrealized_profit_rel_to_market_cap"),
            ),
            unrealized_profit_rel_to_own_market_cap: MetricPattern1::new(
                client.clone(),
                _m(&acc, "unrealized_profit_rel_to_own_market_cap"),
            ),
            unrealized_profit_rel_to_own_total_unrealized_pnl: MetricPattern1::new(
                client.clone(),
                _m(&acc, "unrealized_profit_rel_to_own_total_unrealized_pnl"),
            ),
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
    pub coinbase: CoinbasePattern2,
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
            coinbase: CoinbasePattern2::new(client.clone(), _m(&acc, "coinbase")),
            days_since_block: MetricPattern4::new(client.clone(), _m(&acc, "days_since_block")),
            dominance: MetricPattern1::new(client.clone(), _m(&acc, "dominance")),
            fee: UnclaimedRewardsPattern::new(client.clone(), _m(&acc, "fee")),
            subsidy: UnclaimedRewardsPattern::new(client.clone(), _m(&acc, "subsidy")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct LookbackPattern<T> {
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

impl<T: DeserializeOwned> LookbackPattern<T> {
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
    pub _10y: _2015Pattern,
    pub _1m: _2015Pattern,
    pub _1w: _2015Pattern,
    pub _1y: _2015Pattern,
    pub _2y: _2015Pattern,
    pub _3m: _2015Pattern,
    pub _3y: _2015Pattern,
    pub _4y: _2015Pattern,
    pub _5y: _2015Pattern,
    pub _6m: _2015Pattern,
    pub _6y: _2015Pattern,
    pub _8y: _2015Pattern,
}

impl PeriodLumpSumStackPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            _10y: _2015Pattern::new(client.clone(), _p("10y", &acc)),
            _1m: _2015Pattern::new(client.clone(), _p("1m", &acc)),
            _1w: _2015Pattern::new(client.clone(), _p("1w", &acc)),
            _1y: _2015Pattern::new(client.clone(), _p("1y", &acc)),
            _2y: _2015Pattern::new(client.clone(), _p("2y", &acc)),
            _3m: _2015Pattern::new(client.clone(), _p("3m", &acc)),
            _3y: _2015Pattern::new(client.clone(), _p("3y", &acc)),
            _4y: _2015Pattern::new(client.clone(), _p("4y", &acc)),
            _5y: _2015Pattern::new(client.clone(), _p("5y", &acc)),
            _6m: _2015Pattern::new(client.clone(), _p("6m", &acc)),
            _6y: _2015Pattern::new(client.clone(), _p("6y", &acc)),
            _8y: _2015Pattern::new(client.clone(), _p("8y", &acc)),
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
            _10y: MetricPattern4::new(client.clone(), _p("10y", &acc)),
            _1m: MetricPattern4::new(client.clone(), _p("1m", &acc)),
            _1w: MetricPattern4::new(client.clone(), _p("1w", &acc)),
            _1y: MetricPattern4::new(client.clone(), _p("1y", &acc)),
            _2y: MetricPattern4::new(client.clone(), _p("2y", &acc)),
            _3m: MetricPattern4::new(client.clone(), _p("3m", &acc)),
            _3y: MetricPattern4::new(client.clone(), _p("3y", &acc)),
            _4y: MetricPattern4::new(client.clone(), _p("4y", &acc)),
            _5y: MetricPattern4::new(client.clone(), _p("5y", &acc)),
            _6m: MetricPattern4::new(client.clone(), _p("6m", &acc)),
            _6y: MetricPattern4::new(client.clone(), _p("6y", &acc)),
            _8y: MetricPattern4::new(client.clone(), _p("8y", &acc)),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct BitcoinPattern {
    pub average: MetricPattern2<Bitcoin>,
    pub base: MetricPattern11<Bitcoin>,
    pub cumulative: MetricPattern2<Bitcoin>,
    pub max: MetricPattern2<Bitcoin>,
    pub median: MetricPattern6<Bitcoin>,
    pub min: MetricPattern2<Bitcoin>,
    pub pct10: MetricPattern6<Bitcoin>,
    pub pct25: MetricPattern6<Bitcoin>,
    pub pct75: MetricPattern6<Bitcoin>,
    pub pct90: MetricPattern6<Bitcoin>,
    pub sum: MetricPattern2<Bitcoin>,
}

impl BitcoinPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            average: MetricPattern2::new(client.clone(), _m(&acc, "average")),
            base: MetricPattern11::new(client.clone(), acc.clone()),
            cumulative: MetricPattern2::new(client.clone(), _m(&acc, "cumulative")),
            max: MetricPattern2::new(client.clone(), _m(&acc, "max")),
            median: MetricPattern6::new(client.clone(), _m(&acc, "median")),
            min: MetricPattern2::new(client.clone(), _m(&acc, "min")),
            pct10: MetricPattern6::new(client.clone(), _m(&acc, "pct10")),
            pct25: MetricPattern6::new(client.clone(), _m(&acc, "pct25")),
            pct75: MetricPattern6::new(client.clone(), _m(&acc, "pct75")),
            pct90: MetricPattern6::new(client.clone(), _m(&acc, "pct90")),
            sum: MetricPattern2::new(client.clone(), _m(&acc, "sum")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct DollarsPattern<T> {
    pub average: MetricPattern2<T>,
    pub base: MetricPattern11<T>,
    pub cumulative: MetricPattern1<T>,
    pub max: MetricPattern2<T>,
    pub median: MetricPattern6<T>,
    pub min: MetricPattern2<T>,
    pub pct10: MetricPattern6<T>,
    pub pct25: MetricPattern6<T>,
    pub pct75: MetricPattern6<T>,
    pub pct90: MetricPattern6<T>,
    pub sum: MetricPattern2<T>,
}

impl<T: DeserializeOwned> DollarsPattern<T> {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            average: MetricPattern2::new(client.clone(), _m(&acc, "average")),
            base: MetricPattern11::new(client.clone(), acc.clone()),
            cumulative: MetricPattern1::new(client.clone(), _m(&acc, "cumulative")),
            max: MetricPattern2::new(client.clone(), _m(&acc, "max")),
            median: MetricPattern6::new(client.clone(), _m(&acc, "median")),
            min: MetricPattern2::new(client.clone(), _m(&acc, "min")),
            pct10: MetricPattern6::new(client.clone(), _m(&acc, "pct10")),
            pct25: MetricPattern6::new(client.clone(), _m(&acc, "pct25")),
            pct75: MetricPattern6::new(client.clone(), _m(&acc, "pct75")),
            pct90: MetricPattern6::new(client.clone(), _m(&acc, "pct90")),
            sum: MetricPattern2::new(client.clone(), _m(&acc, "sum")),
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
            _2015: MetricPattern4::new(client.clone(), _m(&acc, "2015_returns")),
            _2016: MetricPattern4::new(client.clone(), _m(&acc, "2016_returns")),
            _2017: MetricPattern4::new(client.clone(), _m(&acc, "2017_returns")),
            _2018: MetricPattern4::new(client.clone(), _m(&acc, "2018_returns")),
            _2019: MetricPattern4::new(client.clone(), _m(&acc, "2019_returns")),
            _2020: MetricPattern4::new(client.clone(), _m(&acc, "2020_returns")),
            _2021: MetricPattern4::new(client.clone(), _m(&acc, "2021_returns")),
            _2022: MetricPattern4::new(client.clone(), _m(&acc, "2022_returns")),
            _2023: MetricPattern4::new(client.clone(), _m(&acc, "2023_returns")),
            _2024: MetricPattern4::new(client.clone(), _m(&acc, "2024_returns")),
            _2025: MetricPattern4::new(client.clone(), _m(&acc, "2025_returns")),
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
            neg_unrealized_loss_rel_to_market_cap: MetricPattern1::new(
                client.clone(),
                _m(&acc, "neg_unrealized_loss_rel_to_market_cap"),
            ),
            net_unrealized_pnl_rel_to_market_cap: MetricPattern1::new(
                client.clone(),
                _m(&acc, "net_unrealized_pnl_rel_to_market_cap"),
            ),
            nupl: MetricPattern1::new(client.clone(), _m(&acc, "nupl")),
            supply_in_loss_rel_to_circulating_supply: MetricPattern1::new(
                client.clone(),
                _m(&acc, "supply_in_loss_rel_to_circulating_supply"),
            ),
            supply_in_loss_rel_to_own_supply: MetricPattern1::new(
                client.clone(),
                _m(&acc, "supply_in_loss_rel_to_own_supply"),
            ),
            supply_in_profit_rel_to_circulating_supply: MetricPattern1::new(
                client.clone(),
                _m(&acc, "supply_in_profit_rel_to_circulating_supply"),
            ),
            supply_in_profit_rel_to_own_supply: MetricPattern1::new(
                client.clone(),
                _m(&acc, "supply_in_profit_rel_to_own_supply"),
            ),
            supply_rel_to_circulating_supply: MetricPattern4::new(
                client.clone(),
                _m(&acc, "supply_rel_to_circulating_supply"),
            ),
            unrealized_loss_rel_to_market_cap: MetricPattern1::new(
                client.clone(),
                _m(&acc, "unrealized_loss_rel_to_market_cap"),
            ),
            unrealized_profit_rel_to_market_cap: MetricPattern1::new(
                client.clone(),
                _m(&acc, "unrealized_profit_rel_to_market_cap"),
            ),
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
            neg_unrealized_loss_rel_to_own_market_cap: MetricPattern1::new(
                client.clone(),
                _m(&acc, "neg_unrealized_loss_rel_to_own_market_cap"),
            ),
            neg_unrealized_loss_rel_to_own_total_unrealized_pnl: MetricPattern1::new(
                client.clone(),
                _m(&acc, "neg_unrealized_loss_rel_to_own_total_unrealized_pnl"),
            ),
            net_unrealized_pnl_rel_to_own_market_cap: MetricPattern1::new(
                client.clone(),
                _m(&acc, "net_unrealized_pnl_rel_to_own_market_cap"),
            ),
            net_unrealized_pnl_rel_to_own_total_unrealized_pnl: MetricPattern1::new(
                client.clone(),
                _m(&acc, "net_unrealized_pnl_rel_to_own_total_unrealized_pnl"),
            ),
            supply_in_loss_rel_to_own_supply: MetricPattern1::new(
                client.clone(),
                _m(&acc, "supply_in_loss_rel_to_own_supply"),
            ),
            supply_in_profit_rel_to_own_supply: MetricPattern1::new(
                client.clone(),
                _m(&acc, "supply_in_profit_rel_to_own_supply"),
            ),
            unrealized_loss_rel_to_own_market_cap: MetricPattern1::new(
                client.clone(),
                _m(&acc, "unrealized_loss_rel_to_own_market_cap"),
            ),
            unrealized_loss_rel_to_own_total_unrealized_pnl: MetricPattern1::new(
                client.clone(),
                _m(&acc, "unrealized_loss_rel_to_own_total_unrealized_pnl"),
            ),
            unrealized_profit_rel_to_own_market_cap: MetricPattern1::new(
                client.clone(),
                _m(&acc, "unrealized_profit_rel_to_own_market_cap"),
            ),
            unrealized_profit_rel_to_own_total_unrealized_pnl: MetricPattern1::new(
                client.clone(),
                _m(&acc, "unrealized_profit_rel_to_own_total_unrealized_pnl"),
            ),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct CountPattern2<T> {
    pub average: MetricPattern1<T>,
    pub cumulative: MetricPattern1<T>,
    pub max: MetricPattern1<T>,
    pub median: MetricPattern11<T>,
    pub min: MetricPattern1<T>,
    pub pct10: MetricPattern11<T>,
    pub pct25: MetricPattern11<T>,
    pub pct75: MetricPattern11<T>,
    pub pct90: MetricPattern11<T>,
    pub sum: MetricPattern1<T>,
}

impl<T: DeserializeOwned> CountPattern2<T> {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            average: MetricPattern1::new(client.clone(), _m(&acc, "average")),
            cumulative: MetricPattern1::new(client.clone(), _m(&acc, "cumulative")),
            max: MetricPattern1::new(client.clone(), _m(&acc, "max")),
            median: MetricPattern11::new(client.clone(), _m(&acc, "median")),
            min: MetricPattern1::new(client.clone(), _m(&acc, "min")),
            pct10: MetricPattern11::new(client.clone(), _m(&acc, "pct10")),
            pct25: MetricPattern11::new(client.clone(), _m(&acc, "pct25")),
            pct75: MetricPattern11::new(client.clone(), _m(&acc, "pct75")),
            pct90: MetricPattern11::new(client.clone(), _m(&acc, "pct90")),
            sum: MetricPattern1::new(client.clone(), _m(&acc, "sum")),
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
            all: MetricPattern1::new(client.clone(), acc.clone()),
            p2a: MetricPattern1::new(client.clone(), _p("p2a", &acc)),
            p2pk33: MetricPattern1::new(client.clone(), _p("p2pk33", &acc)),
            p2pk65: MetricPattern1::new(client.clone(), _p("p2pk65", &acc)),
            p2pkh: MetricPattern1::new(client.clone(), _p("p2pkh", &acc)),
            p2sh: MetricPattern1::new(client.clone(), _p("p2sh", &acc)),
            p2tr: MetricPattern1::new(client.clone(), _p("p2tr", &acc)),
            p2wpkh: MetricPattern1::new(client.clone(), _p("p2wpkh", &acc)),
            p2wsh: MetricPattern1::new(client.clone(), _p("p2wsh", &acc)),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct FeeRatePattern<T> {
    pub average: MetricPattern1<T>,
    pub max: MetricPattern1<T>,
    pub median: MetricPattern11<T>,
    pub min: MetricPattern1<T>,
    pub pct10: MetricPattern11<T>,
    pub pct25: MetricPattern11<T>,
    pub pct75: MetricPattern11<T>,
    pub pct90: MetricPattern11<T>,
    pub txindex: MetricPattern27<T>,
}

impl<T: DeserializeOwned> FeeRatePattern<T> {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            average: MetricPattern1::new(client.clone(), _m(&acc, "average")),
            max: MetricPattern1::new(client.clone(), _m(&acc, "max")),
            median: MetricPattern11::new(client.clone(), _m(&acc, "median")),
            min: MetricPattern1::new(client.clone(), _m(&acc, "min")),
            pct10: MetricPattern11::new(client.clone(), _m(&acc, "pct10")),
            pct25: MetricPattern11::new(client.clone(), _m(&acc, "pct25")),
            pct75: MetricPattern11::new(client.clone(), _m(&acc, "pct75")),
            pct90: MetricPattern11::new(client.clone(), _m(&acc, "pct90")),
            txindex: MetricPattern27::new(client.clone(), acc.clone()),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct FullnessPattern<T> {
    pub average: MetricPattern2<T>,
    pub base: MetricPattern11<T>,
    pub max: MetricPattern2<T>,
    pub median: MetricPattern6<T>,
    pub min: MetricPattern2<T>,
    pub pct10: MetricPattern6<T>,
    pub pct25: MetricPattern6<T>,
    pub pct75: MetricPattern6<T>,
    pub pct90: MetricPattern6<T>,
}

impl<T: DeserializeOwned> FullnessPattern<T> {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            average: MetricPattern2::new(client.clone(), _m(&acc, "average")),
            base: MetricPattern11::new(client.clone(), acc.clone()),
            max: MetricPattern2::new(client.clone(), _m(&acc, "max")),
            median: MetricPattern6::new(client.clone(), _m(&acc, "median")),
            min: MetricPattern2::new(client.clone(), _m(&acc, "min")),
            pct10: MetricPattern6::new(client.clone(), _m(&acc, "pct10")),
            pct25: MetricPattern6::new(client.clone(), _m(&acc, "pct25")),
            pct75: MetricPattern6::new(client.clone(), _m(&acc, "pct75")),
            pct90: MetricPattern6::new(client.clone(), _m(&acc, "pct90")),
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
            outputs: OutputsPattern::new(client.clone(), _m(&acc, "utxo_count")),
            realized: RealizedPattern::new(client.clone(), acc.clone()),
            relative: RelativePattern::new(client.clone(), acc.clone()),
            supply: SupplyPattern2::new(client.clone(), _m(&acc, "supply")),
            unrealized: UnrealizedPattern::new(client.clone(), acc.clone()),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct PhaseDailyCentsPattern<T> {
    pub average: MetricPattern6<T>,
    pub max: MetricPattern6<T>,
    pub median: MetricPattern6<T>,
    pub min: MetricPattern6<T>,
    pub pct10: MetricPattern6<T>,
    pub pct25: MetricPattern6<T>,
    pub pct75: MetricPattern6<T>,
    pub pct90: MetricPattern6<T>,
}

impl<T: DeserializeOwned> PhaseDailyCentsPattern<T> {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            average: MetricPattern6::new(client.clone(), _m(&acc, "average")),
            max: MetricPattern6::new(client.clone(), _m(&acc, "max")),
            median: MetricPattern6::new(client.clone(), _m(&acc, "median")),
            min: MetricPattern6::new(client.clone(), _m(&acc, "min")),
            pct10: MetricPattern6::new(client.clone(), _m(&acc, "pct10")),
            pct25: MetricPattern6::new(client.clone(), _m(&acc, "pct25")),
            pct75: MetricPattern6::new(client.clone(), _m(&acc, "pct75")),
            pct90: MetricPattern6::new(client.clone(), _m(&acc, "pct90")),
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
            _10y: MetricPattern4::new(client.clone(), _p("10y", &acc)),
            _2y: MetricPattern4::new(client.clone(), _p("2y", &acc)),
            _3y: MetricPattern4::new(client.clone(), _p("3y", &acc)),
            _4y: MetricPattern4::new(client.clone(), _p("4y", &acc)),
            _5y: MetricPattern4::new(client.clone(), _p("5y", &acc)),
            _6y: MetricPattern4::new(client.clone(), _p("6y", &acc)),
            _8y: MetricPattern4::new(client.clone(), _p("8y", &acc)),
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
            outputs: OutputsPattern::new(client.clone(), _m(&acc, "utxo_count")),
            realized: RealizedPattern::new(client.clone(), acc.clone()),
            relative: RelativePattern4::new(client.clone(), _m(&acc, "supply_in")),
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
            outputs: OutputsPattern::new(client.clone(), _m(&acc, "utxo_count")),
            realized: RealizedPattern4::new(client.clone(), acc.clone()),
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
            outputs: OutputsPattern::new(client.clone(), _m(&acc, "utxo_count")),
            realized: RealizedPattern2::new(client.clone(), acc.clone()),
            relative: RelativePattern2::new(client.clone(), acc.clone()),
            supply: SupplyPattern2::new(client.clone(), _m(&acc, "supply")),
            unrealized: UnrealizedPattern::new(client.clone(), acc.clone()),
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
            outputs: OutputsPattern::new(client.clone(), _m(&acc, "utxo_count")),
            realized: RealizedPattern::new(client.clone(), acc.clone()),
            relative: RelativePattern::new(client.clone(), acc.clone()),
            supply: SupplyPattern2::new(client.clone(), _m(&acc, "supply")),
            unrealized: UnrealizedPattern::new(client.clone(), acc.clone()),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct UnrealizedPattern {
    pub neg_unrealized_loss: MetricPattern1<Dollars>,
    pub net_unrealized_pnl: MetricPattern1<Dollars>,
    pub supply_in_loss: ActiveSupplyPattern,
    pub supply_in_profit: ActiveSupplyPattern,
    pub total_unrealized_pnl: MetricPattern1<Dollars>,
    pub unrealized_loss: MetricPattern1<Dollars>,
    pub unrealized_profit: MetricPattern1<Dollars>,
}

impl UnrealizedPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            neg_unrealized_loss: MetricPattern1::new(
                client.clone(),
                _m(&acc, "neg_unrealized_loss"),
            ),
            net_unrealized_pnl: MetricPattern1::new(client.clone(), _m(&acc, "net_unrealized_pnl")),
            supply_in_loss: ActiveSupplyPattern::new(client.clone(), _m(&acc, "supply_in_loss")),
            supply_in_profit: ActiveSupplyPattern::new(
                client.clone(),
                _m(&acc, "supply_in_profit"),
            ),
            total_unrealized_pnl: MetricPattern1::new(
                client.clone(),
                _m(&acc, "total_unrealized_pnl"),
            ),
            unrealized_loss: MetricPattern1::new(client.clone(), _m(&acc, "unrealized_loss")),
            unrealized_profit: MetricPattern1::new(client.clone(), _m(&acc, "unrealized_profit")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct ActivityPattern2 {
    pub coinblocks_destroyed: BlockCountPattern<StoredF64>,
    pub coindays_destroyed: BlockCountPattern<StoredF64>,
    pub satblocks_destroyed: MetricPattern11<Sats>,
    pub satdays_destroyed: MetricPattern11<Sats>,
    pub sent: UnclaimedRewardsPattern,
}

impl ActivityPattern2 {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            coinblocks_destroyed: BlockCountPattern::new(
                client.clone(),
                _m(&acc, "coinblocks_destroyed"),
            ),
            coindays_destroyed: BlockCountPattern::new(
                client.clone(),
                _m(&acc, "coindays_destroyed"),
            ),
            satblocks_destroyed: MetricPattern11::new(
                client.clone(),
                _m(&acc, "satblocks_destroyed"),
            ),
            satdays_destroyed: MetricPattern11::new(client.clone(), _m(&acc, "satdays_destroyed")),
            sent: UnclaimedRewardsPattern::new(client.clone(), _m(&acc, "sent")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct SplitPattern2<T> {
    pub close: MetricPattern1<T>,
    pub high: MetricPattern1<T>,
    pub low: MetricPattern1<T>,
    pub open: MetricPattern1<T>,
}

impl<T: DeserializeOwned> SplitPattern2<T> {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            close: MetricPattern1::new(client.clone(), _m(&acc, "close")),
            high: MetricPattern1::new(client.clone(), _m(&acc, "high")),
            low: MetricPattern1::new(client.clone(), _m(&acc, "low")),
            open: MetricPattern1::new(client.clone(), _m(&acc, "open")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct CostBasisPattern2 {
    pub max: MetricPattern1<Dollars>,
    pub min: MetricPattern1<Dollars>,
    pub percentiles: PercentilesPattern,
}

impl CostBasisPattern2 {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            max: MetricPattern1::new(client.clone(), _m(&acc, "max_cost_basis")),
            min: MetricPattern1::new(client.clone(), _m(&acc, "min_cost_basis")),
            percentiles: PercentilesPattern::new(client.clone(), _m(&acc, "cost_basis")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct CoinbasePattern {
    pub bitcoin: BitcoinPattern,
    pub dollars: DollarsPattern<Dollars>,
    pub sats: DollarsPattern<Sats>,
}

impl CoinbasePattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            bitcoin: BitcoinPattern::new(client.clone(), _m(&acc, "btc")),
            dollars: DollarsPattern::new(client.clone(), _m(&acc, "usd")),
            sats: DollarsPattern::new(client.clone(), acc.clone()),
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
pub struct UnclaimedRewardsPattern {
    pub bitcoin: BitcoinPattern2<Bitcoin>,
    pub dollars: BlockCountPattern<Dollars>,
    pub sats: BlockCountPattern<Sats>,
}

impl UnclaimedRewardsPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            bitcoin: BitcoinPattern2::new(client.clone(), _m(&acc, "btc")),
            dollars: BlockCountPattern::new(client.clone(), _m(&acc, "usd")),
            sats: BlockCountPattern::new(client.clone(), acc.clone()),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct CoinbasePattern2 {
    pub bitcoin: BlockCountPattern<Bitcoin>,
    pub dollars: BlockCountPattern<Dollars>,
    pub sats: BlockCountPattern<Sats>,
}

impl CoinbasePattern2 {
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
pub struct _2015Pattern {
    pub bitcoin: MetricPattern4<Bitcoin>,
    pub dollars: MetricPattern4<Dollars>,
    pub sats: MetricPattern4<Sats>,
}

impl _2015Pattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            bitcoin: MetricPattern4::new(client.clone(), _m(&acc, "btc")),
            dollars: MetricPattern4::new(client.clone(), _m(&acc, "usd")),
            sats: MetricPattern4::new(client.clone(), acc.clone()),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct SegwitAdoptionPattern {
    pub base: MetricPattern11<StoredF32>,
    pub cumulative: MetricPattern2<StoredF32>,
    pub sum: MetricPattern2<StoredF32>,
}

impl SegwitAdoptionPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            base: MetricPattern11::new(client.clone(), acc.clone()),
            cumulative: MetricPattern2::new(client.clone(), _m(&acc, "cumulative")),
            sum: MetricPattern2::new(client.clone(), _m(&acc, "sum")),
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
pub struct RelativePattern4 {
    pub supply_in_loss_rel_to_own_supply: MetricPattern1<StoredF64>,
    pub supply_in_profit_rel_to_own_supply: MetricPattern1<StoredF64>,
}

impl RelativePattern4 {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            supply_in_loss_rel_to_own_supply: MetricPattern1::new(
                client.clone(),
                _m(&acc, "loss_rel_to_own_supply"),
            ),
            supply_in_profit_rel_to_own_supply: MetricPattern1::new(
                client.clone(),
                _m(&acc, "profit_rel_to_own_supply"),
            ),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct SupplyPattern2 {
    pub halved: ActiveSupplyPattern,
    pub total: ActiveSupplyPattern,
}

impl SupplyPattern2 {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            halved: ActiveSupplyPattern::new(client.clone(), _m(&acc, "halved")),
            total: ActiveSupplyPattern::new(client.clone(), acc.clone()),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct BitcoinPattern2<T> {
    pub cumulative: MetricPattern2<T>,
    pub sum: MetricPattern1<T>,
}

impl<T: DeserializeOwned> BitcoinPattern2<T> {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            cumulative: MetricPattern2::new(client.clone(), _m(&acc, "cumulative")),
            sum: MetricPattern1::new(client.clone(), acc.clone()),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct SatsPattern<T> {
    pub ohlc: MetricPattern1<T>,
    pub split: SplitPattern2<T>,
}

impl<T: DeserializeOwned> SatsPattern<T> {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            ohlc: MetricPattern1::new(client.clone(), _m(&acc, "ohlc_sats")),
            split: SplitPattern2::new(client.clone(), _m(&acc, "sats")),
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
            utxo_count: MetricPattern1::new(client.clone(), acc.clone()),
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
            ratio: MetricPattern4::new(client.clone(), acc.clone()),
        }
    }
}

// Metrics tree

/// Metrics tree node.
pub struct MetricsTree {
    pub addresses: MetricsTree_Addresses,
    pub blocks: MetricsTree_Blocks,
    pub cointime: MetricsTree_Cointime,
    pub constants: MetricsTree_Constants,
    pub distribution: MetricsTree_Distribution,
    pub indexes: MetricsTree_Indexes,
    pub inputs: MetricsTree_Inputs,
    pub market: MetricsTree_Market,
    pub outputs: MetricsTree_Outputs,
    pub pools: MetricsTree_Pools,
    pub positions: MetricsTree_Positions,
    pub price: MetricsTree_Price,
    pub scripts: MetricsTree_Scripts,
    pub supply: MetricsTree_Supply,
    pub transactions: MetricsTree_Transactions,
}

impl MetricsTree {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            addresses: MetricsTree_Addresses::new(client.clone(), format!("{base_path}_addresses")),
            blocks: MetricsTree_Blocks::new(client.clone(), format!("{base_path}_blocks")),
            cointime: MetricsTree_Cointime::new(client.clone(), format!("{base_path}_cointime")),
            constants: MetricsTree_Constants::new(client.clone(), format!("{base_path}_constants")),
            distribution: MetricsTree_Distribution::new(
                client.clone(),
                format!("{base_path}_distribution"),
            ),
            indexes: MetricsTree_Indexes::new(client.clone(), format!("{base_path}_indexes")),
            inputs: MetricsTree_Inputs::new(client.clone(), format!("{base_path}_inputs")),
            market: MetricsTree_Market::new(client.clone(), format!("{base_path}_market")),
            outputs: MetricsTree_Outputs::new(client.clone(), format!("{base_path}_outputs")),
            pools: MetricsTree_Pools::new(client.clone(), format!("{base_path}_pools")),
            positions: MetricsTree_Positions::new(client.clone(), format!("{base_path}_positions")),
            price: MetricsTree_Price::new(client.clone(), format!("{base_path}_price")),
            scripts: MetricsTree_Scripts::new(client.clone(), format!("{base_path}_scripts")),
            supply: MetricsTree_Supply::new(client.clone(), format!("{base_path}_supply")),
            transactions: MetricsTree_Transactions::new(
                client.clone(),
                format!("{base_path}_transactions"),
            ),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Addresses {
    pub first_p2aaddressindex: MetricPattern11<P2AAddressIndex>,
    pub first_p2pk33addressindex: MetricPattern11<P2PK33AddressIndex>,
    pub first_p2pk65addressindex: MetricPattern11<P2PK65AddressIndex>,
    pub first_p2pkhaddressindex: MetricPattern11<P2PKHAddressIndex>,
    pub first_p2shaddressindex: MetricPattern11<P2SHAddressIndex>,
    pub first_p2traddressindex: MetricPattern11<P2TRAddressIndex>,
    pub first_p2wpkhaddressindex: MetricPattern11<P2WPKHAddressIndex>,
    pub first_p2wshaddressindex: MetricPattern11<P2WSHAddressIndex>,
    pub p2abytes: MetricPattern16<P2ABytes>,
    pub p2pk33bytes: MetricPattern18<P2PK33Bytes>,
    pub p2pk65bytes: MetricPattern19<P2PK65Bytes>,
    pub p2pkhbytes: MetricPattern20<P2PKHBytes>,
    pub p2shbytes: MetricPattern21<P2SHBytes>,
    pub p2trbytes: MetricPattern22<P2TRBytes>,
    pub p2wpkhbytes: MetricPattern23<P2WPKHBytes>,
    pub p2wshbytes: MetricPattern24<P2WSHBytes>,
}

impl MetricsTree_Addresses {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            first_p2aaddressindex: MetricPattern11::new(
                client.clone(),
                "first_p2aaddressindex".to_string(),
            ),
            first_p2pk33addressindex: MetricPattern11::new(
                client.clone(),
                "first_p2pk33addressindex".to_string(),
            ),
            first_p2pk65addressindex: MetricPattern11::new(
                client.clone(),
                "first_p2pk65addressindex".to_string(),
            ),
            first_p2pkhaddressindex: MetricPattern11::new(
                client.clone(),
                "first_p2pkhaddressindex".to_string(),
            ),
            first_p2shaddressindex: MetricPattern11::new(
                client.clone(),
                "first_p2shaddressindex".to_string(),
            ),
            first_p2traddressindex: MetricPattern11::new(
                client.clone(),
                "first_p2traddressindex".to_string(),
            ),
            first_p2wpkhaddressindex: MetricPattern11::new(
                client.clone(),
                "first_p2wpkhaddressindex".to_string(),
            ),
            first_p2wshaddressindex: MetricPattern11::new(
                client.clone(),
                "first_p2wshaddressindex".to_string(),
            ),
            p2abytes: MetricPattern16::new(client.clone(), "p2abytes".to_string()),
            p2pk33bytes: MetricPattern18::new(client.clone(), "p2pk33bytes".to_string()),
            p2pk65bytes: MetricPattern19::new(client.clone(), "p2pk65bytes".to_string()),
            p2pkhbytes: MetricPattern20::new(client.clone(), "p2pkhbytes".to_string()),
            p2shbytes: MetricPattern21::new(client.clone(), "p2shbytes".to_string()),
            p2trbytes: MetricPattern22::new(client.clone(), "p2trbytes".to_string()),
            p2wpkhbytes: MetricPattern23::new(client.clone(), "p2wpkhbytes".to_string()),
            p2wshbytes: MetricPattern24::new(client.clone(), "p2wshbytes".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Blocks {
    pub blockhash: MetricPattern11<BlockHash>,
    pub count: MetricsTree_Blocks_Count,
    pub difficulty: MetricsTree_Blocks_Difficulty,
    pub fullness: FullnessPattern<StoredF32>,
    pub halving: MetricsTree_Blocks_Halving,
    pub interval: FullnessPattern<Timestamp>,
    pub mining: MetricsTree_Blocks_Mining,
    pub rewards: MetricsTree_Blocks_Rewards,
    pub size: MetricsTree_Blocks_Size,
    pub time: MetricsTree_Blocks_Time,
    pub total_size: MetricPattern11<StoredU64>,
    pub vbytes: DollarsPattern<StoredU64>,
    pub weight: DollarsPattern<Weight>,
}

impl MetricsTree_Blocks {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            blockhash: MetricPattern11::new(client.clone(), "blockhash".to_string()),
            count: MetricsTree_Blocks_Count::new(client.clone(), format!("{base_path}_count")),
            difficulty: MetricsTree_Blocks_Difficulty::new(
                client.clone(),
                format!("{base_path}_difficulty"),
            ),
            fullness: FullnessPattern::new(client.clone(), "block_fullness".to_string()),
            halving: MetricsTree_Blocks_Halving::new(
                client.clone(),
                format!("{base_path}_halving"),
            ),
            interval: FullnessPattern::new(client.clone(), "block_interval".to_string()),
            mining: MetricsTree_Blocks_Mining::new(client.clone(), format!("{base_path}_mining")),
            rewards: MetricsTree_Blocks_Rewards::new(
                client.clone(),
                format!("{base_path}_rewards"),
            ),
            size: MetricsTree_Blocks_Size::new(client.clone(), format!("{base_path}_size")),
            time: MetricsTree_Blocks_Time::new(client.clone(), format!("{base_path}_time")),
            total_size: MetricPattern11::new(client.clone(), "total_size".to_string()),
            vbytes: DollarsPattern::new(client.clone(), "block_vbytes".to_string()),
            weight: DollarsPattern::new(client.clone(), "block_weight".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Blocks_Count {
    pub _1m_block_count: MetricPattern1<StoredU32>,
    pub _1m_start: MetricPattern11<Height>,
    pub _1w_block_count: MetricPattern1<StoredU32>,
    pub _1w_start: MetricPattern11<Height>,
    pub _1y_block_count: MetricPattern1<StoredU32>,
    pub _1y_start: MetricPattern11<Height>,
    pub _24h_block_count: MetricPattern1<StoredU32>,
    pub _24h_start: MetricPattern11<Height>,
    pub block_count: BlockCountPattern<StoredU32>,
    pub block_count_target: MetricPattern4<StoredU64>,
}

impl MetricsTree_Blocks_Count {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _1m_block_count: MetricPattern1::new(client.clone(), "1m_block_count".to_string()),
            _1m_start: MetricPattern11::new(client.clone(), "1m_start".to_string()),
            _1w_block_count: MetricPattern1::new(client.clone(), "1w_block_count".to_string()),
            _1w_start: MetricPattern11::new(client.clone(), "1w_start".to_string()),
            _1y_block_count: MetricPattern1::new(client.clone(), "1y_block_count".to_string()),
            _1y_start: MetricPattern11::new(client.clone(), "1y_start".to_string()),
            _24h_block_count: MetricPattern1::new(client.clone(), "24h_block_count".to_string()),
            _24h_start: MetricPattern11::new(client.clone(), "24h_start".to_string()),
            block_count: BlockCountPattern::new(client.clone(), "block_count".to_string()),
            block_count_target: MetricPattern4::new(
                client.clone(),
                "block_count_target".to_string(),
            ),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Blocks_Difficulty {
    pub adjustment: MetricPattern1<StoredF32>,
    pub as_hash: MetricPattern1<StoredF32>,
    pub blocks_before_next_adjustment: MetricPattern1<StoredU32>,
    pub days_before_next_adjustment: MetricPattern1<StoredF32>,
    pub epoch: MetricPattern4<DifficultyEpoch>,
    pub raw: MetricPattern1<StoredF64>,
}

impl MetricsTree_Blocks_Difficulty {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            adjustment: MetricPattern1::new(client.clone(), "difficulty_adjustment".to_string()),
            as_hash: MetricPattern1::new(client.clone(), "difficulty_as_hash".to_string()),
            blocks_before_next_adjustment: MetricPattern1::new(
                client.clone(),
                "blocks_before_next_difficulty_adjustment".to_string(),
            ),
            days_before_next_adjustment: MetricPattern1::new(
                client.clone(),
                "days_before_next_difficulty_adjustment".to_string(),
            ),
            epoch: MetricPattern4::new(client.clone(), "difficultyepoch".to_string()),
            raw: MetricPattern1::new(client.clone(), "difficulty".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Blocks_Halving {
    pub blocks_before_next_halving: MetricPattern1<StoredU32>,
    pub days_before_next_halving: MetricPattern1<StoredF32>,
    pub epoch: MetricPattern4<HalvingEpoch>,
}

impl MetricsTree_Blocks_Halving {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            blocks_before_next_halving: MetricPattern1::new(
                client.clone(),
                "blocks_before_next_halving".to_string(),
            ),
            days_before_next_halving: MetricPattern1::new(
                client.clone(),
                "days_before_next_halving".to_string(),
            ),
            epoch: MetricPattern4::new(client.clone(), "halvingepoch".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Blocks_Mining {
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

impl MetricsTree_Blocks_Mining {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            hash_price_phs: MetricPattern1::new(client.clone(), "hash_price_phs".to_string()),
            hash_price_phs_min: MetricPattern1::new(
                client.clone(),
                "hash_price_phs_min".to_string(),
            ),
            hash_price_rebound: MetricPattern1::new(
                client.clone(),
                "hash_price_rebound".to_string(),
            ),
            hash_price_ths: MetricPattern1::new(client.clone(), "hash_price_ths".to_string()),
            hash_price_ths_min: MetricPattern1::new(
                client.clone(),
                "hash_price_ths_min".to_string(),
            ),
            hash_rate: MetricPattern1::new(client.clone(), "hash_rate".to_string()),
            hash_rate_1m_sma: MetricPattern4::new(client.clone(), "hash_rate_1m_sma".to_string()),
            hash_rate_1w_sma: MetricPattern4::new(client.clone(), "hash_rate_1w_sma".to_string()),
            hash_rate_1y_sma: MetricPattern4::new(client.clone(), "hash_rate_1y_sma".to_string()),
            hash_rate_2m_sma: MetricPattern4::new(client.clone(), "hash_rate_2m_sma".to_string()),
            hash_value_phs: MetricPattern1::new(client.clone(), "hash_value_phs".to_string()),
            hash_value_phs_min: MetricPattern1::new(
                client.clone(),
                "hash_value_phs_min".to_string(),
            ),
            hash_value_rebound: MetricPattern1::new(
                client.clone(),
                "hash_value_rebound".to_string(),
            ),
            hash_value_ths: MetricPattern1::new(client.clone(), "hash_value_ths".to_string()),
            hash_value_ths_min: MetricPattern1::new(
                client.clone(),
                "hash_value_ths_min".to_string(),
            ),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Blocks_Rewards {
    pub _24h_coinbase_sum: MetricsTree_Blocks_Rewards_24hCoinbaseSum,
    pub coinbase: CoinbasePattern,
    pub fee_dominance: MetricPattern6<StoredF32>,
    pub subsidy: CoinbasePattern,
    pub subsidy_dominance: MetricPattern6<StoredF32>,
    pub subsidy_usd_1y_sma: MetricPattern4<Dollars>,
    pub unclaimed_rewards: UnclaimedRewardsPattern,
}

impl MetricsTree_Blocks_Rewards {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _24h_coinbase_sum: MetricsTree_Blocks_Rewards_24hCoinbaseSum::new(
                client.clone(),
                format!("{base_path}_24h_coinbase_sum"),
            ),
            coinbase: CoinbasePattern::new(client.clone(), "coinbase".to_string()),
            fee_dominance: MetricPattern6::new(client.clone(), "fee_dominance".to_string()),
            subsidy: CoinbasePattern::new(client.clone(), "subsidy".to_string()),
            subsidy_dominance: MetricPattern6::new(client.clone(), "subsidy_dominance".to_string()),
            subsidy_usd_1y_sma: MetricPattern4::new(
                client.clone(),
                "subsidy_usd_1y_sma".to_string(),
            ),
            unclaimed_rewards: UnclaimedRewardsPattern::new(
                client.clone(),
                "unclaimed_rewards".to_string(),
            ),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Blocks_Rewards_24hCoinbaseSum {
    pub bitcoin: MetricPattern11<Bitcoin>,
    pub dollars: MetricPattern11<Dollars>,
    pub sats: MetricPattern11<Sats>,
}

impl MetricsTree_Blocks_Rewards_24hCoinbaseSum {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            bitcoin: MetricPattern11::new(client.clone(), "24h_coinbase_sum_btc".to_string()),
            dollars: MetricPattern11::new(client.clone(), "24h_coinbase_sum_usd".to_string()),
            sats: MetricPattern11::new(client.clone(), "24h_coinbase_sum".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Blocks_Size {
    pub average: MetricPattern2<StoredU64>,
    pub cumulative: MetricPattern1<StoredU64>,
    pub max: MetricPattern2<StoredU64>,
    pub median: MetricPattern6<StoredU64>,
    pub min: MetricPattern2<StoredU64>,
    pub pct10: MetricPattern6<StoredU64>,
    pub pct25: MetricPattern6<StoredU64>,
    pub pct75: MetricPattern6<StoredU64>,
    pub pct90: MetricPattern6<StoredU64>,
    pub sum: MetricPattern2<StoredU64>,
}

impl MetricsTree_Blocks_Size {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            average: MetricPattern2::new(client.clone(), "block_size_average".to_string()),
            cumulative: MetricPattern1::new(client.clone(), "block_size_cumulative".to_string()),
            max: MetricPattern2::new(client.clone(), "block_size_max".to_string()),
            median: MetricPattern6::new(client.clone(), "block_size_median".to_string()),
            min: MetricPattern2::new(client.clone(), "block_size_min".to_string()),
            pct10: MetricPattern6::new(client.clone(), "block_size_pct10".to_string()),
            pct25: MetricPattern6::new(client.clone(), "block_size_pct25".to_string()),
            pct75: MetricPattern6::new(client.clone(), "block_size_pct75".to_string()),
            pct90: MetricPattern6::new(client.clone(), "block_size_pct90".to_string()),
            sum: MetricPattern2::new(client.clone(), "block_size_sum".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Blocks_Time {
    pub date: MetricPattern11<Date>,
    pub timestamp: MetricPattern1<Timestamp>,
    pub timestamp_monotonic: MetricPattern11<Timestamp>,
}

impl MetricsTree_Blocks_Time {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            date: MetricPattern11::new(client.clone(), "date".to_string()),
            timestamp: MetricPattern1::new(client.clone(), "timestamp".to_string()),
            timestamp_monotonic: MetricPattern11::new(
                client.clone(),
                "timestamp_monotonic".to_string(),
            ),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Cointime {
    pub activity: MetricsTree_Cointime_Activity,
    pub adjusted: MetricsTree_Cointime_Adjusted,
    pub cap: MetricsTree_Cointime_Cap,
    pub pricing: MetricsTree_Cointime_Pricing,
    pub supply: MetricsTree_Cointime_Supply,
    pub value: MetricsTree_Cointime_Value,
}

impl MetricsTree_Cointime {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            activity: MetricsTree_Cointime_Activity::new(
                client.clone(),
                format!("{base_path}_activity"),
            ),
            adjusted: MetricsTree_Cointime_Adjusted::new(
                client.clone(),
                format!("{base_path}_adjusted"),
            ),
            cap: MetricsTree_Cointime_Cap::new(client.clone(), format!("{base_path}_cap")),
            pricing: MetricsTree_Cointime_Pricing::new(
                client.clone(),
                format!("{base_path}_pricing"),
            ),
            supply: MetricsTree_Cointime_Supply::new(client.clone(), format!("{base_path}_supply")),
            value: MetricsTree_Cointime_Value::new(client.clone(), format!("{base_path}_value")),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Cointime_Activity {
    pub activity_to_vaultedness_ratio: MetricPattern1<StoredF64>,
    pub coinblocks_created: BlockCountPattern<StoredF64>,
    pub coinblocks_stored: BlockCountPattern<StoredF64>,
    pub liveliness: MetricPattern1<StoredF64>,
    pub vaultedness: MetricPattern1<StoredF64>,
}

impl MetricsTree_Cointime_Activity {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            activity_to_vaultedness_ratio: MetricPattern1::new(
                client.clone(),
                "activity_to_vaultedness_ratio".to_string(),
            ),
            coinblocks_created: BlockCountPattern::new(
                client.clone(),
                "coinblocks_created".to_string(),
            ),
            coinblocks_stored: BlockCountPattern::new(
                client.clone(),
                "coinblocks_stored".to_string(),
            ),
            liveliness: MetricPattern1::new(client.clone(), "liveliness".to_string()),
            vaultedness: MetricPattern1::new(client.clone(), "vaultedness".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Cointime_Adjusted {
    pub cointime_adj_inflation_rate: MetricPattern4<StoredF32>,
    pub cointime_adj_tx_btc_velocity: MetricPattern4<StoredF64>,
    pub cointime_adj_tx_usd_velocity: MetricPattern4<StoredF64>,
}

impl MetricsTree_Cointime_Adjusted {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            cointime_adj_inflation_rate: MetricPattern4::new(
                client.clone(),
                "cointime_adj_inflation_rate".to_string(),
            ),
            cointime_adj_tx_btc_velocity: MetricPattern4::new(
                client.clone(),
                "cointime_adj_tx_btc_velocity".to_string(),
            ),
            cointime_adj_tx_usd_velocity: MetricPattern4::new(
                client.clone(),
                "cointime_adj_tx_usd_velocity".to_string(),
            ),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Cointime_Cap {
    pub active_cap: MetricPattern1<Dollars>,
    pub cointime_cap: MetricPattern1<Dollars>,
    pub investor_cap: MetricPattern1<Dollars>,
    pub thermo_cap: MetricPattern1<Dollars>,
    pub vaulted_cap: MetricPattern1<Dollars>,
}

impl MetricsTree_Cointime_Cap {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            active_cap: MetricPattern1::new(client.clone(), "active_cap".to_string()),
            cointime_cap: MetricPattern1::new(client.clone(), "cointime_cap".to_string()),
            investor_cap: MetricPattern1::new(client.clone(), "investor_cap".to_string()),
            thermo_cap: MetricPattern1::new(client.clone(), "thermo_cap".to_string()),
            vaulted_cap: MetricPattern1::new(client.clone(), "vaulted_cap".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Cointime_Pricing {
    pub active_price: MetricPattern1<Dollars>,
    pub active_price_ratio: ActivePriceRatioPattern,
    pub cointime_price: MetricPattern1<Dollars>,
    pub cointime_price_ratio: ActivePriceRatioPattern,
    pub true_market_mean: MetricPattern1<Dollars>,
    pub true_market_mean_ratio: ActivePriceRatioPattern,
    pub vaulted_price: MetricPattern1<Dollars>,
    pub vaulted_price_ratio: ActivePriceRatioPattern,
}

impl MetricsTree_Cointime_Pricing {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            active_price: MetricPattern1::new(client.clone(), "active_price".to_string()),
            active_price_ratio: ActivePriceRatioPattern::new(
                client.clone(),
                "active_price_ratio".to_string(),
            ),
            cointime_price: MetricPattern1::new(client.clone(), "cointime_price".to_string()),
            cointime_price_ratio: ActivePriceRatioPattern::new(
                client.clone(),
                "cointime_price_ratio".to_string(),
            ),
            true_market_mean: MetricPattern1::new(client.clone(), "true_market_mean".to_string()),
            true_market_mean_ratio: ActivePriceRatioPattern::new(
                client.clone(),
                "true_market_mean_ratio".to_string(),
            ),
            vaulted_price: MetricPattern1::new(client.clone(), "vaulted_price".to_string()),
            vaulted_price_ratio: ActivePriceRatioPattern::new(
                client.clone(),
                "vaulted_price_ratio".to_string(),
            ),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Cointime_Supply {
    pub active_supply: ActiveSupplyPattern,
    pub vaulted_supply: ActiveSupplyPattern,
}

impl MetricsTree_Cointime_Supply {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            active_supply: ActiveSupplyPattern::new(client.clone(), "active_supply".to_string()),
            vaulted_supply: ActiveSupplyPattern::new(client.clone(), "vaulted_supply".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Cointime_Value {
    pub cointime_value_created: BlockCountPattern<StoredF64>,
    pub cointime_value_destroyed: BlockCountPattern<StoredF64>,
    pub cointime_value_stored: BlockCountPattern<StoredF64>,
}

impl MetricsTree_Cointime_Value {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            cointime_value_created: BlockCountPattern::new(
                client.clone(),
                "cointime_value_created".to_string(),
            ),
            cointime_value_destroyed: BlockCountPattern::new(
                client.clone(),
                "cointime_value_destroyed".to_string(),
            ),
            cointime_value_stored: BlockCountPattern::new(
                client.clone(),
                "cointime_value_stored".to_string(),
            ),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Constants {
    pub constant_0: MetricPattern1<StoredU16>,
    pub constant_1: MetricPattern1<StoredU16>,
    pub constant_100: MetricPattern1<StoredU16>,
    pub constant_2: MetricPattern1<StoredU16>,
    pub constant_20: MetricPattern1<StoredU16>,
    pub constant_3: MetricPattern1<StoredU16>,
    pub constant_30: MetricPattern1<StoredU16>,
    pub constant_38_2: MetricPattern1<StoredF32>,
    pub constant_4: MetricPattern1<StoredU16>,
    pub constant_50: MetricPattern1<StoredU16>,
    pub constant_600: MetricPattern1<StoredU16>,
    pub constant_61_8: MetricPattern1<StoredF32>,
    pub constant_70: MetricPattern1<StoredU16>,
    pub constant_80: MetricPattern1<StoredU16>,
    pub constant_minus_1: MetricPattern1<StoredI16>,
    pub constant_minus_2: MetricPattern1<StoredI16>,
    pub constant_minus_3: MetricPattern1<StoredI16>,
    pub constant_minus_4: MetricPattern1<StoredI16>,
}

impl MetricsTree_Constants {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            constant_0: MetricPattern1::new(client.clone(), "constant_0".to_string()),
            constant_1: MetricPattern1::new(client.clone(), "constant_1".to_string()),
            constant_100: MetricPattern1::new(client.clone(), "constant_100".to_string()),
            constant_2: MetricPattern1::new(client.clone(), "constant_2".to_string()),
            constant_20: MetricPattern1::new(client.clone(), "constant_20".to_string()),
            constant_3: MetricPattern1::new(client.clone(), "constant_3".to_string()),
            constant_30: MetricPattern1::new(client.clone(), "constant_30".to_string()),
            constant_38_2: MetricPattern1::new(client.clone(), "constant_38_2".to_string()),
            constant_4: MetricPattern1::new(client.clone(), "constant_4".to_string()),
            constant_50: MetricPattern1::new(client.clone(), "constant_50".to_string()),
            constant_600: MetricPattern1::new(client.clone(), "constant_600".to_string()),
            constant_61_8: MetricPattern1::new(client.clone(), "constant_61_8".to_string()),
            constant_70: MetricPattern1::new(client.clone(), "constant_70".to_string()),
            constant_80: MetricPattern1::new(client.clone(), "constant_80".to_string()),
            constant_minus_1: MetricPattern1::new(client.clone(), "constant_minus_1".to_string()),
            constant_minus_2: MetricPattern1::new(client.clone(), "constant_minus_2".to_string()),
            constant_minus_3: MetricPattern1::new(client.clone(), "constant_minus_3".to_string()),
            constant_minus_4: MetricPattern1::new(client.clone(), "constant_minus_4".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Distribution {
    pub addr_count: AddrCountPattern,
    pub address_cohorts: MetricsTree_Distribution_AddressCohorts,
    pub addresses_data: MetricsTree_Distribution_AddressesData,
    pub any_address_indexes: MetricsTree_Distribution_AnyAddressIndexes,
    pub chain_state: MetricPattern11<SupplyState>,
    pub empty_addr_count: AddrCountPattern,
    pub emptyaddressindex: MetricPattern32<EmptyAddressIndex>,
    pub loadedaddressindex: MetricPattern31<LoadedAddressIndex>,
    pub utxo_cohorts: MetricsTree_Distribution_UtxoCohorts,
}

impl MetricsTree_Distribution {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            addr_count: AddrCountPattern::new(client.clone(), "addr_count".to_string()),
            address_cohorts: MetricsTree_Distribution_AddressCohorts::new(
                client.clone(),
                format!("{base_path}_address_cohorts"),
            ),
            addresses_data: MetricsTree_Distribution_AddressesData::new(
                client.clone(),
                format!("{base_path}_addresses_data"),
            ),
            any_address_indexes: MetricsTree_Distribution_AnyAddressIndexes::new(
                client.clone(),
                format!("{base_path}_any_address_indexes"),
            ),
            chain_state: MetricPattern11::new(client.clone(), "chain".to_string()),
            empty_addr_count: AddrCountPattern::new(client.clone(), "empty_addr_count".to_string()),
            emptyaddressindex: MetricPattern32::new(
                client.clone(),
                "emptyaddressindex".to_string(),
            ),
            loadedaddressindex: MetricPattern31::new(
                client.clone(),
                "loadedaddressindex".to_string(),
            ),
            utxo_cohorts: MetricsTree_Distribution_UtxoCohorts::new(
                client.clone(),
                format!("{base_path}_utxo_cohorts"),
            ),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Distribution_AddressCohorts {
    pub amount_range: MetricsTree_Distribution_AddressCohorts_AmountRange,
    pub ge_amount: MetricsTree_Distribution_AddressCohorts_GeAmount,
    pub lt_amount: MetricsTree_Distribution_AddressCohorts_LtAmount,
}

impl MetricsTree_Distribution_AddressCohorts {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            amount_range: MetricsTree_Distribution_AddressCohorts_AmountRange::new(
                client.clone(),
                format!("{base_path}_amount_range"),
            ),
            ge_amount: MetricsTree_Distribution_AddressCohorts_GeAmount::new(
                client.clone(),
                format!("{base_path}_ge_amount"),
            ),
            lt_amount: MetricsTree_Distribution_AddressCohorts_LtAmount::new(
                client.clone(),
                format!("{base_path}_lt_amount"),
            ),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Distribution_AddressCohorts_AmountRange {
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

impl MetricsTree_Distribution_AddressCohorts_AmountRange {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _0sats: _0satsPattern::new(client.clone(), "addrs_with_0sats".to_string()),
            _100btc_to_1k_btc: _0satsPattern::new(
                client.clone(),
                "addrs_above_100btc_under_1k_btc".to_string(),
            ),
            _100k_btc_or_more: _0satsPattern::new(
                client.clone(),
                "addrs_above_100k_btc".to_string(),
            ),
            _100k_sats_to_1m_sats: _0satsPattern::new(
                client.clone(),
                "addrs_above_100k_sats_under_1m_sats".to_string(),
            ),
            _100sats_to_1k_sats: _0satsPattern::new(
                client.clone(),
                "addrs_above_100sats_under_1k_sats".to_string(),
            ),
            _10btc_to_100btc: _0satsPattern::new(
                client.clone(),
                "addrs_above_10btc_under_100btc".to_string(),
            ),
            _10k_btc_to_100k_btc: _0satsPattern::new(
                client.clone(),
                "addrs_above_10k_btc_under_100k_btc".to_string(),
            ),
            _10k_sats_to_100k_sats: _0satsPattern::new(
                client.clone(),
                "addrs_above_10k_sats_under_100k_sats".to_string(),
            ),
            _10m_sats_to_1btc: _0satsPattern::new(
                client.clone(),
                "addrs_above_10m_sats_under_1btc".to_string(),
            ),
            _10sats_to_100sats: _0satsPattern::new(
                client.clone(),
                "addrs_above_10sats_under_100sats".to_string(),
            ),
            _1btc_to_10btc: _0satsPattern::new(
                client.clone(),
                "addrs_above_1btc_under_10btc".to_string(),
            ),
            _1k_btc_to_10k_btc: _0satsPattern::new(
                client.clone(),
                "addrs_above_1k_btc_under_10k_btc".to_string(),
            ),
            _1k_sats_to_10k_sats: _0satsPattern::new(
                client.clone(),
                "addrs_above_1k_sats_under_10k_sats".to_string(),
            ),
            _1m_sats_to_10m_sats: _0satsPattern::new(
                client.clone(),
                "addrs_above_1m_sats_under_10m_sats".to_string(),
            ),
            _1sat_to_10sats: _0satsPattern::new(
                client.clone(),
                "addrs_above_1sat_under_10sats".to_string(),
            ),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Distribution_AddressCohorts_GeAmount {
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

impl MetricsTree_Distribution_AddressCohorts_GeAmount {
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

/// Metrics tree node.
pub struct MetricsTree_Distribution_AddressCohorts_LtAmount {
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

impl MetricsTree_Distribution_AddressCohorts_LtAmount {
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

/// Metrics tree node.
pub struct MetricsTree_Distribution_AddressesData {
    pub empty: MetricPattern32<EmptyAddressData>,
    pub loaded: MetricPattern31<LoadedAddressData>,
}

impl MetricsTree_Distribution_AddressesData {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            empty: MetricPattern32::new(client.clone(), "emptyaddressdata".to_string()),
            loaded: MetricPattern31::new(client.clone(), "loadedaddressdata".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Distribution_AnyAddressIndexes {
    pub p2a: MetricPattern16<AnyAddressIndex>,
    pub p2pk33: MetricPattern18<AnyAddressIndex>,
    pub p2pk65: MetricPattern19<AnyAddressIndex>,
    pub p2pkh: MetricPattern20<AnyAddressIndex>,
    pub p2sh: MetricPattern21<AnyAddressIndex>,
    pub p2tr: MetricPattern22<AnyAddressIndex>,
    pub p2wpkh: MetricPattern23<AnyAddressIndex>,
    pub p2wsh: MetricPattern24<AnyAddressIndex>,
}

impl MetricsTree_Distribution_AnyAddressIndexes {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            p2a: MetricPattern16::new(client.clone(), "anyaddressindex".to_string()),
            p2pk33: MetricPattern18::new(client.clone(), "anyaddressindex".to_string()),
            p2pk65: MetricPattern19::new(client.clone(), "anyaddressindex".to_string()),
            p2pkh: MetricPattern20::new(client.clone(), "anyaddressindex".to_string()),
            p2sh: MetricPattern21::new(client.clone(), "anyaddressindex".to_string()),
            p2tr: MetricPattern22::new(client.clone(), "anyaddressindex".to_string()),
            p2wpkh: MetricPattern23::new(client.clone(), "anyaddressindex".to_string()),
            p2wsh: MetricPattern24::new(client.clone(), "anyaddressindex".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Distribution_UtxoCohorts {
    pub age_range: MetricsTree_Distribution_UtxoCohorts_AgeRange,
    pub all: MetricsTree_Distribution_UtxoCohorts_All,
    pub amount_range: MetricsTree_Distribution_UtxoCohorts_AmountRange,
    pub epoch: MetricsTree_Distribution_UtxoCohorts_Epoch,
    pub ge_amount: MetricsTree_Distribution_UtxoCohorts_GeAmount,
    pub lt_amount: MetricsTree_Distribution_UtxoCohorts_LtAmount,
    pub max_age: MetricsTree_Distribution_UtxoCohorts_MaxAge,
    pub min_age: MetricsTree_Distribution_UtxoCohorts_MinAge,
    pub term: MetricsTree_Distribution_UtxoCohorts_Term,
    pub type_: MetricsTree_Distribution_UtxoCohorts_Type,
    pub year: MetricsTree_Distribution_UtxoCohorts_Year,
}

impl MetricsTree_Distribution_UtxoCohorts {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            age_range: MetricsTree_Distribution_UtxoCohorts_AgeRange::new(
                client.clone(),
                format!("{base_path}_age_range"),
            ),
            all: MetricsTree_Distribution_UtxoCohorts_All::new(
                client.clone(),
                format!("{base_path}_all"),
            ),
            amount_range: MetricsTree_Distribution_UtxoCohorts_AmountRange::new(
                client.clone(),
                format!("{base_path}_amount_range"),
            ),
            epoch: MetricsTree_Distribution_UtxoCohorts_Epoch::new(
                client.clone(),
                format!("{base_path}_epoch"),
            ),
            ge_amount: MetricsTree_Distribution_UtxoCohorts_GeAmount::new(
                client.clone(),
                format!("{base_path}_ge_amount"),
            ),
            lt_amount: MetricsTree_Distribution_UtxoCohorts_LtAmount::new(
                client.clone(),
                format!("{base_path}_lt_amount"),
            ),
            max_age: MetricsTree_Distribution_UtxoCohorts_MaxAge::new(
                client.clone(),
                format!("{base_path}_max_age"),
            ),
            min_age: MetricsTree_Distribution_UtxoCohorts_MinAge::new(
                client.clone(),
                format!("{base_path}_min_age"),
            ),
            term: MetricsTree_Distribution_UtxoCohorts_Term::new(
                client.clone(),
                format!("{base_path}_term"),
            ),
            type_: MetricsTree_Distribution_UtxoCohorts_Type::new(
                client.clone(),
                format!("{base_path}_type_"),
            ),
            year: MetricsTree_Distribution_UtxoCohorts_Year::new(
                client.clone(),
                format!("{base_path}_year"),
            ),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Distribution_UtxoCohorts_AgeRange {
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

impl MetricsTree_Distribution_UtxoCohorts_AgeRange {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _10y_to_12y: _10yTo12yPattern::new(
                client.clone(),
                "utxos_at_least_10y_up_to_12y_old".to_string(),
            ),
            _12y_to_15y: _10yTo12yPattern::new(
                client.clone(),
                "utxos_at_least_12y_up_to_15y_old".to_string(),
            ),
            _1d_to_1w: _10yTo12yPattern::new(
                client.clone(),
                "utxos_at_least_1d_up_to_1w_old".to_string(),
            ),
            _1h_to_1d: _10yTo12yPattern::new(
                client.clone(),
                "utxos_at_least_1h_up_to_1d_old".to_string(),
            ),
            _1m_to_2m: _10yTo12yPattern::new(
                client.clone(),
                "utxos_at_least_1m_up_to_2m_old".to_string(),
            ),
            _1w_to_1m: _10yTo12yPattern::new(
                client.clone(),
                "utxos_at_least_1w_up_to_1m_old".to_string(),
            ),
            _1y_to_2y: _10yTo12yPattern::new(
                client.clone(),
                "utxos_at_least_1y_up_to_2y_old".to_string(),
            ),
            _2m_to_3m: _10yTo12yPattern::new(
                client.clone(),
                "utxos_at_least_2m_up_to_3m_old".to_string(),
            ),
            _2y_to_3y: _10yTo12yPattern::new(
                client.clone(),
                "utxos_at_least_2y_up_to_3y_old".to_string(),
            ),
            _3m_to_4m: _10yTo12yPattern::new(
                client.clone(),
                "utxos_at_least_3m_up_to_4m_old".to_string(),
            ),
            _3y_to_4y: _10yTo12yPattern::new(
                client.clone(),
                "utxos_at_least_3y_up_to_4y_old".to_string(),
            ),
            _4m_to_5m: _10yTo12yPattern::new(
                client.clone(),
                "utxos_at_least_4m_up_to_5m_old".to_string(),
            ),
            _4y_to_5y: _10yTo12yPattern::new(
                client.clone(),
                "utxos_at_least_4y_up_to_5y_old".to_string(),
            ),
            _5m_to_6m: _10yTo12yPattern::new(
                client.clone(),
                "utxos_at_least_5m_up_to_6m_old".to_string(),
            ),
            _5y_to_6y: _10yTo12yPattern::new(
                client.clone(),
                "utxos_at_least_5y_up_to_6y_old".to_string(),
            ),
            _6m_to_1y: _10yTo12yPattern::new(
                client.clone(),
                "utxos_at_least_6m_up_to_1y_old".to_string(),
            ),
            _6y_to_7y: _10yTo12yPattern::new(
                client.clone(),
                "utxos_at_least_6y_up_to_7y_old".to_string(),
            ),
            _7y_to_8y: _10yTo12yPattern::new(
                client.clone(),
                "utxos_at_least_7y_up_to_8y_old".to_string(),
            ),
            _8y_to_10y: _10yTo12yPattern::new(
                client.clone(),
                "utxos_at_least_8y_up_to_10y_old".to_string(),
            ),
            from_15y: _10yTo12yPattern::new(client.clone(), "utxos_at_least_15y_old".to_string()),
            up_to_1h: _10yTo12yPattern::new(client.clone(), "utxos_up_to_1h_old".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Distribution_UtxoCohorts_All {
    pub activity: ActivityPattern2,
    pub cost_basis: MetricsTree_Distribution_UtxoCohorts_All_CostBasis,
    pub outputs: OutputsPattern,
    pub realized: RealizedPattern3,
    pub relative: MetricsTree_Distribution_UtxoCohorts_All_Relative,
    pub supply: SupplyPattern2,
    pub unrealized: UnrealizedPattern,
}

impl MetricsTree_Distribution_UtxoCohorts_All {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            activity: ActivityPattern2::new(client.clone(), "".to_string()),
            cost_basis: MetricsTree_Distribution_UtxoCohorts_All_CostBasis::new(
                client.clone(),
                format!("{base_path}_cost_basis"),
            ),
            outputs: OutputsPattern::new(client.clone(), "utxo_count".to_string()),
            realized: RealizedPattern3::new(client.clone(), "".to_string()),
            relative: MetricsTree_Distribution_UtxoCohorts_All_Relative::new(
                client.clone(),
                format!("{base_path}_relative"),
            ),
            supply: SupplyPattern2::new(client.clone(), "supply".to_string()),
            unrealized: UnrealizedPattern::new(client.clone(), "".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Distribution_UtxoCohorts_All_CostBasis {
    pub max: MetricPattern1<Dollars>,
    pub min: MetricPattern1<Dollars>,
    pub percentiles: PercentilesPattern,
}

impl MetricsTree_Distribution_UtxoCohorts_All_CostBasis {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            max: MetricPattern1::new(client.clone(), "max_cost_basis".to_string()),
            min: MetricPattern1::new(client.clone(), "min_cost_basis".to_string()),
            percentiles: PercentilesPattern::new(client.clone(), "cost_basis".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Distribution_UtxoCohorts_All_Relative {
    pub neg_unrealized_loss_rel_to_own_total_unrealized_pnl: MetricPattern1<StoredF32>,
    pub net_unrealized_pnl_rel_to_own_total_unrealized_pnl: MetricPattern1<StoredF32>,
    pub supply_in_loss_rel_to_own_supply: MetricPattern1<StoredF64>,
    pub supply_in_profit_rel_to_own_supply: MetricPattern1<StoredF64>,
    pub unrealized_loss_rel_to_own_total_unrealized_pnl: MetricPattern1<StoredF32>,
    pub unrealized_profit_rel_to_own_total_unrealized_pnl: MetricPattern1<StoredF32>,
}

impl MetricsTree_Distribution_UtxoCohorts_All_Relative {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            neg_unrealized_loss_rel_to_own_total_unrealized_pnl: MetricPattern1::new(
                client.clone(),
                "neg_unrealized_loss_rel_to_own_total_unrealized_pnl".to_string(),
            ),
            net_unrealized_pnl_rel_to_own_total_unrealized_pnl: MetricPattern1::new(
                client.clone(),
                "net_unrealized_pnl_rel_to_own_total_unrealized_pnl".to_string(),
            ),
            supply_in_loss_rel_to_own_supply: MetricPattern1::new(
                client.clone(),
                "supply_in_loss_rel_to_own_supply".to_string(),
            ),
            supply_in_profit_rel_to_own_supply: MetricPattern1::new(
                client.clone(),
                "supply_in_profit_rel_to_own_supply".to_string(),
            ),
            unrealized_loss_rel_to_own_total_unrealized_pnl: MetricPattern1::new(
                client.clone(),
                "unrealized_loss_rel_to_own_total_unrealized_pnl".to_string(),
            ),
            unrealized_profit_rel_to_own_total_unrealized_pnl: MetricPattern1::new(
                client.clone(),
                "unrealized_profit_rel_to_own_total_unrealized_pnl".to_string(),
            ),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Distribution_UtxoCohorts_AmountRange {
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

impl MetricsTree_Distribution_UtxoCohorts_AmountRange {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _0sats: _0satsPattern2::new(client.clone(), "utxos_with_0sats".to_string()),
            _100btc_to_1k_btc: _0satsPattern2::new(
                client.clone(),
                "utxos_above_100btc_under_1k_btc".to_string(),
            ),
            _100k_btc_or_more: _0satsPattern2::new(
                client.clone(),
                "utxos_above_100k_btc".to_string(),
            ),
            _100k_sats_to_1m_sats: _0satsPattern2::new(
                client.clone(),
                "utxos_above_100k_sats_under_1m_sats".to_string(),
            ),
            _100sats_to_1k_sats: _0satsPattern2::new(
                client.clone(),
                "utxos_above_100sats_under_1k_sats".to_string(),
            ),
            _10btc_to_100btc: _0satsPattern2::new(
                client.clone(),
                "utxos_above_10btc_under_100btc".to_string(),
            ),
            _10k_btc_to_100k_btc: _0satsPattern2::new(
                client.clone(),
                "utxos_above_10k_btc_under_100k_btc".to_string(),
            ),
            _10k_sats_to_100k_sats: _0satsPattern2::new(
                client.clone(),
                "utxos_above_10k_sats_under_100k_sats".to_string(),
            ),
            _10m_sats_to_1btc: _0satsPattern2::new(
                client.clone(),
                "utxos_above_10m_sats_under_1btc".to_string(),
            ),
            _10sats_to_100sats: _0satsPattern2::new(
                client.clone(),
                "utxos_above_10sats_under_100sats".to_string(),
            ),
            _1btc_to_10btc: _0satsPattern2::new(
                client.clone(),
                "utxos_above_1btc_under_10btc".to_string(),
            ),
            _1k_btc_to_10k_btc: _0satsPattern2::new(
                client.clone(),
                "utxos_above_1k_btc_under_10k_btc".to_string(),
            ),
            _1k_sats_to_10k_sats: _0satsPattern2::new(
                client.clone(),
                "utxos_above_1k_sats_under_10k_sats".to_string(),
            ),
            _1m_sats_to_10m_sats: _0satsPattern2::new(
                client.clone(),
                "utxos_above_1m_sats_under_10m_sats".to_string(),
            ),
            _1sat_to_10sats: _0satsPattern2::new(
                client.clone(),
                "utxos_above_1sat_under_10sats".to_string(),
            ),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Distribution_UtxoCohorts_Epoch {
    pub _0: _0satsPattern2,
    pub _1: _0satsPattern2,
    pub _2: _0satsPattern2,
    pub _3: _0satsPattern2,
    pub _4: _0satsPattern2,
}

impl MetricsTree_Distribution_UtxoCohorts_Epoch {
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

/// Metrics tree node.
pub struct MetricsTree_Distribution_UtxoCohorts_GeAmount {
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

impl MetricsTree_Distribution_UtxoCohorts_GeAmount {
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

/// Metrics tree node.
pub struct MetricsTree_Distribution_UtxoCohorts_LtAmount {
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

impl MetricsTree_Distribution_UtxoCohorts_LtAmount {
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

/// Metrics tree node.
pub struct MetricsTree_Distribution_UtxoCohorts_MaxAge {
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

impl MetricsTree_Distribution_UtxoCohorts_MaxAge {
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

/// Metrics tree node.
pub struct MetricsTree_Distribution_UtxoCohorts_MinAge {
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

impl MetricsTree_Distribution_UtxoCohorts_MinAge {
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

/// Metrics tree node.
pub struct MetricsTree_Distribution_UtxoCohorts_Term {
    pub long: MetricsTree_Distribution_UtxoCohorts_Term_Long,
    pub short: MetricsTree_Distribution_UtxoCohorts_Term_Short,
}

impl MetricsTree_Distribution_UtxoCohorts_Term {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            long: MetricsTree_Distribution_UtxoCohorts_Term_Long::new(
                client.clone(),
                format!("{base_path}_long"),
            ),
            short: MetricsTree_Distribution_UtxoCohorts_Term_Short::new(
                client.clone(),
                format!("{base_path}_short"),
            ),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Distribution_UtxoCohorts_Term_Long {
    pub activity: ActivityPattern2,
    pub cost_basis: CostBasisPattern2,
    pub outputs: OutputsPattern,
    pub realized: RealizedPattern2,
    pub relative: RelativePattern5,
    pub supply: SupplyPattern2,
    pub unrealized: UnrealizedPattern,
}

impl MetricsTree_Distribution_UtxoCohorts_Term_Long {
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

/// Metrics tree node.
pub struct MetricsTree_Distribution_UtxoCohorts_Term_Short {
    pub activity: ActivityPattern2,
    pub cost_basis: CostBasisPattern2,
    pub outputs: OutputsPattern,
    pub realized: RealizedPattern3,
    pub relative: RelativePattern5,
    pub supply: SupplyPattern2,
    pub unrealized: UnrealizedPattern,
}

impl MetricsTree_Distribution_UtxoCohorts_Term_Short {
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

/// Metrics tree node.
pub struct MetricsTree_Distribution_UtxoCohorts_Type {
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

impl MetricsTree_Distribution_UtxoCohorts_Type {
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

/// Metrics tree node.
pub struct MetricsTree_Distribution_UtxoCohorts_Year {
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

impl MetricsTree_Distribution_UtxoCohorts_Year {
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

/// Metrics tree node.
pub struct MetricsTree_Indexes {
    pub address: MetricsTree_Indexes_Address,
    pub dateindex: MetricsTree_Indexes_Dateindex,
    pub decadeindex: MetricsTree_Indexes_Decadeindex,
    pub difficultyepoch: MetricsTree_Indexes_Difficultyepoch,
    pub halvingepoch: MetricsTree_Indexes_Halvingepoch,
    pub height: MetricsTree_Indexes_Height,
    pub monthindex: MetricsTree_Indexes_Monthindex,
    pub quarterindex: MetricsTree_Indexes_Quarterindex,
    pub semesterindex: MetricsTree_Indexes_Semesterindex,
    pub txindex: MetricsTree_Indexes_Txindex,
    pub txinindex: MetricsTree_Indexes_Txinindex,
    pub txoutindex: MetricsTree_Indexes_Txoutindex,
    pub weekindex: MetricsTree_Indexes_Weekindex,
    pub yearindex: MetricsTree_Indexes_Yearindex,
}

impl MetricsTree_Indexes {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            address: MetricsTree_Indexes_Address::new(
                client.clone(),
                format!("{base_path}_address"),
            ),
            dateindex: MetricsTree_Indexes_Dateindex::new(
                client.clone(),
                format!("{base_path}_dateindex"),
            ),
            decadeindex: MetricsTree_Indexes_Decadeindex::new(
                client.clone(),
                format!("{base_path}_decadeindex"),
            ),
            difficultyepoch: MetricsTree_Indexes_Difficultyepoch::new(
                client.clone(),
                format!("{base_path}_difficultyepoch"),
            ),
            halvingepoch: MetricsTree_Indexes_Halvingepoch::new(
                client.clone(),
                format!("{base_path}_halvingepoch"),
            ),
            height: MetricsTree_Indexes_Height::new(client.clone(), format!("{base_path}_height")),
            monthindex: MetricsTree_Indexes_Monthindex::new(
                client.clone(),
                format!("{base_path}_monthindex"),
            ),
            quarterindex: MetricsTree_Indexes_Quarterindex::new(
                client.clone(),
                format!("{base_path}_quarterindex"),
            ),
            semesterindex: MetricsTree_Indexes_Semesterindex::new(
                client.clone(),
                format!("{base_path}_semesterindex"),
            ),
            txindex: MetricsTree_Indexes_Txindex::new(
                client.clone(),
                format!("{base_path}_txindex"),
            ),
            txinindex: MetricsTree_Indexes_Txinindex::new(
                client.clone(),
                format!("{base_path}_txinindex"),
            ),
            txoutindex: MetricsTree_Indexes_Txoutindex::new(
                client.clone(),
                format!("{base_path}_txoutindex"),
            ),
            weekindex: MetricsTree_Indexes_Weekindex::new(
                client.clone(),
                format!("{base_path}_weekindex"),
            ),
            yearindex: MetricsTree_Indexes_Yearindex::new(
                client.clone(),
                format!("{base_path}_yearindex"),
            ),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Address {
    pub empty: MetricsTree_Indexes_Address_Empty,
    pub opreturn: MetricsTree_Indexes_Address_Opreturn,
    pub p2a: MetricsTree_Indexes_Address_P2a,
    pub p2ms: MetricsTree_Indexes_Address_P2ms,
    pub p2pk33: MetricsTree_Indexes_Address_P2pk33,
    pub p2pk65: MetricsTree_Indexes_Address_P2pk65,
    pub p2pkh: MetricsTree_Indexes_Address_P2pkh,
    pub p2sh: MetricsTree_Indexes_Address_P2sh,
    pub p2tr: MetricsTree_Indexes_Address_P2tr,
    pub p2wpkh: MetricsTree_Indexes_Address_P2wpkh,
    pub p2wsh: MetricsTree_Indexes_Address_P2wsh,
    pub unknown: MetricsTree_Indexes_Address_Unknown,
}

impl MetricsTree_Indexes_Address {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            empty: MetricsTree_Indexes_Address_Empty::new(
                client.clone(),
                format!("{base_path}_empty"),
            ),
            opreturn: MetricsTree_Indexes_Address_Opreturn::new(
                client.clone(),
                format!("{base_path}_opreturn"),
            ),
            p2a: MetricsTree_Indexes_Address_P2a::new(client.clone(), format!("{base_path}_p2a")),
            p2ms: MetricsTree_Indexes_Address_P2ms::new(
                client.clone(),
                format!("{base_path}_p2ms"),
            ),
            p2pk33: MetricsTree_Indexes_Address_P2pk33::new(
                client.clone(),
                format!("{base_path}_p2pk33"),
            ),
            p2pk65: MetricsTree_Indexes_Address_P2pk65::new(
                client.clone(),
                format!("{base_path}_p2pk65"),
            ),
            p2pkh: MetricsTree_Indexes_Address_P2pkh::new(
                client.clone(),
                format!("{base_path}_p2pkh"),
            ),
            p2sh: MetricsTree_Indexes_Address_P2sh::new(
                client.clone(),
                format!("{base_path}_p2sh"),
            ),
            p2tr: MetricsTree_Indexes_Address_P2tr::new(
                client.clone(),
                format!("{base_path}_p2tr"),
            ),
            p2wpkh: MetricsTree_Indexes_Address_P2wpkh::new(
                client.clone(),
                format!("{base_path}_p2wpkh"),
            ),
            p2wsh: MetricsTree_Indexes_Address_P2wsh::new(
                client.clone(),
                format!("{base_path}_p2wsh"),
            ),
            unknown: MetricsTree_Indexes_Address_Unknown::new(
                client.clone(),
                format!("{base_path}_unknown"),
            ),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Address_Empty {
    pub identity: MetricPattern9<EmptyOutputIndex>,
}

impl MetricsTree_Indexes_Address_Empty {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern9::new(client.clone(), "emptyoutputindex".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Address_Opreturn {
    pub identity: MetricPattern14<OpReturnIndex>,
}

impl MetricsTree_Indexes_Address_Opreturn {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern14::new(client.clone(), "opreturnindex".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Address_P2a {
    pub identity: MetricPattern16<P2AAddressIndex>,
}

impl MetricsTree_Indexes_Address_P2a {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern16::new(client.clone(), "p2aaddressindex".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Address_P2ms {
    pub identity: MetricPattern17<P2MSOutputIndex>,
}

impl MetricsTree_Indexes_Address_P2ms {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern17::new(client.clone(), "p2msoutputindex".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Address_P2pk33 {
    pub identity: MetricPattern18<P2PK33AddressIndex>,
}

impl MetricsTree_Indexes_Address_P2pk33 {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern18::new(client.clone(), "p2pk33addressindex".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Address_P2pk65 {
    pub identity: MetricPattern19<P2PK65AddressIndex>,
}

impl MetricsTree_Indexes_Address_P2pk65 {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern19::new(client.clone(), "p2pk65addressindex".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Address_P2pkh {
    pub identity: MetricPattern20<P2PKHAddressIndex>,
}

impl MetricsTree_Indexes_Address_P2pkh {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern20::new(client.clone(), "p2pkhaddressindex".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Address_P2sh {
    pub identity: MetricPattern21<P2SHAddressIndex>,
}

impl MetricsTree_Indexes_Address_P2sh {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern21::new(client.clone(), "p2shaddressindex".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Address_P2tr {
    pub identity: MetricPattern22<P2TRAddressIndex>,
}

impl MetricsTree_Indexes_Address_P2tr {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern22::new(client.clone(), "p2traddressindex".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Address_P2wpkh {
    pub identity: MetricPattern23<P2WPKHAddressIndex>,
}

impl MetricsTree_Indexes_Address_P2wpkh {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern23::new(client.clone(), "p2wpkhaddressindex".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Address_P2wsh {
    pub identity: MetricPattern24<P2WSHAddressIndex>,
}

impl MetricsTree_Indexes_Address_P2wsh {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern24::new(client.clone(), "p2wshaddressindex".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Address_Unknown {
    pub identity: MetricPattern28<UnknownOutputIndex>,
}

impl MetricsTree_Indexes_Address_Unknown {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern28::new(client.clone(), "unknownoutputindex".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Dateindex {
    pub date: MetricPattern6<Date>,
    pub first_height: MetricPattern6<Height>,
    pub height_count: MetricPattern6<StoredU64>,
    pub identity: MetricPattern6<DateIndex>,
    pub monthindex: MetricPattern6<MonthIndex>,
    pub weekindex: MetricPattern6<WeekIndex>,
}

impl MetricsTree_Indexes_Dateindex {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            date: MetricPattern6::new(client.clone(), "date".to_string()),
            first_height: MetricPattern6::new(client.clone(), "first_height".to_string()),
            height_count: MetricPattern6::new(client.clone(), "height_count".to_string()),
            identity: MetricPattern6::new(client.clone(), "dateindex".to_string()),
            monthindex: MetricPattern6::new(client.clone(), "monthindex".to_string()),
            weekindex: MetricPattern6::new(client.clone(), "weekindex".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Decadeindex {
    pub date: MetricPattern7<Date>,
    pub first_yearindex: MetricPattern7<YearIndex>,
    pub identity: MetricPattern7<DecadeIndex>,
    pub yearindex_count: MetricPattern7<StoredU64>,
}

impl MetricsTree_Indexes_Decadeindex {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            date: MetricPattern7::new(client.clone(), "date".to_string()),
            first_yearindex: MetricPattern7::new(client.clone(), "first_yearindex".to_string()),
            identity: MetricPattern7::new(client.clone(), "decadeindex".to_string()),
            yearindex_count: MetricPattern7::new(client.clone(), "yearindex_count".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Difficultyepoch {
    pub first_height: MetricPattern8<Height>,
    pub height_count: MetricPattern8<StoredU64>,
    pub identity: MetricPattern8<DifficultyEpoch>,
}

impl MetricsTree_Indexes_Difficultyepoch {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            first_height: MetricPattern8::new(client.clone(), "first_height".to_string()),
            height_count: MetricPattern8::new(client.clone(), "height_count".to_string()),
            identity: MetricPattern8::new(client.clone(), "difficultyepoch".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Halvingepoch {
    pub first_height: MetricPattern10<Height>,
    pub identity: MetricPattern10<HalvingEpoch>,
}

impl MetricsTree_Indexes_Halvingepoch {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            first_height: MetricPattern10::new(client.clone(), "first_height".to_string()),
            identity: MetricPattern10::new(client.clone(), "halvingepoch".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Height {
    pub dateindex: MetricPattern11<DateIndex>,
    pub difficultyepoch: MetricPattern11<DifficultyEpoch>,
    pub halvingepoch: MetricPattern11<HalvingEpoch>,
    pub identity: MetricPattern11<Height>,
    pub txindex_count: MetricPattern11<StoredU64>,
}

impl MetricsTree_Indexes_Height {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            dateindex: MetricPattern11::new(client.clone(), "dateindex".to_string()),
            difficultyepoch: MetricPattern11::new(client.clone(), "difficultyepoch".to_string()),
            halvingepoch: MetricPattern11::new(client.clone(), "halvingepoch".to_string()),
            identity: MetricPattern11::new(client.clone(), "height".to_string()),
            txindex_count: MetricPattern11::new(client.clone(), "txindex_count".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Monthindex {
    pub date: MetricPattern13<Date>,
    pub dateindex_count: MetricPattern13<StoredU64>,
    pub first_dateindex: MetricPattern13<DateIndex>,
    pub identity: MetricPattern13<MonthIndex>,
    pub quarterindex: MetricPattern13<QuarterIndex>,
    pub semesterindex: MetricPattern13<SemesterIndex>,
    pub yearindex: MetricPattern13<YearIndex>,
}

impl MetricsTree_Indexes_Monthindex {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            date: MetricPattern13::new(client.clone(), "date".to_string()),
            dateindex_count: MetricPattern13::new(client.clone(), "dateindex_count".to_string()),
            first_dateindex: MetricPattern13::new(client.clone(), "first_dateindex".to_string()),
            identity: MetricPattern13::new(client.clone(), "monthindex".to_string()),
            quarterindex: MetricPattern13::new(client.clone(), "quarterindex".to_string()),
            semesterindex: MetricPattern13::new(client.clone(), "semesterindex".to_string()),
            yearindex: MetricPattern13::new(client.clone(), "yearindex".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Quarterindex {
    pub date: MetricPattern25<Date>,
    pub first_monthindex: MetricPattern25<MonthIndex>,
    pub identity: MetricPattern25<QuarterIndex>,
    pub monthindex_count: MetricPattern25<StoredU64>,
}

impl MetricsTree_Indexes_Quarterindex {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            date: MetricPattern25::new(client.clone(), "date".to_string()),
            first_monthindex: MetricPattern25::new(client.clone(), "first_monthindex".to_string()),
            identity: MetricPattern25::new(client.clone(), "quarterindex".to_string()),
            monthindex_count: MetricPattern25::new(client.clone(), "monthindex_count".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Semesterindex {
    pub date: MetricPattern26<Date>,
    pub first_monthindex: MetricPattern26<MonthIndex>,
    pub identity: MetricPattern26<SemesterIndex>,
    pub monthindex_count: MetricPattern26<StoredU64>,
}

impl MetricsTree_Indexes_Semesterindex {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            date: MetricPattern26::new(client.clone(), "date".to_string()),
            first_monthindex: MetricPattern26::new(client.clone(), "first_monthindex".to_string()),
            identity: MetricPattern26::new(client.clone(), "semesterindex".to_string()),
            monthindex_count: MetricPattern26::new(client.clone(), "monthindex_count".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Txindex {
    pub identity: MetricPattern27<TxIndex>,
    pub input_count: MetricPattern27<StoredU64>,
    pub output_count: MetricPattern27<StoredU64>,
}

impl MetricsTree_Indexes_Txindex {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern27::new(client.clone(), "txindex".to_string()),
            input_count: MetricPattern27::new(client.clone(), "input_count".to_string()),
            output_count: MetricPattern27::new(client.clone(), "output_count".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Txinindex {
    pub identity: MetricPattern12<TxInIndex>,
}

impl MetricsTree_Indexes_Txinindex {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern12::new(client.clone(), "txinindex".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Txoutindex {
    pub identity: MetricPattern15<TxOutIndex>,
}

impl MetricsTree_Indexes_Txoutindex {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern15::new(client.clone(), "txoutindex".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Weekindex {
    pub date: MetricPattern29<Date>,
    pub dateindex_count: MetricPattern29<StoredU64>,
    pub first_dateindex: MetricPattern29<DateIndex>,
    pub identity: MetricPattern29<WeekIndex>,
}

impl MetricsTree_Indexes_Weekindex {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            date: MetricPattern29::new(client.clone(), "date".to_string()),
            dateindex_count: MetricPattern29::new(client.clone(), "dateindex_count".to_string()),
            first_dateindex: MetricPattern29::new(client.clone(), "first_dateindex".to_string()),
            identity: MetricPattern29::new(client.clone(), "weekindex".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Yearindex {
    pub date: MetricPattern30<Date>,
    pub decadeindex: MetricPattern30<DecadeIndex>,
    pub first_monthindex: MetricPattern30<MonthIndex>,
    pub identity: MetricPattern30<YearIndex>,
    pub monthindex_count: MetricPattern30<StoredU64>,
}

impl MetricsTree_Indexes_Yearindex {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            date: MetricPattern30::new(client.clone(), "date".to_string()),
            decadeindex: MetricPattern30::new(client.clone(), "decadeindex".to_string()),
            first_monthindex: MetricPattern30::new(client.clone(), "first_monthindex".to_string()),
            identity: MetricPattern30::new(client.clone(), "yearindex".to_string()),
            monthindex_count: MetricPattern30::new(client.clone(), "monthindex_count".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Inputs {
    pub count: CountPattern2<StoredU64>,
    pub first_txinindex: MetricPattern11<TxInIndex>,
    pub outpoint: MetricPattern12<OutPoint>,
    pub outputtype: MetricPattern12<OutputType>,
    pub spent: MetricsTree_Inputs_Spent,
    pub txindex: MetricPattern12<TxIndex>,
    pub typeindex: MetricPattern12<TypeIndex>,
}

impl MetricsTree_Inputs {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            count: CountPattern2::new(client.clone(), "input_count".to_string()),
            first_txinindex: MetricPattern11::new(client.clone(), "first_txinindex".to_string()),
            outpoint: MetricPattern12::new(client.clone(), "outpoint".to_string()),
            outputtype: MetricPattern12::new(client.clone(), "outputtype".to_string()),
            spent: MetricsTree_Inputs_Spent::new(client.clone(), format!("{base_path}_spent")),
            txindex: MetricPattern12::new(client.clone(), "txindex".to_string()),
            typeindex: MetricPattern12::new(client.clone(), "typeindex".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Inputs_Spent {
    pub txoutindex: MetricPattern12<TxOutIndex>,
    pub value: MetricPattern12<Sats>,
}

impl MetricsTree_Inputs_Spent {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            txoutindex: MetricPattern12::new(client.clone(), "txoutindex".to_string()),
            value: MetricPattern12::new(client.clone(), "value".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Market {
    pub ath: MetricsTree_Market_Ath,
    pub dca: MetricsTree_Market_Dca,
    pub indicators: MetricsTree_Market_Indicators,
    pub lookback: LookbackPattern<Dollars>,
    pub moving_average: MetricsTree_Market_MovingAverage,
    pub range: MetricsTree_Market_Range,
    pub returns: MetricsTree_Market_Returns,
    pub volatility: MetricsTree_Market_Volatility,
}

impl MetricsTree_Market {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            ath: MetricsTree_Market_Ath::new(client.clone(), format!("{base_path}_ath")),
            dca: MetricsTree_Market_Dca::new(client.clone(), format!("{base_path}_dca")),
            indicators: MetricsTree_Market_Indicators::new(
                client.clone(),
                format!("{base_path}_indicators"),
            ),
            lookback: LookbackPattern::new(client.clone(), "price".to_string()),
            moving_average: MetricsTree_Market_MovingAverage::new(
                client.clone(),
                format!("{base_path}_moving_average"),
            ),
            range: MetricsTree_Market_Range::new(client.clone(), format!("{base_path}_range")),
            returns: MetricsTree_Market_Returns::new(
                client.clone(),
                format!("{base_path}_returns"),
            ),
            volatility: MetricsTree_Market_Volatility::new(
                client.clone(),
                format!("{base_path}_volatility"),
            ),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Market_Ath {
    pub days_since_price_ath: MetricPattern4<StoredU16>,
    pub max_days_between_price_aths: MetricPattern4<StoredU16>,
    pub max_years_between_price_aths: MetricPattern4<StoredF32>,
    pub price_ath: MetricPattern1<Dollars>,
    pub price_drawdown: MetricPattern3<StoredF32>,
    pub years_since_price_ath: MetricPattern4<StoredF32>,
}

impl MetricsTree_Market_Ath {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            days_since_price_ath: MetricPattern4::new(
                client.clone(),
                "days_since_price_ath".to_string(),
            ),
            max_days_between_price_aths: MetricPattern4::new(
                client.clone(),
                "max_days_between_price_aths".to_string(),
            ),
            max_years_between_price_aths: MetricPattern4::new(
                client.clone(),
                "max_years_between_price_aths".to_string(),
            ),
            price_ath: MetricPattern1::new(client.clone(), "price_ath".to_string()),
            price_drawdown: MetricPattern3::new(client.clone(), "price_drawdown".to_string()),
            years_since_price_ath: MetricPattern4::new(
                client.clone(),
                "years_since_price_ath".to_string(),
            ),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Market_Dca {
    pub class_average_price: MetricsTree_Market_Dca_ClassAveragePrice,
    pub class_returns: ClassAveragePricePattern<StoredF32>,
    pub class_stack: MetricsTree_Market_Dca_ClassStack,
    pub period_average_price: PeriodAveragePricePattern<Dollars>,
    pub period_cagr: PeriodCagrPattern,
    pub period_lump_sum_stack: PeriodLumpSumStackPattern,
    pub period_returns: PeriodAveragePricePattern<StoredF32>,
    pub period_stack: PeriodLumpSumStackPattern,
}

impl MetricsTree_Market_Dca {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            class_average_price: MetricsTree_Market_Dca_ClassAveragePrice::new(
                client.clone(),
                format!("{base_path}_class_average_price"),
            ),
            class_returns: ClassAveragePricePattern::new(client.clone(), "dca_class".to_string()),
            class_stack: MetricsTree_Market_Dca_ClassStack::new(
                client.clone(),
                format!("{base_path}_class_stack"),
            ),
            period_average_price: PeriodAveragePricePattern::new(
                client.clone(),
                "dca_average_price".to_string(),
            ),
            period_cagr: PeriodCagrPattern::new(client.clone(), "dca_cagr".to_string()),
            period_lump_sum_stack: PeriodLumpSumStackPattern::new(
                client.clone(),
                "lump_sum_stack".to_string(),
            ),
            period_returns: PeriodAveragePricePattern::new(
                client.clone(),
                "dca_returns".to_string(),
            ),
            period_stack: PeriodLumpSumStackPattern::new(client.clone(), "dca_stack".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Market_Dca_ClassAveragePrice {
    pub _2015: MetricPattern4<Dollars>,
    pub _2016: MetricPattern4<Dollars>,
    pub _2017: MetricPattern4<Dollars>,
    pub _2018: MetricPattern4<Dollars>,
    pub _2019: MetricPattern4<Dollars>,
    pub _2020: MetricPattern4<Dollars>,
    pub _2021: MetricPattern4<Dollars>,
    pub _2022: MetricPattern4<Dollars>,
    pub _2023: MetricPattern4<Dollars>,
    pub _2024: MetricPattern4<Dollars>,
    pub _2025: MetricPattern4<Dollars>,
}

impl MetricsTree_Market_Dca_ClassAveragePrice {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _2015: MetricPattern4::new(client.clone(), "dca_class_2015_average_price".to_string()),
            _2016: MetricPattern4::new(client.clone(), "dca_class_2016_average_price".to_string()),
            _2017: MetricPattern4::new(client.clone(), "dca_class_2017_average_price".to_string()),
            _2018: MetricPattern4::new(client.clone(), "dca_class_2018_average_price".to_string()),
            _2019: MetricPattern4::new(client.clone(), "dca_class_2019_average_price".to_string()),
            _2020: MetricPattern4::new(client.clone(), "dca_class_2020_average_price".to_string()),
            _2021: MetricPattern4::new(client.clone(), "dca_class_2021_average_price".to_string()),
            _2022: MetricPattern4::new(client.clone(), "dca_class_2022_average_price".to_string()),
            _2023: MetricPattern4::new(client.clone(), "dca_class_2023_average_price".to_string()),
            _2024: MetricPattern4::new(client.clone(), "dca_class_2024_average_price".to_string()),
            _2025: MetricPattern4::new(client.clone(), "dca_class_2025_average_price".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Market_Dca_ClassStack {
    pub _2015: _2015Pattern,
    pub _2016: _2015Pattern,
    pub _2017: _2015Pattern,
    pub _2018: _2015Pattern,
    pub _2019: _2015Pattern,
    pub _2020: _2015Pattern,
    pub _2021: _2015Pattern,
    pub _2022: _2015Pattern,
    pub _2023: _2015Pattern,
    pub _2024: _2015Pattern,
    pub _2025: _2015Pattern,
}

impl MetricsTree_Market_Dca_ClassStack {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _2015: _2015Pattern::new(client.clone(), "dca_class_2015_stack".to_string()),
            _2016: _2015Pattern::new(client.clone(), "dca_class_2016_stack".to_string()),
            _2017: _2015Pattern::new(client.clone(), "dca_class_2017_stack".to_string()),
            _2018: _2015Pattern::new(client.clone(), "dca_class_2018_stack".to_string()),
            _2019: _2015Pattern::new(client.clone(), "dca_class_2019_stack".to_string()),
            _2020: _2015Pattern::new(client.clone(), "dca_class_2020_stack".to_string()),
            _2021: _2015Pattern::new(client.clone(), "dca_class_2021_stack".to_string()),
            _2022: _2015Pattern::new(client.clone(), "dca_class_2022_stack".to_string()),
            _2023: _2015Pattern::new(client.clone(), "dca_class_2023_stack".to_string()),
            _2024: _2015Pattern::new(client.clone(), "dca_class_2024_stack".to_string()),
            _2025: _2015Pattern::new(client.clone(), "dca_class_2025_stack".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Market_Indicators {
    pub gini: MetricPattern6<StoredF32>,
    pub macd_histogram: MetricPattern6<StoredF32>,
    pub macd_line: MetricPattern6<StoredF32>,
    pub macd_signal: MetricPattern6<StoredF32>,
    pub nvt: MetricPattern4<StoredF32>,
    pub pi_cycle: MetricPattern6<StoredF32>,
    pub puell_multiple: MetricPattern4<StoredF32>,
    pub rsi_14d: MetricPattern6<StoredF32>,
    pub rsi_14d_max: MetricPattern6<StoredF32>,
    pub rsi_14d_min: MetricPattern6<StoredF32>,
    pub rsi_average_gain_14d: MetricPattern6<StoredF32>,
    pub rsi_average_loss_14d: MetricPattern6<StoredF32>,
    pub rsi_gains: MetricPattern6<StoredF32>,
    pub rsi_losses: MetricPattern6<StoredF32>,
    pub stoch_d: MetricPattern6<StoredF32>,
    pub stoch_k: MetricPattern6<StoredF32>,
    pub stoch_rsi: MetricPattern6<StoredF32>,
    pub stoch_rsi_d: MetricPattern6<StoredF32>,
    pub stoch_rsi_k: MetricPattern6<StoredF32>,
}

impl MetricsTree_Market_Indicators {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            gini: MetricPattern6::new(client.clone(), "gini".to_string()),
            macd_histogram: MetricPattern6::new(client.clone(), "macd_histogram".to_string()),
            macd_line: MetricPattern6::new(client.clone(), "macd_line".to_string()),
            macd_signal: MetricPattern6::new(client.clone(), "macd_signal".to_string()),
            nvt: MetricPattern4::new(client.clone(), "nvt".to_string()),
            pi_cycle: MetricPattern6::new(client.clone(), "pi_cycle".to_string()),
            puell_multiple: MetricPattern4::new(client.clone(), "puell_multiple".to_string()),
            rsi_14d: MetricPattern6::new(client.clone(), "rsi_14d".to_string()),
            rsi_14d_max: MetricPattern6::new(client.clone(), "rsi_14d_max".to_string()),
            rsi_14d_min: MetricPattern6::new(client.clone(), "rsi_14d_min".to_string()),
            rsi_average_gain_14d: MetricPattern6::new(
                client.clone(),
                "rsi_average_gain_14d".to_string(),
            ),
            rsi_average_loss_14d: MetricPattern6::new(
                client.clone(),
                "rsi_average_loss_14d".to_string(),
            ),
            rsi_gains: MetricPattern6::new(client.clone(), "rsi_gains".to_string()),
            rsi_losses: MetricPattern6::new(client.clone(), "rsi_losses".to_string()),
            stoch_d: MetricPattern6::new(client.clone(), "stoch_d".to_string()),
            stoch_k: MetricPattern6::new(client.clone(), "stoch_k".to_string()),
            stoch_rsi: MetricPattern6::new(client.clone(), "stoch_rsi".to_string()),
            stoch_rsi_d: MetricPattern6::new(client.clone(), "stoch_rsi_d".to_string()),
            stoch_rsi_k: MetricPattern6::new(client.clone(), "stoch_rsi_k".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Market_MovingAverage {
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

impl MetricsTree_Market_MovingAverage {
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
            price_200d_sma_x0_8: MetricPattern4::new(
                client.clone(),
                "price_200d_sma_x0_8".to_string(),
            ),
            price_200d_sma_x2_4: MetricPattern4::new(
                client.clone(),
                "price_200d_sma_x2_4".to_string(),
            ),
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
            price_350d_sma_x2: MetricPattern4::new(client.clone(), "price_350d_sma_x2".to_string()),
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

/// Metrics tree node.
pub struct MetricsTree_Market_Range {
    pub price_1m_max: MetricPattern4<Dollars>,
    pub price_1m_min: MetricPattern4<Dollars>,
    pub price_1w_max: MetricPattern4<Dollars>,
    pub price_1w_min: MetricPattern4<Dollars>,
    pub price_1y_max: MetricPattern4<Dollars>,
    pub price_1y_min: MetricPattern4<Dollars>,
    pub price_2w_choppiness_index: MetricPattern4<StoredF32>,
    pub price_2w_max: MetricPattern4<Dollars>,
    pub price_2w_min: MetricPattern4<Dollars>,
    pub price_true_range: MetricPattern6<StoredF32>,
    pub price_true_range_2w_sum: MetricPattern6<StoredF32>,
}

impl MetricsTree_Market_Range {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            price_1m_max: MetricPattern4::new(client.clone(), "price_1m_max".to_string()),
            price_1m_min: MetricPattern4::new(client.clone(), "price_1m_min".to_string()),
            price_1w_max: MetricPattern4::new(client.clone(), "price_1w_max".to_string()),
            price_1w_min: MetricPattern4::new(client.clone(), "price_1w_min".to_string()),
            price_1y_max: MetricPattern4::new(client.clone(), "price_1y_max".to_string()),
            price_1y_min: MetricPattern4::new(client.clone(), "price_1y_min".to_string()),
            price_2w_choppiness_index: MetricPattern4::new(
                client.clone(),
                "price_2w_choppiness_index".to_string(),
            ),
            price_2w_max: MetricPattern4::new(client.clone(), "price_2w_max".to_string()),
            price_2w_min: MetricPattern4::new(client.clone(), "price_2w_min".to_string()),
            price_true_range: MetricPattern6::new(client.clone(), "price_true_range".to_string()),
            price_true_range_2w_sum: MetricPattern6::new(
                client.clone(),
                "price_true_range_2w_sum".to_string(),
            ),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Market_Returns {
    pub _1d_returns_1m_sd: _1dReturns1mSdPattern,
    pub _1d_returns_1w_sd: _1dReturns1mSdPattern,
    pub _1d_returns_1y_sd: _1dReturns1mSdPattern,
    pub cagr: PeriodCagrPattern,
    pub downside_1m_sd: _1dReturns1mSdPattern,
    pub downside_1w_sd: _1dReturns1mSdPattern,
    pub downside_1y_sd: _1dReturns1mSdPattern,
    pub downside_returns: MetricPattern6<StoredF32>,
    pub price_returns: MetricsTree_Market_Returns_PriceReturns,
}

impl MetricsTree_Market_Returns {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _1d_returns_1m_sd: _1dReturns1mSdPattern::new(
                client.clone(),
                "1d_returns_1m_sd".to_string(),
            ),
            _1d_returns_1w_sd: _1dReturns1mSdPattern::new(
                client.clone(),
                "1d_returns_1w_sd".to_string(),
            ),
            _1d_returns_1y_sd: _1dReturns1mSdPattern::new(
                client.clone(),
                "1d_returns_1y_sd".to_string(),
            ),
            cagr: PeriodCagrPattern::new(client.clone(), "cagr".to_string()),
            downside_1m_sd: _1dReturns1mSdPattern::new(
                client.clone(),
                "downside_1m_sd".to_string(),
            ),
            downside_1w_sd: _1dReturns1mSdPattern::new(
                client.clone(),
                "downside_1w_sd".to_string(),
            ),
            downside_1y_sd: _1dReturns1mSdPattern::new(
                client.clone(),
                "downside_1y_sd".to_string(),
            ),
            downside_returns: MetricPattern6::new(client.clone(), "downside_returns".to_string()),
            price_returns: MetricsTree_Market_Returns_PriceReturns::new(
                client.clone(),
                format!("{base_path}_price_returns"),
            ),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Market_Returns_PriceReturns {
    pub _10y: MetricPattern4<StoredF32>,
    pub _1d: MetricPattern4<StoredF32>,
    pub _1m: MetricPattern4<StoredF32>,
    pub _1w: MetricPattern4<StoredF32>,
    pub _1y: MetricPattern4<StoredF32>,
    pub _2y: MetricPattern4<StoredF32>,
    pub _3m: MetricPattern4<StoredF32>,
    pub _3y: MetricPattern4<StoredF32>,
    pub _4y: MetricPattern4<StoredF32>,
    pub _5y: MetricPattern4<StoredF32>,
    pub _6m: MetricPattern4<StoredF32>,
    pub _6y: MetricPattern4<StoredF32>,
    pub _8y: MetricPattern4<StoredF32>,
}

impl MetricsTree_Market_Returns_PriceReturns {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _10y: MetricPattern4::new(client.clone(), "10y_price_returns".to_string()),
            _1d: MetricPattern4::new(client.clone(), "1d_price_returns".to_string()),
            _1m: MetricPattern4::new(client.clone(), "1m_price_returns".to_string()),
            _1w: MetricPattern4::new(client.clone(), "1w_price_returns".to_string()),
            _1y: MetricPattern4::new(client.clone(), "1y_price_returns".to_string()),
            _2y: MetricPattern4::new(client.clone(), "2y_price_returns".to_string()),
            _3m: MetricPattern4::new(client.clone(), "3m_price_returns".to_string()),
            _3y: MetricPattern4::new(client.clone(), "3y_price_returns".to_string()),
            _4y: MetricPattern4::new(client.clone(), "4y_price_returns".to_string()),
            _5y: MetricPattern4::new(client.clone(), "5y_price_returns".to_string()),
            _6m: MetricPattern4::new(client.clone(), "6m_price_returns".to_string()),
            _6y: MetricPattern4::new(client.clone(), "6y_price_returns".to_string()),
            _8y: MetricPattern4::new(client.clone(), "8y_price_returns".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Market_Volatility {
    pub price_1m_volatility: MetricPattern4<StoredF32>,
    pub price_1w_volatility: MetricPattern4<StoredF32>,
    pub price_1y_volatility: MetricPattern4<StoredF32>,
    pub sharpe_1m: MetricPattern6<StoredF32>,
    pub sharpe_1w: MetricPattern6<StoredF32>,
    pub sharpe_1y: MetricPattern6<StoredF32>,
    pub sortino_1m: MetricPattern6<StoredF32>,
    pub sortino_1w: MetricPattern6<StoredF32>,
    pub sortino_1y: MetricPattern6<StoredF32>,
}

impl MetricsTree_Market_Volatility {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            price_1m_volatility: MetricPattern4::new(
                client.clone(),
                "price_1m_volatility".to_string(),
            ),
            price_1w_volatility: MetricPattern4::new(
                client.clone(),
                "price_1w_volatility".to_string(),
            ),
            price_1y_volatility: MetricPattern4::new(
                client.clone(),
                "price_1y_volatility".to_string(),
            ),
            sharpe_1m: MetricPattern6::new(client.clone(), "sharpe_1m".to_string()),
            sharpe_1w: MetricPattern6::new(client.clone(), "sharpe_1w".to_string()),
            sharpe_1y: MetricPattern6::new(client.clone(), "sharpe_1y".to_string()),
            sortino_1m: MetricPattern6::new(client.clone(), "sortino_1m".to_string()),
            sortino_1w: MetricPattern6::new(client.clone(), "sortino_1w".to_string()),
            sortino_1y: MetricPattern6::new(client.clone(), "sortino_1y".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Outputs {
    pub count: MetricsTree_Outputs_Count,
    pub first_txoutindex: MetricPattern11<TxOutIndex>,
    pub outputtype: MetricPattern15<OutputType>,
    pub spent: MetricsTree_Outputs_Spent,
    pub txindex: MetricPattern15<TxIndex>,
    pub typeindex: MetricPattern15<TypeIndex>,
    pub value: MetricPattern15<Sats>,
}

impl MetricsTree_Outputs {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            count: MetricsTree_Outputs_Count::new(client.clone(), format!("{base_path}_count")),
            first_txoutindex: MetricPattern11::new(client.clone(), "first_txoutindex".to_string()),
            outputtype: MetricPattern15::new(client.clone(), "outputtype".to_string()),
            spent: MetricsTree_Outputs_Spent::new(client.clone(), format!("{base_path}_spent")),
            txindex: MetricPattern15::new(client.clone(), "txindex".to_string()),
            typeindex: MetricPattern15::new(client.clone(), "typeindex".to_string()),
            value: MetricPattern15::new(client.clone(), "value".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Outputs_Count {
    pub total_count: CountPattern2<StoredU64>,
    pub utxo_count: MetricPattern1<StoredU64>,
}

impl MetricsTree_Outputs_Count {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            total_count: CountPattern2::new(client.clone(), "output_count".to_string()),
            utxo_count: MetricPattern1::new(client.clone(), "exact_utxo_count".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Outputs_Spent {
    pub txinindex: MetricPattern15<TxInIndex>,
}

impl MetricsTree_Outputs_Spent {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            txinindex: MetricPattern15::new(client.clone(), "txinindex".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Pools {
    pub height_to_pool: MetricPattern11<PoolSlug>,
    pub vecs: MetricsTree_Pools_Vecs,
}

impl MetricsTree_Pools {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            height_to_pool: MetricPattern11::new(client.clone(), "pool".to_string()),
            vecs: MetricsTree_Pools_Vecs::new(client.clone(), format!("{base_path}_vecs")),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Pools_Vecs {
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

impl MetricsTree_Pools_Vecs {
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
            bitcoinaffiliatenetwork: AaopoolPattern::new(
                client.clone(),
                "bitcoinaffiliatenetwork".to_string(),
            ),
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
            entrustcharitypool: AaopoolPattern::new(
                client.clone(),
                "entrustcharitypool".to_string(),
            ),
            eobot: AaopoolPattern::new(client.clone(), "eobot".to_string()),
            exxbw: AaopoolPattern::new(client.clone(), "exxbw".to_string()),
            f2pool: AaopoolPattern::new(client.clone(), "f2pool".to_string()),
            fiftyeightcoin: AaopoolPattern::new(client.clone(), "fiftyeightcoin".to_string()),
            foundryusa: AaopoolPattern::new(client.clone(), "foundryusa".to_string()),
            futurebitapollosolo: AaopoolPattern::new(
                client.clone(),
                "futurebitapollosolo".to_string(),
            ),
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
            transactioncoinmining: AaopoolPattern::new(
                client.clone(),
                "transactioncoinmining".to_string(),
            ),
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

/// Metrics tree node.
pub struct MetricsTree_Positions {
    pub block_position: MetricPattern11<BlkPosition>,
    pub tx_position: MetricPattern27<BlkPosition>,
}

impl MetricsTree_Positions {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            block_position: MetricPattern11::new(client.clone(), "position".to_string()),
            tx_position: MetricPattern27::new(client.clone(), "position".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Price {
    pub cents: MetricsTree_Price_Cents,
    pub oracle: MetricsTree_Price_Oracle,
    pub sats: SatsPattern<OHLCSats>,
    pub usd: MetricsTree_Price_Usd,
}

impl MetricsTree_Price {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            cents: MetricsTree_Price_Cents::new(client.clone(), format!("{base_path}_cents")),
            oracle: MetricsTree_Price_Oracle::new(client.clone(), format!("{base_path}_oracle")),
            sats: SatsPattern::new(client.clone(), "price".to_string()),
            usd: MetricsTree_Price_Usd::new(client.clone(), format!("{base_path}_usd")),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Price_Cents {
    pub ohlc: MetricPattern5<OHLCCents>,
    pub split: MetricsTree_Price_Cents_Split,
}

impl MetricsTree_Price_Cents {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            ohlc: MetricPattern5::new(client.clone(), "ohlc_cents".to_string()),
            split: MetricsTree_Price_Cents_Split::new(client.clone(), format!("{base_path}_split")),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Price_Cents_Split {
    pub close: MetricPattern5<Cents>,
    pub high: MetricPattern5<Cents>,
    pub low: MetricPattern5<Cents>,
    pub open: MetricPattern5<Cents>,
}

impl MetricsTree_Price_Cents_Split {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            close: MetricPattern5::new(client.clone(), "price_close_cents".to_string()),
            high: MetricPattern5::new(client.clone(), "price_high_cents".to_string()),
            low: MetricPattern5::new(client.clone(), "price_low_cents".to_string()),
            open: MetricPattern5::new(client.clone(), "price_open_cents".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Price_Oracle {
    pub height_to_first_pairoutputindex: MetricPattern11<PairOutputIndex>,
    pub ohlc_cents: MetricPattern6<OHLCCents>,
    pub ohlc_dollars: MetricPattern6<OHLCDollars>,
    pub output0_value: MetricPattern33<Sats>,
    pub output1_value: MetricPattern33<Sats>,
    pub pairoutputindex_to_txindex: MetricPattern33<TxIndex>,
    pub phase_daily_cents: PhaseDailyCentsPattern<Cents>,
    pub phase_daily_dollars: PhaseDailyCentsPattern<Dollars>,
    pub phase_histogram: MetricPattern11<OracleBins>,
    pub phase_price_cents: MetricPattern11<Cents>,
    pub phase_v2_daily_cents: PhaseDailyCentsPattern<Cents>,
    pub phase_v2_daily_dollars: PhaseDailyCentsPattern<Dollars>,
    pub phase_v2_histogram: MetricPattern11<OracleBinsV2>,
    pub phase_v2_peak_daily_cents: PhaseDailyCentsPattern<Cents>,
    pub phase_v2_peak_daily_dollars: PhaseDailyCentsPattern<Dollars>,
    pub phase_v2_peak_price_cents: MetricPattern11<Cents>,
    pub phase_v2_price_cents: MetricPattern11<Cents>,
    pub phase_v3_daily_cents: PhaseDailyCentsPattern<Cents>,
    pub phase_v3_daily_dollars: PhaseDailyCentsPattern<Dollars>,
    pub phase_v3_histogram: MetricPattern11<OracleBinsV2>,
    pub phase_v3_peak_daily_cents: PhaseDailyCentsPattern<Cents>,
    pub phase_v3_peak_daily_dollars: PhaseDailyCentsPattern<Dollars>,
    pub phase_v3_peak_price_cents: MetricPattern11<Cents>,
    pub phase_v3_price_cents: MetricPattern11<Cents>,
    pub price_cents: MetricPattern11<Cents>,
    pub tx_count: MetricPattern6<StoredU32>,
}

impl MetricsTree_Price_Oracle {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            height_to_first_pairoutputindex: MetricPattern11::new(
                client.clone(),
                "height_to_first_pairoutputindex".to_string(),
            ),
            ohlc_cents: MetricPattern6::new(client.clone(), "oracle_ohlc_cents".to_string()),
            ohlc_dollars: MetricPattern6::new(client.clone(), "oracle_ohlc".to_string()),
            output0_value: MetricPattern33::new(client.clone(), "pair_output0_value".to_string()),
            output1_value: MetricPattern33::new(client.clone(), "pair_output1_value".to_string()),
            pairoutputindex_to_txindex: MetricPattern33::new(
                client.clone(),
                "pairoutputindex_to_txindex".to_string(),
            ),
            phase_daily_cents: PhaseDailyCentsPattern::new(
                client.clone(),
                "phase_daily".to_string(),
            ),
            phase_daily_dollars: PhaseDailyCentsPattern::new(
                client.clone(),
                "phase_daily_dollars".to_string(),
            ),
            phase_histogram: MetricPattern11::new(client.clone(), "phase_histogram".to_string()),
            phase_price_cents: MetricPattern11::new(
                client.clone(),
                "phase_price_cents".to_string(),
            ),
            phase_v2_daily_cents: PhaseDailyCentsPattern::new(
                client.clone(),
                "phase_v2_daily".to_string(),
            ),
            phase_v2_daily_dollars: PhaseDailyCentsPattern::new(
                client.clone(),
                "phase_v2_daily_dollars".to_string(),
            ),
            phase_v2_histogram: MetricPattern11::new(
                client.clone(),
                "phase_v2_histogram".to_string(),
            ),
            phase_v2_peak_daily_cents: PhaseDailyCentsPattern::new(
                client.clone(),
                "phase_v2_peak_daily".to_string(),
            ),
            phase_v2_peak_daily_dollars: PhaseDailyCentsPattern::new(
                client.clone(),
                "phase_v2_peak_daily_dollars".to_string(),
            ),
            phase_v2_peak_price_cents: MetricPattern11::new(
                client.clone(),
                "phase_v2_peak_price_cents".to_string(),
            ),
            phase_v2_price_cents: MetricPattern11::new(
                client.clone(),
                "phase_v2_price_cents".to_string(),
            ),
            phase_v3_daily_cents: PhaseDailyCentsPattern::new(
                client.clone(),
                "phase_v3_daily".to_string(),
            ),
            phase_v3_daily_dollars: PhaseDailyCentsPattern::new(
                client.clone(),
                "phase_v3_daily_dollars".to_string(),
            ),
            phase_v3_histogram: MetricPattern11::new(
                client.clone(),
                "phase_v3_histogram".to_string(),
            ),
            phase_v3_peak_daily_cents: PhaseDailyCentsPattern::new(
                client.clone(),
                "phase_v3_peak_daily".to_string(),
            ),
            phase_v3_peak_daily_dollars: PhaseDailyCentsPattern::new(
                client.clone(),
                "phase_v3_peak_daily_dollars".to_string(),
            ),
            phase_v3_peak_price_cents: MetricPattern11::new(
                client.clone(),
                "phase_v3_peak_price_cents".to_string(),
            ),
            phase_v3_price_cents: MetricPattern11::new(
                client.clone(),
                "phase_v3_price_cents".to_string(),
            ),
            price_cents: MetricPattern11::new(client.clone(), "oracle_price_cents".to_string()),
            tx_count: MetricPattern6::new(client.clone(), "oracle_tx_count".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Price_Usd {
    pub ohlc: MetricPattern1<OHLCDollars>,
    pub split: SplitPattern2<Dollars>,
}

impl MetricsTree_Price_Usd {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            ohlc: MetricPattern1::new(client.clone(), "price_ohlc".to_string()),
            split: SplitPattern2::new(client.clone(), "price".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Scripts {
    pub count: MetricsTree_Scripts_Count,
    pub empty_to_txindex: MetricPattern9<TxIndex>,
    pub first_emptyoutputindex: MetricPattern11<EmptyOutputIndex>,
    pub first_opreturnindex: MetricPattern11<OpReturnIndex>,
    pub first_p2msoutputindex: MetricPattern11<P2MSOutputIndex>,
    pub first_unknownoutputindex: MetricPattern11<UnknownOutputIndex>,
    pub opreturn_to_txindex: MetricPattern14<TxIndex>,
    pub p2ms_to_txindex: MetricPattern17<TxIndex>,
    pub unknown_to_txindex: MetricPattern28<TxIndex>,
    pub value: MetricsTree_Scripts_Value,
}

impl MetricsTree_Scripts {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            count: MetricsTree_Scripts_Count::new(client.clone(), format!("{base_path}_count")),
            empty_to_txindex: MetricPattern9::new(client.clone(), "txindex".to_string()),
            first_emptyoutputindex: MetricPattern11::new(
                client.clone(),
                "first_emptyoutputindex".to_string(),
            ),
            first_opreturnindex: MetricPattern11::new(
                client.clone(),
                "first_opreturnindex".to_string(),
            ),
            first_p2msoutputindex: MetricPattern11::new(
                client.clone(),
                "first_p2msoutputindex".to_string(),
            ),
            first_unknownoutputindex: MetricPattern11::new(
                client.clone(),
                "first_unknownoutputindex".to_string(),
            ),
            opreturn_to_txindex: MetricPattern14::new(client.clone(), "txindex".to_string()),
            p2ms_to_txindex: MetricPattern17::new(client.clone(), "txindex".to_string()),
            unknown_to_txindex: MetricPattern28::new(client.clone(), "txindex".to_string()),
            value: MetricsTree_Scripts_Value::new(client.clone(), format!("{base_path}_value")),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Scripts_Count {
    pub emptyoutput: DollarsPattern<StoredU64>,
    pub opreturn: DollarsPattern<StoredU64>,
    pub p2a: DollarsPattern<StoredU64>,
    pub p2ms: DollarsPattern<StoredU64>,
    pub p2pk33: DollarsPattern<StoredU64>,
    pub p2pk65: DollarsPattern<StoredU64>,
    pub p2pkh: DollarsPattern<StoredU64>,
    pub p2sh: DollarsPattern<StoredU64>,
    pub p2tr: DollarsPattern<StoredU64>,
    pub p2wpkh: DollarsPattern<StoredU64>,
    pub p2wsh: DollarsPattern<StoredU64>,
    pub segwit: DollarsPattern<StoredU64>,
    pub segwit_adoption: SegwitAdoptionPattern,
    pub taproot_adoption: SegwitAdoptionPattern,
    pub unknownoutput: DollarsPattern<StoredU64>,
}

impl MetricsTree_Scripts_Count {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            emptyoutput: DollarsPattern::new(client.clone(), "emptyoutput_count".to_string()),
            opreturn: DollarsPattern::new(client.clone(), "opreturn_count".to_string()),
            p2a: DollarsPattern::new(client.clone(), "p2a_count".to_string()),
            p2ms: DollarsPattern::new(client.clone(), "p2ms_count".to_string()),
            p2pk33: DollarsPattern::new(client.clone(), "p2pk33_count".to_string()),
            p2pk65: DollarsPattern::new(client.clone(), "p2pk65_count".to_string()),
            p2pkh: DollarsPattern::new(client.clone(), "p2pkh_count".to_string()),
            p2sh: DollarsPattern::new(client.clone(), "p2sh_count".to_string()),
            p2tr: DollarsPattern::new(client.clone(), "p2tr_count".to_string()),
            p2wpkh: DollarsPattern::new(client.clone(), "p2wpkh_count".to_string()),
            p2wsh: DollarsPattern::new(client.clone(), "p2wsh_count".to_string()),
            segwit: DollarsPattern::new(client.clone(), "segwit_count".to_string()),
            segwit_adoption: SegwitAdoptionPattern::new(
                client.clone(),
                "segwit_adoption".to_string(),
            ),
            taproot_adoption: SegwitAdoptionPattern::new(
                client.clone(),
                "taproot_adoption".to_string(),
            ),
            unknownoutput: DollarsPattern::new(client.clone(), "unknownoutput_count".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Scripts_Value {
    pub opreturn: CoinbasePattern,
}

impl MetricsTree_Scripts_Value {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            opreturn: CoinbasePattern::new(client.clone(), "opreturn_value".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Supply {
    pub burned: MetricsTree_Supply_Burned,
    pub circulating: MetricsTree_Supply_Circulating,
    pub inflation: MetricPattern4<StoredF32>,
    pub market_cap: MetricPattern1<Dollars>,
    pub velocity: MetricsTree_Supply_Velocity,
}

impl MetricsTree_Supply {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            burned: MetricsTree_Supply_Burned::new(client.clone(), format!("{base_path}_burned")),
            circulating: MetricsTree_Supply_Circulating::new(
                client.clone(),
                format!("{base_path}_circulating"),
            ),
            inflation: MetricPattern4::new(client.clone(), "inflation_rate".to_string()),
            market_cap: MetricPattern1::new(client.clone(), "market_cap".to_string()),
            velocity: MetricsTree_Supply_Velocity::new(
                client.clone(),
                format!("{base_path}_velocity"),
            ),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Supply_Burned {
    pub opreturn: UnclaimedRewardsPattern,
    pub unspendable: UnclaimedRewardsPattern,
}

impl MetricsTree_Supply_Burned {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            opreturn: UnclaimedRewardsPattern::new(client.clone(), "opreturn_supply".to_string()),
            unspendable: UnclaimedRewardsPattern::new(
                client.clone(),
                "unspendable_supply".to_string(),
            ),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Supply_Circulating {
    pub bitcoin: MetricPattern3<Bitcoin>,
    pub dollars: MetricPattern3<Dollars>,
    pub sats: MetricPattern3<Sats>,
}

impl MetricsTree_Supply_Circulating {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            bitcoin: MetricPattern3::new(client.clone(), "circulating_supply_btc".to_string()),
            dollars: MetricPattern3::new(client.clone(), "circulating_supply_usd".to_string()),
            sats: MetricPattern3::new(client.clone(), "circulating_supply".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Supply_Velocity {
    pub btc: MetricPattern4<StoredF64>,
    pub usd: MetricPattern4<StoredF64>,
}

impl MetricsTree_Supply_Velocity {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            btc: MetricPattern4::new(client.clone(), "btc_velocity".to_string()),
            usd: MetricPattern4::new(client.clone(), "usd_velocity".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Transactions {
    pub base_size: MetricPattern27<StoredU32>,
    pub count: MetricsTree_Transactions_Count,
    pub fees: MetricsTree_Transactions_Fees,
    pub first_txindex: MetricPattern11<TxIndex>,
    pub first_txinindex: MetricPattern27<TxInIndex>,
    pub first_txoutindex: MetricPattern27<TxOutIndex>,
    pub height: MetricPattern27<Height>,
    pub is_explicitly_rbf: MetricPattern27<StoredBool>,
    pub rawlocktime: MetricPattern27<RawLockTime>,
    pub size: MetricsTree_Transactions_Size,
    pub total_size: MetricPattern27<StoredU32>,
    pub txid: MetricPattern27<Txid>,
    pub txversion: MetricPattern27<TxVersion>,
    pub versions: MetricsTree_Transactions_Versions,
    pub volume: MetricsTree_Transactions_Volume,
}

impl MetricsTree_Transactions {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            base_size: MetricPattern27::new(client.clone(), "base_size".to_string()),
            count: MetricsTree_Transactions_Count::new(
                client.clone(),
                format!("{base_path}_count"),
            ),
            fees: MetricsTree_Transactions_Fees::new(client.clone(), format!("{base_path}_fees")),
            first_txindex: MetricPattern11::new(client.clone(), "first_txindex".to_string()),
            first_txinindex: MetricPattern27::new(client.clone(), "first_txinindex".to_string()),
            first_txoutindex: MetricPattern27::new(client.clone(), "first_txoutindex".to_string()),
            height: MetricPattern27::new(client.clone(), "height".to_string()),
            is_explicitly_rbf: MetricPattern27::new(
                client.clone(),
                "is_explicitly_rbf".to_string(),
            ),
            rawlocktime: MetricPattern27::new(client.clone(), "rawlocktime".to_string()),
            size: MetricsTree_Transactions_Size::new(client.clone(), format!("{base_path}_size")),
            total_size: MetricPattern27::new(client.clone(), "total_size".to_string()),
            txid: MetricPattern27::new(client.clone(), "txid".to_string()),
            txversion: MetricPattern27::new(client.clone(), "txversion".to_string()),
            versions: MetricsTree_Transactions_Versions::new(
                client.clone(),
                format!("{base_path}_versions"),
            ),
            volume: MetricsTree_Transactions_Volume::new(
                client.clone(),
                format!("{base_path}_volume"),
            ),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Transactions_Count {
    pub is_coinbase: MetricPattern27<StoredBool>,
    pub tx_count: DollarsPattern<StoredU64>,
}

impl MetricsTree_Transactions_Count {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            is_coinbase: MetricPattern27::new(client.clone(), "is_coinbase".to_string()),
            tx_count: DollarsPattern::new(client.clone(), "tx_count".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Transactions_Fees {
    pub fee: MetricsTree_Transactions_Fees_Fee,
    pub fee_rate: FeeRatePattern<FeeRate>,
    pub input_value: MetricPattern27<Sats>,
    pub output_value: MetricPattern27<Sats>,
}

impl MetricsTree_Transactions_Fees {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            fee: MetricsTree_Transactions_Fees_Fee::new(client.clone(), format!("{base_path}_fee")),
            fee_rate: FeeRatePattern::new(client.clone(), "fee_rate".to_string()),
            input_value: MetricPattern27::new(client.clone(), "input_value".to_string()),
            output_value: MetricPattern27::new(client.clone(), "output_value".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Transactions_Fees_Fee {
    pub bitcoin: CountPattern2<Bitcoin>,
    pub dollars: MetricsTree_Transactions_Fees_Fee_Dollars,
    pub sats: CountPattern2<Sats>,
    pub txindex: MetricPattern27<Sats>,
}

impl MetricsTree_Transactions_Fees_Fee {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            bitcoin: CountPattern2::new(client.clone(), "fee_btc".to_string()),
            dollars: MetricsTree_Transactions_Fees_Fee_Dollars::new(
                client.clone(),
                format!("{base_path}_dollars"),
            ),
            sats: CountPattern2::new(client.clone(), "fee".to_string()),
            txindex: MetricPattern27::new(client.clone(), "fee".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Transactions_Fees_Fee_Dollars {
    pub average: MetricPattern1<Dollars>,
    pub cumulative: MetricPattern2<Dollars>,
    pub height_cumulative: MetricPattern11<Dollars>,
    pub max: MetricPattern1<Dollars>,
    pub median: MetricPattern11<Dollars>,
    pub min: MetricPattern1<Dollars>,
    pub pct10: MetricPattern11<Dollars>,
    pub pct25: MetricPattern11<Dollars>,
    pub pct75: MetricPattern11<Dollars>,
    pub pct90: MetricPattern11<Dollars>,
    pub sum: MetricPattern1<Dollars>,
}

impl MetricsTree_Transactions_Fees_Fee_Dollars {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            average: MetricPattern1::new(client.clone(), "fee_usd_average".to_string()),
            cumulative: MetricPattern2::new(client.clone(), "fee_usd_cumulative".to_string()),
            height_cumulative: MetricPattern11::new(
                client.clone(),
                "fee_usd_cumulative".to_string(),
            ),
            max: MetricPattern1::new(client.clone(), "fee_usd_max".to_string()),
            median: MetricPattern11::new(client.clone(), "fee_usd_median".to_string()),
            min: MetricPattern1::new(client.clone(), "fee_usd_min".to_string()),
            pct10: MetricPattern11::new(client.clone(), "fee_usd_pct10".to_string()),
            pct25: MetricPattern11::new(client.clone(), "fee_usd_pct25".to_string()),
            pct75: MetricPattern11::new(client.clone(), "fee_usd_pct75".to_string()),
            pct90: MetricPattern11::new(client.clone(), "fee_usd_pct90".to_string()),
            sum: MetricPattern1::new(client.clone(), "fee_usd_sum".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Transactions_Size {
    pub vsize: FeeRatePattern<VSize>,
    pub weight: FeeRatePattern<Weight>,
}

impl MetricsTree_Transactions_Size {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            vsize: FeeRatePattern::new(client.clone(), "tx_vsize".to_string()),
            weight: FeeRatePattern::new(client.clone(), "tx_weight".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Transactions_Versions {
    pub v1: BlockCountPattern<StoredU64>,
    pub v2: BlockCountPattern<StoredU64>,
    pub v3: BlockCountPattern<StoredU64>,
}

impl MetricsTree_Transactions_Versions {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            v1: BlockCountPattern::new(client.clone(), "tx_v1".to_string()),
            v2: BlockCountPattern::new(client.clone(), "tx_v2".to_string()),
            v3: BlockCountPattern::new(client.clone(), "tx_v3".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Transactions_Volume {
    pub annualized_volume: _2015Pattern,
    pub inputs_per_sec: MetricPattern4<StoredF32>,
    pub outputs_per_sec: MetricPattern4<StoredF32>,
    pub sent_sum: ActiveSupplyPattern,
    pub tx_per_sec: MetricPattern4<StoredF32>,
}

impl MetricsTree_Transactions_Volume {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            annualized_volume: _2015Pattern::new(client.clone(), "annualized_volume".to_string()),
            inputs_per_sec: MetricPattern4::new(client.clone(), "inputs_per_sec".to_string()),
            outputs_per_sec: MetricPattern4::new(client.clone(), "outputs_per_sec".to_string()),
            sent_sum: ActiveSupplyPattern::new(client.clone(), "sent_sum".to_string()),
            tx_per_sec: MetricPattern4::new(client.clone(), "tx_per_sec".to_string()),
        }
    }
}

/// Main BRK client with metrics tree and API methods.
pub struct BrkClient {
    base: Arc<BrkClientBase>,
    metrics: MetricsTree,
}

impl BrkClient {
    /// Client version.
    pub const VERSION: &'static str = "v0.1.0-alpha.3";

    /// Create a new client with the given base URL.
    pub fn new(base_url: impl Into<String>) -> Self {
        let base = Arc::new(BrkClientBase::new(base_url));
        let metrics = MetricsTree::new(base.clone(), String::new());
        Self { base, metrics }
    }

    /// Create a new client with options.
    pub fn with_options(options: BrkClientOptions) -> Self {
        let base = Arc::new(BrkClientBase::with_options(options));
        let metrics = MetricsTree::new(base.clone(), String::new());
        Self { base, metrics }
    }

    /// Get the metrics tree for navigating metrics.
    pub fn metrics(&self) -> &MetricsTree {
        &self.metrics
    }

    /// Create a dynamic metric endpoint builder for any metric/index combination.
    ///
    /// Use this for programmatic access when the metric name is determined at runtime.
    /// For type-safe access, use the `metrics()` tree instead.
    ///
    /// # Example
    /// ```ignore
    /// let data = client.metric("realized_price", Index::Height)
    ///     .last(10)
    ///     .json::<f64>()?;
    /// ```
    pub fn metric(
        &self,
        metric: impl Into<Metric>,
        index: Index,
    ) -> MetricEndpointBuilder<serde_json::Value> {
        MetricEndpointBuilder::new(self.base.clone(), Arc::from(metric.into().as_str()), index)
    }

    /// Compact OpenAPI specification
    ///
    /// Compact OpenAPI specification optimized for LLM consumption. Removes redundant fields while preserving essential API information. Full spec available at `/openapi.json`.
    ///
    /// Endpoint: `GET /api.json`
    pub fn get_api(&self) -> Result<serde_json::Value> {
        self.base.get_json(&format!("/api.json"))
    }

    /// Address information
    ///
    /// Retrieve address information including balance and transaction counts. Supports all standard Bitcoin address types (P2PKH, P2SH, P2WPKH, P2WSH, P2TR).
    ///
    /// *[Mempool.space docs](https://mempool.space/docs/api/rest#get-address)*
    ///
    /// Endpoint: `GET /api/address/{address}`
    pub fn get_address(&self, address: Address) -> Result<AddressStats> {
        self.base.get_json(&format!("/api/address/{address}"))
    }

    /// Address transaction IDs
    ///
    /// Get transaction IDs for an address, newest first. Use after_txid for pagination.
    ///
    /// *[Mempool.space docs](https://mempool.space/docs/api/rest#get-address-transactions)*
    ///
    /// Endpoint: `GET /api/address/{address}/txs`
    pub fn get_address_txs(
        &self,
        address: Address,
        after_txid: Option<&str>,
        limit: Option<i64>,
    ) -> Result<Vec<Txid>> {
        let mut query = Vec::new();
        if let Some(v) = after_txid {
            query.push(format!("after_txid={}", v));
        }
        if let Some(v) = limit {
            query.push(format!("limit={}", v));
        }
        let query_str = if query.is_empty() {
            String::new()
        } else {
            format!("?{}", query.join("&"))
        };
        let path = format!("/api/address/{address}/txs{}", query_str);
        self.base.get_json(&path)
    }

    /// Address confirmed transactions
    ///
    /// Get confirmed transaction IDs for an address, 25 per page. Use ?after_txid=<txid> for pagination.
    ///
    /// *[Mempool.space docs](https://mempool.space/docs/api/rest#get-address-transactions-chain)*
    ///
    /// Endpoint: `GET /api/address/{address}/txs/chain`
    pub fn get_address_confirmed_txs(
        &self,
        address: Address,
        after_txid: Option<&str>,
        limit: Option<i64>,
    ) -> Result<Vec<Txid>> {
        let mut query = Vec::new();
        if let Some(v) = after_txid {
            query.push(format!("after_txid={}", v));
        }
        if let Some(v) = limit {
            query.push(format!("limit={}", v));
        }
        let query_str = if query.is_empty() {
            String::new()
        } else {
            format!("?{}", query.join("&"))
        };
        let path = format!("/api/address/{address}/txs/chain{}", query_str);
        self.base.get_json(&path)
    }

    /// Address mempool transactions
    ///
    /// Get unconfirmed transaction IDs for an address from the mempool (up to 50).
    ///
    /// *[Mempool.space docs](https://mempool.space/docs/api/rest#get-address-transactions-mempool)*
    ///
    /// Endpoint: `GET /api/address/{address}/txs/mempool`
    pub fn get_address_mempool_txs(&self, address: Address) -> Result<Vec<Txid>> {
        self.base
            .get_json(&format!("/api/address/{address}/txs/mempool"))
    }

    /// Address UTXOs
    ///
    /// Get unspent transaction outputs (UTXOs) for an address. Returns txid, vout, value, and confirmation status for each UTXO.
    ///
    /// *[Mempool.space docs](https://mempool.space/docs/api/rest#get-address-utxo)*
    ///
    /// Endpoint: `GET /api/address/{address}/utxo`
    pub fn get_address_utxos(&self, address: Address) -> Result<Vec<Utxo>> {
        self.base.get_json(&format!("/api/address/{address}/utxo"))
    }

    /// Block by height
    ///
    /// Retrieve block information by block height. Returns block metadata including hash, timestamp, difficulty, size, weight, and transaction count.
    ///
    /// *[Mempool.space docs](https://mempool.space/docs/api/rest#get-block-height)*
    ///
    /// Endpoint: `GET /api/block-height/{height}`
    pub fn get_block_by_height(&self, height: Height) -> Result<BlockInfo> {
        self.base.get_json(&format!("/api/block-height/{height}"))
    }

    /// Block information
    ///
    /// Retrieve block information by block hash. Returns block metadata including height, timestamp, difficulty, size, weight, and transaction count.
    ///
    /// *[Mempool.space docs](https://mempool.space/docs/api/rest#get-block)*
    ///
    /// Endpoint: `GET /api/block/{hash}`
    pub fn get_block(&self, hash: BlockHash) -> Result<BlockInfo> {
        self.base.get_json(&format!("/api/block/{hash}"))
    }

    /// Raw block
    ///
    /// Returns the raw block data in binary format.
    ///
    /// *[Mempool.space docs](https://mempool.space/docs/api/rest#get-block-raw)*
    ///
    /// Endpoint: `GET /api/block/{hash}/raw`
    pub fn get_block_raw(&self, hash: BlockHash) -> Result<Vec<f64>> {
        self.base.get_json(&format!("/api/block/{hash}/raw"))
    }

    /// Block status
    ///
    /// Retrieve the status of a block. Returns whether the block is in the best chain and, if so, its height and the hash of the next block.
    ///
    /// *[Mempool.space docs](https://mempool.space/docs/api/rest#get-block-status)*
    ///
    /// Endpoint: `GET /api/block/{hash}/status`
    pub fn get_block_status(&self, hash: BlockHash) -> Result<BlockStatus> {
        self.base.get_json(&format!("/api/block/{hash}/status"))
    }

    /// Transaction ID at index
    ///
    /// Retrieve a single transaction ID at a specific index within a block. Returns plain text txid.
    ///
    /// *[Mempool.space docs](https://mempool.space/docs/api/rest#get-block-transaction-id)*
    ///
    /// Endpoint: `GET /api/block/{hash}/txid/{index}`
    pub fn get_block_txid(&self, hash: BlockHash, index: TxIndex) -> Result<Txid> {
        self.base
            .get_json(&format!("/api/block/{hash}/txid/{index}"))
    }

    /// Block transaction IDs
    ///
    /// Retrieve all transaction IDs in a block. Returns an array of txids in block order.
    ///
    /// *[Mempool.space docs](https://mempool.space/docs/api/rest#get-block-transaction-ids)*
    ///
    /// Endpoint: `GET /api/block/{hash}/txids`
    pub fn get_block_txids(&self, hash: BlockHash) -> Result<Vec<Txid>> {
        self.base.get_json(&format!("/api/block/{hash}/txids"))
    }

    /// Block transactions (paginated)
    ///
    /// Retrieve transactions in a block by block hash, starting from the specified index. Returns up to 25 transactions at a time.
    ///
    /// *[Mempool.space docs](https://mempool.space/docs/api/rest#get-block-transactions)*
    ///
    /// Endpoint: `GET /api/block/{hash}/txs/{start_index}`
    pub fn get_block_txs(&self, hash: BlockHash, start_index: TxIndex) -> Result<Vec<Transaction>> {
        self.base
            .get_json(&format!("/api/block/{hash}/txs/{start_index}"))
    }

    /// Recent blocks
    ///
    /// Retrieve the last 10 blocks. Returns block metadata for each block.
    ///
    /// *[Mempool.space docs](https://mempool.space/docs/api/rest#get-blocks)*
    ///
    /// Endpoint: `GET /api/blocks`
    pub fn get_blocks(&self) -> Result<Vec<BlockInfo>> {
        self.base.get_json(&format!("/api/blocks"))
    }

    /// Blocks from height
    ///
    /// Retrieve up to 10 blocks going backwards from the given height. For example, height=100 returns blocks 100, 99, 98, ..., 91. Height=0 returns only block 0.
    ///
    /// *[Mempool.space docs](https://mempool.space/docs/api/rest#get-blocks)*
    ///
    /// Endpoint: `GET /api/blocks/{height}`
    pub fn get_blocks_from_height(&self, height: Height) -> Result<Vec<BlockInfo>> {
        self.base.get_json(&format!("/api/blocks/{height}"))
    }

    /// Mempool statistics
    ///
    /// Get current mempool statistics including transaction count, total vsize, and total fees.
    ///
    /// *[Mempool.space docs](https://mempool.space/docs/api/rest#get-mempool)*
    ///
    /// Endpoint: `GET /api/mempool/info`
    pub fn get_mempool(&self) -> Result<MempoolInfo> {
        self.base.get_json(&format!("/api/mempool/info"))
    }

    /// Mempool transaction IDs
    ///
    /// Get all transaction IDs currently in the mempool.
    ///
    /// *[Mempool.space docs](https://mempool.space/docs/api/rest#get-mempool-transaction-ids)*
    ///
    /// Endpoint: `GET /api/mempool/txids`
    pub fn get_mempool_txids(&self) -> Result<Vec<Txid>> {
        self.base.get_json(&format!("/api/mempool/txids"))
    }

    /// Get supported indexes for a metric
    ///
    /// Returns the list of indexes supported by the specified metric. For example, `realized_price` might be available on dateindex, weekindex, and monthindex.
    ///
    /// Endpoint: `GET /api/metric/{metric}`
    pub fn get_metric_info(&self, metric: Metric) -> Result<Vec<Index>> {
        self.base.get_json(&format!("/api/metric/{metric}"))
    }

    /// Get metric data
    ///
    /// Fetch data for a specific metric at the given index. Use query parameters to filter by date range and format (json/csv).
    ///
    /// Endpoint: `GET /api/metric/{metric}/{index}`
    pub fn get_metric(
        &self,
        metric: Metric,
        index: Index,
        start: Option<i64>,
        end: Option<i64>,
        limit: Option<&str>,
        format: Option<Format>,
    ) -> Result<FormatResponse<MetricData>> {
        let mut query = Vec::new();
        if let Some(v) = start {
            query.push(format!("start={}", v));
        }
        if let Some(v) = end {
            query.push(format!("end={}", v));
        }
        if let Some(v) = limit {
            query.push(format!("limit={}", v));
        }
        if let Some(v) = format {
            query.push(format!("format={}", v));
        }
        let query_str = if query.is_empty() {
            String::new()
        } else {
            format!("?{}", query.join("&"))
        };
        let path = format!(
            "/api/metric/{metric}/{}{}",
            index.serialize_long(),
            query_str
        );
        if format == Some(Format::CSV) {
            self.base.get_text(&path).map(FormatResponse::Csv)
        } else {
            self.base.get_json(&path).map(FormatResponse::Json)
        }
    }

    /// Metrics catalog
    ///
    /// Returns the complete hierarchical catalog of available metrics organized as a tree structure. Metrics are grouped by categories and subcategories.
    ///
    /// Endpoint: `GET /api/metrics`
    pub fn get_metrics_tree(&self) -> Result<TreeNode> {
        self.base.get_json(&format!("/api/metrics"))
    }

    /// Bulk metric data
    ///
    /// Fetch multiple metrics in a single request. Supports filtering by index and date range. Returns an array of MetricData objects. For a single metric, use `get_metric` instead.
    ///
    /// Endpoint: `GET /api/metrics/bulk`
    pub fn get_metrics(
        &self,
        metrics: Metrics,
        index: Index,
        start: Option<i64>,
        end: Option<i64>,
        limit: Option<&str>,
        format: Option<Format>,
    ) -> Result<FormatResponse<Vec<MetricData>>> {
        let mut query = Vec::new();
        query.push(format!("metrics={}", metrics));
        query.push(format!("index={}", index));
        if let Some(v) = start {
            query.push(format!("start={}", v));
        }
        if let Some(v) = end {
            query.push(format!("end={}", v));
        }
        if let Some(v) = limit {
            query.push(format!("limit={}", v));
        }
        if let Some(v) = format {
            query.push(format!("format={}", v));
        }
        let query_str = if query.is_empty() {
            String::new()
        } else {
            format!("?{}", query.join("&"))
        };
        let path = format!("/api/metrics/bulk{}", query_str);
        if format == Some(Format::CSV) {
            self.base.get_text(&path).map(FormatResponse::Csv)
        } else {
            self.base.get_json(&path).map(FormatResponse::Json)
        }
    }

    /// Metric count
    ///
    /// Returns the number of metrics available per index type.
    ///
    /// Endpoint: `GET /api/metrics/count`
    pub fn get_metrics_count(&self) -> Result<Vec<MetricCount>> {
        self.base.get_json(&format!("/api/metrics/count"))
    }

    /// List available indexes
    ///
    /// Returns all available indexes with their accepted query aliases. Use any alias when querying metrics.
    ///
    /// Endpoint: `GET /api/metrics/indexes`
    pub fn get_indexes(&self) -> Result<Vec<IndexInfo>> {
        self.base.get_json(&format!("/api/metrics/indexes"))
    }

    /// Metrics list
    ///
    /// Paginated flat list of all available metric names. Use `page` query param for pagination.
    ///
    /// Endpoint: `GET /api/metrics/list`
    pub fn list_metrics(&self, page: Option<i64>) -> Result<PaginatedMetrics> {
        let mut query = Vec::new();
        if let Some(v) = page {
            query.push(format!("page={}", v));
        }
        let query_str = if query.is_empty() {
            String::new()
        } else {
            format!("?{}", query.join("&"))
        };
        let path = format!("/api/metrics/list{}", query_str);
        self.base.get_json(&path)
    }

    /// Search metrics
    ///
    /// Fuzzy search for metrics by name. Supports partial matches and typos.
    ///
    /// Endpoint: `GET /api/metrics/search/{metric}`
    pub fn search_metrics(&self, metric: Metric, limit: Option<Limit>) -> Result<Vec<Metric>> {
        let mut query = Vec::new();
        if let Some(v) = limit {
            query.push(format!("limit={}", v));
        }
        let query_str = if query.is_empty() {
            String::new()
        } else {
            format!("?{}", query.join("&"))
        };
        let path = format!("/api/metrics/search/{metric}{}", query_str);
        self.base.get_json(&path)
    }

    /// Disk usage
    ///
    /// Returns the disk space used by BRK and Bitcoin data.
    ///
    /// Endpoint: `GET /api/server/disk`
    pub fn get_disk_usage(&self) -> Result<DiskUsage> {
        self.base.get_json(&format!("/api/server/disk"))
    }

    /// Sync status
    ///
    /// Returns the sync status of the indexer, including indexed height, tip height, blocks behind, and last indexed timestamp.
    ///
    /// Endpoint: `GET /api/server/sync`
    pub fn get_sync_status(&self) -> Result<SyncStatus> {
        self.base.get_json(&format!("/api/server/sync"))
    }

    /// Transaction information
    ///
    /// Retrieve complete transaction data by transaction ID (txid). Returns inputs, outputs, fee, size, and confirmation status.
    ///
    /// *[Mempool.space docs](https://mempool.space/docs/api/rest#get-transaction)*
    ///
    /// Endpoint: `GET /api/tx/{txid}`
    pub fn get_tx(&self, txid: Txid) -> Result<Transaction> {
        self.base.get_json(&format!("/api/tx/{txid}"))
    }

    /// Transaction hex
    ///
    /// Retrieve the raw transaction as a hex-encoded string. Returns the serialized transaction in hexadecimal format.
    ///
    /// *[Mempool.space docs](https://mempool.space/docs/api/rest#get-transaction-hex)*
    ///
    /// Endpoint: `GET /api/tx/{txid}/hex`
    pub fn get_tx_hex(&self, txid: Txid) -> Result<Hex> {
        self.base.get_json(&format!("/api/tx/{txid}/hex"))
    }

    /// Output spend status
    ///
    /// Get the spending status of a transaction output. Returns whether the output has been spent and, if so, the spending transaction details.
    ///
    /// *[Mempool.space docs](https://mempool.space/docs/api/rest#get-transaction-outspend)*
    ///
    /// Endpoint: `GET /api/tx/{txid}/outspend/{vout}`
    pub fn get_tx_outspend(&self, txid: Txid, vout: Vout) -> Result<TxOutspend> {
        self.base
            .get_json(&format!("/api/tx/{txid}/outspend/{vout}"))
    }

    /// All output spend statuses
    ///
    /// Get the spending status of all outputs in a transaction. Returns an array with the spend status for each output.
    ///
    /// *[Mempool.space docs](https://mempool.space/docs/api/rest#get-transaction-outspends)*
    ///
    /// Endpoint: `GET /api/tx/{txid}/outspends`
    pub fn get_tx_outspends(&self, txid: Txid) -> Result<Vec<TxOutspend>> {
        self.base.get_json(&format!("/api/tx/{txid}/outspends"))
    }

    /// Transaction status
    ///
    /// Retrieve the confirmation status of a transaction. Returns whether the transaction is confirmed and, if so, the block height, hash, and timestamp.
    ///
    /// *[Mempool.space docs](https://mempool.space/docs/api/rest#get-transaction-status)*
    ///
    /// Endpoint: `GET /api/tx/{txid}/status`
    pub fn get_tx_status(&self, txid: Txid) -> Result<TxStatus> {
        self.base.get_json(&format!("/api/tx/{txid}/status"))
    }

    /// Difficulty adjustment
    ///
    /// Get current difficulty adjustment information including progress through the current epoch, estimated retarget date, and difficulty change prediction.
    ///
    /// *[Mempool.space docs](https://mempool.space/docs/api/rest#get-difficulty-adjustment)*
    ///
    /// Endpoint: `GET /api/v1/difficulty-adjustment`
    pub fn get_difficulty_adjustment(&self) -> Result<DifficultyAdjustment> {
        self.base
            .get_json(&format!("/api/v1/difficulty-adjustment"))
    }

    /// Projected mempool blocks
    ///
    /// Get projected blocks from the mempool for fee estimation. Each block contains statistics about transactions that would be included if a block were mined now.
    ///
    /// *[Mempool.space docs](https://mempool.space/docs/api/rest#get-mempool-blocks-fees)*
    ///
    /// Endpoint: `GET /api/v1/fees/mempool-blocks`
    pub fn get_mempool_blocks(&self) -> Result<Vec<MempoolBlock>> {
        self.base.get_json(&format!("/api/v1/fees/mempool-blocks"))
    }

    /// Recommended fees
    ///
    /// Get recommended fee rates for different confirmation targets based on current mempool state.
    ///
    /// *[Mempool.space docs](https://mempool.space/docs/api/rest#get-recommended-fees)*
    ///
    /// Endpoint: `GET /api/v1/fees/recommended`
    pub fn get_recommended_fees(&self) -> Result<RecommendedFees> {
        self.base.get_json(&format!("/api/v1/fees/recommended"))
    }

    /// Block fee rates (WIP)
    ///
    /// **Work in progress.** Get block fee rate percentiles (min, 10th, 25th, median, 75th, 90th, max) for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y
    ///
    /// *[Mempool.space docs](https://mempool.space/docs/api/rest#get-block-feerates)*
    ///
    /// Endpoint: `GET /api/v1/mining/blocks/fee-rates/{time_period}`
    pub fn get_block_fee_rates(&self, time_period: TimePeriod) -> Result<serde_json::Value> {
        self.base
            .get_json(&format!("/api/v1/mining/blocks/fee-rates/{time_period}"))
    }

    /// Block fees
    ///
    /// Get average block fees for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y
    ///
    /// *[Mempool.space docs](https://mempool.space/docs/api/rest#get-block-fees)*
    ///
    /// Endpoint: `GET /api/v1/mining/blocks/fees/{time_period}`
    pub fn get_block_fees(&self, time_period: TimePeriod) -> Result<Vec<BlockFeesEntry>> {
        self.base
            .get_json(&format!("/api/v1/mining/blocks/fees/{time_period}"))
    }

    /// Block rewards
    ///
    /// Get average block rewards (coinbase = subsidy + fees) for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y
    ///
    /// *[Mempool.space docs](https://mempool.space/docs/api/rest#get-block-rewards)*
    ///
    /// Endpoint: `GET /api/v1/mining/blocks/rewards/{time_period}`
    pub fn get_block_rewards(&self, time_period: TimePeriod) -> Result<Vec<BlockRewardsEntry>> {
        self.base
            .get_json(&format!("/api/v1/mining/blocks/rewards/{time_period}"))
    }

    /// Block sizes and weights
    ///
    /// Get average block sizes and weights for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y
    ///
    /// *[Mempool.space docs](https://mempool.space/docs/api/rest#get-sizes-weights)*
    ///
    /// Endpoint: `GET /api/v1/mining/blocks/sizes-weights/{time_period}`
    pub fn get_block_sizes_weights(&self, time_period: TimePeriod) -> Result<BlockSizesWeights> {
        self.base.get_json(&format!(
            "/api/v1/mining/blocks/sizes-weights/{time_period}"
        ))
    }

    /// Block by timestamp
    ///
    /// Find the block closest to a given UNIX timestamp.
    ///
    /// *[Mempool.space docs](https://mempool.space/docs/api/rest#get-block-timestamp)*
    ///
    /// Endpoint: `GET /api/v1/mining/blocks/timestamp/{timestamp}`
    pub fn get_block_by_timestamp(&self, timestamp: Timestamp) -> Result<BlockTimestamp> {
        self.base
            .get_json(&format!("/api/v1/mining/blocks/timestamp/{timestamp}"))
    }

    /// Difficulty adjustments (all time)
    ///
    /// Get historical difficulty adjustments including timestamp, block height, difficulty value, and percentage change.
    ///
    /// *[Mempool.space docs](https://mempool.space/docs/api/rest#get-difficulty-adjustments)*
    ///
    /// Endpoint: `GET /api/v1/mining/difficulty-adjustments`
    pub fn get_difficulty_adjustments(&self) -> Result<Vec<DifficultyAdjustmentEntry>> {
        self.base
            .get_json(&format!("/api/v1/mining/difficulty-adjustments"))
    }

    /// Difficulty adjustments
    ///
    /// Get historical difficulty adjustments for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y.
    ///
    /// *[Mempool.space docs](https://mempool.space/docs/api/rest#get-difficulty-adjustments)*
    ///
    /// Endpoint: `GET /api/v1/mining/difficulty-adjustments/{time_period}`
    pub fn get_difficulty_adjustments_by_period(
        &self,
        time_period: TimePeriod,
    ) -> Result<Vec<DifficultyAdjustmentEntry>> {
        self.base.get_json(&format!(
            "/api/v1/mining/difficulty-adjustments/{time_period}"
        ))
    }

    /// Network hashrate (all time)
    ///
    /// Get network hashrate and difficulty data for all time.
    ///
    /// *[Mempool.space docs](https://mempool.space/docs/api/rest#get-hashrate)*
    ///
    /// Endpoint: `GET /api/v1/mining/hashrate`
    pub fn get_hashrate(&self) -> Result<HashrateSummary> {
        self.base.get_json(&format!("/api/v1/mining/hashrate"))
    }

    /// Network hashrate
    ///
    /// Get network hashrate and difficulty data for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y
    ///
    /// *[Mempool.space docs](https://mempool.space/docs/api/rest#get-hashrate)*
    ///
    /// Endpoint: `GET /api/v1/mining/hashrate/{time_period}`
    pub fn get_hashrate_by_period(&self, time_period: TimePeriod) -> Result<HashrateSummary> {
        self.base
            .get_json(&format!("/api/v1/mining/hashrate/{time_period}"))
    }

    /// Mining pool details
    ///
    /// Get detailed information about a specific mining pool including block counts and shares for different time periods.
    ///
    /// *[Mempool.space docs](https://mempool.space/docs/api/rest#get-mining-pool)*
    ///
    /// Endpoint: `GET /api/v1/mining/pool/{slug}`
    pub fn get_pool(&self, slug: PoolSlug) -> Result<PoolDetail> {
        self.base.get_json(&format!("/api/v1/mining/pool/{slug}"))
    }

    /// List all mining pools
    ///
    /// Get list of all known mining pools with their identifiers.
    ///
    /// *[Mempool.space docs](https://mempool.space/docs/api/rest#get-mining-pools)*
    ///
    /// Endpoint: `GET /api/v1/mining/pools`
    pub fn get_pools(&self) -> Result<Vec<PoolInfo>> {
        self.base.get_json(&format!("/api/v1/mining/pools"))
    }

    /// Mining pool statistics
    ///
    /// Get mining pool statistics for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y
    ///
    /// *[Mempool.space docs](https://mempool.space/docs/api/rest#get-mining-pools)*
    ///
    /// Endpoint: `GET /api/v1/mining/pools/{time_period}`
    pub fn get_pool_stats(&self, time_period: TimePeriod) -> Result<PoolsSummary> {
        self.base
            .get_json(&format!("/api/v1/mining/pools/{time_period}"))
    }

    /// Mining reward statistics
    ///
    /// Get mining reward statistics for the last N blocks including total rewards, fees, and transaction count.
    ///
    /// *[Mempool.space docs](https://mempool.space/docs/api/rest#get-reward-stats)*
    ///
    /// Endpoint: `GET /api/v1/mining/reward-stats/{block_count}`
    pub fn get_reward_stats(&self, block_count: i64) -> Result<RewardStats> {
        self.base
            .get_json(&format!("/api/v1/mining/reward-stats/{block_count}"))
    }

    /// Validate address
    ///
    /// Validate a Bitcoin address and get information about its type and scriptPubKey.
    ///
    /// *[Mempool.space docs](https://mempool.space/docs/api/rest#get-address-validate)*
    ///
    /// Endpoint: `GET /api/v1/validate-address/{address}`
    pub fn validate_address(&self, address: &str) -> Result<AddressValidation> {
        self.base
            .get_json(&format!("/api/v1/validate-address/{address}"))
    }

    /// Health check
    ///
    /// Returns the health status of the API server, including uptime information.
    ///
    /// Endpoint: `GET /health`
    pub fn get_health(&self) -> Result<Health> {
        self.base.get_json(&format!("/health"))
    }

    /// OpenAPI specification
    ///
    /// Full OpenAPI 3.1 specification for this API.
    ///
    /// Endpoint: `GET /openapi.json`
    pub fn get_openapi(&self) -> Result<serde_json::Value> {
        self.base.get_json(&format!("/openapi.json"))
    }

    /// API version
    ///
    /// Returns the current version of the API server
    ///
    /// Endpoint: `GET /version`
    pub fn get_version(&self) -> Result<String> {
        self.base.get_json(&format!("/version"))
    }
}
