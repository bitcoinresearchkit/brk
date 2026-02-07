// Auto-generated BRK Rust client
// Do not edit manually

#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(clippy::useless_format)]
#![allow(clippy::unnecessary_to_owned)]

use std::sync::Arc;
use std::ops::{Bound, RangeBounds};
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

    fn get(&self, path: &str) -> Result<minreq::Response> {
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

        Ok(response)
    }

    /// Make a GET request and deserialize JSON response.
    pub fn get_json<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        self.get(path)?
            .json()
            .map_err(|e| BrkError { message: e.to_string() })
    }

    /// Make a GET request and return raw text response.
    pub fn get_text(&self, path: &str) -> Result<String> {
        self.get(path)?
            .as_str()
            .map(|s| s.to_string())
            .map_err(|e| BrkError { message: e.to_string() })
    }
}

/// Build metric name with suffix.
#[inline]
fn _m(acc: &str, s: &str) -> String {
    if s.is_empty() { acc.to_string() }
    else if acc.is_empty() { s.to_string() }
    else { format!("{acc}_{s}") }
}

/// Build metric name with prefix.
#[inline]
fn _p(prefix: &str, acc: &str) -> String {
    if acc.is_empty() { prefix.to_string() } else { format!("{prefix}_{acc}") }
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
        Self { client, name, index, start: None, end: None }
    }

    fn path(&self) -> String {
        format!("/api/metric/{}/{}", self.name, self.index.serialize_long())
    }

    fn build_path(&self, format: Option<&str>) -> String {
        let mut params = Vec::new();
        if let Some(s) = self.start { params.push(format!("start={}", s)); }
        if let Some(e) = self.end { params.push(format!("end={}", e)); }
        if let Some(fmt) = format { params.push(format!("format={}", fmt)); }
        let p = self.path();
        if params.is_empty() { p } else { format!("{}?{}", p, params.join("&")) }
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
        Self { config: EndpointConfig::new(client, name, index), _marker: std::marker::PhantomData }
    }

    /// Select a specific index position.
    pub fn get(mut self, index: usize) -> SingleItemBuilder<T> {
        self.config.start = Some(index as i64);
        self.config.end = Some(index as i64 + 1);
        SingleItemBuilder { config: self.config, _marker: std::marker::PhantomData }
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
        RangeBuilder { config: self.config, _marker: std::marker::PhantomData }
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
        RangeBuilder { config: self.config, _marker: std::marker::PhantomData }
    }

    /// Skip the first n items. Chain with `take(n)` to get a range.
    pub fn skip(mut self, n: usize) -> SkippedBuilder<T> {
        self.config.start = Some(n as i64);
        SkippedBuilder { config: self.config, _marker: std::marker::PhantomData }
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
        RangeBuilder { config: self.config, _marker: std::marker::PhantomData }
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
const _I1: &[Index] = &[Index::DateIndex, Index::DecadeIndex, Index::DifficultyEpoch, Index::Height, Index::MonthIndex, Index::QuarterIndex, Index::SemesterIndex, Index::WeekIndex, Index::YearIndex];
const _I2: &[Index] = &[Index::DateIndex, Index::DecadeIndex, Index::DifficultyEpoch, Index::MonthIndex, Index::QuarterIndex, Index::SemesterIndex, Index::WeekIndex, Index::YearIndex];
const _I3: &[Index] = &[Index::DateIndex, Index::DecadeIndex, Index::Height, Index::MonthIndex, Index::QuarterIndex, Index::SemesterIndex, Index::WeekIndex, Index::YearIndex];
const _I4: &[Index] = &[Index::DateIndex, Index::DecadeIndex, Index::MonthIndex, Index::QuarterIndex, Index::SemesterIndex, Index::WeekIndex, Index::YearIndex];
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
const _I31: &[Index] = &[Index::FundedAddressIndex];
const _I32: &[Index] = &[Index::EmptyAddressIndex];

#[inline]
fn _ep<T: DeserializeOwned>(c: &Arc<BrkClientBase>, n: &Arc<str>, i: Index) -> MetricEndpointBuilder<T> {
    MetricEndpointBuilder::new(c.clone(), n.clone(), i)
}

// Index accessor structs

pub struct MetricPattern1By<T> { client: Arc<BrkClientBase>, name: Arc<str>, _marker: std::marker::PhantomData<T> }
impl<T: DeserializeOwned> MetricPattern1By<T> {
    pub fn dateindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::DateIndex) }
    pub fn decadeindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::DecadeIndex) }
    pub fn difficultyepoch(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::DifficultyEpoch) }
    pub fn height(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::Height) }
    pub fn monthindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::MonthIndex) }
    pub fn quarterindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::QuarterIndex) }
    pub fn semesterindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::SemesterIndex) }
    pub fn weekindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::WeekIndex) }
    pub fn yearindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::YearIndex) }
}

pub struct MetricPattern1<T> { name: Arc<str>, pub by: MetricPattern1By<T> }
impl<T: DeserializeOwned> MetricPattern1<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self { let name: Arc<str> = name.into(); Self { name: name.clone(), by: MetricPattern1By { client, name, _marker: std::marker::PhantomData } } }
    pub fn name(&self) -> &str { &self.name }
}

impl<T> AnyMetricPattern for MetricPattern1<T> { fn name(&self) -> &str { &self.name } fn indexes(&self) -> &'static [Index] { _I1 } }
impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern1<T> { fn get(&self, index: Index) -> Option<MetricEndpointBuilder<T>> { _I1.contains(&index).then(|| _ep(&self.by.client, &self.by.name, index)) } }

pub struct MetricPattern2By<T> { client: Arc<BrkClientBase>, name: Arc<str>, _marker: std::marker::PhantomData<T> }
impl<T: DeserializeOwned> MetricPattern2By<T> {
    pub fn dateindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::DateIndex) }
    pub fn decadeindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::DecadeIndex) }
    pub fn difficultyepoch(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::DifficultyEpoch) }
    pub fn monthindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::MonthIndex) }
    pub fn quarterindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::QuarterIndex) }
    pub fn semesterindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::SemesterIndex) }
    pub fn weekindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::WeekIndex) }
    pub fn yearindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::YearIndex) }
}

pub struct MetricPattern2<T> { name: Arc<str>, pub by: MetricPattern2By<T> }
impl<T: DeserializeOwned> MetricPattern2<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self { let name: Arc<str> = name.into(); Self { name: name.clone(), by: MetricPattern2By { client, name, _marker: std::marker::PhantomData } } }
    pub fn name(&self) -> &str { &self.name }
}

impl<T> AnyMetricPattern for MetricPattern2<T> { fn name(&self) -> &str { &self.name } fn indexes(&self) -> &'static [Index] { _I2 } }
impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern2<T> { fn get(&self, index: Index) -> Option<MetricEndpointBuilder<T>> { _I2.contains(&index).then(|| _ep(&self.by.client, &self.by.name, index)) } }

pub struct MetricPattern3By<T> { client: Arc<BrkClientBase>, name: Arc<str>, _marker: std::marker::PhantomData<T> }
impl<T: DeserializeOwned> MetricPattern3By<T> {
    pub fn dateindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::DateIndex) }
    pub fn decadeindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::DecadeIndex) }
    pub fn height(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::Height) }
    pub fn monthindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::MonthIndex) }
    pub fn quarterindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::QuarterIndex) }
    pub fn semesterindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::SemesterIndex) }
    pub fn weekindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::WeekIndex) }
    pub fn yearindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::YearIndex) }
}

pub struct MetricPattern3<T> { name: Arc<str>, pub by: MetricPattern3By<T> }
impl<T: DeserializeOwned> MetricPattern3<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self { let name: Arc<str> = name.into(); Self { name: name.clone(), by: MetricPattern3By { client, name, _marker: std::marker::PhantomData } } }
    pub fn name(&self) -> &str { &self.name }
}

impl<T> AnyMetricPattern for MetricPattern3<T> { fn name(&self) -> &str { &self.name } fn indexes(&self) -> &'static [Index] { _I3 } }
impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern3<T> { fn get(&self, index: Index) -> Option<MetricEndpointBuilder<T>> { _I3.contains(&index).then(|| _ep(&self.by.client, &self.by.name, index)) } }

pub struct MetricPattern4By<T> { client: Arc<BrkClientBase>, name: Arc<str>, _marker: std::marker::PhantomData<T> }
impl<T: DeserializeOwned> MetricPattern4By<T> {
    pub fn dateindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::DateIndex) }
    pub fn decadeindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::DecadeIndex) }
    pub fn monthindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::MonthIndex) }
    pub fn quarterindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::QuarterIndex) }
    pub fn semesterindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::SemesterIndex) }
    pub fn weekindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::WeekIndex) }
    pub fn yearindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::YearIndex) }
}

pub struct MetricPattern4<T> { name: Arc<str>, pub by: MetricPattern4By<T> }
impl<T: DeserializeOwned> MetricPattern4<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self { let name: Arc<str> = name.into(); Self { name: name.clone(), by: MetricPattern4By { client, name, _marker: std::marker::PhantomData } } }
    pub fn name(&self) -> &str { &self.name }
}

impl<T> AnyMetricPattern for MetricPattern4<T> { fn name(&self) -> &str { &self.name } fn indexes(&self) -> &'static [Index] { _I4 } }
impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern4<T> { fn get(&self, index: Index) -> Option<MetricEndpointBuilder<T>> { _I4.contains(&index).then(|| _ep(&self.by.client, &self.by.name, index)) } }

pub struct MetricPattern5By<T> { client: Arc<BrkClientBase>, name: Arc<str>, _marker: std::marker::PhantomData<T> }
impl<T: DeserializeOwned> MetricPattern5By<T> {
    pub fn dateindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::DateIndex) }
    pub fn height(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::Height) }
}

pub struct MetricPattern5<T> { name: Arc<str>, pub by: MetricPattern5By<T> }
impl<T: DeserializeOwned> MetricPattern5<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self { let name: Arc<str> = name.into(); Self { name: name.clone(), by: MetricPattern5By { client, name, _marker: std::marker::PhantomData } } }
    pub fn name(&self) -> &str { &self.name }
}

impl<T> AnyMetricPattern for MetricPattern5<T> { fn name(&self) -> &str { &self.name } fn indexes(&self) -> &'static [Index] { _I5 } }
impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern5<T> { fn get(&self, index: Index) -> Option<MetricEndpointBuilder<T>> { _I5.contains(&index).then(|| _ep(&self.by.client, &self.by.name, index)) } }

pub struct MetricPattern6By<T> { client: Arc<BrkClientBase>, name: Arc<str>, _marker: std::marker::PhantomData<T> }
impl<T: DeserializeOwned> MetricPattern6By<T> {
    pub fn dateindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::DateIndex) }
}

pub struct MetricPattern6<T> { name: Arc<str>, pub by: MetricPattern6By<T> }
impl<T: DeserializeOwned> MetricPattern6<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self { let name: Arc<str> = name.into(); Self { name: name.clone(), by: MetricPattern6By { client, name, _marker: std::marker::PhantomData } } }
    pub fn name(&self) -> &str { &self.name }
}

impl<T> AnyMetricPattern for MetricPattern6<T> { fn name(&self) -> &str { &self.name } fn indexes(&self) -> &'static [Index] { _I6 } }
impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern6<T> { fn get(&self, index: Index) -> Option<MetricEndpointBuilder<T>> { _I6.contains(&index).then(|| _ep(&self.by.client, &self.by.name, index)) } }

pub struct MetricPattern7By<T> { client: Arc<BrkClientBase>, name: Arc<str>, _marker: std::marker::PhantomData<T> }
impl<T: DeserializeOwned> MetricPattern7By<T> {
    pub fn decadeindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::DecadeIndex) }
}

pub struct MetricPattern7<T> { name: Arc<str>, pub by: MetricPattern7By<T> }
impl<T: DeserializeOwned> MetricPattern7<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self { let name: Arc<str> = name.into(); Self { name: name.clone(), by: MetricPattern7By { client, name, _marker: std::marker::PhantomData } } }
    pub fn name(&self) -> &str { &self.name }
}

impl<T> AnyMetricPattern for MetricPattern7<T> { fn name(&self) -> &str { &self.name } fn indexes(&self) -> &'static [Index] { _I7 } }
impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern7<T> { fn get(&self, index: Index) -> Option<MetricEndpointBuilder<T>> { _I7.contains(&index).then(|| _ep(&self.by.client, &self.by.name, index)) } }

pub struct MetricPattern8By<T> { client: Arc<BrkClientBase>, name: Arc<str>, _marker: std::marker::PhantomData<T> }
impl<T: DeserializeOwned> MetricPattern8By<T> {
    pub fn difficultyepoch(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::DifficultyEpoch) }
}

pub struct MetricPattern8<T> { name: Arc<str>, pub by: MetricPattern8By<T> }
impl<T: DeserializeOwned> MetricPattern8<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self { let name: Arc<str> = name.into(); Self { name: name.clone(), by: MetricPattern8By { client, name, _marker: std::marker::PhantomData } } }
    pub fn name(&self) -> &str { &self.name }
}

impl<T> AnyMetricPattern for MetricPattern8<T> { fn name(&self) -> &str { &self.name } fn indexes(&self) -> &'static [Index] { _I8 } }
impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern8<T> { fn get(&self, index: Index) -> Option<MetricEndpointBuilder<T>> { _I8.contains(&index).then(|| _ep(&self.by.client, &self.by.name, index)) } }

pub struct MetricPattern9By<T> { client: Arc<BrkClientBase>, name: Arc<str>, _marker: std::marker::PhantomData<T> }
impl<T: DeserializeOwned> MetricPattern9By<T> {
    pub fn emptyoutputindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::EmptyOutputIndex) }
}

pub struct MetricPattern9<T> { name: Arc<str>, pub by: MetricPattern9By<T> }
impl<T: DeserializeOwned> MetricPattern9<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self { let name: Arc<str> = name.into(); Self { name: name.clone(), by: MetricPattern9By { client, name, _marker: std::marker::PhantomData } } }
    pub fn name(&self) -> &str { &self.name }
}

impl<T> AnyMetricPattern for MetricPattern9<T> { fn name(&self) -> &str { &self.name } fn indexes(&self) -> &'static [Index] { _I9 } }
impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern9<T> { fn get(&self, index: Index) -> Option<MetricEndpointBuilder<T>> { _I9.contains(&index).then(|| _ep(&self.by.client, &self.by.name, index)) } }

pub struct MetricPattern10By<T> { client: Arc<BrkClientBase>, name: Arc<str>, _marker: std::marker::PhantomData<T> }
impl<T: DeserializeOwned> MetricPattern10By<T> {
    pub fn halvingepoch(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::HalvingEpoch) }
}

pub struct MetricPattern10<T> { name: Arc<str>, pub by: MetricPattern10By<T> }
impl<T: DeserializeOwned> MetricPattern10<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self { let name: Arc<str> = name.into(); Self { name: name.clone(), by: MetricPattern10By { client, name, _marker: std::marker::PhantomData } } }
    pub fn name(&self) -> &str { &self.name }
}

impl<T> AnyMetricPattern for MetricPattern10<T> { fn name(&self) -> &str { &self.name } fn indexes(&self) -> &'static [Index] { _I10 } }
impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern10<T> { fn get(&self, index: Index) -> Option<MetricEndpointBuilder<T>> { _I10.contains(&index).then(|| _ep(&self.by.client, &self.by.name, index)) } }

pub struct MetricPattern11By<T> { client: Arc<BrkClientBase>, name: Arc<str>, _marker: std::marker::PhantomData<T> }
impl<T: DeserializeOwned> MetricPattern11By<T> {
    pub fn height(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::Height) }
}

pub struct MetricPattern11<T> { name: Arc<str>, pub by: MetricPattern11By<T> }
impl<T: DeserializeOwned> MetricPattern11<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self { let name: Arc<str> = name.into(); Self { name: name.clone(), by: MetricPattern11By { client, name, _marker: std::marker::PhantomData } } }
    pub fn name(&self) -> &str { &self.name }
}

impl<T> AnyMetricPattern for MetricPattern11<T> { fn name(&self) -> &str { &self.name } fn indexes(&self) -> &'static [Index] { _I11 } }
impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern11<T> { fn get(&self, index: Index) -> Option<MetricEndpointBuilder<T>> { _I11.contains(&index).then(|| _ep(&self.by.client, &self.by.name, index)) } }

pub struct MetricPattern12By<T> { client: Arc<BrkClientBase>, name: Arc<str>, _marker: std::marker::PhantomData<T> }
impl<T: DeserializeOwned> MetricPattern12By<T> {
    pub fn txinindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::TxInIndex) }
}

pub struct MetricPattern12<T> { name: Arc<str>, pub by: MetricPattern12By<T> }
impl<T: DeserializeOwned> MetricPattern12<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self { let name: Arc<str> = name.into(); Self { name: name.clone(), by: MetricPattern12By { client, name, _marker: std::marker::PhantomData } } }
    pub fn name(&self) -> &str { &self.name }
}

impl<T> AnyMetricPattern for MetricPattern12<T> { fn name(&self) -> &str { &self.name } fn indexes(&self) -> &'static [Index] { _I12 } }
impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern12<T> { fn get(&self, index: Index) -> Option<MetricEndpointBuilder<T>> { _I12.contains(&index).then(|| _ep(&self.by.client, &self.by.name, index)) } }

pub struct MetricPattern13By<T> { client: Arc<BrkClientBase>, name: Arc<str>, _marker: std::marker::PhantomData<T> }
impl<T: DeserializeOwned> MetricPattern13By<T> {
    pub fn monthindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::MonthIndex) }
}

pub struct MetricPattern13<T> { name: Arc<str>, pub by: MetricPattern13By<T> }
impl<T: DeserializeOwned> MetricPattern13<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self { let name: Arc<str> = name.into(); Self { name: name.clone(), by: MetricPattern13By { client, name, _marker: std::marker::PhantomData } } }
    pub fn name(&self) -> &str { &self.name }
}

impl<T> AnyMetricPattern for MetricPattern13<T> { fn name(&self) -> &str { &self.name } fn indexes(&self) -> &'static [Index] { _I13 } }
impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern13<T> { fn get(&self, index: Index) -> Option<MetricEndpointBuilder<T>> { _I13.contains(&index).then(|| _ep(&self.by.client, &self.by.name, index)) } }

pub struct MetricPattern14By<T> { client: Arc<BrkClientBase>, name: Arc<str>, _marker: std::marker::PhantomData<T> }
impl<T: DeserializeOwned> MetricPattern14By<T> {
    pub fn opreturnindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::OpReturnIndex) }
}

pub struct MetricPattern14<T> { name: Arc<str>, pub by: MetricPattern14By<T> }
impl<T: DeserializeOwned> MetricPattern14<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self { let name: Arc<str> = name.into(); Self { name: name.clone(), by: MetricPattern14By { client, name, _marker: std::marker::PhantomData } } }
    pub fn name(&self) -> &str { &self.name }
}

impl<T> AnyMetricPattern for MetricPattern14<T> { fn name(&self) -> &str { &self.name } fn indexes(&self) -> &'static [Index] { _I14 } }
impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern14<T> { fn get(&self, index: Index) -> Option<MetricEndpointBuilder<T>> { _I14.contains(&index).then(|| _ep(&self.by.client, &self.by.name, index)) } }

pub struct MetricPattern15By<T> { client: Arc<BrkClientBase>, name: Arc<str>, _marker: std::marker::PhantomData<T> }
impl<T: DeserializeOwned> MetricPattern15By<T> {
    pub fn txoutindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::TxOutIndex) }
}

pub struct MetricPattern15<T> { name: Arc<str>, pub by: MetricPattern15By<T> }
impl<T: DeserializeOwned> MetricPattern15<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self { let name: Arc<str> = name.into(); Self { name: name.clone(), by: MetricPattern15By { client, name, _marker: std::marker::PhantomData } } }
    pub fn name(&self) -> &str { &self.name }
}

impl<T> AnyMetricPattern for MetricPattern15<T> { fn name(&self) -> &str { &self.name } fn indexes(&self) -> &'static [Index] { _I15 } }
impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern15<T> { fn get(&self, index: Index) -> Option<MetricEndpointBuilder<T>> { _I15.contains(&index).then(|| _ep(&self.by.client, &self.by.name, index)) } }

pub struct MetricPattern16By<T> { client: Arc<BrkClientBase>, name: Arc<str>, _marker: std::marker::PhantomData<T> }
impl<T: DeserializeOwned> MetricPattern16By<T> {
    pub fn p2aaddressindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::P2AAddressIndex) }
}

pub struct MetricPattern16<T> { name: Arc<str>, pub by: MetricPattern16By<T> }
impl<T: DeserializeOwned> MetricPattern16<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self { let name: Arc<str> = name.into(); Self { name: name.clone(), by: MetricPattern16By { client, name, _marker: std::marker::PhantomData } } }
    pub fn name(&self) -> &str { &self.name }
}

impl<T> AnyMetricPattern for MetricPattern16<T> { fn name(&self) -> &str { &self.name } fn indexes(&self) -> &'static [Index] { _I16 } }
impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern16<T> { fn get(&self, index: Index) -> Option<MetricEndpointBuilder<T>> { _I16.contains(&index).then(|| _ep(&self.by.client, &self.by.name, index)) } }

pub struct MetricPattern17By<T> { client: Arc<BrkClientBase>, name: Arc<str>, _marker: std::marker::PhantomData<T> }
impl<T: DeserializeOwned> MetricPattern17By<T> {
    pub fn p2msoutputindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::P2MSOutputIndex) }
}

pub struct MetricPattern17<T> { name: Arc<str>, pub by: MetricPattern17By<T> }
impl<T: DeserializeOwned> MetricPattern17<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self { let name: Arc<str> = name.into(); Self { name: name.clone(), by: MetricPattern17By { client, name, _marker: std::marker::PhantomData } } }
    pub fn name(&self) -> &str { &self.name }
}

impl<T> AnyMetricPattern for MetricPattern17<T> { fn name(&self) -> &str { &self.name } fn indexes(&self) -> &'static [Index] { _I17 } }
impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern17<T> { fn get(&self, index: Index) -> Option<MetricEndpointBuilder<T>> { _I17.contains(&index).then(|| _ep(&self.by.client, &self.by.name, index)) } }

pub struct MetricPattern18By<T> { client: Arc<BrkClientBase>, name: Arc<str>, _marker: std::marker::PhantomData<T> }
impl<T: DeserializeOwned> MetricPattern18By<T> {
    pub fn p2pk33addressindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::P2PK33AddressIndex) }
}

pub struct MetricPattern18<T> { name: Arc<str>, pub by: MetricPattern18By<T> }
impl<T: DeserializeOwned> MetricPattern18<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self { let name: Arc<str> = name.into(); Self { name: name.clone(), by: MetricPattern18By { client, name, _marker: std::marker::PhantomData } } }
    pub fn name(&self) -> &str { &self.name }
}

impl<T> AnyMetricPattern for MetricPattern18<T> { fn name(&self) -> &str { &self.name } fn indexes(&self) -> &'static [Index] { _I18 } }
impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern18<T> { fn get(&self, index: Index) -> Option<MetricEndpointBuilder<T>> { _I18.contains(&index).then(|| _ep(&self.by.client, &self.by.name, index)) } }

pub struct MetricPattern19By<T> { client: Arc<BrkClientBase>, name: Arc<str>, _marker: std::marker::PhantomData<T> }
impl<T: DeserializeOwned> MetricPattern19By<T> {
    pub fn p2pk65addressindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::P2PK65AddressIndex) }
}

pub struct MetricPattern19<T> { name: Arc<str>, pub by: MetricPattern19By<T> }
impl<T: DeserializeOwned> MetricPattern19<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self { let name: Arc<str> = name.into(); Self { name: name.clone(), by: MetricPattern19By { client, name, _marker: std::marker::PhantomData } } }
    pub fn name(&self) -> &str { &self.name }
}

impl<T> AnyMetricPattern for MetricPattern19<T> { fn name(&self) -> &str { &self.name } fn indexes(&self) -> &'static [Index] { _I19 } }
impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern19<T> { fn get(&self, index: Index) -> Option<MetricEndpointBuilder<T>> { _I19.contains(&index).then(|| _ep(&self.by.client, &self.by.name, index)) } }

pub struct MetricPattern20By<T> { client: Arc<BrkClientBase>, name: Arc<str>, _marker: std::marker::PhantomData<T> }
impl<T: DeserializeOwned> MetricPattern20By<T> {
    pub fn p2pkhaddressindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::P2PKHAddressIndex) }
}

pub struct MetricPattern20<T> { name: Arc<str>, pub by: MetricPattern20By<T> }
impl<T: DeserializeOwned> MetricPattern20<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self { let name: Arc<str> = name.into(); Self { name: name.clone(), by: MetricPattern20By { client, name, _marker: std::marker::PhantomData } } }
    pub fn name(&self) -> &str { &self.name }
}

impl<T> AnyMetricPattern for MetricPattern20<T> { fn name(&self) -> &str { &self.name } fn indexes(&self) -> &'static [Index] { _I20 } }
impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern20<T> { fn get(&self, index: Index) -> Option<MetricEndpointBuilder<T>> { _I20.contains(&index).then(|| _ep(&self.by.client, &self.by.name, index)) } }

pub struct MetricPattern21By<T> { client: Arc<BrkClientBase>, name: Arc<str>, _marker: std::marker::PhantomData<T> }
impl<T: DeserializeOwned> MetricPattern21By<T> {
    pub fn p2shaddressindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::P2SHAddressIndex) }
}

pub struct MetricPattern21<T> { name: Arc<str>, pub by: MetricPattern21By<T> }
impl<T: DeserializeOwned> MetricPattern21<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self { let name: Arc<str> = name.into(); Self { name: name.clone(), by: MetricPattern21By { client, name, _marker: std::marker::PhantomData } } }
    pub fn name(&self) -> &str { &self.name }
}

impl<T> AnyMetricPattern for MetricPattern21<T> { fn name(&self) -> &str { &self.name } fn indexes(&self) -> &'static [Index] { _I21 } }
impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern21<T> { fn get(&self, index: Index) -> Option<MetricEndpointBuilder<T>> { _I21.contains(&index).then(|| _ep(&self.by.client, &self.by.name, index)) } }

pub struct MetricPattern22By<T> { client: Arc<BrkClientBase>, name: Arc<str>, _marker: std::marker::PhantomData<T> }
impl<T: DeserializeOwned> MetricPattern22By<T> {
    pub fn p2traddressindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::P2TRAddressIndex) }
}

pub struct MetricPattern22<T> { name: Arc<str>, pub by: MetricPattern22By<T> }
impl<T: DeserializeOwned> MetricPattern22<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self { let name: Arc<str> = name.into(); Self { name: name.clone(), by: MetricPattern22By { client, name, _marker: std::marker::PhantomData } } }
    pub fn name(&self) -> &str { &self.name }
}

impl<T> AnyMetricPattern for MetricPattern22<T> { fn name(&self) -> &str { &self.name } fn indexes(&self) -> &'static [Index] { _I22 } }
impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern22<T> { fn get(&self, index: Index) -> Option<MetricEndpointBuilder<T>> { _I22.contains(&index).then(|| _ep(&self.by.client, &self.by.name, index)) } }

pub struct MetricPattern23By<T> { client: Arc<BrkClientBase>, name: Arc<str>, _marker: std::marker::PhantomData<T> }
impl<T: DeserializeOwned> MetricPattern23By<T> {
    pub fn p2wpkhaddressindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::P2WPKHAddressIndex) }
}

pub struct MetricPattern23<T> { name: Arc<str>, pub by: MetricPattern23By<T> }
impl<T: DeserializeOwned> MetricPattern23<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self { let name: Arc<str> = name.into(); Self { name: name.clone(), by: MetricPattern23By { client, name, _marker: std::marker::PhantomData } } }
    pub fn name(&self) -> &str { &self.name }
}

impl<T> AnyMetricPattern for MetricPattern23<T> { fn name(&self) -> &str { &self.name } fn indexes(&self) -> &'static [Index] { _I23 } }
impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern23<T> { fn get(&self, index: Index) -> Option<MetricEndpointBuilder<T>> { _I23.contains(&index).then(|| _ep(&self.by.client, &self.by.name, index)) } }

pub struct MetricPattern24By<T> { client: Arc<BrkClientBase>, name: Arc<str>, _marker: std::marker::PhantomData<T> }
impl<T: DeserializeOwned> MetricPattern24By<T> {
    pub fn p2wshaddressindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::P2WSHAddressIndex) }
}

pub struct MetricPattern24<T> { name: Arc<str>, pub by: MetricPattern24By<T> }
impl<T: DeserializeOwned> MetricPattern24<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self { let name: Arc<str> = name.into(); Self { name: name.clone(), by: MetricPattern24By { client, name, _marker: std::marker::PhantomData } } }
    pub fn name(&self) -> &str { &self.name }
}

impl<T> AnyMetricPattern for MetricPattern24<T> { fn name(&self) -> &str { &self.name } fn indexes(&self) -> &'static [Index] { _I24 } }
impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern24<T> { fn get(&self, index: Index) -> Option<MetricEndpointBuilder<T>> { _I24.contains(&index).then(|| _ep(&self.by.client, &self.by.name, index)) } }

pub struct MetricPattern25By<T> { client: Arc<BrkClientBase>, name: Arc<str>, _marker: std::marker::PhantomData<T> }
impl<T: DeserializeOwned> MetricPattern25By<T> {
    pub fn quarterindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::QuarterIndex) }
}

pub struct MetricPattern25<T> { name: Arc<str>, pub by: MetricPattern25By<T> }
impl<T: DeserializeOwned> MetricPattern25<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self { let name: Arc<str> = name.into(); Self { name: name.clone(), by: MetricPattern25By { client, name, _marker: std::marker::PhantomData } } }
    pub fn name(&self) -> &str { &self.name }
}

impl<T> AnyMetricPattern for MetricPattern25<T> { fn name(&self) -> &str { &self.name } fn indexes(&self) -> &'static [Index] { _I25 } }
impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern25<T> { fn get(&self, index: Index) -> Option<MetricEndpointBuilder<T>> { _I25.contains(&index).then(|| _ep(&self.by.client, &self.by.name, index)) } }

pub struct MetricPattern26By<T> { client: Arc<BrkClientBase>, name: Arc<str>, _marker: std::marker::PhantomData<T> }
impl<T: DeserializeOwned> MetricPattern26By<T> {
    pub fn semesterindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::SemesterIndex) }
}

pub struct MetricPattern26<T> { name: Arc<str>, pub by: MetricPattern26By<T> }
impl<T: DeserializeOwned> MetricPattern26<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self { let name: Arc<str> = name.into(); Self { name: name.clone(), by: MetricPattern26By { client, name, _marker: std::marker::PhantomData } } }
    pub fn name(&self) -> &str { &self.name }
}

impl<T> AnyMetricPattern for MetricPattern26<T> { fn name(&self) -> &str { &self.name } fn indexes(&self) -> &'static [Index] { _I26 } }
impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern26<T> { fn get(&self, index: Index) -> Option<MetricEndpointBuilder<T>> { _I26.contains(&index).then(|| _ep(&self.by.client, &self.by.name, index)) } }

pub struct MetricPattern27By<T> { client: Arc<BrkClientBase>, name: Arc<str>, _marker: std::marker::PhantomData<T> }
impl<T: DeserializeOwned> MetricPattern27By<T> {
    pub fn txindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::TxIndex) }
}

pub struct MetricPattern27<T> { name: Arc<str>, pub by: MetricPattern27By<T> }
impl<T: DeserializeOwned> MetricPattern27<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self { let name: Arc<str> = name.into(); Self { name: name.clone(), by: MetricPattern27By { client, name, _marker: std::marker::PhantomData } } }
    pub fn name(&self) -> &str { &self.name }
}

impl<T> AnyMetricPattern for MetricPattern27<T> { fn name(&self) -> &str { &self.name } fn indexes(&self) -> &'static [Index] { _I27 } }
impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern27<T> { fn get(&self, index: Index) -> Option<MetricEndpointBuilder<T>> { _I27.contains(&index).then(|| _ep(&self.by.client, &self.by.name, index)) } }

pub struct MetricPattern28By<T> { client: Arc<BrkClientBase>, name: Arc<str>, _marker: std::marker::PhantomData<T> }
impl<T: DeserializeOwned> MetricPattern28By<T> {
    pub fn unknownoutputindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::UnknownOutputIndex) }
}

pub struct MetricPattern28<T> { name: Arc<str>, pub by: MetricPattern28By<T> }
impl<T: DeserializeOwned> MetricPattern28<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self { let name: Arc<str> = name.into(); Self { name: name.clone(), by: MetricPattern28By { client, name, _marker: std::marker::PhantomData } } }
    pub fn name(&self) -> &str { &self.name }
}

impl<T> AnyMetricPattern for MetricPattern28<T> { fn name(&self) -> &str { &self.name } fn indexes(&self) -> &'static [Index] { _I28 } }
impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern28<T> { fn get(&self, index: Index) -> Option<MetricEndpointBuilder<T>> { _I28.contains(&index).then(|| _ep(&self.by.client, &self.by.name, index)) } }

pub struct MetricPattern29By<T> { client: Arc<BrkClientBase>, name: Arc<str>, _marker: std::marker::PhantomData<T> }
impl<T: DeserializeOwned> MetricPattern29By<T> {
    pub fn weekindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::WeekIndex) }
}

pub struct MetricPattern29<T> { name: Arc<str>, pub by: MetricPattern29By<T> }
impl<T: DeserializeOwned> MetricPattern29<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self { let name: Arc<str> = name.into(); Self { name: name.clone(), by: MetricPattern29By { client, name, _marker: std::marker::PhantomData } } }
    pub fn name(&self) -> &str { &self.name }
}

impl<T> AnyMetricPattern for MetricPattern29<T> { fn name(&self) -> &str { &self.name } fn indexes(&self) -> &'static [Index] { _I29 } }
impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern29<T> { fn get(&self, index: Index) -> Option<MetricEndpointBuilder<T>> { _I29.contains(&index).then(|| _ep(&self.by.client, &self.by.name, index)) } }

pub struct MetricPattern30By<T> { client: Arc<BrkClientBase>, name: Arc<str>, _marker: std::marker::PhantomData<T> }
impl<T: DeserializeOwned> MetricPattern30By<T> {
    pub fn yearindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::YearIndex) }
}

pub struct MetricPattern30<T> { name: Arc<str>, pub by: MetricPattern30By<T> }
impl<T: DeserializeOwned> MetricPattern30<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self { let name: Arc<str> = name.into(); Self { name: name.clone(), by: MetricPattern30By { client, name, _marker: std::marker::PhantomData } } }
    pub fn name(&self) -> &str { &self.name }
}

impl<T> AnyMetricPattern for MetricPattern30<T> { fn name(&self) -> &str { &self.name } fn indexes(&self) -> &'static [Index] { _I30 } }
impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern30<T> { fn get(&self, index: Index) -> Option<MetricEndpointBuilder<T>> { _I30.contains(&index).then(|| _ep(&self.by.client, &self.by.name, index)) } }

pub struct MetricPattern31By<T> { client: Arc<BrkClientBase>, name: Arc<str>, _marker: std::marker::PhantomData<T> }
impl<T: DeserializeOwned> MetricPattern31By<T> {
    pub fn fundedaddressindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::FundedAddressIndex) }
}

pub struct MetricPattern31<T> { name: Arc<str>, pub by: MetricPattern31By<T> }
impl<T: DeserializeOwned> MetricPattern31<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self { let name: Arc<str> = name.into(); Self { name: name.clone(), by: MetricPattern31By { client, name, _marker: std::marker::PhantomData } } }
    pub fn name(&self) -> &str { &self.name }
}

impl<T> AnyMetricPattern for MetricPattern31<T> { fn name(&self) -> &str { &self.name } fn indexes(&self) -> &'static [Index] { _I31 } }
impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern31<T> { fn get(&self, index: Index) -> Option<MetricEndpointBuilder<T>> { _I31.contains(&index).then(|| _ep(&self.by.client, &self.by.name, index)) } }

pub struct MetricPattern32By<T> { client: Arc<BrkClientBase>, name: Arc<str>, _marker: std::marker::PhantomData<T> }
impl<T: DeserializeOwned> MetricPattern32By<T> {
    pub fn emptyaddressindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::EmptyAddressIndex) }
}

pub struct MetricPattern32<T> { name: Arc<str>, pub by: MetricPattern32By<T> }
impl<T: DeserializeOwned> MetricPattern32<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self { let name: Arc<str> = name.into(); Self { name: name.clone(), by: MetricPattern32By { client, name, _marker: std::marker::PhantomData } } }
    pub fn name(&self) -> &str { &self.name }
}

impl<T> AnyMetricPattern for MetricPattern32<T> { fn name(&self) -> &str { &self.name } fn indexes(&self) -> &'static [Index] { _I32 } }
impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern32<T> { fn get(&self, index: Index) -> Option<MetricEndpointBuilder<T>> { _I32.contains(&index).then(|| _ep(&self.by.client, &self.by.name, index)) } }

// Reusable pattern structs

/// Pattern struct for repeated tree structure.
pub struct AdjustedCapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern {
    pub adjusted_sopr: MetricPattern6<StoredF64>,
    pub adjusted_sopr_30d_ema: MetricPattern6<StoredF64>,
    pub adjusted_sopr_7d_ema: MetricPattern6<StoredF64>,
    pub adjusted_value_created: MetricPattern1<Dollars>,
    pub adjusted_value_destroyed: MetricPattern1<Dollars>,
    pub cap_raw: MetricPattern11<CentsSats>,
    pub capitulation_flow: MetricPattern1<Dollars>,
    pub investor_cap_raw: MetricPattern11<CentsSquaredSats>,
    pub investor_price: DollarsSatsPattern,
    pub investor_price_cents: MetricPattern1<CentsUnsigned>,
    pub investor_price_extra: RatioPattern,
    pub loss_value_created: MetricPattern1<Dollars>,
    pub loss_value_destroyed: MetricPattern1<Dollars>,
    pub lower_price_band: DollarsSatsPattern,
    pub mvrv: MetricPattern4<StoredF32>,
    pub neg_realized_loss: CumulativeSumPattern2<Dollars>,
    pub net_realized_pnl: CumulativeSumPattern<Dollars>,
    pub net_realized_pnl_7d_ema: MetricPattern4<Dollars>,
    pub net_realized_pnl_cumulative_30d_delta: MetricPattern4<Dollars>,
    pub net_realized_pnl_cumulative_30d_delta_rel_to_market_cap: MetricPattern4<StoredF32>,
    pub net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap: MetricPattern4<StoredF32>,
    pub net_realized_pnl_rel_to_realized_cap: CumulativeSumPattern<StoredF32>,
    pub peak_regret: CumulativeSumPattern<Dollars>,
    pub peak_regret_rel_to_realized_cap: MetricPattern1<StoredF32>,
    pub profit_flow: MetricPattern1<Dollars>,
    pub profit_value_created: MetricPattern1<Dollars>,
    pub profit_value_destroyed: MetricPattern1<Dollars>,
    pub realized_cap: MetricPattern1<Dollars>,
    pub realized_cap_30d_delta: MetricPattern4<Dollars>,
    pub realized_cap_cents: MetricPattern1<CentsUnsigned>,
    pub realized_cap_rel_to_own_market_cap: MetricPattern1<StoredF32>,
    pub realized_loss: CumulativeSumPattern<Dollars>,
    pub realized_loss_7d_ema: MetricPattern4<Dollars>,
    pub realized_loss_rel_to_realized_cap: CumulativeSumPattern<StoredF32>,
    pub realized_price: DollarsSatsPattern,
    pub realized_price_extra: RatioPattern,
    pub realized_profit: CumulativeSumPattern<Dollars>,
    pub realized_profit_7d_ema: MetricPattern4<Dollars>,
    pub realized_profit_rel_to_realized_cap: CumulativeSumPattern<StoredF32>,
    pub realized_profit_to_loss_ratio: MetricPattern6<StoredF64>,
    pub realized_value: MetricPattern1<Dollars>,
    pub sell_side_risk_ratio: MetricPattern6<StoredF32>,
    pub sell_side_risk_ratio_30d_ema: MetricPattern6<StoredF32>,
    pub sell_side_risk_ratio_7d_ema: MetricPattern6<StoredF32>,
    pub sent_in_loss: BitcoinDollarsSatsPattern3,
    pub sent_in_loss_14d_ema: BitcoinDollarsSatsPattern5,
    pub sent_in_profit: BitcoinDollarsSatsPattern3,
    pub sent_in_profit_14d_ema: BitcoinDollarsSatsPattern5,
    pub sopr: MetricPattern6<StoredF64>,
    pub sopr_30d_ema: MetricPattern6<StoredF64>,
    pub sopr_7d_ema: MetricPattern6<StoredF64>,
    pub total_realized_pnl: MetricPattern1<Dollars>,
    pub upper_price_band: DollarsSatsPattern,
    pub value_created: MetricPattern1<Dollars>,
    pub value_destroyed: MetricPattern1<Dollars>,
}

impl AdjustedCapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            adjusted_sopr: MetricPattern6::new(client.clone(), _m(&acc, "adjusted_sopr")),
            adjusted_sopr_30d_ema: MetricPattern6::new(client.clone(), _m(&acc, "adjusted_sopr_30d_ema")),
            adjusted_sopr_7d_ema: MetricPattern6::new(client.clone(), _m(&acc, "adjusted_sopr_7d_ema")),
            adjusted_value_created: MetricPattern1::new(client.clone(), _m(&acc, "adjusted_value_created")),
            adjusted_value_destroyed: MetricPattern1::new(client.clone(), _m(&acc, "adjusted_value_destroyed")),
            cap_raw: MetricPattern11::new(client.clone(), _m(&acc, "cap_raw")),
            capitulation_flow: MetricPattern1::new(client.clone(), _m(&acc, "capitulation_flow")),
            investor_cap_raw: MetricPattern11::new(client.clone(), _m(&acc, "investor_cap_raw")),
            investor_price: DollarsSatsPattern::new(client.clone(), _m(&acc, "investor_price")),
            investor_price_cents: MetricPattern1::new(client.clone(), _m(&acc, "investor_price_cents")),
            investor_price_extra: RatioPattern::new(client.clone(), _m(&acc, "investor_price_ratio")),
            loss_value_created: MetricPattern1::new(client.clone(), _m(&acc, "loss_value_created")),
            loss_value_destroyed: MetricPattern1::new(client.clone(), _m(&acc, "loss_value_destroyed")),
            lower_price_band: DollarsSatsPattern::new(client.clone(), _m(&acc, "lower_price_band")),
            mvrv: MetricPattern4::new(client.clone(), _m(&acc, "mvrv")),
            neg_realized_loss: CumulativeSumPattern2::new(client.clone(), _m(&acc, "neg_realized_loss")),
            net_realized_pnl: CumulativeSumPattern::new(client.clone(), _m(&acc, "net_realized_pnl")),
            net_realized_pnl_7d_ema: MetricPattern4::new(client.clone(), _m(&acc, "net_realized_pnl_7d_ema")),
            net_realized_pnl_cumulative_30d_delta: MetricPattern4::new(client.clone(), _m(&acc, "net_realized_pnl_cumulative_30d_delta")),
            net_realized_pnl_cumulative_30d_delta_rel_to_market_cap: MetricPattern4::new(client.clone(), _m(&acc, "net_realized_pnl_cumulative_30d_delta_rel_to_market_cap")),
            net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap: MetricPattern4::new(client.clone(), _m(&acc, "net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap")),
            net_realized_pnl_rel_to_realized_cap: CumulativeSumPattern::new(client.clone(), _m(&acc, "net_realized_pnl_rel_to_realized_cap")),
            peak_regret: CumulativeSumPattern::new(client.clone(), _m(&acc, "realized_peak_regret")),
            peak_regret_rel_to_realized_cap: MetricPattern1::new(client.clone(), _m(&acc, "peak_regret_rel_to_realized_cap")),
            profit_flow: MetricPattern1::new(client.clone(), _m(&acc, "profit_flow")),
            profit_value_created: MetricPattern1::new(client.clone(), _m(&acc, "profit_value_created")),
            profit_value_destroyed: MetricPattern1::new(client.clone(), _m(&acc, "profit_value_destroyed")),
            realized_cap: MetricPattern1::new(client.clone(), _m(&acc, "realized_cap")),
            realized_cap_30d_delta: MetricPattern4::new(client.clone(), _m(&acc, "realized_cap_30d_delta")),
            realized_cap_cents: MetricPattern1::new(client.clone(), _m(&acc, "realized_cap_cents")),
            realized_cap_rel_to_own_market_cap: MetricPattern1::new(client.clone(), _m(&acc, "realized_cap_rel_to_own_market_cap")),
            realized_loss: CumulativeSumPattern::new(client.clone(), _m(&acc, "realized_loss")),
            realized_loss_7d_ema: MetricPattern4::new(client.clone(), _m(&acc, "realized_loss_7d_ema")),
            realized_loss_rel_to_realized_cap: CumulativeSumPattern::new(client.clone(), _m(&acc, "realized_loss_rel_to_realized_cap")),
            realized_price: DollarsSatsPattern::new(client.clone(), _m(&acc, "realized_price")),
            realized_price_extra: RatioPattern::new(client.clone(), _m(&acc, "realized_price_ratio")),
            realized_profit: CumulativeSumPattern::new(client.clone(), _m(&acc, "realized_profit")),
            realized_profit_7d_ema: MetricPattern4::new(client.clone(), _m(&acc, "realized_profit_7d_ema")),
            realized_profit_rel_to_realized_cap: CumulativeSumPattern::new(client.clone(), _m(&acc, "realized_profit_rel_to_realized_cap")),
            realized_profit_to_loss_ratio: MetricPattern6::new(client.clone(), _m(&acc, "realized_profit_to_loss_ratio")),
            realized_value: MetricPattern1::new(client.clone(), _m(&acc, "realized_value")),
            sell_side_risk_ratio: MetricPattern6::new(client.clone(), _m(&acc, "sell_side_risk_ratio")),
            sell_side_risk_ratio_30d_ema: MetricPattern6::new(client.clone(), _m(&acc, "sell_side_risk_ratio_30d_ema")),
            sell_side_risk_ratio_7d_ema: MetricPattern6::new(client.clone(), _m(&acc, "sell_side_risk_ratio_7d_ema")),
            sent_in_loss: BitcoinDollarsSatsPattern3::new(client.clone(), _m(&acc, "sent_in_loss")),
            sent_in_loss_14d_ema: BitcoinDollarsSatsPattern5::new(client.clone(), _m(&acc, "sent_in_loss_14d_ema")),
            sent_in_profit: BitcoinDollarsSatsPattern3::new(client.clone(), _m(&acc, "sent_in_profit")),
            sent_in_profit_14d_ema: BitcoinDollarsSatsPattern5::new(client.clone(), _m(&acc, "sent_in_profit_14d_ema")),
            sopr: MetricPattern6::new(client.clone(), _m(&acc, "sopr")),
            sopr_30d_ema: MetricPattern6::new(client.clone(), _m(&acc, "sopr_30d_ema")),
            sopr_7d_ema: MetricPattern6::new(client.clone(), _m(&acc, "sopr_7d_ema")),
            total_realized_pnl: MetricPattern1::new(client.clone(), _m(&acc, "total_realized_pnl")),
            upper_price_band: DollarsSatsPattern::new(client.clone(), _m(&acc, "upper_price_band")),
            value_created: MetricPattern1::new(client.clone(), _m(&acc, "value_created")),
            value_destroyed: MetricPattern1::new(client.clone(), _m(&acc, "value_destroyed")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct AdjustedCapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern2 {
    pub adjusted_sopr: MetricPattern6<StoredF64>,
    pub adjusted_sopr_30d_ema: MetricPattern6<StoredF64>,
    pub adjusted_sopr_7d_ema: MetricPattern6<StoredF64>,
    pub adjusted_value_created: MetricPattern1<Dollars>,
    pub adjusted_value_destroyed: MetricPattern1<Dollars>,
    pub cap_raw: MetricPattern11<CentsSats>,
    pub capitulation_flow: MetricPattern1<Dollars>,
    pub investor_cap_raw: MetricPattern11<CentsSquaredSats>,
    pub investor_price: DollarsSatsPattern,
    pub investor_price_cents: MetricPattern1<CentsUnsigned>,
    pub investor_price_extra: RatioPattern2,
    pub loss_value_created: MetricPattern1<Dollars>,
    pub loss_value_destroyed: MetricPattern1<Dollars>,
    pub lower_price_band: DollarsSatsPattern,
    pub mvrv: MetricPattern4<StoredF32>,
    pub neg_realized_loss: CumulativeSumPattern2<Dollars>,
    pub net_realized_pnl: CumulativeSumPattern<Dollars>,
    pub net_realized_pnl_7d_ema: MetricPattern4<Dollars>,
    pub net_realized_pnl_cumulative_30d_delta: MetricPattern4<Dollars>,
    pub net_realized_pnl_cumulative_30d_delta_rel_to_market_cap: MetricPattern4<StoredF32>,
    pub net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap: MetricPattern4<StoredF32>,
    pub net_realized_pnl_rel_to_realized_cap: CumulativeSumPattern<StoredF32>,
    pub peak_regret: CumulativeSumPattern<Dollars>,
    pub peak_regret_rel_to_realized_cap: MetricPattern1<StoredF32>,
    pub profit_flow: MetricPattern1<Dollars>,
    pub profit_value_created: MetricPattern1<Dollars>,
    pub profit_value_destroyed: MetricPattern1<Dollars>,
    pub realized_cap: MetricPattern1<Dollars>,
    pub realized_cap_30d_delta: MetricPattern4<Dollars>,
    pub realized_cap_cents: MetricPattern1<CentsUnsigned>,
    pub realized_loss: CumulativeSumPattern<Dollars>,
    pub realized_loss_7d_ema: MetricPattern4<Dollars>,
    pub realized_loss_rel_to_realized_cap: CumulativeSumPattern<StoredF32>,
    pub realized_price: DollarsSatsPattern,
    pub realized_price_extra: RatioPattern2,
    pub realized_profit: CumulativeSumPattern<Dollars>,
    pub realized_profit_7d_ema: MetricPattern4<Dollars>,
    pub realized_profit_rel_to_realized_cap: CumulativeSumPattern<StoredF32>,
    pub realized_value: MetricPattern1<Dollars>,
    pub sell_side_risk_ratio: MetricPattern6<StoredF32>,
    pub sell_side_risk_ratio_30d_ema: MetricPattern6<StoredF32>,
    pub sell_side_risk_ratio_7d_ema: MetricPattern6<StoredF32>,
    pub sent_in_loss: BitcoinDollarsSatsPattern3,
    pub sent_in_loss_14d_ema: BitcoinDollarsSatsPattern5,
    pub sent_in_profit: BitcoinDollarsSatsPattern3,
    pub sent_in_profit_14d_ema: BitcoinDollarsSatsPattern5,
    pub sopr: MetricPattern6<StoredF64>,
    pub sopr_30d_ema: MetricPattern6<StoredF64>,
    pub sopr_7d_ema: MetricPattern6<StoredF64>,
    pub total_realized_pnl: MetricPattern1<Dollars>,
    pub upper_price_band: DollarsSatsPattern,
    pub value_created: MetricPattern1<Dollars>,
    pub value_destroyed: MetricPattern1<Dollars>,
}

impl AdjustedCapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern2 {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            adjusted_sopr: MetricPattern6::new(client.clone(), _m(&acc, "adjusted_sopr")),
            adjusted_sopr_30d_ema: MetricPattern6::new(client.clone(), _m(&acc, "adjusted_sopr_30d_ema")),
            adjusted_sopr_7d_ema: MetricPattern6::new(client.clone(), _m(&acc, "adjusted_sopr_7d_ema")),
            adjusted_value_created: MetricPattern1::new(client.clone(), _m(&acc, "adjusted_value_created")),
            adjusted_value_destroyed: MetricPattern1::new(client.clone(), _m(&acc, "adjusted_value_destroyed")),
            cap_raw: MetricPattern11::new(client.clone(), _m(&acc, "cap_raw")),
            capitulation_flow: MetricPattern1::new(client.clone(), _m(&acc, "capitulation_flow")),
            investor_cap_raw: MetricPattern11::new(client.clone(), _m(&acc, "investor_cap_raw")),
            investor_price: DollarsSatsPattern::new(client.clone(), _m(&acc, "investor_price")),
            investor_price_cents: MetricPattern1::new(client.clone(), _m(&acc, "investor_price_cents")),
            investor_price_extra: RatioPattern2::new(client.clone(), _m(&acc, "investor_price_ratio")),
            loss_value_created: MetricPattern1::new(client.clone(), _m(&acc, "loss_value_created")),
            loss_value_destroyed: MetricPattern1::new(client.clone(), _m(&acc, "loss_value_destroyed")),
            lower_price_band: DollarsSatsPattern::new(client.clone(), _m(&acc, "lower_price_band")),
            mvrv: MetricPattern4::new(client.clone(), _m(&acc, "mvrv")),
            neg_realized_loss: CumulativeSumPattern2::new(client.clone(), _m(&acc, "neg_realized_loss")),
            net_realized_pnl: CumulativeSumPattern::new(client.clone(), _m(&acc, "net_realized_pnl")),
            net_realized_pnl_7d_ema: MetricPattern4::new(client.clone(), _m(&acc, "net_realized_pnl_7d_ema")),
            net_realized_pnl_cumulative_30d_delta: MetricPattern4::new(client.clone(), _m(&acc, "net_realized_pnl_cumulative_30d_delta")),
            net_realized_pnl_cumulative_30d_delta_rel_to_market_cap: MetricPattern4::new(client.clone(), _m(&acc, "net_realized_pnl_cumulative_30d_delta_rel_to_market_cap")),
            net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap: MetricPattern4::new(client.clone(), _m(&acc, "net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap")),
            net_realized_pnl_rel_to_realized_cap: CumulativeSumPattern::new(client.clone(), _m(&acc, "net_realized_pnl_rel_to_realized_cap")),
            peak_regret: CumulativeSumPattern::new(client.clone(), _m(&acc, "realized_peak_regret")),
            peak_regret_rel_to_realized_cap: MetricPattern1::new(client.clone(), _m(&acc, "peak_regret_rel_to_realized_cap")),
            profit_flow: MetricPattern1::new(client.clone(), _m(&acc, "profit_flow")),
            profit_value_created: MetricPattern1::new(client.clone(), _m(&acc, "profit_value_created")),
            profit_value_destroyed: MetricPattern1::new(client.clone(), _m(&acc, "profit_value_destroyed")),
            realized_cap: MetricPattern1::new(client.clone(), _m(&acc, "realized_cap")),
            realized_cap_30d_delta: MetricPattern4::new(client.clone(), _m(&acc, "realized_cap_30d_delta")),
            realized_cap_cents: MetricPattern1::new(client.clone(), _m(&acc, "realized_cap_cents")),
            realized_loss: CumulativeSumPattern::new(client.clone(), _m(&acc, "realized_loss")),
            realized_loss_7d_ema: MetricPattern4::new(client.clone(), _m(&acc, "realized_loss_7d_ema")),
            realized_loss_rel_to_realized_cap: CumulativeSumPattern::new(client.clone(), _m(&acc, "realized_loss_rel_to_realized_cap")),
            realized_price: DollarsSatsPattern::new(client.clone(), _m(&acc, "realized_price")),
            realized_price_extra: RatioPattern2::new(client.clone(), _m(&acc, "realized_price_ratio")),
            realized_profit: CumulativeSumPattern::new(client.clone(), _m(&acc, "realized_profit")),
            realized_profit_7d_ema: MetricPattern4::new(client.clone(), _m(&acc, "realized_profit_7d_ema")),
            realized_profit_rel_to_realized_cap: CumulativeSumPattern::new(client.clone(), _m(&acc, "realized_profit_rel_to_realized_cap")),
            realized_value: MetricPattern1::new(client.clone(), _m(&acc, "realized_value")),
            sell_side_risk_ratio: MetricPattern6::new(client.clone(), _m(&acc, "sell_side_risk_ratio")),
            sell_side_risk_ratio_30d_ema: MetricPattern6::new(client.clone(), _m(&acc, "sell_side_risk_ratio_30d_ema")),
            sell_side_risk_ratio_7d_ema: MetricPattern6::new(client.clone(), _m(&acc, "sell_side_risk_ratio_7d_ema")),
            sent_in_loss: BitcoinDollarsSatsPattern3::new(client.clone(), _m(&acc, "sent_in_loss")),
            sent_in_loss_14d_ema: BitcoinDollarsSatsPattern5::new(client.clone(), _m(&acc, "sent_in_loss_14d_ema")),
            sent_in_profit: BitcoinDollarsSatsPattern3::new(client.clone(), _m(&acc, "sent_in_profit")),
            sent_in_profit_14d_ema: BitcoinDollarsSatsPattern5::new(client.clone(), _m(&acc, "sent_in_profit_14d_ema")),
            sopr: MetricPattern6::new(client.clone(), _m(&acc, "sopr")),
            sopr_30d_ema: MetricPattern6::new(client.clone(), _m(&acc, "sopr_30d_ema")),
            sopr_7d_ema: MetricPattern6::new(client.clone(), _m(&acc, "sopr_7d_ema")),
            total_realized_pnl: MetricPattern1::new(client.clone(), _m(&acc, "total_realized_pnl")),
            upper_price_band: DollarsSatsPattern::new(client.clone(), _m(&acc, "upper_price_band")),
            value_created: MetricPattern1::new(client.clone(), _m(&acc, "value_created")),
            value_destroyed: MetricPattern1::new(client.clone(), _m(&acc, "value_destroyed")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct CapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern2 {
    pub cap_raw: MetricPattern11<CentsSats>,
    pub capitulation_flow: MetricPattern1<Dollars>,
    pub investor_cap_raw: MetricPattern11<CentsSquaredSats>,
    pub investor_price: DollarsSatsPattern,
    pub investor_price_cents: MetricPattern1<CentsUnsigned>,
    pub investor_price_extra: RatioPattern,
    pub loss_value_created: MetricPattern1<Dollars>,
    pub loss_value_destroyed: MetricPattern1<Dollars>,
    pub lower_price_band: DollarsSatsPattern,
    pub mvrv: MetricPattern4<StoredF32>,
    pub neg_realized_loss: CumulativeSumPattern2<Dollars>,
    pub net_realized_pnl: CumulativeSumPattern<Dollars>,
    pub net_realized_pnl_7d_ema: MetricPattern4<Dollars>,
    pub net_realized_pnl_cumulative_30d_delta: MetricPattern4<Dollars>,
    pub net_realized_pnl_cumulative_30d_delta_rel_to_market_cap: MetricPattern4<StoredF32>,
    pub net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap: MetricPattern4<StoredF32>,
    pub net_realized_pnl_rel_to_realized_cap: CumulativeSumPattern<StoredF32>,
    pub peak_regret: CumulativeSumPattern<Dollars>,
    pub peak_regret_rel_to_realized_cap: MetricPattern1<StoredF32>,
    pub profit_flow: MetricPattern1<Dollars>,
    pub profit_value_created: MetricPattern1<Dollars>,
    pub profit_value_destroyed: MetricPattern1<Dollars>,
    pub realized_cap: MetricPattern1<Dollars>,
    pub realized_cap_30d_delta: MetricPattern4<Dollars>,
    pub realized_cap_cents: MetricPattern1<CentsUnsigned>,
    pub realized_cap_rel_to_own_market_cap: MetricPattern1<StoredF32>,
    pub realized_loss: CumulativeSumPattern<Dollars>,
    pub realized_loss_7d_ema: MetricPattern4<Dollars>,
    pub realized_loss_rel_to_realized_cap: CumulativeSumPattern<StoredF32>,
    pub realized_price: DollarsSatsPattern,
    pub realized_price_extra: RatioPattern,
    pub realized_profit: CumulativeSumPattern<Dollars>,
    pub realized_profit_7d_ema: MetricPattern4<Dollars>,
    pub realized_profit_rel_to_realized_cap: CumulativeSumPattern<StoredF32>,
    pub realized_profit_to_loss_ratio: MetricPattern6<StoredF64>,
    pub realized_value: MetricPattern1<Dollars>,
    pub sell_side_risk_ratio: MetricPattern6<StoredF32>,
    pub sell_side_risk_ratio_30d_ema: MetricPattern6<StoredF32>,
    pub sell_side_risk_ratio_7d_ema: MetricPattern6<StoredF32>,
    pub sent_in_loss: BitcoinDollarsSatsPattern3,
    pub sent_in_loss_14d_ema: BitcoinDollarsSatsPattern5,
    pub sent_in_profit: BitcoinDollarsSatsPattern3,
    pub sent_in_profit_14d_ema: BitcoinDollarsSatsPattern5,
    pub sopr: MetricPattern6<StoredF64>,
    pub sopr_30d_ema: MetricPattern6<StoredF64>,
    pub sopr_7d_ema: MetricPattern6<StoredF64>,
    pub total_realized_pnl: MetricPattern1<Dollars>,
    pub upper_price_band: DollarsSatsPattern,
    pub value_created: MetricPattern1<Dollars>,
    pub value_destroyed: MetricPattern1<Dollars>,
}

impl CapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern2 {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            cap_raw: MetricPattern11::new(client.clone(), _m(&acc, "cap_raw")),
            capitulation_flow: MetricPattern1::new(client.clone(), _m(&acc, "capitulation_flow")),
            investor_cap_raw: MetricPattern11::new(client.clone(), _m(&acc, "investor_cap_raw")),
            investor_price: DollarsSatsPattern::new(client.clone(), _m(&acc, "investor_price")),
            investor_price_cents: MetricPattern1::new(client.clone(), _m(&acc, "investor_price_cents")),
            investor_price_extra: RatioPattern::new(client.clone(), _m(&acc, "investor_price_ratio")),
            loss_value_created: MetricPattern1::new(client.clone(), _m(&acc, "loss_value_created")),
            loss_value_destroyed: MetricPattern1::new(client.clone(), _m(&acc, "loss_value_destroyed")),
            lower_price_band: DollarsSatsPattern::new(client.clone(), _m(&acc, "lower_price_band")),
            mvrv: MetricPattern4::new(client.clone(), _m(&acc, "mvrv")),
            neg_realized_loss: CumulativeSumPattern2::new(client.clone(), _m(&acc, "neg_realized_loss")),
            net_realized_pnl: CumulativeSumPattern::new(client.clone(), _m(&acc, "net_realized_pnl")),
            net_realized_pnl_7d_ema: MetricPattern4::new(client.clone(), _m(&acc, "net_realized_pnl_7d_ema")),
            net_realized_pnl_cumulative_30d_delta: MetricPattern4::new(client.clone(), _m(&acc, "net_realized_pnl_cumulative_30d_delta")),
            net_realized_pnl_cumulative_30d_delta_rel_to_market_cap: MetricPattern4::new(client.clone(), _m(&acc, "net_realized_pnl_cumulative_30d_delta_rel_to_market_cap")),
            net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap: MetricPattern4::new(client.clone(), _m(&acc, "net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap")),
            net_realized_pnl_rel_to_realized_cap: CumulativeSumPattern::new(client.clone(), _m(&acc, "net_realized_pnl_rel_to_realized_cap")),
            peak_regret: CumulativeSumPattern::new(client.clone(), _m(&acc, "realized_peak_regret")),
            peak_regret_rel_to_realized_cap: MetricPattern1::new(client.clone(), _m(&acc, "peak_regret_rel_to_realized_cap")),
            profit_flow: MetricPattern1::new(client.clone(), _m(&acc, "profit_flow")),
            profit_value_created: MetricPattern1::new(client.clone(), _m(&acc, "profit_value_created")),
            profit_value_destroyed: MetricPattern1::new(client.clone(), _m(&acc, "profit_value_destroyed")),
            realized_cap: MetricPattern1::new(client.clone(), _m(&acc, "realized_cap")),
            realized_cap_30d_delta: MetricPattern4::new(client.clone(), _m(&acc, "realized_cap_30d_delta")),
            realized_cap_cents: MetricPattern1::new(client.clone(), _m(&acc, "realized_cap_cents")),
            realized_cap_rel_to_own_market_cap: MetricPattern1::new(client.clone(), _m(&acc, "realized_cap_rel_to_own_market_cap")),
            realized_loss: CumulativeSumPattern::new(client.clone(), _m(&acc, "realized_loss")),
            realized_loss_7d_ema: MetricPattern4::new(client.clone(), _m(&acc, "realized_loss_7d_ema")),
            realized_loss_rel_to_realized_cap: CumulativeSumPattern::new(client.clone(), _m(&acc, "realized_loss_rel_to_realized_cap")),
            realized_price: DollarsSatsPattern::new(client.clone(), _m(&acc, "realized_price")),
            realized_price_extra: RatioPattern::new(client.clone(), _m(&acc, "realized_price_ratio")),
            realized_profit: CumulativeSumPattern::new(client.clone(), _m(&acc, "realized_profit")),
            realized_profit_7d_ema: MetricPattern4::new(client.clone(), _m(&acc, "realized_profit_7d_ema")),
            realized_profit_rel_to_realized_cap: CumulativeSumPattern::new(client.clone(), _m(&acc, "realized_profit_rel_to_realized_cap")),
            realized_profit_to_loss_ratio: MetricPattern6::new(client.clone(), _m(&acc, "realized_profit_to_loss_ratio")),
            realized_value: MetricPattern1::new(client.clone(), _m(&acc, "realized_value")),
            sell_side_risk_ratio: MetricPattern6::new(client.clone(), _m(&acc, "sell_side_risk_ratio")),
            sell_side_risk_ratio_30d_ema: MetricPattern6::new(client.clone(), _m(&acc, "sell_side_risk_ratio_30d_ema")),
            sell_side_risk_ratio_7d_ema: MetricPattern6::new(client.clone(), _m(&acc, "sell_side_risk_ratio_7d_ema")),
            sent_in_loss: BitcoinDollarsSatsPattern3::new(client.clone(), _m(&acc, "sent_in_loss")),
            sent_in_loss_14d_ema: BitcoinDollarsSatsPattern5::new(client.clone(), _m(&acc, "sent_in_loss_14d_ema")),
            sent_in_profit: BitcoinDollarsSatsPattern3::new(client.clone(), _m(&acc, "sent_in_profit")),
            sent_in_profit_14d_ema: BitcoinDollarsSatsPattern5::new(client.clone(), _m(&acc, "sent_in_profit_14d_ema")),
            sopr: MetricPattern6::new(client.clone(), _m(&acc, "sopr")),
            sopr_30d_ema: MetricPattern6::new(client.clone(), _m(&acc, "sopr_30d_ema")),
            sopr_7d_ema: MetricPattern6::new(client.clone(), _m(&acc, "sopr_7d_ema")),
            total_realized_pnl: MetricPattern1::new(client.clone(), _m(&acc, "total_realized_pnl")),
            upper_price_band: DollarsSatsPattern::new(client.clone(), _m(&acc, "upper_price_band")),
            value_created: MetricPattern1::new(client.clone(), _m(&acc, "value_created")),
            value_destroyed: MetricPattern1::new(client.clone(), _m(&acc, "value_destroyed")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct CapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern {
    pub cap_raw: MetricPattern11<CentsSats>,
    pub capitulation_flow: MetricPattern1<Dollars>,
    pub investor_cap_raw: MetricPattern11<CentsSquaredSats>,
    pub investor_price: DollarsSatsPattern,
    pub investor_price_cents: MetricPattern1<CentsUnsigned>,
    pub investor_price_extra: RatioPattern2,
    pub loss_value_created: MetricPattern1<Dollars>,
    pub loss_value_destroyed: MetricPattern1<Dollars>,
    pub lower_price_band: DollarsSatsPattern,
    pub mvrv: MetricPattern4<StoredF32>,
    pub neg_realized_loss: CumulativeSumPattern2<Dollars>,
    pub net_realized_pnl: CumulativeSumPattern<Dollars>,
    pub net_realized_pnl_7d_ema: MetricPattern4<Dollars>,
    pub net_realized_pnl_cumulative_30d_delta: MetricPattern4<Dollars>,
    pub net_realized_pnl_cumulative_30d_delta_rel_to_market_cap: MetricPattern4<StoredF32>,
    pub net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap: MetricPattern4<StoredF32>,
    pub net_realized_pnl_rel_to_realized_cap: CumulativeSumPattern<StoredF32>,
    pub peak_regret: CumulativeSumPattern<Dollars>,
    pub peak_regret_rel_to_realized_cap: MetricPattern1<StoredF32>,
    pub profit_flow: MetricPattern1<Dollars>,
    pub profit_value_created: MetricPattern1<Dollars>,
    pub profit_value_destroyed: MetricPattern1<Dollars>,
    pub realized_cap: MetricPattern1<Dollars>,
    pub realized_cap_30d_delta: MetricPattern4<Dollars>,
    pub realized_cap_cents: MetricPattern1<CentsUnsigned>,
    pub realized_loss: CumulativeSumPattern<Dollars>,
    pub realized_loss_7d_ema: MetricPattern4<Dollars>,
    pub realized_loss_rel_to_realized_cap: CumulativeSumPattern<StoredF32>,
    pub realized_price: DollarsSatsPattern,
    pub realized_price_extra: RatioPattern2,
    pub realized_profit: CumulativeSumPattern<Dollars>,
    pub realized_profit_7d_ema: MetricPattern4<Dollars>,
    pub realized_profit_rel_to_realized_cap: CumulativeSumPattern<StoredF32>,
    pub realized_value: MetricPattern1<Dollars>,
    pub sell_side_risk_ratio: MetricPattern6<StoredF32>,
    pub sell_side_risk_ratio_30d_ema: MetricPattern6<StoredF32>,
    pub sell_side_risk_ratio_7d_ema: MetricPattern6<StoredF32>,
    pub sent_in_loss: BitcoinDollarsSatsPattern3,
    pub sent_in_loss_14d_ema: BitcoinDollarsSatsPattern5,
    pub sent_in_profit: BitcoinDollarsSatsPattern3,
    pub sent_in_profit_14d_ema: BitcoinDollarsSatsPattern5,
    pub sopr: MetricPattern6<StoredF64>,
    pub sopr_30d_ema: MetricPattern6<StoredF64>,
    pub sopr_7d_ema: MetricPattern6<StoredF64>,
    pub total_realized_pnl: MetricPattern1<Dollars>,
    pub upper_price_band: DollarsSatsPattern,
    pub value_created: MetricPattern1<Dollars>,
    pub value_destroyed: MetricPattern1<Dollars>,
}

impl CapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            cap_raw: MetricPattern11::new(client.clone(), _m(&acc, "cap_raw")),
            capitulation_flow: MetricPattern1::new(client.clone(), _m(&acc, "capitulation_flow")),
            investor_cap_raw: MetricPattern11::new(client.clone(), _m(&acc, "investor_cap_raw")),
            investor_price: DollarsSatsPattern::new(client.clone(), _m(&acc, "investor_price")),
            investor_price_cents: MetricPattern1::new(client.clone(), _m(&acc, "investor_price_cents")),
            investor_price_extra: RatioPattern2::new(client.clone(), _m(&acc, "investor_price_ratio")),
            loss_value_created: MetricPattern1::new(client.clone(), _m(&acc, "loss_value_created")),
            loss_value_destroyed: MetricPattern1::new(client.clone(), _m(&acc, "loss_value_destroyed")),
            lower_price_band: DollarsSatsPattern::new(client.clone(), _m(&acc, "lower_price_band")),
            mvrv: MetricPattern4::new(client.clone(), _m(&acc, "mvrv")),
            neg_realized_loss: CumulativeSumPattern2::new(client.clone(), _m(&acc, "neg_realized_loss")),
            net_realized_pnl: CumulativeSumPattern::new(client.clone(), _m(&acc, "net_realized_pnl")),
            net_realized_pnl_7d_ema: MetricPattern4::new(client.clone(), _m(&acc, "net_realized_pnl_7d_ema")),
            net_realized_pnl_cumulative_30d_delta: MetricPattern4::new(client.clone(), _m(&acc, "net_realized_pnl_cumulative_30d_delta")),
            net_realized_pnl_cumulative_30d_delta_rel_to_market_cap: MetricPattern4::new(client.clone(), _m(&acc, "net_realized_pnl_cumulative_30d_delta_rel_to_market_cap")),
            net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap: MetricPattern4::new(client.clone(), _m(&acc, "net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap")),
            net_realized_pnl_rel_to_realized_cap: CumulativeSumPattern::new(client.clone(), _m(&acc, "net_realized_pnl_rel_to_realized_cap")),
            peak_regret: CumulativeSumPattern::new(client.clone(), _m(&acc, "realized_peak_regret")),
            peak_regret_rel_to_realized_cap: MetricPattern1::new(client.clone(), _m(&acc, "peak_regret_rel_to_realized_cap")),
            profit_flow: MetricPattern1::new(client.clone(), _m(&acc, "profit_flow")),
            profit_value_created: MetricPattern1::new(client.clone(), _m(&acc, "profit_value_created")),
            profit_value_destroyed: MetricPattern1::new(client.clone(), _m(&acc, "profit_value_destroyed")),
            realized_cap: MetricPattern1::new(client.clone(), _m(&acc, "realized_cap")),
            realized_cap_30d_delta: MetricPattern4::new(client.clone(), _m(&acc, "realized_cap_30d_delta")),
            realized_cap_cents: MetricPattern1::new(client.clone(), _m(&acc, "realized_cap_cents")),
            realized_loss: CumulativeSumPattern::new(client.clone(), _m(&acc, "realized_loss")),
            realized_loss_7d_ema: MetricPattern4::new(client.clone(), _m(&acc, "realized_loss_7d_ema")),
            realized_loss_rel_to_realized_cap: CumulativeSumPattern::new(client.clone(), _m(&acc, "realized_loss_rel_to_realized_cap")),
            realized_price: DollarsSatsPattern::new(client.clone(), _m(&acc, "realized_price")),
            realized_price_extra: RatioPattern2::new(client.clone(), _m(&acc, "realized_price_ratio")),
            realized_profit: CumulativeSumPattern::new(client.clone(), _m(&acc, "realized_profit")),
            realized_profit_7d_ema: MetricPattern4::new(client.clone(), _m(&acc, "realized_profit_7d_ema")),
            realized_profit_rel_to_realized_cap: CumulativeSumPattern::new(client.clone(), _m(&acc, "realized_profit_rel_to_realized_cap")),
            realized_value: MetricPattern1::new(client.clone(), _m(&acc, "realized_value")),
            sell_side_risk_ratio: MetricPattern6::new(client.clone(), _m(&acc, "sell_side_risk_ratio")),
            sell_side_risk_ratio_30d_ema: MetricPattern6::new(client.clone(), _m(&acc, "sell_side_risk_ratio_30d_ema")),
            sell_side_risk_ratio_7d_ema: MetricPattern6::new(client.clone(), _m(&acc, "sell_side_risk_ratio_7d_ema")),
            sent_in_loss: BitcoinDollarsSatsPattern3::new(client.clone(), _m(&acc, "sent_in_loss")),
            sent_in_loss_14d_ema: BitcoinDollarsSatsPattern5::new(client.clone(), _m(&acc, "sent_in_loss_14d_ema")),
            sent_in_profit: BitcoinDollarsSatsPattern3::new(client.clone(), _m(&acc, "sent_in_profit")),
            sent_in_profit_14d_ema: BitcoinDollarsSatsPattern5::new(client.clone(), _m(&acc, "sent_in_profit_14d_ema")),
            sopr: MetricPattern6::new(client.clone(), _m(&acc, "sopr")),
            sopr_30d_ema: MetricPattern6::new(client.clone(), _m(&acc, "sopr_30d_ema")),
            sopr_7d_ema: MetricPattern6::new(client.clone(), _m(&acc, "sopr_7d_ema")),
            total_realized_pnl: MetricPattern1::new(client.clone(), _m(&acc, "total_realized_pnl")),
            upper_price_band: DollarsSatsPattern::new(client.clone(), _m(&acc, "upper_price_band")),
            value_created: MetricPattern1::new(client.clone(), _m(&acc, "value_created")),
            value_destroyed: MetricPattern1::new(client.clone(), _m(&acc, "value_destroyed")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern {
    pub _0sd_usd: DollarsSatsPattern2,
    pub m0_5sd: MetricPattern4<StoredF32>,
    pub m0_5sd_usd: DollarsSatsPattern2,
    pub m1_5sd: MetricPattern4<StoredF32>,
    pub m1_5sd_usd: DollarsSatsPattern2,
    pub m1sd: MetricPattern4<StoredF32>,
    pub m1sd_usd: DollarsSatsPattern2,
    pub m2_5sd: MetricPattern4<StoredF32>,
    pub m2_5sd_usd: DollarsSatsPattern2,
    pub m2sd: MetricPattern4<StoredF32>,
    pub m2sd_usd: DollarsSatsPattern2,
    pub m3sd: MetricPattern4<StoredF32>,
    pub m3sd_usd: DollarsSatsPattern2,
    pub p0_5sd: MetricPattern4<StoredF32>,
    pub p0_5sd_usd: DollarsSatsPattern2,
    pub p1_5sd: MetricPattern4<StoredF32>,
    pub p1_5sd_usd: DollarsSatsPattern2,
    pub p1sd: MetricPattern4<StoredF32>,
    pub p1sd_usd: DollarsSatsPattern2,
    pub p2_5sd: MetricPattern4<StoredF32>,
    pub p2_5sd_usd: DollarsSatsPattern2,
    pub p2sd: MetricPattern4<StoredF32>,
    pub p2sd_usd: DollarsSatsPattern2,
    pub p3sd: MetricPattern4<StoredF32>,
    pub p3sd_usd: DollarsSatsPattern2,
    pub sd: MetricPattern4<StoredF32>,
    pub sma: MetricPattern4<StoredF32>,
    pub zscore: MetricPattern4<StoredF32>,
}

impl _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            _0sd_usd: DollarsSatsPattern2::new(client.clone(), _m(&acc, "0sd_usd")),
            m0_5sd: MetricPattern4::new(client.clone(), _m(&acc, "m0_5sd")),
            m0_5sd_usd: DollarsSatsPattern2::new(client.clone(), _m(&acc, "m0_5sd_usd")),
            m1_5sd: MetricPattern4::new(client.clone(), _m(&acc, "m1_5sd")),
            m1_5sd_usd: DollarsSatsPattern2::new(client.clone(), _m(&acc, "m1_5sd_usd")),
            m1sd: MetricPattern4::new(client.clone(), _m(&acc, "m1sd")),
            m1sd_usd: DollarsSatsPattern2::new(client.clone(), _m(&acc, "m1sd_usd")),
            m2_5sd: MetricPattern4::new(client.clone(), _m(&acc, "m2_5sd")),
            m2_5sd_usd: DollarsSatsPattern2::new(client.clone(), _m(&acc, "m2_5sd_usd")),
            m2sd: MetricPattern4::new(client.clone(), _m(&acc, "m2sd")),
            m2sd_usd: DollarsSatsPattern2::new(client.clone(), _m(&acc, "m2sd_usd")),
            m3sd: MetricPattern4::new(client.clone(), _m(&acc, "m3sd")),
            m3sd_usd: DollarsSatsPattern2::new(client.clone(), _m(&acc, "m3sd_usd")),
            p0_5sd: MetricPattern4::new(client.clone(), _m(&acc, "p0_5sd")),
            p0_5sd_usd: DollarsSatsPattern2::new(client.clone(), _m(&acc, "p0_5sd_usd")),
            p1_5sd: MetricPattern4::new(client.clone(), _m(&acc, "p1_5sd")),
            p1_5sd_usd: DollarsSatsPattern2::new(client.clone(), _m(&acc, "p1_5sd_usd")),
            p1sd: MetricPattern4::new(client.clone(), _m(&acc, "p1sd")),
            p1sd_usd: DollarsSatsPattern2::new(client.clone(), _m(&acc, "p1sd_usd")),
            p2_5sd: MetricPattern4::new(client.clone(), _m(&acc, "p2_5sd")),
            p2_5sd_usd: DollarsSatsPattern2::new(client.clone(), _m(&acc, "p2_5sd_usd")),
            p2sd: MetricPattern4::new(client.clone(), _m(&acc, "p2sd")),
            p2sd_usd: DollarsSatsPattern2::new(client.clone(), _m(&acc, "p2sd_usd")),
            p3sd: MetricPattern4::new(client.clone(), _m(&acc, "p3sd")),
            p3sd_usd: DollarsSatsPattern2::new(client.clone(), _m(&acc, "p3sd_usd")),
            sd: MetricPattern4::new(client.clone(), _m(&acc, "sd")),
            sma: MetricPattern4::new(client.clone(), _m(&acc, "sma")),
            zscore: MetricPattern4::new(client.clone(), _m(&acc, "zscore")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct InvestedNegNetNuplSupplyUnrealizedPattern4 {
    pub invested_capital_in_loss_pct: MetricPattern1<StoredF32>,
    pub invested_capital_in_profit_pct: MetricPattern1<StoredF32>,
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
    pub unrealized_peak_regret_rel_to_market_cap: MetricPattern4<StoredF32>,
    pub unrealized_profit_rel_to_market_cap: MetricPattern1<StoredF32>,
    pub unrealized_profit_rel_to_own_market_cap: MetricPattern1<StoredF32>,
    pub unrealized_profit_rel_to_own_total_unrealized_pnl: MetricPattern1<StoredF32>,
}

impl InvestedNegNetNuplSupplyUnrealizedPattern4 {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            invested_capital_in_loss_pct: MetricPattern1::new(client.clone(), _m(&acc, "invested_capital_in_loss_pct")),
            invested_capital_in_profit_pct: MetricPattern1::new(client.clone(), _m(&acc, "invested_capital_in_profit_pct")),
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
            unrealized_peak_regret_rel_to_market_cap: MetricPattern4::new(client.clone(), _m(&acc, "unrealized_peak_regret_rel_to_market_cap")),
            unrealized_profit_rel_to_market_cap: MetricPattern1::new(client.clone(), _m(&acc, "unrealized_profit_rel_to_market_cap")),
            unrealized_profit_rel_to_own_market_cap: MetricPattern1::new(client.clone(), _m(&acc, "unrealized_profit_rel_to_own_market_cap")),
            unrealized_profit_rel_to_own_total_unrealized_pnl: MetricPattern1::new(client.clone(), _m(&acc, "unrealized_profit_rel_to_own_total_unrealized_pnl")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct PriceRatioPattern {
    pub price: DollarsSatsPattern2,
    pub ratio: MetricPattern4<StoredF32>,
    pub ratio_1m_sma: MetricPattern4<StoredF32>,
    pub ratio_1w_sma: MetricPattern4<StoredF32>,
    pub ratio_1y_sd: _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern,
    pub ratio_2y_sd: _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern,
    pub ratio_4y_sd: _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern,
    pub ratio_pct1: MetricPattern4<StoredF32>,
    pub ratio_pct1_usd: DollarsSatsPattern2,
    pub ratio_pct2: MetricPattern4<StoredF32>,
    pub ratio_pct2_usd: DollarsSatsPattern2,
    pub ratio_pct5: MetricPattern4<StoredF32>,
    pub ratio_pct5_usd: DollarsSatsPattern2,
    pub ratio_pct95: MetricPattern4<StoredF32>,
    pub ratio_pct95_usd: DollarsSatsPattern2,
    pub ratio_pct98: MetricPattern4<StoredF32>,
    pub ratio_pct98_usd: DollarsSatsPattern2,
    pub ratio_pct99: MetricPattern4<StoredF32>,
    pub ratio_pct99_usd: DollarsSatsPattern2,
    pub ratio_sd: _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern,
}

impl PriceRatioPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            price: DollarsSatsPattern2::new(client.clone(), acc.clone()),
            ratio: MetricPattern4::new(client.clone(), _m(&acc, "ratio")),
            ratio_1m_sma: MetricPattern4::new(client.clone(), _m(&acc, "ratio_1m_sma")),
            ratio_1w_sma: MetricPattern4::new(client.clone(), _m(&acc, "ratio_1w_sma")),
            ratio_1y_sd: _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern::new(client.clone(), _m(&acc, "ratio_1y")),
            ratio_2y_sd: _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern::new(client.clone(), _m(&acc, "ratio_2y")),
            ratio_4y_sd: _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern::new(client.clone(), _m(&acc, "ratio_4y")),
            ratio_pct1: MetricPattern4::new(client.clone(), _m(&acc, "ratio_pct1")),
            ratio_pct1_usd: DollarsSatsPattern2::new(client.clone(), _m(&acc, "ratio_pct1_usd")),
            ratio_pct2: MetricPattern4::new(client.clone(), _m(&acc, "ratio_pct2")),
            ratio_pct2_usd: DollarsSatsPattern2::new(client.clone(), _m(&acc, "ratio_pct2_usd")),
            ratio_pct5: MetricPattern4::new(client.clone(), _m(&acc, "ratio_pct5")),
            ratio_pct5_usd: DollarsSatsPattern2::new(client.clone(), _m(&acc, "ratio_pct5_usd")),
            ratio_pct95: MetricPattern4::new(client.clone(), _m(&acc, "ratio_pct95")),
            ratio_pct95_usd: DollarsSatsPattern2::new(client.clone(), _m(&acc, "ratio_pct95_usd")),
            ratio_pct98: MetricPattern4::new(client.clone(), _m(&acc, "ratio_pct98")),
            ratio_pct98_usd: DollarsSatsPattern2::new(client.clone(), _m(&acc, "ratio_pct98_usd")),
            ratio_pct99: MetricPattern4::new(client.clone(), _m(&acc, "ratio_pct99")),
            ratio_pct99_usd: DollarsSatsPattern2::new(client.clone(), _m(&acc, "ratio_pct99_usd")),
            ratio_sd: _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern::new(client.clone(), _m(&acc, "ratio")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct Pct05Pct10Pct15Pct20Pct25Pct30Pct35Pct40Pct45Pct50Pct55Pct60Pct65Pct70Pct75Pct80Pct85Pct90Pct95Pattern {
    pub pct05: DollarsSatsPattern2,
    pub pct10: DollarsSatsPattern2,
    pub pct15: DollarsSatsPattern2,
    pub pct20: DollarsSatsPattern2,
    pub pct25: DollarsSatsPattern2,
    pub pct30: DollarsSatsPattern2,
    pub pct35: DollarsSatsPattern2,
    pub pct40: DollarsSatsPattern2,
    pub pct45: DollarsSatsPattern2,
    pub pct50: DollarsSatsPattern2,
    pub pct55: DollarsSatsPattern2,
    pub pct60: DollarsSatsPattern2,
    pub pct65: DollarsSatsPattern2,
    pub pct70: DollarsSatsPattern2,
    pub pct75: DollarsSatsPattern2,
    pub pct80: DollarsSatsPattern2,
    pub pct85: DollarsSatsPattern2,
    pub pct90: DollarsSatsPattern2,
    pub pct95: DollarsSatsPattern2,
}

impl Pct05Pct10Pct15Pct20Pct25Pct30Pct35Pct40Pct45Pct50Pct55Pct60Pct65Pct70Pct75Pct80Pct85Pct90Pct95Pattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            pct05: DollarsSatsPattern2::new(client.clone(), _m(&acc, "pct05")),
            pct10: DollarsSatsPattern2::new(client.clone(), _m(&acc, "pct10")),
            pct15: DollarsSatsPattern2::new(client.clone(), _m(&acc, "pct15")),
            pct20: DollarsSatsPattern2::new(client.clone(), _m(&acc, "pct20")),
            pct25: DollarsSatsPattern2::new(client.clone(), _m(&acc, "pct25")),
            pct30: DollarsSatsPattern2::new(client.clone(), _m(&acc, "pct30")),
            pct35: DollarsSatsPattern2::new(client.clone(), _m(&acc, "pct35")),
            pct40: DollarsSatsPattern2::new(client.clone(), _m(&acc, "pct40")),
            pct45: DollarsSatsPattern2::new(client.clone(), _m(&acc, "pct45")),
            pct50: DollarsSatsPattern2::new(client.clone(), _m(&acc, "pct50")),
            pct55: DollarsSatsPattern2::new(client.clone(), _m(&acc, "pct55")),
            pct60: DollarsSatsPattern2::new(client.clone(), _m(&acc, "pct60")),
            pct65: DollarsSatsPattern2::new(client.clone(), _m(&acc, "pct65")),
            pct70: DollarsSatsPattern2::new(client.clone(), _m(&acc, "pct70")),
            pct75: DollarsSatsPattern2::new(client.clone(), _m(&acc, "pct75")),
            pct80: DollarsSatsPattern2::new(client.clone(), _m(&acc, "pct80")),
            pct85: DollarsSatsPattern2::new(client.clone(), _m(&acc, "pct85")),
            pct90: DollarsSatsPattern2::new(client.clone(), _m(&acc, "pct90")),
            pct95: DollarsSatsPattern2::new(client.clone(), _m(&acc, "pct95")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct RatioPattern {
    pub ratio: MetricPattern4<StoredF32>,
    pub ratio_1m_sma: MetricPattern4<StoredF32>,
    pub ratio_1w_sma: MetricPattern4<StoredF32>,
    pub ratio_1y_sd: _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern,
    pub ratio_2y_sd: _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern,
    pub ratio_4y_sd: _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern,
    pub ratio_pct1: MetricPattern4<StoredF32>,
    pub ratio_pct1_usd: DollarsSatsPattern2,
    pub ratio_pct2: MetricPattern4<StoredF32>,
    pub ratio_pct2_usd: DollarsSatsPattern2,
    pub ratio_pct5: MetricPattern4<StoredF32>,
    pub ratio_pct5_usd: DollarsSatsPattern2,
    pub ratio_pct95: MetricPattern4<StoredF32>,
    pub ratio_pct95_usd: DollarsSatsPattern2,
    pub ratio_pct98: MetricPattern4<StoredF32>,
    pub ratio_pct98_usd: DollarsSatsPattern2,
    pub ratio_pct99: MetricPattern4<StoredF32>,
    pub ratio_pct99_usd: DollarsSatsPattern2,
    pub ratio_sd: _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern,
}

impl RatioPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            ratio: MetricPattern4::new(client.clone(), acc.clone()),
            ratio_1m_sma: MetricPattern4::new(client.clone(), _m(&acc, "1m_sma")),
            ratio_1w_sma: MetricPattern4::new(client.clone(), _m(&acc, "1w_sma")),
            ratio_1y_sd: _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern::new(client.clone(), _m(&acc, "1y")),
            ratio_2y_sd: _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern::new(client.clone(), _m(&acc, "2y")),
            ratio_4y_sd: _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern::new(client.clone(), _m(&acc, "4y")),
            ratio_pct1: MetricPattern4::new(client.clone(), _m(&acc, "pct1")),
            ratio_pct1_usd: DollarsSatsPattern2::new(client.clone(), _m(&acc, "pct1_usd")),
            ratio_pct2: MetricPattern4::new(client.clone(), _m(&acc, "pct2")),
            ratio_pct2_usd: DollarsSatsPattern2::new(client.clone(), _m(&acc, "pct2_usd")),
            ratio_pct5: MetricPattern4::new(client.clone(), _m(&acc, "pct5")),
            ratio_pct5_usd: DollarsSatsPattern2::new(client.clone(), _m(&acc, "pct5_usd")),
            ratio_pct95: MetricPattern4::new(client.clone(), _m(&acc, "pct95")),
            ratio_pct95_usd: DollarsSatsPattern2::new(client.clone(), _m(&acc, "pct95_usd")),
            ratio_pct98: MetricPattern4::new(client.clone(), _m(&acc, "pct98")),
            ratio_pct98_usd: DollarsSatsPattern2::new(client.clone(), _m(&acc, "pct98_usd")),
            ratio_pct99: MetricPattern4::new(client.clone(), _m(&acc, "pct99")),
            ratio_pct99_usd: DollarsSatsPattern2::new(client.clone(), _m(&acc, "pct99_usd")),
            ratio_sd: _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern::new(client.clone(), acc.clone()),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct GreedInvestedInvestorNegNetPainPeakSupplyTotalUnrealizedPattern {
    pub greed_index: MetricPattern1<Dollars>,
    pub invested_capital_in_loss: MetricPattern1<Dollars>,
    pub invested_capital_in_loss_raw: MetricPattern11<CentsSats>,
    pub invested_capital_in_profit: MetricPattern1<Dollars>,
    pub invested_capital_in_profit_raw: MetricPattern11<CentsSats>,
    pub investor_cap_in_loss_raw: MetricPattern11<CentsSquaredSats>,
    pub investor_cap_in_profit_raw: MetricPattern11<CentsSquaredSats>,
    pub neg_unrealized_loss: MetricPattern1<Dollars>,
    pub net_sentiment: MetricPattern1<Dollars>,
    pub net_unrealized_pnl: MetricPattern1<Dollars>,
    pub pain_index: MetricPattern1<Dollars>,
    pub peak_regret: MetricPattern4<Dollars>,
    pub supply_in_loss: BitcoinDollarsSatsPattern4,
    pub supply_in_profit: BitcoinDollarsSatsPattern4,
    pub total_unrealized_pnl: MetricPattern1<Dollars>,
    pub unrealized_loss: MetricPattern1<Dollars>,
    pub unrealized_profit: MetricPattern1<Dollars>,
}

impl GreedInvestedInvestorNegNetPainPeakSupplyTotalUnrealizedPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            greed_index: MetricPattern1::new(client.clone(), _m(&acc, "greed_index")),
            invested_capital_in_loss: MetricPattern1::new(client.clone(), _m(&acc, "invested_capital_in_loss")),
            invested_capital_in_loss_raw: MetricPattern11::new(client.clone(), _m(&acc, "invested_capital_in_loss_raw")),
            invested_capital_in_profit: MetricPattern1::new(client.clone(), _m(&acc, "invested_capital_in_profit")),
            invested_capital_in_profit_raw: MetricPattern11::new(client.clone(), _m(&acc, "invested_capital_in_profit_raw")),
            investor_cap_in_loss_raw: MetricPattern11::new(client.clone(), _m(&acc, "investor_cap_in_loss_raw")),
            investor_cap_in_profit_raw: MetricPattern11::new(client.clone(), _m(&acc, "investor_cap_in_profit_raw")),
            neg_unrealized_loss: MetricPattern1::new(client.clone(), _m(&acc, "neg_unrealized_loss")),
            net_sentiment: MetricPattern1::new(client.clone(), _m(&acc, "net_sentiment")),
            net_unrealized_pnl: MetricPattern1::new(client.clone(), _m(&acc, "net_unrealized_pnl")),
            pain_index: MetricPattern1::new(client.clone(), _m(&acc, "pain_index")),
            peak_regret: MetricPattern4::new(client.clone(), _m(&acc, "unrealized_peak_regret")),
            supply_in_loss: BitcoinDollarsSatsPattern4::new(client.clone(), _m(&acc, "supply_in_loss")),
            supply_in_profit: BitcoinDollarsSatsPattern4::new(client.clone(), _m(&acc, "supply_in_profit")),
            total_unrealized_pnl: MetricPattern1::new(client.clone(), _m(&acc, "total_unrealized_pnl")),
            unrealized_loss: MetricPattern1::new(client.clone(), _m(&acc, "unrealized_loss")),
            unrealized_profit: MetricPattern1::new(client.clone(), _m(&acc, "unrealized_profit")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct GreedInvestedInvestorNegNetPainSupplyTotalUnrealizedPattern {
    pub greed_index: MetricPattern1<Dollars>,
    pub invested_capital_in_loss: MetricPattern1<Dollars>,
    pub invested_capital_in_loss_raw: MetricPattern11<CentsSats>,
    pub invested_capital_in_profit: MetricPattern1<Dollars>,
    pub invested_capital_in_profit_raw: MetricPattern11<CentsSats>,
    pub investor_cap_in_loss_raw: MetricPattern11<CentsSquaredSats>,
    pub investor_cap_in_profit_raw: MetricPattern11<CentsSquaredSats>,
    pub neg_unrealized_loss: MetricPattern1<Dollars>,
    pub net_sentiment: MetricPattern1<Dollars>,
    pub net_unrealized_pnl: MetricPattern1<Dollars>,
    pub pain_index: MetricPattern1<Dollars>,
    pub supply_in_loss: BitcoinDollarsSatsPattern4,
    pub supply_in_profit: BitcoinDollarsSatsPattern4,
    pub total_unrealized_pnl: MetricPattern1<Dollars>,
    pub unrealized_loss: MetricPattern1<Dollars>,
    pub unrealized_profit: MetricPattern1<Dollars>,
}

impl GreedInvestedInvestorNegNetPainSupplyTotalUnrealizedPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            greed_index: MetricPattern1::new(client.clone(), _m(&acc, "greed_index")),
            invested_capital_in_loss: MetricPattern1::new(client.clone(), _m(&acc, "invested_capital_in_loss")),
            invested_capital_in_loss_raw: MetricPattern11::new(client.clone(), _m(&acc, "invested_capital_in_loss_raw")),
            invested_capital_in_profit: MetricPattern1::new(client.clone(), _m(&acc, "invested_capital_in_profit")),
            invested_capital_in_profit_raw: MetricPattern11::new(client.clone(), _m(&acc, "invested_capital_in_profit_raw")),
            investor_cap_in_loss_raw: MetricPattern11::new(client.clone(), _m(&acc, "investor_cap_in_loss_raw")),
            investor_cap_in_profit_raw: MetricPattern11::new(client.clone(), _m(&acc, "investor_cap_in_profit_raw")),
            neg_unrealized_loss: MetricPattern1::new(client.clone(), _m(&acc, "neg_unrealized_loss")),
            net_sentiment: MetricPattern1::new(client.clone(), _m(&acc, "net_sentiment")),
            net_unrealized_pnl: MetricPattern1::new(client.clone(), _m(&acc, "net_unrealized_pnl")),
            pain_index: MetricPattern1::new(client.clone(), _m(&acc, "pain_index")),
            supply_in_loss: BitcoinDollarsSatsPattern4::new(client.clone(), _m(&acc, "supply_in_loss")),
            supply_in_profit: BitcoinDollarsSatsPattern4::new(client.clone(), _m(&acc, "supply_in_profit")),
            total_unrealized_pnl: MetricPattern1::new(client.clone(), _m(&acc, "total_unrealized_pnl")),
            unrealized_loss: MetricPattern1::new(client.clone(), _m(&acc, "unrealized_loss")),
            unrealized_profit: MetricPattern1::new(client.clone(), _m(&acc, "unrealized_profit")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern {
    pub _1m_blocks_mined: MetricPattern1<StoredU32>,
    pub _1m_dominance: MetricPattern1<StoredF32>,
    pub _1w_blocks_mined: MetricPattern1<StoredU32>,
    pub _1w_dominance: MetricPattern1<StoredF32>,
    pub _1y_blocks_mined: MetricPattern1<StoredU32>,
    pub _1y_dominance: MetricPattern1<StoredF32>,
    pub _24h_blocks_mined: MetricPattern1<StoredU32>,
    pub _24h_dominance: MetricPattern1<StoredF32>,
    pub blocks_mined: CumulativeSumPattern<StoredU32>,
    pub blocks_since_block: MetricPattern1<StoredU32>,
    pub coinbase: BitcoinDollarsSatsPattern6,
    pub days_since_block: MetricPattern4<StoredU16>,
    pub dominance: MetricPattern1<StoredF32>,
    pub fee: BitcoinDollarsSatsPattern3,
    pub subsidy: BitcoinDollarsSatsPattern3,
}

impl _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern {
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
            blocks_mined: CumulativeSumPattern::new(client.clone(), _m(&acc, "blocks_mined")),
            blocks_since_block: MetricPattern1::new(client.clone(), _m(&acc, "blocks_since_block")),
            coinbase: BitcoinDollarsSatsPattern6::new(client.clone(), _m(&acc, "coinbase")),
            days_since_block: MetricPattern4::new(client.clone(), _m(&acc, "days_since_block")),
            dominance: MetricPattern1::new(client.clone(), _m(&acc, "dominance")),
            fee: BitcoinDollarsSatsPattern3::new(client.clone(), _m(&acc, "fee")),
            subsidy: BitcoinDollarsSatsPattern3::new(client.clone(), _m(&acc, "subsidy")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct InvestedNegNetNuplSupplyUnrealizedPattern3 {
    pub invested_capital_in_loss_pct: MetricPattern1<StoredF32>,
    pub invested_capital_in_profit_pct: MetricPattern1<StoredF32>,
    pub neg_unrealized_loss_rel_to_market_cap: MetricPattern1<StoredF32>,
    pub net_unrealized_pnl_rel_to_market_cap: MetricPattern1<StoredF32>,
    pub nupl: MetricPattern1<StoredF32>,
    pub supply_in_loss_rel_to_circulating_supply: MetricPattern1<StoredF64>,
    pub supply_in_loss_rel_to_own_supply: MetricPattern1<StoredF64>,
    pub supply_in_profit_rel_to_circulating_supply: MetricPattern1<StoredF64>,
    pub supply_in_profit_rel_to_own_supply: MetricPattern1<StoredF64>,
    pub supply_rel_to_circulating_supply: MetricPattern4<StoredF64>,
    pub unrealized_loss_rel_to_market_cap: MetricPattern1<StoredF32>,
    pub unrealized_peak_regret_rel_to_market_cap: MetricPattern4<StoredF32>,
    pub unrealized_profit_rel_to_market_cap: MetricPattern1<StoredF32>,
}

impl InvestedNegNetNuplSupplyUnrealizedPattern3 {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            invested_capital_in_loss_pct: MetricPattern1::new(client.clone(), _m(&acc, "invested_capital_in_loss_pct")),
            invested_capital_in_profit_pct: MetricPattern1::new(client.clone(), _m(&acc, "invested_capital_in_profit_pct")),
            neg_unrealized_loss_rel_to_market_cap: MetricPattern1::new(client.clone(), _m(&acc, "neg_unrealized_loss_rel_to_market_cap")),
            net_unrealized_pnl_rel_to_market_cap: MetricPattern1::new(client.clone(), _m(&acc, "net_unrealized_pnl_rel_to_market_cap")),
            nupl: MetricPattern1::new(client.clone(), _m(&acc, "nupl")),
            supply_in_loss_rel_to_circulating_supply: MetricPattern1::new(client.clone(), _m(&acc, "supply_in_loss_rel_to_circulating_supply")),
            supply_in_loss_rel_to_own_supply: MetricPattern1::new(client.clone(), _m(&acc, "supply_in_loss_rel_to_own_supply")),
            supply_in_profit_rel_to_circulating_supply: MetricPattern1::new(client.clone(), _m(&acc, "supply_in_profit_rel_to_circulating_supply")),
            supply_in_profit_rel_to_own_supply: MetricPattern1::new(client.clone(), _m(&acc, "supply_in_profit_rel_to_own_supply")),
            supply_rel_to_circulating_supply: MetricPattern4::new(client.clone(), _m(&acc, "supply_rel_to_circulating_supply")),
            unrealized_loss_rel_to_market_cap: MetricPattern1::new(client.clone(), _m(&acc, "unrealized_loss_rel_to_market_cap")),
            unrealized_peak_regret_rel_to_market_cap: MetricPattern4::new(client.clone(), _m(&acc, "unrealized_peak_regret_rel_to_market_cap")),
            unrealized_profit_rel_to_market_cap: MetricPattern1::new(client.clone(), _m(&acc, "unrealized_profit_rel_to_market_cap")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct _10y1m1w1y2y3m3y4y5y6m6y8yPattern3 {
    pub _10y: BitcoinDollarsSatsPattern5,
    pub _1m: BitcoinDollarsSatsPattern5,
    pub _1w: BitcoinDollarsSatsPattern5,
    pub _1y: BitcoinDollarsSatsPattern5,
    pub _2y: BitcoinDollarsSatsPattern5,
    pub _3m: BitcoinDollarsSatsPattern5,
    pub _3y: BitcoinDollarsSatsPattern5,
    pub _4y: BitcoinDollarsSatsPattern5,
    pub _5y: BitcoinDollarsSatsPattern5,
    pub _6m: BitcoinDollarsSatsPattern5,
    pub _6y: BitcoinDollarsSatsPattern5,
    pub _8y: BitcoinDollarsSatsPattern5,
}

impl _10y1m1w1y2y3m3y4y5y6m6y8yPattern3 {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            _10y: BitcoinDollarsSatsPattern5::new(client.clone(), _p("10y", &acc)),
            _1m: BitcoinDollarsSatsPattern5::new(client.clone(), _p("1m", &acc)),
            _1w: BitcoinDollarsSatsPattern5::new(client.clone(), _p("1w", &acc)),
            _1y: BitcoinDollarsSatsPattern5::new(client.clone(), _p("1y", &acc)),
            _2y: BitcoinDollarsSatsPattern5::new(client.clone(), _p("2y", &acc)),
            _3m: BitcoinDollarsSatsPattern5::new(client.clone(), _p("3m", &acc)),
            _3y: BitcoinDollarsSatsPattern5::new(client.clone(), _p("3y", &acc)),
            _4y: BitcoinDollarsSatsPattern5::new(client.clone(), _p("4y", &acc)),
            _5y: BitcoinDollarsSatsPattern5::new(client.clone(), _p("5y", &acc)),
            _6m: BitcoinDollarsSatsPattern5::new(client.clone(), _p("6m", &acc)),
            _6y: BitcoinDollarsSatsPattern5::new(client.clone(), _p("6y", &acc)),
            _8y: BitcoinDollarsSatsPattern5::new(client.clone(), _p("8y", &acc)),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct InvestedNegNetNuplSupplyUnrealizedPattern {
    pub invested_capital_in_loss_pct: MetricPattern1<StoredF32>,
    pub invested_capital_in_profit_pct: MetricPattern1<StoredF32>,
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

impl InvestedNegNetNuplSupplyUnrealizedPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            invested_capital_in_loss_pct: MetricPattern1::new(client.clone(), _m(&acc, "invested_capital_in_loss_pct")),
            invested_capital_in_profit_pct: MetricPattern1::new(client.clone(), _m(&acc, "invested_capital_in_profit_pct")),
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
pub struct InvestedNegNetSupplyUnrealizedPattern {
    pub invested_capital_in_loss_pct: MetricPattern1<StoredF32>,
    pub invested_capital_in_profit_pct: MetricPattern1<StoredF32>,
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

impl InvestedNegNetSupplyUnrealizedPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            invested_capital_in_loss_pct: MetricPattern1::new(client.clone(), _m(&acc, "invested_capital_in_loss_pct")),
            invested_capital_in_profit_pct: MetricPattern1::new(client.clone(), _m(&acc, "invested_capital_in_profit_pct")),
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
pub struct _10y1m1w1y2y3m3y4y5y6m6y8yPattern2<T> {
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

impl<T: DeserializeOwned> _10y1m1w1y2y3m3y4y5y6m6y8yPattern2<T> {
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
pub struct _201520162017201820192020202120222023202420252026Pattern2<T> {
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
    pub _2026: MetricPattern4<T>,
}

impl<T: DeserializeOwned> _201520162017201820192020202120222023202420252026Pattern2<T> {
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
            _2026: MetricPattern4::new(client.clone(), _m(&acc, "2026_returns")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern {
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

impl AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern {
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
pub struct AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2<T> {
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

impl<T: DeserializeOwned> AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2<T> {
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
pub struct AverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2<T> {
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

impl<T: DeserializeOwned> AverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2<T> {
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
pub struct ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern {
    pub activity: CoinblocksCoindaysSatblocksSatdaysSentPattern,
    pub addr_count: MetricPattern1<StoredU64>,
    pub addr_count_30d_change: MetricPattern4<StoredF64>,
    pub cost_basis: MaxMinPattern,
    pub outputs: UtxoPattern,
    pub realized: CapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern,
    pub relative: InvestedNegNetNuplSupplyUnrealizedPattern,
    pub supply: _30dHalvedTotalPattern,
    pub unrealized: GreedInvestedInvestorNegNetPainSupplyTotalUnrealizedPattern,
}

impl ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            activity: CoinblocksCoindaysSatblocksSatdaysSentPattern::new(client.clone(), acc.clone()),
            addr_count: MetricPattern1::new(client.clone(), _m(&acc, "addr_count")),
            addr_count_30d_change: MetricPattern4::new(client.clone(), _m(&acc, "addr_count_30d_change")),
            cost_basis: MaxMinPattern::new(client.clone(), acc.clone()),
            outputs: UtxoPattern::new(client.clone(), _m(&acc, "utxo_count")),
            realized: CapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern::new(client.clone(), acc.clone()),
            relative: InvestedNegNetNuplSupplyUnrealizedPattern::new(client.clone(), acc.clone()),
            supply: _30dHalvedTotalPattern::new(client.clone(), acc.clone()),
            unrealized: GreedInvestedInvestorNegNetPainSupplyTotalUnrealizedPattern::new(client.clone(), acc.clone()),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct AllP2aP2pk33P2pk65P2pkhP2shP2trP2wpkhP2wshPattern {
    pub all: _30dCountPattern,
    pub p2a: _30dCountPattern,
    pub p2pk33: _30dCountPattern,
    pub p2pk65: _30dCountPattern,
    pub p2pkh: _30dCountPattern,
    pub p2sh: _30dCountPattern,
    pub p2tr: _30dCountPattern,
    pub p2wpkh: _30dCountPattern,
    pub p2wsh: _30dCountPattern,
}

impl AllP2aP2pk33P2pk65P2pkhP2shP2trP2wpkhP2wshPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            all: _30dCountPattern::new(client.clone(), acc.clone()),
            p2a: _30dCountPattern::new(client.clone(), _p("p2a", &acc)),
            p2pk33: _30dCountPattern::new(client.clone(), _p("p2pk33", &acc)),
            p2pk65: _30dCountPattern::new(client.clone(), _p("p2pk65", &acc)),
            p2pkh: _30dCountPattern::new(client.clone(), _p("p2pkh", &acc)),
            p2sh: _30dCountPattern::new(client.clone(), _p("p2sh", &acc)),
            p2tr: _30dCountPattern::new(client.clone(), _p("p2tr", &acc)),
            p2wpkh: _30dCountPattern::new(client.clone(), _p("p2wpkh", &acc)),
            p2wsh: _30dCountPattern::new(client.clone(), _p("p2wsh", &acc)),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct AverageMaxMedianMinPct10Pct25Pct75Pct90TxindexPattern<T> {
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

impl<T: DeserializeOwned> AverageMaxMedianMinPct10Pct25Pct75Pct90TxindexPattern<T> {
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
pub struct AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern<T> {
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

impl<T: DeserializeOwned> AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern<T> {
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
pub struct _10y2y3y4y5y6y8yPattern {
    pub _10y: MetricPattern4<StoredF32>,
    pub _2y: MetricPattern4<StoredF32>,
    pub _3y: MetricPattern4<StoredF32>,
    pub _4y: MetricPattern4<StoredF32>,
    pub _5y: MetricPattern4<StoredF32>,
    pub _6y: MetricPattern4<StoredF32>,
    pub _8y: MetricPattern4<StoredF32>,
}

impl _10y2y3y4y5y6y8yPattern {
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
pub struct ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern {
    pub activity: CoinblocksCoindaysSatblocksSatdaysSentPattern,
    pub cost_basis: InvestedMaxMinPercentilesSpotPattern,
    pub outputs: UtxoPattern,
    pub realized: CapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern2,
    pub relative: InvestedNegNetSupplyUnrealizedPattern,
    pub supply: _30dHalvedTotalPattern,
    pub unrealized: GreedInvestedInvestorNegNetPainPeakSupplyTotalUnrealizedPattern,
}

impl ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            activity: CoinblocksCoindaysSatblocksSatdaysSentPattern::new(client.clone(), acc.clone()),
            cost_basis: InvestedMaxMinPercentilesSpotPattern::new(client.clone(), acc.clone()),
            outputs: UtxoPattern::new(client.clone(), _m(&acc, "utxo_count")),
            realized: CapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern2::new(client.clone(), acc.clone()),
            relative: InvestedNegNetSupplyUnrealizedPattern::new(client.clone(), acc.clone()),
            supply: _30dHalvedTotalPattern::new(client.clone(), acc.clone()),
            unrealized: GreedInvestedInvestorNegNetPainPeakSupplyTotalUnrealizedPattern::new(client.clone(), acc.clone()),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5 {
    pub activity: CoinblocksCoindaysSatblocksSatdaysSentPattern,
    pub cost_basis: MaxMinPattern,
    pub outputs: UtxoPattern,
    pub realized: AdjustedCapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern2,
    pub relative: InvestedNegNetNuplSupplyUnrealizedPattern3,
    pub supply: _30dHalvedTotalPattern,
    pub unrealized: GreedInvestedInvestorNegNetPainPeakSupplyTotalUnrealizedPattern,
}

impl ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5 {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            activity: CoinblocksCoindaysSatblocksSatdaysSentPattern::new(client.clone(), acc.clone()),
            cost_basis: MaxMinPattern::new(client.clone(), acc.clone()),
            outputs: UtxoPattern::new(client.clone(), _m(&acc, "utxo_count")),
            realized: AdjustedCapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern2::new(client.clone(), acc.clone()),
            relative: InvestedNegNetNuplSupplyUnrealizedPattern3::new(client.clone(), acc.clone()),
            supply: _30dHalvedTotalPattern::new(client.clone(), acc.clone()),
            unrealized: GreedInvestedInvestorNegNetPainPeakSupplyTotalUnrealizedPattern::new(client.clone(), acc.clone()),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4 {
    pub activity: CoinblocksCoindaysSatblocksSatdaysSentPattern,
    pub cost_basis: MaxMinPattern,
    pub outputs: UtxoPattern,
    pub realized: CapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern,
    pub relative: InvestedNegNetNuplSupplyUnrealizedPattern,
    pub supply: _30dHalvedTotalPattern,
    pub unrealized: GreedInvestedInvestorNegNetPainSupplyTotalUnrealizedPattern,
}

impl ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4 {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            activity: CoinblocksCoindaysSatblocksSatdaysSentPattern::new(client.clone(), acc.clone()),
            cost_basis: MaxMinPattern::new(client.clone(), acc.clone()),
            outputs: UtxoPattern::new(client.clone(), _m(&acc, "utxo_count")),
            realized: CapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern::new(client.clone(), acc.clone()),
            relative: InvestedNegNetNuplSupplyUnrealizedPattern::new(client.clone(), acc.clone()),
            supply: _30dHalvedTotalPattern::new(client.clone(), acc.clone()),
            unrealized: GreedInvestedInvestorNegNetPainSupplyTotalUnrealizedPattern::new(client.clone(), acc.clone()),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6 {
    pub activity: CoinblocksCoindaysSatblocksSatdaysSentPattern,
    pub cost_basis: MaxMinPattern,
    pub outputs: UtxoPattern,
    pub realized: CapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern,
    pub relative: InvestedNegNetNuplSupplyUnrealizedPattern3,
    pub supply: _30dHalvedTotalPattern,
    pub unrealized: GreedInvestedInvestorNegNetPainPeakSupplyTotalUnrealizedPattern,
}

impl ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6 {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            activity: CoinblocksCoindaysSatblocksSatdaysSentPattern::new(client.clone(), acc.clone()),
            cost_basis: MaxMinPattern::new(client.clone(), acc.clone()),
            outputs: UtxoPattern::new(client.clone(), _m(&acc, "utxo_count")),
            realized: CapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern::new(client.clone(), acc.clone()),
            relative: InvestedNegNetNuplSupplyUnrealizedPattern3::new(client.clone(), acc.clone()),
            supply: _30dHalvedTotalPattern::new(client.clone(), acc.clone()),
            unrealized: GreedInvestedInvestorNegNetPainPeakSupplyTotalUnrealizedPattern::new(client.clone(), acc.clone()),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 {
    pub activity: CoinblocksCoindaysSatblocksSatdaysSentPattern,
    pub cost_basis: MaxMinPattern,
    pub outputs: UtxoPattern,
    pub realized: CapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern,
    pub relative: InvestedSupplyPattern,
    pub supply: _30dHalvedTotalPattern,
    pub unrealized: GreedInvestedInvestorNegNetPainSupplyTotalUnrealizedPattern,
}

impl ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3 {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            activity: CoinblocksCoindaysSatblocksSatdaysSentPattern::new(client.clone(), acc.clone()),
            cost_basis: MaxMinPattern::new(client.clone(), acc.clone()),
            outputs: UtxoPattern::new(client.clone(), _m(&acc, "utxo_count")),
            realized: CapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern::new(client.clone(), acc.clone()),
            relative: InvestedSupplyPattern::new(client.clone(), acc.clone()),
            supply: _30dHalvedTotalPattern::new(client.clone(), acc.clone()),
            unrealized: GreedInvestedInvestorNegNetPainSupplyTotalUnrealizedPattern::new(client.clone(), acc.clone()),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct ActivityCostOutputsRealizedSupplyUnrealizedPattern {
    pub activity: CoinblocksCoindaysSatblocksSatdaysSentPattern,
    pub cost_basis: MaxMinPattern,
    pub outputs: UtxoPattern,
    pub realized: CapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern,
    pub supply: _30dHalvedTotalPattern,
    pub unrealized: GreedInvestedInvestorNegNetPainSupplyTotalUnrealizedPattern,
}

impl ActivityCostOutputsRealizedSupplyUnrealizedPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            activity: CoinblocksCoindaysSatblocksSatdaysSentPattern::new(client.clone(), acc.clone()),
            cost_basis: MaxMinPattern::new(client.clone(), acc.clone()),
            outputs: UtxoPattern::new(client.clone(), _m(&acc, "utxo_count")),
            realized: CapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern::new(client.clone(), acc.clone()),
            supply: _30dHalvedTotalPattern::new(client.clone(), acc.clone()),
            unrealized: GreedInvestedInvestorNegNetPainSupplyTotalUnrealizedPattern::new(client.clone(), acc.clone()),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct BalanceBothReactivatedReceivingSendingPattern {
    pub balance_decreased: AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern<StoredU32>,
    pub balance_increased: AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern<StoredU32>,
    pub both: AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern<StoredU32>,
    pub reactivated: AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern<StoredU32>,
    pub receiving: AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern<StoredU32>,
    pub sending: AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern<StoredU32>,
}

impl BalanceBothReactivatedReceivingSendingPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            balance_decreased: AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern::new(client.clone(), _m(&acc, "balance_decreased")),
            balance_increased: AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern::new(client.clone(), _m(&acc, "balance_increased")),
            both: AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern::new(client.clone(), _m(&acc, "both")),
            reactivated: AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern::new(client.clone(), _m(&acc, "reactivated")),
            receiving: AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern::new(client.clone(), _m(&acc, "receiving")),
            sending: AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern::new(client.clone(), _m(&acc, "sending")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct CoinblocksCoindaysSatblocksSatdaysSentPattern {
    pub coinblocks_destroyed: CumulativeSumPattern<StoredF64>,
    pub coindays_destroyed: CumulativeSumPattern<StoredF64>,
    pub satblocks_destroyed: MetricPattern11<Sats>,
    pub satdays_destroyed: MetricPattern11<Sats>,
    pub sent: BitcoinDollarsSatsPattern3,
    pub sent_14d_ema: BitcoinDollarsSatsPattern5,
}

impl CoinblocksCoindaysSatblocksSatdaysSentPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            coinblocks_destroyed: CumulativeSumPattern::new(client.clone(), _m(&acc, "coinblocks_destroyed")),
            coindays_destroyed: CumulativeSumPattern::new(client.clone(), _m(&acc, "coindays_destroyed")),
            satblocks_destroyed: MetricPattern11::new(client.clone(), _m(&acc, "satblocks_destroyed")),
            satdays_destroyed: MetricPattern11::new(client.clone(), _m(&acc, "satdays_destroyed")),
            sent: BitcoinDollarsSatsPattern3::new(client.clone(), _m(&acc, "sent")),
            sent_14d_ema: BitcoinDollarsSatsPattern5::new(client.clone(), _m(&acc, "sent_14d_ema")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct InvestedMaxMinPercentilesSpotPattern {
    pub invested_capital: Pct05Pct10Pct15Pct20Pct25Pct30Pct35Pct40Pct45Pct50Pct55Pct60Pct65Pct70Pct75Pct80Pct85Pct90Pct95Pattern,
    pub max: DollarsSatsPattern,
    pub min: DollarsSatsPattern,
    pub percentiles: Pct05Pct10Pct15Pct20Pct25Pct30Pct35Pct40Pct45Pct50Pct55Pct60Pct65Pct70Pct75Pct80Pct85Pct90Pct95Pattern,
    pub spot_cost_basis_percentile: MetricPattern4<StoredF32>,
    pub spot_invested_capital_percentile: MetricPattern4<StoredF32>,
}

impl InvestedMaxMinPercentilesSpotPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            invested_capital: Pct05Pct10Pct15Pct20Pct25Pct30Pct35Pct40Pct45Pct50Pct55Pct60Pct65Pct70Pct75Pct80Pct85Pct90Pct95Pattern::new(client.clone(), _m(&acc, "invested_capital")),
            max: DollarsSatsPattern::new(client.clone(), _m(&acc, "max_cost_basis")),
            min: DollarsSatsPattern::new(client.clone(), _m(&acc, "min_cost_basis")),
            percentiles: Pct05Pct10Pct15Pct20Pct25Pct30Pct35Pct40Pct45Pct50Pct55Pct60Pct65Pct70Pct75Pct80Pct85Pct90Pct95Pattern::new(client.clone(), _m(&acc, "cost_basis")),
            spot_cost_basis_percentile: MetricPattern4::new(client.clone(), _m(&acc, "spot_cost_basis_percentile")),
            spot_invested_capital_percentile: MetricPattern4::new(client.clone(), _m(&acc, "spot_invested_capital_percentile")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct InvestedSupplyPattern {
    pub invested_capital_in_loss_pct: MetricPattern1<StoredF32>,
    pub invested_capital_in_profit_pct: MetricPattern1<StoredF32>,
    pub supply_in_loss_rel_to_own_supply: MetricPattern1<StoredF64>,
    pub supply_in_profit_rel_to_own_supply: MetricPattern1<StoredF64>,
}

impl InvestedSupplyPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            invested_capital_in_loss_pct: MetricPattern1::new(client.clone(), _m(&acc, "invested_capital_in_loss_pct")),
            invested_capital_in_profit_pct: MetricPattern1::new(client.clone(), _m(&acc, "invested_capital_in_profit_pct")),
            supply_in_loss_rel_to_own_supply: MetricPattern1::new(client.clone(), _m(&acc, "supply_in_loss_rel_to_own_supply")),
            supply_in_profit_rel_to_own_supply: MetricPattern1::new(client.clone(), _m(&acc, "supply_in_profit_rel_to_own_supply")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct CloseHighLowOpenPattern2<T> {
    pub close: MetricPattern1<T>,
    pub high: MetricPattern1<T>,
    pub low: MetricPattern1<T>,
    pub open: MetricPattern1<T>,
}

impl<T: DeserializeOwned> CloseHighLowOpenPattern2<T> {
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
pub struct _30dHalvedTotalPattern {
    pub _30d_change: BitcoinDollarsSatsPattern5,
    pub halved: BitcoinDollarsSatsPattern4,
    pub total: BitcoinDollarsSatsPattern4,
}

impl _30dHalvedTotalPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            _30d_change: BitcoinDollarsSatsPattern5::new(client.clone(), _m(&acc, "_30d_change")),
            halved: BitcoinDollarsSatsPattern4::new(client.clone(), _m(&acc, "supply_halved")),
            total: BitcoinDollarsSatsPattern4::new(client.clone(), _m(&acc, "supply")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct BaseCumulativeSumPattern {
    pub base: MetricPattern11<StoredF32>,
    pub cumulative: MetricPattern2<StoredF32>,
    pub sum: MetricPattern2<StoredF32>,
}

impl BaseCumulativeSumPattern {
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
pub struct BitcoinDollarsSatsPattern2 {
    pub bitcoin: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern,
    pub dollars: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2<Dollars>,
    pub sats: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2<Sats>,
}

impl BitcoinDollarsSatsPattern2 {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            bitcoin: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern::new(client.clone(), _m(&acc, "btc")),
            dollars: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2::new(client.clone(), _m(&acc, "usd")),
            sats: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2::new(client.clone(), acc.clone()),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct BitcoinDollarsSatsPattern4 {
    pub bitcoin: MetricPattern1<Bitcoin>,
    pub dollars: MetricPattern1<Dollars>,
    pub sats: MetricPattern1<Sats>,
}

impl BitcoinDollarsSatsPattern4 {
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
pub struct BitcoinDollarsSatsPattern5 {
    pub bitcoin: MetricPattern4<Bitcoin>,
    pub dollars: MetricPattern4<Dollars>,
    pub sats: MetricPattern4<Sats>,
}

impl BitcoinDollarsSatsPattern5 {
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
pub struct BitcoinDollarsSatsPattern6 {
    pub bitcoin: CumulativeSumPattern<Bitcoin>,
    pub dollars: CumulativeSumPattern<Dollars>,
    pub sats: CumulativeSumPattern<Sats>,
}

impl BitcoinDollarsSatsPattern6 {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            bitcoin: CumulativeSumPattern::new(client.clone(), _m(&acc, "btc")),
            dollars: CumulativeSumPattern::new(client.clone(), _m(&acc, "usd")),
            sats: CumulativeSumPattern::new(client.clone(), acc.clone()),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct BitcoinDollarsSatsPattern3 {
    pub bitcoin: CumulativeSumPattern2<Bitcoin>,
    pub dollars: CumulativeSumPattern<Dollars>,
    pub sats: CumulativeSumPattern<Sats>,
}

impl BitcoinDollarsSatsPattern3 {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            bitcoin: CumulativeSumPattern2::new(client.clone(), _m(&acc, "btc")),
            dollars: CumulativeSumPattern::new(client.clone(), _m(&acc, "usd")),
            sats: CumulativeSumPattern::new(client.clone(), acc.clone()),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct _30dCountPattern {
    pub _30d_change: MetricPattern4<StoredF64>,
    pub count: MetricPattern1<StoredU64>,
}

impl _30dCountPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            _30d_change: MetricPattern4::new(client.clone(), _m(&acc, "30d_change")),
            count: MetricPattern1::new(client.clone(), acc.clone()),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct DollarsSatsPattern {
    pub dollars: MetricPattern1<Dollars>,
    pub sats: MetricPattern1<SatsFract>,
}

impl DollarsSatsPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            dollars: MetricPattern1::new(client.clone(), acc.clone()),
            sats: MetricPattern1::new(client.clone(), _m(&acc, "sats")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct DollarsSatsPattern2 {
    pub dollars: MetricPattern4<Dollars>,
    pub sats: MetricPattern4<SatsFract>,
}

impl DollarsSatsPattern2 {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            dollars: MetricPattern4::new(client.clone(), acc.clone()),
            sats: MetricPattern4::new(client.clone(), _m(&acc, "sats")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct MaxMinPattern {
    pub max: DollarsSatsPattern,
    pub min: DollarsSatsPattern,
}

impl MaxMinPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            max: DollarsSatsPattern::new(client.clone(), _m(&acc, "max_cost_basis")),
            min: DollarsSatsPattern::new(client.clone(), _m(&acc, "min_cost_basis")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct SdSmaPattern {
    pub sd: MetricPattern4<StoredF32>,
    pub sma: MetricPattern4<StoredF32>,
}

impl SdSmaPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            sd: MetricPattern4::new(client.clone(), _m(&acc, "sd")),
            sma: MetricPattern4::new(client.clone(), _m(&acc, "sma")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct UtxoPattern {
    pub utxo_count: MetricPattern1<StoredU64>,
    pub utxo_count_30d_change: MetricPattern4<StoredF64>,
}

impl UtxoPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            utxo_count: MetricPattern1::new(client.clone(), acc.clone()),
            utxo_count_30d_change: MetricPattern4::new(client.clone(), _m(&acc, "30d_change")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct CumulativeSumPattern<T> {
    pub cumulative: MetricPattern1<T>,
    pub sum: MetricPattern1<T>,
}

impl<T: DeserializeOwned> CumulativeSumPattern<T> {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            cumulative: MetricPattern1::new(client.clone(), _m(&acc, "cumulative")),
            sum: MetricPattern1::new(client.clone(), acc.clone()),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct CumulativeSumPattern2<T> {
    pub cumulative: MetricPattern2<T>,
    pub sum: MetricPattern1<T>,
}

impl<T: DeserializeOwned> CumulativeSumPattern2<T> {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            cumulative: MetricPattern2::new(client.clone(), _m(&acc, "cumulative")),
            sum: MetricPattern1::new(client.clone(), acc.clone()),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct OhlcSplitPattern2<T> {
    pub ohlc: MetricPattern1<T>,
    pub split: CloseHighLowOpenPattern2<T>,
}

impl<T: DeserializeOwned> OhlcSplitPattern2<T> {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            ohlc: MetricPattern1::new(client.clone(), _m(&acc, "ohlc_sats")),
            split: CloseHighLowOpenPattern2::new(client.clone(), _m(&acc, "sats")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct RatioPattern2 {
    pub ratio: MetricPattern4<StoredF32>,
}

impl RatioPattern2 {
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
    pub blocks: MetricsTree_Blocks,
    pub transactions: MetricsTree_Transactions,
    pub inputs: MetricsTree_Inputs,
    pub outputs: MetricsTree_Outputs,
    pub addresses: MetricsTree_Addresses,
    pub scripts: MetricsTree_Scripts,
    pub positions: MetricsTree_Positions,
    pub cointime: MetricsTree_Cointime,
    pub constants: MetricsTree_Constants,
    pub indexes: MetricsTree_Indexes,
    pub market: MetricsTree_Market,
    pub pools: MetricsTree_Pools,
    pub price: MetricsTree_Price,
    pub distribution: MetricsTree_Distribution,
    pub supply: MetricsTree_Supply,
}

impl MetricsTree {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            blocks: MetricsTree_Blocks::new(client.clone(), format!("{base_path}_blocks")),
            transactions: MetricsTree_Transactions::new(client.clone(), format!("{base_path}_transactions")),
            inputs: MetricsTree_Inputs::new(client.clone(), format!("{base_path}_inputs")),
            outputs: MetricsTree_Outputs::new(client.clone(), format!("{base_path}_outputs")),
            addresses: MetricsTree_Addresses::new(client.clone(), format!("{base_path}_addresses")),
            scripts: MetricsTree_Scripts::new(client.clone(), format!("{base_path}_scripts")),
            positions: MetricsTree_Positions::new(client.clone(), format!("{base_path}_positions")),
            cointime: MetricsTree_Cointime::new(client.clone(), format!("{base_path}_cointime")),
            constants: MetricsTree_Constants::new(client.clone(), format!("{base_path}_constants")),
            indexes: MetricsTree_Indexes::new(client.clone(), format!("{base_path}_indexes")),
            market: MetricsTree_Market::new(client.clone(), format!("{base_path}_market")),
            pools: MetricsTree_Pools::new(client.clone(), format!("{base_path}_pools")),
            price: MetricsTree_Price::new(client.clone(), format!("{base_path}_price")),
            distribution: MetricsTree_Distribution::new(client.clone(), format!("{base_path}_distribution")),
            supply: MetricsTree_Supply::new(client.clone(), format!("{base_path}_supply")),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Blocks {
    pub blockhash: MetricPattern11<BlockHash>,
    pub difficulty: MetricsTree_Blocks_Difficulty,
    pub time: MetricsTree_Blocks_Time,
    pub total_size: MetricPattern11<StoredU64>,
    pub weight: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2<Weight>,
    pub count: MetricsTree_Blocks_Count,
    pub interval: AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern<Timestamp>,
    pub mining: MetricsTree_Blocks_Mining,
    pub rewards: MetricsTree_Blocks_Rewards,
    pub halving: MetricsTree_Blocks_Halving,
    pub vbytes: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2<StoredU64>,
    pub size: MetricsTree_Blocks_Size,
    pub fullness: AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern<StoredF32>,
}

impl MetricsTree_Blocks {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            blockhash: MetricPattern11::new(client.clone(), "blockhash".to_string()),
            difficulty: MetricsTree_Blocks_Difficulty::new(client.clone(), format!("{base_path}_difficulty")),
            time: MetricsTree_Blocks_Time::new(client.clone(), format!("{base_path}_time")),
            total_size: MetricPattern11::new(client.clone(), "total_size".to_string()),
            weight: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2::new(client.clone(), "block_weight".to_string()),
            count: MetricsTree_Blocks_Count::new(client.clone(), format!("{base_path}_count")),
            interval: AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern::new(client.clone(), "block_interval".to_string()),
            mining: MetricsTree_Blocks_Mining::new(client.clone(), format!("{base_path}_mining")),
            rewards: MetricsTree_Blocks_Rewards::new(client.clone(), format!("{base_path}_rewards")),
            halving: MetricsTree_Blocks_Halving::new(client.clone(), format!("{base_path}_halving")),
            vbytes: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2::new(client.clone(), "block_vbytes".to_string()),
            size: MetricsTree_Blocks_Size::new(client.clone(), format!("{base_path}_size")),
            fullness: AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern::new(client.clone(), "block_fullness".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Blocks_Difficulty {
    pub raw: MetricPattern1<StoredF64>,
    pub as_hash: MetricPattern1<StoredF32>,
    pub adjustment: MetricPattern1<StoredF32>,
    pub epoch: MetricPattern4<DifficultyEpoch>,
    pub blocks_before_next_adjustment: MetricPattern1<StoredU32>,
    pub days_before_next_adjustment: MetricPattern1<StoredF32>,
}

impl MetricsTree_Blocks_Difficulty {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            raw: MetricPattern1::new(client.clone(), "difficulty".to_string()),
            as_hash: MetricPattern1::new(client.clone(), "difficulty_as_hash".to_string()),
            adjustment: MetricPattern1::new(client.clone(), "difficulty_adjustment".to_string()),
            epoch: MetricPattern4::new(client.clone(), "difficultyepoch".to_string()),
            blocks_before_next_adjustment: MetricPattern1::new(client.clone(), "blocks_before_next_difficulty_adjustment".to_string()),
            days_before_next_adjustment: MetricPattern1::new(client.clone(), "days_before_next_difficulty_adjustment".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Blocks_Time {
    pub timestamp: MetricPattern1<Timestamp>,
    pub date: MetricPattern11<Date>,
    pub timestamp_monotonic: MetricPattern11<Timestamp>,
}

impl MetricsTree_Blocks_Time {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            timestamp: MetricPattern1::new(client.clone(), "timestamp".to_string()),
            date: MetricPattern11::new(client.clone(), "date".to_string()),
            timestamp_monotonic: MetricPattern11::new(client.clone(), "timestamp_monotonic".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Blocks_Count {
    pub block_count_target: MetricPattern4<StoredU64>,
    pub block_count: CumulativeSumPattern<StoredU32>,
    pub _24h_start: MetricPattern11<Height>,
    pub _1w_start: MetricPattern11<Height>,
    pub _1m_start: MetricPattern11<Height>,
    pub _1y_start: MetricPattern11<Height>,
    pub _24h_block_count: MetricPattern1<StoredU32>,
    pub _1w_block_count: MetricPattern1<StoredU32>,
    pub _1m_block_count: MetricPattern1<StoredU32>,
    pub _1y_block_count: MetricPattern1<StoredU32>,
}

impl MetricsTree_Blocks_Count {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            block_count_target: MetricPattern4::new(client.clone(), "block_count_target".to_string()),
            block_count: CumulativeSumPattern::new(client.clone(), "block_count".to_string()),
            _24h_start: MetricPattern11::new(client.clone(), "24h_start".to_string()),
            _1w_start: MetricPattern11::new(client.clone(), "1w_start".to_string()),
            _1m_start: MetricPattern11::new(client.clone(), "1m_start".to_string()),
            _1y_start: MetricPattern11::new(client.clone(), "1y_start".to_string()),
            _24h_block_count: MetricPattern1::new(client.clone(), "24h_block_count".to_string()),
            _1w_block_count: MetricPattern1::new(client.clone(), "1w_block_count".to_string()),
            _1m_block_count: MetricPattern1::new(client.clone(), "1m_block_count".to_string()),
            _1y_block_count: MetricPattern1::new(client.clone(), "1y_block_count".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Blocks_Mining {
    pub hash_rate: MetricPattern1<StoredF64>,
    pub hash_rate_1w_sma: MetricPattern4<StoredF64>,
    pub hash_rate_1m_sma: MetricPattern4<StoredF32>,
    pub hash_rate_2m_sma: MetricPattern4<StoredF32>,
    pub hash_rate_1y_sma: MetricPattern4<StoredF32>,
    pub hash_rate_ath: MetricPattern1<StoredF64>,
    pub hash_rate_drawdown: MetricPattern1<StoredF32>,
    pub hash_price_ths: MetricPattern1<StoredF32>,
    pub hash_price_ths_min: MetricPattern1<StoredF32>,
    pub hash_price_phs: MetricPattern1<StoredF32>,
    pub hash_price_phs_min: MetricPattern1<StoredF32>,
    pub hash_price_rebound: MetricPattern1<StoredF32>,
    pub hash_value_ths: MetricPattern1<StoredF32>,
    pub hash_value_ths_min: MetricPattern1<StoredF32>,
    pub hash_value_phs: MetricPattern1<StoredF32>,
    pub hash_value_phs_min: MetricPattern1<StoredF32>,
    pub hash_value_rebound: MetricPattern1<StoredF32>,
}

impl MetricsTree_Blocks_Mining {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            hash_rate: MetricPattern1::new(client.clone(), "hash_rate".to_string()),
            hash_rate_1w_sma: MetricPattern4::new(client.clone(), "hash_rate_1w_sma".to_string()),
            hash_rate_1m_sma: MetricPattern4::new(client.clone(), "hash_rate_1m_sma".to_string()),
            hash_rate_2m_sma: MetricPattern4::new(client.clone(), "hash_rate_2m_sma".to_string()),
            hash_rate_1y_sma: MetricPattern4::new(client.clone(), "hash_rate_1y_sma".to_string()),
            hash_rate_ath: MetricPattern1::new(client.clone(), "hash_rate_ath".to_string()),
            hash_rate_drawdown: MetricPattern1::new(client.clone(), "hash_rate_drawdown".to_string()),
            hash_price_ths: MetricPattern1::new(client.clone(), "hash_price_ths".to_string()),
            hash_price_ths_min: MetricPattern1::new(client.clone(), "hash_price_ths_min".to_string()),
            hash_price_phs: MetricPattern1::new(client.clone(), "hash_price_phs".to_string()),
            hash_price_phs_min: MetricPattern1::new(client.clone(), "hash_price_phs_min".to_string()),
            hash_price_rebound: MetricPattern1::new(client.clone(), "hash_price_rebound".to_string()),
            hash_value_ths: MetricPattern1::new(client.clone(), "hash_value_ths".to_string()),
            hash_value_ths_min: MetricPattern1::new(client.clone(), "hash_value_ths_min".to_string()),
            hash_value_phs: MetricPattern1::new(client.clone(), "hash_value_phs".to_string()),
            hash_value_phs_min: MetricPattern1::new(client.clone(), "hash_value_phs_min".to_string()),
            hash_value_rebound: MetricPattern1::new(client.clone(), "hash_value_rebound".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Blocks_Rewards {
    pub _24h_coinbase_sum: MetricsTree_Blocks_Rewards_24hCoinbaseSum,
    pub coinbase: BitcoinDollarsSatsPattern2,
    pub subsidy: BitcoinDollarsSatsPattern2,
    pub unclaimed_rewards: BitcoinDollarsSatsPattern3,
    pub fee_dominance: MetricPattern6<StoredF32>,
    pub subsidy_dominance: MetricPattern6<StoredF32>,
    pub subsidy_usd_1y_sma: MetricPattern4<Dollars>,
}

impl MetricsTree_Blocks_Rewards {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _24h_coinbase_sum: MetricsTree_Blocks_Rewards_24hCoinbaseSum::new(client.clone(), format!("{base_path}_24h_coinbase_sum")),
            coinbase: BitcoinDollarsSatsPattern2::new(client.clone(), "coinbase".to_string()),
            subsidy: BitcoinDollarsSatsPattern2::new(client.clone(), "subsidy".to_string()),
            unclaimed_rewards: BitcoinDollarsSatsPattern3::new(client.clone(), "unclaimed_rewards".to_string()),
            fee_dominance: MetricPattern6::new(client.clone(), "fee_dominance".to_string()),
            subsidy_dominance: MetricPattern6::new(client.clone(), "subsidy_dominance".to_string()),
            subsidy_usd_1y_sma: MetricPattern4::new(client.clone(), "subsidy_usd_1y_sma".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Blocks_Rewards_24hCoinbaseSum {
    pub sats: MetricPattern11<Sats>,
    pub bitcoin: MetricPattern11<Bitcoin>,
    pub dollars: MetricPattern11<Dollars>,
}

impl MetricsTree_Blocks_Rewards_24hCoinbaseSum {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            sats: MetricPattern11::new(client.clone(), "24h_coinbase_sum".to_string()),
            bitcoin: MetricPattern11::new(client.clone(), "24h_coinbase_sum_btc".to_string()),
            dollars: MetricPattern11::new(client.clone(), "24h_coinbase_sum_usd".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Blocks_Halving {
    pub epoch: MetricPattern4<HalvingEpoch>,
    pub blocks_before_next_halving: MetricPattern1<StoredU32>,
    pub days_before_next_halving: MetricPattern1<StoredF32>,
}

impl MetricsTree_Blocks_Halving {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            epoch: MetricPattern4::new(client.clone(), "halvingepoch".to_string()),
            blocks_before_next_halving: MetricPattern1::new(client.clone(), "blocks_before_next_halving".to_string()),
            days_before_next_halving: MetricPattern1::new(client.clone(), "days_before_next_halving".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Blocks_Size {
    pub cumulative: MetricPattern1<StoredU64>,
    pub average: MetricPattern2<StoredU64>,
    pub min: MetricPattern2<StoredU64>,
    pub max: MetricPattern2<StoredU64>,
    pub pct10: MetricPattern6<StoredU64>,
    pub pct25: MetricPattern6<StoredU64>,
    pub median: MetricPattern6<StoredU64>,
    pub pct75: MetricPattern6<StoredU64>,
    pub pct90: MetricPattern6<StoredU64>,
    pub sum: MetricPattern2<StoredU64>,
}

impl MetricsTree_Blocks_Size {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            cumulative: MetricPattern1::new(client.clone(), "block_size_cumulative".to_string()),
            average: MetricPattern2::new(client.clone(), "block_size_average".to_string()),
            min: MetricPattern2::new(client.clone(), "block_size_min".to_string()),
            max: MetricPattern2::new(client.clone(), "block_size_max".to_string()),
            pct10: MetricPattern6::new(client.clone(), "block_size_pct10".to_string()),
            pct25: MetricPattern6::new(client.clone(), "block_size_pct25".to_string()),
            median: MetricPattern6::new(client.clone(), "block_size_median".to_string()),
            pct75: MetricPattern6::new(client.clone(), "block_size_pct75".to_string()),
            pct90: MetricPattern6::new(client.clone(), "block_size_pct90".to_string()),
            sum: MetricPattern2::new(client.clone(), "block_size_sum".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Transactions {
    pub first_txindex: MetricPattern11<TxIndex>,
    pub height: MetricPattern27<Height>,
    pub txid: MetricPattern27<Txid>,
    pub txversion: MetricPattern27<TxVersion>,
    pub rawlocktime: MetricPattern27<RawLockTime>,
    pub base_size: MetricPattern27<StoredU32>,
    pub total_size: MetricPattern27<StoredU32>,
    pub is_explicitly_rbf: MetricPattern27<StoredBool>,
    pub first_txinindex: MetricPattern27<TxInIndex>,
    pub first_txoutindex: MetricPattern27<TxOutIndex>,
    pub count: MetricsTree_Transactions_Count,
    pub size: MetricsTree_Transactions_Size,
    pub fees: MetricsTree_Transactions_Fees,
    pub versions: MetricsTree_Transactions_Versions,
    pub volume: MetricsTree_Transactions_Volume,
}

impl MetricsTree_Transactions {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            first_txindex: MetricPattern11::new(client.clone(), "first_txindex".to_string()),
            height: MetricPattern27::new(client.clone(), "height".to_string()),
            txid: MetricPattern27::new(client.clone(), "txid".to_string()),
            txversion: MetricPattern27::new(client.clone(), "txversion".to_string()),
            rawlocktime: MetricPattern27::new(client.clone(), "rawlocktime".to_string()),
            base_size: MetricPattern27::new(client.clone(), "base_size".to_string()),
            total_size: MetricPattern27::new(client.clone(), "total_size".to_string()),
            is_explicitly_rbf: MetricPattern27::new(client.clone(), "is_explicitly_rbf".to_string()),
            first_txinindex: MetricPattern27::new(client.clone(), "first_txinindex".to_string()),
            first_txoutindex: MetricPattern27::new(client.clone(), "first_txoutindex".to_string()),
            count: MetricsTree_Transactions_Count::new(client.clone(), format!("{base_path}_count")),
            size: MetricsTree_Transactions_Size::new(client.clone(), format!("{base_path}_size")),
            fees: MetricsTree_Transactions_Fees::new(client.clone(), format!("{base_path}_fees")),
            versions: MetricsTree_Transactions_Versions::new(client.clone(), format!("{base_path}_versions")),
            volume: MetricsTree_Transactions_Volume::new(client.clone(), format!("{base_path}_volume")),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Transactions_Count {
    pub tx_count: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2<StoredU64>,
    pub is_coinbase: MetricPattern27<StoredBool>,
}

impl MetricsTree_Transactions_Count {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            tx_count: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2::new(client.clone(), "tx_count".to_string()),
            is_coinbase: MetricPattern27::new(client.clone(), "is_coinbase".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Transactions_Size {
    pub vsize: AverageMaxMedianMinPct10Pct25Pct75Pct90TxindexPattern<VSize>,
    pub weight: AverageMaxMedianMinPct10Pct25Pct75Pct90TxindexPattern<Weight>,
}

impl MetricsTree_Transactions_Size {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            vsize: AverageMaxMedianMinPct10Pct25Pct75Pct90TxindexPattern::new(client.clone(), "tx_vsize".to_string()),
            weight: AverageMaxMedianMinPct10Pct25Pct75Pct90TxindexPattern::new(client.clone(), "tx_weight".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Transactions_Fees {
    pub input_value: MetricPattern27<Sats>,
    pub output_value: MetricPattern27<Sats>,
    pub fee: MetricsTree_Transactions_Fees_Fee,
    pub fee_rate: AverageMaxMedianMinPct10Pct25Pct75Pct90TxindexPattern<FeeRate>,
}

impl MetricsTree_Transactions_Fees {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            input_value: MetricPattern27::new(client.clone(), "input_value".to_string()),
            output_value: MetricPattern27::new(client.clone(), "output_value".to_string()),
            fee: MetricsTree_Transactions_Fees_Fee::new(client.clone(), format!("{base_path}_fee")),
            fee_rate: AverageMaxMedianMinPct10Pct25Pct75Pct90TxindexPattern::new(client.clone(), "fee_rate".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Transactions_Fees_Fee {
    pub txindex: MetricPattern27<Sats>,
    pub sats: AverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2<Sats>,
    pub bitcoin: AverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2<Bitcoin>,
    pub dollars: AverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2<Dollars>,
}

impl MetricsTree_Transactions_Fees_Fee {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            txindex: MetricPattern27::new(client.clone(), "fee".to_string()),
            sats: AverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2::new(client.clone(), "fee".to_string()),
            bitcoin: AverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2::new(client.clone(), "fee_btc".to_string()),
            dollars: AverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2::new(client.clone(), "fee_usd".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Transactions_Versions {
    pub v1: CumulativeSumPattern<StoredU64>,
    pub v2: CumulativeSumPattern<StoredU64>,
    pub v3: CumulativeSumPattern<StoredU64>,
}

impl MetricsTree_Transactions_Versions {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            v1: CumulativeSumPattern::new(client.clone(), "tx_v1".to_string()),
            v2: CumulativeSumPattern::new(client.clone(), "tx_v2".to_string()),
            v3: CumulativeSumPattern::new(client.clone(), "tx_v3".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Transactions_Volume {
    pub sent_sum: BitcoinDollarsSatsPattern4,
    pub received_sum: BitcoinDollarsSatsPattern4,
    pub annualized_volume: BitcoinDollarsSatsPattern5,
    pub tx_per_sec: MetricPattern4<StoredF32>,
    pub outputs_per_sec: MetricPattern4<StoredF32>,
    pub inputs_per_sec: MetricPattern4<StoredF32>,
}

impl MetricsTree_Transactions_Volume {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            sent_sum: BitcoinDollarsSatsPattern4::new(client.clone(), "sent_sum".to_string()),
            received_sum: BitcoinDollarsSatsPattern4::new(client.clone(), "received_sum".to_string()),
            annualized_volume: BitcoinDollarsSatsPattern5::new(client.clone(), "annualized_volume".to_string()),
            tx_per_sec: MetricPattern4::new(client.clone(), "tx_per_sec".to_string()),
            outputs_per_sec: MetricPattern4::new(client.clone(), "outputs_per_sec".to_string()),
            inputs_per_sec: MetricPattern4::new(client.clone(), "inputs_per_sec".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Inputs {
    pub first_txinindex: MetricPattern11<TxInIndex>,
    pub outpoint: MetricPattern12<OutPoint>,
    pub txindex: MetricPattern12<TxIndex>,
    pub outputtype: MetricPattern12<OutputType>,
    pub typeindex: MetricPattern12<TypeIndex>,
    pub spent: MetricsTree_Inputs_Spent,
    pub count: AverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2<StoredU64>,
}

impl MetricsTree_Inputs {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            first_txinindex: MetricPattern11::new(client.clone(), "first_txinindex".to_string()),
            outpoint: MetricPattern12::new(client.clone(), "outpoint".to_string()),
            txindex: MetricPattern12::new(client.clone(), "txindex".to_string()),
            outputtype: MetricPattern12::new(client.clone(), "outputtype".to_string()),
            typeindex: MetricPattern12::new(client.clone(), "typeindex".to_string()),
            spent: MetricsTree_Inputs_Spent::new(client.clone(), format!("{base_path}_spent")),
            count: AverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2::new(client.clone(), "input_count".to_string()),
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
pub struct MetricsTree_Outputs {
    pub first_txoutindex: MetricPattern11<TxOutIndex>,
    pub value: MetricPattern15<Sats>,
    pub outputtype: MetricPattern15<OutputType>,
    pub typeindex: MetricPattern15<TypeIndex>,
    pub txindex: MetricPattern15<TxIndex>,
    pub spent: MetricsTree_Outputs_Spent,
    pub count: MetricsTree_Outputs_Count,
}

impl MetricsTree_Outputs {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            first_txoutindex: MetricPattern11::new(client.clone(), "first_txoutindex".to_string()),
            value: MetricPattern15::new(client.clone(), "value".to_string()),
            outputtype: MetricPattern15::new(client.clone(), "outputtype".to_string()),
            typeindex: MetricPattern15::new(client.clone(), "typeindex".to_string()),
            txindex: MetricPattern15::new(client.clone(), "txindex".to_string()),
            spent: MetricsTree_Outputs_Spent::new(client.clone(), format!("{base_path}_spent")),
            count: MetricsTree_Outputs_Count::new(client.clone(), format!("{base_path}_count")),
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
pub struct MetricsTree_Outputs_Count {
    pub total_count: AverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2<StoredU64>,
    pub utxo_count: MetricPattern1<StoredU64>,
}

impl MetricsTree_Outputs_Count {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            total_count: AverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2::new(client.clone(), "output_count".to_string()),
            utxo_count: MetricPattern1::new(client.clone(), "exact_utxo_count".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Addresses {
    pub first_p2pk65addressindex: MetricPattern11<P2PK65AddressIndex>,
    pub first_p2pk33addressindex: MetricPattern11<P2PK33AddressIndex>,
    pub first_p2pkhaddressindex: MetricPattern11<P2PKHAddressIndex>,
    pub first_p2shaddressindex: MetricPattern11<P2SHAddressIndex>,
    pub first_p2wpkhaddressindex: MetricPattern11<P2WPKHAddressIndex>,
    pub first_p2wshaddressindex: MetricPattern11<P2WSHAddressIndex>,
    pub first_p2traddressindex: MetricPattern11<P2TRAddressIndex>,
    pub first_p2aaddressindex: MetricPattern11<P2AAddressIndex>,
    pub p2pk65bytes: MetricPattern19<P2PK65Bytes>,
    pub p2pk33bytes: MetricPattern18<P2PK33Bytes>,
    pub p2pkhbytes: MetricPattern20<P2PKHBytes>,
    pub p2shbytes: MetricPattern21<P2SHBytes>,
    pub p2wpkhbytes: MetricPattern23<P2WPKHBytes>,
    pub p2wshbytes: MetricPattern24<P2WSHBytes>,
    pub p2trbytes: MetricPattern22<P2TRBytes>,
    pub p2abytes: MetricPattern16<P2ABytes>,
}

impl MetricsTree_Addresses {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            first_p2pk65addressindex: MetricPattern11::new(client.clone(), "first_p2pk65addressindex".to_string()),
            first_p2pk33addressindex: MetricPattern11::new(client.clone(), "first_p2pk33addressindex".to_string()),
            first_p2pkhaddressindex: MetricPattern11::new(client.clone(), "first_p2pkhaddressindex".to_string()),
            first_p2shaddressindex: MetricPattern11::new(client.clone(), "first_p2shaddressindex".to_string()),
            first_p2wpkhaddressindex: MetricPattern11::new(client.clone(), "first_p2wpkhaddressindex".to_string()),
            first_p2wshaddressindex: MetricPattern11::new(client.clone(), "first_p2wshaddressindex".to_string()),
            first_p2traddressindex: MetricPattern11::new(client.clone(), "first_p2traddressindex".to_string()),
            first_p2aaddressindex: MetricPattern11::new(client.clone(), "first_p2aaddressindex".to_string()),
            p2pk65bytes: MetricPattern19::new(client.clone(), "p2pk65bytes".to_string()),
            p2pk33bytes: MetricPattern18::new(client.clone(), "p2pk33bytes".to_string()),
            p2pkhbytes: MetricPattern20::new(client.clone(), "p2pkhbytes".to_string()),
            p2shbytes: MetricPattern21::new(client.clone(), "p2shbytes".to_string()),
            p2wpkhbytes: MetricPattern23::new(client.clone(), "p2wpkhbytes".to_string()),
            p2wshbytes: MetricPattern24::new(client.clone(), "p2wshbytes".to_string()),
            p2trbytes: MetricPattern22::new(client.clone(), "p2trbytes".to_string()),
            p2abytes: MetricPattern16::new(client.clone(), "p2abytes".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Scripts {
    pub first_emptyoutputindex: MetricPattern11<EmptyOutputIndex>,
    pub first_opreturnindex: MetricPattern11<OpReturnIndex>,
    pub first_p2msoutputindex: MetricPattern11<P2MSOutputIndex>,
    pub first_unknownoutputindex: MetricPattern11<UnknownOutputIndex>,
    pub empty_to_txindex: MetricPattern9<TxIndex>,
    pub opreturn_to_txindex: MetricPattern14<TxIndex>,
    pub p2ms_to_txindex: MetricPattern17<TxIndex>,
    pub unknown_to_txindex: MetricPattern28<TxIndex>,
    pub count: MetricsTree_Scripts_Count,
    pub value: MetricsTree_Scripts_Value,
}

impl MetricsTree_Scripts {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            first_emptyoutputindex: MetricPattern11::new(client.clone(), "first_emptyoutputindex".to_string()),
            first_opreturnindex: MetricPattern11::new(client.clone(), "first_opreturnindex".to_string()),
            first_p2msoutputindex: MetricPattern11::new(client.clone(), "first_p2msoutputindex".to_string()),
            first_unknownoutputindex: MetricPattern11::new(client.clone(), "first_unknownoutputindex".to_string()),
            empty_to_txindex: MetricPattern9::new(client.clone(), "txindex".to_string()),
            opreturn_to_txindex: MetricPattern14::new(client.clone(), "txindex".to_string()),
            p2ms_to_txindex: MetricPattern17::new(client.clone(), "txindex".to_string()),
            unknown_to_txindex: MetricPattern28::new(client.clone(), "txindex".to_string()),
            count: MetricsTree_Scripts_Count::new(client.clone(), format!("{base_path}_count")),
            value: MetricsTree_Scripts_Value::new(client.clone(), format!("{base_path}_value")),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Scripts_Count {
    pub p2a: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2<StoredU64>,
    pub p2ms: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2<StoredU64>,
    pub p2pk33: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2<StoredU64>,
    pub p2pk65: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2<StoredU64>,
    pub p2pkh: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2<StoredU64>,
    pub p2sh: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2<StoredU64>,
    pub p2tr: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2<StoredU64>,
    pub p2wpkh: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2<StoredU64>,
    pub p2wsh: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2<StoredU64>,
    pub opreturn: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2<StoredU64>,
    pub emptyoutput: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2<StoredU64>,
    pub unknownoutput: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2<StoredU64>,
    pub segwit: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2<StoredU64>,
    pub taproot_adoption: BaseCumulativeSumPattern,
    pub segwit_adoption: BaseCumulativeSumPattern,
}

impl MetricsTree_Scripts_Count {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            p2a: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2::new(client.clone(), "p2a_count".to_string()),
            p2ms: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2::new(client.clone(), "p2ms_count".to_string()),
            p2pk33: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2::new(client.clone(), "p2pk33_count".to_string()),
            p2pk65: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2::new(client.clone(), "p2pk65_count".to_string()),
            p2pkh: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2::new(client.clone(), "p2pkh_count".to_string()),
            p2sh: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2::new(client.clone(), "p2sh_count".to_string()),
            p2tr: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2::new(client.clone(), "p2tr_count".to_string()),
            p2wpkh: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2::new(client.clone(), "p2wpkh_count".to_string()),
            p2wsh: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2::new(client.clone(), "p2wsh_count".to_string()),
            opreturn: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2::new(client.clone(), "opreturn_count".to_string()),
            emptyoutput: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2::new(client.clone(), "emptyoutput_count".to_string()),
            unknownoutput: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2::new(client.clone(), "unknownoutput_count".to_string()),
            segwit: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2::new(client.clone(), "segwit_count".to_string()),
            taproot_adoption: BaseCumulativeSumPattern::new(client.clone(), "taproot_adoption".to_string()),
            segwit_adoption: BaseCumulativeSumPattern::new(client.clone(), "segwit_adoption".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Scripts_Value {
    pub opreturn: BitcoinDollarsSatsPattern2,
}

impl MetricsTree_Scripts_Value {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            opreturn: BitcoinDollarsSatsPattern2::new(client.clone(), "opreturn_value".to_string()),
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
pub struct MetricsTree_Cointime {
    pub activity: MetricsTree_Cointime_Activity,
    pub supply: MetricsTree_Cointime_Supply,
    pub value: MetricsTree_Cointime_Value,
    pub cap: MetricsTree_Cointime_Cap,
    pub pricing: MetricsTree_Cointime_Pricing,
    pub adjusted: MetricsTree_Cointime_Adjusted,
    pub reserve_risk: MetricsTree_Cointime_ReserveRisk,
}

impl MetricsTree_Cointime {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            activity: MetricsTree_Cointime_Activity::new(client.clone(), format!("{base_path}_activity")),
            supply: MetricsTree_Cointime_Supply::new(client.clone(), format!("{base_path}_supply")),
            value: MetricsTree_Cointime_Value::new(client.clone(), format!("{base_path}_value")),
            cap: MetricsTree_Cointime_Cap::new(client.clone(), format!("{base_path}_cap")),
            pricing: MetricsTree_Cointime_Pricing::new(client.clone(), format!("{base_path}_pricing")),
            adjusted: MetricsTree_Cointime_Adjusted::new(client.clone(), format!("{base_path}_adjusted")),
            reserve_risk: MetricsTree_Cointime_ReserveRisk::new(client.clone(), format!("{base_path}_reserve_risk")),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Cointime_Activity {
    pub coinblocks_created: CumulativeSumPattern<StoredF64>,
    pub coinblocks_stored: CumulativeSumPattern<StoredF64>,
    pub liveliness: MetricPattern1<StoredF64>,
    pub vaultedness: MetricPattern1<StoredF64>,
    pub activity_to_vaultedness_ratio: MetricPattern1<StoredF64>,
}

impl MetricsTree_Cointime_Activity {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            coinblocks_created: CumulativeSumPattern::new(client.clone(), "coinblocks_created".to_string()),
            coinblocks_stored: CumulativeSumPattern::new(client.clone(), "coinblocks_stored".to_string()),
            liveliness: MetricPattern1::new(client.clone(), "liveliness".to_string()),
            vaultedness: MetricPattern1::new(client.clone(), "vaultedness".to_string()),
            activity_to_vaultedness_ratio: MetricPattern1::new(client.clone(), "activity_to_vaultedness_ratio".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Cointime_Supply {
    pub vaulted_supply: BitcoinDollarsSatsPattern4,
    pub active_supply: BitcoinDollarsSatsPattern4,
}

impl MetricsTree_Cointime_Supply {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            vaulted_supply: BitcoinDollarsSatsPattern4::new(client.clone(), "vaulted_supply".to_string()),
            active_supply: BitcoinDollarsSatsPattern4::new(client.clone(), "active_supply".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Cointime_Value {
    pub cointime_value_destroyed: CumulativeSumPattern<StoredF64>,
    pub cointime_value_created: CumulativeSumPattern<StoredF64>,
    pub cointime_value_stored: CumulativeSumPattern<StoredF64>,
    pub vocdd: CumulativeSumPattern<StoredF64>,
}

impl MetricsTree_Cointime_Value {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            cointime_value_destroyed: CumulativeSumPattern::new(client.clone(), "cointime_value_destroyed".to_string()),
            cointime_value_created: CumulativeSumPattern::new(client.clone(), "cointime_value_created".to_string()),
            cointime_value_stored: CumulativeSumPattern::new(client.clone(), "cointime_value_stored".to_string()),
            vocdd: CumulativeSumPattern::new(client.clone(), "vocdd".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Cointime_Cap {
    pub thermo_cap: MetricPattern1<Dollars>,
    pub investor_cap: MetricPattern1<Dollars>,
    pub vaulted_cap: MetricPattern1<Dollars>,
    pub active_cap: MetricPattern1<Dollars>,
    pub cointime_cap: MetricPattern1<Dollars>,
}

impl MetricsTree_Cointime_Cap {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            thermo_cap: MetricPattern1::new(client.clone(), "thermo_cap".to_string()),
            investor_cap: MetricPattern1::new(client.clone(), "investor_cap".to_string()),
            vaulted_cap: MetricPattern1::new(client.clone(), "vaulted_cap".to_string()),
            active_cap: MetricPattern1::new(client.clone(), "active_cap".to_string()),
            cointime_cap: MetricPattern1::new(client.clone(), "cointime_cap".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Cointime_Pricing {
    pub vaulted_price: DollarsSatsPattern,
    pub vaulted_price_ratio: RatioPattern,
    pub active_price: DollarsSatsPattern,
    pub active_price_ratio: RatioPattern,
    pub true_market_mean: DollarsSatsPattern,
    pub true_market_mean_ratio: RatioPattern,
    pub cointime_price: DollarsSatsPattern,
    pub cointime_price_ratio: RatioPattern,
}

impl MetricsTree_Cointime_Pricing {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            vaulted_price: DollarsSatsPattern::new(client.clone(), "vaulted_price".to_string()),
            vaulted_price_ratio: RatioPattern::new(client.clone(), "vaulted_price_ratio".to_string()),
            active_price: DollarsSatsPattern::new(client.clone(), "active_price".to_string()),
            active_price_ratio: RatioPattern::new(client.clone(), "active_price_ratio".to_string()),
            true_market_mean: DollarsSatsPattern::new(client.clone(), "true_market_mean".to_string()),
            true_market_mean_ratio: RatioPattern::new(client.clone(), "true_market_mean_ratio".to_string()),
            cointime_price: DollarsSatsPattern::new(client.clone(), "cointime_price".to_string()),
            cointime_price_ratio: RatioPattern::new(client.clone(), "cointime_price_ratio".to_string()),
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
            cointime_adj_inflation_rate: MetricPattern4::new(client.clone(), "cointime_adj_inflation_rate".to_string()),
            cointime_adj_tx_btc_velocity: MetricPattern4::new(client.clone(), "cointime_adj_tx_btc_velocity".to_string()),
            cointime_adj_tx_usd_velocity: MetricPattern4::new(client.clone(), "cointime_adj_tx_usd_velocity".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Cointime_ReserveRisk {
    pub vocdd_365d_median: MetricPattern6<StoredF64>,
    pub hodl_bank: MetricPattern6<StoredF64>,
    pub reserve_risk: MetricPattern4<StoredF64>,
}

impl MetricsTree_Cointime_ReserveRisk {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            vocdd_365d_median: MetricPattern6::new(client.clone(), "vocdd_365d_median".to_string()),
            hodl_bank: MetricPattern6::new(client.clone(), "hodl_bank".to_string()),
            reserve_risk: MetricPattern4::new(client.clone(), "reserve_risk".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Constants {
    pub constant_0: MetricPattern1<StoredU16>,
    pub constant_1: MetricPattern1<StoredU16>,
    pub constant_2: MetricPattern1<StoredU16>,
    pub constant_3: MetricPattern1<StoredU16>,
    pub constant_4: MetricPattern1<StoredU16>,
    pub constant_20: MetricPattern1<StoredU16>,
    pub constant_30: MetricPattern1<StoredU16>,
    pub constant_38_2: MetricPattern1<StoredF32>,
    pub constant_50: MetricPattern1<StoredU16>,
    pub constant_61_8: MetricPattern1<StoredF32>,
    pub constant_70: MetricPattern1<StoredU16>,
    pub constant_80: MetricPattern1<StoredU16>,
    pub constant_100: MetricPattern1<StoredU16>,
    pub constant_600: MetricPattern1<StoredU16>,
    pub constant_minus_1: MetricPattern1<StoredI8>,
    pub constant_minus_2: MetricPattern1<StoredI8>,
    pub constant_minus_3: MetricPattern1<StoredI8>,
    pub constant_minus_4: MetricPattern1<StoredI8>,
}

impl MetricsTree_Constants {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            constant_0: MetricPattern1::new(client.clone(), "constant_0".to_string()),
            constant_1: MetricPattern1::new(client.clone(), "constant_1".to_string()),
            constant_2: MetricPattern1::new(client.clone(), "constant_2".to_string()),
            constant_3: MetricPattern1::new(client.clone(), "constant_3".to_string()),
            constant_4: MetricPattern1::new(client.clone(), "constant_4".to_string()),
            constant_20: MetricPattern1::new(client.clone(), "constant_20".to_string()),
            constant_30: MetricPattern1::new(client.clone(), "constant_30".to_string()),
            constant_38_2: MetricPattern1::new(client.clone(), "constant_38_2".to_string()),
            constant_50: MetricPattern1::new(client.clone(), "constant_50".to_string()),
            constant_61_8: MetricPattern1::new(client.clone(), "constant_61_8".to_string()),
            constant_70: MetricPattern1::new(client.clone(), "constant_70".to_string()),
            constant_80: MetricPattern1::new(client.clone(), "constant_80".to_string()),
            constant_100: MetricPattern1::new(client.clone(), "constant_100".to_string()),
            constant_600: MetricPattern1::new(client.clone(), "constant_600".to_string()),
            constant_minus_1: MetricPattern1::new(client.clone(), "constant_minus_1".to_string()),
            constant_minus_2: MetricPattern1::new(client.clone(), "constant_minus_2".to_string()),
            constant_minus_3: MetricPattern1::new(client.clone(), "constant_minus_3".to_string()),
            constant_minus_4: MetricPattern1::new(client.clone(), "constant_minus_4".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes {
    pub address: MetricsTree_Indexes_Address,
    pub height: MetricsTree_Indexes_Height,
    pub difficultyepoch: MetricsTree_Indexes_Difficultyepoch,
    pub halvingepoch: MetricsTree_Indexes_Halvingepoch,
    pub dateindex: MetricsTree_Indexes_Dateindex,
    pub weekindex: MetricsTree_Indexes_Weekindex,
    pub monthindex: MetricsTree_Indexes_Monthindex,
    pub quarterindex: MetricsTree_Indexes_Quarterindex,
    pub semesterindex: MetricsTree_Indexes_Semesterindex,
    pub yearindex: MetricsTree_Indexes_Yearindex,
    pub decadeindex: MetricsTree_Indexes_Decadeindex,
    pub txindex: MetricsTree_Indexes_Txindex,
    pub txinindex: MetricsTree_Indexes_Txinindex,
    pub txoutindex: MetricsTree_Indexes_Txoutindex,
}

impl MetricsTree_Indexes {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            address: MetricsTree_Indexes_Address::new(client.clone(), format!("{base_path}_address")),
            height: MetricsTree_Indexes_Height::new(client.clone(), format!("{base_path}_height")),
            difficultyepoch: MetricsTree_Indexes_Difficultyepoch::new(client.clone(), format!("{base_path}_difficultyepoch")),
            halvingepoch: MetricsTree_Indexes_Halvingepoch::new(client.clone(), format!("{base_path}_halvingepoch")),
            dateindex: MetricsTree_Indexes_Dateindex::new(client.clone(), format!("{base_path}_dateindex")),
            weekindex: MetricsTree_Indexes_Weekindex::new(client.clone(), format!("{base_path}_weekindex")),
            monthindex: MetricsTree_Indexes_Monthindex::new(client.clone(), format!("{base_path}_monthindex")),
            quarterindex: MetricsTree_Indexes_Quarterindex::new(client.clone(), format!("{base_path}_quarterindex")),
            semesterindex: MetricsTree_Indexes_Semesterindex::new(client.clone(), format!("{base_path}_semesterindex")),
            yearindex: MetricsTree_Indexes_Yearindex::new(client.clone(), format!("{base_path}_yearindex")),
            decadeindex: MetricsTree_Indexes_Decadeindex::new(client.clone(), format!("{base_path}_decadeindex")),
            txindex: MetricsTree_Indexes_Txindex::new(client.clone(), format!("{base_path}_txindex")),
            txinindex: MetricsTree_Indexes_Txinindex::new(client.clone(), format!("{base_path}_txinindex")),
            txoutindex: MetricsTree_Indexes_Txoutindex::new(client.clone(), format!("{base_path}_txoutindex")),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Address {
    pub p2pk33: MetricsTree_Indexes_Address_P2pk33,
    pub p2pk65: MetricsTree_Indexes_Address_P2pk65,
    pub p2pkh: MetricsTree_Indexes_Address_P2pkh,
    pub p2sh: MetricsTree_Indexes_Address_P2sh,
    pub p2tr: MetricsTree_Indexes_Address_P2tr,
    pub p2wpkh: MetricsTree_Indexes_Address_P2wpkh,
    pub p2wsh: MetricsTree_Indexes_Address_P2wsh,
    pub p2a: MetricsTree_Indexes_Address_P2a,
    pub p2ms: MetricsTree_Indexes_Address_P2ms,
    pub empty: MetricsTree_Indexes_Address_Empty,
    pub unknown: MetricsTree_Indexes_Address_Unknown,
    pub opreturn: MetricsTree_Indexes_Address_Opreturn,
}

impl MetricsTree_Indexes_Address {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            p2pk33: MetricsTree_Indexes_Address_P2pk33::new(client.clone(), format!("{base_path}_p2pk33")),
            p2pk65: MetricsTree_Indexes_Address_P2pk65::new(client.clone(), format!("{base_path}_p2pk65")),
            p2pkh: MetricsTree_Indexes_Address_P2pkh::new(client.clone(), format!("{base_path}_p2pkh")),
            p2sh: MetricsTree_Indexes_Address_P2sh::new(client.clone(), format!("{base_path}_p2sh")),
            p2tr: MetricsTree_Indexes_Address_P2tr::new(client.clone(), format!("{base_path}_p2tr")),
            p2wpkh: MetricsTree_Indexes_Address_P2wpkh::new(client.clone(), format!("{base_path}_p2wpkh")),
            p2wsh: MetricsTree_Indexes_Address_P2wsh::new(client.clone(), format!("{base_path}_p2wsh")),
            p2a: MetricsTree_Indexes_Address_P2a::new(client.clone(), format!("{base_path}_p2a")),
            p2ms: MetricsTree_Indexes_Address_P2ms::new(client.clone(), format!("{base_path}_p2ms")),
            empty: MetricsTree_Indexes_Address_Empty::new(client.clone(), format!("{base_path}_empty")),
            unknown: MetricsTree_Indexes_Address_Unknown::new(client.clone(), format!("{base_path}_unknown")),
            opreturn: MetricsTree_Indexes_Address_Opreturn::new(client.clone(), format!("{base_path}_opreturn")),
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
pub struct MetricsTree_Indexes_Height {
    pub identity: MetricPattern11<Height>,
    pub dateindex: MetricPattern11<DateIndex>,
    pub difficultyepoch: MetricPattern11<DifficultyEpoch>,
    pub halvingepoch: MetricPattern11<HalvingEpoch>,
    pub txindex_count: MetricPattern11<StoredU64>,
}

impl MetricsTree_Indexes_Height {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern11::new(client.clone(), "height".to_string()),
            dateindex: MetricPattern11::new(client.clone(), "dateindex".to_string()),
            difficultyepoch: MetricPattern11::new(client.clone(), "difficultyepoch".to_string()),
            halvingepoch: MetricPattern11::new(client.clone(), "halvingepoch".to_string()),
            txindex_count: MetricPattern11::new(client.clone(), "txindex_count".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Difficultyepoch {
    pub identity: MetricPattern8<DifficultyEpoch>,
    pub first_height: MetricPattern8<Height>,
    pub height_count: MetricPattern8<StoredU64>,
}

impl MetricsTree_Indexes_Difficultyepoch {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern8::new(client.clone(), "difficultyepoch".to_string()),
            first_height: MetricPattern8::new(client.clone(), "first_height".to_string()),
            height_count: MetricPattern8::new(client.clone(), "height_count".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Halvingepoch {
    pub identity: MetricPattern10<HalvingEpoch>,
    pub first_height: MetricPattern10<Height>,
}

impl MetricsTree_Indexes_Halvingepoch {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern10::new(client.clone(), "halvingepoch".to_string()),
            first_height: MetricPattern10::new(client.clone(), "first_height".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Dateindex {
    pub identity: MetricPattern6<DateIndex>,
    pub date: MetricPattern6<Date>,
    pub first_height: MetricPattern6<Height>,
    pub height_count: MetricPattern6<StoredU64>,
    pub weekindex: MetricPattern6<WeekIndex>,
    pub monthindex: MetricPattern6<MonthIndex>,
}

impl MetricsTree_Indexes_Dateindex {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern6::new(client.clone(), "dateindex".to_string()),
            date: MetricPattern6::new(client.clone(), "date".to_string()),
            first_height: MetricPattern6::new(client.clone(), "first_height".to_string()),
            height_count: MetricPattern6::new(client.clone(), "height_count".to_string()),
            weekindex: MetricPattern6::new(client.clone(), "weekindex".to_string()),
            monthindex: MetricPattern6::new(client.clone(), "monthindex".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Weekindex {
    pub identity: MetricPattern29<WeekIndex>,
    pub date: MetricPattern29<Date>,
    pub first_dateindex: MetricPattern29<DateIndex>,
    pub dateindex_count: MetricPattern29<StoredU64>,
}

impl MetricsTree_Indexes_Weekindex {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern29::new(client.clone(), "weekindex".to_string()),
            date: MetricPattern29::new(client.clone(), "date".to_string()),
            first_dateindex: MetricPattern29::new(client.clone(), "first_dateindex".to_string()),
            dateindex_count: MetricPattern29::new(client.clone(), "dateindex_count".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Monthindex {
    pub identity: MetricPattern13<MonthIndex>,
    pub date: MetricPattern13<Date>,
    pub first_dateindex: MetricPattern13<DateIndex>,
    pub dateindex_count: MetricPattern13<StoredU64>,
    pub quarterindex: MetricPattern13<QuarterIndex>,
    pub semesterindex: MetricPattern13<SemesterIndex>,
    pub yearindex: MetricPattern13<YearIndex>,
}

impl MetricsTree_Indexes_Monthindex {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern13::new(client.clone(), "monthindex".to_string()),
            date: MetricPattern13::new(client.clone(), "date".to_string()),
            first_dateindex: MetricPattern13::new(client.clone(), "first_dateindex".to_string()),
            dateindex_count: MetricPattern13::new(client.clone(), "dateindex_count".to_string()),
            quarterindex: MetricPattern13::new(client.clone(), "quarterindex".to_string()),
            semesterindex: MetricPattern13::new(client.clone(), "semesterindex".to_string()),
            yearindex: MetricPattern13::new(client.clone(), "yearindex".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Quarterindex {
    pub identity: MetricPattern25<QuarterIndex>,
    pub date: MetricPattern25<Date>,
    pub first_monthindex: MetricPattern25<MonthIndex>,
    pub monthindex_count: MetricPattern25<StoredU64>,
}

impl MetricsTree_Indexes_Quarterindex {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern25::new(client.clone(), "quarterindex".to_string()),
            date: MetricPattern25::new(client.clone(), "date".to_string()),
            first_monthindex: MetricPattern25::new(client.clone(), "first_monthindex".to_string()),
            monthindex_count: MetricPattern25::new(client.clone(), "monthindex_count".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Semesterindex {
    pub identity: MetricPattern26<SemesterIndex>,
    pub date: MetricPattern26<Date>,
    pub first_monthindex: MetricPattern26<MonthIndex>,
    pub monthindex_count: MetricPattern26<StoredU64>,
}

impl MetricsTree_Indexes_Semesterindex {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern26::new(client.clone(), "semesterindex".to_string()),
            date: MetricPattern26::new(client.clone(), "date".to_string()),
            first_monthindex: MetricPattern26::new(client.clone(), "first_monthindex".to_string()),
            monthindex_count: MetricPattern26::new(client.clone(), "monthindex_count".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Yearindex {
    pub identity: MetricPattern30<YearIndex>,
    pub date: MetricPattern30<Date>,
    pub first_monthindex: MetricPattern30<MonthIndex>,
    pub monthindex_count: MetricPattern30<StoredU64>,
    pub decadeindex: MetricPattern30<DecadeIndex>,
}

impl MetricsTree_Indexes_Yearindex {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern30::new(client.clone(), "yearindex".to_string()),
            date: MetricPattern30::new(client.clone(), "date".to_string()),
            first_monthindex: MetricPattern30::new(client.clone(), "first_monthindex".to_string()),
            monthindex_count: MetricPattern30::new(client.clone(), "monthindex_count".to_string()),
            decadeindex: MetricPattern30::new(client.clone(), "decadeindex".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Decadeindex {
    pub identity: MetricPattern7<DecadeIndex>,
    pub date: MetricPattern7<Date>,
    pub first_yearindex: MetricPattern7<YearIndex>,
    pub yearindex_count: MetricPattern7<StoredU64>,
}

impl MetricsTree_Indexes_Decadeindex {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern7::new(client.clone(), "decadeindex".to_string()),
            date: MetricPattern7::new(client.clone(), "date".to_string()),
            first_yearindex: MetricPattern7::new(client.clone(), "first_yearindex".to_string()),
            yearindex_count: MetricPattern7::new(client.clone(), "yearindex_count".to_string()),
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
pub struct MetricsTree_Market {
    pub ath: MetricsTree_Market_Ath,
    pub lookback: MetricsTree_Market_Lookback,
    pub returns: MetricsTree_Market_Returns,
    pub volatility: MetricsTree_Market_Volatility,
    pub range: MetricsTree_Market_Range,
    pub moving_average: MetricsTree_Market_MovingAverage,
    pub dca: MetricsTree_Market_Dca,
    pub indicators: MetricsTree_Market_Indicators,
}

impl MetricsTree_Market {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            ath: MetricsTree_Market_Ath::new(client.clone(), format!("{base_path}_ath")),
            lookback: MetricsTree_Market_Lookback::new(client.clone(), format!("{base_path}_lookback")),
            returns: MetricsTree_Market_Returns::new(client.clone(), format!("{base_path}_returns")),
            volatility: MetricsTree_Market_Volatility::new(client.clone(), format!("{base_path}_volatility")),
            range: MetricsTree_Market_Range::new(client.clone(), format!("{base_path}_range")),
            moving_average: MetricsTree_Market_MovingAverage::new(client.clone(), format!("{base_path}_moving_average")),
            dca: MetricsTree_Market_Dca::new(client.clone(), format!("{base_path}_dca")),
            indicators: MetricsTree_Market_Indicators::new(client.clone(), format!("{base_path}_indicators")),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Market_Ath {
    pub price_ath: DollarsSatsPattern,
    pub price_drawdown: MetricPattern3<StoredF32>,
    pub days_since_price_ath: MetricPattern4<StoredU16>,
    pub years_since_price_ath: MetricPattern4<StoredF32>,
    pub max_days_between_price_aths: MetricPattern4<StoredU16>,
    pub max_years_between_price_aths: MetricPattern4<StoredF32>,
}

impl MetricsTree_Market_Ath {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            price_ath: DollarsSatsPattern::new(client.clone(), "price_ath".to_string()),
            price_drawdown: MetricPattern3::new(client.clone(), "price_drawdown".to_string()),
            days_since_price_ath: MetricPattern4::new(client.clone(), "days_since_price_ath".to_string()),
            years_since_price_ath: MetricPattern4::new(client.clone(), "years_since_price_ath".to_string()),
            max_days_between_price_aths: MetricPattern4::new(client.clone(), "max_days_between_price_aths".to_string()),
            max_years_between_price_aths: MetricPattern4::new(client.clone(), "max_years_between_price_aths".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Market_Lookback {
    pub _1d: DollarsSatsPattern2,
    pub _1w: DollarsSatsPattern2,
    pub _1m: DollarsSatsPattern2,
    pub _3m: DollarsSatsPattern2,
    pub _6m: DollarsSatsPattern2,
    pub _1y: DollarsSatsPattern2,
    pub _2y: DollarsSatsPattern2,
    pub _3y: DollarsSatsPattern2,
    pub _4y: DollarsSatsPattern2,
    pub _5y: DollarsSatsPattern2,
    pub _6y: DollarsSatsPattern2,
    pub _8y: DollarsSatsPattern2,
    pub _10y: DollarsSatsPattern2,
}

impl MetricsTree_Market_Lookback {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _1d: DollarsSatsPattern2::new(client.clone(), "price_1d_ago".to_string()),
            _1w: DollarsSatsPattern2::new(client.clone(), "price_1w_ago".to_string()),
            _1m: DollarsSatsPattern2::new(client.clone(), "price_1m_ago".to_string()),
            _3m: DollarsSatsPattern2::new(client.clone(), "price_3m_ago".to_string()),
            _6m: DollarsSatsPattern2::new(client.clone(), "price_6m_ago".to_string()),
            _1y: DollarsSatsPattern2::new(client.clone(), "price_1y_ago".to_string()),
            _2y: DollarsSatsPattern2::new(client.clone(), "price_2y_ago".to_string()),
            _3y: DollarsSatsPattern2::new(client.clone(), "price_3y_ago".to_string()),
            _4y: DollarsSatsPattern2::new(client.clone(), "price_4y_ago".to_string()),
            _5y: DollarsSatsPattern2::new(client.clone(), "price_5y_ago".to_string()),
            _6y: DollarsSatsPattern2::new(client.clone(), "price_6y_ago".to_string()),
            _8y: DollarsSatsPattern2::new(client.clone(), "price_8y_ago".to_string()),
            _10y: DollarsSatsPattern2::new(client.clone(), "price_10y_ago".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Market_Returns {
    pub price_returns: MetricsTree_Market_Returns_PriceReturns,
    pub cagr: _10y2y3y4y5y6y8yPattern,
    pub _1d_returns_1w_sd: SdSmaPattern,
    pub _1d_returns_1m_sd: SdSmaPattern,
    pub _1d_returns_1y_sd: SdSmaPattern,
    pub downside_returns: MetricPattern6<StoredF32>,
    pub downside_1w_sd: SdSmaPattern,
    pub downside_1m_sd: SdSmaPattern,
    pub downside_1y_sd: SdSmaPattern,
}

impl MetricsTree_Market_Returns {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            price_returns: MetricsTree_Market_Returns_PriceReturns::new(client.clone(), format!("{base_path}_price_returns")),
            cagr: _10y2y3y4y5y6y8yPattern::new(client.clone(), "cagr".to_string()),
            _1d_returns_1w_sd: SdSmaPattern::new(client.clone(), "1d_returns_1w_sd".to_string()),
            _1d_returns_1m_sd: SdSmaPattern::new(client.clone(), "1d_returns_1m_sd".to_string()),
            _1d_returns_1y_sd: SdSmaPattern::new(client.clone(), "1d_returns_1y_sd".to_string()),
            downside_returns: MetricPattern6::new(client.clone(), "downside_returns".to_string()),
            downside_1w_sd: SdSmaPattern::new(client.clone(), "downside_1w_sd".to_string()),
            downside_1m_sd: SdSmaPattern::new(client.clone(), "downside_1m_sd".to_string()),
            downside_1y_sd: SdSmaPattern::new(client.clone(), "downside_1y_sd".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Market_Returns_PriceReturns {
    pub _1d: MetricPattern4<StoredF32>,
    pub _1w: MetricPattern4<StoredF32>,
    pub _1m: MetricPattern4<StoredF32>,
    pub _3m: MetricPattern4<StoredF32>,
    pub _6m: MetricPattern4<StoredF32>,
    pub _1y: MetricPattern4<StoredF32>,
    pub _2y: MetricPattern4<StoredF32>,
    pub _3y: MetricPattern4<StoredF32>,
    pub _4y: MetricPattern4<StoredF32>,
    pub _5y: MetricPattern4<StoredF32>,
    pub _6y: MetricPattern4<StoredF32>,
    pub _8y: MetricPattern4<StoredF32>,
    pub _10y: MetricPattern4<StoredF32>,
}

impl MetricsTree_Market_Returns_PriceReturns {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _1d: MetricPattern4::new(client.clone(), "1d_price_returns".to_string()),
            _1w: MetricPattern4::new(client.clone(), "1w_price_returns".to_string()),
            _1m: MetricPattern4::new(client.clone(), "1m_price_returns".to_string()),
            _3m: MetricPattern4::new(client.clone(), "3m_price_returns".to_string()),
            _6m: MetricPattern4::new(client.clone(), "6m_price_returns".to_string()),
            _1y: MetricPattern4::new(client.clone(), "1y_price_returns".to_string()),
            _2y: MetricPattern4::new(client.clone(), "2y_price_returns".to_string()),
            _3y: MetricPattern4::new(client.clone(), "3y_price_returns".to_string()),
            _4y: MetricPattern4::new(client.clone(), "4y_price_returns".to_string()),
            _5y: MetricPattern4::new(client.clone(), "5y_price_returns".to_string()),
            _6y: MetricPattern4::new(client.clone(), "6y_price_returns".to_string()),
            _8y: MetricPattern4::new(client.clone(), "8y_price_returns".to_string()),
            _10y: MetricPattern4::new(client.clone(), "10y_price_returns".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Market_Volatility {
    pub price_1w_volatility: MetricPattern4<StoredF32>,
    pub price_1m_volatility: MetricPattern4<StoredF32>,
    pub price_1y_volatility: MetricPattern4<StoredF32>,
    pub sharpe_1w: MetricPattern6<StoredF32>,
    pub sharpe_1m: MetricPattern6<StoredF32>,
    pub sharpe_1y: MetricPattern6<StoredF32>,
    pub sortino_1w: MetricPattern6<StoredF32>,
    pub sortino_1m: MetricPattern6<StoredF32>,
    pub sortino_1y: MetricPattern6<StoredF32>,
}

impl MetricsTree_Market_Volatility {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            price_1w_volatility: MetricPattern4::new(client.clone(), "price_1w_volatility".to_string()),
            price_1m_volatility: MetricPattern4::new(client.clone(), "price_1m_volatility".to_string()),
            price_1y_volatility: MetricPattern4::new(client.clone(), "price_1y_volatility".to_string()),
            sharpe_1w: MetricPattern6::new(client.clone(), "sharpe_1w".to_string()),
            sharpe_1m: MetricPattern6::new(client.clone(), "sharpe_1m".to_string()),
            sharpe_1y: MetricPattern6::new(client.clone(), "sharpe_1y".to_string()),
            sortino_1w: MetricPattern6::new(client.clone(), "sortino_1w".to_string()),
            sortino_1m: MetricPattern6::new(client.clone(), "sortino_1m".to_string()),
            sortino_1y: MetricPattern6::new(client.clone(), "sortino_1y".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Market_Range {
    pub price_1w_min: DollarsSatsPattern2,
    pub price_1w_max: DollarsSatsPattern2,
    pub price_2w_min: DollarsSatsPattern2,
    pub price_2w_max: DollarsSatsPattern2,
    pub price_1m_min: DollarsSatsPattern2,
    pub price_1m_max: DollarsSatsPattern2,
    pub price_1y_min: DollarsSatsPattern2,
    pub price_1y_max: DollarsSatsPattern2,
    pub price_true_range: MetricPattern6<StoredF32>,
    pub price_true_range_2w_sum: MetricPattern6<StoredF32>,
    pub price_2w_choppiness_index: MetricPattern4<StoredF32>,
}

impl MetricsTree_Market_Range {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            price_1w_min: DollarsSatsPattern2::new(client.clone(), "price_1w_min".to_string()),
            price_1w_max: DollarsSatsPattern2::new(client.clone(), "price_1w_max".to_string()),
            price_2w_min: DollarsSatsPattern2::new(client.clone(), "price_2w_min".to_string()),
            price_2w_max: DollarsSatsPattern2::new(client.clone(), "price_2w_max".to_string()),
            price_1m_min: DollarsSatsPattern2::new(client.clone(), "price_1m_min".to_string()),
            price_1m_max: DollarsSatsPattern2::new(client.clone(), "price_1m_max".to_string()),
            price_1y_min: DollarsSatsPattern2::new(client.clone(), "price_1y_min".to_string()),
            price_1y_max: DollarsSatsPattern2::new(client.clone(), "price_1y_max".to_string()),
            price_true_range: MetricPattern6::new(client.clone(), "price_true_range".to_string()),
            price_true_range_2w_sum: MetricPattern6::new(client.clone(), "price_true_range_2w_sum".to_string()),
            price_2w_choppiness_index: MetricPattern4::new(client.clone(), "price_2w_choppiness_index".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Market_MovingAverage {
    pub price_1w_sma: PriceRatioPattern,
    pub price_8d_sma: PriceRatioPattern,
    pub price_13d_sma: PriceRatioPattern,
    pub price_21d_sma: PriceRatioPattern,
    pub price_1m_sma: PriceRatioPattern,
    pub price_34d_sma: PriceRatioPattern,
    pub price_55d_sma: PriceRatioPattern,
    pub price_89d_sma: PriceRatioPattern,
    pub price_111d_sma: PriceRatioPattern,
    pub price_144d_sma: PriceRatioPattern,
    pub price_200d_sma: PriceRatioPattern,
    pub price_350d_sma: PriceRatioPattern,
    pub price_1y_sma: PriceRatioPattern,
    pub price_2y_sma: PriceRatioPattern,
    pub price_200w_sma: PriceRatioPattern,
    pub price_4y_sma: PriceRatioPattern,
    pub price_1w_ema: PriceRatioPattern,
    pub price_8d_ema: PriceRatioPattern,
    pub price_12d_ema: PriceRatioPattern,
    pub price_13d_ema: PriceRatioPattern,
    pub price_21d_ema: PriceRatioPattern,
    pub price_26d_ema: PriceRatioPattern,
    pub price_1m_ema: PriceRatioPattern,
    pub price_34d_ema: PriceRatioPattern,
    pub price_55d_ema: PriceRatioPattern,
    pub price_89d_ema: PriceRatioPattern,
    pub price_144d_ema: PriceRatioPattern,
    pub price_200d_ema: PriceRatioPattern,
    pub price_1y_ema: PriceRatioPattern,
    pub price_2y_ema: PriceRatioPattern,
    pub price_200w_ema: PriceRatioPattern,
    pub price_4y_ema: PriceRatioPattern,
    pub price_200d_sma_x2_4: DollarsSatsPattern2,
    pub price_200d_sma_x0_8: DollarsSatsPattern2,
    pub price_350d_sma_x2: DollarsSatsPattern2,
}

impl MetricsTree_Market_MovingAverage {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            price_1w_sma: PriceRatioPattern::new(client.clone(), "price_1w_sma".to_string()),
            price_8d_sma: PriceRatioPattern::new(client.clone(), "price_8d_sma".to_string()),
            price_13d_sma: PriceRatioPattern::new(client.clone(), "price_13d_sma".to_string()),
            price_21d_sma: PriceRatioPattern::new(client.clone(), "price_21d_sma".to_string()),
            price_1m_sma: PriceRatioPattern::new(client.clone(), "price_1m_sma".to_string()),
            price_34d_sma: PriceRatioPattern::new(client.clone(), "price_34d_sma".to_string()),
            price_55d_sma: PriceRatioPattern::new(client.clone(), "price_55d_sma".to_string()),
            price_89d_sma: PriceRatioPattern::new(client.clone(), "price_89d_sma".to_string()),
            price_111d_sma: PriceRatioPattern::new(client.clone(), "price_111d_sma".to_string()),
            price_144d_sma: PriceRatioPattern::new(client.clone(), "price_144d_sma".to_string()),
            price_200d_sma: PriceRatioPattern::new(client.clone(), "price_200d_sma".to_string()),
            price_350d_sma: PriceRatioPattern::new(client.clone(), "price_350d_sma".to_string()),
            price_1y_sma: PriceRatioPattern::new(client.clone(), "price_1y_sma".to_string()),
            price_2y_sma: PriceRatioPattern::new(client.clone(), "price_2y_sma".to_string()),
            price_200w_sma: PriceRatioPattern::new(client.clone(), "price_200w_sma".to_string()),
            price_4y_sma: PriceRatioPattern::new(client.clone(), "price_4y_sma".to_string()),
            price_1w_ema: PriceRatioPattern::new(client.clone(), "price_1w_ema".to_string()),
            price_8d_ema: PriceRatioPattern::new(client.clone(), "price_8d_ema".to_string()),
            price_12d_ema: PriceRatioPattern::new(client.clone(), "price_12d_ema".to_string()),
            price_13d_ema: PriceRatioPattern::new(client.clone(), "price_13d_ema".to_string()),
            price_21d_ema: PriceRatioPattern::new(client.clone(), "price_21d_ema".to_string()),
            price_26d_ema: PriceRatioPattern::new(client.clone(), "price_26d_ema".to_string()),
            price_1m_ema: PriceRatioPattern::new(client.clone(), "price_1m_ema".to_string()),
            price_34d_ema: PriceRatioPattern::new(client.clone(), "price_34d_ema".to_string()),
            price_55d_ema: PriceRatioPattern::new(client.clone(), "price_55d_ema".to_string()),
            price_89d_ema: PriceRatioPattern::new(client.clone(), "price_89d_ema".to_string()),
            price_144d_ema: PriceRatioPattern::new(client.clone(), "price_144d_ema".to_string()),
            price_200d_ema: PriceRatioPattern::new(client.clone(), "price_200d_ema".to_string()),
            price_1y_ema: PriceRatioPattern::new(client.clone(), "price_1y_ema".to_string()),
            price_2y_ema: PriceRatioPattern::new(client.clone(), "price_2y_ema".to_string()),
            price_200w_ema: PriceRatioPattern::new(client.clone(), "price_200w_ema".to_string()),
            price_4y_ema: PriceRatioPattern::new(client.clone(), "price_4y_ema".to_string()),
            price_200d_sma_x2_4: DollarsSatsPattern2::new(client.clone(), "price_200d_sma_x2_4".to_string()),
            price_200d_sma_x0_8: DollarsSatsPattern2::new(client.clone(), "price_200d_sma_x0_8".to_string()),
            price_350d_sma_x2: DollarsSatsPattern2::new(client.clone(), "price_350d_sma_x2".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Market_Dca {
    pub period_stack: _10y1m1w1y2y3m3y4y5y6m6y8yPattern3,
    pub period_average_price: MetricsTree_Market_Dca_PeriodAveragePrice,
    pub period_returns: _10y1m1w1y2y3m3y4y5y6m6y8yPattern2<StoredF32>,
    pub period_cagr: _10y2y3y4y5y6y8yPattern,
    pub period_days_in_profit: _10y1m1w1y2y3m3y4y5y6m6y8yPattern2<StoredU32>,
    pub period_days_in_loss: _10y1m1w1y2y3m3y4y5y6m6y8yPattern2<StoredU32>,
    pub period_min_return: _10y1m1w1y2y3m3y4y5y6m6y8yPattern2<StoredF32>,
    pub period_max_return: _10y1m1w1y2y3m3y4y5y6m6y8yPattern2<StoredF32>,
    pub period_lump_sum_stack: _10y1m1w1y2y3m3y4y5y6m6y8yPattern3,
    pub period_lump_sum_returns: _10y1m1w1y2y3m3y4y5y6m6y8yPattern2<StoredF32>,
    pub period_lump_sum_days_in_profit: _10y1m1w1y2y3m3y4y5y6m6y8yPattern2<StoredU32>,
    pub period_lump_sum_days_in_loss: _10y1m1w1y2y3m3y4y5y6m6y8yPattern2<StoredU32>,
    pub period_lump_sum_min_return: _10y1m1w1y2y3m3y4y5y6m6y8yPattern2<StoredF32>,
    pub period_lump_sum_max_return: _10y1m1w1y2y3m3y4y5y6m6y8yPattern2<StoredF32>,
    pub class_stack: MetricsTree_Market_Dca_ClassStack,
    pub class_average_price: MetricsTree_Market_Dca_ClassAveragePrice,
    pub class_returns: _201520162017201820192020202120222023202420252026Pattern2<StoredF32>,
    pub class_days_in_profit: MetricsTree_Market_Dca_ClassDaysInProfit,
    pub class_days_in_loss: MetricsTree_Market_Dca_ClassDaysInLoss,
    pub class_min_return: MetricsTree_Market_Dca_ClassMinReturn,
    pub class_max_return: MetricsTree_Market_Dca_ClassMaxReturn,
}

impl MetricsTree_Market_Dca {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            period_stack: _10y1m1w1y2y3m3y4y5y6m6y8yPattern3::new(client.clone(), "dca_stack".to_string()),
            period_average_price: MetricsTree_Market_Dca_PeriodAveragePrice::new(client.clone(), format!("{base_path}_period_average_price")),
            period_returns: _10y1m1w1y2y3m3y4y5y6m6y8yPattern2::new(client.clone(), "dca_returns".to_string()),
            period_cagr: _10y2y3y4y5y6y8yPattern::new(client.clone(), "dca_cagr".to_string()),
            period_days_in_profit: _10y1m1w1y2y3m3y4y5y6m6y8yPattern2::new(client.clone(), "dca_days_in_profit".to_string()),
            period_days_in_loss: _10y1m1w1y2y3m3y4y5y6m6y8yPattern2::new(client.clone(), "dca_days_in_loss".to_string()),
            period_min_return: _10y1m1w1y2y3m3y4y5y6m6y8yPattern2::new(client.clone(), "dca_min_return".to_string()),
            period_max_return: _10y1m1w1y2y3m3y4y5y6m6y8yPattern2::new(client.clone(), "dca_max_return".to_string()),
            period_lump_sum_stack: _10y1m1w1y2y3m3y4y5y6m6y8yPattern3::new(client.clone(), "lump_sum_stack".to_string()),
            period_lump_sum_returns: _10y1m1w1y2y3m3y4y5y6m6y8yPattern2::new(client.clone(), "lump_sum_returns".to_string()),
            period_lump_sum_days_in_profit: _10y1m1w1y2y3m3y4y5y6m6y8yPattern2::new(client.clone(), "lump_sum_days_in_profit".to_string()),
            period_lump_sum_days_in_loss: _10y1m1w1y2y3m3y4y5y6m6y8yPattern2::new(client.clone(), "lump_sum_days_in_loss".to_string()),
            period_lump_sum_min_return: _10y1m1w1y2y3m3y4y5y6m6y8yPattern2::new(client.clone(), "lump_sum_min_return".to_string()),
            period_lump_sum_max_return: _10y1m1w1y2y3m3y4y5y6m6y8yPattern2::new(client.clone(), "lump_sum_max_return".to_string()),
            class_stack: MetricsTree_Market_Dca_ClassStack::new(client.clone(), format!("{base_path}_class_stack")),
            class_average_price: MetricsTree_Market_Dca_ClassAveragePrice::new(client.clone(), format!("{base_path}_class_average_price")),
            class_returns: _201520162017201820192020202120222023202420252026Pattern2::new(client.clone(), "dca_class".to_string()),
            class_days_in_profit: MetricsTree_Market_Dca_ClassDaysInProfit::new(client.clone(), format!("{base_path}_class_days_in_profit")),
            class_days_in_loss: MetricsTree_Market_Dca_ClassDaysInLoss::new(client.clone(), format!("{base_path}_class_days_in_loss")),
            class_min_return: MetricsTree_Market_Dca_ClassMinReturn::new(client.clone(), format!("{base_path}_class_min_return")),
            class_max_return: MetricsTree_Market_Dca_ClassMaxReturn::new(client.clone(), format!("{base_path}_class_max_return")),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Market_Dca_PeriodAveragePrice {
    pub _1w: DollarsSatsPattern2,
    pub _1m: DollarsSatsPattern2,
    pub _3m: DollarsSatsPattern2,
    pub _6m: DollarsSatsPattern2,
    pub _1y: DollarsSatsPattern2,
    pub _2y: DollarsSatsPattern2,
    pub _3y: DollarsSatsPattern2,
    pub _4y: DollarsSatsPattern2,
    pub _5y: DollarsSatsPattern2,
    pub _6y: DollarsSatsPattern2,
    pub _8y: DollarsSatsPattern2,
    pub _10y: DollarsSatsPattern2,
}

impl MetricsTree_Market_Dca_PeriodAveragePrice {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _1w: DollarsSatsPattern2::new(client.clone(), "1w_dca_average_price".to_string()),
            _1m: DollarsSatsPattern2::new(client.clone(), "1m_dca_average_price".to_string()),
            _3m: DollarsSatsPattern2::new(client.clone(), "3m_dca_average_price".to_string()),
            _6m: DollarsSatsPattern2::new(client.clone(), "6m_dca_average_price".to_string()),
            _1y: DollarsSatsPattern2::new(client.clone(), "1y_dca_average_price".to_string()),
            _2y: DollarsSatsPattern2::new(client.clone(), "2y_dca_average_price".to_string()),
            _3y: DollarsSatsPattern2::new(client.clone(), "3y_dca_average_price".to_string()),
            _4y: DollarsSatsPattern2::new(client.clone(), "4y_dca_average_price".to_string()),
            _5y: DollarsSatsPattern2::new(client.clone(), "5y_dca_average_price".to_string()),
            _6y: DollarsSatsPattern2::new(client.clone(), "6y_dca_average_price".to_string()),
            _8y: DollarsSatsPattern2::new(client.clone(), "8y_dca_average_price".to_string()),
            _10y: DollarsSatsPattern2::new(client.clone(), "10y_dca_average_price".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Market_Dca_ClassStack {
    pub _2015: BitcoinDollarsSatsPattern5,
    pub _2016: BitcoinDollarsSatsPattern5,
    pub _2017: BitcoinDollarsSatsPattern5,
    pub _2018: BitcoinDollarsSatsPattern5,
    pub _2019: BitcoinDollarsSatsPattern5,
    pub _2020: BitcoinDollarsSatsPattern5,
    pub _2021: BitcoinDollarsSatsPattern5,
    pub _2022: BitcoinDollarsSatsPattern5,
    pub _2023: BitcoinDollarsSatsPattern5,
    pub _2024: BitcoinDollarsSatsPattern5,
    pub _2025: BitcoinDollarsSatsPattern5,
    pub _2026: BitcoinDollarsSatsPattern5,
}

impl MetricsTree_Market_Dca_ClassStack {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _2015: BitcoinDollarsSatsPattern5::new(client.clone(), "dca_class_2015_stack".to_string()),
            _2016: BitcoinDollarsSatsPattern5::new(client.clone(), "dca_class_2016_stack".to_string()),
            _2017: BitcoinDollarsSatsPattern5::new(client.clone(), "dca_class_2017_stack".to_string()),
            _2018: BitcoinDollarsSatsPattern5::new(client.clone(), "dca_class_2018_stack".to_string()),
            _2019: BitcoinDollarsSatsPattern5::new(client.clone(), "dca_class_2019_stack".to_string()),
            _2020: BitcoinDollarsSatsPattern5::new(client.clone(), "dca_class_2020_stack".to_string()),
            _2021: BitcoinDollarsSatsPattern5::new(client.clone(), "dca_class_2021_stack".to_string()),
            _2022: BitcoinDollarsSatsPattern5::new(client.clone(), "dca_class_2022_stack".to_string()),
            _2023: BitcoinDollarsSatsPattern5::new(client.clone(), "dca_class_2023_stack".to_string()),
            _2024: BitcoinDollarsSatsPattern5::new(client.clone(), "dca_class_2024_stack".to_string()),
            _2025: BitcoinDollarsSatsPattern5::new(client.clone(), "dca_class_2025_stack".to_string()),
            _2026: BitcoinDollarsSatsPattern5::new(client.clone(), "dca_class_2026_stack".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Market_Dca_ClassAveragePrice {
    pub _2015: DollarsSatsPattern2,
    pub _2016: DollarsSatsPattern2,
    pub _2017: DollarsSatsPattern2,
    pub _2018: DollarsSatsPattern2,
    pub _2019: DollarsSatsPattern2,
    pub _2020: DollarsSatsPattern2,
    pub _2021: DollarsSatsPattern2,
    pub _2022: DollarsSatsPattern2,
    pub _2023: DollarsSatsPattern2,
    pub _2024: DollarsSatsPattern2,
    pub _2025: DollarsSatsPattern2,
    pub _2026: DollarsSatsPattern2,
}

impl MetricsTree_Market_Dca_ClassAveragePrice {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _2015: DollarsSatsPattern2::new(client.clone(), "dca_class_2015_average_price".to_string()),
            _2016: DollarsSatsPattern2::new(client.clone(), "dca_class_2016_average_price".to_string()),
            _2017: DollarsSatsPattern2::new(client.clone(), "dca_class_2017_average_price".to_string()),
            _2018: DollarsSatsPattern2::new(client.clone(), "dca_class_2018_average_price".to_string()),
            _2019: DollarsSatsPattern2::new(client.clone(), "dca_class_2019_average_price".to_string()),
            _2020: DollarsSatsPattern2::new(client.clone(), "dca_class_2020_average_price".to_string()),
            _2021: DollarsSatsPattern2::new(client.clone(), "dca_class_2021_average_price".to_string()),
            _2022: DollarsSatsPattern2::new(client.clone(), "dca_class_2022_average_price".to_string()),
            _2023: DollarsSatsPattern2::new(client.clone(), "dca_class_2023_average_price".to_string()),
            _2024: DollarsSatsPattern2::new(client.clone(), "dca_class_2024_average_price".to_string()),
            _2025: DollarsSatsPattern2::new(client.clone(), "dca_class_2025_average_price".to_string()),
            _2026: DollarsSatsPattern2::new(client.clone(), "dca_class_2026_average_price".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Market_Dca_ClassDaysInProfit {
    pub _2015: MetricPattern4<StoredU32>,
    pub _2016: MetricPattern4<StoredU32>,
    pub _2017: MetricPattern4<StoredU32>,
    pub _2018: MetricPattern4<StoredU32>,
    pub _2019: MetricPattern4<StoredU32>,
    pub _2020: MetricPattern4<StoredU32>,
    pub _2021: MetricPattern4<StoredU32>,
    pub _2022: MetricPattern4<StoredU32>,
    pub _2023: MetricPattern4<StoredU32>,
    pub _2024: MetricPattern4<StoredU32>,
    pub _2025: MetricPattern4<StoredU32>,
    pub _2026: MetricPattern4<StoredU32>,
}

impl MetricsTree_Market_Dca_ClassDaysInProfit {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _2015: MetricPattern4::new(client.clone(), "dca_class_2015_days_in_profit".to_string()),
            _2016: MetricPattern4::new(client.clone(), "dca_class_2016_days_in_profit".to_string()),
            _2017: MetricPattern4::new(client.clone(), "dca_class_2017_days_in_profit".to_string()),
            _2018: MetricPattern4::new(client.clone(), "dca_class_2018_days_in_profit".to_string()),
            _2019: MetricPattern4::new(client.clone(), "dca_class_2019_days_in_profit".to_string()),
            _2020: MetricPattern4::new(client.clone(), "dca_class_2020_days_in_profit".to_string()),
            _2021: MetricPattern4::new(client.clone(), "dca_class_2021_days_in_profit".to_string()),
            _2022: MetricPattern4::new(client.clone(), "dca_class_2022_days_in_profit".to_string()),
            _2023: MetricPattern4::new(client.clone(), "dca_class_2023_days_in_profit".to_string()),
            _2024: MetricPattern4::new(client.clone(), "dca_class_2024_days_in_profit".to_string()),
            _2025: MetricPattern4::new(client.clone(), "dca_class_2025_days_in_profit".to_string()),
            _2026: MetricPattern4::new(client.clone(), "dca_class_2026_days_in_profit".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Market_Dca_ClassDaysInLoss {
    pub _2015: MetricPattern4<StoredU32>,
    pub _2016: MetricPattern4<StoredU32>,
    pub _2017: MetricPattern4<StoredU32>,
    pub _2018: MetricPattern4<StoredU32>,
    pub _2019: MetricPattern4<StoredU32>,
    pub _2020: MetricPattern4<StoredU32>,
    pub _2021: MetricPattern4<StoredU32>,
    pub _2022: MetricPattern4<StoredU32>,
    pub _2023: MetricPattern4<StoredU32>,
    pub _2024: MetricPattern4<StoredU32>,
    pub _2025: MetricPattern4<StoredU32>,
    pub _2026: MetricPattern4<StoredU32>,
}

impl MetricsTree_Market_Dca_ClassDaysInLoss {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _2015: MetricPattern4::new(client.clone(), "dca_class_2015_days_in_loss".to_string()),
            _2016: MetricPattern4::new(client.clone(), "dca_class_2016_days_in_loss".to_string()),
            _2017: MetricPattern4::new(client.clone(), "dca_class_2017_days_in_loss".to_string()),
            _2018: MetricPattern4::new(client.clone(), "dca_class_2018_days_in_loss".to_string()),
            _2019: MetricPattern4::new(client.clone(), "dca_class_2019_days_in_loss".to_string()),
            _2020: MetricPattern4::new(client.clone(), "dca_class_2020_days_in_loss".to_string()),
            _2021: MetricPattern4::new(client.clone(), "dca_class_2021_days_in_loss".to_string()),
            _2022: MetricPattern4::new(client.clone(), "dca_class_2022_days_in_loss".to_string()),
            _2023: MetricPattern4::new(client.clone(), "dca_class_2023_days_in_loss".to_string()),
            _2024: MetricPattern4::new(client.clone(), "dca_class_2024_days_in_loss".to_string()),
            _2025: MetricPattern4::new(client.clone(), "dca_class_2025_days_in_loss".to_string()),
            _2026: MetricPattern4::new(client.clone(), "dca_class_2026_days_in_loss".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Market_Dca_ClassMinReturn {
    pub _2015: MetricPattern4<StoredF32>,
    pub _2016: MetricPattern4<StoredF32>,
    pub _2017: MetricPattern4<StoredF32>,
    pub _2018: MetricPattern4<StoredF32>,
    pub _2019: MetricPattern4<StoredF32>,
    pub _2020: MetricPattern4<StoredF32>,
    pub _2021: MetricPattern4<StoredF32>,
    pub _2022: MetricPattern4<StoredF32>,
    pub _2023: MetricPattern4<StoredF32>,
    pub _2024: MetricPattern4<StoredF32>,
    pub _2025: MetricPattern4<StoredF32>,
    pub _2026: MetricPattern4<StoredF32>,
}

impl MetricsTree_Market_Dca_ClassMinReturn {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _2015: MetricPattern4::new(client.clone(), "dca_class_2015_min_return".to_string()),
            _2016: MetricPattern4::new(client.clone(), "dca_class_2016_min_return".to_string()),
            _2017: MetricPattern4::new(client.clone(), "dca_class_2017_min_return".to_string()),
            _2018: MetricPattern4::new(client.clone(), "dca_class_2018_min_return".to_string()),
            _2019: MetricPattern4::new(client.clone(), "dca_class_2019_min_return".to_string()),
            _2020: MetricPattern4::new(client.clone(), "dca_class_2020_min_return".to_string()),
            _2021: MetricPattern4::new(client.clone(), "dca_class_2021_min_return".to_string()),
            _2022: MetricPattern4::new(client.clone(), "dca_class_2022_min_return".to_string()),
            _2023: MetricPattern4::new(client.clone(), "dca_class_2023_min_return".to_string()),
            _2024: MetricPattern4::new(client.clone(), "dca_class_2024_min_return".to_string()),
            _2025: MetricPattern4::new(client.clone(), "dca_class_2025_min_return".to_string()),
            _2026: MetricPattern4::new(client.clone(), "dca_class_2026_min_return".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Market_Dca_ClassMaxReturn {
    pub _2015: MetricPattern4<StoredF32>,
    pub _2016: MetricPattern4<StoredF32>,
    pub _2017: MetricPattern4<StoredF32>,
    pub _2018: MetricPattern4<StoredF32>,
    pub _2019: MetricPattern4<StoredF32>,
    pub _2020: MetricPattern4<StoredF32>,
    pub _2021: MetricPattern4<StoredF32>,
    pub _2022: MetricPattern4<StoredF32>,
    pub _2023: MetricPattern4<StoredF32>,
    pub _2024: MetricPattern4<StoredF32>,
    pub _2025: MetricPattern4<StoredF32>,
    pub _2026: MetricPattern4<StoredF32>,
}

impl MetricsTree_Market_Dca_ClassMaxReturn {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _2015: MetricPattern4::new(client.clone(), "dca_class_2015_max_return".to_string()),
            _2016: MetricPattern4::new(client.clone(), "dca_class_2016_max_return".to_string()),
            _2017: MetricPattern4::new(client.clone(), "dca_class_2017_max_return".to_string()),
            _2018: MetricPattern4::new(client.clone(), "dca_class_2018_max_return".to_string()),
            _2019: MetricPattern4::new(client.clone(), "dca_class_2019_max_return".to_string()),
            _2020: MetricPattern4::new(client.clone(), "dca_class_2020_max_return".to_string()),
            _2021: MetricPattern4::new(client.clone(), "dca_class_2021_max_return".to_string()),
            _2022: MetricPattern4::new(client.clone(), "dca_class_2022_max_return".to_string()),
            _2023: MetricPattern4::new(client.clone(), "dca_class_2023_max_return".to_string()),
            _2024: MetricPattern4::new(client.clone(), "dca_class_2024_max_return".to_string()),
            _2025: MetricPattern4::new(client.clone(), "dca_class_2025_max_return".to_string()),
            _2026: MetricPattern4::new(client.clone(), "dca_class_2026_max_return".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Market_Indicators {
    pub puell_multiple: MetricPattern4<StoredF32>,
    pub nvt: MetricPattern4<StoredF32>,
    pub rsi_gains: MetricPattern6<StoredF32>,
    pub rsi_losses: MetricPattern6<StoredF32>,
    pub rsi_average_gain_14d: MetricPattern6<StoredF32>,
    pub rsi_average_loss_14d: MetricPattern6<StoredF32>,
    pub rsi_14d: MetricPattern6<StoredF32>,
    pub rsi_14d_min: MetricPattern6<StoredF32>,
    pub rsi_14d_max: MetricPattern6<StoredF32>,
    pub stoch_rsi: MetricPattern6<StoredF32>,
    pub stoch_rsi_k: MetricPattern6<StoredF32>,
    pub stoch_rsi_d: MetricPattern6<StoredF32>,
    pub stoch_k: MetricPattern6<StoredF32>,
    pub stoch_d: MetricPattern6<StoredF32>,
    pub pi_cycle: MetricPattern6<StoredF32>,
    pub macd_line: MetricPattern6<StoredF32>,
    pub macd_signal: MetricPattern6<StoredF32>,
    pub macd_histogram: MetricPattern6<StoredF32>,
    pub gini: MetricPattern6<StoredF32>,
}

impl MetricsTree_Market_Indicators {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            puell_multiple: MetricPattern4::new(client.clone(), "puell_multiple".to_string()),
            nvt: MetricPattern4::new(client.clone(), "nvt".to_string()),
            rsi_gains: MetricPattern6::new(client.clone(), "rsi_gains".to_string()),
            rsi_losses: MetricPattern6::new(client.clone(), "rsi_losses".to_string()),
            rsi_average_gain_14d: MetricPattern6::new(client.clone(), "rsi_average_gain_14d".to_string()),
            rsi_average_loss_14d: MetricPattern6::new(client.clone(), "rsi_average_loss_14d".to_string()),
            rsi_14d: MetricPattern6::new(client.clone(), "rsi_14d".to_string()),
            rsi_14d_min: MetricPattern6::new(client.clone(), "rsi_14d_min".to_string()),
            rsi_14d_max: MetricPattern6::new(client.clone(), "rsi_14d_max".to_string()),
            stoch_rsi: MetricPattern6::new(client.clone(), "stoch_rsi".to_string()),
            stoch_rsi_k: MetricPattern6::new(client.clone(), "stoch_rsi_k".to_string()),
            stoch_rsi_d: MetricPattern6::new(client.clone(), "stoch_rsi_d".to_string()),
            stoch_k: MetricPattern6::new(client.clone(), "stoch_k".to_string()),
            stoch_d: MetricPattern6::new(client.clone(), "stoch_d".to_string()),
            pi_cycle: MetricPattern6::new(client.clone(), "pi_cycle".to_string()),
            macd_line: MetricPattern6::new(client.clone(), "macd_line".to_string()),
            macd_signal: MetricPattern6::new(client.clone(), "macd_signal".to_string()),
            macd_histogram: MetricPattern6::new(client.clone(), "macd_histogram".to_string()),
            gini: MetricPattern6::new(client.clone(), "gini".to_string()),
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
    pub unknown: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub blockfills: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub ultimuspool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub terrapool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub luxor: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub onethash: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub btccom: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub bitfarms: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub huobipool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub wayicn: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub canoepool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub btctop: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub bitcoincom: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub pool175btc: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub gbminers: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub axbt: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub asicminer: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub bitminter: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub bitcoinrussia: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub btcserv: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub simplecoinus: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub btcguild: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub eligius: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub ozcoin: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub eclipsemc: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub maxbtc: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub triplemining: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub coinlab: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub pool50btc: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub ghashio: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub stminingcorp: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub bitparking: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub mmpool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub polmine: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub kncminer: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub bitalo: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub f2pool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub hhtt: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub megabigpower: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub mtred: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub nmcbit: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub yourbtcnet: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub givemecoins: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub braiinspool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub antpool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub multicoinco: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub bcpoolio: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub cointerra: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub kanopool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub solock: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub ckpool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub nicehash: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub bitclub: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub bitcoinaffiliatenetwork: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub btcc: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub bwpool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub exxbw: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub bitsolo: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub bitfury: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub twentyoneinc: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub digitalbtc: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub eightbaochi: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub mybtccoinpool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub tbdice: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub hashpool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub nexious: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub bravomining: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub hotpool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub okexpool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub bcmonster: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub onehash: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub bixin: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub tatmaspool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub viabtc: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub connectbtc: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub batpool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub waterhole: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub dcexploration: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub dcex: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub btpool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub fiftyeightcoin: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub bitcoinindia: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub shawnp0wers: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub phashio: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub rigpool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub haozhuzhu: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub sevenpool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub miningkings: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub hashbx: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub dpool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub rawpool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub haominer: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub helix: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub bitcoinukraine: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub poolin: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub secretsuperstar: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub tigerpoolnet: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub sigmapoolcom: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub okpooltop: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub hummerpool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub tangpool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub bytepool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub spiderpool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub novablock: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub miningcity: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub binancepool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub minerium: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub lubiancom: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub okkong: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub aaopool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub emcdpool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub foundryusa: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub sbicrypto: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub arkpool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub purebtccom: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub marapool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub kucoinpool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub entrustcharitypool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub okminer: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub titan: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub pegapool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub btcnuggets: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub cloudhashing: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub digitalxmintsy: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub telco214: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub btcpoolparty: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub multipool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub transactioncoinmining: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub btcdig: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub trickysbtcpool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub btcmp: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub eobot: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub unomp: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub patels: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub gogreenlight: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub ekanembtc: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub canoe: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub tiger: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub onem1x: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub zulupool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub secpool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub ocean: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub whitepool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub wk057: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub futurebitapollosolo: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub carbonnegative: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub portlandhodl: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub phoenix: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub neopool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub maxipool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub bitfufupool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub luckypool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub miningdutch: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub publicpool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub miningsquared: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub innopolistech: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub btclab: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub parasite: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern,
}

impl MetricsTree_Pools_Vecs {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            unknown: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "unknown".to_string()),
            blockfills: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "blockfills".to_string()),
            ultimuspool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "ultimuspool".to_string()),
            terrapool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "terrapool".to_string()),
            luxor: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "luxor".to_string()),
            onethash: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "onethash".to_string()),
            btccom: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "btccom".to_string()),
            bitfarms: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "bitfarms".to_string()),
            huobipool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "huobipool".to_string()),
            wayicn: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "wayicn".to_string()),
            canoepool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "canoepool".to_string()),
            btctop: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "btctop".to_string()),
            bitcoincom: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "bitcoincom".to_string()),
            pool175btc: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "pool175btc".to_string()),
            gbminers: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "gbminers".to_string()),
            axbt: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "axbt".to_string()),
            asicminer: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "asicminer".to_string()),
            bitminter: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "bitminter".to_string()),
            bitcoinrussia: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "bitcoinrussia".to_string()),
            btcserv: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "btcserv".to_string()),
            simplecoinus: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "simplecoinus".to_string()),
            btcguild: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "btcguild".to_string()),
            eligius: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "eligius".to_string()),
            ozcoin: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "ozcoin".to_string()),
            eclipsemc: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "eclipsemc".to_string()),
            maxbtc: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "maxbtc".to_string()),
            triplemining: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "triplemining".to_string()),
            coinlab: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "coinlab".to_string()),
            pool50btc: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "pool50btc".to_string()),
            ghashio: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "ghashio".to_string()),
            stminingcorp: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "stminingcorp".to_string()),
            bitparking: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "bitparking".to_string()),
            mmpool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "mmpool".to_string()),
            polmine: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "polmine".to_string()),
            kncminer: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "kncminer".to_string()),
            bitalo: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "bitalo".to_string()),
            f2pool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "f2pool".to_string()),
            hhtt: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "hhtt".to_string()),
            megabigpower: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "megabigpower".to_string()),
            mtred: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "mtred".to_string()),
            nmcbit: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "nmcbit".to_string()),
            yourbtcnet: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "yourbtcnet".to_string()),
            givemecoins: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "givemecoins".to_string()),
            braiinspool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "braiinspool".to_string()),
            antpool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "antpool".to_string()),
            multicoinco: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "multicoinco".to_string()),
            bcpoolio: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "bcpoolio".to_string()),
            cointerra: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "cointerra".to_string()),
            kanopool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "kanopool".to_string()),
            solock: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "solock".to_string()),
            ckpool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "ckpool".to_string()),
            nicehash: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "nicehash".to_string()),
            bitclub: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "bitclub".to_string()),
            bitcoinaffiliatenetwork: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "bitcoinaffiliatenetwork".to_string()),
            btcc: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "btcc".to_string()),
            bwpool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "bwpool".to_string()),
            exxbw: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "exxbw".to_string()),
            bitsolo: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "bitsolo".to_string()),
            bitfury: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "bitfury".to_string()),
            twentyoneinc: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "twentyoneinc".to_string()),
            digitalbtc: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "digitalbtc".to_string()),
            eightbaochi: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "eightbaochi".to_string()),
            mybtccoinpool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "mybtccoinpool".to_string()),
            tbdice: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "tbdice".to_string()),
            hashpool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "hashpool".to_string()),
            nexious: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "nexious".to_string()),
            bravomining: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "bravomining".to_string()),
            hotpool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "hotpool".to_string()),
            okexpool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "okexpool".to_string()),
            bcmonster: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "bcmonster".to_string()),
            onehash: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "onehash".to_string()),
            bixin: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "bixin".to_string()),
            tatmaspool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "tatmaspool".to_string()),
            viabtc: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "viabtc".to_string()),
            connectbtc: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "connectbtc".to_string()),
            batpool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "batpool".to_string()),
            waterhole: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "waterhole".to_string()),
            dcexploration: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "dcexploration".to_string()),
            dcex: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "dcex".to_string()),
            btpool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "btpool".to_string()),
            fiftyeightcoin: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "fiftyeightcoin".to_string()),
            bitcoinindia: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "bitcoinindia".to_string()),
            shawnp0wers: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "shawnp0wers".to_string()),
            phashio: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "phashio".to_string()),
            rigpool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "rigpool".to_string()),
            haozhuzhu: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "haozhuzhu".to_string()),
            sevenpool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "sevenpool".to_string()),
            miningkings: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "miningkings".to_string()),
            hashbx: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "hashbx".to_string()),
            dpool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "dpool".to_string()),
            rawpool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "rawpool".to_string()),
            haominer: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "haominer".to_string()),
            helix: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "helix".to_string()),
            bitcoinukraine: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "bitcoinukraine".to_string()),
            poolin: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "poolin".to_string()),
            secretsuperstar: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "secretsuperstar".to_string()),
            tigerpoolnet: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "tigerpoolnet".to_string()),
            sigmapoolcom: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "sigmapoolcom".to_string()),
            okpooltop: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "okpooltop".to_string()),
            hummerpool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "hummerpool".to_string()),
            tangpool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "tangpool".to_string()),
            bytepool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "bytepool".to_string()),
            spiderpool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "spiderpool".to_string()),
            novablock: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "novablock".to_string()),
            miningcity: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "miningcity".to_string()),
            binancepool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "binancepool".to_string()),
            minerium: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "minerium".to_string()),
            lubiancom: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "lubiancom".to_string()),
            okkong: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "okkong".to_string()),
            aaopool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "aaopool".to_string()),
            emcdpool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "emcdpool".to_string()),
            foundryusa: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "foundryusa".to_string()),
            sbicrypto: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "sbicrypto".to_string()),
            arkpool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "arkpool".to_string()),
            purebtccom: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "purebtccom".to_string()),
            marapool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "marapool".to_string()),
            kucoinpool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "kucoinpool".to_string()),
            entrustcharitypool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "entrustcharitypool".to_string()),
            okminer: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "okminer".to_string()),
            titan: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "titan".to_string()),
            pegapool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "pegapool".to_string()),
            btcnuggets: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "btcnuggets".to_string()),
            cloudhashing: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "cloudhashing".to_string()),
            digitalxmintsy: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "digitalxmintsy".to_string()),
            telco214: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "telco214".to_string()),
            btcpoolparty: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "btcpoolparty".to_string()),
            multipool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "multipool".to_string()),
            transactioncoinmining: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "transactioncoinmining".to_string()),
            btcdig: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "btcdig".to_string()),
            trickysbtcpool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "trickysbtcpool".to_string()),
            btcmp: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "btcmp".to_string()),
            eobot: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "eobot".to_string()),
            unomp: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "unomp".to_string()),
            patels: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "patels".to_string()),
            gogreenlight: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "gogreenlight".to_string()),
            ekanembtc: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "ekanembtc".to_string()),
            canoe: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "canoe".to_string()),
            tiger: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "tiger".to_string()),
            onem1x: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "onem1x".to_string()),
            zulupool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "zulupool".to_string()),
            secpool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "secpool".to_string()),
            ocean: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "ocean".to_string()),
            whitepool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "whitepool".to_string()),
            wk057: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "wk057".to_string()),
            futurebitapollosolo: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "futurebitapollosolo".to_string()),
            carbonnegative: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "carbonnegative".to_string()),
            portlandhodl: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "portlandhodl".to_string()),
            phoenix: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "phoenix".to_string()),
            neopool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "neopool".to_string()),
            maxipool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "maxipool".to_string()),
            bitfufupool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "bitfufupool".to_string()),
            luckypool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "luckypool".to_string()),
            miningdutch: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "miningdutch".to_string()),
            publicpool: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "publicpool".to_string()),
            miningsquared: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "miningsquared".to_string()),
            innopolistech: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "innopolistech".to_string()),
            btclab: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "btclab".to_string()),
            parasite: _1m1w1y24hBlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "parasite".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Price {
    pub cents: MetricsTree_Price_Cents,
    pub usd: MetricsTree_Price_Usd,
    pub sats: OhlcSplitPattern2<OHLCSats>,
}

impl MetricsTree_Price {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            cents: MetricsTree_Price_Cents::new(client.clone(), format!("{base_path}_cents")),
            usd: MetricsTree_Price_Usd::new(client.clone(), format!("{base_path}_usd")),
            sats: OhlcSplitPattern2::new(client.clone(), "price".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Price_Cents {
    pub split: MetricsTree_Price_Cents_Split,
    pub ohlc: MetricPattern5<OHLCCentsUnsigned>,
}

impl MetricsTree_Price_Cents {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            split: MetricsTree_Price_Cents_Split::new(client.clone(), format!("{base_path}_split")),
            ohlc: MetricPattern5::new(client.clone(), "ohlc_cents".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Price_Cents_Split {
    pub open: MetricPattern5<CentsUnsigned>,
    pub high: MetricPattern5<CentsUnsigned>,
    pub low: MetricPattern5<CentsUnsigned>,
    pub close: MetricPattern5<CentsUnsigned>,
}

impl MetricsTree_Price_Cents_Split {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            open: MetricPattern5::new(client.clone(), "price_open_cents".to_string()),
            high: MetricPattern5::new(client.clone(), "price_high_cents".to_string()),
            low: MetricPattern5::new(client.clone(), "price_low_cents".to_string()),
            close: MetricPattern5::new(client.clone(), "price_close_cents".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Price_Usd {
    pub split: CloseHighLowOpenPattern2<Dollars>,
    pub ohlc: MetricPattern1<OHLCDollars>,
}

impl MetricsTree_Price_Usd {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            split: CloseHighLowOpenPattern2::new(client.clone(), "price".to_string()),
            ohlc: MetricPattern1::new(client.clone(), "price_ohlc".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Distribution {
    pub supply_state: MetricPattern11<SupplyState>,
    pub any_address_indexes: MetricsTree_Distribution_AnyAddressIndexes,
    pub addresses_data: MetricsTree_Distribution_AddressesData,
    pub utxo_cohorts: MetricsTree_Distribution_UtxoCohorts,
    pub address_cohorts: MetricsTree_Distribution_AddressCohorts,
    pub addr_count: AllP2aP2pk33P2pk65P2pkhP2shP2trP2wpkhP2wshPattern,
    pub empty_addr_count: AllP2aP2pk33P2pk65P2pkhP2shP2trP2wpkhP2wshPattern,
    pub address_activity: MetricsTree_Distribution_AddressActivity,
    pub total_addr_count: MetricsTree_Distribution_TotalAddrCount,
    pub new_addr_count: MetricsTree_Distribution_NewAddrCount,
    pub growth_rate: MetricsTree_Distribution_GrowthRate,
    pub fundedaddressindex: MetricPattern31<FundedAddressIndex>,
    pub emptyaddressindex: MetricPattern32<EmptyAddressIndex>,
}

impl MetricsTree_Distribution {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            supply_state: MetricPattern11::new(client.clone(), "supply_state".to_string()),
            any_address_indexes: MetricsTree_Distribution_AnyAddressIndexes::new(client.clone(), format!("{base_path}_any_address_indexes")),
            addresses_data: MetricsTree_Distribution_AddressesData::new(client.clone(), format!("{base_path}_addresses_data")),
            utxo_cohorts: MetricsTree_Distribution_UtxoCohorts::new(client.clone(), format!("{base_path}_utxo_cohorts")),
            address_cohorts: MetricsTree_Distribution_AddressCohorts::new(client.clone(), format!("{base_path}_address_cohorts")),
            addr_count: AllP2aP2pk33P2pk65P2pkhP2shP2trP2wpkhP2wshPattern::new(client.clone(), "addr_count".to_string()),
            empty_addr_count: AllP2aP2pk33P2pk65P2pkhP2shP2trP2wpkhP2wshPattern::new(client.clone(), "empty_addr_count".to_string()),
            address_activity: MetricsTree_Distribution_AddressActivity::new(client.clone(), format!("{base_path}_address_activity")),
            total_addr_count: MetricsTree_Distribution_TotalAddrCount::new(client.clone(), format!("{base_path}_total_addr_count")),
            new_addr_count: MetricsTree_Distribution_NewAddrCount::new(client.clone(), format!("{base_path}_new_addr_count")),
            growth_rate: MetricsTree_Distribution_GrowthRate::new(client.clone(), format!("{base_path}_growth_rate")),
            fundedaddressindex: MetricPattern31::new(client.clone(), "fundedaddressindex".to_string()),
            emptyaddressindex: MetricPattern32::new(client.clone(), "emptyaddressindex".to_string()),
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
pub struct MetricsTree_Distribution_AddressesData {
    pub funded: MetricPattern31<FundedAddressData>,
    pub empty: MetricPattern32<EmptyAddressData>,
}

impl MetricsTree_Distribution_AddressesData {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            funded: MetricPattern31::new(client.clone(), "fundedaddressdata".to_string()),
            empty: MetricPattern32::new(client.clone(), "emptyaddressdata".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Distribution_UtxoCohorts {
    pub all: MetricsTree_Distribution_UtxoCohorts_All,
    pub age_range: MetricsTree_Distribution_UtxoCohorts_AgeRange,
    pub epoch: MetricsTree_Distribution_UtxoCohorts_Epoch,
    pub year: MetricsTree_Distribution_UtxoCohorts_Year,
    pub min_age: MetricsTree_Distribution_UtxoCohorts_MinAge,
    pub ge_amount: MetricsTree_Distribution_UtxoCohorts_GeAmount,
    pub amount_range: MetricsTree_Distribution_UtxoCohorts_AmountRange,
    pub term: MetricsTree_Distribution_UtxoCohorts_Term,
    pub type_: MetricsTree_Distribution_UtxoCohorts_Type,
    pub max_age: MetricsTree_Distribution_UtxoCohorts_MaxAge,
    pub lt_amount: MetricsTree_Distribution_UtxoCohorts_LtAmount,
}

impl MetricsTree_Distribution_UtxoCohorts {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            all: MetricsTree_Distribution_UtxoCohorts_All::new(client.clone(), format!("{base_path}_all")),
            age_range: MetricsTree_Distribution_UtxoCohorts_AgeRange::new(client.clone(), format!("{base_path}_age_range")),
            epoch: MetricsTree_Distribution_UtxoCohorts_Epoch::new(client.clone(), format!("{base_path}_epoch")),
            year: MetricsTree_Distribution_UtxoCohorts_Year::new(client.clone(), format!("{base_path}_year")),
            min_age: MetricsTree_Distribution_UtxoCohorts_MinAge::new(client.clone(), format!("{base_path}_min_age")),
            ge_amount: MetricsTree_Distribution_UtxoCohorts_GeAmount::new(client.clone(), format!("{base_path}_ge_amount")),
            amount_range: MetricsTree_Distribution_UtxoCohorts_AmountRange::new(client.clone(), format!("{base_path}_amount_range")),
            term: MetricsTree_Distribution_UtxoCohorts_Term::new(client.clone(), format!("{base_path}_term")),
            type_: MetricsTree_Distribution_UtxoCohorts_Type::new(client.clone(), format!("{base_path}_type_")),
            max_age: MetricsTree_Distribution_UtxoCohorts_MaxAge::new(client.clone(), format!("{base_path}_max_age")),
            lt_amount: MetricsTree_Distribution_UtxoCohorts_LtAmount::new(client.clone(), format!("{base_path}_lt_amount")),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Distribution_UtxoCohorts_All {
    pub supply: _30dHalvedTotalPattern,
    pub outputs: UtxoPattern,
    pub activity: CoinblocksCoindaysSatblocksSatdaysSentPattern,
    pub realized: AdjustedCapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern,
    pub unrealized: GreedInvestedInvestorNegNetPainPeakSupplyTotalUnrealizedPattern,
    pub cost_basis: InvestedMaxMinPercentilesSpotPattern,
    pub relative: MetricsTree_Distribution_UtxoCohorts_All_Relative,
}

impl MetricsTree_Distribution_UtxoCohorts_All {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            supply: _30dHalvedTotalPattern::new(client.clone(), "".to_string()),
            outputs: UtxoPattern::new(client.clone(), "utxo_count".to_string()),
            activity: CoinblocksCoindaysSatblocksSatdaysSentPattern::new(client.clone(), "".to_string()),
            realized: AdjustedCapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern::new(client.clone(), "".to_string()),
            unrealized: GreedInvestedInvestorNegNetPainPeakSupplyTotalUnrealizedPattern::new(client.clone(), "".to_string()),
            cost_basis: InvestedMaxMinPercentilesSpotPattern::new(client.clone(), "".to_string()),
            relative: MetricsTree_Distribution_UtxoCohorts_All_Relative::new(client.clone(), format!("{base_path}_relative")),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Distribution_UtxoCohorts_All_Relative {
    pub supply_in_profit_rel_to_own_supply: MetricPattern1<StoredF64>,
    pub supply_in_loss_rel_to_own_supply: MetricPattern1<StoredF64>,
    pub unrealized_profit_rel_to_market_cap: MetricPattern1<StoredF32>,
    pub unrealized_loss_rel_to_market_cap: MetricPattern1<StoredF32>,
    pub neg_unrealized_loss_rel_to_market_cap: MetricPattern1<StoredF32>,
    pub net_unrealized_pnl_rel_to_market_cap: MetricPattern1<StoredF32>,
    pub nupl: MetricPattern1<StoredF32>,
    pub unrealized_profit_rel_to_own_total_unrealized_pnl: MetricPattern1<StoredF32>,
    pub unrealized_loss_rel_to_own_total_unrealized_pnl: MetricPattern1<StoredF32>,
    pub neg_unrealized_loss_rel_to_own_total_unrealized_pnl: MetricPattern1<StoredF32>,
    pub net_unrealized_pnl_rel_to_own_total_unrealized_pnl: MetricPattern1<StoredF32>,
    pub invested_capital_in_profit_pct: MetricPattern1<StoredF32>,
    pub invested_capital_in_loss_pct: MetricPattern1<StoredF32>,
    pub unrealized_peak_regret_rel_to_market_cap: MetricPattern4<StoredF32>,
}

impl MetricsTree_Distribution_UtxoCohorts_All_Relative {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            supply_in_profit_rel_to_own_supply: MetricPattern1::new(client.clone(), "supply_in_profit_rel_to_own_supply".to_string()),
            supply_in_loss_rel_to_own_supply: MetricPattern1::new(client.clone(), "supply_in_loss_rel_to_own_supply".to_string()),
            unrealized_profit_rel_to_market_cap: MetricPattern1::new(client.clone(), "unrealized_profit_rel_to_market_cap".to_string()),
            unrealized_loss_rel_to_market_cap: MetricPattern1::new(client.clone(), "unrealized_loss_rel_to_market_cap".to_string()),
            neg_unrealized_loss_rel_to_market_cap: MetricPattern1::new(client.clone(), "neg_unrealized_loss_rel_to_market_cap".to_string()),
            net_unrealized_pnl_rel_to_market_cap: MetricPattern1::new(client.clone(), "net_unrealized_pnl_rel_to_market_cap".to_string()),
            nupl: MetricPattern1::new(client.clone(), "nupl".to_string()),
            unrealized_profit_rel_to_own_total_unrealized_pnl: MetricPattern1::new(client.clone(), "unrealized_profit_rel_to_own_total_unrealized_pnl".to_string()),
            unrealized_loss_rel_to_own_total_unrealized_pnl: MetricPattern1::new(client.clone(), "unrealized_loss_rel_to_own_total_unrealized_pnl".to_string()),
            neg_unrealized_loss_rel_to_own_total_unrealized_pnl: MetricPattern1::new(client.clone(), "neg_unrealized_loss_rel_to_own_total_unrealized_pnl".to_string()),
            net_unrealized_pnl_rel_to_own_total_unrealized_pnl: MetricPattern1::new(client.clone(), "net_unrealized_pnl_rel_to_own_total_unrealized_pnl".to_string()),
            invested_capital_in_profit_pct: MetricPattern1::new(client.clone(), "invested_capital_in_profit_pct".to_string()),
            invested_capital_in_loss_pct: MetricPattern1::new(client.clone(), "invested_capital_in_loss_pct".to_string()),
            unrealized_peak_regret_rel_to_market_cap: MetricPattern4::new(client.clone(), "unrealized_peak_regret_rel_to_market_cap".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Distribution_UtxoCohorts_AgeRange {
    pub up_to_1h: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _1h_to_1d: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _1d_to_1w: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _1w_to_1m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _1m_to_2m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _2m_to_3m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _3m_to_4m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _4m_to_5m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _5m_to_6m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _6m_to_1y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _1y_to_2y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _2y_to_3y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _3y_to_4y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _4y_to_5y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _5y_to_6y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _6y_to_7y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _7y_to_8y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _8y_to_10y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _10y_to_12y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _12y_to_15y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub from_15y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern,
}

impl MetricsTree_Distribution_UtxoCohorts_AgeRange {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            up_to_1h: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "utxos_under_1h_old".to_string()),
            _1h_to_1d: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "utxos_1h_to_1d_old".to_string()),
            _1d_to_1w: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "utxos_1d_to_1w_old".to_string()),
            _1w_to_1m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "utxos_1w_to_1m_old".to_string()),
            _1m_to_2m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "utxos_1m_to_2m_old".to_string()),
            _2m_to_3m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "utxos_2m_to_3m_old".to_string()),
            _3m_to_4m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "utxos_3m_to_4m_old".to_string()),
            _4m_to_5m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "utxos_4m_to_5m_old".to_string()),
            _5m_to_6m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "utxos_5m_to_6m_old".to_string()),
            _6m_to_1y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "utxos_6m_to_1y_old".to_string()),
            _1y_to_2y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "utxos_1y_to_2y_old".to_string()),
            _2y_to_3y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "utxos_2y_to_3y_old".to_string()),
            _3y_to_4y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "utxos_3y_to_4y_old".to_string()),
            _4y_to_5y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "utxos_4y_to_5y_old".to_string()),
            _5y_to_6y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "utxos_5y_to_6y_old".to_string()),
            _6y_to_7y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "utxos_6y_to_7y_old".to_string()),
            _7y_to_8y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "utxos_7y_to_8y_old".to_string()),
            _8y_to_10y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "utxos_8y_to_10y_old".to_string()),
            _10y_to_12y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "utxos_10y_to_12y_old".to_string()),
            _12y_to_15y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "utxos_12y_to_15y_old".to_string()),
            from_15y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "utxos_over_15y_old".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Distribution_UtxoCohorts_Epoch {
    pub _0: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
    pub _1: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
    pub _2: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
    pub _3: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
    pub _4: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
}

impl MetricsTree_Distribution_UtxoCohorts_Epoch {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _0: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "epoch_0".to_string()),
            _1: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "epoch_1".to_string()),
            _2: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "epoch_2".to_string()),
            _3: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "epoch_3".to_string()),
            _4: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "epoch_4".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Distribution_UtxoCohorts_Year {
    pub _2009: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
    pub _2010: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
    pub _2011: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
    pub _2012: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
    pub _2013: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
    pub _2014: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
    pub _2015: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
    pub _2016: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
    pub _2017: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
    pub _2018: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
    pub _2019: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
    pub _2020: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
    pub _2021: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
    pub _2022: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
    pub _2023: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
    pub _2024: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
    pub _2025: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
    pub _2026: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
}

impl MetricsTree_Distribution_UtxoCohorts_Year {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _2009: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "year_2009".to_string()),
            _2010: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "year_2010".to_string()),
            _2011: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "year_2011".to_string()),
            _2012: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "year_2012".to_string()),
            _2013: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "year_2013".to_string()),
            _2014: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "year_2014".to_string()),
            _2015: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "year_2015".to_string()),
            _2016: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "year_2016".to_string()),
            _2017: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "year_2017".to_string()),
            _2018: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "year_2018".to_string()),
            _2019: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "year_2019".to_string()),
            _2020: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "year_2020".to_string()),
            _2021: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "year_2021".to_string()),
            _2022: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "year_2022".to_string()),
            _2023: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "year_2023".to_string()),
            _2024: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "year_2024".to_string()),
            _2025: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "year_2025".to_string()),
            _2026: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "year_2026".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Distribution_UtxoCohorts_MinAge {
    pub _1d: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6,
    pub _1w: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6,
    pub _1m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6,
    pub _2m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6,
    pub _3m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6,
    pub _4m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6,
    pub _5m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6,
    pub _6m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6,
    pub _1y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6,
    pub _2y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6,
    pub _3y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6,
    pub _4y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6,
    pub _5y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6,
    pub _6y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6,
    pub _7y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6,
    pub _8y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6,
    pub _10y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6,
    pub _12y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6,
}

impl MetricsTree_Distribution_UtxoCohorts_MinAge {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _1d: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6::new(client.clone(), "utxos_over_1d_old".to_string()),
            _1w: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6::new(client.clone(), "utxos_over_1w_old".to_string()),
            _1m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6::new(client.clone(), "utxos_over_1m_old".to_string()),
            _2m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6::new(client.clone(), "utxos_over_2m_old".to_string()),
            _3m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6::new(client.clone(), "utxos_over_3m_old".to_string()),
            _4m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6::new(client.clone(), "utxos_over_4m_old".to_string()),
            _5m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6::new(client.clone(), "utxos_over_5m_old".to_string()),
            _6m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6::new(client.clone(), "utxos_over_6m_old".to_string()),
            _1y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6::new(client.clone(), "utxos_over_1y_old".to_string()),
            _2y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6::new(client.clone(), "utxos_over_2y_old".to_string()),
            _3y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6::new(client.clone(), "utxos_over_3y_old".to_string()),
            _4y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6::new(client.clone(), "utxos_over_4y_old".to_string()),
            _5y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6::new(client.clone(), "utxos_over_5y_old".to_string()),
            _6y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6::new(client.clone(), "utxos_over_6y_old".to_string()),
            _7y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6::new(client.clone(), "utxos_over_7y_old".to_string()),
            _8y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6::new(client.clone(), "utxos_over_8y_old".to_string()),
            _10y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6::new(client.clone(), "utxos_over_10y_old".to_string()),
            _12y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern6::new(client.clone(), "utxos_over_12y_old".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Distribution_UtxoCohorts_GeAmount {
    pub _1sat: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4,
    pub _10sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4,
    pub _100sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4,
    pub _1k_sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4,
    pub _10k_sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4,
    pub _100k_sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4,
    pub _1m_sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4,
    pub _10m_sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4,
    pub _1btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4,
    pub _10btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4,
    pub _100btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4,
    pub _1k_btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4,
    pub _10k_btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4,
}

impl MetricsTree_Distribution_UtxoCohorts_GeAmount {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _1sat: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4::new(client.clone(), "utxos_over_1sat".to_string()),
            _10sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4::new(client.clone(), "utxos_over_10sats".to_string()),
            _100sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4::new(client.clone(), "utxos_over_100sats".to_string()),
            _1k_sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4::new(client.clone(), "utxos_over_1k_sats".to_string()),
            _10k_sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4::new(client.clone(), "utxos_over_10k_sats".to_string()),
            _100k_sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4::new(client.clone(), "utxos_over_100k_sats".to_string()),
            _1m_sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4::new(client.clone(), "utxos_over_1m_sats".to_string()),
            _10m_sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4::new(client.clone(), "utxos_over_10m_sats".to_string()),
            _1btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4::new(client.clone(), "utxos_over_1btc".to_string()),
            _10btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4::new(client.clone(), "utxos_over_10btc".to_string()),
            _100btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4::new(client.clone(), "utxos_over_100btc".to_string()),
            _1k_btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4::new(client.clone(), "utxos_over_1k_btc".to_string()),
            _10k_btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4::new(client.clone(), "utxos_over_10k_btc".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Distribution_UtxoCohorts_AmountRange {
    pub _0sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
    pub _1sat_to_10sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
    pub _10sats_to_100sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
    pub _100sats_to_1k_sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
    pub _1k_sats_to_10k_sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
    pub _10k_sats_to_100k_sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
    pub _100k_sats_to_1m_sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
    pub _1m_sats_to_10m_sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
    pub _10m_sats_to_1btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
    pub _1btc_to_10btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
    pub _10btc_to_100btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
    pub _100btc_to_1k_btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
    pub _1k_btc_to_10k_btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
    pub _10k_btc_to_100k_btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
    pub _100k_btc_or_more: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
}

impl MetricsTree_Distribution_UtxoCohorts_AmountRange {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _0sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "utxos_with_0sats".to_string()),
            _1sat_to_10sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "utxos_above_1sat_under_10sats".to_string()),
            _10sats_to_100sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "utxos_above_10sats_under_100sats".to_string()),
            _100sats_to_1k_sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "utxos_above_100sats_under_1k_sats".to_string()),
            _1k_sats_to_10k_sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "utxos_above_1k_sats_under_10k_sats".to_string()),
            _10k_sats_to_100k_sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "utxos_above_10k_sats_under_100k_sats".to_string()),
            _100k_sats_to_1m_sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "utxos_above_100k_sats_under_1m_sats".to_string()),
            _1m_sats_to_10m_sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "utxos_above_1m_sats_under_10m_sats".to_string()),
            _10m_sats_to_1btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "utxos_above_10m_sats_under_1btc".to_string()),
            _1btc_to_10btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "utxos_above_1btc_under_10btc".to_string()),
            _10btc_to_100btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "utxos_above_10btc_under_100btc".to_string()),
            _100btc_to_1k_btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "utxos_above_100btc_under_1k_btc".to_string()),
            _1k_btc_to_10k_btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "utxos_above_1k_btc_under_10k_btc".to_string()),
            _10k_btc_to_100k_btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "utxos_above_10k_btc_under_100k_btc".to_string()),
            _100k_btc_or_more: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "utxos_above_100k_btc".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Distribution_UtxoCohorts_Term {
    pub short: MetricsTree_Distribution_UtxoCohorts_Term_Short,
    pub long: MetricsTree_Distribution_UtxoCohorts_Term_Long,
}

impl MetricsTree_Distribution_UtxoCohorts_Term {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            short: MetricsTree_Distribution_UtxoCohorts_Term_Short::new(client.clone(), format!("{base_path}_short")),
            long: MetricsTree_Distribution_UtxoCohorts_Term_Long::new(client.clone(), format!("{base_path}_long")),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Distribution_UtxoCohorts_Term_Short {
    pub supply: _30dHalvedTotalPattern,
    pub outputs: UtxoPattern,
    pub activity: CoinblocksCoindaysSatblocksSatdaysSentPattern,
    pub realized: AdjustedCapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern,
    pub unrealized: GreedInvestedInvestorNegNetPainPeakSupplyTotalUnrealizedPattern,
    pub cost_basis: InvestedMaxMinPercentilesSpotPattern,
    pub relative: InvestedNegNetNuplSupplyUnrealizedPattern4,
}

impl MetricsTree_Distribution_UtxoCohorts_Term_Short {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            supply: _30dHalvedTotalPattern::new(client.clone(), "sth".to_string()),
            outputs: UtxoPattern::new(client.clone(), "sth_utxo_count".to_string()),
            activity: CoinblocksCoindaysSatblocksSatdaysSentPattern::new(client.clone(), "sth".to_string()),
            realized: AdjustedCapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern::new(client.clone(), "sth".to_string()),
            unrealized: GreedInvestedInvestorNegNetPainPeakSupplyTotalUnrealizedPattern::new(client.clone(), "sth".to_string()),
            cost_basis: InvestedMaxMinPercentilesSpotPattern::new(client.clone(), "sth".to_string()),
            relative: InvestedNegNetNuplSupplyUnrealizedPattern4::new(client.clone(), "sth".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Distribution_UtxoCohorts_Term_Long {
    pub supply: _30dHalvedTotalPattern,
    pub outputs: UtxoPattern,
    pub activity: CoinblocksCoindaysSatblocksSatdaysSentPattern,
    pub realized: CapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern2,
    pub unrealized: GreedInvestedInvestorNegNetPainPeakSupplyTotalUnrealizedPattern,
    pub cost_basis: InvestedMaxMinPercentilesSpotPattern,
    pub relative: InvestedNegNetNuplSupplyUnrealizedPattern4,
}

impl MetricsTree_Distribution_UtxoCohorts_Term_Long {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            supply: _30dHalvedTotalPattern::new(client.clone(), "lth".to_string()),
            outputs: UtxoPattern::new(client.clone(), "lth_utxo_count".to_string()),
            activity: CoinblocksCoindaysSatblocksSatdaysSentPattern::new(client.clone(), "lth".to_string()),
            realized: CapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern2::new(client.clone(), "lth".to_string()),
            unrealized: GreedInvestedInvestorNegNetPainPeakSupplyTotalUnrealizedPattern::new(client.clone(), "lth".to_string()),
            cost_basis: InvestedMaxMinPercentilesSpotPattern::new(client.clone(), "lth".to_string()),
            relative: InvestedNegNetNuplSupplyUnrealizedPattern4::new(client.clone(), "lth".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Distribution_UtxoCohorts_Type {
    pub p2pk65: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
    pub p2pk33: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
    pub p2pkh: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
    pub p2ms: ActivityCostOutputsRealizedSupplyUnrealizedPattern,
    pub p2sh: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
    pub p2wpkh: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
    pub p2wsh: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
    pub p2tr: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
    pub p2a: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
    pub unknown: ActivityCostOutputsRealizedSupplyUnrealizedPattern,
    pub empty: ActivityCostOutputsRealizedSupplyUnrealizedPattern,
}

impl MetricsTree_Distribution_UtxoCohorts_Type {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            p2pk65: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "p2pk65".to_string()),
            p2pk33: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "p2pk33".to_string()),
            p2pkh: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "p2pkh".to_string()),
            p2ms: ActivityCostOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "p2ms".to_string()),
            p2sh: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "p2sh".to_string()),
            p2wpkh: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "p2wpkh".to_string()),
            p2wsh: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "p2wsh".to_string()),
            p2tr: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "p2tr".to_string()),
            p2a: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "p2a".to_string()),
            unknown: ActivityCostOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "unknown_outputs".to_string()),
            empty: ActivityCostOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "empty_outputs".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Distribution_UtxoCohorts_MaxAge {
    pub _1w: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5,
    pub _1m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5,
    pub _2m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5,
    pub _3m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5,
    pub _4m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5,
    pub _5m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5,
    pub _6m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5,
    pub _1y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5,
    pub _2y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5,
    pub _3y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5,
    pub _4y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5,
    pub _5y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5,
    pub _6y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5,
    pub _7y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5,
    pub _8y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5,
    pub _10y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5,
    pub _12y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5,
    pub _15y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5,
}

impl MetricsTree_Distribution_UtxoCohorts_MaxAge {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _1w: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5::new(client.clone(), "utxos_under_1w_old".to_string()),
            _1m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5::new(client.clone(), "utxos_under_1m_old".to_string()),
            _2m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5::new(client.clone(), "utxos_under_2m_old".to_string()),
            _3m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5::new(client.clone(), "utxos_under_3m_old".to_string()),
            _4m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5::new(client.clone(), "utxos_under_4m_old".to_string()),
            _5m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5::new(client.clone(), "utxos_under_5m_old".to_string()),
            _6m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5::new(client.clone(), "utxos_under_6m_old".to_string()),
            _1y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5::new(client.clone(), "utxos_under_1y_old".to_string()),
            _2y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5::new(client.clone(), "utxos_under_2y_old".to_string()),
            _3y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5::new(client.clone(), "utxos_under_3y_old".to_string()),
            _4y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5::new(client.clone(), "utxos_under_4y_old".to_string()),
            _5y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5::new(client.clone(), "utxos_under_5y_old".to_string()),
            _6y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5::new(client.clone(), "utxos_under_6y_old".to_string()),
            _7y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5::new(client.clone(), "utxos_under_7y_old".to_string()),
            _8y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5::new(client.clone(), "utxos_under_8y_old".to_string()),
            _10y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5::new(client.clone(), "utxos_under_10y_old".to_string()),
            _12y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5::new(client.clone(), "utxos_under_12y_old".to_string()),
            _15y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5::new(client.clone(), "utxos_under_15y_old".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Distribution_UtxoCohorts_LtAmount {
    pub _10sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4,
    pub _100sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4,
    pub _1k_sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4,
    pub _10k_sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4,
    pub _100k_sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4,
    pub _1m_sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4,
    pub _10m_sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4,
    pub _1btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4,
    pub _10btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4,
    pub _100btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4,
    pub _1k_btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4,
    pub _10k_btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4,
    pub _100k_btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4,
}

impl MetricsTree_Distribution_UtxoCohorts_LtAmount {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _10sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4::new(client.clone(), "utxos_under_10sats".to_string()),
            _100sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4::new(client.clone(), "utxos_under_100sats".to_string()),
            _1k_sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4::new(client.clone(), "utxos_under_1k_sats".to_string()),
            _10k_sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4::new(client.clone(), "utxos_under_10k_sats".to_string()),
            _100k_sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4::new(client.clone(), "utxos_under_100k_sats".to_string()),
            _1m_sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4::new(client.clone(), "utxos_under_1m_sats".to_string()),
            _10m_sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4::new(client.clone(), "utxos_under_10m_sats".to_string()),
            _1btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4::new(client.clone(), "utxos_under_1btc".to_string()),
            _10btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4::new(client.clone(), "utxos_under_10btc".to_string()),
            _100btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4::new(client.clone(), "utxos_under_100btc".to_string()),
            _1k_btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4::new(client.clone(), "utxos_under_1k_btc".to_string()),
            _10k_btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4::new(client.clone(), "utxos_under_10k_btc".to_string()),
            _100k_btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4::new(client.clone(), "utxos_under_100k_btc".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Distribution_AddressCohorts {
    pub ge_amount: MetricsTree_Distribution_AddressCohorts_GeAmount,
    pub amount_range: MetricsTree_Distribution_AddressCohorts_AmountRange,
    pub lt_amount: MetricsTree_Distribution_AddressCohorts_LtAmount,
}

impl MetricsTree_Distribution_AddressCohorts {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            ge_amount: MetricsTree_Distribution_AddressCohorts_GeAmount::new(client.clone(), format!("{base_path}_ge_amount")),
            amount_range: MetricsTree_Distribution_AddressCohorts_AmountRange::new(client.clone(), format!("{base_path}_amount_range")),
            lt_amount: MetricsTree_Distribution_AddressCohorts_LtAmount::new(client.clone(), format!("{base_path}_lt_amount")),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Distribution_AddressCohorts_GeAmount {
    pub _1sat: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _10sats: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _100sats: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _1k_sats: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _10k_sats: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _100k_sats: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _1m_sats: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _10m_sats: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _1btc: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _10btc: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _100btc: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _1k_btc: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _10k_btc: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern,
}

impl MetricsTree_Distribution_AddressCohorts_GeAmount {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _1sat: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "addrs_over_1sat".to_string()),
            _10sats: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "addrs_over_10sats".to_string()),
            _100sats: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "addrs_over_100sats".to_string()),
            _1k_sats: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "addrs_over_1k_sats".to_string()),
            _10k_sats: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "addrs_over_10k_sats".to_string()),
            _100k_sats: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "addrs_over_100k_sats".to_string()),
            _1m_sats: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "addrs_over_1m_sats".to_string()),
            _10m_sats: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "addrs_over_10m_sats".to_string()),
            _1btc: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "addrs_over_1btc".to_string()),
            _10btc: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "addrs_over_10btc".to_string()),
            _100btc: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "addrs_over_100btc".to_string()),
            _1k_btc: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "addrs_over_1k_btc".to_string()),
            _10k_btc: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "addrs_over_10k_btc".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Distribution_AddressCohorts_AmountRange {
    pub _0sats: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _1sat_to_10sats: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _10sats_to_100sats: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _100sats_to_1k_sats: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _1k_sats_to_10k_sats: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _10k_sats_to_100k_sats: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _100k_sats_to_1m_sats: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _1m_sats_to_10m_sats: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _10m_sats_to_1btc: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _1btc_to_10btc: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _10btc_to_100btc: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _100btc_to_1k_btc: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _1k_btc_to_10k_btc: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _10k_btc_to_100k_btc: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _100k_btc_or_more: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern,
}

impl MetricsTree_Distribution_AddressCohorts_AmountRange {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _0sats: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "addrs_with_0sats".to_string()),
            _1sat_to_10sats: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "addrs_above_1sat_under_10sats".to_string()),
            _10sats_to_100sats: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "addrs_above_10sats_under_100sats".to_string()),
            _100sats_to_1k_sats: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "addrs_above_100sats_under_1k_sats".to_string()),
            _1k_sats_to_10k_sats: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "addrs_above_1k_sats_under_10k_sats".to_string()),
            _10k_sats_to_100k_sats: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "addrs_above_10k_sats_under_100k_sats".to_string()),
            _100k_sats_to_1m_sats: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "addrs_above_100k_sats_under_1m_sats".to_string()),
            _1m_sats_to_10m_sats: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "addrs_above_1m_sats_under_10m_sats".to_string()),
            _10m_sats_to_1btc: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "addrs_above_10m_sats_under_1btc".to_string()),
            _1btc_to_10btc: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "addrs_above_1btc_under_10btc".to_string()),
            _10btc_to_100btc: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "addrs_above_10btc_under_100btc".to_string()),
            _100btc_to_1k_btc: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "addrs_above_100btc_under_1k_btc".to_string()),
            _1k_btc_to_10k_btc: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "addrs_above_1k_btc_under_10k_btc".to_string()),
            _10k_btc_to_100k_btc: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "addrs_above_10k_btc_under_100k_btc".to_string()),
            _100k_btc_or_more: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "addrs_above_100k_btc".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Distribution_AddressCohorts_LtAmount {
    pub _10sats: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _100sats: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _1k_sats: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _10k_sats: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _100k_sats: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _1m_sats: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _10m_sats: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _1btc: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _10btc: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _100btc: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _1k_btc: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _10k_btc: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _100k_btc: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern,
}

impl MetricsTree_Distribution_AddressCohorts_LtAmount {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _10sats: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "addrs_under_10sats".to_string()),
            _100sats: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "addrs_under_100sats".to_string()),
            _1k_sats: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "addrs_under_1k_sats".to_string()),
            _10k_sats: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "addrs_under_10k_sats".to_string()),
            _100k_sats: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "addrs_under_100k_sats".to_string()),
            _1m_sats: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "addrs_under_1m_sats".to_string()),
            _10m_sats: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "addrs_under_10m_sats".to_string()),
            _1btc: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "addrs_under_1btc".to_string()),
            _10btc: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "addrs_under_10btc".to_string()),
            _100btc: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "addrs_under_100btc".to_string()),
            _1k_btc: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "addrs_under_1k_btc".to_string()),
            _10k_btc: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "addrs_under_10k_btc".to_string()),
            _100k_btc: ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "addrs_under_100k_btc".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Distribution_AddressActivity {
    pub all: BalanceBothReactivatedReceivingSendingPattern,
    pub p2pk65: BalanceBothReactivatedReceivingSendingPattern,
    pub p2pk33: BalanceBothReactivatedReceivingSendingPattern,
    pub p2pkh: BalanceBothReactivatedReceivingSendingPattern,
    pub p2sh: BalanceBothReactivatedReceivingSendingPattern,
    pub p2wpkh: BalanceBothReactivatedReceivingSendingPattern,
    pub p2wsh: BalanceBothReactivatedReceivingSendingPattern,
    pub p2tr: BalanceBothReactivatedReceivingSendingPattern,
    pub p2a: BalanceBothReactivatedReceivingSendingPattern,
}

impl MetricsTree_Distribution_AddressActivity {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            all: BalanceBothReactivatedReceivingSendingPattern::new(client.clone(), "address_activity".to_string()),
            p2pk65: BalanceBothReactivatedReceivingSendingPattern::new(client.clone(), "p2pk65_address_activity".to_string()),
            p2pk33: BalanceBothReactivatedReceivingSendingPattern::new(client.clone(), "p2pk33_address_activity".to_string()),
            p2pkh: BalanceBothReactivatedReceivingSendingPattern::new(client.clone(), "p2pkh_address_activity".to_string()),
            p2sh: BalanceBothReactivatedReceivingSendingPattern::new(client.clone(), "p2sh_address_activity".to_string()),
            p2wpkh: BalanceBothReactivatedReceivingSendingPattern::new(client.clone(), "p2wpkh_address_activity".to_string()),
            p2wsh: BalanceBothReactivatedReceivingSendingPattern::new(client.clone(), "p2wsh_address_activity".to_string()),
            p2tr: BalanceBothReactivatedReceivingSendingPattern::new(client.clone(), "p2tr_address_activity".to_string()),
            p2a: BalanceBothReactivatedReceivingSendingPattern::new(client.clone(), "p2a_address_activity".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Distribution_TotalAddrCount {
    pub all: MetricPattern1<StoredU64>,
    pub p2pk65: MetricPattern1<StoredU64>,
    pub p2pk33: MetricPattern1<StoredU64>,
    pub p2pkh: MetricPattern1<StoredU64>,
    pub p2sh: MetricPattern1<StoredU64>,
    pub p2wpkh: MetricPattern1<StoredU64>,
    pub p2wsh: MetricPattern1<StoredU64>,
    pub p2tr: MetricPattern1<StoredU64>,
    pub p2a: MetricPattern1<StoredU64>,
}

impl MetricsTree_Distribution_TotalAddrCount {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            all: MetricPattern1::new(client.clone(), "total_addr_count".to_string()),
            p2pk65: MetricPattern1::new(client.clone(), "p2pk65_total_addr_count".to_string()),
            p2pk33: MetricPattern1::new(client.clone(), "p2pk33_total_addr_count".to_string()),
            p2pkh: MetricPattern1::new(client.clone(), "p2pkh_total_addr_count".to_string()),
            p2sh: MetricPattern1::new(client.clone(), "p2sh_total_addr_count".to_string()),
            p2wpkh: MetricPattern1::new(client.clone(), "p2wpkh_total_addr_count".to_string()),
            p2wsh: MetricPattern1::new(client.clone(), "p2wsh_total_addr_count".to_string()),
            p2tr: MetricPattern1::new(client.clone(), "p2tr_total_addr_count".to_string()),
            p2a: MetricPattern1::new(client.clone(), "p2a_total_addr_count".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Distribution_NewAddrCount {
    pub all: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2<StoredU64>,
    pub p2pk65: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2<StoredU64>,
    pub p2pk33: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2<StoredU64>,
    pub p2pkh: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2<StoredU64>,
    pub p2sh: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2<StoredU64>,
    pub p2wpkh: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2<StoredU64>,
    pub p2wsh: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2<StoredU64>,
    pub p2tr: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2<StoredU64>,
    pub p2a: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2<StoredU64>,
}

impl MetricsTree_Distribution_NewAddrCount {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            all: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2::new(client.clone(), "new_addr_count".to_string()),
            p2pk65: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2::new(client.clone(), "p2pk65_new_addr_count".to_string()),
            p2pk33: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2::new(client.clone(), "p2pk33_new_addr_count".to_string()),
            p2pkh: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2::new(client.clone(), "p2pkh_new_addr_count".to_string()),
            p2sh: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2::new(client.clone(), "p2sh_new_addr_count".to_string()),
            p2wpkh: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2::new(client.clone(), "p2wpkh_new_addr_count".to_string()),
            p2wsh: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2::new(client.clone(), "p2wsh_new_addr_count".to_string()),
            p2tr: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2::new(client.clone(), "p2tr_new_addr_count".to_string()),
            p2a: AverageBaseCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern2::new(client.clone(), "p2a_new_addr_count".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Distribution_GrowthRate {
    pub all: AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern<StoredF32>,
    pub p2pk65: AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern<StoredF32>,
    pub p2pk33: AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern<StoredF32>,
    pub p2pkh: AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern<StoredF32>,
    pub p2sh: AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern<StoredF32>,
    pub p2wpkh: AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern<StoredF32>,
    pub p2wsh: AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern<StoredF32>,
    pub p2tr: AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern<StoredF32>,
    pub p2a: AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern<StoredF32>,
}

impl MetricsTree_Distribution_GrowthRate {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            all: AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern::new(client.clone(), "growth_rate".to_string()),
            p2pk65: AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern::new(client.clone(), "p2pk65_growth_rate".to_string()),
            p2pk33: AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern::new(client.clone(), "p2pk33_growth_rate".to_string()),
            p2pkh: AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern::new(client.clone(), "p2pkh_growth_rate".to_string()),
            p2sh: AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern::new(client.clone(), "p2sh_growth_rate".to_string()),
            p2wpkh: AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern::new(client.clone(), "p2wpkh_growth_rate".to_string()),
            p2wsh: AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern::new(client.clone(), "p2wsh_growth_rate".to_string()),
            p2tr: AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern::new(client.clone(), "p2tr_growth_rate".to_string()),
            p2a: AverageBaseMaxMedianMinPct10Pct25Pct75Pct90Pattern::new(client.clone(), "p2a_growth_rate".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Supply {
    pub circulating: MetricsTree_Supply_Circulating,
    pub burned: MetricsTree_Supply_Burned,
    pub inflation: MetricPattern4<StoredF32>,
    pub velocity: MetricsTree_Supply_Velocity,
    pub market_cap: MetricPattern1<Dollars>,
}

impl MetricsTree_Supply {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            circulating: MetricsTree_Supply_Circulating::new(client.clone(), format!("{base_path}_circulating")),
            burned: MetricsTree_Supply_Burned::new(client.clone(), format!("{base_path}_burned")),
            inflation: MetricPattern4::new(client.clone(), "inflation_rate".to_string()),
            velocity: MetricsTree_Supply_Velocity::new(client.clone(), format!("{base_path}_velocity")),
            market_cap: MetricPattern1::new(client.clone(), "market_cap".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Supply_Circulating {
    pub sats: MetricPattern3<Sats>,
    pub bitcoin: MetricPattern3<Bitcoin>,
    pub dollars: MetricPattern3<Dollars>,
}

impl MetricsTree_Supply_Circulating {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            sats: MetricPattern3::new(client.clone(), "circulating_supply".to_string()),
            bitcoin: MetricPattern3::new(client.clone(), "circulating_supply_btc".to_string()),
            dollars: MetricPattern3::new(client.clone(), "circulating_supply_usd".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Supply_Burned {
    pub opreturn: BitcoinDollarsSatsPattern3,
    pub unspendable: BitcoinDollarsSatsPattern3,
}

impl MetricsTree_Supply_Burned {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            opreturn: BitcoinDollarsSatsPattern3::new(client.clone(), "opreturn_supply".to_string()),
            unspendable: BitcoinDollarsSatsPattern3::new(client.clone(), "unspendable_supply".to_string()),
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

/// Main BRK client with metrics tree and API methods.
pub struct BrkClient {
    base: Arc<BrkClientBase>,
    metrics: MetricsTree,
}

impl BrkClient {
    /// Client version.
    pub const VERSION: &'static str = "v0.1.6";

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
    pub fn metric(&self, metric: impl Into<Metric>, index: Index) -> MetricEndpointBuilder<serde_json::Value> {
        MetricEndpointBuilder::new(
            self.base.clone(),
            Arc::from(metric.into().as_str()),
            index,
        )
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
    pub fn get_address_txs(&self, address: Address, after_txid: Option<&str>, limit: Option<i64>) -> Result<Vec<Txid>> {
        let mut query = Vec::new();
        if let Some(v) = after_txid { query.push(format!("after_txid={}", v)); }
        if let Some(v) = limit { query.push(format!("limit={}", v)); }
        let query_str = if query.is_empty() { String::new() } else { format!("?{}", query.join("&")) };
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
    pub fn get_address_confirmed_txs(&self, address: Address, after_txid: Option<&str>, limit: Option<i64>) -> Result<Vec<Txid>> {
        let mut query = Vec::new();
        if let Some(v) = after_txid { query.push(format!("after_txid={}", v)); }
        if let Some(v) = limit { query.push(format!("limit={}", v)); }
        let query_str = if query.is_empty() { String::new() } else { format!("?{}", query.join("&")) };
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
        self.base.get_json(&format!("/api/address/{address}/txs/mempool"))
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
        self.base.get_json(&format!("/api/block/{hash}/txid/{index}"))
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
        self.base.get_json(&format!("/api/block/{hash}/txs/{start_index}"))
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
    pub fn get_metric(&self, metric: Metric, index: Index, start: Option<i64>, end: Option<i64>, limit: Option<&str>, format: Option<Format>) -> Result<FormatResponse<MetricData>> {
        let mut query = Vec::new();
        if let Some(v) = start { query.push(format!("start={}", v)); }
        if let Some(v) = end { query.push(format!("end={}", v)); }
        if let Some(v) = limit { query.push(format!("limit={}", v)); }
        if let Some(v) = format { query.push(format!("format={}", v)); }
        let query_str = if query.is_empty() { String::new() } else { format!("?{}", query.join("&")) };
        let path = format!("/api/metric/{metric}/{}{}", index.serialize_long(), query_str);
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
    pub fn get_metrics(&self, metrics: Metrics, index: Index, start: Option<i64>, end: Option<i64>, limit: Option<&str>, format: Option<Format>) -> Result<FormatResponse<Vec<MetricData>>> {
        let mut query = Vec::new();
        query.push(format!("metrics={}", metrics));
        query.push(format!("index={}", index));
        if let Some(v) = start { query.push(format!("start={}", v)); }
        if let Some(v) = end { query.push(format!("end={}", v)); }
        if let Some(v) = limit { query.push(format!("limit={}", v)); }
        if let Some(v) = format { query.push(format!("format={}", v)); }
        let query_str = if query.is_empty() { String::new() } else { format!("?{}", query.join("&")) };
        let path = format!("/api/metrics/bulk{}", query_str);
        if format == Some(Format::CSV) {
            self.base.get_text(&path).map(FormatResponse::Csv)
        } else {
            self.base.get_json(&path).map(FormatResponse::Json)
        }
    }

    /// Available cost basis cohorts
    ///
    /// List available cohorts for cost basis distribution.
    ///
    /// Endpoint: `GET /api/metrics/cost-basis`
    pub fn get_cost_basis_cohorts(&self) -> Result<Vec<String>> {
        self.base.get_json(&format!("/api/metrics/cost-basis"))
    }

    /// Available cost basis dates
    ///
    /// List available dates for a cohort's cost basis distribution.
    ///
    /// Endpoint: `GET /api/metrics/cost-basis/{cohort}/dates`
    pub fn get_cost_basis_dates(&self, cohort: Cohort) -> Result<Vec<Date>> {
        self.base.get_json(&format!("/api/metrics/cost-basis/{cohort}/dates"))
    }

    /// Cost basis distribution
    ///
    /// Get the cost basis distribution for a cohort on a specific date.
    ///
    /// Query params:
    /// - `bucket`: raw (default), lin200, lin500, lin1000, log10, log50, log100
    /// - `value`: supply (default, in BTC), realized (USD), unrealized (USD)
    ///
    /// Endpoint: `GET /api/metrics/cost-basis/{cohort}/{date}`
    pub fn get_cost_basis(&self, cohort: Cohort, date: &str, bucket: Option<CostBasisBucket>, value: Option<CostBasisValue>) -> Result<serde_json::Value> {
        let mut query = Vec::new();
        if let Some(v) = bucket { query.push(format!("bucket={}", v)); }
        if let Some(v) = value { query.push(format!("value={}", v)); }
        let query_str = if query.is_empty() { String::new() } else { format!("?{}", query.join("&")) };
        let path = format!("/api/metrics/cost-basis/{cohort}/{date}{}", query_str);
        self.base.get_json(&path)
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
        if let Some(v) = page { query.push(format!("page={}", v)); }
        let query_str = if query.is_empty() { String::new() } else { format!("?{}", query.join("&")) };
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
        if let Some(v) = limit { query.push(format!("limit={}", v)); }
        let query_str = if query.is_empty() { String::new() } else { format!("?{}", query.join("&")) };
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
        self.base.get_json(&format!("/api/tx/{txid}/outspend/{vout}"))
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
        self.base.get_json(&format!("/api/v1/difficulty-adjustment"))
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
        self.base.get_json(&format!("/api/v1/mining/blocks/fee-rates/{time_period}"))
    }

    /// Block fees
    ///
    /// Get average block fees for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y
    ///
    /// *[Mempool.space docs](https://mempool.space/docs/api/rest#get-block-fees)*
    ///
    /// Endpoint: `GET /api/v1/mining/blocks/fees/{time_period}`
    pub fn get_block_fees(&self, time_period: TimePeriod) -> Result<Vec<BlockFeesEntry>> {
        self.base.get_json(&format!("/api/v1/mining/blocks/fees/{time_period}"))
    }

    /// Block rewards
    ///
    /// Get average block rewards (coinbase = subsidy + fees) for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y
    ///
    /// *[Mempool.space docs](https://mempool.space/docs/api/rest#get-block-rewards)*
    ///
    /// Endpoint: `GET /api/v1/mining/blocks/rewards/{time_period}`
    pub fn get_block_rewards(&self, time_period: TimePeriod) -> Result<Vec<BlockRewardsEntry>> {
        self.base.get_json(&format!("/api/v1/mining/blocks/rewards/{time_period}"))
    }

    /// Block sizes and weights
    ///
    /// Get average block sizes and weights for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y
    ///
    /// *[Mempool.space docs](https://mempool.space/docs/api/rest#get-sizes-weights)*
    ///
    /// Endpoint: `GET /api/v1/mining/blocks/sizes-weights/{time_period}`
    pub fn get_block_sizes_weights(&self, time_period: TimePeriod) -> Result<BlockSizesWeights> {
        self.base.get_json(&format!("/api/v1/mining/blocks/sizes-weights/{time_period}"))
    }

    /// Block by timestamp
    ///
    /// Find the block closest to a given UNIX timestamp.
    ///
    /// *[Mempool.space docs](https://mempool.space/docs/api/rest#get-block-timestamp)*
    ///
    /// Endpoint: `GET /api/v1/mining/blocks/timestamp/{timestamp}`
    pub fn get_block_by_timestamp(&self, timestamp: Timestamp) -> Result<BlockTimestamp> {
        self.base.get_json(&format!("/api/v1/mining/blocks/timestamp/{timestamp}"))
    }

    /// Difficulty adjustments (all time)
    ///
    /// Get historical difficulty adjustments including timestamp, block height, difficulty value, and percentage change.
    ///
    /// *[Mempool.space docs](https://mempool.space/docs/api/rest#get-difficulty-adjustments)*
    ///
    /// Endpoint: `GET /api/v1/mining/difficulty-adjustments`
    pub fn get_difficulty_adjustments(&self) -> Result<Vec<DifficultyAdjustmentEntry>> {
        self.base.get_json(&format!("/api/v1/mining/difficulty-adjustments"))
    }

    /// Difficulty adjustments
    ///
    /// Get historical difficulty adjustments for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y.
    ///
    /// *[Mempool.space docs](https://mempool.space/docs/api/rest#get-difficulty-adjustments)*
    ///
    /// Endpoint: `GET /api/v1/mining/difficulty-adjustments/{time_period}`
    pub fn get_difficulty_adjustments_by_period(&self, time_period: TimePeriod) -> Result<Vec<DifficultyAdjustmentEntry>> {
        self.base.get_json(&format!("/api/v1/mining/difficulty-adjustments/{time_period}"))
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
        self.base.get_json(&format!("/api/v1/mining/hashrate/{time_period}"))
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
        self.base.get_json(&format!("/api/v1/mining/pools/{time_period}"))
    }

    /// Mining reward statistics
    ///
    /// Get mining reward statistics for the last N blocks including total rewards, fees, and transaction count.
    ///
    /// *[Mempool.space docs](https://mempool.space/docs/api/rest#get-reward-stats)*
    ///
    /// Endpoint: `GET /api/v1/mining/reward-stats/{block_count}`
    pub fn get_reward_stats(&self, block_count: i64) -> Result<RewardStats> {
        self.base.get_json(&format!("/api/v1/mining/reward-stats/{block_count}"))
    }

    /// Validate address
    ///
    /// Validate a Bitcoin address and get information about its type and scriptPubKey.
    ///
    /// *[Mempool.space docs](https://mempool.space/docs/api/rest#get-address-validate)*
    ///
    /// Endpoint: `GET /api/v1/validate-address/{address}`
    pub fn validate_address(&self, address: &str) -> Result<AddressValidation> {
        self.base.get_json(&format!("/api/v1/validate-address/{address}"))
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
