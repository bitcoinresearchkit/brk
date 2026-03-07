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
        format!("/api/metric/{}/{}", self.name, self.index.name())
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

/// Builder for metric endpoint queries.
///
/// Parameterized by element type `T` and response type `D` (defaults to `MetricData<T>`).
/// For date-based indexes, use `DateMetricEndpointBuilder<T>` which sets `D = DateMetricData<T>`.
///
/// # Examples
/// ```ignore
/// let data = endpoint.fetch()?;                   // all data
/// let data = endpoint.get(5).fetch()?;             // single item
/// let data = endpoint.range(..10).fetch()?;        // first 10
/// let data = endpoint.range(100..200).fetch()?;    // range [100, 200)
/// let data = endpoint.take(10).fetch()?;           // first 10 (convenience)
/// let data = endpoint.last(10).fetch()?;           // last 10
/// let data = endpoint.skip(100).take(10).fetch()?; // iterator-style
/// ```
pub struct MetricEndpointBuilder<T, D = MetricData<T>> {
    config: EndpointConfig,
    _marker: std::marker::PhantomData<fn() -> (T, D)>,
}

/// Builder for date-based metric endpoint queries.
///
/// Like `MetricEndpointBuilder` but returns `DateMetricData` and provides
/// date-based access methods (`get_date`, `date_range`).
pub type DateMetricEndpointBuilder<T> = MetricEndpointBuilder<T, DateMetricData<T>>;

impl<T: DeserializeOwned, D: DeserializeOwned> MetricEndpointBuilder<T, D> {
    pub fn new(client: Arc<BrkClientBase>, name: Arc<str>, index: Index) -> Self {
        Self { config: EndpointConfig::new(client, name, index), _marker: std::marker::PhantomData }
    }

    /// Select a specific index position.
    pub fn get(mut self, index: usize) -> SingleItemBuilder<T, D> {
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
    pub fn range<R: RangeBounds<usize>>(mut self, range: R) -> RangeBuilder<T, D> {
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
    pub fn take(self, n: usize) -> RangeBuilder<T, D> {
        self.range(..n)
    }

    /// Take the last n items.
    pub fn last(mut self, n: usize) -> RangeBuilder<T, D> {
        if n == 0 {
            self.config.end = Some(0);
        } else {
            self.config.start = Some(-(n as i64));
        }
        RangeBuilder { config: self.config, _marker: std::marker::PhantomData }
    }

    /// Skip the first n items. Chain with `take(n)` to get a range.
    pub fn skip(mut self, n: usize) -> SkippedBuilder<T, D> {
        self.config.start = Some(n as i64);
        SkippedBuilder { config: self.config, _marker: std::marker::PhantomData }
    }

    /// Fetch all data as parsed JSON.
    pub fn fetch(self) -> Result<D> {
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

/// Date-specific methods available only on `DateMetricEndpointBuilder`.
impl<T: DeserializeOwned> MetricEndpointBuilder<T, DateMetricData<T>> {
    /// Select a specific date position (for day-precision or coarser indexes).
    pub fn get_date(self, date: Date) -> SingleItemBuilder<T, DateMetricData<T>> {
        let index = self.config.index.date_to_index(date).unwrap_or(0);
        self.get(index)
    }

    /// Select a date range (for day-precision or coarser indexes).
    pub fn date_range(self, start: Date, end: Date) -> RangeBuilder<T, DateMetricData<T>> {
        let s = self.config.index.date_to_index(start).unwrap_or(0);
        let e = self.config.index.date_to_index(end).unwrap_or(0);
        self.range(s..e)
    }

    /// Select a specific timestamp position (works for all date-based indexes including sub-daily).
    pub fn get_timestamp(self, ts: Timestamp) -> SingleItemBuilder<T, DateMetricData<T>> {
        let index = self.config.index.timestamp_to_index(ts).unwrap_or(0);
        self.get(index)
    }

    /// Select a timestamp range (works for all date-based indexes including sub-daily).
    pub fn timestamp_range(self, start: Timestamp, end: Timestamp) -> RangeBuilder<T, DateMetricData<T>> {
        let s = self.config.index.timestamp_to_index(start).unwrap_or(0);
        let e = self.config.index.timestamp_to_index(end).unwrap_or(0);
        self.range(s..e)
    }
}

/// Builder for single item access.
pub struct SingleItemBuilder<T, D = MetricData<T>> {
    config: EndpointConfig,
    _marker: std::marker::PhantomData<fn() -> (T, D)>,
}

/// Date-aware single item builder.
pub type DateSingleItemBuilder<T> = SingleItemBuilder<T, DateMetricData<T>>;

impl<T: DeserializeOwned, D: DeserializeOwned> SingleItemBuilder<T, D> {
    /// Fetch the single item.
    pub fn fetch(self) -> Result<D> {
        self.config.get_json(None)
    }

    /// Fetch the single item as CSV.
    pub fn fetch_csv(self) -> Result<String> {
        self.config.get_text(Some("csv"))
    }
}

/// Builder after calling `skip(n)`. Chain with `take(n)` to specify count.
pub struct SkippedBuilder<T, D = MetricData<T>> {
    config: EndpointConfig,
    _marker: std::marker::PhantomData<fn() -> (T, D)>,
}

/// Date-aware skipped builder.
pub type DateSkippedBuilder<T> = SkippedBuilder<T, DateMetricData<T>>;

impl<T: DeserializeOwned, D: DeserializeOwned> SkippedBuilder<T, D> {
    /// Take n items after the skipped position.
    pub fn take(mut self, n: usize) -> RangeBuilder<T, D> {
        let start = self.config.start.unwrap_or(0);
        self.config.end = Some(start + n as i64);
        RangeBuilder { config: self.config, _marker: std::marker::PhantomData }
    }

    /// Fetch from the skipped position to the end.
    pub fn fetch(self) -> Result<D> {
        self.config.get_json(None)
    }

    /// Fetch from the skipped position to the end as CSV.
    pub fn fetch_csv(self) -> Result<String> {
        self.config.get_text(Some("csv"))
    }
}

/// Builder with range fully specified.
pub struct RangeBuilder<T, D = MetricData<T>> {
    config: EndpointConfig,
    _marker: std::marker::PhantomData<fn() -> (T, D)>,
}

/// Date-aware range builder.
pub type DateRangeBuilder<T> = RangeBuilder<T, DateMetricData<T>>;

impl<T: DeserializeOwned, D: DeserializeOwned> RangeBuilder<T, D> {
    /// Fetch the range as parsed JSON.
    pub fn fetch(self) -> Result<D> {
        self.config.get_json(None)
    }

    /// Fetch the range as CSV string.
    pub fn fetch_csv(self) -> Result<String> {
        self.config.get_text(Some("csv"))
    }
}


// Static index arrays
const _I1: &[Index] = &[Index::Minute10, Index::Minute30, Index::Hour1, Index::Hour4, Index::Hour12, Index::Day1, Index::Day3, Index::Week1, Index::Month1, Index::Month3, Index::Month6, Index::Year1, Index::Year10, Index::HalvingEpoch, Index::DifficultyEpoch, Index::Height];
const _I2: &[Index] = &[Index::Minute10, Index::Minute30, Index::Hour1, Index::Hour4, Index::Hour12, Index::Day1, Index::Day3, Index::Week1, Index::Month1, Index::Month3, Index::Month6, Index::Year1, Index::Year10, Index::HalvingEpoch, Index::DifficultyEpoch];
const _I3: &[Index] = &[Index::Minute10];
const _I4: &[Index] = &[Index::Minute30];
const _I5: &[Index] = &[Index::Hour1];
const _I6: &[Index] = &[Index::Hour4];
const _I7: &[Index] = &[Index::Hour12];
const _I8: &[Index] = &[Index::Day1];
const _I9: &[Index] = &[Index::Day3];
const _I10: &[Index] = &[Index::Week1];
const _I11: &[Index] = &[Index::Month1];
const _I12: &[Index] = &[Index::Month3];
const _I13: &[Index] = &[Index::Month6];
const _I14: &[Index] = &[Index::Year1];
const _I15: &[Index] = &[Index::Year10];
const _I16: &[Index] = &[Index::HalvingEpoch];
const _I17: &[Index] = &[Index::DifficultyEpoch];
const _I18: &[Index] = &[Index::Height];
const _I19: &[Index] = &[Index::TxIndex];
const _I20: &[Index] = &[Index::TxInIndex];
const _I21: &[Index] = &[Index::TxOutIndex];
const _I22: &[Index] = &[Index::EmptyOutputIndex];
const _I23: &[Index] = &[Index::OpReturnIndex];
const _I24: &[Index] = &[Index::P2AAddressIndex];
const _I25: &[Index] = &[Index::P2MSOutputIndex];
const _I26: &[Index] = &[Index::P2PK33AddressIndex];
const _I27: &[Index] = &[Index::P2PK65AddressIndex];
const _I28: &[Index] = &[Index::P2PKHAddressIndex];
const _I29: &[Index] = &[Index::P2SHAddressIndex];
const _I30: &[Index] = &[Index::P2TRAddressIndex];
const _I31: &[Index] = &[Index::P2WPKHAddressIndex];
const _I32: &[Index] = &[Index::P2WSHAddressIndex];
const _I33: &[Index] = &[Index::UnknownOutputIndex];
const _I34: &[Index] = &[Index::FundedAddressIndex];
const _I35: &[Index] = &[Index::EmptyAddressIndex];

#[inline]
fn _ep<T: DeserializeOwned>(c: &Arc<BrkClientBase>, n: &Arc<str>, i: Index) -> MetricEndpointBuilder<T> {
    MetricEndpointBuilder::new(c.clone(), n.clone(), i)
}

#[inline]
fn _dep<T: DeserializeOwned>(c: &Arc<BrkClientBase>, n: &Arc<str>, i: Index) -> DateMetricEndpointBuilder<T> {
    DateMetricEndpointBuilder::new(c.clone(), n.clone(), i)
}

// Index accessor structs

pub struct MetricPattern1By<T> { client: Arc<BrkClientBase>, name: Arc<str>, _marker: std::marker::PhantomData<T> }
impl<T: DeserializeOwned> MetricPattern1By<T> {
    pub fn minute10(&self) -> DateMetricEndpointBuilder<T> { _dep(&self.client, &self.name, Index::Minute10) }
    pub fn minute30(&self) -> DateMetricEndpointBuilder<T> { _dep(&self.client, &self.name, Index::Minute30) }
    pub fn hour1(&self) -> DateMetricEndpointBuilder<T> { _dep(&self.client, &self.name, Index::Hour1) }
    pub fn hour4(&self) -> DateMetricEndpointBuilder<T> { _dep(&self.client, &self.name, Index::Hour4) }
    pub fn hour12(&self) -> DateMetricEndpointBuilder<T> { _dep(&self.client, &self.name, Index::Hour12) }
    pub fn day1(&self) -> DateMetricEndpointBuilder<T> { _dep(&self.client, &self.name, Index::Day1) }
    pub fn day3(&self) -> DateMetricEndpointBuilder<T> { _dep(&self.client, &self.name, Index::Day3) }
    pub fn week1(&self) -> DateMetricEndpointBuilder<T> { _dep(&self.client, &self.name, Index::Week1) }
    pub fn month1(&self) -> DateMetricEndpointBuilder<T> { _dep(&self.client, &self.name, Index::Month1) }
    pub fn month3(&self) -> DateMetricEndpointBuilder<T> { _dep(&self.client, &self.name, Index::Month3) }
    pub fn month6(&self) -> DateMetricEndpointBuilder<T> { _dep(&self.client, &self.name, Index::Month6) }
    pub fn year1(&self) -> DateMetricEndpointBuilder<T> { _dep(&self.client, &self.name, Index::Year1) }
    pub fn year10(&self) -> DateMetricEndpointBuilder<T> { _dep(&self.client, &self.name, Index::Year10) }
    pub fn halvingepoch(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::HalvingEpoch) }
    pub fn difficultyepoch(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::DifficultyEpoch) }
    pub fn height(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::Height) }
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
    pub fn minute10(&self) -> DateMetricEndpointBuilder<T> { _dep(&self.client, &self.name, Index::Minute10) }
    pub fn minute30(&self) -> DateMetricEndpointBuilder<T> { _dep(&self.client, &self.name, Index::Minute30) }
    pub fn hour1(&self) -> DateMetricEndpointBuilder<T> { _dep(&self.client, &self.name, Index::Hour1) }
    pub fn hour4(&self) -> DateMetricEndpointBuilder<T> { _dep(&self.client, &self.name, Index::Hour4) }
    pub fn hour12(&self) -> DateMetricEndpointBuilder<T> { _dep(&self.client, &self.name, Index::Hour12) }
    pub fn day1(&self) -> DateMetricEndpointBuilder<T> { _dep(&self.client, &self.name, Index::Day1) }
    pub fn day3(&self) -> DateMetricEndpointBuilder<T> { _dep(&self.client, &self.name, Index::Day3) }
    pub fn week1(&self) -> DateMetricEndpointBuilder<T> { _dep(&self.client, &self.name, Index::Week1) }
    pub fn month1(&self) -> DateMetricEndpointBuilder<T> { _dep(&self.client, &self.name, Index::Month1) }
    pub fn month3(&self) -> DateMetricEndpointBuilder<T> { _dep(&self.client, &self.name, Index::Month3) }
    pub fn month6(&self) -> DateMetricEndpointBuilder<T> { _dep(&self.client, &self.name, Index::Month6) }
    pub fn year1(&self) -> DateMetricEndpointBuilder<T> { _dep(&self.client, &self.name, Index::Year1) }
    pub fn year10(&self) -> DateMetricEndpointBuilder<T> { _dep(&self.client, &self.name, Index::Year10) }
    pub fn halvingepoch(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::HalvingEpoch) }
    pub fn difficultyepoch(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::DifficultyEpoch) }
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
    pub fn minute10(&self) -> DateMetricEndpointBuilder<T> { _dep(&self.client, &self.name, Index::Minute10) }
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
    pub fn minute30(&self) -> DateMetricEndpointBuilder<T> { _dep(&self.client, &self.name, Index::Minute30) }
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
    pub fn hour1(&self) -> DateMetricEndpointBuilder<T> { _dep(&self.client, &self.name, Index::Hour1) }
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
    pub fn hour4(&self) -> DateMetricEndpointBuilder<T> { _dep(&self.client, &self.name, Index::Hour4) }
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
    pub fn hour12(&self) -> DateMetricEndpointBuilder<T> { _dep(&self.client, &self.name, Index::Hour12) }
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
    pub fn day1(&self) -> DateMetricEndpointBuilder<T> { _dep(&self.client, &self.name, Index::Day1) }
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
    pub fn day3(&self) -> DateMetricEndpointBuilder<T> { _dep(&self.client, &self.name, Index::Day3) }
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
    pub fn week1(&self) -> DateMetricEndpointBuilder<T> { _dep(&self.client, &self.name, Index::Week1) }
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
    pub fn month1(&self) -> DateMetricEndpointBuilder<T> { _dep(&self.client, &self.name, Index::Month1) }
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
    pub fn month3(&self) -> DateMetricEndpointBuilder<T> { _dep(&self.client, &self.name, Index::Month3) }
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
    pub fn month6(&self) -> DateMetricEndpointBuilder<T> { _dep(&self.client, &self.name, Index::Month6) }
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
    pub fn year1(&self) -> DateMetricEndpointBuilder<T> { _dep(&self.client, &self.name, Index::Year1) }
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
    pub fn year10(&self) -> DateMetricEndpointBuilder<T> { _dep(&self.client, &self.name, Index::Year10) }
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
    pub fn halvingepoch(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::HalvingEpoch) }
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
    pub fn difficultyepoch(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::DifficultyEpoch) }
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
    pub fn height(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::Height) }
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
    pub fn txindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::TxIndex) }
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
    pub fn txinindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::TxInIndex) }
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
    pub fn txoutindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::TxOutIndex) }
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
    pub fn emptyoutputindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::EmptyOutputIndex) }
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
    pub fn opreturnindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::OpReturnIndex) }
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
    pub fn p2aaddressindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::P2AAddressIndex) }
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
    pub fn p2msoutputindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::P2MSOutputIndex) }
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
    pub fn p2pk33addressindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::P2PK33AddressIndex) }
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
    pub fn p2pk65addressindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::P2PK65AddressIndex) }
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
    pub fn p2pkhaddressindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::P2PKHAddressIndex) }
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
    pub fn p2shaddressindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::P2SHAddressIndex) }
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
    pub fn p2traddressindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::P2TRAddressIndex) }
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
    pub fn p2wpkhaddressindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::P2WPKHAddressIndex) }
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
    pub fn p2wshaddressindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::P2WSHAddressIndex) }
}

pub struct MetricPattern32<T> { name: Arc<str>, pub by: MetricPattern32By<T> }
impl<T: DeserializeOwned> MetricPattern32<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self { let name: Arc<str> = name.into(); Self { name: name.clone(), by: MetricPattern32By { client, name, _marker: std::marker::PhantomData } } }
    pub fn name(&self) -> &str { &self.name }
}

impl<T> AnyMetricPattern for MetricPattern32<T> { fn name(&self) -> &str { &self.name } fn indexes(&self) -> &'static [Index] { _I32 } }
impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern32<T> { fn get(&self, index: Index) -> Option<MetricEndpointBuilder<T>> { _I32.contains(&index).then(|| _ep(&self.by.client, &self.by.name, index)) } }

pub struct MetricPattern33By<T> { client: Arc<BrkClientBase>, name: Arc<str>, _marker: std::marker::PhantomData<T> }
impl<T: DeserializeOwned> MetricPattern33By<T> {
    pub fn unknownoutputindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::UnknownOutputIndex) }
}

pub struct MetricPattern33<T> { name: Arc<str>, pub by: MetricPattern33By<T> }
impl<T: DeserializeOwned> MetricPattern33<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self { let name: Arc<str> = name.into(); Self { name: name.clone(), by: MetricPattern33By { client, name, _marker: std::marker::PhantomData } } }
    pub fn name(&self) -> &str { &self.name }
}

impl<T> AnyMetricPattern for MetricPattern33<T> { fn name(&self) -> &str { &self.name } fn indexes(&self) -> &'static [Index] { _I33 } }
impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern33<T> { fn get(&self, index: Index) -> Option<MetricEndpointBuilder<T>> { _I33.contains(&index).then(|| _ep(&self.by.client, &self.by.name, index)) } }

pub struct MetricPattern34By<T> { client: Arc<BrkClientBase>, name: Arc<str>, _marker: std::marker::PhantomData<T> }
impl<T: DeserializeOwned> MetricPattern34By<T> {
    pub fn fundedaddressindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::FundedAddressIndex) }
}

pub struct MetricPattern34<T> { name: Arc<str>, pub by: MetricPattern34By<T> }
impl<T: DeserializeOwned> MetricPattern34<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self { let name: Arc<str> = name.into(); Self { name: name.clone(), by: MetricPattern34By { client, name, _marker: std::marker::PhantomData } } }
    pub fn name(&self) -> &str { &self.name }
}

impl<T> AnyMetricPattern for MetricPattern34<T> { fn name(&self) -> &str { &self.name } fn indexes(&self) -> &'static [Index] { _I34 } }
impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern34<T> { fn get(&self, index: Index) -> Option<MetricEndpointBuilder<T>> { _I34.contains(&index).then(|| _ep(&self.by.client, &self.by.name, index)) } }

pub struct MetricPattern35By<T> { client: Arc<BrkClientBase>, name: Arc<str>, _marker: std::marker::PhantomData<T> }
impl<T: DeserializeOwned> MetricPattern35By<T> {
    pub fn emptyaddressindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::EmptyAddressIndex) }
}

pub struct MetricPattern35<T> { name: Arc<str>, pub by: MetricPattern35By<T> }
impl<T: DeserializeOwned> MetricPattern35<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self { let name: Arc<str> = name.into(); Self { name: name.clone(), by: MetricPattern35By { client, name, _marker: std::marker::PhantomData } } }
    pub fn name(&self) -> &str { &self.name }
}

impl<T> AnyMetricPattern for MetricPattern35<T> { fn name(&self) -> &str { &self.name } fn indexes(&self) -> &'static [Index] { _I35 } }
impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern35<T> { fn get(&self, index: Index) -> Option<MetricEndpointBuilder<T>> { _I35.contains(&index).then(|| _ep(&self.by.client, &self.by.name, index)) } }

// Reusable pattern structs

/// Pattern struct for repeated tree structure.
pub struct CapCapitulationGrossInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprUpperValuePattern {
    pub cap_raw: MetricPattern18<CentsSats>,
    pub capitulation_flow: MetricPattern1<Dollars>,
    pub gross_pnl: CentsUsdPattern,
    pub gross_pnl_sum: _1m1w1y24hPattern<Cents>,
    pub investor_cap_raw: MetricPattern18<CentsSquaredSats>,
    pub investor_price: CentsSatsUsdPattern,
    pub investor_price_ratio: BpsRatioPattern,
    pub investor_price_ratio_percentiles: RatioPattern,
    pub loss_value_created: MetricPattern1<Cents>,
    pub loss_value_destroyed: MetricPattern1<Cents>,
    pub lower_price_band: CentsSatsUsdPattern,
    pub mvrv: MetricPattern1<StoredF32>,
    pub neg_realized_loss: MetricPattern1<Dollars>,
    pub net_pnl_change_1m: MetricPattern1<CentsSigned>,
    pub net_pnl_change_1m_rel_to_market_cap: BpsPercentRatioPattern,
    pub net_pnl_change_1m_rel_to_realized_cap: BpsPercentRatioPattern,
    pub net_realized_pnl: CumulativeHeightPattern<CentsSigned>,
    pub net_realized_pnl_ema_1w: MetricPattern1<CentsSigned>,
    pub net_realized_pnl_rel_to_realized_cap: BpsPercentRatioPattern,
    pub peak_regret: CumulativeHeightPattern<Cents>,
    pub peak_regret_rel_to_realized_cap: BpsPercentRatioPattern,
    pub profit_flow: MetricPattern1<Dollars>,
    pub profit_value_created: MetricPattern1<Cents>,
    pub profit_value_destroyed: MetricPattern1<Cents>,
    pub realized_cap: MetricPattern1<Dollars>,
    pub realized_cap_cents: MetricPattern1<Cents>,
    pub realized_cap_change_1m: MetricPattern1<CentsSigned>,
    pub realized_cap_rel_to_own_market_cap: BpsPercentRatioPattern,
    pub realized_loss: CumulativeHeightPattern<Cents>,
    pub realized_loss_ema_1w: MetricPattern1<Cents>,
    pub realized_loss_rel_to_realized_cap: BpsPercentRatioPattern,
    pub realized_loss_sum: _1m1w1y24hPattern<Cents>,
    pub realized_price: CentsSatsUsdPattern,
    pub realized_price_ratio: BpsRatioPattern,
    pub realized_price_ratio_percentiles: RatioPattern,
    pub realized_price_ratio_std_dev: RatioPattern2,
    pub realized_profit: CumulativeHeightPattern<Cents>,
    pub realized_profit_ema_1w: MetricPattern1<Cents>,
    pub realized_profit_rel_to_realized_cap: BpsPercentRatioPattern,
    pub realized_profit_sum: _1m1w1y24hPattern<Cents>,
    pub realized_profit_to_loss_ratio: _1m1w1y24hPattern<StoredF64>,
    pub sell_side_risk_ratio: _1m1w1y24hPattern2,
    pub sell_side_risk_ratio_24h_ema: _1m1wPattern2,
    pub sent_in_loss: MetricPattern1<Sats>,
    pub sent_in_loss_sum: _1m1w1y24hPattern<Sats>,
    pub sent_in_profit: MetricPattern1<Sats>,
    pub sent_in_profit_sum: _1m1w1y24hPattern<Sats>,
    pub sopr: _24hPattern<StoredF64>,
    pub sopr_24h_ema: _1m1wPattern,
    pub sopr_extended: _1m1w1yPattern<StoredF64>,
    pub upper_price_band: CentsSatsUsdPattern,
    pub value_created: MetricPattern1<Cents>,
    pub value_created_sum: _24hPattern<Cents>,
    pub value_created_sum_extended: _1m1w1yPattern<Cents>,
    pub value_destroyed: MetricPattern1<Cents>,
    pub value_destroyed_sum: _24hPattern<Cents>,
    pub value_destroyed_sum_extended: _1m1w1yPattern<Cents>,
}

impl CapCapitulationGrossInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprUpperValuePattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            cap_raw: MetricPattern18::new(client.clone(), _m(&acc, "cap_raw")),
            capitulation_flow: MetricPattern1::new(client.clone(), _m(&acc, "capitulation_flow")),
            gross_pnl: CentsUsdPattern::new(client.clone(), _m(&acc, "realized_gross_pnl")),
            gross_pnl_sum: _1m1w1y24hPattern::new(client.clone(), _m(&acc, "gross_pnl_sum")),
            investor_cap_raw: MetricPattern18::new(client.clone(), _m(&acc, "investor_cap_raw")),
            investor_price: CentsSatsUsdPattern::new(client.clone(), _m(&acc, "investor_price")),
            investor_price_ratio: BpsRatioPattern::new(client.clone(), _m(&acc, "investor_price_ratio")),
            investor_price_ratio_percentiles: RatioPattern::new(client.clone(), _m(&acc, "investor_price_ratio")),
            loss_value_created: MetricPattern1::new(client.clone(), _m(&acc, "loss_value_created")),
            loss_value_destroyed: MetricPattern1::new(client.clone(), _m(&acc, "loss_value_destroyed")),
            lower_price_band: CentsSatsUsdPattern::new(client.clone(), _m(&acc, "lower_price_band")),
            mvrv: MetricPattern1::new(client.clone(), _m(&acc, "mvrv")),
            neg_realized_loss: MetricPattern1::new(client.clone(), _m(&acc, "neg_realized_loss")),
            net_pnl_change_1m: MetricPattern1::new(client.clone(), _m(&acc, "net_pnl_change_1m")),
            net_pnl_change_1m_rel_to_market_cap: BpsPercentRatioPattern::new(client.clone(), _m(&acc, "net_pnl_change_1m_rel_to_market_cap")),
            net_pnl_change_1m_rel_to_realized_cap: BpsPercentRatioPattern::new(client.clone(), _m(&acc, "net_pnl_change_1m_rel_to_realized_cap")),
            net_realized_pnl: CumulativeHeightPattern::new(client.clone(), _m(&acc, "net_realized_pnl")),
            net_realized_pnl_ema_1w: MetricPattern1::new(client.clone(), _m(&acc, "net_realized_pnl_ema_1w")),
            net_realized_pnl_rel_to_realized_cap: BpsPercentRatioPattern::new(client.clone(), _m(&acc, "net_realized_pnl_rel_to_realized_cap")),
            peak_regret: CumulativeHeightPattern::new(client.clone(), _m(&acc, "realized_peak_regret")),
            peak_regret_rel_to_realized_cap: BpsPercentRatioPattern::new(client.clone(), _m(&acc, "realized_peak_regret_rel_to_realized_cap")),
            profit_flow: MetricPattern1::new(client.clone(), _m(&acc, "profit_flow")),
            profit_value_created: MetricPattern1::new(client.clone(), _m(&acc, "profit_value_created")),
            profit_value_destroyed: MetricPattern1::new(client.clone(), _m(&acc, "profit_value_destroyed")),
            realized_cap: MetricPattern1::new(client.clone(), _m(&acc, "realized_cap")),
            realized_cap_cents: MetricPattern1::new(client.clone(), _m(&acc, "realized_cap_cents")),
            realized_cap_change_1m: MetricPattern1::new(client.clone(), _m(&acc, "realized_cap_change_1m")),
            realized_cap_rel_to_own_market_cap: BpsPercentRatioPattern::new(client.clone(), _m(&acc, "realized_cap_rel_to_own_market_cap")),
            realized_loss: CumulativeHeightPattern::new(client.clone(), _m(&acc, "realized_loss")),
            realized_loss_ema_1w: MetricPattern1::new(client.clone(), _m(&acc, "realized_loss_ema_1w")),
            realized_loss_rel_to_realized_cap: BpsPercentRatioPattern::new(client.clone(), _m(&acc, "realized_loss_rel_to_realized_cap")),
            realized_loss_sum: _1m1w1y24hPattern::new(client.clone(), _m(&acc, "realized_loss")),
            realized_price: CentsSatsUsdPattern::new(client.clone(), _m(&acc, "realized_price")),
            realized_price_ratio: BpsRatioPattern::new(client.clone(), _m(&acc, "realized_price_ratio")),
            realized_price_ratio_percentiles: RatioPattern::new(client.clone(), _m(&acc, "realized_price_ratio")),
            realized_price_ratio_std_dev: RatioPattern2::new(client.clone(), _m(&acc, "realized_price_ratio")),
            realized_profit: CumulativeHeightPattern::new(client.clone(), _m(&acc, "realized_profit")),
            realized_profit_ema_1w: MetricPattern1::new(client.clone(), _m(&acc, "realized_profit_ema_1w")),
            realized_profit_rel_to_realized_cap: BpsPercentRatioPattern::new(client.clone(), _m(&acc, "realized_profit_rel_to_realized_cap")),
            realized_profit_sum: _1m1w1y24hPattern::new(client.clone(), _m(&acc, "realized_profit")),
            realized_profit_to_loss_ratio: _1m1w1y24hPattern::new(client.clone(), _m(&acc, "realized_profit_to_loss_ratio")),
            sell_side_risk_ratio: _1m1w1y24hPattern2::new(client.clone(), _m(&acc, "sell_side_risk_ratio")),
            sell_side_risk_ratio_24h_ema: _1m1wPattern2::new(client.clone(), _m(&acc, "sell_side_risk_ratio_24h_ema")),
            sent_in_loss: MetricPattern1::new(client.clone(), _m(&acc, "sent_in_loss")),
            sent_in_loss_sum: _1m1w1y24hPattern::new(client.clone(), _m(&acc, "sent_in_loss")),
            sent_in_profit: MetricPattern1::new(client.clone(), _m(&acc, "sent_in_profit")),
            sent_in_profit_sum: _1m1w1y24hPattern::new(client.clone(), _m(&acc, "sent_in_profit")),
            sopr: _24hPattern::new(client.clone(), _m(&acc, "sopr_24h")),
            sopr_24h_ema: _1m1wPattern::new(client.clone(), _m(&acc, "sopr_24h_ema")),
            sopr_extended: _1m1w1yPattern::new(client.clone(), _m(&acc, "sopr")),
            upper_price_band: CentsSatsUsdPattern::new(client.clone(), _m(&acc, "upper_price_band")),
            value_created: MetricPattern1::new(client.clone(), _m(&acc, "value_created")),
            value_created_sum: _24hPattern::new(client.clone(), _m(&acc, "value_created_24h")),
            value_created_sum_extended: _1m1w1yPattern::new(client.clone(), _m(&acc, "value_created")),
            value_destroyed: MetricPattern1::new(client.clone(), _m(&acc, "value_destroyed")),
            value_destroyed_sum: _24hPattern::new(client.clone(), _m(&acc, "value_destroyed_24h")),
            value_destroyed_sum_extended: _1m1w1yPattern::new(client.clone(), _m(&acc, "value_destroyed")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern {
    pub _0sd_price: CentsSatsUsdPattern,
    pub m0_5sd: MetricPattern1<StoredF32>,
    pub m0_5sd_price: CentsSatsUsdPattern,
    pub m1_5sd: MetricPattern1<StoredF32>,
    pub m1_5sd_price: CentsSatsUsdPattern,
    pub m1sd: MetricPattern1<StoredF32>,
    pub m1sd_price: CentsSatsUsdPattern,
    pub m2_5sd: MetricPattern1<StoredF32>,
    pub m2_5sd_price: CentsSatsUsdPattern,
    pub m2sd: MetricPattern1<StoredF32>,
    pub m2sd_price: CentsSatsUsdPattern,
    pub m3sd: MetricPattern1<StoredF32>,
    pub m3sd_price: CentsSatsUsdPattern,
    pub p0_5sd: MetricPattern1<StoredF32>,
    pub p0_5sd_price: CentsSatsUsdPattern,
    pub p1_5sd: MetricPattern1<StoredF32>,
    pub p1_5sd_price: CentsSatsUsdPattern,
    pub p1sd: MetricPattern1<StoredF32>,
    pub p1sd_price: CentsSatsUsdPattern,
    pub p2_5sd: MetricPattern1<StoredF32>,
    pub p2_5sd_price: CentsSatsUsdPattern,
    pub p2sd: MetricPattern1<StoredF32>,
    pub p2sd_price: CentsSatsUsdPattern,
    pub p3sd: MetricPattern1<StoredF32>,
    pub p3sd_price: CentsSatsUsdPattern,
    pub sd: MetricPattern1<StoredF32>,
    pub sma: MetricPattern1<StoredF32>,
    pub zscore: MetricPattern1<StoredF32>,
}

impl _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            _0sd_price: CentsSatsUsdPattern::new(client.clone(), _m(&acc, "0sd_4y")),
            m0_5sd: MetricPattern1::new(client.clone(), _m(&acc, "m0_5sd_4y")),
            m0_5sd_price: CentsSatsUsdPattern::new(client.clone(), _m(&acc, "m0_5sd_4y")),
            m1_5sd: MetricPattern1::new(client.clone(), _m(&acc, "m1_5sd_4y")),
            m1_5sd_price: CentsSatsUsdPattern::new(client.clone(), _m(&acc, "m1_5sd_4y")),
            m1sd: MetricPattern1::new(client.clone(), _m(&acc, "m1sd_4y")),
            m1sd_price: CentsSatsUsdPattern::new(client.clone(), _m(&acc, "m1sd_4y")),
            m2_5sd: MetricPattern1::new(client.clone(), _m(&acc, "m2_5sd_4y")),
            m2_5sd_price: CentsSatsUsdPattern::new(client.clone(), _m(&acc, "m2_5sd_4y")),
            m2sd: MetricPattern1::new(client.clone(), _m(&acc, "m2sd_4y")),
            m2sd_price: CentsSatsUsdPattern::new(client.clone(), _m(&acc, "m2sd_4y")),
            m3sd: MetricPattern1::new(client.clone(), _m(&acc, "m3sd_4y")),
            m3sd_price: CentsSatsUsdPattern::new(client.clone(), _m(&acc, "m3sd_4y")),
            p0_5sd: MetricPattern1::new(client.clone(), _m(&acc, "p0_5sd_4y")),
            p0_5sd_price: CentsSatsUsdPattern::new(client.clone(), _m(&acc, "p0_5sd_4y")),
            p1_5sd: MetricPattern1::new(client.clone(), _m(&acc, "p1_5sd_4y")),
            p1_5sd_price: CentsSatsUsdPattern::new(client.clone(), _m(&acc, "p1_5sd_4y")),
            p1sd: MetricPattern1::new(client.clone(), _m(&acc, "p1sd_4y")),
            p1sd_price: CentsSatsUsdPattern::new(client.clone(), _m(&acc, "p1sd_4y")),
            p2_5sd: MetricPattern1::new(client.clone(), _m(&acc, "p2_5sd_4y")),
            p2_5sd_price: CentsSatsUsdPattern::new(client.clone(), _m(&acc, "p2_5sd_4y")),
            p2sd: MetricPattern1::new(client.clone(), _m(&acc, "p2sd_4y")),
            p2sd_price: CentsSatsUsdPattern::new(client.clone(), _m(&acc, "p2sd_4y")),
            p3sd: MetricPattern1::new(client.clone(), _m(&acc, "p3sd_4y")),
            p3sd_price: CentsSatsUsdPattern::new(client.clone(), _m(&acc, "p3sd_4y")),
            sd: MetricPattern1::new(client.clone(), _m(&acc, "sd_4y")),
            sma: MetricPattern1::new(client.clone(), _m(&acc, "sma_4y")),
            zscore: MetricPattern1::new(client.clone(), _m(&acc, "zscore_4y")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct Pct05Pct10Pct15Pct20Pct25Pct30Pct35Pct40Pct45Pct50Pct55Pct60Pct65Pct70Pct75Pct80Pct85Pct90Pct95Pattern {
    pub pct05: CentsSatsUsdPattern,
    pub pct10: CentsSatsUsdPattern,
    pub pct15: CentsSatsUsdPattern,
    pub pct20: CentsSatsUsdPattern,
    pub pct25: CentsSatsUsdPattern,
    pub pct30: CentsSatsUsdPattern,
    pub pct35: CentsSatsUsdPattern,
    pub pct40: CentsSatsUsdPattern,
    pub pct45: CentsSatsUsdPattern,
    pub pct50: CentsSatsUsdPattern,
    pub pct55: CentsSatsUsdPattern,
    pub pct60: CentsSatsUsdPattern,
    pub pct65: CentsSatsUsdPattern,
    pub pct70: CentsSatsUsdPattern,
    pub pct75: CentsSatsUsdPattern,
    pub pct80: CentsSatsUsdPattern,
    pub pct85: CentsSatsUsdPattern,
    pub pct90: CentsSatsUsdPattern,
    pub pct95: CentsSatsUsdPattern,
}

impl Pct05Pct10Pct15Pct20Pct25Pct30Pct35Pct40Pct45Pct50Pct55Pct60Pct65Pct70Pct75Pct80Pct85Pct90Pct95Pattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            pct05: CentsSatsUsdPattern::new(client.clone(), _m(&acc, "pct05")),
            pct10: CentsSatsUsdPattern::new(client.clone(), _m(&acc, "pct10")),
            pct15: CentsSatsUsdPattern::new(client.clone(), _m(&acc, "pct15")),
            pct20: CentsSatsUsdPattern::new(client.clone(), _m(&acc, "pct20")),
            pct25: CentsSatsUsdPattern::new(client.clone(), _m(&acc, "pct25")),
            pct30: CentsSatsUsdPattern::new(client.clone(), _m(&acc, "pct30")),
            pct35: CentsSatsUsdPattern::new(client.clone(), _m(&acc, "pct35")),
            pct40: CentsSatsUsdPattern::new(client.clone(), _m(&acc, "pct40")),
            pct45: CentsSatsUsdPattern::new(client.clone(), _m(&acc, "pct45")),
            pct50: CentsSatsUsdPattern::new(client.clone(), _m(&acc, "pct50")),
            pct55: CentsSatsUsdPattern::new(client.clone(), _m(&acc, "pct55")),
            pct60: CentsSatsUsdPattern::new(client.clone(), _m(&acc, "pct60")),
            pct65: CentsSatsUsdPattern::new(client.clone(), _m(&acc, "pct65")),
            pct70: CentsSatsUsdPattern::new(client.clone(), _m(&acc, "pct70")),
            pct75: CentsSatsUsdPattern::new(client.clone(), _m(&acc, "pct75")),
            pct80: CentsSatsUsdPattern::new(client.clone(), _m(&acc, "pct80")),
            pct85: CentsSatsUsdPattern::new(client.clone(), _m(&acc, "pct85")),
            pct90: CentsSatsUsdPattern::new(client.clone(), _m(&acc, "pct90")),
            pct95: CentsSatsUsdPattern::new(client.clone(), _m(&acc, "pct95")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct MvrvNegNetRealizedSentSoprValuePattern {
    pub mvrv: MetricPattern1<StoredF32>,
    pub neg_realized_loss: MetricPattern1<Dollars>,
    pub net_realized_pnl: CumulativeHeightPattern<CentsSigned>,
    pub realized_cap: MetricPattern1<Dollars>,
    pub realized_cap_cents: MetricPattern1<Cents>,
    pub realized_cap_change_1m: MetricPattern1<CentsSigned>,
    pub realized_loss: CumulativeHeightPattern<Cents>,
    pub realized_price: CentsSatsUsdPattern,
    pub realized_price_ratio: BpsRatioPattern,
    pub realized_profit: CumulativeHeightPattern<Cents>,
    pub sent_in_loss: MetricPattern1<Sats>,
    pub sent_in_profit: MetricPattern1<Sats>,
    pub sopr: _24hPattern<StoredF64>,
    pub value_created: MetricPattern1<Cents>,
    pub value_created_sum: _24hPattern<Cents>,
    pub value_destroyed: MetricPattern1<Cents>,
    pub value_destroyed_sum: _24hPattern<Cents>,
}

impl MvrvNegNetRealizedSentSoprValuePattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            mvrv: MetricPattern1::new(client.clone(), _m(&acc, "mvrv")),
            neg_realized_loss: MetricPattern1::new(client.clone(), _m(&acc, "neg_realized_loss")),
            net_realized_pnl: CumulativeHeightPattern::new(client.clone(), _m(&acc, "net_realized_pnl")),
            realized_cap: MetricPattern1::new(client.clone(), _m(&acc, "realized_cap")),
            realized_cap_cents: MetricPattern1::new(client.clone(), _m(&acc, "realized_cap_cents")),
            realized_cap_change_1m: MetricPattern1::new(client.clone(), _m(&acc, "realized_cap_change_1m")),
            realized_loss: CumulativeHeightPattern::new(client.clone(), _m(&acc, "realized_loss")),
            realized_price: CentsSatsUsdPattern::new(client.clone(), _m(&acc, "realized_price")),
            realized_price_ratio: BpsRatioPattern::new(client.clone(), _m(&acc, "realized_price_ratio")),
            realized_profit: CumulativeHeightPattern::new(client.clone(), _m(&acc, "realized_profit")),
            sent_in_loss: MetricPattern1::new(client.clone(), _m(&acc, "sent_in_loss")),
            sent_in_profit: MetricPattern1::new(client.clone(), _m(&acc, "sent_in_profit")),
            sopr: _24hPattern::new(client.clone(), _m(&acc, "sopr_24h")),
            value_created: MetricPattern1::new(client.clone(), _m(&acc, "value_created")),
            value_created_sum: _24hPattern::new(client.clone(), _m(&acc, "value_created_24h")),
            value_destroyed: MetricPattern1::new(client.clone(), _m(&acc, "value_destroyed")),
            value_destroyed_sum: _24hPattern::new(client.clone(), _m(&acc, "value_destroyed_24h")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct BpsRatioPattern2 {
    pub bps: MetricPattern1<BasisPoints32>,
    pub ratio: MetricPattern1<StoredF32>,
    pub ratio_pct1: BpsRatioPattern,
    pub ratio_pct1_price: CentsSatsUsdPattern,
    pub ratio_pct2: BpsRatioPattern,
    pub ratio_pct2_price: CentsSatsUsdPattern,
    pub ratio_pct5: BpsRatioPattern,
    pub ratio_pct5_price: CentsSatsUsdPattern,
    pub ratio_pct95: BpsRatioPattern,
    pub ratio_pct95_price: CentsSatsUsdPattern,
    pub ratio_pct98: BpsRatioPattern,
    pub ratio_pct98_price: CentsSatsUsdPattern,
    pub ratio_pct99: BpsRatioPattern,
    pub ratio_pct99_price: CentsSatsUsdPattern,
    pub ratio_sma_1m: BpsRatioPattern,
    pub ratio_sma_1w: BpsRatioPattern,
}

impl BpsRatioPattern2 {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            bps: MetricPattern1::new(client.clone(), _m(&acc, "bps")),
            ratio: MetricPattern1::new(client.clone(), acc.clone()),
            ratio_pct1: BpsRatioPattern::new(client.clone(), _m(&acc, "pct1")),
            ratio_pct1_price: CentsSatsUsdPattern::new(client.clone(), _m(&acc, "pct1")),
            ratio_pct2: BpsRatioPattern::new(client.clone(), _m(&acc, "pct2")),
            ratio_pct2_price: CentsSatsUsdPattern::new(client.clone(), _m(&acc, "pct2")),
            ratio_pct5: BpsRatioPattern::new(client.clone(), _m(&acc, "pct5")),
            ratio_pct5_price: CentsSatsUsdPattern::new(client.clone(), _m(&acc, "pct5")),
            ratio_pct95: BpsRatioPattern::new(client.clone(), _m(&acc, "pct95")),
            ratio_pct95_price: CentsSatsUsdPattern::new(client.clone(), _m(&acc, "pct95")),
            ratio_pct98: BpsRatioPattern::new(client.clone(), _m(&acc, "pct98")),
            ratio_pct98_price: CentsSatsUsdPattern::new(client.clone(), _m(&acc, "pct98")),
            ratio_pct99: BpsRatioPattern::new(client.clone(), _m(&acc, "pct99")),
            ratio_pct99_price: CentsSatsUsdPattern::new(client.clone(), _m(&acc, "pct99")),
            ratio_sma_1m: BpsRatioPattern::new(client.clone(), _m(&acc, "sma_1m")),
            ratio_sma_1w: BpsRatioPattern::new(client.clone(), _m(&acc, "sma_1w")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct GreedGrossInvestedInvestorNegNetPainSupplyUnrealizedPattern {
    pub greed_index: CentsUsdPattern,
    pub gross_pnl: CentsUsdPattern,
    pub invested_capital_in_loss: CentsUsdPattern,
    pub invested_capital_in_loss_raw: MetricPattern18<CentsSats>,
    pub invested_capital_in_profit: CentsUsdPattern,
    pub invested_capital_in_profit_raw: MetricPattern18<CentsSats>,
    pub investor_cap_in_loss_raw: MetricPattern18<CentsSquaredSats>,
    pub investor_cap_in_profit_raw: MetricPattern18<CentsSquaredSats>,
    pub neg_unrealized_loss: MetricPattern1<Dollars>,
    pub net_sentiment: CentsUsdPattern,
    pub net_unrealized_pnl: CentsUsdPattern,
    pub pain_index: CentsUsdPattern,
    pub supply_in_loss: BtcCentsSatsUsdPattern,
    pub supply_in_profit: BtcCentsSatsUsdPattern,
    pub unrealized_loss: CentsUsdPattern,
    pub unrealized_profit: CentsUsdPattern,
}

impl GreedGrossInvestedInvestorNegNetPainSupplyUnrealizedPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            greed_index: CentsUsdPattern::new(client.clone(), _m(&acc, "greed_index")),
            gross_pnl: CentsUsdPattern::new(client.clone(), _m(&acc, "unrealized_gross_pnl")),
            invested_capital_in_loss: CentsUsdPattern::new(client.clone(), _m(&acc, "invested_capital_in_loss")),
            invested_capital_in_loss_raw: MetricPattern18::new(client.clone(), _m(&acc, "invested_capital_in_loss_raw")),
            invested_capital_in_profit: CentsUsdPattern::new(client.clone(), _m(&acc, "invested_capital_in_profit")),
            invested_capital_in_profit_raw: MetricPattern18::new(client.clone(), _m(&acc, "invested_capital_in_profit_raw")),
            investor_cap_in_loss_raw: MetricPattern18::new(client.clone(), _m(&acc, "investor_cap_in_loss_raw")),
            investor_cap_in_profit_raw: MetricPattern18::new(client.clone(), _m(&acc, "investor_cap_in_profit_raw")),
            neg_unrealized_loss: MetricPattern1::new(client.clone(), _m(&acc, "neg_unrealized_loss")),
            net_sentiment: CentsUsdPattern::new(client.clone(), _m(&acc, "net_sentiment")),
            net_unrealized_pnl: CentsUsdPattern::new(client.clone(), _m(&acc, "net_unrealized_pnl")),
            pain_index: CentsUsdPattern::new(client.clone(), _m(&acc, "pain_index")),
            supply_in_loss: BtcCentsSatsUsdPattern::new(client.clone(), _m(&acc, "supply_in_loss")),
            supply_in_profit: BtcCentsSatsUsdPattern::new(client.clone(), _m(&acc, "supply_in_profit")),
            unrealized_loss: CentsUsdPattern::new(client.clone(), _m(&acc, "unrealized_loss")),
            unrealized_profit: CentsUsdPattern::new(client.clone(), _m(&acc, "unrealized_profit")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct MvrvNegNetRealizedSoprValuePattern {
    pub mvrv: MetricPattern1<StoredF32>,
    pub neg_realized_loss: MetricPattern1<Dollars>,
    pub net_realized_pnl: CumulativeHeightPattern<CentsSigned>,
    pub realized_cap: MetricPattern1<Dollars>,
    pub realized_cap_cents: MetricPattern1<Cents>,
    pub realized_cap_change_1m: MetricPattern1<CentsSigned>,
    pub realized_loss: CumulativeHeightPattern<Cents>,
    pub realized_price: CentsSatsUsdPattern,
    pub realized_price_ratio: BpsRatioPattern,
    pub realized_profit: CumulativeHeightPattern<Cents>,
    pub sopr: _24hPattern<StoredF64>,
    pub value_created: MetricPattern1<Cents>,
    pub value_created_sum: _24hPattern<Cents>,
    pub value_destroyed: MetricPattern1<Cents>,
    pub value_destroyed_sum: _24hPattern<Cents>,
}

impl MvrvNegNetRealizedSoprValuePattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            mvrv: MetricPattern1::new(client.clone(), _m(&acc, "mvrv")),
            neg_realized_loss: MetricPattern1::new(client.clone(), _m(&acc, "neg_realized_loss")),
            net_realized_pnl: CumulativeHeightPattern::new(client.clone(), _m(&acc, "net_realized_pnl")),
            realized_cap: MetricPattern1::new(client.clone(), _m(&acc, "realized_cap")),
            realized_cap_cents: MetricPattern1::new(client.clone(), _m(&acc, "realized_cap_cents")),
            realized_cap_change_1m: MetricPattern1::new(client.clone(), _m(&acc, "realized_cap_change_1m")),
            realized_loss: CumulativeHeightPattern::new(client.clone(), _m(&acc, "realized_loss")),
            realized_price: CentsSatsUsdPattern::new(client.clone(), _m(&acc, "realized_price")),
            realized_price_ratio: BpsRatioPattern::new(client.clone(), _m(&acc, "realized_price_ratio")),
            realized_profit: CumulativeHeightPattern::new(client.clone(), _m(&acc, "realized_profit")),
            sopr: _24hPattern::new(client.clone(), _m(&acc, "sopr_24h")),
            value_created: MetricPattern1::new(client.clone(), _m(&acc, "value_created")),
            value_created_sum: _24hPattern::new(client.clone(), _m(&acc, "value_created_24h")),
            value_destroyed: MetricPattern1::new(client.clone(), _m(&acc, "value_destroyed")),
            value_destroyed_sum: _24hPattern::new(client.clone(), _m(&acc, "value_destroyed_24h")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct NetNuplSupplyUnrealizedPattern2 {
    pub net_unrealized_pnl_rel_to_market_cap: BpsPercentRatioPattern,
    pub net_unrealized_pnl_rel_to_own_gross_pnl: BpsPercentRatioPattern,
    pub net_unrealized_pnl_rel_to_own_market_cap: BpsPercentRatioPattern,
    pub nupl: MetricPattern1<StoredF32>,
    pub supply_in_loss_rel_to_circulating_supply: BpsPercentRatioPattern,
    pub supply_in_loss_rel_to_own_supply: BpsPercentRatioPattern,
    pub supply_in_profit_rel_to_circulating_supply: BpsPercentRatioPattern,
    pub supply_in_profit_rel_to_own_supply: BpsPercentRatioPattern,
    pub supply_rel_to_circulating_supply: BpsPercentRatioPattern,
    pub unrealized_loss_rel_to_market_cap: BpsPercentRatioPattern,
    pub unrealized_loss_rel_to_own_gross_pnl: BpsPercentRatioPattern,
    pub unrealized_loss_rel_to_own_market_cap: BpsPercentRatioPattern,
    pub unrealized_profit_rel_to_market_cap: BpsPercentRatioPattern,
    pub unrealized_profit_rel_to_own_gross_pnl: BpsPercentRatioPattern,
    pub unrealized_profit_rel_to_own_market_cap: BpsPercentRatioPattern,
}

impl NetNuplSupplyUnrealizedPattern2 {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            net_unrealized_pnl_rel_to_market_cap: BpsPercentRatioPattern::new(client.clone(), _m(&acc, "net_unrealized_pnl_rel_to_market_cap")),
            net_unrealized_pnl_rel_to_own_gross_pnl: BpsPercentRatioPattern::new(client.clone(), _m(&acc, "net_unrealized_pnl_rel_to_own_gross_pnl")),
            net_unrealized_pnl_rel_to_own_market_cap: BpsPercentRatioPattern::new(client.clone(), _m(&acc, "net_unrealized_pnl_rel_to_own_market_cap")),
            nupl: MetricPattern1::new(client.clone(), _m(&acc, "nupl")),
            supply_in_loss_rel_to_circulating_supply: BpsPercentRatioPattern::new(client.clone(), _m(&acc, "supply_in_loss_rel_to_circulating_supply")),
            supply_in_loss_rel_to_own_supply: BpsPercentRatioPattern::new(client.clone(), _m(&acc, "supply_in_loss_rel_to_own_supply")),
            supply_in_profit_rel_to_circulating_supply: BpsPercentRatioPattern::new(client.clone(), _m(&acc, "supply_in_profit_rel_to_circulating_supply")),
            supply_in_profit_rel_to_own_supply: BpsPercentRatioPattern::new(client.clone(), _m(&acc, "supply_in_profit_rel_to_own_supply")),
            supply_rel_to_circulating_supply: BpsPercentRatioPattern::new(client.clone(), _m(&acc, "supply_rel_to_circulating_supply")),
            unrealized_loss_rel_to_market_cap: BpsPercentRatioPattern::new(client.clone(), _m(&acc, "unrealized_loss_rel_to_market_cap")),
            unrealized_loss_rel_to_own_gross_pnl: BpsPercentRatioPattern::new(client.clone(), _m(&acc, "unrealized_loss_rel_to_own_gross_pnl")),
            unrealized_loss_rel_to_own_market_cap: BpsPercentRatioPattern::new(client.clone(), _m(&acc, "unrealized_loss_rel_to_own_market_cap")),
            unrealized_profit_rel_to_market_cap: BpsPercentRatioPattern::new(client.clone(), _m(&acc, "unrealized_profit_rel_to_market_cap")),
            unrealized_profit_rel_to_own_gross_pnl: BpsPercentRatioPattern::new(client.clone(), _m(&acc, "unrealized_profit_rel_to_own_gross_pnl")),
            unrealized_profit_rel_to_own_market_cap: BpsPercentRatioPattern::new(client.clone(), _m(&acc, "unrealized_profit_rel_to_own_market_cap")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct RatioPattern {
    pub ratio_pct1: BpsRatioPattern,
    pub ratio_pct1_price: CentsSatsUsdPattern,
    pub ratio_pct2: BpsRatioPattern,
    pub ratio_pct2_price: CentsSatsUsdPattern,
    pub ratio_pct5: BpsRatioPattern,
    pub ratio_pct5_price: CentsSatsUsdPattern,
    pub ratio_pct95: BpsRatioPattern,
    pub ratio_pct95_price: CentsSatsUsdPattern,
    pub ratio_pct98: BpsRatioPattern,
    pub ratio_pct98_price: CentsSatsUsdPattern,
    pub ratio_pct99: BpsRatioPattern,
    pub ratio_pct99_price: CentsSatsUsdPattern,
    pub ratio_sma_1m: BpsRatioPattern,
    pub ratio_sma_1w: BpsRatioPattern,
}

impl RatioPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            ratio_pct1: BpsRatioPattern::new(client.clone(), _m(&acc, "pct1")),
            ratio_pct1_price: CentsSatsUsdPattern::new(client.clone(), _m(&acc, "pct1")),
            ratio_pct2: BpsRatioPattern::new(client.clone(), _m(&acc, "pct2")),
            ratio_pct2_price: CentsSatsUsdPattern::new(client.clone(), _m(&acc, "pct2")),
            ratio_pct5: BpsRatioPattern::new(client.clone(), _m(&acc, "pct5")),
            ratio_pct5_price: CentsSatsUsdPattern::new(client.clone(), _m(&acc, "pct5")),
            ratio_pct95: BpsRatioPattern::new(client.clone(), _m(&acc, "pct95")),
            ratio_pct95_price: CentsSatsUsdPattern::new(client.clone(), _m(&acc, "pct95")),
            ratio_pct98: BpsRatioPattern::new(client.clone(), _m(&acc, "pct98")),
            ratio_pct98_price: CentsSatsUsdPattern::new(client.clone(), _m(&acc, "pct98")),
            ratio_pct99: BpsRatioPattern::new(client.clone(), _m(&acc, "pct99")),
            ratio_pct99_price: CentsSatsUsdPattern::new(client.clone(), _m(&acc, "pct99")),
            ratio_sma_1m: BpsRatioPattern::new(client.clone(), _m(&acc, "sma_1m")),
            ratio_sma_1w: BpsRatioPattern::new(client.clone(), _m(&acc, "sma_1w")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct _10y1m1w1y2y3m3y4y5y6m6y8yPattern2 {
    pub _10y: BpsPercentRatioPattern,
    pub _1m: BpsPercentRatioPattern,
    pub _1w: BpsPercentRatioPattern,
    pub _1y: BpsPercentRatioPattern,
    pub _2y: BpsPercentRatioPattern,
    pub _3m: BpsPercentRatioPattern,
    pub _3y: BpsPercentRatioPattern,
    pub _4y: BpsPercentRatioPattern,
    pub _5y: BpsPercentRatioPattern,
    pub _6m: BpsPercentRatioPattern,
    pub _6y: BpsPercentRatioPattern,
    pub _8y: BpsPercentRatioPattern,
}

impl _10y1m1w1y2y3m3y4y5y6m6y8yPattern2 {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            _10y: BpsPercentRatioPattern::new(client.clone(), _m(&acc, "10y")),
            _1m: BpsPercentRatioPattern::new(client.clone(), _m(&acc, "1m")),
            _1w: BpsPercentRatioPattern::new(client.clone(), _m(&acc, "1w")),
            _1y: BpsPercentRatioPattern::new(client.clone(), _m(&acc, "1y")),
            _2y: BpsPercentRatioPattern::new(client.clone(), _m(&acc, "2y")),
            _3m: BpsPercentRatioPattern::new(client.clone(), _m(&acc, "3m")),
            _3y: BpsPercentRatioPattern::new(client.clone(), _m(&acc, "3y")),
            _4y: BpsPercentRatioPattern::new(client.clone(), _m(&acc, "4y")),
            _5y: BpsPercentRatioPattern::new(client.clone(), _m(&acc, "5y")),
            _6m: BpsPercentRatioPattern::new(client.clone(), _m(&acc, "6m")),
            _6y: BpsPercentRatioPattern::new(client.clone(), _m(&acc, "6y")),
            _8y: BpsPercentRatioPattern::new(client.clone(), _m(&acc, "8y")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct _10y1m1w1y2y3m3y4y5y6m6y8yPattern3 {
    pub _10y: BtcCentsSatsUsdPattern,
    pub _1m: BtcCentsSatsUsdPattern,
    pub _1w: BtcCentsSatsUsdPattern,
    pub _1y: BtcCentsSatsUsdPattern,
    pub _2y: BtcCentsSatsUsdPattern,
    pub _3m: BtcCentsSatsUsdPattern,
    pub _3y: BtcCentsSatsUsdPattern,
    pub _4y: BtcCentsSatsUsdPattern,
    pub _5y: BtcCentsSatsUsdPattern,
    pub _6m: BtcCentsSatsUsdPattern,
    pub _6y: BtcCentsSatsUsdPattern,
    pub _8y: BtcCentsSatsUsdPattern,
}

impl _10y1m1w1y2y3m3y4y5y6m6y8yPattern3 {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            _10y: BtcCentsSatsUsdPattern::new(client.clone(), _m(&acc, "10y")),
            _1m: BtcCentsSatsUsdPattern::new(client.clone(), _m(&acc, "1m")),
            _1w: BtcCentsSatsUsdPattern::new(client.clone(), _m(&acc, "1w")),
            _1y: BtcCentsSatsUsdPattern::new(client.clone(), _m(&acc, "1y")),
            _2y: BtcCentsSatsUsdPattern::new(client.clone(), _m(&acc, "2y")),
            _3m: BtcCentsSatsUsdPattern::new(client.clone(), _m(&acc, "3m")),
            _3y: BtcCentsSatsUsdPattern::new(client.clone(), _m(&acc, "3y")),
            _4y: BtcCentsSatsUsdPattern::new(client.clone(), _m(&acc, "4y")),
            _5y: BtcCentsSatsUsdPattern::new(client.clone(), _m(&acc, "5y")),
            _6m: BtcCentsSatsUsdPattern::new(client.clone(), _m(&acc, "6m")),
            _6y: BtcCentsSatsUsdPattern::new(client.clone(), _m(&acc, "6y")),
            _8y: BtcCentsSatsUsdPattern::new(client.clone(), _m(&acc, "8y")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern {
    pub _1m: BpsPercentRatioPattern,
    pub _1w: BpsPercentRatioPattern,
    pub _1y: BpsPercentRatioPattern,
    pub _24h: BpsPercentRatioPattern,
    pub base: BtcCentsSatsUsdPattern,
    pub bps: MetricPattern1<BasisPoints16>,
    pub cumulative: BaseBtcCentsSatsUsdPattern,
    pub height: MetricPattern18<StoredU32>,
    pub percent: MetricPattern1<StoredF32>,
    pub ratio: MetricPattern1<StoredF32>,
    pub sum: _1m1w1y24hPattern6,
}

impl _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            _1m: BpsPercentRatioPattern::new(client.clone(), _m(&acc, "dominance_1m")),
            _1w: BpsPercentRatioPattern::new(client.clone(), _m(&acc, "dominance_1w")),
            _1y: BpsPercentRatioPattern::new(client.clone(), _m(&acc, "dominance_1y")),
            _24h: BpsPercentRatioPattern::new(client.clone(), _m(&acc, "dominance_24h")),
            base: BtcCentsSatsUsdPattern::new(client.clone(), _m(&acc, "rewards")),
            bps: MetricPattern1::new(client.clone(), _m(&acc, "dominance_bps")),
            cumulative: BaseBtcCentsSatsUsdPattern::new(client.clone(), acc.clone()),
            height: MetricPattern18::new(client.clone(), _m(&acc, "blocks_mined")),
            percent: MetricPattern1::new(client.clone(), _m(&acc, "dominance")),
            ratio: MetricPattern1::new(client.clone(), _m(&acc, "dominance_ratio")),
            sum: _1m1w1y24hPattern6::new(client.clone(), acc.clone()),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct AverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90RollingSumPattern {
    pub average: MetricPattern18<StoredU64>,
    pub cumulative: MetricPattern18<StoredU64>,
    pub max: MetricPattern18<StoredU64>,
    pub median: MetricPattern18<StoredU64>,
    pub min: MetricPattern18<StoredU64>,
    pub pct10: MetricPattern18<StoredU64>,
    pub pct25: MetricPattern18<StoredU64>,
    pub pct75: MetricPattern18<StoredU64>,
    pub pct90: MetricPattern18<StoredU64>,
    pub rolling: AverageMaxMedianMinPct10Pct25Pct75Pct90SumPattern,
    pub sum: MetricPattern18<StoredU64>,
}

impl AverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90RollingSumPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            average: MetricPattern18::new(client.clone(), _m(&acc, "average")),
            cumulative: MetricPattern18::new(client.clone(), _m(&acc, "cumulative")),
            max: MetricPattern18::new(client.clone(), _m(&acc, "max")),
            median: MetricPattern18::new(client.clone(), _m(&acc, "median")),
            min: MetricPattern18::new(client.clone(), _m(&acc, "min")),
            pct10: MetricPattern18::new(client.clone(), _m(&acc, "p10")),
            pct25: MetricPattern18::new(client.clone(), _m(&acc, "p25")),
            pct75: MetricPattern18::new(client.clone(), _m(&acc, "p75")),
            pct90: MetricPattern18::new(client.clone(), _m(&acc, "p90")),
            rolling: AverageMaxMedianMinPct10Pct25Pct75Pct90SumPattern::new(client.clone(), acc.clone()),
            sum: MetricPattern18::new(client.clone(), _m(&acc, "sum")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct AverageCumulativeHeightMaxMedianMinPct10Pct25Pct75Pct90SumPattern {
    pub average: _1m1w1y24hPattern<StoredU64>,
    pub cumulative: MetricPattern1<StoredU64>,
    pub height: MetricPattern18<StoredU64>,
    pub max: _1m1w1y24hPattern<StoredU64>,
    pub median: _1m1w1y24hPattern<StoredU64>,
    pub min: _1m1w1y24hPattern<StoredU64>,
    pub pct10: _1m1w1y24hPattern<StoredU64>,
    pub pct25: _1m1w1y24hPattern<StoredU64>,
    pub pct75: _1m1w1y24hPattern<StoredU64>,
    pub pct90: _1m1w1y24hPattern<StoredU64>,
    pub sum: _1m1w1y24hPattern<StoredU64>,
}

impl AverageCumulativeHeightMaxMedianMinPct10Pct25Pct75Pct90SumPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            average: _1m1w1y24hPattern::new(client.clone(), _m(&acc, "average")),
            cumulative: MetricPattern1::new(client.clone(), _m(&acc, "cumulative")),
            height: MetricPattern18::new(client.clone(), acc.clone()),
            max: _1m1w1y24hPattern::new(client.clone(), _m(&acc, "max")),
            median: _1m1w1y24hPattern::new(client.clone(), _m(&acc, "median")),
            min: _1m1w1y24hPattern::new(client.clone(), _m(&acc, "min")),
            pct10: _1m1w1y24hPattern::new(client.clone(), _m(&acc, "p10")),
            pct25: _1m1w1y24hPattern::new(client.clone(), _m(&acc, "p25")),
            pct75: _1m1w1y24hPattern::new(client.clone(), _m(&acc, "p75")),
            pct90: _1m1w1y24hPattern::new(client.clone(), _m(&acc, "p90")),
            sum: _1m1w1y24hPattern::new(client.clone(), _m(&acc, "sum")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct AverageGainsLossesRsiStochPattern {
    pub average_gain: MetricPattern1<StoredF32>,
    pub average_loss: MetricPattern1<StoredF32>,
    pub gains: MetricPattern1<StoredF32>,
    pub losses: MetricPattern1<StoredF32>,
    pub rsi: BpsPercentRatioPattern,
    pub rsi_max: BpsPercentRatioPattern,
    pub rsi_min: BpsPercentRatioPattern,
    pub stoch_rsi: BpsPercentRatioPattern,
    pub stoch_rsi_d: BpsPercentRatioPattern,
    pub stoch_rsi_k: BpsPercentRatioPattern,
}

impl AverageGainsLossesRsiStochPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            average_gain: MetricPattern1::new(client.clone(), _m(&acc, "average_gain_24h")),
            average_loss: MetricPattern1::new(client.clone(), _m(&acc, "average_loss_24h")),
            gains: MetricPattern1::new(client.clone(), _m(&acc, "gains_24h")),
            losses: MetricPattern1::new(client.clone(), _m(&acc, "losses_24h")),
            rsi: BpsPercentRatioPattern::new(client.clone(), _m(&acc, "24h")),
            rsi_max: BpsPercentRatioPattern::new(client.clone(), _m(&acc, "max_24h")),
            rsi_min: BpsPercentRatioPattern::new(client.clone(), _m(&acc, "min_24h")),
            stoch_rsi: BpsPercentRatioPattern::new(client.clone(), _m(&acc, "stoch_24h")),
            stoch_rsi_d: BpsPercentRatioPattern::new(client.clone(), _m(&acc, "stoch_d_24h")),
            stoch_rsi_k: BpsPercentRatioPattern::new(client.clone(), _m(&acc, "stoch_k_24h")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct InvestedInvestorNegNetSupplyUnrealizedPattern {
    pub invested_capital_in_loss_raw: MetricPattern18<CentsSats>,
    pub invested_capital_in_profit_raw: MetricPattern18<CentsSats>,
    pub investor_cap_in_loss_raw: MetricPattern18<CentsSquaredSats>,
    pub investor_cap_in_profit_raw: MetricPattern18<CentsSquaredSats>,
    pub neg_unrealized_loss: MetricPattern1<Dollars>,
    pub net_unrealized_pnl: CentsUsdPattern,
    pub supply_in_loss: BtcCentsSatsUsdPattern,
    pub supply_in_profit: BtcCentsSatsUsdPattern,
    pub unrealized_loss: CentsUsdPattern,
    pub unrealized_profit: CentsUsdPattern,
}

impl InvestedInvestorNegNetSupplyUnrealizedPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            invested_capital_in_loss_raw: MetricPattern18::new(client.clone(), _m(&acc, "invested_capital_in_loss_raw")),
            invested_capital_in_profit_raw: MetricPattern18::new(client.clone(), _m(&acc, "invested_capital_in_profit_raw")),
            investor_cap_in_loss_raw: MetricPattern18::new(client.clone(), _m(&acc, "investor_cap_in_loss_raw")),
            investor_cap_in_profit_raw: MetricPattern18::new(client.clone(), _m(&acc, "investor_cap_in_profit_raw")),
            neg_unrealized_loss: MetricPattern1::new(client.clone(), _m(&acc, "neg_unrealized_loss")),
            net_unrealized_pnl: CentsUsdPattern::new(client.clone(), _m(&acc, "net_unrealized_pnl")),
            supply_in_loss: BtcCentsSatsUsdPattern::new(client.clone(), _m(&acc, "supply_in_loss")),
            supply_in_profit: BtcCentsSatsUsdPattern::new(client.clone(), _m(&acc, "supply_in_profit")),
            unrealized_loss: CentsUsdPattern::new(client.clone(), _m(&acc, "unrealized_loss")),
            unrealized_profit: CentsUsdPattern::new(client.clone(), _m(&acc, "unrealized_profit")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct AllP2aP2pk33P2pk65P2pkhP2shP2trP2wpkhP2wshPattern {
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

impl AllP2aP2pk33P2pk65P2pkhP2shP2trP2wpkhP2wshPattern {
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
pub struct AverageMaxMedianMinPct10Pct25Pct75Pct90SumPattern2 {
    pub average: BtcCentsSatsUsdPattern,
    pub max: BtcCentsSatsUsdPattern,
    pub median: BtcCentsSatsUsdPattern,
    pub min: BtcCentsSatsUsdPattern,
    pub pct10: BtcCentsSatsUsdPattern,
    pub pct25: BtcCentsSatsUsdPattern,
    pub pct75: BtcCentsSatsUsdPattern,
    pub pct90: BtcCentsSatsUsdPattern,
    pub sum: BtcCentsSatsUsdPattern,
}

impl AverageMaxMedianMinPct10Pct25Pct75Pct90SumPattern2 {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            average: BtcCentsSatsUsdPattern::new(client.clone(), _m(&acc, "average")),
            max: BtcCentsSatsUsdPattern::new(client.clone(), _m(&acc, "max")),
            median: BtcCentsSatsUsdPattern::new(client.clone(), _m(&acc, "median")),
            min: BtcCentsSatsUsdPattern::new(client.clone(), _m(&acc, "min")),
            pct10: BtcCentsSatsUsdPattern::new(client.clone(), _m(&acc, "p10")),
            pct25: BtcCentsSatsUsdPattern::new(client.clone(), _m(&acc, "p25")),
            pct75: BtcCentsSatsUsdPattern::new(client.clone(), _m(&acc, "p75")),
            pct90: BtcCentsSatsUsdPattern::new(client.clone(), _m(&acc, "p90")),
            sum: BtcCentsSatsUsdPattern::new(client.clone(), _m(&acc, "sum")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct AverageMaxMedianMinPct10Pct25Pct75Pct90SumPattern {
    pub average: _1m1w1y24hPattern<StoredU64>,
    pub max: _1m1w1y24hPattern<StoredU64>,
    pub median: _1m1w1y24hPattern<StoredU64>,
    pub min: _1m1w1y24hPattern<StoredU64>,
    pub pct10: _1m1w1y24hPattern<StoredU64>,
    pub pct25: _1m1w1y24hPattern<StoredU64>,
    pub pct75: _1m1w1y24hPattern<StoredU64>,
    pub pct90: _1m1w1y24hPattern<StoredU64>,
    pub sum: _1m1w1y24hPattern<StoredU64>,
}

impl AverageMaxMedianMinPct10Pct25Pct75Pct90SumPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            average: _1m1w1y24hPattern::new(client.clone(), _m(&acc, "average")),
            max: _1m1w1y24hPattern::new(client.clone(), _m(&acc, "max")),
            median: _1m1w1y24hPattern::new(client.clone(), _m(&acc, "median")),
            min: _1m1w1y24hPattern::new(client.clone(), _m(&acc, "min")),
            pct10: _1m1w1y24hPattern::new(client.clone(), _m(&acc, "p10")),
            pct25: _1m1w1y24hPattern::new(client.clone(), _m(&acc, "p25")),
            pct75: _1m1w1y24hPattern::new(client.clone(), _m(&acc, "p75")),
            pct90: _1m1w1y24hPattern::new(client.clone(), _m(&acc, "p90")),
            sum: _1m1w1y24hPattern::new(client.clone(), _m(&acc, "sum")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct _1m1w1y24hBtcCentsSatsUsdPattern {
    pub _1m: BtcCentsSatsUsdPattern,
    pub _1w: BtcCentsSatsUsdPattern,
    pub _1y: BtcCentsSatsUsdPattern,
    pub _24h: BtcCentsSatsUsdPattern,
    pub btc: MetricPattern18<Bitcoin>,
    pub cents: MetricPattern18<Cents>,
    pub sats: MetricPattern18<Sats>,
    pub usd: MetricPattern18<Dollars>,
}

impl _1m1w1y24hBtcCentsSatsUsdPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            _1m: BtcCentsSatsUsdPattern::new(client.clone(), _m(&acc, "1m")),
            _1w: BtcCentsSatsUsdPattern::new(client.clone(), _m(&acc, "1w")),
            _1y: BtcCentsSatsUsdPattern::new(client.clone(), _m(&acc, "1y")),
            _24h: BtcCentsSatsUsdPattern::new(client.clone(), _m(&acc, "24h")),
            btc: MetricPattern18::new(client.clone(), _m(&acc, "btc")),
            cents: MetricPattern18::new(client.clone(), _m(&acc, "cents")),
            sats: MetricPattern18::new(client.clone(), acc.clone()),
            usd: MetricPattern18::new(client.clone(), _m(&acc, "usd")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct AverageMaxMedianMinPct10Pct25Pct75Pct90Pattern<T> {
    pub average: MetricPattern18<T>,
    pub max: MetricPattern18<T>,
    pub median: MetricPattern18<T>,
    pub min: MetricPattern18<T>,
    pub pct10: MetricPattern18<T>,
    pub pct25: MetricPattern18<T>,
    pub pct75: MetricPattern18<T>,
    pub pct90: MetricPattern18<T>,
}

impl<T: DeserializeOwned> AverageMaxMedianMinPct10Pct25Pct75Pct90Pattern<T> {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            average: MetricPattern18::new(client.clone(), _m(&acc, "average")),
            max: MetricPattern18::new(client.clone(), _m(&acc, "max")),
            median: MetricPattern18::new(client.clone(), _m(&acc, "median")),
            min: MetricPattern18::new(client.clone(), _m(&acc, "min")),
            pct10: MetricPattern18::new(client.clone(), _m(&acc, "p10")),
            pct25: MetricPattern18::new(client.clone(), _m(&acc, "p25")),
            pct75: MetricPattern18::new(client.clone(), _m(&acc, "p75")),
            pct90: MetricPattern18::new(client.clone(), _m(&acc, "p90")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct _10y2y3y4y5y6y8yPattern {
    pub _10y: BpsPercentRatioPattern,
    pub _2y: BpsPercentRatioPattern,
    pub _3y: BpsPercentRatioPattern,
    pub _4y: BpsPercentRatioPattern,
    pub _5y: BpsPercentRatioPattern,
    pub _6y: BpsPercentRatioPattern,
    pub _8y: BpsPercentRatioPattern,
}

impl _10y2y3y4y5y6y8yPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            _10y: BpsPercentRatioPattern::new(client.clone(), _m(&acc, "10y")),
            _2y: BpsPercentRatioPattern::new(client.clone(), _m(&acc, "2y")),
            _3y: BpsPercentRatioPattern::new(client.clone(), _m(&acc, "3y")),
            _4y: BpsPercentRatioPattern::new(client.clone(), _m(&acc, "4y")),
            _5y: BpsPercentRatioPattern::new(client.clone(), _m(&acc, "5y")),
            _6y: BpsPercentRatioPattern::new(client.clone(), _m(&acc, "6y")),
            _8y: BpsPercentRatioPattern::new(client.clone(), _m(&acc, "8y")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern {
    pub activity: CoinblocksCoindaysSentPattern,
    pub cost_basis: MaxMinPattern,
    pub outputs: UtxoPattern,
    pub realized: MvrvNegNetRealizedSentSoprValuePattern,
    pub relative: SupplyPattern2,
    pub supply: ChangeHalvedTotalPattern,
    pub unrealized: InvestedInvestorNegNetSupplyUnrealizedPattern,
}

impl ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            activity: CoinblocksCoindaysSentPattern::new(client.clone(), acc.clone()),
            cost_basis: MaxMinPattern::new(client.clone(), _m(&acc, "cost_basis")),
            outputs: UtxoPattern::new(client.clone(), _m(&acc, "utxo_count")),
            realized: MvrvNegNetRealizedSentSoprValuePattern::new(client.clone(), acc.clone()),
            relative: SupplyPattern2::new(client.clone(), _m(&acc, "supply")),
            supply: ChangeHalvedTotalPattern::new(client.clone(), _m(&acc, "supply")),
            unrealized: InvestedInvestorNegNetSupplyUnrealizedPattern::new(client.clone(), acc.clone()),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct ActivityAddrOutputsRealizedSupplyUnrealizedPattern {
    pub activity: SentPattern,
    pub addr_count: MetricPattern1<StoredU64>,
    pub addr_count_change_1m: MetricPattern1<StoredF64>,
    pub outputs: UtxoPattern,
    pub realized: MvrvRealizedPattern,
    pub supply: ChangeHalvedTotalPattern,
    pub unrealized: SupplyPattern,
}

impl ActivityAddrOutputsRealizedSupplyUnrealizedPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            activity: SentPattern::new(client.clone(), _m(&acc, "sent")),
            addr_count: MetricPattern1::new(client.clone(), _m(&acc, "addr_count")),
            addr_count_change_1m: MetricPattern1::new(client.clone(), _m(&acc, "addr_count_change_1m")),
            outputs: UtxoPattern::new(client.clone(), _m(&acc, "utxo_count")),
            realized: MvrvRealizedPattern::new(client.clone(), acc.clone()),
            supply: ChangeHalvedTotalPattern::new(client.clone(), _m(&acc, "supply")),
            unrealized: SupplyPattern::new(client.clone(), _m(&acc, "supply_in")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct MvrvRealizedPattern {
    pub mvrv: MetricPattern1<StoredF32>,
    pub realized_cap: MetricPattern1<Dollars>,
    pub realized_cap_cents: MetricPattern1<Cents>,
    pub realized_loss: CumulativeHeightPattern<Cents>,
    pub realized_price: CentsSatsUsdPattern,
    pub realized_price_ratio: BpsRatioPattern,
    pub realized_profit: CumulativeHeightPattern<Cents>,
}

impl MvrvRealizedPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            mvrv: MetricPattern1::new(client.clone(), _m(&acc, "mvrv")),
            realized_cap: MetricPattern1::new(client.clone(), _m(&acc, "realized_cap")),
            realized_cap_cents: MetricPattern1::new(client.clone(), _m(&acc, "realized_cap_cents")),
            realized_loss: CumulativeHeightPattern::new(client.clone(), _m(&acc, "realized_loss")),
            realized_price: CentsSatsUsdPattern::new(client.clone(), _m(&acc, "realized_price")),
            realized_price_ratio: BpsRatioPattern::new(client.clone(), _m(&acc, "realized_price_ratio")),
            realized_profit: CumulativeHeightPattern::new(client.clone(), _m(&acc, "realized_profit")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct ActivityOutputsRealizedRelativeSupplyUnrealizedPattern {
    pub activity: SentPattern,
    pub outputs: UtxoPattern,
    pub realized: MvrvNegNetRealizedSoprValuePattern,
    pub relative: SupplyPattern2,
    pub supply: ChangeHalvedTotalPattern,
    pub unrealized: NegNetSupplyUnrealizedPattern,
}

impl ActivityOutputsRealizedRelativeSupplyUnrealizedPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            activity: SentPattern::new(client.clone(), _m(&acc, "sent")),
            outputs: UtxoPattern::new(client.clone(), _m(&acc, "utxo_count")),
            realized: MvrvNegNetRealizedSoprValuePattern::new(client.clone(), acc.clone()),
            relative: SupplyPattern2::new(client.clone(), _m(&acc, "supply")),
            supply: ChangeHalvedTotalPattern::new(client.clone(), _m(&acc, "supply")),
            unrealized: NegNetSupplyUnrealizedPattern::new(client.clone(), acc.clone()),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct NegNetSupplyUnrealizedPattern {
    pub neg_unrealized_loss: MetricPattern1<Dollars>,
    pub net_unrealized_pnl: CentsUsdPattern,
    pub supply_in_loss: BtcCentsSatsUsdPattern,
    pub supply_in_profit: BtcCentsSatsUsdPattern,
    pub unrealized_loss: CentsUsdPattern,
    pub unrealized_profit: CentsUsdPattern,
}

impl NegNetSupplyUnrealizedPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            neg_unrealized_loss: MetricPattern1::new(client.clone(), _m(&acc, "neg_unrealized_loss")),
            net_unrealized_pnl: CentsUsdPattern::new(client.clone(), _m(&acc, "net_unrealized_pnl")),
            supply_in_loss: BtcCentsSatsUsdPattern::new(client.clone(), _m(&acc, "supply_in_loss")),
            supply_in_profit: BtcCentsSatsUsdPattern::new(client.clone(), _m(&acc, "supply_in_profit")),
            unrealized_loss: CentsUsdPattern::new(client.clone(), _m(&acc, "unrealized_loss")),
            unrealized_profit: CentsUsdPattern::new(client.clone(), _m(&acc, "unrealized_profit")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct ActivityOutputsRealizedSupplyUnrealizedPattern {
    pub activity: SentPattern,
    pub outputs: UtxoPattern,
    pub realized: MvrvRealizedPattern,
    pub supply: ChangeHalvedTotalPattern,
    pub unrealized: SupplyPattern,
}

impl ActivityOutputsRealizedSupplyUnrealizedPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            activity: SentPattern::new(client.clone(), _m(&acc, "sent")),
            outputs: UtxoPattern::new(client.clone(), _m(&acc, "utxo_count")),
            realized: MvrvRealizedPattern::new(client.clone(), acc.clone()),
            supply: ChangeHalvedTotalPattern::new(client.clone(), _m(&acc, "supply")),
            unrealized: SupplyPattern::new(client.clone(), _m(&acc, "supply_in")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct BaseBtcCentsSatsUsdPattern {
    pub base: MetricPattern1<StoredU32>,
    pub btc: MetricPattern1<Bitcoin>,
    pub cents: MetricPattern1<Cents>,
    pub sats: MetricPattern1<Sats>,
    pub usd: MetricPattern1<Dollars>,
}

impl BaseBtcCentsSatsUsdPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            base: MetricPattern1::new(client.clone(), _m(&acc, "blocks_mined_cumulative")),
            btc: MetricPattern1::new(client.clone(), _m(&acc, "rewards_cumulative_btc")),
            cents: MetricPattern1::new(client.clone(), _m(&acc, "rewards_cumulative_cents")),
            sats: MetricPattern1::new(client.clone(), _m(&acc, "rewards_cumulative")),
            usd: MetricPattern1::new(client.clone(), _m(&acc, "rewards_cumulative_usd")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct EmaHistogramLineSignalPattern {
    pub ema_fast: MetricPattern1<StoredF32>,
    pub ema_slow: MetricPattern1<StoredF32>,
    pub histogram: MetricPattern1<StoredF32>,
    pub line: MetricPattern1<StoredF32>,
    pub signal: MetricPattern1<StoredF32>,
}

impl EmaHistogramLineSignalPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            ema_fast: MetricPattern1::new(client.clone(), _m(&acc, "ema_fast_24h")),
            ema_slow: MetricPattern1::new(client.clone(), _m(&acc, "ema_slow_24h")),
            histogram: MetricPattern1::new(client.clone(), _m(&acc, "histogram_24h")),
            line: MetricPattern1::new(client.clone(), _m(&acc, "line_24h")),
            signal: MetricPattern1::new(client.clone(), _m(&acc, "signal_24h")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct SupplyPattern2 {
    pub supply_in_loss_rel_to_circulating_supply: BpsPercentRatioPattern,
    pub supply_in_loss_rel_to_own_supply: BpsPercentRatioPattern,
    pub supply_in_profit_rel_to_circulating_supply: BpsPercentRatioPattern,
    pub supply_in_profit_rel_to_own_supply: BpsPercentRatioPattern,
    pub supply_rel_to_circulating_supply: BpsPercentRatioPattern,
}

impl SupplyPattern2 {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            supply_in_loss_rel_to_circulating_supply: BpsPercentRatioPattern::new(client.clone(), _m(&acc, "in_loss_rel_to_circulating_supply")),
            supply_in_loss_rel_to_own_supply: BpsPercentRatioPattern::new(client.clone(), _m(&acc, "in_loss_rel_to_own_supply")),
            supply_in_profit_rel_to_circulating_supply: BpsPercentRatioPattern::new(client.clone(), _m(&acc, "in_profit_rel_to_circulating_supply")),
            supply_in_profit_rel_to_own_supply: BpsPercentRatioPattern::new(client.clone(), _m(&acc, "in_profit_rel_to_own_supply")),
            supply_rel_to_circulating_supply: BpsPercentRatioPattern::new(client.clone(), _m(&acc, "rel_to_circulating_supply")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct _1m1w1y24hHeightPattern<T> {
    pub _1m: MetricPattern1<T>,
    pub _1w: MetricPattern1<T>,
    pub _1y: MetricPattern1<T>,
    pub _24h: MetricPattern1<T>,
    pub height: MetricPattern18<T>,
}

impl<T: DeserializeOwned> _1m1w1y24hHeightPattern<T> {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            _1m: MetricPattern1::new(client.clone(), _m(&acc, "average_1m")),
            _1w: MetricPattern1::new(client.clone(), _m(&acc, "average_1w")),
            _1y: MetricPattern1::new(client.clone(), _m(&acc, "average_1y")),
            _24h: MetricPattern1::new(client.clone(), _m(&acc, "average_24h")),
            height: MetricPattern18::new(client.clone(), acc.clone()),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct _1m1w1y24hPattern6 {
    pub _1m: BaseBtcCentsSatsUsdPattern,
    pub _1w: BaseBtcCentsSatsUsdPattern,
    pub _1y: BaseBtcCentsSatsUsdPattern,
    pub _24h: BaseBtcCentsSatsUsdPattern,
}

impl _1m1w1y24hPattern6 {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            _1m: BaseBtcCentsSatsUsdPattern::new(client.clone(), acc.clone()),
            _1w: BaseBtcCentsSatsUsdPattern::new(client.clone(), acc.clone()),
            _1y: BaseBtcCentsSatsUsdPattern::new(client.clone(), acc.clone()),
            _24h: BaseBtcCentsSatsUsdPattern::new(client.clone(), acc.clone()),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct _1m1w1y24hPattern2 {
    pub _1m: BpsPercentRatioPattern,
    pub _1w: BpsPercentRatioPattern,
    pub _1y: BpsPercentRatioPattern,
    pub _24h: BpsPercentRatioPattern,
}

impl _1m1w1y24hPattern2 {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            _1m: BpsPercentRatioPattern::new(client.clone(), _m(&acc, "1m")),
            _1w: BpsPercentRatioPattern::new(client.clone(), _m(&acc, "1w")),
            _1y: BpsPercentRatioPattern::new(client.clone(), _m(&acc, "1y")),
            _24h: BpsPercentRatioPattern::new(client.clone(), _m(&acc, "24h")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct _1m1w1y24hPattern5 {
    pub _1m: BtcCentsSatsUsdPattern,
    pub _1w: BtcCentsSatsUsdPattern,
    pub _1y: BtcCentsSatsUsdPattern,
    pub _24h: BtcCentsSatsUsdPattern,
}

impl _1m1w1y24hPattern5 {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            _1m: BtcCentsSatsUsdPattern::new(client.clone(), _m(&acc, "1m")),
            _1w: BtcCentsSatsUsdPattern::new(client.clone(), _m(&acc, "1w")),
            _1y: BtcCentsSatsUsdPattern::new(client.clone(), _m(&acc, "1y")),
            _24h: BtcCentsSatsUsdPattern::new(client.clone(), _m(&acc, "24h")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct BothReactivatedReceivingSendingPattern {
    pub both: _1m1w1y24hHeightPattern<StoredU32>,
    pub reactivated: _1m1w1y24hHeightPattern<StoredU32>,
    pub receiving: _1m1w1y24hHeightPattern<StoredU32>,
    pub sending: _1m1w1y24hHeightPattern<StoredU32>,
}

impl BothReactivatedReceivingSendingPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            both: _1m1w1y24hHeightPattern::new(client.clone(), _m(&acc, "both")),
            reactivated: _1m1w1y24hHeightPattern::new(client.clone(), _m(&acc, "reactivated")),
            receiving: _1m1w1y24hHeightPattern::new(client.clone(), _m(&acc, "receiving")),
            sending: _1m1w1y24hHeightPattern::new(client.clone(), _m(&acc, "sending")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct BtcCentsSatsUsdPattern {
    pub btc: MetricPattern1<Bitcoin>,
    pub cents: MetricPattern1<Cents>,
    pub sats: MetricPattern1<Sats>,
    pub usd: MetricPattern1<Dollars>,
}

impl BtcCentsSatsUsdPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            btc: MetricPattern1::new(client.clone(), _m(&acc, "btc")),
            cents: MetricPattern1::new(client.clone(), _m(&acc, "cents")),
            sats: MetricPattern1::new(client.clone(), acc.clone()),
            usd: MetricPattern1::new(client.clone(), _m(&acc, "usd")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct CoinblocksCoindaysSentPattern {
    pub coinblocks_destroyed: CumulativeHeightPattern<StoredF64>,
    pub coindays_destroyed: CumulativeHeightSumPattern<StoredF64>,
    pub sent: BaseCumulativePattern,
    pub sent_ema: _2wPattern,
}

impl CoinblocksCoindaysSentPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            coinblocks_destroyed: CumulativeHeightPattern::new(client.clone(), _m(&acc, "coinblocks_destroyed")),
            coindays_destroyed: CumulativeHeightSumPattern::new(client.clone(), _m(&acc, "coindays_destroyed")),
            sent: BaseCumulativePattern::new(client.clone(), _m(&acc, "sent")),
            sent_ema: _2wPattern::new(client.clone(), _m(&acc, "sent_ema_2w")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct InvestedMaxMinPercentilesPattern {
    pub invested_capital: Pct05Pct10Pct15Pct20Pct25Pct30Pct35Pct40Pct45Pct50Pct55Pct60Pct65Pct70Pct75Pct80Pct85Pct90Pct95Pattern,
    pub max: CentsSatsUsdPattern,
    pub min: CentsSatsUsdPattern,
    pub percentiles: Pct05Pct10Pct15Pct20Pct25Pct30Pct35Pct40Pct45Pct50Pct55Pct60Pct65Pct70Pct75Pct80Pct85Pct90Pct95Pattern,
}

impl InvestedMaxMinPercentilesPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            invested_capital: Pct05Pct10Pct15Pct20Pct25Pct30Pct35Pct40Pct45Pct50Pct55Pct60Pct65Pct70Pct75Pct80Pct85Pct90Pct95Pattern::new(client.clone(), _m(&acc, "invested_capital")),
            max: CentsSatsUsdPattern::new(client.clone(), _m(&acc, "cost_basis_max")),
            min: CentsSatsUsdPattern::new(client.clone(), _m(&acc, "cost_basis_min")),
            percentiles: Pct05Pct10Pct15Pct20Pct25Pct30Pct35Pct40Pct45Pct50Pct55Pct60Pct65Pct70Pct75Pct80Pct85Pct90Pct95Pattern::new(client.clone(), _m(&acc, "cost_basis")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct RatioPattern2 {
    pub ratio_sd: _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern,
    pub ratio_sd_1y: _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern,
    pub ratio_sd_2y: _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern,
    pub ratio_sd_4y: _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern,
}

impl RatioPattern2 {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            ratio_sd: _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern::new(client.clone(), acc.clone()),
            ratio_sd_1y: _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern::new(client.clone(), acc.clone()),
            ratio_sd_2y: _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern::new(client.clone(), acc.clone()),
            ratio_sd_4y: _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern::new(client.clone(), acc.clone()),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct _1m1w1y24hPattern<T> {
    pub _1m: MetricPattern1<T>,
    pub _1w: MetricPattern1<T>,
    pub _1y: MetricPattern1<T>,
    pub _24h: MetricPattern1<T>,
}

impl<T: DeserializeOwned> _1m1w1y24hPattern<T> {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            _1m: MetricPattern1::new(client.clone(), _m(&acc, "1m")),
            _1w: MetricPattern1::new(client.clone(), _m(&acc, "1w")),
            _1y: MetricPattern1::new(client.clone(), _m(&acc, "1y")),
            _24h: MetricPattern1::new(client.clone(), _m(&acc, "24h")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct BaseCumulativeSumPattern {
    pub base: BtcCentsSatsUsdPattern,
    pub cumulative: BtcCentsSatsUsdPattern,
    pub sum: _1m1w1y24hPattern5,
}

impl BaseCumulativeSumPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            base: BtcCentsSatsUsdPattern::new(client.clone(), acc.clone()),
            cumulative: BtcCentsSatsUsdPattern::new(client.clone(), _m(&acc, "cumulative")),
            sum: _1m1w1y24hPattern5::new(client.clone(), _m(&acc, "sum")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct BpsPercentRatioPattern {
    pub bps: MetricPattern1<BasisPoints16>,
    pub percent: MetricPattern1<StoredF32>,
    pub ratio: MetricPattern1<StoredF32>,
}

impl BpsPercentRatioPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            bps: MetricPattern1::new(client.clone(), _m(&acc, "bps")),
            percent: MetricPattern1::new(client.clone(), acc.clone()),
            ratio: MetricPattern1::new(client.clone(), _m(&acc, "ratio")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct BpsPriceRatioPattern {
    pub bps: MetricPattern1<BasisPoints32>,
    pub price: CentsSatsUsdPattern,
    pub ratio: MetricPattern1<StoredF32>,
}

impl BpsPriceRatioPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            bps: MetricPattern1::new(client.clone(), _m(&acc, "ratio_bps")),
            price: CentsSatsUsdPattern::new(client.clone(), acc.clone()),
            ratio: MetricPattern1::new(client.clone(), _m(&acc, "ratio")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct CentsSatsUsdPattern2 {
    pub cents: MetricPattern2<Cents>,
    pub sats: MetricPattern2<Sats>,
    pub usd: MetricPattern2<Dollars>,
}

impl CentsSatsUsdPattern2 {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            cents: MetricPattern2::new(client.clone(), _m(&acc, "cents")),
            sats: MetricPattern2::new(client.clone(), _m(&acc, "sats")),
            usd: MetricPattern2::new(client.clone(), acc.clone()),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct CentsSatsUsdPattern {
    pub cents: MetricPattern1<Cents>,
    pub sats: MetricPattern1<SatsFract>,
    pub usd: MetricPattern1<Dollars>,
}

impl CentsSatsUsdPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            cents: MetricPattern1::new(client.clone(), _m(&acc, "cents")),
            sats: MetricPattern1::new(client.clone(), _m(&acc, "sats")),
            usd: MetricPattern1::new(client.clone(), _m(&acc, "usd")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct ChangeHalvedTotalPattern {
    pub change_1m: BtcCentsSatsUsdPattern,
    pub halved: BtcCentsSatsUsdPattern,
    pub total: BtcCentsSatsUsdPattern,
}

impl ChangeHalvedTotalPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            change_1m: BtcCentsSatsUsdPattern::new(client.clone(), _m(&acc, "change_1m")),
            halved: BtcCentsSatsUsdPattern::new(client.clone(), _m(&acc, "halved")),
            total: BtcCentsSatsUsdPattern::new(client.clone(), acc.clone()),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct _1m1w1yPattern<T> {
    pub _1m: MetricPattern1<T>,
    pub _1w: MetricPattern1<T>,
    pub _1y: MetricPattern1<T>,
}

impl<T: DeserializeOwned> _1m1w1yPattern<T> {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            _1m: MetricPattern1::new(client.clone(), _m(&acc, "1m")),
            _1w: MetricPattern1::new(client.clone(), _m(&acc, "1w")),
            _1y: MetricPattern1::new(client.clone(), _m(&acc, "1y")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct _6bBlockTxindexPattern<T> {
    pub _6b: AverageMaxMedianMinPct10Pct25Pct75Pct90Pattern<T>,
    pub block: AverageMaxMedianMinPct10Pct25Pct75Pct90Pattern<T>,
    pub txindex: MetricPattern19<T>,
}

impl<T: DeserializeOwned> _6bBlockTxindexPattern<T> {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            _6b: AverageMaxMedianMinPct10Pct25Pct75Pct90Pattern::new(client.clone(), _m(&acc, "6b")),
            block: AverageMaxMedianMinPct10Pct25Pct75Pct90Pattern::new(client.clone(), acc.clone()),
            txindex: MetricPattern19::new(client.clone(), acc.clone()),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct CumulativeHeightSumPattern<T> {
    pub cumulative: MetricPattern1<T>,
    pub height: MetricPattern18<T>,
    pub sum: _1m1w1y24hPattern<T>,
}

impl<T: DeserializeOwned> CumulativeHeightSumPattern<T> {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            cumulative: MetricPattern1::new(client.clone(), _m(&acc, "cumulative")),
            height: MetricPattern18::new(client.clone(), acc.clone()),
            sum: _1m1w1y24hPattern::new(client.clone(), acc.clone()),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct _1m1wPattern2 {
    pub _1m: BpsPercentRatioPattern,
    pub _1w: BpsPercentRatioPattern,
}

impl _1m1wPattern2 {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            _1m: BpsPercentRatioPattern::new(client.clone(), _m(&acc, "1m")),
            _1w: BpsPercentRatioPattern::new(client.clone(), _m(&acc, "1w")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct _1m1wPattern {
    pub _1m: MetricPattern1<StoredF64>,
    pub _1w: MetricPattern1<StoredF64>,
}

impl _1m1wPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            _1m: MetricPattern1::new(client.clone(), _m(&acc, "1m")),
            _1w: MetricPattern1::new(client.clone(), _m(&acc, "1w")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct BaseCumulativePattern {
    pub base: BtcCentsSatsUsdPattern,
    pub cumulative: BtcCentsSatsUsdPattern,
}

impl BaseCumulativePattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            base: BtcCentsSatsUsdPattern::new(client.clone(), acc.clone()),
            cumulative: BtcCentsSatsUsdPattern::new(client.clone(), _m(&acc, "cumulative")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct BlocksDominancePattern {
    pub blocks_mined: CumulativeHeightPattern<StoredU32>,
    pub dominance: BpsPercentRatioPattern,
}

impl BlocksDominancePattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            blocks_mined: CumulativeHeightPattern::new(client.clone(), _m(&acc, "blocks_mined")),
            dominance: BpsPercentRatioPattern::new(client.clone(), _m(&acc, "dominance")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct BpsRatioPattern {
    pub bps: MetricPattern1<BasisPoints32>,
    pub ratio: MetricPattern1<StoredF32>,
}

impl BpsRatioPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            bps: MetricPattern1::new(client.clone(), _m(&acc, "bps")),
            ratio: MetricPattern1::new(client.clone(), acc.clone()),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct CentsUsdPattern {
    pub cents: MetricPattern1<Cents>,
    pub usd: MetricPattern1<Dollars>,
}

impl CentsUsdPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            cents: MetricPattern1::new(client.clone(), _m(&acc, "cents")),
            usd: MetricPattern1::new(client.clone(), _m(&acc, "usd")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct ChangeRatePattern {
    pub change: _1m1w1y24hPattern<StoredI64>,
    pub rate: _1m1w1y24hPattern2,
}

impl ChangeRatePattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            change: _1m1w1y24hPattern::new(client.clone(), _m(&acc, "change")),
            rate: _1m1w1y24hPattern2::new(client.clone(), _m(&acc, "rate")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct MaxMinPattern {
    pub max: CentsSatsUsdPattern,
    pub min: CentsSatsUsdPattern,
}

impl MaxMinPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            max: CentsSatsUsdPattern::new(client.clone(), _m(&acc, "max")),
            min: CentsSatsUsdPattern::new(client.clone(), _m(&acc, "min")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct SdSmaPattern {
    pub sd: MetricPattern1<StoredF32>,
    pub sma: MetricPattern1<StoredF32>,
}

impl SdSmaPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            sd: MetricPattern1::new(client.clone(), _m(&acc, "sd_1y")),
            sma: MetricPattern1::new(client.clone(), _m(&acc, "sma_1y")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct SentPattern {
    pub sent: BaseCumulativePattern,
    pub sent_ema: _2wPattern,
}

impl SentPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            sent: BaseCumulativePattern::new(client.clone(), acc.clone()),
            sent_ema: _2wPattern::new(client.clone(), _m(&acc, "ema_2w")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct SupplyPattern {
    pub supply_in_loss: BtcCentsSatsUsdPattern,
    pub supply_in_profit: BtcCentsSatsUsdPattern,
}

impl SupplyPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            supply_in_loss: BtcCentsSatsUsdPattern::new(client.clone(), _m(&acc, "loss")),
            supply_in_profit: BtcCentsSatsUsdPattern::new(client.clone(), _m(&acc, "profit")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct UtxoPattern {
    pub utxo_count: MetricPattern1<StoredU64>,
    pub utxo_count_change_1m: MetricPattern1<StoredF64>,
}

impl UtxoPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            utxo_count: MetricPattern1::new(client.clone(), acc.clone()),
            utxo_count_change_1m: MetricPattern1::new(client.clone(), _m(&acc, "change_1m")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct CumulativeHeightPattern<T> {
    pub cumulative: MetricPattern1<T>,
    pub height: MetricPattern18<T>,
}

impl<T: DeserializeOwned> CumulativeHeightPattern<T> {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            cumulative: MetricPattern1::new(client.clone(), _m(&acc, "cumulative")),
            height: MetricPattern18::new(client.clone(), acc.clone()),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct _2wPattern {
    pub _2w: BtcCentsSatsUsdPattern,
}

impl _2wPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            _2w: BtcCentsSatsUsdPattern::new(client.clone(), acc.clone()),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct _24hPattern<T> {
    pub _24h: MetricPattern1<T>,
}

impl<T: DeserializeOwned> _24hPattern<T> {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            _24h: MetricPattern1::new(client.clone(), acc.clone()),
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
    pub mining: MetricsTree_Mining,
    pub positions: MetricsTree_Positions,
    pub cointime: MetricsTree_Cointime,
    pub constants: MetricsTree_Constants,
    pub indexes: MetricsTree_Indexes,
    pub market: MetricsTree_Market,
    pub pools: MetricsTree_Pools,
    pub prices: MetricsTree_Prices,
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
            mining: MetricsTree_Mining::new(client.clone(), format!("{base_path}_mining")),
            positions: MetricsTree_Positions::new(client.clone(), format!("{base_path}_positions")),
            cointime: MetricsTree_Cointime::new(client.clone(), format!("{base_path}_cointime")),
            constants: MetricsTree_Constants::new(client.clone(), format!("{base_path}_constants")),
            indexes: MetricsTree_Indexes::new(client.clone(), format!("{base_path}_indexes")),
            market: MetricsTree_Market::new(client.clone(), format!("{base_path}_market")),
            pools: MetricsTree_Pools::new(client.clone(), format!("{base_path}_pools")),
            prices: MetricsTree_Prices::new(client.clone(), format!("{base_path}_prices")),
            distribution: MetricsTree_Distribution::new(client.clone(), format!("{base_path}_distribution")),
            supply: MetricsTree_Supply::new(client.clone(), format!("{base_path}_supply")),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Blocks {
    pub blockhash: MetricPattern18<BlockHash>,
    pub difficulty: MetricsTree_Blocks_Difficulty,
    pub time: MetricsTree_Blocks_Time,
    pub total_size: MetricPattern18<StoredU64>,
    pub weight: MetricsTree_Blocks_Weight,
    pub count: MetricsTree_Blocks_Count,
    pub interval: _1m1w1y24hHeightPattern<Timestamp>,
    pub halving: MetricsTree_Blocks_Halving,
    pub vbytes: AverageCumulativeHeightMaxMedianMinPct10Pct25Pct75Pct90SumPattern,
    pub size: MetricsTree_Blocks_Size,
    pub fullness: MetricsTree_Blocks_Fullness,
}

impl MetricsTree_Blocks {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            blockhash: MetricPattern18::new(client.clone(), "blockhash".to_string()),
            difficulty: MetricsTree_Blocks_Difficulty::new(client.clone(), format!("{base_path}_difficulty")),
            time: MetricsTree_Blocks_Time::new(client.clone(), format!("{base_path}_time")),
            total_size: MetricPattern18::new(client.clone(), "total_size".to_string()),
            weight: MetricsTree_Blocks_Weight::new(client.clone(), format!("{base_path}_weight")),
            count: MetricsTree_Blocks_Count::new(client.clone(), format!("{base_path}_count")),
            interval: _1m1w1y24hHeightPattern::new(client.clone(), "block_interval".to_string()),
            halving: MetricsTree_Blocks_Halving::new(client.clone(), format!("{base_path}_halving")),
            vbytes: AverageCumulativeHeightMaxMedianMinPct10Pct25Pct75Pct90SumPattern::new(client.clone(), "block_vbytes".to_string()),
            size: MetricsTree_Blocks_Size::new(client.clone(), format!("{base_path}_size")),
            fullness: MetricsTree_Blocks_Fullness::new(client.clone(), format!("{base_path}_fullness")),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Blocks_Difficulty {
    pub raw: MetricPattern1<StoredF64>,
    pub as_hash: MetricPattern1<StoredF64>,
    pub adjustment: BpsPercentRatioPattern,
    pub epoch: MetricPattern1<DifficultyEpoch>,
    pub blocks_before_next_adjustment: MetricPattern1<StoredU32>,
    pub days_before_next_adjustment: MetricPattern1<StoredF32>,
}

impl MetricsTree_Blocks_Difficulty {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            raw: MetricPattern1::new(client.clone(), "difficulty".to_string()),
            as_hash: MetricPattern1::new(client.clone(), "difficulty_as_hash".to_string()),
            adjustment: BpsPercentRatioPattern::new(client.clone(), "difficulty_adjustment".to_string()),
            epoch: MetricPattern1::new(client.clone(), "difficulty_epoch".to_string()),
            blocks_before_next_adjustment: MetricPattern1::new(client.clone(), "blocks_before_next_difficulty_adjustment".to_string()),
            days_before_next_adjustment: MetricPattern1::new(client.clone(), "days_before_next_difficulty_adjustment".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Blocks_Time {
    pub timestamp: MetricPattern1<Timestamp>,
    pub date: MetricPattern18<Date>,
    pub timestamp_monotonic: MetricPattern18<Timestamp>,
}

impl MetricsTree_Blocks_Time {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            timestamp: MetricPattern1::new(client.clone(), "timestamp".to_string()),
            date: MetricPattern18::new(client.clone(), "date".to_string()),
            timestamp_monotonic: MetricPattern18::new(client.clone(), "timestamp_monotonic".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Blocks_Weight {
    pub base: MetricPattern18<Weight>,
    pub cumulative: MetricPattern1<Weight>,
    pub sum: _1m1w1y24hPattern<Weight>,
    pub average: _1m1w1y24hPattern<Weight>,
    pub min: _1m1w1y24hPattern<Weight>,
    pub max: _1m1w1y24hPattern<Weight>,
    pub pct10: _1m1w1y24hPattern<Weight>,
    pub pct25: _1m1w1y24hPattern<Weight>,
    pub median: _1m1w1y24hPattern<Weight>,
    pub pct75: _1m1w1y24hPattern<Weight>,
    pub pct90: _1m1w1y24hPattern<Weight>,
}

impl MetricsTree_Blocks_Weight {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            base: MetricPattern18::new(client.clone(), "block_weight".to_string()),
            cumulative: MetricPattern1::new(client.clone(), "block_weight_cumulative".to_string()),
            sum: _1m1w1y24hPattern::new(client.clone(), "block_weight_sum".to_string()),
            average: _1m1w1y24hPattern::new(client.clone(), "block_weight_average".to_string()),
            min: _1m1w1y24hPattern::new(client.clone(), "block_weight_min".to_string()),
            max: _1m1w1y24hPattern::new(client.clone(), "block_weight_max".to_string()),
            pct10: _1m1w1y24hPattern::new(client.clone(), "block_weight_p10".to_string()),
            pct25: _1m1w1y24hPattern::new(client.clone(), "block_weight_p25".to_string()),
            median: _1m1w1y24hPattern::new(client.clone(), "block_weight_median".to_string()),
            pct75: _1m1w1y24hPattern::new(client.clone(), "block_weight_p75".to_string()),
            pct90: _1m1w1y24hPattern::new(client.clone(), "block_weight_p90".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Blocks_Count {
    pub block_count_target: MetricPattern1<StoredU64>,
    pub block_count: CumulativeHeightSumPattern<StoredU32>,
    pub block_count_sum: _1m1w1y24hPattern<StoredU32>,
    pub height_1h_ago: MetricPattern18<Height>,
    pub height_24h_ago: MetricPattern18<Height>,
    pub height_3d_ago: MetricPattern18<Height>,
    pub height_1w_ago: MetricPattern18<Height>,
    pub height_8d_ago: MetricPattern18<Height>,
    pub height_9d_ago: MetricPattern18<Height>,
    pub height_12d_ago: MetricPattern18<Height>,
    pub height_13d_ago: MetricPattern18<Height>,
    pub height_2w_ago: MetricPattern18<Height>,
    pub height_21d_ago: MetricPattern18<Height>,
    pub height_26d_ago: MetricPattern18<Height>,
    pub height_1m_ago: MetricPattern18<Height>,
    pub height_34d_ago: MetricPattern18<Height>,
    pub height_55d_ago: MetricPattern18<Height>,
    pub height_2m_ago: MetricPattern18<Height>,
    pub height_9w_ago: MetricPattern18<Height>,
    pub height_12w_ago: MetricPattern18<Height>,
    pub height_89d_ago: MetricPattern18<Height>,
    pub height_3m_ago: MetricPattern18<Height>,
    pub height_14w_ago: MetricPattern18<Height>,
    pub height_111d_ago: MetricPattern18<Height>,
    pub height_144d_ago: MetricPattern18<Height>,
    pub height_6m_ago: MetricPattern18<Height>,
    pub height_26w_ago: MetricPattern18<Height>,
    pub height_200d_ago: MetricPattern18<Height>,
    pub height_9m_ago: MetricPattern18<Height>,
    pub height_350d_ago: MetricPattern18<Height>,
    pub height_12m_ago: MetricPattern18<Height>,
    pub height_1y_ago: MetricPattern18<Height>,
    pub height_14m_ago: MetricPattern18<Height>,
    pub height_2y_ago: MetricPattern18<Height>,
    pub height_26m_ago: MetricPattern18<Height>,
    pub height_3y_ago: MetricPattern18<Height>,
    pub height_200w_ago: MetricPattern18<Height>,
    pub height_4y_ago: MetricPattern18<Height>,
    pub height_5y_ago: MetricPattern18<Height>,
    pub height_6y_ago: MetricPattern18<Height>,
    pub height_8y_ago: MetricPattern18<Height>,
    pub height_9y_ago: MetricPattern18<Height>,
    pub height_10y_ago: MetricPattern18<Height>,
    pub height_12y_ago: MetricPattern18<Height>,
    pub height_14y_ago: MetricPattern18<Height>,
    pub height_26y_ago: MetricPattern18<Height>,
}

impl MetricsTree_Blocks_Count {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            block_count_target: MetricPattern1::new(client.clone(), "block_count_target".to_string()),
            block_count: CumulativeHeightSumPattern::new(client.clone(), "block_count".to_string()),
            block_count_sum: _1m1w1y24hPattern::new(client.clone(), "block_count_sum".to_string()),
            height_1h_ago: MetricPattern18::new(client.clone(), "height_1h_ago".to_string()),
            height_24h_ago: MetricPattern18::new(client.clone(), "height_24h_ago".to_string()),
            height_3d_ago: MetricPattern18::new(client.clone(), "height_3d_ago".to_string()),
            height_1w_ago: MetricPattern18::new(client.clone(), "height_1w_ago".to_string()),
            height_8d_ago: MetricPattern18::new(client.clone(), "height_8d_ago".to_string()),
            height_9d_ago: MetricPattern18::new(client.clone(), "height_9d_ago".to_string()),
            height_12d_ago: MetricPattern18::new(client.clone(), "height_12d_ago".to_string()),
            height_13d_ago: MetricPattern18::new(client.clone(), "height_13d_ago".to_string()),
            height_2w_ago: MetricPattern18::new(client.clone(), "height_2w_ago".to_string()),
            height_21d_ago: MetricPattern18::new(client.clone(), "height_21d_ago".to_string()),
            height_26d_ago: MetricPattern18::new(client.clone(), "height_26d_ago".to_string()),
            height_1m_ago: MetricPattern18::new(client.clone(), "height_1m_ago".to_string()),
            height_34d_ago: MetricPattern18::new(client.clone(), "height_34d_ago".to_string()),
            height_55d_ago: MetricPattern18::new(client.clone(), "height_55d_ago".to_string()),
            height_2m_ago: MetricPattern18::new(client.clone(), "height_2m_ago".to_string()),
            height_9w_ago: MetricPattern18::new(client.clone(), "height_9w_ago".to_string()),
            height_12w_ago: MetricPattern18::new(client.clone(), "height_12w_ago".to_string()),
            height_89d_ago: MetricPattern18::new(client.clone(), "height_89d_ago".to_string()),
            height_3m_ago: MetricPattern18::new(client.clone(), "height_3m_ago".to_string()),
            height_14w_ago: MetricPattern18::new(client.clone(), "height_14w_ago".to_string()),
            height_111d_ago: MetricPattern18::new(client.clone(), "height_111d_ago".to_string()),
            height_144d_ago: MetricPattern18::new(client.clone(), "height_144d_ago".to_string()),
            height_6m_ago: MetricPattern18::new(client.clone(), "height_6m_ago".to_string()),
            height_26w_ago: MetricPattern18::new(client.clone(), "height_26w_ago".to_string()),
            height_200d_ago: MetricPattern18::new(client.clone(), "height_200d_ago".to_string()),
            height_9m_ago: MetricPattern18::new(client.clone(), "height_9m_ago".to_string()),
            height_350d_ago: MetricPattern18::new(client.clone(), "height_350d_ago".to_string()),
            height_12m_ago: MetricPattern18::new(client.clone(), "height_12m_ago".to_string()),
            height_1y_ago: MetricPattern18::new(client.clone(), "height_1y_ago".to_string()),
            height_14m_ago: MetricPattern18::new(client.clone(), "height_14m_ago".to_string()),
            height_2y_ago: MetricPattern18::new(client.clone(), "height_2y_ago".to_string()),
            height_26m_ago: MetricPattern18::new(client.clone(), "height_26m_ago".to_string()),
            height_3y_ago: MetricPattern18::new(client.clone(), "height_3y_ago".to_string()),
            height_200w_ago: MetricPattern18::new(client.clone(), "height_200w_ago".to_string()),
            height_4y_ago: MetricPattern18::new(client.clone(), "height_4y_ago".to_string()),
            height_5y_ago: MetricPattern18::new(client.clone(), "height_5y_ago".to_string()),
            height_6y_ago: MetricPattern18::new(client.clone(), "height_6y_ago".to_string()),
            height_8y_ago: MetricPattern18::new(client.clone(), "height_8y_ago".to_string()),
            height_9y_ago: MetricPattern18::new(client.clone(), "height_9y_ago".to_string()),
            height_10y_ago: MetricPattern18::new(client.clone(), "height_10y_ago".to_string()),
            height_12y_ago: MetricPattern18::new(client.clone(), "height_12y_ago".to_string()),
            height_14y_ago: MetricPattern18::new(client.clone(), "height_14y_ago".to_string()),
            height_26y_ago: MetricPattern18::new(client.clone(), "height_26y_ago".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Blocks_Halving {
    pub epoch: MetricPattern1<HalvingEpoch>,
    pub blocks_before_next_halving: MetricPattern1<StoredU32>,
    pub days_before_next_halving: MetricPattern1<StoredF32>,
}

impl MetricsTree_Blocks_Halving {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            epoch: MetricPattern1::new(client.clone(), "halving_epoch".to_string()),
            blocks_before_next_halving: MetricPattern1::new(client.clone(), "blocks_before_next_halving".to_string()),
            days_before_next_halving: MetricPattern1::new(client.clone(), "days_before_next_halving".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Blocks_Size {
    pub cumulative: MetricPattern1<StoredU64>,
    pub sum: _1m1w1y24hPattern<StoredU64>,
    pub average: _1m1w1y24hPattern<StoredU64>,
    pub min: _1m1w1y24hPattern<StoredU64>,
    pub max: _1m1w1y24hPattern<StoredU64>,
    pub pct10: _1m1w1y24hPattern<StoredU64>,
    pub pct25: _1m1w1y24hPattern<StoredU64>,
    pub median: _1m1w1y24hPattern<StoredU64>,
    pub pct75: _1m1w1y24hPattern<StoredU64>,
    pub pct90: _1m1w1y24hPattern<StoredU64>,
}

impl MetricsTree_Blocks_Size {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            cumulative: MetricPattern1::new(client.clone(), "block_size_cumulative".to_string()),
            sum: _1m1w1y24hPattern::new(client.clone(), "block_size_sum".to_string()),
            average: _1m1w1y24hPattern::new(client.clone(), "block_size_average".to_string()),
            min: _1m1w1y24hPattern::new(client.clone(), "block_size_min".to_string()),
            max: _1m1w1y24hPattern::new(client.clone(), "block_size_max".to_string()),
            pct10: _1m1w1y24hPattern::new(client.clone(), "block_size_p10".to_string()),
            pct25: _1m1w1y24hPattern::new(client.clone(), "block_size_p25".to_string()),
            median: _1m1w1y24hPattern::new(client.clone(), "block_size_median".to_string()),
            pct75: _1m1w1y24hPattern::new(client.clone(), "block_size_p75".to_string()),
            pct90: _1m1w1y24hPattern::new(client.clone(), "block_size_p90".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Blocks_Fullness {
    pub bps: _1m1w1y24hHeightPattern<BasisPoints16>,
    pub ratio: MetricPattern1<StoredF32>,
    pub percent: MetricPattern1<StoredF32>,
}

impl MetricsTree_Blocks_Fullness {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            bps: _1m1w1y24hHeightPattern::new(client.clone(), "block_fullness_bps".to_string()),
            ratio: MetricPattern1::new(client.clone(), "block_fullness_ratio".to_string()),
            percent: MetricPattern1::new(client.clone(), "block_fullness".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Transactions {
    pub first_txindex: MetricPattern18<TxIndex>,
    pub height: MetricPattern19<Height>,
    pub txid: MetricPattern19<Txid>,
    pub txversion: MetricPattern19<TxVersion>,
    pub rawlocktime: MetricPattern19<RawLockTime>,
    pub base_size: MetricPattern19<StoredU32>,
    pub total_size: MetricPattern19<StoredU32>,
    pub is_explicitly_rbf: MetricPattern19<StoredBool>,
    pub first_txinindex: MetricPattern19<TxInIndex>,
    pub first_txoutindex: MetricPattern19<TxOutIndex>,
    pub count: MetricsTree_Transactions_Count,
    pub size: MetricsTree_Transactions_Size,
    pub fees: MetricsTree_Transactions_Fees,
    pub versions: MetricsTree_Transactions_Versions,
    pub volume: MetricsTree_Transactions_Volume,
}

impl MetricsTree_Transactions {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            first_txindex: MetricPattern18::new(client.clone(), "first_txindex".to_string()),
            height: MetricPattern19::new(client.clone(), "height".to_string()),
            txid: MetricPattern19::new(client.clone(), "txid".to_string()),
            txversion: MetricPattern19::new(client.clone(), "txversion".to_string()),
            rawlocktime: MetricPattern19::new(client.clone(), "rawlocktime".to_string()),
            base_size: MetricPattern19::new(client.clone(), "base_size".to_string()),
            total_size: MetricPattern19::new(client.clone(), "total_size".to_string()),
            is_explicitly_rbf: MetricPattern19::new(client.clone(), "is_explicitly_rbf".to_string()),
            first_txinindex: MetricPattern19::new(client.clone(), "first_txinindex".to_string()),
            first_txoutindex: MetricPattern19::new(client.clone(), "first_txoutindex".to_string()),
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
    pub tx_count: AverageCumulativeHeightMaxMedianMinPct10Pct25Pct75Pct90SumPattern,
    pub is_coinbase: MetricPattern19<StoredBool>,
}

impl MetricsTree_Transactions_Count {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            tx_count: AverageCumulativeHeightMaxMedianMinPct10Pct25Pct75Pct90SumPattern::new(client.clone(), "tx_count".to_string()),
            is_coinbase: MetricPattern19::new(client.clone(), "is_coinbase".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Transactions_Size {
    pub vsize: _6bBlockTxindexPattern<VSize>,
    pub weight: _6bBlockTxindexPattern<Weight>,
}

impl MetricsTree_Transactions_Size {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            vsize: _6bBlockTxindexPattern::new(client.clone(), "tx_vsize".to_string()),
            weight: _6bBlockTxindexPattern::new(client.clone(), "tx_weight".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Transactions_Fees {
    pub input_value: MetricPattern19<Sats>,
    pub output_value: MetricPattern19<Sats>,
    pub fee: _6bBlockTxindexPattern<Sats>,
    pub fee_rate: _6bBlockTxindexPattern<FeeRate>,
}

impl MetricsTree_Transactions_Fees {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            input_value: MetricPattern19::new(client.clone(), "input_value".to_string()),
            output_value: MetricPattern19::new(client.clone(), "output_value".to_string()),
            fee: _6bBlockTxindexPattern::new(client.clone(), "fee".to_string()),
            fee_rate: _6bBlockTxindexPattern::new(client.clone(), "fee_rate".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Transactions_Versions {
    pub v1: CumulativeHeightSumPattern<StoredU64>,
    pub v2: CumulativeHeightSumPattern<StoredU64>,
    pub v3: CumulativeHeightSumPattern<StoredU64>,
}

impl MetricsTree_Transactions_Versions {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            v1: CumulativeHeightSumPattern::new(client.clone(), "tx_v1".to_string()),
            v2: CumulativeHeightSumPattern::new(client.clone(), "tx_v2".to_string()),
            v3: CumulativeHeightSumPattern::new(client.clone(), "tx_v3".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Transactions_Volume {
    pub sent_sum: _1m1w1y24hBtcCentsSatsUsdPattern,
    pub received_sum: _1m1w1y24hBtcCentsSatsUsdPattern,
    pub annualized_volume: BtcCentsSatsUsdPattern,
    pub tx_per_sec: MetricPattern1<StoredF32>,
    pub outputs_per_sec: MetricPattern1<StoredF32>,
    pub inputs_per_sec: MetricPattern1<StoredF32>,
}

impl MetricsTree_Transactions_Volume {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            sent_sum: _1m1w1y24hBtcCentsSatsUsdPattern::new(client.clone(), "sent_sum".to_string()),
            received_sum: _1m1w1y24hBtcCentsSatsUsdPattern::new(client.clone(), "received_sum".to_string()),
            annualized_volume: BtcCentsSatsUsdPattern::new(client.clone(), "annualized_volume".to_string()),
            tx_per_sec: MetricPattern1::new(client.clone(), "tx_per_sec".to_string()),
            outputs_per_sec: MetricPattern1::new(client.clone(), "outputs_per_sec".to_string()),
            inputs_per_sec: MetricPattern1::new(client.clone(), "inputs_per_sec".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Inputs {
    pub first_txinindex: MetricPattern18<TxInIndex>,
    pub outpoint: MetricPattern20<OutPoint>,
    pub txindex: MetricPattern20<TxIndex>,
    pub outputtype: MetricPattern20<OutputType>,
    pub typeindex: MetricPattern20<TypeIndex>,
    pub spent: MetricsTree_Inputs_Spent,
    pub count: AverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90RollingSumPattern,
}

impl MetricsTree_Inputs {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            first_txinindex: MetricPattern18::new(client.clone(), "first_txinindex".to_string()),
            outpoint: MetricPattern20::new(client.clone(), "outpoint".to_string()),
            txindex: MetricPattern20::new(client.clone(), "txindex".to_string()),
            outputtype: MetricPattern20::new(client.clone(), "outputtype".to_string()),
            typeindex: MetricPattern20::new(client.clone(), "typeindex".to_string()),
            spent: MetricsTree_Inputs_Spent::new(client.clone(), format!("{base_path}_spent")),
            count: AverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90RollingSumPattern::new(client.clone(), "input_count".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Inputs_Spent {
    pub txoutindex: MetricPattern20<TxOutIndex>,
    pub value: MetricPattern20<Sats>,
}

impl MetricsTree_Inputs_Spent {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            txoutindex: MetricPattern20::new(client.clone(), "txoutindex".to_string()),
            value: MetricPattern20::new(client.clone(), "value".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Outputs {
    pub first_txoutindex: MetricPattern18<TxOutIndex>,
    pub value: MetricPattern21<Sats>,
    pub outputtype: MetricPattern21<OutputType>,
    pub typeindex: MetricPattern21<TypeIndex>,
    pub txindex: MetricPattern21<TxIndex>,
    pub spent: MetricsTree_Outputs_Spent,
    pub count: MetricsTree_Outputs_Count,
}

impl MetricsTree_Outputs {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            first_txoutindex: MetricPattern18::new(client.clone(), "first_txoutindex".to_string()),
            value: MetricPattern21::new(client.clone(), "value".to_string()),
            outputtype: MetricPattern21::new(client.clone(), "outputtype".to_string()),
            typeindex: MetricPattern21::new(client.clone(), "typeindex".to_string()),
            txindex: MetricPattern21::new(client.clone(), "txindex".to_string()),
            spent: MetricsTree_Outputs_Spent::new(client.clone(), format!("{base_path}_spent")),
            count: MetricsTree_Outputs_Count::new(client.clone(), format!("{base_path}_count")),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Outputs_Spent {
    pub txinindex: MetricPattern21<TxInIndex>,
}

impl MetricsTree_Outputs_Spent {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            txinindex: MetricPattern21::new(client.clone(), "txinindex".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Outputs_Count {
    pub total_count: AverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90RollingSumPattern,
    pub utxo_count: MetricPattern1<StoredU64>,
}

impl MetricsTree_Outputs_Count {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            total_count: AverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90RollingSumPattern::new(client.clone(), "output_count".to_string()),
            utxo_count: MetricPattern1::new(client.clone(), "exact_utxo_count".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Addresses {
    pub first_p2pk65addressindex: MetricPattern18<P2PK65AddressIndex>,
    pub first_p2pk33addressindex: MetricPattern18<P2PK33AddressIndex>,
    pub first_p2pkhaddressindex: MetricPattern18<P2PKHAddressIndex>,
    pub first_p2shaddressindex: MetricPattern18<P2SHAddressIndex>,
    pub first_p2wpkhaddressindex: MetricPattern18<P2WPKHAddressIndex>,
    pub first_p2wshaddressindex: MetricPattern18<P2WSHAddressIndex>,
    pub first_p2traddressindex: MetricPattern18<P2TRAddressIndex>,
    pub first_p2aaddressindex: MetricPattern18<P2AAddressIndex>,
    pub p2pk65bytes: MetricPattern27<P2PK65Bytes>,
    pub p2pk33bytes: MetricPattern26<P2PK33Bytes>,
    pub p2pkhbytes: MetricPattern28<P2PKHBytes>,
    pub p2shbytes: MetricPattern29<P2SHBytes>,
    pub p2wpkhbytes: MetricPattern31<P2WPKHBytes>,
    pub p2wshbytes: MetricPattern32<P2WSHBytes>,
    pub p2trbytes: MetricPattern30<P2TRBytes>,
    pub p2abytes: MetricPattern24<P2ABytes>,
}

impl MetricsTree_Addresses {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            first_p2pk65addressindex: MetricPattern18::new(client.clone(), "first_p2pk65addressindex".to_string()),
            first_p2pk33addressindex: MetricPattern18::new(client.clone(), "first_p2pk33addressindex".to_string()),
            first_p2pkhaddressindex: MetricPattern18::new(client.clone(), "first_p2pkhaddressindex".to_string()),
            first_p2shaddressindex: MetricPattern18::new(client.clone(), "first_p2shaddressindex".to_string()),
            first_p2wpkhaddressindex: MetricPattern18::new(client.clone(), "first_p2wpkhaddressindex".to_string()),
            first_p2wshaddressindex: MetricPattern18::new(client.clone(), "first_p2wshaddressindex".to_string()),
            first_p2traddressindex: MetricPattern18::new(client.clone(), "first_p2traddressindex".to_string()),
            first_p2aaddressindex: MetricPattern18::new(client.clone(), "first_p2aaddressindex".to_string()),
            p2pk65bytes: MetricPattern27::new(client.clone(), "p2pk65bytes".to_string()),
            p2pk33bytes: MetricPattern26::new(client.clone(), "p2pk33bytes".to_string()),
            p2pkhbytes: MetricPattern28::new(client.clone(), "p2pkhbytes".to_string()),
            p2shbytes: MetricPattern29::new(client.clone(), "p2shbytes".to_string()),
            p2wpkhbytes: MetricPattern31::new(client.clone(), "p2wpkhbytes".to_string()),
            p2wshbytes: MetricPattern32::new(client.clone(), "p2wshbytes".to_string()),
            p2trbytes: MetricPattern30::new(client.clone(), "p2trbytes".to_string()),
            p2abytes: MetricPattern24::new(client.clone(), "p2abytes".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Scripts {
    pub first_emptyoutputindex: MetricPattern18<EmptyOutputIndex>,
    pub first_opreturnindex: MetricPattern18<OpReturnIndex>,
    pub first_p2msoutputindex: MetricPattern18<P2MSOutputIndex>,
    pub first_unknownoutputindex: MetricPattern18<UnknownOutputIndex>,
    pub empty_to_txindex: MetricPattern22<TxIndex>,
    pub opreturn_to_txindex: MetricPattern23<TxIndex>,
    pub p2ms_to_txindex: MetricPattern25<TxIndex>,
    pub unknown_to_txindex: MetricPattern33<TxIndex>,
    pub count: MetricsTree_Scripts_Count,
    pub value: MetricsTree_Scripts_Value,
    pub adoption: MetricsTree_Scripts_Adoption,
}

impl MetricsTree_Scripts {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            first_emptyoutputindex: MetricPattern18::new(client.clone(), "first_emptyoutputindex".to_string()),
            first_opreturnindex: MetricPattern18::new(client.clone(), "first_opreturnindex".to_string()),
            first_p2msoutputindex: MetricPattern18::new(client.clone(), "first_p2msoutputindex".to_string()),
            first_unknownoutputindex: MetricPattern18::new(client.clone(), "first_unknownoutputindex".to_string()),
            empty_to_txindex: MetricPattern22::new(client.clone(), "txindex".to_string()),
            opreturn_to_txindex: MetricPattern23::new(client.clone(), "txindex".to_string()),
            p2ms_to_txindex: MetricPattern25::new(client.clone(), "txindex".to_string()),
            unknown_to_txindex: MetricPattern33::new(client.clone(), "txindex".to_string()),
            count: MetricsTree_Scripts_Count::new(client.clone(), format!("{base_path}_count")),
            value: MetricsTree_Scripts_Value::new(client.clone(), format!("{base_path}_value")),
            adoption: MetricsTree_Scripts_Adoption::new(client.clone(), format!("{base_path}_adoption")),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Scripts_Count {
    pub p2a: CumulativeHeightSumPattern<StoredU64>,
    pub p2ms: CumulativeHeightSumPattern<StoredU64>,
    pub p2pk33: CumulativeHeightSumPattern<StoredU64>,
    pub p2pk65: CumulativeHeightSumPattern<StoredU64>,
    pub p2pkh: CumulativeHeightSumPattern<StoredU64>,
    pub p2sh: CumulativeHeightSumPattern<StoredU64>,
    pub p2tr: CumulativeHeightSumPattern<StoredU64>,
    pub p2wpkh: CumulativeHeightSumPattern<StoredU64>,
    pub p2wsh: CumulativeHeightSumPattern<StoredU64>,
    pub opreturn: CumulativeHeightSumPattern<StoredU64>,
    pub emptyoutput: CumulativeHeightSumPattern<StoredU64>,
    pub unknownoutput: CumulativeHeightSumPattern<StoredU64>,
    pub segwit: CumulativeHeightSumPattern<StoredU64>,
}

impl MetricsTree_Scripts_Count {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            p2a: CumulativeHeightSumPattern::new(client.clone(), "p2a_count".to_string()),
            p2ms: CumulativeHeightSumPattern::new(client.clone(), "p2ms_count".to_string()),
            p2pk33: CumulativeHeightSumPattern::new(client.clone(), "p2pk33_count".to_string()),
            p2pk65: CumulativeHeightSumPattern::new(client.clone(), "p2pk65_count".to_string()),
            p2pkh: CumulativeHeightSumPattern::new(client.clone(), "p2pkh_count".to_string()),
            p2sh: CumulativeHeightSumPattern::new(client.clone(), "p2sh_count".to_string()),
            p2tr: CumulativeHeightSumPattern::new(client.clone(), "p2tr_count".to_string()),
            p2wpkh: CumulativeHeightSumPattern::new(client.clone(), "p2wpkh_count".to_string()),
            p2wsh: CumulativeHeightSumPattern::new(client.clone(), "p2wsh_count".to_string()),
            opreturn: CumulativeHeightSumPattern::new(client.clone(), "opreturn_count".to_string()),
            emptyoutput: CumulativeHeightSumPattern::new(client.clone(), "emptyoutput_count".to_string()),
            unknownoutput: CumulativeHeightSumPattern::new(client.clone(), "unknownoutput_count".to_string()),
            segwit: CumulativeHeightSumPattern::new(client.clone(), "segwit_count".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Scripts_Value {
    pub opreturn: BaseCumulativePattern,
}

impl MetricsTree_Scripts_Value {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            opreturn: BaseCumulativePattern::new(client.clone(), "opreturn_value".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Scripts_Adoption {
    pub taproot: BpsPercentRatioPattern,
    pub segwit: BpsPercentRatioPattern,
}

impl MetricsTree_Scripts_Adoption {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            taproot: BpsPercentRatioPattern::new(client.clone(), "taproot_adoption".to_string()),
            segwit: BpsPercentRatioPattern::new(client.clone(), "segwit_adoption".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Mining {
    pub rewards: MetricsTree_Mining_Rewards,
    pub hashrate: MetricsTree_Mining_Hashrate,
}

impl MetricsTree_Mining {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            rewards: MetricsTree_Mining_Rewards::new(client.clone(), format!("{base_path}_rewards")),
            hashrate: MetricsTree_Mining_Hashrate::new(client.clone(), format!("{base_path}_hashrate")),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Mining_Rewards {
    pub coinbase: BaseCumulativeSumPattern,
    pub subsidy: BaseCumulativePattern,
    pub fees: MetricsTree_Mining_Rewards_Fees,
    pub unclaimed_rewards: BaseCumulativeSumPattern,
    pub fee_dominance: BpsPercentRatioPattern,
    pub fee_dominance_rolling: _1m1w1y24hPattern2,
    pub subsidy_dominance: BpsPercentRatioPattern,
    pub subsidy_dominance_rolling: _1m1w1y24hPattern2,
    pub subsidy_sma_1y: CentsUsdPattern,
}

impl MetricsTree_Mining_Rewards {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            coinbase: BaseCumulativeSumPattern::new(client.clone(), "coinbase".to_string()),
            subsidy: BaseCumulativePattern::new(client.clone(), "subsidy".to_string()),
            fees: MetricsTree_Mining_Rewards_Fees::new(client.clone(), format!("{base_path}_fees")),
            unclaimed_rewards: BaseCumulativeSumPattern::new(client.clone(), "unclaimed_rewards".to_string()),
            fee_dominance: BpsPercentRatioPattern::new(client.clone(), "fee_dominance".to_string()),
            fee_dominance_rolling: _1m1w1y24hPattern2::new(client.clone(), "fee_dominance".to_string()),
            subsidy_dominance: BpsPercentRatioPattern::new(client.clone(), "subsidy_dominance".to_string()),
            subsidy_dominance_rolling: _1m1w1y24hPattern2::new(client.clone(), "subsidy_dominance".to_string()),
            subsidy_sma_1y: CentsUsdPattern::new(client.clone(), "subsidy_sma_1y".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Mining_Rewards_Fees {
    pub base: BtcCentsSatsUsdPattern,
    pub cumulative: BtcCentsSatsUsdPattern,
    pub _24h: AverageMaxMedianMinPct10Pct25Pct75Pct90SumPattern2,
    pub _1w: AverageMaxMedianMinPct10Pct25Pct75Pct90SumPattern2,
    pub _1m: AverageMaxMedianMinPct10Pct25Pct75Pct90SumPattern2,
    pub _1y: AverageMaxMedianMinPct10Pct25Pct75Pct90SumPattern2,
}

impl MetricsTree_Mining_Rewards_Fees {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            base: BtcCentsSatsUsdPattern::new(client.clone(), "fees".to_string()),
            cumulative: BtcCentsSatsUsdPattern::new(client.clone(), "fees_cumulative".to_string()),
            _24h: AverageMaxMedianMinPct10Pct25Pct75Pct90SumPattern2::new(client.clone(), "fees_24h".to_string()),
            _1w: AverageMaxMedianMinPct10Pct25Pct75Pct90SumPattern2::new(client.clone(), "fees_1w".to_string()),
            _1m: AverageMaxMedianMinPct10Pct25Pct75Pct90SumPattern2::new(client.clone(), "fees_1m".to_string()),
            _1y: AverageMaxMedianMinPct10Pct25Pct75Pct90SumPattern2::new(client.clone(), "fees_1y".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Mining_Hashrate {
    pub hash_rate: MetricPattern1<StoredF64>,
    pub hash_rate_sma_1w: MetricPattern1<StoredF64>,
    pub hash_rate_sma_1m: MetricPattern1<StoredF64>,
    pub hash_rate_sma_2m: MetricPattern1<StoredF64>,
    pub hash_rate_sma_1y: MetricPattern1<StoredF64>,
    pub hash_rate_ath: MetricPattern1<StoredF64>,
    pub hash_rate_drawdown: BpsPercentRatioPattern,
    pub hash_price_ths: MetricPattern1<StoredF32>,
    pub hash_price_ths_min: MetricPattern1<StoredF32>,
    pub hash_price_phs: MetricPattern1<StoredF32>,
    pub hash_price_phs_min: MetricPattern1<StoredF32>,
    pub hash_price_rebound: BpsPercentRatioPattern,
    pub hash_value_ths: MetricPattern1<StoredF32>,
    pub hash_value_ths_min: MetricPattern1<StoredF32>,
    pub hash_value_phs: MetricPattern1<StoredF32>,
    pub hash_value_phs_min: MetricPattern1<StoredF32>,
    pub hash_value_rebound: BpsPercentRatioPattern,
}

impl MetricsTree_Mining_Hashrate {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            hash_rate: MetricPattern1::new(client.clone(), "hash_rate".to_string()),
            hash_rate_sma_1w: MetricPattern1::new(client.clone(), "hash_rate_sma_1w".to_string()),
            hash_rate_sma_1m: MetricPattern1::new(client.clone(), "hash_rate_sma_1m".to_string()),
            hash_rate_sma_2m: MetricPattern1::new(client.clone(), "hash_rate_sma_2m".to_string()),
            hash_rate_sma_1y: MetricPattern1::new(client.clone(), "hash_rate_sma_1y".to_string()),
            hash_rate_ath: MetricPattern1::new(client.clone(), "hash_rate_ath".to_string()),
            hash_rate_drawdown: BpsPercentRatioPattern::new(client.clone(), "hash_rate_drawdown".to_string()),
            hash_price_ths: MetricPattern1::new(client.clone(), "hash_price_ths".to_string()),
            hash_price_ths_min: MetricPattern1::new(client.clone(), "hash_price_ths_min".to_string()),
            hash_price_phs: MetricPattern1::new(client.clone(), "hash_price_phs".to_string()),
            hash_price_phs_min: MetricPattern1::new(client.clone(), "hash_price_phs_min".to_string()),
            hash_price_rebound: BpsPercentRatioPattern::new(client.clone(), "hash_price_rebound".to_string()),
            hash_value_ths: MetricPattern1::new(client.clone(), "hash_value_ths".to_string()),
            hash_value_ths_min: MetricPattern1::new(client.clone(), "hash_value_ths_min".to_string()),
            hash_value_phs: MetricPattern1::new(client.clone(), "hash_value_phs".to_string()),
            hash_value_phs_min: MetricPattern1::new(client.clone(), "hash_value_phs_min".to_string()),
            hash_value_rebound: BpsPercentRatioPattern::new(client.clone(), "hash_value_rebound".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Positions {
    pub block_position: MetricPattern18<BlkPosition>,
    pub tx_position: MetricPattern19<BlkPosition>,
}

impl MetricsTree_Positions {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            block_position: MetricPattern18::new(client.clone(), "position".to_string()),
            tx_position: MetricPattern19::new(client.clone(), "position".to_string()),
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
    pub coinblocks_created: CumulativeHeightSumPattern<StoredF64>,
    pub coinblocks_stored: CumulativeHeightSumPattern<StoredF64>,
    pub liveliness: MetricPattern1<StoredF64>,
    pub vaultedness: MetricPattern1<StoredF64>,
    pub activity_to_vaultedness_ratio: MetricPattern1<StoredF64>,
}

impl MetricsTree_Cointime_Activity {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            coinblocks_created: CumulativeHeightSumPattern::new(client.clone(), "coinblocks_created".to_string()),
            coinblocks_stored: CumulativeHeightSumPattern::new(client.clone(), "coinblocks_stored".to_string()),
            liveliness: MetricPattern1::new(client.clone(), "liveliness".to_string()),
            vaultedness: MetricPattern1::new(client.clone(), "vaultedness".to_string()),
            activity_to_vaultedness_ratio: MetricPattern1::new(client.clone(), "activity_to_vaultedness_ratio".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Cointime_Supply {
    pub vaulted_supply: BtcCentsSatsUsdPattern,
    pub active_supply: BtcCentsSatsUsdPattern,
}

impl MetricsTree_Cointime_Supply {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            vaulted_supply: BtcCentsSatsUsdPattern::new(client.clone(), "vaulted_supply".to_string()),
            active_supply: BtcCentsSatsUsdPattern::new(client.clone(), "active_supply".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Cointime_Value {
    pub cointime_value_destroyed: CumulativeHeightSumPattern<StoredF64>,
    pub cointime_value_created: CumulativeHeightSumPattern<StoredF64>,
    pub cointime_value_stored: CumulativeHeightSumPattern<StoredF64>,
    pub vocdd: CumulativeHeightSumPattern<StoredF64>,
}

impl MetricsTree_Cointime_Value {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            cointime_value_destroyed: CumulativeHeightSumPattern::new(client.clone(), "cointime_value_destroyed".to_string()),
            cointime_value_created: CumulativeHeightSumPattern::new(client.clone(), "cointime_value_created".to_string()),
            cointime_value_stored: CumulativeHeightSumPattern::new(client.clone(), "cointime_value_stored".to_string()),
            vocdd: CumulativeHeightSumPattern::new(client.clone(), "vocdd".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Cointime_Cap {
    pub thermo_cap: CentsUsdPattern,
    pub investor_cap: CentsUsdPattern,
    pub vaulted_cap: CentsUsdPattern,
    pub active_cap: CentsUsdPattern,
    pub cointime_cap: CentsUsdPattern,
}

impl MetricsTree_Cointime_Cap {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            thermo_cap: CentsUsdPattern::new(client.clone(), "thermo_cap".to_string()),
            investor_cap: CentsUsdPattern::new(client.clone(), "investor_cap".to_string()),
            vaulted_cap: CentsUsdPattern::new(client.clone(), "vaulted_cap".to_string()),
            active_cap: CentsUsdPattern::new(client.clone(), "active_cap".to_string()),
            cointime_cap: CentsUsdPattern::new(client.clone(), "cointime_cap".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Cointime_Pricing {
    pub vaulted_price: CentsSatsUsdPattern,
    pub vaulted_price_ratio: BpsRatioPattern2,
    pub active_price: CentsSatsUsdPattern,
    pub active_price_ratio: BpsRatioPattern2,
    pub true_market_mean: CentsSatsUsdPattern,
    pub true_market_mean_ratio: BpsRatioPattern2,
    pub cointime_price: CentsSatsUsdPattern,
    pub cointime_price_ratio: BpsRatioPattern2,
}

impl MetricsTree_Cointime_Pricing {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            vaulted_price: CentsSatsUsdPattern::new(client.clone(), "vaulted_price".to_string()),
            vaulted_price_ratio: BpsRatioPattern2::new(client.clone(), "vaulted_price_ratio".to_string()),
            active_price: CentsSatsUsdPattern::new(client.clone(), "active_price".to_string()),
            active_price_ratio: BpsRatioPattern2::new(client.clone(), "active_price_ratio".to_string()),
            true_market_mean: CentsSatsUsdPattern::new(client.clone(), "true_market_mean".to_string()),
            true_market_mean_ratio: BpsRatioPattern2::new(client.clone(), "true_market_mean_ratio".to_string()),
            cointime_price: CentsSatsUsdPattern::new(client.clone(), "cointime_price".to_string()),
            cointime_price_ratio: BpsRatioPattern2::new(client.clone(), "cointime_price_ratio".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Cointime_Adjusted {
    pub cointime_adj_inflation_rate: BpsPercentRatioPattern,
    pub cointime_adj_tx_velocity_btc: MetricPattern1<StoredF64>,
    pub cointime_adj_tx_velocity_usd: MetricPattern1<StoredF64>,
}

impl MetricsTree_Cointime_Adjusted {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            cointime_adj_inflation_rate: BpsPercentRatioPattern::new(client.clone(), "cointime_adj_inflation_rate".to_string()),
            cointime_adj_tx_velocity_btc: MetricPattern1::new(client.clone(), "cointime_adj_tx_velocity_btc".to_string()),
            cointime_adj_tx_velocity_usd: MetricPattern1::new(client.clone(), "cointime_adj_tx_velocity_usd".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Cointime_ReserveRisk {
    pub vocdd_median_1y: MetricPattern18<StoredF64>,
    pub hodl_bank: MetricPattern18<StoredF64>,
    pub reserve_risk: MetricPattern1<StoredF64>,
}

impl MetricsTree_Cointime_ReserveRisk {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            vocdd_median_1y: MetricPattern18::new(client.clone(), "vocdd_median_1y".to_string()),
            hodl_bank: MetricPattern18::new(client.clone(), "hodl_bank".to_string()),
            reserve_risk: MetricPattern1::new(client.clone(), "reserve_risk".to_string()),
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
    pub minute10: MetricsTree_Indexes_Minute10,
    pub minute30: MetricsTree_Indexes_Minute30,
    pub hour1: MetricsTree_Indexes_Hour1,
    pub hour4: MetricsTree_Indexes_Hour4,
    pub hour12: MetricsTree_Indexes_Hour12,
    pub day1: MetricsTree_Indexes_Day1,
    pub day3: MetricsTree_Indexes_Day3,
    pub week1: MetricsTree_Indexes_Week1,
    pub month1: MetricsTree_Indexes_Month1,
    pub month3: MetricsTree_Indexes_Month3,
    pub month6: MetricsTree_Indexes_Month6,
    pub year1: MetricsTree_Indexes_Year1,
    pub year10: MetricsTree_Indexes_Year10,
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
            minute10: MetricsTree_Indexes_Minute10::new(client.clone(), format!("{base_path}_minute10")),
            minute30: MetricsTree_Indexes_Minute30::new(client.clone(), format!("{base_path}_minute30")),
            hour1: MetricsTree_Indexes_Hour1::new(client.clone(), format!("{base_path}_hour1")),
            hour4: MetricsTree_Indexes_Hour4::new(client.clone(), format!("{base_path}_hour4")),
            hour12: MetricsTree_Indexes_Hour12::new(client.clone(), format!("{base_path}_hour12")),
            day1: MetricsTree_Indexes_Day1::new(client.clone(), format!("{base_path}_day1")),
            day3: MetricsTree_Indexes_Day3::new(client.clone(), format!("{base_path}_day3")),
            week1: MetricsTree_Indexes_Week1::new(client.clone(), format!("{base_path}_week1")),
            month1: MetricsTree_Indexes_Month1::new(client.clone(), format!("{base_path}_month1")),
            month3: MetricsTree_Indexes_Month3::new(client.clone(), format!("{base_path}_month3")),
            month6: MetricsTree_Indexes_Month6::new(client.clone(), format!("{base_path}_month6")),
            year1: MetricsTree_Indexes_Year1::new(client.clone(), format!("{base_path}_year1")),
            year10: MetricsTree_Indexes_Year10::new(client.clone(), format!("{base_path}_year10")),
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
    pub identity: MetricPattern26<P2PK33AddressIndex>,
}

impl MetricsTree_Indexes_Address_P2pk33 {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern26::new(client.clone(), "p2pk33addressindex".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Address_P2pk65 {
    pub identity: MetricPattern27<P2PK65AddressIndex>,
}

impl MetricsTree_Indexes_Address_P2pk65 {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern27::new(client.clone(), "p2pk65addressindex".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Address_P2pkh {
    pub identity: MetricPattern28<P2PKHAddressIndex>,
}

impl MetricsTree_Indexes_Address_P2pkh {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern28::new(client.clone(), "p2pkhaddressindex".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Address_P2sh {
    pub identity: MetricPattern29<P2SHAddressIndex>,
}

impl MetricsTree_Indexes_Address_P2sh {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern29::new(client.clone(), "p2shaddressindex".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Address_P2tr {
    pub identity: MetricPattern30<P2TRAddressIndex>,
}

impl MetricsTree_Indexes_Address_P2tr {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern30::new(client.clone(), "p2traddressindex".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Address_P2wpkh {
    pub identity: MetricPattern31<P2WPKHAddressIndex>,
}

impl MetricsTree_Indexes_Address_P2wpkh {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern31::new(client.clone(), "p2wpkhaddressindex".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Address_P2wsh {
    pub identity: MetricPattern32<P2WSHAddressIndex>,
}

impl MetricsTree_Indexes_Address_P2wsh {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern32::new(client.clone(), "p2wshaddressindex".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Address_P2a {
    pub identity: MetricPattern24<P2AAddressIndex>,
}

impl MetricsTree_Indexes_Address_P2a {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern24::new(client.clone(), "p2aaddressindex".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Address_P2ms {
    pub identity: MetricPattern25<P2MSOutputIndex>,
}

impl MetricsTree_Indexes_Address_P2ms {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern25::new(client.clone(), "p2msoutputindex".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Address_Empty {
    pub identity: MetricPattern22<EmptyOutputIndex>,
}

impl MetricsTree_Indexes_Address_Empty {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern22::new(client.clone(), "emptyoutputindex".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Address_Unknown {
    pub identity: MetricPattern33<UnknownOutputIndex>,
}

impl MetricsTree_Indexes_Address_Unknown {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern33::new(client.clone(), "unknownoutputindex".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Address_Opreturn {
    pub identity: MetricPattern23<OpReturnIndex>,
}

impl MetricsTree_Indexes_Address_Opreturn {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern23::new(client.clone(), "opreturnindex".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Height {
    pub identity: MetricPattern18<Height>,
    pub minute10: MetricPattern18<Minute10>,
    pub minute30: MetricPattern18<Minute30>,
    pub hour1: MetricPattern18<Hour1>,
    pub hour4: MetricPattern18<Hour4>,
    pub hour12: MetricPattern18<Hour12>,
    pub day1: MetricPattern18<Day1>,
    pub day3: MetricPattern18<Day3>,
    pub difficultyepoch: MetricPattern18<DifficultyEpoch>,
    pub halvingepoch: MetricPattern18<HalvingEpoch>,
    pub week1: MetricPattern18<Week1>,
    pub month1: MetricPattern18<Month1>,
    pub month3: MetricPattern18<Month3>,
    pub month6: MetricPattern18<Month6>,
    pub year1: MetricPattern18<Year1>,
    pub year10: MetricPattern18<Year10>,
    pub txindex_count: MetricPattern18<StoredU64>,
}

impl MetricsTree_Indexes_Height {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern18::new(client.clone(), "height".to_string()),
            minute10: MetricPattern18::new(client.clone(), "minute10".to_string()),
            minute30: MetricPattern18::new(client.clone(), "minute30".to_string()),
            hour1: MetricPattern18::new(client.clone(), "hour1".to_string()),
            hour4: MetricPattern18::new(client.clone(), "hour4".to_string()),
            hour12: MetricPattern18::new(client.clone(), "hour12".to_string()),
            day1: MetricPattern18::new(client.clone(), "day1".to_string()),
            day3: MetricPattern18::new(client.clone(), "day3".to_string()),
            difficultyepoch: MetricPattern18::new(client.clone(), "difficultyepoch".to_string()),
            halvingepoch: MetricPattern18::new(client.clone(), "halvingepoch".to_string()),
            week1: MetricPattern18::new(client.clone(), "week1".to_string()),
            month1: MetricPattern18::new(client.clone(), "month1".to_string()),
            month3: MetricPattern18::new(client.clone(), "month3".to_string()),
            month6: MetricPattern18::new(client.clone(), "month6".to_string()),
            year1: MetricPattern18::new(client.clone(), "year1".to_string()),
            year10: MetricPattern18::new(client.clone(), "year10".to_string()),
            txindex_count: MetricPattern18::new(client.clone(), "txindex_count".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Difficultyepoch {
    pub identity: MetricPattern17<DifficultyEpoch>,
    pub first_height: MetricPattern17<Height>,
    pub height_count: MetricPattern17<StoredU64>,
}

impl MetricsTree_Indexes_Difficultyepoch {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern17::new(client.clone(), "difficultyepoch".to_string()),
            first_height: MetricPattern17::new(client.clone(), "first_height".to_string()),
            height_count: MetricPattern17::new(client.clone(), "height_count".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Halvingepoch {
    pub identity: MetricPattern16<HalvingEpoch>,
    pub first_height: MetricPattern16<Height>,
}

impl MetricsTree_Indexes_Halvingepoch {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern16::new(client.clone(), "halvingepoch".to_string()),
            first_height: MetricPattern16::new(client.clone(), "first_height".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Minute10 {
    pub identity: MetricPattern3<Minute10>,
    pub first_height: MetricPattern3<Height>,
}

impl MetricsTree_Indexes_Minute10 {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern3::new(client.clone(), "minute10".to_string()),
            first_height: MetricPattern3::new(client.clone(), "minute10_first_height".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Minute30 {
    pub identity: MetricPattern4<Minute30>,
    pub first_height: MetricPattern4<Height>,
}

impl MetricsTree_Indexes_Minute30 {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern4::new(client.clone(), "minute30".to_string()),
            first_height: MetricPattern4::new(client.clone(), "minute30_first_height".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Hour1 {
    pub identity: MetricPattern5<Hour1>,
    pub first_height: MetricPattern5<Height>,
}

impl MetricsTree_Indexes_Hour1 {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern5::new(client.clone(), "hour1".to_string()),
            first_height: MetricPattern5::new(client.clone(), "hour1_first_height".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Hour4 {
    pub identity: MetricPattern6<Hour4>,
    pub first_height: MetricPattern6<Height>,
}

impl MetricsTree_Indexes_Hour4 {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern6::new(client.clone(), "hour4".to_string()),
            first_height: MetricPattern6::new(client.clone(), "hour4_first_height".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Hour12 {
    pub identity: MetricPattern7<Hour12>,
    pub first_height: MetricPattern7<Height>,
}

impl MetricsTree_Indexes_Hour12 {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern7::new(client.clone(), "hour12".to_string()),
            first_height: MetricPattern7::new(client.clone(), "hour12_first_height".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Day1 {
    pub identity: MetricPattern8<Day1>,
    pub date: MetricPattern8<Date>,
    pub first_height: MetricPattern8<Height>,
    pub height_count: MetricPattern8<StoredU64>,
}

impl MetricsTree_Indexes_Day1 {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern8::new(client.clone(), "day1".to_string()),
            date: MetricPattern8::new(client.clone(), "date".to_string()),
            first_height: MetricPattern8::new(client.clone(), "first_height".to_string()),
            height_count: MetricPattern8::new(client.clone(), "height_count".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Day3 {
    pub identity: MetricPattern9<Day3>,
    pub first_height: MetricPattern9<Height>,
}

impl MetricsTree_Indexes_Day3 {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern9::new(client.clone(), "day3".to_string()),
            first_height: MetricPattern9::new(client.clone(), "day3_first_height".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Week1 {
    pub identity: MetricPattern10<Week1>,
    pub date: MetricPattern10<Date>,
    pub first_height: MetricPattern10<Height>,
}

impl MetricsTree_Indexes_Week1 {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern10::new(client.clone(), "week1".to_string()),
            date: MetricPattern10::new(client.clone(), "date".to_string()),
            first_height: MetricPattern10::new(client.clone(), "week1_first_height".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Month1 {
    pub identity: MetricPattern11<Month1>,
    pub date: MetricPattern11<Date>,
    pub first_height: MetricPattern11<Height>,
}

impl MetricsTree_Indexes_Month1 {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern11::new(client.clone(), "month1".to_string()),
            date: MetricPattern11::new(client.clone(), "date".to_string()),
            first_height: MetricPattern11::new(client.clone(), "month1_first_height".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Month3 {
    pub identity: MetricPattern12<Month3>,
    pub date: MetricPattern12<Date>,
    pub first_height: MetricPattern12<Height>,
}

impl MetricsTree_Indexes_Month3 {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern12::new(client.clone(), "month3".to_string()),
            date: MetricPattern12::new(client.clone(), "date".to_string()),
            first_height: MetricPattern12::new(client.clone(), "month3_first_height".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Month6 {
    pub identity: MetricPattern13<Month6>,
    pub date: MetricPattern13<Date>,
    pub first_height: MetricPattern13<Height>,
}

impl MetricsTree_Indexes_Month6 {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern13::new(client.clone(), "month6".to_string()),
            date: MetricPattern13::new(client.clone(), "date".to_string()),
            first_height: MetricPattern13::new(client.clone(), "month6_first_height".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Year1 {
    pub identity: MetricPattern14<Year1>,
    pub date: MetricPattern14<Date>,
    pub first_height: MetricPattern14<Height>,
}

impl MetricsTree_Indexes_Year1 {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern14::new(client.clone(), "year1".to_string()),
            date: MetricPattern14::new(client.clone(), "date".to_string()),
            first_height: MetricPattern14::new(client.clone(), "year1_first_height".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Year10 {
    pub identity: MetricPattern15<Year10>,
    pub date: MetricPattern15<Date>,
    pub first_height: MetricPattern15<Height>,
}

impl MetricsTree_Indexes_Year10 {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern15::new(client.clone(), "year10".to_string()),
            date: MetricPattern15::new(client.clone(), "date".to_string()),
            first_height: MetricPattern15::new(client.clone(), "year10_first_height".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Txindex {
    pub identity: MetricPattern19<TxIndex>,
    pub input_count: MetricPattern19<StoredU64>,
    pub output_count: MetricPattern19<StoredU64>,
}

impl MetricsTree_Indexes_Txindex {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern19::new(client.clone(), "txindex".to_string()),
            input_count: MetricPattern19::new(client.clone(), "input_count".to_string()),
            output_count: MetricPattern19::new(client.clone(), "output_count".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Txinindex {
    pub identity: MetricPattern20<TxInIndex>,
}

impl MetricsTree_Indexes_Txinindex {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern20::new(client.clone(), "txinindex".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Txoutindex {
    pub identity: MetricPattern21<TxOutIndex>,
}

impl MetricsTree_Indexes_Txoutindex {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern21::new(client.clone(), "txoutindex".to_string()),
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
    pub price_ath: CentsSatsUsdPattern,
    pub price_drawdown: BpsPercentRatioPattern,
    pub days_since_price_ath: MetricPattern1<StoredF32>,
    pub years_since_price_ath: MetricPattern2<StoredF32>,
    pub max_days_between_price_ath: MetricPattern1<StoredF32>,
    pub max_years_between_price_ath: MetricPattern2<StoredF32>,
}

impl MetricsTree_Market_Ath {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            price_ath: CentsSatsUsdPattern::new(client.clone(), "price_ath".to_string()),
            price_drawdown: BpsPercentRatioPattern::new(client.clone(), "price_drawdown".to_string()),
            days_since_price_ath: MetricPattern1::new(client.clone(), "days_since_price_ath".to_string()),
            years_since_price_ath: MetricPattern2::new(client.clone(), "years_since_price_ath".to_string()),
            max_days_between_price_ath: MetricPattern1::new(client.clone(), "max_days_between_price_ath".to_string()),
            max_years_between_price_ath: MetricPattern2::new(client.clone(), "max_years_between_price_ath".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Market_Lookback {
    pub _24h: CentsSatsUsdPattern,
    pub _1w: CentsSatsUsdPattern,
    pub _1m: CentsSatsUsdPattern,
    pub _3m: CentsSatsUsdPattern,
    pub _6m: CentsSatsUsdPattern,
    pub _1y: CentsSatsUsdPattern,
    pub _2y: CentsSatsUsdPattern,
    pub _3y: CentsSatsUsdPattern,
    pub _4y: CentsSatsUsdPattern,
    pub _5y: CentsSatsUsdPattern,
    pub _6y: CentsSatsUsdPattern,
    pub _8y: CentsSatsUsdPattern,
    pub _10y: CentsSatsUsdPattern,
}

impl MetricsTree_Market_Lookback {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _24h: CentsSatsUsdPattern::new(client.clone(), "price_lookback_24h".to_string()),
            _1w: CentsSatsUsdPattern::new(client.clone(), "price_lookback_1w".to_string()),
            _1m: CentsSatsUsdPattern::new(client.clone(), "price_lookback_1m".to_string()),
            _3m: CentsSatsUsdPattern::new(client.clone(), "price_lookback_3m".to_string()),
            _6m: CentsSatsUsdPattern::new(client.clone(), "price_lookback_6m".to_string()),
            _1y: CentsSatsUsdPattern::new(client.clone(), "price_lookback_1y".to_string()),
            _2y: CentsSatsUsdPattern::new(client.clone(), "price_lookback_2y".to_string()),
            _3y: CentsSatsUsdPattern::new(client.clone(), "price_lookback_3y".to_string()),
            _4y: CentsSatsUsdPattern::new(client.clone(), "price_lookback_4y".to_string()),
            _5y: CentsSatsUsdPattern::new(client.clone(), "price_lookback_5y".to_string()),
            _6y: CentsSatsUsdPattern::new(client.clone(), "price_lookback_6y".to_string()),
            _8y: CentsSatsUsdPattern::new(client.clone(), "price_lookback_8y".to_string()),
            _10y: CentsSatsUsdPattern::new(client.clone(), "price_lookback_10y".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Market_Returns {
    pub price_return: MetricsTree_Market_Returns_PriceReturn,
    pub price_cagr: _10y2y3y4y5y6y8yPattern,
    pub price_return_24h_sd_1w: MetricsTree_Market_Returns_PriceReturn24hSd1w,
    pub price_return_24h_sd_1m: MetricsTree_Market_Returns_PriceReturn24hSd1m,
    pub price_return_24h_sd_1y: SdSmaPattern,
}

impl MetricsTree_Market_Returns {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            price_return: MetricsTree_Market_Returns_PriceReturn::new(client.clone(), format!("{base_path}_price_return")),
            price_cagr: _10y2y3y4y5y6y8yPattern::new(client.clone(), "price_cagr".to_string()),
            price_return_24h_sd_1w: MetricsTree_Market_Returns_PriceReturn24hSd1w::new(client.clone(), format!("{base_path}_price_return_24h_sd_1w")),
            price_return_24h_sd_1m: MetricsTree_Market_Returns_PriceReturn24hSd1m::new(client.clone(), format!("{base_path}_price_return_24h_sd_1m")),
            price_return_24h_sd_1y: SdSmaPattern::new(client.clone(), "price_return_24h".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Market_Returns_PriceReturn {
    pub _24h: BpsPercentRatioPattern,
    pub _1w: BpsPercentRatioPattern,
    pub _1m: BpsPercentRatioPattern,
    pub _3m: BpsPercentRatioPattern,
    pub _6m: BpsPercentRatioPattern,
    pub _1y: BpsPercentRatioPattern,
    pub _2y: BpsPercentRatioPattern,
    pub _3y: BpsPercentRatioPattern,
    pub _4y: BpsPercentRatioPattern,
    pub _5y: BpsPercentRatioPattern,
    pub _6y: BpsPercentRatioPattern,
    pub _8y: BpsPercentRatioPattern,
    pub _10y: BpsPercentRatioPattern,
}

impl MetricsTree_Market_Returns_PriceReturn {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _24h: BpsPercentRatioPattern::new(client.clone(), "price_return_24h".to_string()),
            _1w: BpsPercentRatioPattern::new(client.clone(), "price_return_1w".to_string()),
            _1m: BpsPercentRatioPattern::new(client.clone(), "price_return_1m".to_string()),
            _3m: BpsPercentRatioPattern::new(client.clone(), "price_return_3m".to_string()),
            _6m: BpsPercentRatioPattern::new(client.clone(), "price_return_6m".to_string()),
            _1y: BpsPercentRatioPattern::new(client.clone(), "price_return_1y".to_string()),
            _2y: BpsPercentRatioPattern::new(client.clone(), "price_return_2y".to_string()),
            _3y: BpsPercentRatioPattern::new(client.clone(), "price_return_3y".to_string()),
            _4y: BpsPercentRatioPattern::new(client.clone(), "price_return_4y".to_string()),
            _5y: BpsPercentRatioPattern::new(client.clone(), "price_return_5y".to_string()),
            _6y: BpsPercentRatioPattern::new(client.clone(), "price_return_6y".to_string()),
            _8y: BpsPercentRatioPattern::new(client.clone(), "price_return_8y".to_string()),
            _10y: BpsPercentRatioPattern::new(client.clone(), "price_return_10y".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Market_Returns_PriceReturn24hSd1w {
    pub sma: MetricPattern1<StoredF32>,
    pub sd: MetricPattern1<StoredF32>,
}

impl MetricsTree_Market_Returns_PriceReturn24hSd1w {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            sma: MetricPattern1::new(client.clone(), "price_return_24h_sma_1w".to_string()),
            sd: MetricPattern1::new(client.clone(), "price_return_24h_sd_1w".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Market_Returns_PriceReturn24hSd1m {
    pub sma: MetricPattern1<StoredF32>,
    pub sd: MetricPattern1<StoredF32>,
}

impl MetricsTree_Market_Returns_PriceReturn24hSd1m {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            sma: MetricPattern1::new(client.clone(), "price_return_24h_sma_1m".to_string()),
            sd: MetricPattern1::new(client.clone(), "price_return_24h_sd_1m".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Market_Volatility {
    pub price_volatility_1w: MetricPattern1<StoredF32>,
    pub price_volatility_1m: MetricPattern1<StoredF32>,
    pub price_volatility_1y: MetricPattern1<StoredF32>,
}

impl MetricsTree_Market_Volatility {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            price_volatility_1w: MetricPattern1::new(client.clone(), "price_volatility_1w".to_string()),
            price_volatility_1m: MetricPattern1::new(client.clone(), "price_volatility_1m".to_string()),
            price_volatility_1y: MetricPattern1::new(client.clone(), "price_volatility_1y".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Market_Range {
    pub price_min_1w: CentsSatsUsdPattern,
    pub price_max_1w: CentsSatsUsdPattern,
    pub price_min_2w: CentsSatsUsdPattern,
    pub price_max_2w: CentsSatsUsdPattern,
    pub price_min_1m: CentsSatsUsdPattern,
    pub price_max_1m: CentsSatsUsdPattern,
    pub price_min_1y: CentsSatsUsdPattern,
    pub price_max_1y: CentsSatsUsdPattern,
    pub price_true_range: MetricPattern1<StoredF32>,
    pub price_true_range_sum_2w: MetricPattern1<StoredF32>,
    pub price_choppiness_index_2w: BpsPercentRatioPattern,
}

impl MetricsTree_Market_Range {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            price_min_1w: CentsSatsUsdPattern::new(client.clone(), "price_min_1w".to_string()),
            price_max_1w: CentsSatsUsdPattern::new(client.clone(), "price_max_1w".to_string()),
            price_min_2w: CentsSatsUsdPattern::new(client.clone(), "price_min_2w".to_string()),
            price_max_2w: CentsSatsUsdPattern::new(client.clone(), "price_max_2w".to_string()),
            price_min_1m: CentsSatsUsdPattern::new(client.clone(), "price_min_1m".to_string()),
            price_max_1m: CentsSatsUsdPattern::new(client.clone(), "price_max_1m".to_string()),
            price_min_1y: CentsSatsUsdPattern::new(client.clone(), "price_min_1y".to_string()),
            price_max_1y: CentsSatsUsdPattern::new(client.clone(), "price_max_1y".to_string()),
            price_true_range: MetricPattern1::new(client.clone(), "price_true_range".to_string()),
            price_true_range_sum_2w: MetricPattern1::new(client.clone(), "price_true_range_sum_2w".to_string()),
            price_choppiness_index_2w: BpsPercentRatioPattern::new(client.clone(), "price_choppiness_index_2w".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Market_MovingAverage {
    pub price_sma_1w: BpsPriceRatioPattern,
    pub price_sma_8d: BpsPriceRatioPattern,
    pub price_sma_13d: BpsPriceRatioPattern,
    pub price_sma_21d: BpsPriceRatioPattern,
    pub price_sma_1m: BpsPriceRatioPattern,
    pub price_sma_34d: BpsPriceRatioPattern,
    pub price_sma_55d: BpsPriceRatioPattern,
    pub price_sma_89d: BpsPriceRatioPattern,
    pub price_sma_111d: BpsPriceRatioPattern,
    pub price_sma_144d: BpsPriceRatioPattern,
    pub price_sma_200d: BpsPriceRatioPattern,
    pub price_sma_350d: BpsPriceRatioPattern,
    pub price_sma_1y: BpsPriceRatioPattern,
    pub price_sma_2y: BpsPriceRatioPattern,
    pub price_sma_200w: BpsPriceRatioPattern,
    pub price_sma_4y: BpsPriceRatioPattern,
    pub price_ema_1w: BpsPriceRatioPattern,
    pub price_ema_8d: BpsPriceRatioPattern,
    pub price_ema_12d: BpsPriceRatioPattern,
    pub price_ema_13d: BpsPriceRatioPattern,
    pub price_ema_21d: BpsPriceRatioPattern,
    pub price_ema_26d: BpsPriceRatioPattern,
    pub price_ema_1m: BpsPriceRatioPattern,
    pub price_ema_34d: BpsPriceRatioPattern,
    pub price_ema_55d: BpsPriceRatioPattern,
    pub price_ema_89d: BpsPriceRatioPattern,
    pub price_ema_144d: BpsPriceRatioPattern,
    pub price_ema_200d: BpsPriceRatioPattern,
    pub price_ema_1y: BpsPriceRatioPattern,
    pub price_ema_2y: BpsPriceRatioPattern,
    pub price_ema_200w: BpsPriceRatioPattern,
    pub price_ema_4y: BpsPriceRatioPattern,
    pub price_sma_200d_x2_4: CentsSatsUsdPattern,
    pub price_sma_200d_x0_8: CentsSatsUsdPattern,
    pub price_sma_350d_x2: CentsSatsUsdPattern,
}

impl MetricsTree_Market_MovingAverage {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            price_sma_1w: BpsPriceRatioPattern::new(client.clone(), "price_sma_1w".to_string()),
            price_sma_8d: BpsPriceRatioPattern::new(client.clone(), "price_sma_8d".to_string()),
            price_sma_13d: BpsPriceRatioPattern::new(client.clone(), "price_sma_13d".to_string()),
            price_sma_21d: BpsPriceRatioPattern::new(client.clone(), "price_sma_21d".to_string()),
            price_sma_1m: BpsPriceRatioPattern::new(client.clone(), "price_sma_1m".to_string()),
            price_sma_34d: BpsPriceRatioPattern::new(client.clone(), "price_sma_34d".to_string()),
            price_sma_55d: BpsPriceRatioPattern::new(client.clone(), "price_sma_55d".to_string()),
            price_sma_89d: BpsPriceRatioPattern::new(client.clone(), "price_sma_89d".to_string()),
            price_sma_111d: BpsPriceRatioPattern::new(client.clone(), "price_sma_111d".to_string()),
            price_sma_144d: BpsPriceRatioPattern::new(client.clone(), "price_sma_144d".to_string()),
            price_sma_200d: BpsPriceRatioPattern::new(client.clone(), "price_sma_200d".to_string()),
            price_sma_350d: BpsPriceRatioPattern::new(client.clone(), "price_sma_350d".to_string()),
            price_sma_1y: BpsPriceRatioPattern::new(client.clone(), "price_sma_1y".to_string()),
            price_sma_2y: BpsPriceRatioPattern::new(client.clone(), "price_sma_2y".to_string()),
            price_sma_200w: BpsPriceRatioPattern::new(client.clone(), "price_sma_200w".to_string()),
            price_sma_4y: BpsPriceRatioPattern::new(client.clone(), "price_sma_4y".to_string()),
            price_ema_1w: BpsPriceRatioPattern::new(client.clone(), "price_ema_1w".to_string()),
            price_ema_8d: BpsPriceRatioPattern::new(client.clone(), "price_ema_8d".to_string()),
            price_ema_12d: BpsPriceRatioPattern::new(client.clone(), "price_ema_12d".to_string()),
            price_ema_13d: BpsPriceRatioPattern::new(client.clone(), "price_ema_13d".to_string()),
            price_ema_21d: BpsPriceRatioPattern::new(client.clone(), "price_ema_21d".to_string()),
            price_ema_26d: BpsPriceRatioPattern::new(client.clone(), "price_ema_26d".to_string()),
            price_ema_1m: BpsPriceRatioPattern::new(client.clone(), "price_ema_1m".to_string()),
            price_ema_34d: BpsPriceRatioPattern::new(client.clone(), "price_ema_34d".to_string()),
            price_ema_55d: BpsPriceRatioPattern::new(client.clone(), "price_ema_55d".to_string()),
            price_ema_89d: BpsPriceRatioPattern::new(client.clone(), "price_ema_89d".to_string()),
            price_ema_144d: BpsPriceRatioPattern::new(client.clone(), "price_ema_144d".to_string()),
            price_ema_200d: BpsPriceRatioPattern::new(client.clone(), "price_ema_200d".to_string()),
            price_ema_1y: BpsPriceRatioPattern::new(client.clone(), "price_ema_1y".to_string()),
            price_ema_2y: BpsPriceRatioPattern::new(client.clone(), "price_ema_2y".to_string()),
            price_ema_200w: BpsPriceRatioPattern::new(client.clone(), "price_ema_200w".to_string()),
            price_ema_4y: BpsPriceRatioPattern::new(client.clone(), "price_ema_4y".to_string()),
            price_sma_200d_x2_4: CentsSatsUsdPattern::new(client.clone(), "price_sma_200d_x2_4".to_string()),
            price_sma_200d_x0_8: CentsSatsUsdPattern::new(client.clone(), "price_sma_200d_x0_8".to_string()),
            price_sma_350d_x2: CentsSatsUsdPattern::new(client.clone(), "price_sma_350d_x2".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Market_Dca {
    pub dca_sats_per_day: MetricPattern18<Sats>,
    pub period_stack: _10y1m1w1y2y3m3y4y5y6m6y8yPattern3,
    pub period_cost_basis: MetricsTree_Market_Dca_PeriodCostBasis,
    pub period_return: _10y1m1w1y2y3m3y4y5y6m6y8yPattern2,
    pub period_cagr: _10y2y3y4y5y6y8yPattern,
    pub period_lump_sum_stack: _10y1m1w1y2y3m3y4y5y6m6y8yPattern3,
    pub period_lump_sum_return: _10y1m1w1y2y3m3y4y5y6m6y8yPattern2,
    pub class_stack: MetricsTree_Market_Dca_ClassStack,
    pub class_cost_basis: MetricsTree_Market_Dca_ClassCostBasis,
    pub class_return: MetricsTree_Market_Dca_ClassReturn,
}

impl MetricsTree_Market_Dca {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            dca_sats_per_day: MetricPattern18::new(client.clone(), "dca_sats_per_day".to_string()),
            period_stack: _10y1m1w1y2y3m3y4y5y6m6y8yPattern3::new(client.clone(), "dca_stack".to_string()),
            period_cost_basis: MetricsTree_Market_Dca_PeriodCostBasis::new(client.clone(), format!("{base_path}_period_cost_basis")),
            period_return: _10y1m1w1y2y3m3y4y5y6m6y8yPattern2::new(client.clone(), "dca_return".to_string()),
            period_cagr: _10y2y3y4y5y6y8yPattern::new(client.clone(), "dca_cagr".to_string()),
            period_lump_sum_stack: _10y1m1w1y2y3m3y4y5y6m6y8yPattern3::new(client.clone(), "lump_sum_stack".to_string()),
            period_lump_sum_return: _10y1m1w1y2y3m3y4y5y6m6y8yPattern2::new(client.clone(), "lump_sum_return".to_string()),
            class_stack: MetricsTree_Market_Dca_ClassStack::new(client.clone(), format!("{base_path}_class_stack")),
            class_cost_basis: MetricsTree_Market_Dca_ClassCostBasis::new(client.clone(), format!("{base_path}_class_cost_basis")),
            class_return: MetricsTree_Market_Dca_ClassReturn::new(client.clone(), format!("{base_path}_class_return")),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Market_Dca_PeriodCostBasis {
    pub _1w: CentsSatsUsdPattern,
    pub _1m: CentsSatsUsdPattern,
    pub _3m: CentsSatsUsdPattern,
    pub _6m: CentsSatsUsdPattern,
    pub _1y: CentsSatsUsdPattern,
    pub _2y: CentsSatsUsdPattern,
    pub _3y: CentsSatsUsdPattern,
    pub _4y: CentsSatsUsdPattern,
    pub _5y: CentsSatsUsdPattern,
    pub _6y: CentsSatsUsdPattern,
    pub _8y: CentsSatsUsdPattern,
    pub _10y: CentsSatsUsdPattern,
}

impl MetricsTree_Market_Dca_PeriodCostBasis {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _1w: CentsSatsUsdPattern::new(client.clone(), "dca_cost_basis_1w".to_string()),
            _1m: CentsSatsUsdPattern::new(client.clone(), "dca_cost_basis_1m".to_string()),
            _3m: CentsSatsUsdPattern::new(client.clone(), "dca_cost_basis_3m".to_string()),
            _6m: CentsSatsUsdPattern::new(client.clone(), "dca_cost_basis_6m".to_string()),
            _1y: CentsSatsUsdPattern::new(client.clone(), "dca_cost_basis_1y".to_string()),
            _2y: CentsSatsUsdPattern::new(client.clone(), "dca_cost_basis_2y".to_string()),
            _3y: CentsSatsUsdPattern::new(client.clone(), "dca_cost_basis_3y".to_string()),
            _4y: CentsSatsUsdPattern::new(client.clone(), "dca_cost_basis_4y".to_string()),
            _5y: CentsSatsUsdPattern::new(client.clone(), "dca_cost_basis_5y".to_string()),
            _6y: CentsSatsUsdPattern::new(client.clone(), "dca_cost_basis_6y".to_string()),
            _8y: CentsSatsUsdPattern::new(client.clone(), "dca_cost_basis_8y".to_string()),
            _10y: CentsSatsUsdPattern::new(client.clone(), "dca_cost_basis_10y".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Market_Dca_ClassStack {
    pub from_2015: BtcCentsSatsUsdPattern,
    pub from_2016: BtcCentsSatsUsdPattern,
    pub from_2017: BtcCentsSatsUsdPattern,
    pub from_2018: BtcCentsSatsUsdPattern,
    pub from_2019: BtcCentsSatsUsdPattern,
    pub from_2020: BtcCentsSatsUsdPattern,
    pub from_2021: BtcCentsSatsUsdPattern,
    pub from_2022: BtcCentsSatsUsdPattern,
    pub from_2023: BtcCentsSatsUsdPattern,
    pub from_2024: BtcCentsSatsUsdPattern,
    pub from_2025: BtcCentsSatsUsdPattern,
    pub from_2026: BtcCentsSatsUsdPattern,
}

impl MetricsTree_Market_Dca_ClassStack {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            from_2015: BtcCentsSatsUsdPattern::new(client.clone(), "dca_stack_from_2015".to_string()),
            from_2016: BtcCentsSatsUsdPattern::new(client.clone(), "dca_stack_from_2016".to_string()),
            from_2017: BtcCentsSatsUsdPattern::new(client.clone(), "dca_stack_from_2017".to_string()),
            from_2018: BtcCentsSatsUsdPattern::new(client.clone(), "dca_stack_from_2018".to_string()),
            from_2019: BtcCentsSatsUsdPattern::new(client.clone(), "dca_stack_from_2019".to_string()),
            from_2020: BtcCentsSatsUsdPattern::new(client.clone(), "dca_stack_from_2020".to_string()),
            from_2021: BtcCentsSatsUsdPattern::new(client.clone(), "dca_stack_from_2021".to_string()),
            from_2022: BtcCentsSatsUsdPattern::new(client.clone(), "dca_stack_from_2022".to_string()),
            from_2023: BtcCentsSatsUsdPattern::new(client.clone(), "dca_stack_from_2023".to_string()),
            from_2024: BtcCentsSatsUsdPattern::new(client.clone(), "dca_stack_from_2024".to_string()),
            from_2025: BtcCentsSatsUsdPattern::new(client.clone(), "dca_stack_from_2025".to_string()),
            from_2026: BtcCentsSatsUsdPattern::new(client.clone(), "dca_stack_from_2026".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Market_Dca_ClassCostBasis {
    pub from_2015: CentsSatsUsdPattern,
    pub from_2016: CentsSatsUsdPattern,
    pub from_2017: CentsSatsUsdPattern,
    pub from_2018: CentsSatsUsdPattern,
    pub from_2019: CentsSatsUsdPattern,
    pub from_2020: CentsSatsUsdPattern,
    pub from_2021: CentsSatsUsdPattern,
    pub from_2022: CentsSatsUsdPattern,
    pub from_2023: CentsSatsUsdPattern,
    pub from_2024: CentsSatsUsdPattern,
    pub from_2025: CentsSatsUsdPattern,
    pub from_2026: CentsSatsUsdPattern,
}

impl MetricsTree_Market_Dca_ClassCostBasis {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            from_2015: CentsSatsUsdPattern::new(client.clone(), "dca_cost_basis_from_2015".to_string()),
            from_2016: CentsSatsUsdPattern::new(client.clone(), "dca_cost_basis_from_2016".to_string()),
            from_2017: CentsSatsUsdPattern::new(client.clone(), "dca_cost_basis_from_2017".to_string()),
            from_2018: CentsSatsUsdPattern::new(client.clone(), "dca_cost_basis_from_2018".to_string()),
            from_2019: CentsSatsUsdPattern::new(client.clone(), "dca_cost_basis_from_2019".to_string()),
            from_2020: CentsSatsUsdPattern::new(client.clone(), "dca_cost_basis_from_2020".to_string()),
            from_2021: CentsSatsUsdPattern::new(client.clone(), "dca_cost_basis_from_2021".to_string()),
            from_2022: CentsSatsUsdPattern::new(client.clone(), "dca_cost_basis_from_2022".to_string()),
            from_2023: CentsSatsUsdPattern::new(client.clone(), "dca_cost_basis_from_2023".to_string()),
            from_2024: CentsSatsUsdPattern::new(client.clone(), "dca_cost_basis_from_2024".to_string()),
            from_2025: CentsSatsUsdPattern::new(client.clone(), "dca_cost_basis_from_2025".to_string()),
            from_2026: CentsSatsUsdPattern::new(client.clone(), "dca_cost_basis_from_2026".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Market_Dca_ClassReturn {
    pub from_2015: BpsPercentRatioPattern,
    pub from_2016: BpsPercentRatioPattern,
    pub from_2017: BpsPercentRatioPattern,
    pub from_2018: BpsPercentRatioPattern,
    pub from_2019: BpsPercentRatioPattern,
    pub from_2020: BpsPercentRatioPattern,
    pub from_2021: BpsPercentRatioPattern,
    pub from_2022: BpsPercentRatioPattern,
    pub from_2023: BpsPercentRatioPattern,
    pub from_2024: BpsPercentRatioPattern,
    pub from_2025: BpsPercentRatioPattern,
    pub from_2026: BpsPercentRatioPattern,
}

impl MetricsTree_Market_Dca_ClassReturn {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            from_2015: BpsPercentRatioPattern::new(client.clone(), "dca_return_from_2015".to_string()),
            from_2016: BpsPercentRatioPattern::new(client.clone(), "dca_return_from_2016".to_string()),
            from_2017: BpsPercentRatioPattern::new(client.clone(), "dca_return_from_2017".to_string()),
            from_2018: BpsPercentRatioPattern::new(client.clone(), "dca_return_from_2018".to_string()),
            from_2019: BpsPercentRatioPattern::new(client.clone(), "dca_return_from_2019".to_string()),
            from_2020: BpsPercentRatioPattern::new(client.clone(), "dca_return_from_2020".to_string()),
            from_2021: BpsPercentRatioPattern::new(client.clone(), "dca_return_from_2021".to_string()),
            from_2022: BpsPercentRatioPattern::new(client.clone(), "dca_return_from_2022".to_string()),
            from_2023: BpsPercentRatioPattern::new(client.clone(), "dca_return_from_2023".to_string()),
            from_2024: BpsPercentRatioPattern::new(client.clone(), "dca_return_from_2024".to_string()),
            from_2025: BpsPercentRatioPattern::new(client.clone(), "dca_return_from_2025".to_string()),
            from_2026: BpsPercentRatioPattern::new(client.clone(), "dca_return_from_2026".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Market_Indicators {
    pub puell_multiple: BpsRatioPattern,
    pub nvt: BpsRatioPattern,
    pub rsi: MetricsTree_Market_Indicators_Rsi,
    pub stoch_k: BpsPercentRatioPattern,
    pub stoch_d: BpsPercentRatioPattern,
    pub pi_cycle: BpsRatioPattern,
    pub macd: MetricsTree_Market_Indicators_Macd,
    pub gini: BpsPercentRatioPattern,
}

impl MetricsTree_Market_Indicators {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            puell_multiple: BpsRatioPattern::new(client.clone(), "puell_multiple".to_string()),
            nvt: BpsRatioPattern::new(client.clone(), "nvt".to_string()),
            rsi: MetricsTree_Market_Indicators_Rsi::new(client.clone(), format!("{base_path}_rsi")),
            stoch_k: BpsPercentRatioPattern::new(client.clone(), "stoch_k".to_string()),
            stoch_d: BpsPercentRatioPattern::new(client.clone(), "stoch_d".to_string()),
            pi_cycle: BpsRatioPattern::new(client.clone(), "pi_cycle".to_string()),
            macd: MetricsTree_Market_Indicators_Macd::new(client.clone(), format!("{base_path}_macd")),
            gini: BpsPercentRatioPattern::new(client.clone(), "gini".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Market_Indicators_Rsi {
    pub _24h: AverageGainsLossesRsiStochPattern,
    pub _1w: MetricsTree_Market_Indicators_Rsi_1w,
    pub _1m: MetricsTree_Market_Indicators_Rsi_1m,
    pub _1y: MetricsTree_Market_Indicators_Rsi_1y,
}

impl MetricsTree_Market_Indicators_Rsi {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _24h: AverageGainsLossesRsiStochPattern::new(client.clone(), "rsi".to_string()),
            _1w: MetricsTree_Market_Indicators_Rsi_1w::new(client.clone(), format!("{base_path}_1w")),
            _1m: MetricsTree_Market_Indicators_Rsi_1m::new(client.clone(), format!("{base_path}_1m")),
            _1y: MetricsTree_Market_Indicators_Rsi_1y::new(client.clone(), format!("{base_path}_1y")),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Market_Indicators_Rsi_1w {
    pub gains: MetricPattern1<StoredF32>,
    pub losses: MetricPattern1<StoredF32>,
    pub average_gain: MetricPattern1<StoredF32>,
    pub average_loss: MetricPattern1<StoredF32>,
    pub rsi: BpsPercentRatioPattern,
    pub rsi_min: BpsPercentRatioPattern,
    pub rsi_max: BpsPercentRatioPattern,
    pub stoch_rsi: BpsPercentRatioPattern,
    pub stoch_rsi_k: BpsPercentRatioPattern,
    pub stoch_rsi_d: BpsPercentRatioPattern,
}

impl MetricsTree_Market_Indicators_Rsi_1w {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            gains: MetricPattern1::new(client.clone(), "rsi_gains_1w".to_string()),
            losses: MetricPattern1::new(client.clone(), "rsi_losses_1w".to_string()),
            average_gain: MetricPattern1::new(client.clone(), "rsi_average_gain_1w".to_string()),
            average_loss: MetricPattern1::new(client.clone(), "rsi_average_loss_1w".to_string()),
            rsi: BpsPercentRatioPattern::new(client.clone(), "rsi_1w".to_string()),
            rsi_min: BpsPercentRatioPattern::new(client.clone(), "rsi_min_1w".to_string()),
            rsi_max: BpsPercentRatioPattern::new(client.clone(), "rsi_max_1w".to_string()),
            stoch_rsi: BpsPercentRatioPattern::new(client.clone(), "rsi_stoch_1w".to_string()),
            stoch_rsi_k: BpsPercentRatioPattern::new(client.clone(), "rsi_stoch_k_1w".to_string()),
            stoch_rsi_d: BpsPercentRatioPattern::new(client.clone(), "rsi_stoch_d_1w".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Market_Indicators_Rsi_1m {
    pub gains: MetricPattern1<StoredF32>,
    pub losses: MetricPattern1<StoredF32>,
    pub average_gain: MetricPattern1<StoredF32>,
    pub average_loss: MetricPattern1<StoredF32>,
    pub rsi: BpsPercentRatioPattern,
    pub rsi_min: BpsPercentRatioPattern,
    pub rsi_max: BpsPercentRatioPattern,
    pub stoch_rsi: BpsPercentRatioPattern,
    pub stoch_rsi_k: BpsPercentRatioPattern,
    pub stoch_rsi_d: BpsPercentRatioPattern,
}

impl MetricsTree_Market_Indicators_Rsi_1m {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            gains: MetricPattern1::new(client.clone(), "rsi_gains_1m".to_string()),
            losses: MetricPattern1::new(client.clone(), "rsi_losses_1m".to_string()),
            average_gain: MetricPattern1::new(client.clone(), "rsi_average_gain_1m".to_string()),
            average_loss: MetricPattern1::new(client.clone(), "rsi_average_loss_1m".to_string()),
            rsi: BpsPercentRatioPattern::new(client.clone(), "rsi_1m".to_string()),
            rsi_min: BpsPercentRatioPattern::new(client.clone(), "rsi_min_1m".to_string()),
            rsi_max: BpsPercentRatioPattern::new(client.clone(), "rsi_max_1m".to_string()),
            stoch_rsi: BpsPercentRatioPattern::new(client.clone(), "rsi_stoch_1m".to_string()),
            stoch_rsi_k: BpsPercentRatioPattern::new(client.clone(), "rsi_stoch_k_1m".to_string()),
            stoch_rsi_d: BpsPercentRatioPattern::new(client.clone(), "rsi_stoch_d_1m".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Market_Indicators_Rsi_1y {
    pub gains: MetricPattern1<StoredF32>,
    pub losses: MetricPattern1<StoredF32>,
    pub average_gain: MetricPattern1<StoredF32>,
    pub average_loss: MetricPattern1<StoredF32>,
    pub rsi: BpsPercentRatioPattern,
    pub rsi_min: BpsPercentRatioPattern,
    pub rsi_max: BpsPercentRatioPattern,
    pub stoch_rsi: BpsPercentRatioPattern,
    pub stoch_rsi_k: BpsPercentRatioPattern,
    pub stoch_rsi_d: BpsPercentRatioPattern,
}

impl MetricsTree_Market_Indicators_Rsi_1y {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            gains: MetricPattern1::new(client.clone(), "rsi_gains_1y".to_string()),
            losses: MetricPattern1::new(client.clone(), "rsi_losses_1y".to_string()),
            average_gain: MetricPattern1::new(client.clone(), "rsi_average_gain_1y".to_string()),
            average_loss: MetricPattern1::new(client.clone(), "rsi_average_loss_1y".to_string()),
            rsi: BpsPercentRatioPattern::new(client.clone(), "rsi_1y".to_string()),
            rsi_min: BpsPercentRatioPattern::new(client.clone(), "rsi_min_1y".to_string()),
            rsi_max: BpsPercentRatioPattern::new(client.clone(), "rsi_max_1y".to_string()),
            stoch_rsi: BpsPercentRatioPattern::new(client.clone(), "rsi_stoch_1y".to_string()),
            stoch_rsi_k: BpsPercentRatioPattern::new(client.clone(), "rsi_stoch_k_1y".to_string()),
            stoch_rsi_d: BpsPercentRatioPattern::new(client.clone(), "rsi_stoch_d_1y".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Market_Indicators_Macd {
    pub _24h: EmaHistogramLineSignalPattern,
    pub _1w: MetricsTree_Market_Indicators_Macd_1w,
    pub _1m: MetricsTree_Market_Indicators_Macd_1m,
    pub _1y: MetricsTree_Market_Indicators_Macd_1y,
}

impl MetricsTree_Market_Indicators_Macd {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _24h: EmaHistogramLineSignalPattern::new(client.clone(), "macd".to_string()),
            _1w: MetricsTree_Market_Indicators_Macd_1w::new(client.clone(), format!("{base_path}_1w")),
            _1m: MetricsTree_Market_Indicators_Macd_1m::new(client.clone(), format!("{base_path}_1m")),
            _1y: MetricsTree_Market_Indicators_Macd_1y::new(client.clone(), format!("{base_path}_1y")),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Market_Indicators_Macd_1w {
    pub ema_fast: MetricPattern1<StoredF32>,
    pub ema_slow: MetricPattern1<StoredF32>,
    pub line: MetricPattern1<StoredF32>,
    pub signal: MetricPattern1<StoredF32>,
    pub histogram: MetricPattern1<StoredF32>,
}

impl MetricsTree_Market_Indicators_Macd_1w {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            ema_fast: MetricPattern1::new(client.clone(), "macd_ema_fast_1w".to_string()),
            ema_slow: MetricPattern1::new(client.clone(), "macd_ema_slow_1w".to_string()),
            line: MetricPattern1::new(client.clone(), "macd_line_1w".to_string()),
            signal: MetricPattern1::new(client.clone(), "macd_signal_1w".to_string()),
            histogram: MetricPattern1::new(client.clone(), "macd_histogram_1w".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Market_Indicators_Macd_1m {
    pub ema_fast: MetricPattern1<StoredF32>,
    pub ema_slow: MetricPattern1<StoredF32>,
    pub line: MetricPattern1<StoredF32>,
    pub signal: MetricPattern1<StoredF32>,
    pub histogram: MetricPattern1<StoredF32>,
}

impl MetricsTree_Market_Indicators_Macd_1m {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            ema_fast: MetricPattern1::new(client.clone(), "macd_ema_fast_1m".to_string()),
            ema_slow: MetricPattern1::new(client.clone(), "macd_ema_slow_1m".to_string()),
            line: MetricPattern1::new(client.clone(), "macd_line_1m".to_string()),
            signal: MetricPattern1::new(client.clone(), "macd_signal_1m".to_string()),
            histogram: MetricPattern1::new(client.clone(), "macd_histogram_1m".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Market_Indicators_Macd_1y {
    pub ema_fast: MetricPattern1<StoredF32>,
    pub ema_slow: MetricPattern1<StoredF32>,
    pub line: MetricPattern1<StoredF32>,
    pub signal: MetricPattern1<StoredF32>,
    pub histogram: MetricPattern1<StoredF32>,
}

impl MetricsTree_Market_Indicators_Macd_1y {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            ema_fast: MetricPattern1::new(client.clone(), "macd_ema_fast_1y".to_string()),
            ema_slow: MetricPattern1::new(client.clone(), "macd_ema_slow_1y".to_string()),
            line: MetricPattern1::new(client.clone(), "macd_line_1y".to_string()),
            signal: MetricPattern1::new(client.clone(), "macd_signal_1y".to_string()),
            histogram: MetricPattern1::new(client.clone(), "macd_histogram_1y".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Pools {
    pub height_to_pool: MetricPattern18<PoolSlug>,
    pub major: MetricsTree_Pools_Major,
    pub minor: MetricsTree_Pools_Minor,
}

impl MetricsTree_Pools {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            height_to_pool: MetricPattern18::new(client.clone(), "pool".to_string()),
            major: MetricsTree_Pools_Major::new(client.clone(), format!("{base_path}_major")),
            minor: MetricsTree_Pools_Minor::new(client.clone(), format!("{base_path}_minor")),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Pools_Major {
    pub unknown: _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern,
    pub luxor: _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern,
    pub btccom: _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern,
    pub btctop: _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern,
    pub btcguild: _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern,
    pub eligius: _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern,
    pub f2pool: _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern,
    pub braiinspool: _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern,
    pub antpool: _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern,
    pub btcc: _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern,
    pub bwpool: _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern,
    pub bitfury: _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern,
    pub viabtc: _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern,
    pub poolin: _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern,
    pub spiderpool: _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern,
    pub binancepool: _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern,
    pub foundryusa: _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern,
    pub sbicrypto: _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern,
    pub marapool: _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern,
    pub secpool: _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern,
    pub ocean: _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern,
    pub whitepool: _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern,
}

impl MetricsTree_Pools_Major {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            unknown: _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern::new(client.clone(), "unknown".to_string()),
            luxor: _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern::new(client.clone(), "luxor".to_string()),
            btccom: _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern::new(client.clone(), "btccom".to_string()),
            btctop: _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern::new(client.clone(), "btctop".to_string()),
            btcguild: _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern::new(client.clone(), "btcguild".to_string()),
            eligius: _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern::new(client.clone(), "eligius".to_string()),
            f2pool: _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern::new(client.clone(), "f2pool".to_string()),
            braiinspool: _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern::new(client.clone(), "braiinspool".to_string()),
            antpool: _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern::new(client.clone(), "antpool".to_string()),
            btcc: _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern::new(client.clone(), "btcc".to_string()),
            bwpool: _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern::new(client.clone(), "bwpool".to_string()),
            bitfury: _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern::new(client.clone(), "bitfury".to_string()),
            viabtc: _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern::new(client.clone(), "viabtc".to_string()),
            poolin: _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern::new(client.clone(), "poolin".to_string()),
            spiderpool: _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern::new(client.clone(), "spiderpool".to_string()),
            binancepool: _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern::new(client.clone(), "binancepool".to_string()),
            foundryusa: _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern::new(client.clone(), "foundryusa".to_string()),
            sbicrypto: _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern::new(client.clone(), "sbicrypto".to_string()),
            marapool: _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern::new(client.clone(), "marapool".to_string()),
            secpool: _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern::new(client.clone(), "secpool".to_string()),
            ocean: _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern::new(client.clone(), "ocean".to_string()),
            whitepool: _1m1w1y24hBaseBpsCumulativeHeightPercentRatioSumPattern::new(client.clone(), "whitepool".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Pools_Minor {
    pub blockfills: BlocksDominancePattern,
    pub ultimuspool: BlocksDominancePattern,
    pub terrapool: BlocksDominancePattern,
    pub onethash: BlocksDominancePattern,
    pub bitfarms: BlocksDominancePattern,
    pub huobipool: BlocksDominancePattern,
    pub wayicn: BlocksDominancePattern,
    pub canoepool: BlocksDominancePattern,
    pub bitcoincom: BlocksDominancePattern,
    pub pool175btc: BlocksDominancePattern,
    pub gbminers: BlocksDominancePattern,
    pub axbt: BlocksDominancePattern,
    pub asicminer: BlocksDominancePattern,
    pub bitminter: BlocksDominancePattern,
    pub bitcoinrussia: BlocksDominancePattern,
    pub btcserv: BlocksDominancePattern,
    pub simplecoinus: BlocksDominancePattern,
    pub ozcoin: BlocksDominancePattern,
    pub eclipsemc: BlocksDominancePattern,
    pub maxbtc: BlocksDominancePattern,
    pub triplemining: BlocksDominancePattern,
    pub coinlab: BlocksDominancePattern,
    pub pool50btc: BlocksDominancePattern,
    pub ghashio: BlocksDominancePattern,
    pub stminingcorp: BlocksDominancePattern,
    pub bitparking: BlocksDominancePattern,
    pub mmpool: BlocksDominancePattern,
    pub polmine: BlocksDominancePattern,
    pub kncminer: BlocksDominancePattern,
    pub bitalo: BlocksDominancePattern,
    pub hhtt: BlocksDominancePattern,
    pub megabigpower: BlocksDominancePattern,
    pub mtred: BlocksDominancePattern,
    pub nmcbit: BlocksDominancePattern,
    pub yourbtcnet: BlocksDominancePattern,
    pub givemecoins: BlocksDominancePattern,
    pub multicoinco: BlocksDominancePattern,
    pub bcpoolio: BlocksDominancePattern,
    pub cointerra: BlocksDominancePattern,
    pub kanopool: BlocksDominancePattern,
    pub solock: BlocksDominancePattern,
    pub ckpool: BlocksDominancePattern,
    pub nicehash: BlocksDominancePattern,
    pub bitclub: BlocksDominancePattern,
    pub bitcoinaffiliatenetwork: BlocksDominancePattern,
    pub exxbw: BlocksDominancePattern,
    pub bitsolo: BlocksDominancePattern,
    pub twentyoneinc: BlocksDominancePattern,
    pub digitalbtc: BlocksDominancePattern,
    pub eightbaochi: BlocksDominancePattern,
    pub mybtccoinpool: BlocksDominancePattern,
    pub tbdice: BlocksDominancePattern,
    pub hashpool: BlocksDominancePattern,
    pub nexious: BlocksDominancePattern,
    pub bravomining: BlocksDominancePattern,
    pub hotpool: BlocksDominancePattern,
    pub okexpool: BlocksDominancePattern,
    pub bcmonster: BlocksDominancePattern,
    pub onehash: BlocksDominancePattern,
    pub bixin: BlocksDominancePattern,
    pub tatmaspool: BlocksDominancePattern,
    pub connectbtc: BlocksDominancePattern,
    pub batpool: BlocksDominancePattern,
    pub waterhole: BlocksDominancePattern,
    pub dcexploration: BlocksDominancePattern,
    pub dcex: BlocksDominancePattern,
    pub btpool: BlocksDominancePattern,
    pub fiftyeightcoin: BlocksDominancePattern,
    pub bitcoinindia: BlocksDominancePattern,
    pub shawnp0wers: BlocksDominancePattern,
    pub phashio: BlocksDominancePattern,
    pub rigpool: BlocksDominancePattern,
    pub haozhuzhu: BlocksDominancePattern,
    pub sevenpool: BlocksDominancePattern,
    pub miningkings: BlocksDominancePattern,
    pub hashbx: BlocksDominancePattern,
    pub dpool: BlocksDominancePattern,
    pub rawpool: BlocksDominancePattern,
    pub haominer: BlocksDominancePattern,
    pub helix: BlocksDominancePattern,
    pub bitcoinukraine: BlocksDominancePattern,
    pub secretsuperstar: BlocksDominancePattern,
    pub tigerpoolnet: BlocksDominancePattern,
    pub sigmapoolcom: BlocksDominancePattern,
    pub okpooltop: BlocksDominancePattern,
    pub hummerpool: BlocksDominancePattern,
    pub tangpool: BlocksDominancePattern,
    pub bytepool: BlocksDominancePattern,
    pub novablock: BlocksDominancePattern,
    pub miningcity: BlocksDominancePattern,
    pub minerium: BlocksDominancePattern,
    pub lubiancom: BlocksDominancePattern,
    pub okkong: BlocksDominancePattern,
    pub aaopool: BlocksDominancePattern,
    pub emcdpool: BlocksDominancePattern,
    pub arkpool: BlocksDominancePattern,
    pub purebtccom: BlocksDominancePattern,
    pub kucoinpool: BlocksDominancePattern,
    pub entrustcharitypool: BlocksDominancePattern,
    pub okminer: BlocksDominancePattern,
    pub titan: BlocksDominancePattern,
    pub pegapool: BlocksDominancePattern,
    pub btcnuggets: BlocksDominancePattern,
    pub cloudhashing: BlocksDominancePattern,
    pub digitalxmintsy: BlocksDominancePattern,
    pub telco214: BlocksDominancePattern,
    pub btcpoolparty: BlocksDominancePattern,
    pub multipool: BlocksDominancePattern,
    pub transactioncoinmining: BlocksDominancePattern,
    pub btcdig: BlocksDominancePattern,
    pub trickysbtcpool: BlocksDominancePattern,
    pub btcmp: BlocksDominancePattern,
    pub eobot: BlocksDominancePattern,
    pub unomp: BlocksDominancePattern,
    pub patels: BlocksDominancePattern,
    pub gogreenlight: BlocksDominancePattern,
    pub bitcoinindiapool: BlocksDominancePattern,
    pub ekanembtc: BlocksDominancePattern,
    pub canoe: BlocksDominancePattern,
    pub tiger: BlocksDominancePattern,
    pub onem1x: BlocksDominancePattern,
    pub zulupool: BlocksDominancePattern,
    pub wiz: BlocksDominancePattern,
    pub wk057: BlocksDominancePattern,
    pub futurebitapollosolo: BlocksDominancePattern,
    pub carbonnegative: BlocksDominancePattern,
    pub portlandhodl: BlocksDominancePattern,
    pub phoenix: BlocksDominancePattern,
    pub neopool: BlocksDominancePattern,
    pub maxipool: BlocksDominancePattern,
    pub bitfufupool: BlocksDominancePattern,
    pub gdpool: BlocksDominancePattern,
    pub miningdutch: BlocksDominancePattern,
    pub publicpool: BlocksDominancePattern,
    pub miningsquared: BlocksDominancePattern,
    pub innopolistech: BlocksDominancePattern,
    pub btclab: BlocksDominancePattern,
    pub parasite: BlocksDominancePattern,
    pub redrockpool: BlocksDominancePattern,
    pub est3lar: BlocksDominancePattern,
}

impl MetricsTree_Pools_Minor {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            blockfills: BlocksDominancePattern::new(client.clone(), "blockfills".to_string()),
            ultimuspool: BlocksDominancePattern::new(client.clone(), "ultimuspool".to_string()),
            terrapool: BlocksDominancePattern::new(client.clone(), "terrapool".to_string()),
            onethash: BlocksDominancePattern::new(client.clone(), "onethash".to_string()),
            bitfarms: BlocksDominancePattern::new(client.clone(), "bitfarms".to_string()),
            huobipool: BlocksDominancePattern::new(client.clone(), "huobipool".to_string()),
            wayicn: BlocksDominancePattern::new(client.clone(), "wayicn".to_string()),
            canoepool: BlocksDominancePattern::new(client.clone(), "canoepool".to_string()),
            bitcoincom: BlocksDominancePattern::new(client.clone(), "bitcoincom".to_string()),
            pool175btc: BlocksDominancePattern::new(client.clone(), "pool175btc".to_string()),
            gbminers: BlocksDominancePattern::new(client.clone(), "gbminers".to_string()),
            axbt: BlocksDominancePattern::new(client.clone(), "axbt".to_string()),
            asicminer: BlocksDominancePattern::new(client.clone(), "asicminer".to_string()),
            bitminter: BlocksDominancePattern::new(client.clone(), "bitminter".to_string()),
            bitcoinrussia: BlocksDominancePattern::new(client.clone(), "bitcoinrussia".to_string()),
            btcserv: BlocksDominancePattern::new(client.clone(), "btcserv".to_string()),
            simplecoinus: BlocksDominancePattern::new(client.clone(), "simplecoinus".to_string()),
            ozcoin: BlocksDominancePattern::new(client.clone(), "ozcoin".to_string()),
            eclipsemc: BlocksDominancePattern::new(client.clone(), "eclipsemc".to_string()),
            maxbtc: BlocksDominancePattern::new(client.clone(), "maxbtc".to_string()),
            triplemining: BlocksDominancePattern::new(client.clone(), "triplemining".to_string()),
            coinlab: BlocksDominancePattern::new(client.clone(), "coinlab".to_string()),
            pool50btc: BlocksDominancePattern::new(client.clone(), "pool50btc".to_string()),
            ghashio: BlocksDominancePattern::new(client.clone(), "ghashio".to_string()),
            stminingcorp: BlocksDominancePattern::new(client.clone(), "stminingcorp".to_string()),
            bitparking: BlocksDominancePattern::new(client.clone(), "bitparking".to_string()),
            mmpool: BlocksDominancePattern::new(client.clone(), "mmpool".to_string()),
            polmine: BlocksDominancePattern::new(client.clone(), "polmine".to_string()),
            kncminer: BlocksDominancePattern::new(client.clone(), "kncminer".to_string()),
            bitalo: BlocksDominancePattern::new(client.clone(), "bitalo".to_string()),
            hhtt: BlocksDominancePattern::new(client.clone(), "hhtt".to_string()),
            megabigpower: BlocksDominancePattern::new(client.clone(), "megabigpower".to_string()),
            mtred: BlocksDominancePattern::new(client.clone(), "mtred".to_string()),
            nmcbit: BlocksDominancePattern::new(client.clone(), "nmcbit".to_string()),
            yourbtcnet: BlocksDominancePattern::new(client.clone(), "yourbtcnet".to_string()),
            givemecoins: BlocksDominancePattern::new(client.clone(), "givemecoins".to_string()),
            multicoinco: BlocksDominancePattern::new(client.clone(), "multicoinco".to_string()),
            bcpoolio: BlocksDominancePattern::new(client.clone(), "bcpoolio".to_string()),
            cointerra: BlocksDominancePattern::new(client.clone(), "cointerra".to_string()),
            kanopool: BlocksDominancePattern::new(client.clone(), "kanopool".to_string()),
            solock: BlocksDominancePattern::new(client.clone(), "solock".to_string()),
            ckpool: BlocksDominancePattern::new(client.clone(), "ckpool".to_string()),
            nicehash: BlocksDominancePattern::new(client.clone(), "nicehash".to_string()),
            bitclub: BlocksDominancePattern::new(client.clone(), "bitclub".to_string()),
            bitcoinaffiliatenetwork: BlocksDominancePattern::new(client.clone(), "bitcoinaffiliatenetwork".to_string()),
            exxbw: BlocksDominancePattern::new(client.clone(), "exxbw".to_string()),
            bitsolo: BlocksDominancePattern::new(client.clone(), "bitsolo".to_string()),
            twentyoneinc: BlocksDominancePattern::new(client.clone(), "twentyoneinc".to_string()),
            digitalbtc: BlocksDominancePattern::new(client.clone(), "digitalbtc".to_string()),
            eightbaochi: BlocksDominancePattern::new(client.clone(), "eightbaochi".to_string()),
            mybtccoinpool: BlocksDominancePattern::new(client.clone(), "mybtccoinpool".to_string()),
            tbdice: BlocksDominancePattern::new(client.clone(), "tbdice".to_string()),
            hashpool: BlocksDominancePattern::new(client.clone(), "hashpool".to_string()),
            nexious: BlocksDominancePattern::new(client.clone(), "nexious".to_string()),
            bravomining: BlocksDominancePattern::new(client.clone(), "bravomining".to_string()),
            hotpool: BlocksDominancePattern::new(client.clone(), "hotpool".to_string()),
            okexpool: BlocksDominancePattern::new(client.clone(), "okexpool".to_string()),
            bcmonster: BlocksDominancePattern::new(client.clone(), "bcmonster".to_string()),
            onehash: BlocksDominancePattern::new(client.clone(), "onehash".to_string()),
            bixin: BlocksDominancePattern::new(client.clone(), "bixin".to_string()),
            tatmaspool: BlocksDominancePattern::new(client.clone(), "tatmaspool".to_string()),
            connectbtc: BlocksDominancePattern::new(client.clone(), "connectbtc".to_string()),
            batpool: BlocksDominancePattern::new(client.clone(), "batpool".to_string()),
            waterhole: BlocksDominancePattern::new(client.clone(), "waterhole".to_string()),
            dcexploration: BlocksDominancePattern::new(client.clone(), "dcexploration".to_string()),
            dcex: BlocksDominancePattern::new(client.clone(), "dcex".to_string()),
            btpool: BlocksDominancePattern::new(client.clone(), "btpool".to_string()),
            fiftyeightcoin: BlocksDominancePattern::new(client.clone(), "fiftyeightcoin".to_string()),
            bitcoinindia: BlocksDominancePattern::new(client.clone(), "bitcoinindia".to_string()),
            shawnp0wers: BlocksDominancePattern::new(client.clone(), "shawnp0wers".to_string()),
            phashio: BlocksDominancePattern::new(client.clone(), "phashio".to_string()),
            rigpool: BlocksDominancePattern::new(client.clone(), "rigpool".to_string()),
            haozhuzhu: BlocksDominancePattern::new(client.clone(), "haozhuzhu".to_string()),
            sevenpool: BlocksDominancePattern::new(client.clone(), "sevenpool".to_string()),
            miningkings: BlocksDominancePattern::new(client.clone(), "miningkings".to_string()),
            hashbx: BlocksDominancePattern::new(client.clone(), "hashbx".to_string()),
            dpool: BlocksDominancePattern::new(client.clone(), "dpool".to_string()),
            rawpool: BlocksDominancePattern::new(client.clone(), "rawpool".to_string()),
            haominer: BlocksDominancePattern::new(client.clone(), "haominer".to_string()),
            helix: BlocksDominancePattern::new(client.clone(), "helix".to_string()),
            bitcoinukraine: BlocksDominancePattern::new(client.clone(), "bitcoinukraine".to_string()),
            secretsuperstar: BlocksDominancePattern::new(client.clone(), "secretsuperstar".to_string()),
            tigerpoolnet: BlocksDominancePattern::new(client.clone(), "tigerpoolnet".to_string()),
            sigmapoolcom: BlocksDominancePattern::new(client.clone(), "sigmapoolcom".to_string()),
            okpooltop: BlocksDominancePattern::new(client.clone(), "okpooltop".to_string()),
            hummerpool: BlocksDominancePattern::new(client.clone(), "hummerpool".to_string()),
            tangpool: BlocksDominancePattern::new(client.clone(), "tangpool".to_string()),
            bytepool: BlocksDominancePattern::new(client.clone(), "bytepool".to_string()),
            novablock: BlocksDominancePattern::new(client.clone(), "novablock".to_string()),
            miningcity: BlocksDominancePattern::new(client.clone(), "miningcity".to_string()),
            minerium: BlocksDominancePattern::new(client.clone(), "minerium".to_string()),
            lubiancom: BlocksDominancePattern::new(client.clone(), "lubiancom".to_string()),
            okkong: BlocksDominancePattern::new(client.clone(), "okkong".to_string()),
            aaopool: BlocksDominancePattern::new(client.clone(), "aaopool".to_string()),
            emcdpool: BlocksDominancePattern::new(client.clone(), "emcdpool".to_string()),
            arkpool: BlocksDominancePattern::new(client.clone(), "arkpool".to_string()),
            purebtccom: BlocksDominancePattern::new(client.clone(), "purebtccom".to_string()),
            kucoinpool: BlocksDominancePattern::new(client.clone(), "kucoinpool".to_string()),
            entrustcharitypool: BlocksDominancePattern::new(client.clone(), "entrustcharitypool".to_string()),
            okminer: BlocksDominancePattern::new(client.clone(), "okminer".to_string()),
            titan: BlocksDominancePattern::new(client.clone(), "titan".to_string()),
            pegapool: BlocksDominancePattern::new(client.clone(), "pegapool".to_string()),
            btcnuggets: BlocksDominancePattern::new(client.clone(), "btcnuggets".to_string()),
            cloudhashing: BlocksDominancePattern::new(client.clone(), "cloudhashing".to_string()),
            digitalxmintsy: BlocksDominancePattern::new(client.clone(), "digitalxmintsy".to_string()),
            telco214: BlocksDominancePattern::new(client.clone(), "telco214".to_string()),
            btcpoolparty: BlocksDominancePattern::new(client.clone(), "btcpoolparty".to_string()),
            multipool: BlocksDominancePattern::new(client.clone(), "multipool".to_string()),
            transactioncoinmining: BlocksDominancePattern::new(client.clone(), "transactioncoinmining".to_string()),
            btcdig: BlocksDominancePattern::new(client.clone(), "btcdig".to_string()),
            trickysbtcpool: BlocksDominancePattern::new(client.clone(), "trickysbtcpool".to_string()),
            btcmp: BlocksDominancePattern::new(client.clone(), "btcmp".to_string()),
            eobot: BlocksDominancePattern::new(client.clone(), "eobot".to_string()),
            unomp: BlocksDominancePattern::new(client.clone(), "unomp".to_string()),
            patels: BlocksDominancePattern::new(client.clone(), "patels".to_string()),
            gogreenlight: BlocksDominancePattern::new(client.clone(), "gogreenlight".to_string()),
            bitcoinindiapool: BlocksDominancePattern::new(client.clone(), "bitcoinindiapool".to_string()),
            ekanembtc: BlocksDominancePattern::new(client.clone(), "ekanembtc".to_string()),
            canoe: BlocksDominancePattern::new(client.clone(), "canoe".to_string()),
            tiger: BlocksDominancePattern::new(client.clone(), "tiger".to_string()),
            onem1x: BlocksDominancePattern::new(client.clone(), "onem1x".to_string()),
            zulupool: BlocksDominancePattern::new(client.clone(), "zulupool".to_string()),
            wiz: BlocksDominancePattern::new(client.clone(), "wiz".to_string()),
            wk057: BlocksDominancePattern::new(client.clone(), "wk057".to_string()),
            futurebitapollosolo: BlocksDominancePattern::new(client.clone(), "futurebitapollosolo".to_string()),
            carbonnegative: BlocksDominancePattern::new(client.clone(), "carbonnegative".to_string()),
            portlandhodl: BlocksDominancePattern::new(client.clone(), "portlandhodl".to_string()),
            phoenix: BlocksDominancePattern::new(client.clone(), "phoenix".to_string()),
            neopool: BlocksDominancePattern::new(client.clone(), "neopool".to_string()),
            maxipool: BlocksDominancePattern::new(client.clone(), "maxipool".to_string()),
            bitfufupool: BlocksDominancePattern::new(client.clone(), "bitfufupool".to_string()),
            gdpool: BlocksDominancePattern::new(client.clone(), "gdpool".to_string()),
            miningdutch: BlocksDominancePattern::new(client.clone(), "miningdutch".to_string()),
            publicpool: BlocksDominancePattern::new(client.clone(), "publicpool".to_string()),
            miningsquared: BlocksDominancePattern::new(client.clone(), "miningsquared".to_string()),
            innopolistech: BlocksDominancePattern::new(client.clone(), "innopolistech".to_string()),
            btclab: BlocksDominancePattern::new(client.clone(), "btclab".to_string()),
            parasite: BlocksDominancePattern::new(client.clone(), "parasite".to_string()),
            redrockpool: BlocksDominancePattern::new(client.clone(), "redrockpool".to_string()),
            est3lar: BlocksDominancePattern::new(client.clone(), "est3lar".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Prices {
    pub split: MetricsTree_Prices_Split,
    pub ohlc: MetricsTree_Prices_Ohlc,
    pub price: MetricsTree_Prices_Price,
}

impl MetricsTree_Prices {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            split: MetricsTree_Prices_Split::new(client.clone(), format!("{base_path}_split")),
            ohlc: MetricsTree_Prices_Ohlc::new(client.clone(), format!("{base_path}_ohlc")),
            price: MetricsTree_Prices_Price::new(client.clone(), format!("{base_path}_price")),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Prices_Split {
    pub open: CentsSatsUsdPattern2,
    pub high: CentsSatsUsdPattern2,
    pub low: CentsSatsUsdPattern2,
    pub close: MetricsTree_Prices_Split_Close,
}

impl MetricsTree_Prices_Split {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            open: CentsSatsUsdPattern2::new(client.clone(), "price_open".to_string()),
            high: CentsSatsUsdPattern2::new(client.clone(), "price_high".to_string()),
            low: CentsSatsUsdPattern2::new(client.clone(), "price_low".to_string()),
            close: MetricsTree_Prices_Split_Close::new(client.clone(), format!("{base_path}_close")),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Prices_Split_Close {
    pub cents: MetricPattern2<Cents>,
    pub usd: MetricPattern2<Dollars>,
    pub sats: MetricPattern2<Sats>,
}

impl MetricsTree_Prices_Split_Close {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            cents: MetricPattern2::new(client.clone(), "price_close_cents".to_string()),
            usd: MetricPattern2::new(client.clone(), "price_close".to_string()),
            sats: MetricPattern2::new(client.clone(), "price_close_sats".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Prices_Ohlc {
    pub cents: MetricPattern2<OHLCCents>,
    pub usd: MetricPattern2<OHLCDollars>,
    pub sats: MetricPattern2<OHLCSats>,
}

impl MetricsTree_Prices_Ohlc {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            cents: MetricPattern2::new(client.clone(), "price_ohlc_cents".to_string()),
            usd: MetricPattern2::new(client.clone(), "price_ohlc".to_string()),
            sats: MetricPattern2::new(client.clone(), "price_ohlc_sats".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Prices_Price {
    pub cents: MetricPattern1<Cents>,
    pub usd: MetricPattern1<Dollars>,
    pub sats: MetricPattern1<Sats>,
}

impl MetricsTree_Prices_Price {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            cents: MetricPattern1::new(client.clone(), "price_cents".to_string()),
            usd: MetricPattern1::new(client.clone(), "price".to_string()),
            sats: MetricPattern1::new(client.clone(), "price_sats".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Distribution {
    pub supply_state: MetricPattern18<SupplyState>,
    pub any_address_indexes: MetricsTree_Distribution_AnyAddressIndexes,
    pub addresses_data: MetricsTree_Distribution_AddressesData,
    pub utxo_cohorts: MetricsTree_Distribution_UtxoCohorts,
    pub address_cohorts: MetricsTree_Distribution_AddressCohorts,
    pub addr_count: AllP2aP2pk33P2pk65P2pkhP2shP2trP2wpkhP2wshPattern,
    pub empty_addr_count: AllP2aP2pk33P2pk65P2pkhP2shP2trP2wpkhP2wshPattern,
    pub address_activity: MetricsTree_Distribution_AddressActivity,
    pub total_addr_count: AllP2aP2pk33P2pk65P2pkhP2shP2trP2wpkhP2wshPattern,
    pub new_addr_count: MetricsTree_Distribution_NewAddrCount,
    pub delta: MetricsTree_Distribution_Delta,
    pub fundedaddressindex: MetricPattern34<FundedAddressIndex>,
    pub emptyaddressindex: MetricPattern35<EmptyAddressIndex>,
}

impl MetricsTree_Distribution {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            supply_state: MetricPattern18::new(client.clone(), "supply_state".to_string()),
            any_address_indexes: MetricsTree_Distribution_AnyAddressIndexes::new(client.clone(), format!("{base_path}_any_address_indexes")),
            addresses_data: MetricsTree_Distribution_AddressesData::new(client.clone(), format!("{base_path}_addresses_data")),
            utxo_cohorts: MetricsTree_Distribution_UtxoCohorts::new(client.clone(), format!("{base_path}_utxo_cohorts")),
            address_cohorts: MetricsTree_Distribution_AddressCohorts::new(client.clone(), format!("{base_path}_address_cohorts")),
            addr_count: AllP2aP2pk33P2pk65P2pkhP2shP2trP2wpkhP2wshPattern::new(client.clone(), "addr_count".to_string()),
            empty_addr_count: AllP2aP2pk33P2pk65P2pkhP2shP2trP2wpkhP2wshPattern::new(client.clone(), "empty_addr_count".to_string()),
            address_activity: MetricsTree_Distribution_AddressActivity::new(client.clone(), format!("{base_path}_address_activity")),
            total_addr_count: AllP2aP2pk33P2pk65P2pkhP2shP2trP2wpkhP2wshPattern::new(client.clone(), "total_addr_count".to_string()),
            new_addr_count: MetricsTree_Distribution_NewAddrCount::new(client.clone(), format!("{base_path}_new_addr_count")),
            delta: MetricsTree_Distribution_Delta::new(client.clone(), format!("{base_path}_delta")),
            fundedaddressindex: MetricPattern34::new(client.clone(), "fundedaddressindex".to_string()),
            emptyaddressindex: MetricPattern35::new(client.clone(), "emptyaddressindex".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Distribution_AnyAddressIndexes {
    pub p2a: MetricPattern24<AnyAddressIndex>,
    pub p2pk33: MetricPattern26<AnyAddressIndex>,
    pub p2pk65: MetricPattern27<AnyAddressIndex>,
    pub p2pkh: MetricPattern28<AnyAddressIndex>,
    pub p2sh: MetricPattern29<AnyAddressIndex>,
    pub p2tr: MetricPattern30<AnyAddressIndex>,
    pub p2wpkh: MetricPattern31<AnyAddressIndex>,
    pub p2wsh: MetricPattern32<AnyAddressIndex>,
}

impl MetricsTree_Distribution_AnyAddressIndexes {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            p2a: MetricPattern24::new(client.clone(), "anyaddressindex".to_string()),
            p2pk33: MetricPattern26::new(client.clone(), "anyaddressindex".to_string()),
            p2pk65: MetricPattern27::new(client.clone(), "anyaddressindex".to_string()),
            p2pkh: MetricPattern28::new(client.clone(), "anyaddressindex".to_string()),
            p2sh: MetricPattern29::new(client.clone(), "anyaddressindex".to_string()),
            p2tr: MetricPattern30::new(client.clone(), "anyaddressindex".to_string()),
            p2wpkh: MetricPattern31::new(client.clone(), "anyaddressindex".to_string()),
            p2wsh: MetricPattern32::new(client.clone(), "anyaddressindex".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Distribution_AddressesData {
    pub funded: MetricPattern34<FundedAddressData>,
    pub empty: MetricPattern35<EmptyAddressData>,
}

impl MetricsTree_Distribution_AddressesData {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            funded: MetricPattern34::new(client.clone(), "fundedaddressdata".to_string()),
            empty: MetricPattern35::new(client.clone(), "emptyaddressdata".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Distribution_UtxoCohorts {
    pub all: MetricsTree_Distribution_UtxoCohorts_All,
    pub sth: MetricsTree_Distribution_UtxoCohorts_Sth,
    pub lth: MetricsTree_Distribution_UtxoCohorts_Lth,
    pub age_range: MetricsTree_Distribution_UtxoCohorts_AgeRange,
    pub max_age: MetricsTree_Distribution_UtxoCohorts_MaxAge,
    pub min_age: MetricsTree_Distribution_UtxoCohorts_MinAge,
    pub epoch: MetricsTree_Distribution_UtxoCohorts_Epoch,
    pub class: MetricsTree_Distribution_UtxoCohorts_Class,
    pub ge_amount: MetricsTree_Distribution_UtxoCohorts_GeAmount,
    pub amount_range: MetricsTree_Distribution_UtxoCohorts_AmountRange,
    pub lt_amount: MetricsTree_Distribution_UtxoCohorts_LtAmount,
    pub type_: MetricsTree_Distribution_UtxoCohorts_Type,
}

impl MetricsTree_Distribution_UtxoCohorts {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            all: MetricsTree_Distribution_UtxoCohorts_All::new(client.clone(), format!("{base_path}_all")),
            sth: MetricsTree_Distribution_UtxoCohorts_Sth::new(client.clone(), format!("{base_path}_sth")),
            lth: MetricsTree_Distribution_UtxoCohorts_Lth::new(client.clone(), format!("{base_path}_lth")),
            age_range: MetricsTree_Distribution_UtxoCohorts_AgeRange::new(client.clone(), format!("{base_path}_age_range")),
            max_age: MetricsTree_Distribution_UtxoCohorts_MaxAge::new(client.clone(), format!("{base_path}_max_age")),
            min_age: MetricsTree_Distribution_UtxoCohorts_MinAge::new(client.clone(), format!("{base_path}_min_age")),
            epoch: MetricsTree_Distribution_UtxoCohorts_Epoch::new(client.clone(), format!("{base_path}_epoch")),
            class: MetricsTree_Distribution_UtxoCohorts_Class::new(client.clone(), format!("{base_path}_class")),
            ge_amount: MetricsTree_Distribution_UtxoCohorts_GeAmount::new(client.clone(), format!("{base_path}_ge_amount")),
            amount_range: MetricsTree_Distribution_UtxoCohorts_AmountRange::new(client.clone(), format!("{base_path}_amount_range")),
            lt_amount: MetricsTree_Distribution_UtxoCohorts_LtAmount::new(client.clone(), format!("{base_path}_lt_amount")),
            type_: MetricsTree_Distribution_UtxoCohorts_Type::new(client.clone(), format!("{base_path}_type_")),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Distribution_UtxoCohorts_All {
    pub supply: ChangeHalvedTotalPattern,
    pub outputs: UtxoPattern,
    pub activity: CoinblocksCoindaysSentPattern,
    pub realized: CapCapitulationGrossInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprUpperValuePattern,
    pub cost_basis: InvestedMaxMinPercentilesPattern,
    pub unrealized: GreedGrossInvestedInvestorNegNetPainSupplyUnrealizedPattern,
    pub adjusted: MetricsTree_Distribution_UtxoCohorts_All_Adjusted,
    pub relative: MetricsTree_Distribution_UtxoCohorts_All_Relative,
    pub dormancy: MetricPattern1<StoredF32>,
    pub velocity: MetricPattern1<StoredF32>,
}

impl MetricsTree_Distribution_UtxoCohorts_All {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            supply: ChangeHalvedTotalPattern::new(client.clone(), "supply".to_string()),
            outputs: UtxoPattern::new(client.clone(), "utxo_count".to_string()),
            activity: CoinblocksCoindaysSentPattern::new(client.clone(), "".to_string()),
            realized: CapCapitulationGrossInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprUpperValuePattern::new(client.clone(), "".to_string()),
            cost_basis: InvestedMaxMinPercentilesPattern::new(client.clone(), "".to_string()),
            unrealized: GreedGrossInvestedInvestorNegNetPainSupplyUnrealizedPattern::new(client.clone(), "".to_string()),
            adjusted: MetricsTree_Distribution_UtxoCohorts_All_Adjusted::new(client.clone(), format!("{base_path}_adjusted")),
            relative: MetricsTree_Distribution_UtxoCohorts_All_Relative::new(client.clone(), format!("{base_path}_relative")),
            dormancy: MetricPattern1::new(client.clone(), "dormancy".to_string()),
            velocity: MetricPattern1::new(client.clone(), "velocity".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Distribution_UtxoCohorts_All_Adjusted {
    pub adjusted_value_created: MetricPattern1<Cents>,
    pub adjusted_value_destroyed: MetricPattern1<Cents>,
    pub adjusted_value_created_sum: _1m1w1y24hPattern<Cents>,
    pub adjusted_value_destroyed_sum: _1m1w1y24hPattern<Cents>,
    pub adjusted_sopr: _1m1w1y24hPattern<StoredF64>,
    pub adjusted_sopr_ema: _1m1wPattern,
}

impl MetricsTree_Distribution_UtxoCohorts_All_Adjusted {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            adjusted_value_created: MetricPattern1::new(client.clone(), "adjusted_value_created".to_string()),
            adjusted_value_destroyed: MetricPattern1::new(client.clone(), "adjusted_value_destroyed".to_string()),
            adjusted_value_created_sum: _1m1w1y24hPattern::new(client.clone(), "adjusted_value_created".to_string()),
            adjusted_value_destroyed_sum: _1m1w1y24hPattern::new(client.clone(), "adjusted_value_destroyed".to_string()),
            adjusted_sopr: _1m1w1y24hPattern::new(client.clone(), "adjusted_sopr".to_string()),
            adjusted_sopr_ema: _1m1wPattern::new(client.clone(), "adjusted_sopr_24h_ema".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Distribution_UtxoCohorts_All_Relative {
    pub unrealized_profit_rel_to_market_cap: BpsPercentRatioPattern,
    pub unrealized_loss_rel_to_market_cap: BpsPercentRatioPattern,
    pub net_unrealized_pnl_rel_to_market_cap: BpsPercentRatioPattern,
    pub nupl: MetricPattern1<StoredF32>,
    pub supply_in_profit_rel_to_own_supply: BpsPercentRatioPattern,
    pub supply_in_loss_rel_to_own_supply: BpsPercentRatioPattern,
    pub unrealized_profit_rel_to_own_gross_pnl: BpsPercentRatioPattern,
    pub unrealized_loss_rel_to_own_gross_pnl: BpsPercentRatioPattern,
    pub net_unrealized_pnl_rel_to_own_gross_pnl: BpsPercentRatioPattern,
}

impl MetricsTree_Distribution_UtxoCohorts_All_Relative {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            unrealized_profit_rel_to_market_cap: BpsPercentRatioPattern::new(client.clone(), "unrealized_profit_rel_to_market_cap".to_string()),
            unrealized_loss_rel_to_market_cap: BpsPercentRatioPattern::new(client.clone(), "unrealized_loss_rel_to_market_cap".to_string()),
            net_unrealized_pnl_rel_to_market_cap: BpsPercentRatioPattern::new(client.clone(), "net_unrealized_pnl_rel_to_market_cap".to_string()),
            nupl: MetricPattern1::new(client.clone(), "nupl".to_string()),
            supply_in_profit_rel_to_own_supply: BpsPercentRatioPattern::new(client.clone(), "supply_in_profit_rel_to_own_supply".to_string()),
            supply_in_loss_rel_to_own_supply: BpsPercentRatioPattern::new(client.clone(), "supply_in_loss_rel_to_own_supply".to_string()),
            unrealized_profit_rel_to_own_gross_pnl: BpsPercentRatioPattern::new(client.clone(), "unrealized_profit_rel_to_own_gross_pnl".to_string()),
            unrealized_loss_rel_to_own_gross_pnl: BpsPercentRatioPattern::new(client.clone(), "unrealized_loss_rel_to_own_gross_pnl".to_string()),
            net_unrealized_pnl_rel_to_own_gross_pnl: BpsPercentRatioPattern::new(client.clone(), "net_unrealized_pnl_rel_to_own_gross_pnl".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Distribution_UtxoCohorts_Sth {
    pub supply: ChangeHalvedTotalPattern,
    pub outputs: UtxoPattern,
    pub activity: CoinblocksCoindaysSentPattern,
    pub realized: CapCapitulationGrossInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprUpperValuePattern,
    pub cost_basis: InvestedMaxMinPercentilesPattern,
    pub unrealized: GreedGrossInvestedInvestorNegNetPainSupplyUnrealizedPattern,
    pub relative: NetNuplSupplyUnrealizedPattern2,
    pub dormancy: MetricPattern1<StoredF32>,
    pub velocity: MetricPattern1<StoredF32>,
    pub adjusted_value_created: MetricPattern1<Cents>,
    pub adjusted_value_destroyed: MetricPattern1<Cents>,
    pub adjusted_value_created_sum: _1m1w1y24hPattern<Cents>,
    pub adjusted_value_destroyed_sum: _1m1w1y24hPattern<Cents>,
    pub adjusted_sopr: _1m1w1y24hPattern<StoredF64>,
    pub adjusted_sopr_ema: _1m1wPattern,
}

impl MetricsTree_Distribution_UtxoCohorts_Sth {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            supply: ChangeHalvedTotalPattern::new(client.clone(), "sth_supply".to_string()),
            outputs: UtxoPattern::new(client.clone(), "sth_utxo_count".to_string()),
            activity: CoinblocksCoindaysSentPattern::new(client.clone(), "sth".to_string()),
            realized: CapCapitulationGrossInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprUpperValuePattern::new(client.clone(), "sth".to_string()),
            cost_basis: InvestedMaxMinPercentilesPattern::new(client.clone(), "sth".to_string()),
            unrealized: GreedGrossInvestedInvestorNegNetPainSupplyUnrealizedPattern::new(client.clone(), "sth".to_string()),
            relative: NetNuplSupplyUnrealizedPattern2::new(client.clone(), "sth".to_string()),
            dormancy: MetricPattern1::new(client.clone(), "sth_dormancy".to_string()),
            velocity: MetricPattern1::new(client.clone(), "sth_velocity".to_string()),
            adjusted_value_created: MetricPattern1::new(client.clone(), "sth_adjusted_value_created".to_string()),
            adjusted_value_destroyed: MetricPattern1::new(client.clone(), "sth_adjusted_value_destroyed".to_string()),
            adjusted_value_created_sum: _1m1w1y24hPattern::new(client.clone(), "sth_adjusted_value_created".to_string()),
            adjusted_value_destroyed_sum: _1m1w1y24hPattern::new(client.clone(), "sth_adjusted_value_destroyed".to_string()),
            adjusted_sopr: _1m1w1y24hPattern::new(client.clone(), "sth_adjusted_sopr".to_string()),
            adjusted_sopr_ema: _1m1wPattern::new(client.clone(), "sth_adjusted_sopr_24h_ema".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Distribution_UtxoCohorts_Lth {
    pub supply: ChangeHalvedTotalPattern,
    pub outputs: UtxoPattern,
    pub activity: CoinblocksCoindaysSentPattern,
    pub realized: CapCapitulationGrossInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprUpperValuePattern,
    pub cost_basis: InvestedMaxMinPercentilesPattern,
    pub unrealized: GreedGrossInvestedInvestorNegNetPainSupplyUnrealizedPattern,
    pub relative: NetNuplSupplyUnrealizedPattern2,
    pub dormancy: MetricPattern1<StoredF32>,
    pub velocity: MetricPattern1<StoredF32>,
}

impl MetricsTree_Distribution_UtxoCohorts_Lth {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            supply: ChangeHalvedTotalPattern::new(client.clone(), "lth_supply".to_string()),
            outputs: UtxoPattern::new(client.clone(), "lth_utxo_count".to_string()),
            activity: CoinblocksCoindaysSentPattern::new(client.clone(), "lth".to_string()),
            realized: CapCapitulationGrossInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprUpperValuePattern::new(client.clone(), "lth".to_string()),
            cost_basis: InvestedMaxMinPercentilesPattern::new(client.clone(), "lth".to_string()),
            unrealized: GreedGrossInvestedInvestorNegNetPainSupplyUnrealizedPattern::new(client.clone(), "lth".to_string()),
            relative: NetNuplSupplyUnrealizedPattern2::new(client.clone(), "lth".to_string()),
            dormancy: MetricPattern1::new(client.clone(), "lth_dormancy".to_string()),
            velocity: MetricPattern1::new(client.clone(), "lth_velocity".to_string()),
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
pub struct MetricsTree_Distribution_UtxoCohorts_MaxAge {
    pub _1w: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _1m: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _2m: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _3m: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _4m: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _5m: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _6m: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _1y: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _2y: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _3y: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _4y: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _5y: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _6y: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _7y: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _8y: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _10y: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _12y: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _15y: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern,
}

impl MetricsTree_Distribution_UtxoCohorts_MaxAge {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _1w: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "utxos_under_1w_old".to_string()),
            _1m: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "utxos_under_1m_old".to_string()),
            _2m: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "utxos_under_2m_old".to_string()),
            _3m: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "utxos_under_3m_old".to_string()),
            _4m: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "utxos_under_4m_old".to_string()),
            _5m: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "utxos_under_5m_old".to_string()),
            _6m: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "utxos_under_6m_old".to_string()),
            _1y: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "utxos_under_1y_old".to_string()),
            _2y: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "utxos_under_2y_old".to_string()),
            _3y: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "utxos_under_3y_old".to_string()),
            _4y: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "utxos_under_4y_old".to_string()),
            _5y: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "utxos_under_5y_old".to_string()),
            _6y: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "utxos_under_6y_old".to_string()),
            _7y: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "utxos_under_7y_old".to_string()),
            _8y: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "utxos_under_8y_old".to_string()),
            _10y: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "utxos_under_10y_old".to_string()),
            _12y: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "utxos_under_12y_old".to_string()),
            _15y: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "utxos_under_15y_old".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Distribution_UtxoCohorts_MinAge {
    pub _1d: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _1w: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _1m: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _2m: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _3m: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _4m: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _5m: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _6m: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _1y: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _2y: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _3y: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _4y: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _5y: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _6y: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _7y: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _8y: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _10y: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _12y: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern,
}

impl MetricsTree_Distribution_UtxoCohorts_MinAge {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _1d: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "utxos_over_1d_old".to_string()),
            _1w: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "utxos_over_1w_old".to_string()),
            _1m: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "utxos_over_1m_old".to_string()),
            _2m: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "utxos_over_2m_old".to_string()),
            _3m: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "utxos_over_3m_old".to_string()),
            _4m: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "utxos_over_4m_old".to_string()),
            _5m: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "utxos_over_5m_old".to_string()),
            _6m: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "utxos_over_6m_old".to_string()),
            _1y: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "utxos_over_1y_old".to_string()),
            _2y: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "utxos_over_2y_old".to_string()),
            _3y: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "utxos_over_3y_old".to_string()),
            _4y: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "utxos_over_4y_old".to_string()),
            _5y: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "utxos_over_5y_old".to_string()),
            _6y: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "utxos_over_6y_old".to_string()),
            _7y: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "utxos_over_7y_old".to_string()),
            _8y: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "utxos_over_8y_old".to_string()),
            _10y: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "utxos_over_10y_old".to_string()),
            _12y: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "utxos_over_12y_old".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Distribution_UtxoCohorts_Epoch {
    pub _0: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _1: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _2: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _3: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _4: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern,
}

impl MetricsTree_Distribution_UtxoCohorts_Epoch {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _0: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "epoch_0".to_string()),
            _1: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "epoch_1".to_string()),
            _2: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "epoch_2".to_string()),
            _3: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "epoch_3".to_string()),
            _4: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "epoch_4".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Distribution_UtxoCohorts_Class {
    pub _2009: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _2010: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _2011: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _2012: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _2013: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _2014: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _2015: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _2016: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _2017: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _2018: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _2019: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _2020: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _2021: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _2022: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _2023: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _2024: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _2025: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub _2026: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern,
}

impl MetricsTree_Distribution_UtxoCohorts_Class {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _2009: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "class_2009".to_string()),
            _2010: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "class_2010".to_string()),
            _2011: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "class_2011".to_string()),
            _2012: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "class_2012".to_string()),
            _2013: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "class_2013".to_string()),
            _2014: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "class_2014".to_string()),
            _2015: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "class_2015".to_string()),
            _2016: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "class_2016".to_string()),
            _2017: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "class_2017".to_string()),
            _2018: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "class_2018".to_string()),
            _2019: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "class_2019".to_string()),
            _2020: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "class_2020".to_string()),
            _2021: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "class_2021".to_string()),
            _2022: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "class_2022".to_string()),
            _2023: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "class_2023".to_string()),
            _2024: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "class_2024".to_string()),
            _2025: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "class_2025".to_string()),
            _2026: ActivityOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "class_2026".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Distribution_UtxoCohorts_GeAmount {
    pub _1sat: ActivityOutputsRealizedSupplyUnrealizedPattern,
    pub _10sats: ActivityOutputsRealizedSupplyUnrealizedPattern,
    pub _100sats: ActivityOutputsRealizedSupplyUnrealizedPattern,
    pub _1k_sats: ActivityOutputsRealizedSupplyUnrealizedPattern,
    pub _10k_sats: ActivityOutputsRealizedSupplyUnrealizedPattern,
    pub _100k_sats: ActivityOutputsRealizedSupplyUnrealizedPattern,
    pub _1m_sats: ActivityOutputsRealizedSupplyUnrealizedPattern,
    pub _10m_sats: ActivityOutputsRealizedSupplyUnrealizedPattern,
    pub _1btc: ActivityOutputsRealizedSupplyUnrealizedPattern,
    pub _10btc: ActivityOutputsRealizedSupplyUnrealizedPattern,
    pub _100btc: ActivityOutputsRealizedSupplyUnrealizedPattern,
    pub _1k_btc: ActivityOutputsRealizedSupplyUnrealizedPattern,
    pub _10k_btc: ActivityOutputsRealizedSupplyUnrealizedPattern,
}

impl MetricsTree_Distribution_UtxoCohorts_GeAmount {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _1sat: ActivityOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "utxos_over_1sat".to_string()),
            _10sats: ActivityOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "utxos_over_10sats".to_string()),
            _100sats: ActivityOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "utxos_over_100sats".to_string()),
            _1k_sats: ActivityOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "utxos_over_1k_sats".to_string()),
            _10k_sats: ActivityOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "utxos_over_10k_sats".to_string()),
            _100k_sats: ActivityOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "utxos_over_100k_sats".to_string()),
            _1m_sats: ActivityOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "utxos_over_1m_sats".to_string()),
            _10m_sats: ActivityOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "utxos_over_10m_sats".to_string()),
            _1btc: ActivityOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "utxos_over_1btc".to_string()),
            _10btc: ActivityOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "utxos_over_10btc".to_string()),
            _100btc: ActivityOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "utxos_over_100btc".to_string()),
            _1k_btc: ActivityOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "utxos_over_1k_btc".to_string()),
            _10k_btc: ActivityOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "utxos_over_10k_btc".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Distribution_UtxoCohorts_AmountRange {
    pub _0sats: ActivityOutputsRealizedSupplyUnrealizedPattern,
    pub _1sat_to_10sats: ActivityOutputsRealizedSupplyUnrealizedPattern,
    pub _10sats_to_100sats: ActivityOutputsRealizedSupplyUnrealizedPattern,
    pub _100sats_to_1k_sats: ActivityOutputsRealizedSupplyUnrealizedPattern,
    pub _1k_sats_to_10k_sats: ActivityOutputsRealizedSupplyUnrealizedPattern,
    pub _10k_sats_to_100k_sats: ActivityOutputsRealizedSupplyUnrealizedPattern,
    pub _100k_sats_to_1m_sats: ActivityOutputsRealizedSupplyUnrealizedPattern,
    pub _1m_sats_to_10m_sats: ActivityOutputsRealizedSupplyUnrealizedPattern,
    pub _10m_sats_to_1btc: ActivityOutputsRealizedSupplyUnrealizedPattern,
    pub _1btc_to_10btc: ActivityOutputsRealizedSupplyUnrealizedPattern,
    pub _10btc_to_100btc: ActivityOutputsRealizedSupplyUnrealizedPattern,
    pub _100btc_to_1k_btc: ActivityOutputsRealizedSupplyUnrealizedPattern,
    pub _1k_btc_to_10k_btc: ActivityOutputsRealizedSupplyUnrealizedPattern,
    pub _10k_btc_to_100k_btc: ActivityOutputsRealizedSupplyUnrealizedPattern,
    pub _100k_btc_or_more: ActivityOutputsRealizedSupplyUnrealizedPattern,
}

impl MetricsTree_Distribution_UtxoCohorts_AmountRange {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _0sats: ActivityOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "utxos_with_0sats".to_string()),
            _1sat_to_10sats: ActivityOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "utxos_above_1sat_under_10sats".to_string()),
            _10sats_to_100sats: ActivityOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "utxos_above_10sats_under_100sats".to_string()),
            _100sats_to_1k_sats: ActivityOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "utxos_above_100sats_under_1k_sats".to_string()),
            _1k_sats_to_10k_sats: ActivityOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "utxos_above_1k_sats_under_10k_sats".to_string()),
            _10k_sats_to_100k_sats: ActivityOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "utxos_above_10k_sats_under_100k_sats".to_string()),
            _100k_sats_to_1m_sats: ActivityOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "utxos_above_100k_sats_under_1m_sats".to_string()),
            _1m_sats_to_10m_sats: ActivityOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "utxos_above_1m_sats_under_10m_sats".to_string()),
            _10m_sats_to_1btc: ActivityOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "utxos_above_10m_sats_under_1btc".to_string()),
            _1btc_to_10btc: ActivityOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "utxos_above_1btc_under_10btc".to_string()),
            _10btc_to_100btc: ActivityOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "utxos_above_10btc_under_100btc".to_string()),
            _100btc_to_1k_btc: ActivityOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "utxos_above_100btc_under_1k_btc".to_string()),
            _1k_btc_to_10k_btc: ActivityOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "utxos_above_1k_btc_under_10k_btc".to_string()),
            _10k_btc_to_100k_btc: ActivityOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "utxos_above_10k_btc_under_100k_btc".to_string()),
            _100k_btc_or_more: ActivityOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "utxos_above_100k_btc".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Distribution_UtxoCohorts_LtAmount {
    pub _10sats: ActivityOutputsRealizedSupplyUnrealizedPattern,
    pub _100sats: ActivityOutputsRealizedSupplyUnrealizedPattern,
    pub _1k_sats: ActivityOutputsRealizedSupplyUnrealizedPattern,
    pub _10k_sats: ActivityOutputsRealizedSupplyUnrealizedPattern,
    pub _100k_sats: ActivityOutputsRealizedSupplyUnrealizedPattern,
    pub _1m_sats: ActivityOutputsRealizedSupplyUnrealizedPattern,
    pub _10m_sats: ActivityOutputsRealizedSupplyUnrealizedPattern,
    pub _1btc: ActivityOutputsRealizedSupplyUnrealizedPattern,
    pub _10btc: ActivityOutputsRealizedSupplyUnrealizedPattern,
    pub _100btc: ActivityOutputsRealizedSupplyUnrealizedPattern,
    pub _1k_btc: ActivityOutputsRealizedSupplyUnrealizedPattern,
    pub _10k_btc: ActivityOutputsRealizedSupplyUnrealizedPattern,
    pub _100k_btc: ActivityOutputsRealizedSupplyUnrealizedPattern,
}

impl MetricsTree_Distribution_UtxoCohorts_LtAmount {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _10sats: ActivityOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "utxos_under_10sats".to_string()),
            _100sats: ActivityOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "utxos_under_100sats".to_string()),
            _1k_sats: ActivityOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "utxos_under_1k_sats".to_string()),
            _10k_sats: ActivityOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "utxos_under_10k_sats".to_string()),
            _100k_sats: ActivityOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "utxos_under_100k_sats".to_string()),
            _1m_sats: ActivityOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "utxos_under_1m_sats".to_string()),
            _10m_sats: ActivityOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "utxos_under_10m_sats".to_string()),
            _1btc: ActivityOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "utxos_under_1btc".to_string()),
            _10btc: ActivityOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "utxos_under_10btc".to_string()),
            _100btc: ActivityOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "utxos_under_100btc".to_string()),
            _1k_btc: ActivityOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "utxos_under_1k_btc".to_string()),
            _10k_btc: ActivityOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "utxos_under_10k_btc".to_string()),
            _100k_btc: ActivityOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "utxos_under_100k_btc".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Distribution_UtxoCohorts_Type {
    pub p2pk65: ActivityOutputsRealizedSupplyUnrealizedPattern,
    pub p2pk33: ActivityOutputsRealizedSupplyUnrealizedPattern,
    pub p2pkh: ActivityOutputsRealizedSupplyUnrealizedPattern,
    pub p2ms: ActivityOutputsRealizedSupplyUnrealizedPattern,
    pub p2sh: ActivityOutputsRealizedSupplyUnrealizedPattern,
    pub p2wpkh: ActivityOutputsRealizedSupplyUnrealizedPattern,
    pub p2wsh: ActivityOutputsRealizedSupplyUnrealizedPattern,
    pub p2tr: ActivityOutputsRealizedSupplyUnrealizedPattern,
    pub p2a: ActivityOutputsRealizedSupplyUnrealizedPattern,
    pub unknown: ActivityOutputsRealizedSupplyUnrealizedPattern,
    pub empty: ActivityOutputsRealizedSupplyUnrealizedPattern,
}

impl MetricsTree_Distribution_UtxoCohorts_Type {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            p2pk65: ActivityOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "p2pk65".to_string()),
            p2pk33: ActivityOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "p2pk33".to_string()),
            p2pkh: ActivityOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "p2pkh".to_string()),
            p2ms: ActivityOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "p2ms".to_string()),
            p2sh: ActivityOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "p2sh".to_string()),
            p2wpkh: ActivityOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "p2wpkh".to_string()),
            p2wsh: ActivityOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "p2wsh".to_string()),
            p2tr: ActivityOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "p2tr".to_string()),
            p2a: ActivityOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "p2a".to_string()),
            unknown: ActivityOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "unknown_outputs".to_string()),
            empty: ActivityOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "empty_outputs".to_string()),
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
    pub _1sat: ActivityAddrOutputsRealizedSupplyUnrealizedPattern,
    pub _10sats: ActivityAddrOutputsRealizedSupplyUnrealizedPattern,
    pub _100sats: ActivityAddrOutputsRealizedSupplyUnrealizedPattern,
    pub _1k_sats: ActivityAddrOutputsRealizedSupplyUnrealizedPattern,
    pub _10k_sats: ActivityAddrOutputsRealizedSupplyUnrealizedPattern,
    pub _100k_sats: ActivityAddrOutputsRealizedSupplyUnrealizedPattern,
    pub _1m_sats: ActivityAddrOutputsRealizedSupplyUnrealizedPattern,
    pub _10m_sats: ActivityAddrOutputsRealizedSupplyUnrealizedPattern,
    pub _1btc: ActivityAddrOutputsRealizedSupplyUnrealizedPattern,
    pub _10btc: ActivityAddrOutputsRealizedSupplyUnrealizedPattern,
    pub _100btc: ActivityAddrOutputsRealizedSupplyUnrealizedPattern,
    pub _1k_btc: ActivityAddrOutputsRealizedSupplyUnrealizedPattern,
    pub _10k_btc: ActivityAddrOutputsRealizedSupplyUnrealizedPattern,
}

impl MetricsTree_Distribution_AddressCohorts_GeAmount {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _1sat: ActivityAddrOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "addrs_over_1sat".to_string()),
            _10sats: ActivityAddrOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "addrs_over_10sats".to_string()),
            _100sats: ActivityAddrOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "addrs_over_100sats".to_string()),
            _1k_sats: ActivityAddrOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "addrs_over_1k_sats".to_string()),
            _10k_sats: ActivityAddrOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "addrs_over_10k_sats".to_string()),
            _100k_sats: ActivityAddrOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "addrs_over_100k_sats".to_string()),
            _1m_sats: ActivityAddrOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "addrs_over_1m_sats".to_string()),
            _10m_sats: ActivityAddrOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "addrs_over_10m_sats".to_string()),
            _1btc: ActivityAddrOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "addrs_over_1btc".to_string()),
            _10btc: ActivityAddrOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "addrs_over_10btc".to_string()),
            _100btc: ActivityAddrOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "addrs_over_100btc".to_string()),
            _1k_btc: ActivityAddrOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "addrs_over_1k_btc".to_string()),
            _10k_btc: ActivityAddrOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "addrs_over_10k_btc".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Distribution_AddressCohorts_AmountRange {
    pub _0sats: ActivityAddrOutputsRealizedSupplyUnrealizedPattern,
    pub _1sat_to_10sats: ActivityAddrOutputsRealizedSupplyUnrealizedPattern,
    pub _10sats_to_100sats: ActivityAddrOutputsRealizedSupplyUnrealizedPattern,
    pub _100sats_to_1k_sats: ActivityAddrOutputsRealizedSupplyUnrealizedPattern,
    pub _1k_sats_to_10k_sats: ActivityAddrOutputsRealizedSupplyUnrealizedPattern,
    pub _10k_sats_to_100k_sats: ActivityAddrOutputsRealizedSupplyUnrealizedPattern,
    pub _100k_sats_to_1m_sats: ActivityAddrOutputsRealizedSupplyUnrealizedPattern,
    pub _1m_sats_to_10m_sats: ActivityAddrOutputsRealizedSupplyUnrealizedPattern,
    pub _10m_sats_to_1btc: ActivityAddrOutputsRealizedSupplyUnrealizedPattern,
    pub _1btc_to_10btc: ActivityAddrOutputsRealizedSupplyUnrealizedPattern,
    pub _10btc_to_100btc: ActivityAddrOutputsRealizedSupplyUnrealizedPattern,
    pub _100btc_to_1k_btc: ActivityAddrOutputsRealizedSupplyUnrealizedPattern,
    pub _1k_btc_to_10k_btc: ActivityAddrOutputsRealizedSupplyUnrealizedPattern,
    pub _10k_btc_to_100k_btc: ActivityAddrOutputsRealizedSupplyUnrealizedPattern,
    pub _100k_btc_or_more: ActivityAddrOutputsRealizedSupplyUnrealizedPattern,
}

impl MetricsTree_Distribution_AddressCohorts_AmountRange {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _0sats: ActivityAddrOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "addrs_with_0sats".to_string()),
            _1sat_to_10sats: ActivityAddrOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "addrs_above_1sat_under_10sats".to_string()),
            _10sats_to_100sats: ActivityAddrOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "addrs_above_10sats_under_100sats".to_string()),
            _100sats_to_1k_sats: ActivityAddrOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "addrs_above_100sats_under_1k_sats".to_string()),
            _1k_sats_to_10k_sats: ActivityAddrOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "addrs_above_1k_sats_under_10k_sats".to_string()),
            _10k_sats_to_100k_sats: ActivityAddrOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "addrs_above_10k_sats_under_100k_sats".to_string()),
            _100k_sats_to_1m_sats: ActivityAddrOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "addrs_above_100k_sats_under_1m_sats".to_string()),
            _1m_sats_to_10m_sats: ActivityAddrOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "addrs_above_1m_sats_under_10m_sats".to_string()),
            _10m_sats_to_1btc: ActivityAddrOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "addrs_above_10m_sats_under_1btc".to_string()),
            _1btc_to_10btc: ActivityAddrOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "addrs_above_1btc_under_10btc".to_string()),
            _10btc_to_100btc: ActivityAddrOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "addrs_above_10btc_under_100btc".to_string()),
            _100btc_to_1k_btc: ActivityAddrOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "addrs_above_100btc_under_1k_btc".to_string()),
            _1k_btc_to_10k_btc: ActivityAddrOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "addrs_above_1k_btc_under_10k_btc".to_string()),
            _10k_btc_to_100k_btc: ActivityAddrOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "addrs_above_10k_btc_under_100k_btc".to_string()),
            _100k_btc_or_more: ActivityAddrOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "addrs_above_100k_btc".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Distribution_AddressCohorts_LtAmount {
    pub _10sats: ActivityAddrOutputsRealizedSupplyUnrealizedPattern,
    pub _100sats: ActivityAddrOutputsRealizedSupplyUnrealizedPattern,
    pub _1k_sats: ActivityAddrOutputsRealizedSupplyUnrealizedPattern,
    pub _10k_sats: ActivityAddrOutputsRealizedSupplyUnrealizedPattern,
    pub _100k_sats: ActivityAddrOutputsRealizedSupplyUnrealizedPattern,
    pub _1m_sats: ActivityAddrOutputsRealizedSupplyUnrealizedPattern,
    pub _10m_sats: ActivityAddrOutputsRealizedSupplyUnrealizedPattern,
    pub _1btc: ActivityAddrOutputsRealizedSupplyUnrealizedPattern,
    pub _10btc: ActivityAddrOutputsRealizedSupplyUnrealizedPattern,
    pub _100btc: ActivityAddrOutputsRealizedSupplyUnrealizedPattern,
    pub _1k_btc: ActivityAddrOutputsRealizedSupplyUnrealizedPattern,
    pub _10k_btc: ActivityAddrOutputsRealizedSupplyUnrealizedPattern,
    pub _100k_btc: ActivityAddrOutputsRealizedSupplyUnrealizedPattern,
}

impl MetricsTree_Distribution_AddressCohorts_LtAmount {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _10sats: ActivityAddrOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "addrs_under_10sats".to_string()),
            _100sats: ActivityAddrOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "addrs_under_100sats".to_string()),
            _1k_sats: ActivityAddrOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "addrs_under_1k_sats".to_string()),
            _10k_sats: ActivityAddrOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "addrs_under_10k_sats".to_string()),
            _100k_sats: ActivityAddrOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "addrs_under_100k_sats".to_string()),
            _1m_sats: ActivityAddrOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "addrs_under_1m_sats".to_string()),
            _10m_sats: ActivityAddrOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "addrs_under_10m_sats".to_string()),
            _1btc: ActivityAddrOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "addrs_under_1btc".to_string()),
            _10btc: ActivityAddrOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "addrs_under_10btc".to_string()),
            _100btc: ActivityAddrOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "addrs_under_100btc".to_string()),
            _1k_btc: ActivityAddrOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "addrs_under_1k_btc".to_string()),
            _10k_btc: ActivityAddrOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "addrs_under_10k_btc".to_string()),
            _100k_btc: ActivityAddrOutputsRealizedSupplyUnrealizedPattern::new(client.clone(), "addrs_under_100k_btc".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Distribution_AddressActivity {
    pub all: BothReactivatedReceivingSendingPattern,
    pub p2pk65: BothReactivatedReceivingSendingPattern,
    pub p2pk33: BothReactivatedReceivingSendingPattern,
    pub p2pkh: BothReactivatedReceivingSendingPattern,
    pub p2sh: BothReactivatedReceivingSendingPattern,
    pub p2wpkh: BothReactivatedReceivingSendingPattern,
    pub p2wsh: BothReactivatedReceivingSendingPattern,
    pub p2tr: BothReactivatedReceivingSendingPattern,
    pub p2a: BothReactivatedReceivingSendingPattern,
}

impl MetricsTree_Distribution_AddressActivity {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            all: BothReactivatedReceivingSendingPattern::new(client.clone(), "address_activity".to_string()),
            p2pk65: BothReactivatedReceivingSendingPattern::new(client.clone(), "p2pk65_address_activity".to_string()),
            p2pk33: BothReactivatedReceivingSendingPattern::new(client.clone(), "p2pk33_address_activity".to_string()),
            p2pkh: BothReactivatedReceivingSendingPattern::new(client.clone(), "p2pkh_address_activity".to_string()),
            p2sh: BothReactivatedReceivingSendingPattern::new(client.clone(), "p2sh_address_activity".to_string()),
            p2wpkh: BothReactivatedReceivingSendingPattern::new(client.clone(), "p2wpkh_address_activity".to_string()),
            p2wsh: BothReactivatedReceivingSendingPattern::new(client.clone(), "p2wsh_address_activity".to_string()),
            p2tr: BothReactivatedReceivingSendingPattern::new(client.clone(), "p2tr_address_activity".to_string()),
            p2a: BothReactivatedReceivingSendingPattern::new(client.clone(), "p2a_address_activity".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Distribution_NewAddrCount {
    pub all: AverageCumulativeHeightMaxMedianMinPct10Pct25Pct75Pct90SumPattern,
    pub p2pk65: AverageCumulativeHeightMaxMedianMinPct10Pct25Pct75Pct90SumPattern,
    pub p2pk33: AverageCumulativeHeightMaxMedianMinPct10Pct25Pct75Pct90SumPattern,
    pub p2pkh: AverageCumulativeHeightMaxMedianMinPct10Pct25Pct75Pct90SumPattern,
    pub p2sh: AverageCumulativeHeightMaxMedianMinPct10Pct25Pct75Pct90SumPattern,
    pub p2wpkh: AverageCumulativeHeightMaxMedianMinPct10Pct25Pct75Pct90SumPattern,
    pub p2wsh: AverageCumulativeHeightMaxMedianMinPct10Pct25Pct75Pct90SumPattern,
    pub p2tr: AverageCumulativeHeightMaxMedianMinPct10Pct25Pct75Pct90SumPattern,
    pub p2a: AverageCumulativeHeightMaxMedianMinPct10Pct25Pct75Pct90SumPattern,
}

impl MetricsTree_Distribution_NewAddrCount {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            all: AverageCumulativeHeightMaxMedianMinPct10Pct25Pct75Pct90SumPattern::new(client.clone(), "new_addr_count".to_string()),
            p2pk65: AverageCumulativeHeightMaxMedianMinPct10Pct25Pct75Pct90SumPattern::new(client.clone(), "p2pk65_new_addr_count".to_string()),
            p2pk33: AverageCumulativeHeightMaxMedianMinPct10Pct25Pct75Pct90SumPattern::new(client.clone(), "p2pk33_new_addr_count".to_string()),
            p2pkh: AverageCumulativeHeightMaxMedianMinPct10Pct25Pct75Pct90SumPattern::new(client.clone(), "p2pkh_new_addr_count".to_string()),
            p2sh: AverageCumulativeHeightMaxMedianMinPct10Pct25Pct75Pct90SumPattern::new(client.clone(), "p2sh_new_addr_count".to_string()),
            p2wpkh: AverageCumulativeHeightMaxMedianMinPct10Pct25Pct75Pct90SumPattern::new(client.clone(), "p2wpkh_new_addr_count".to_string()),
            p2wsh: AverageCumulativeHeightMaxMedianMinPct10Pct25Pct75Pct90SumPattern::new(client.clone(), "p2wsh_new_addr_count".to_string()),
            p2tr: AverageCumulativeHeightMaxMedianMinPct10Pct25Pct75Pct90SumPattern::new(client.clone(), "p2tr_new_addr_count".to_string()),
            p2a: AverageCumulativeHeightMaxMedianMinPct10Pct25Pct75Pct90SumPattern::new(client.clone(), "p2a_new_addr_count".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Distribution_Delta {
    pub all: ChangeRatePattern,
    pub p2pk65: ChangeRatePattern,
    pub p2pk33: ChangeRatePattern,
    pub p2pkh: ChangeRatePattern,
    pub p2sh: ChangeRatePattern,
    pub p2wpkh: ChangeRatePattern,
    pub p2wsh: ChangeRatePattern,
    pub p2tr: ChangeRatePattern,
    pub p2a: ChangeRatePattern,
}

impl MetricsTree_Distribution_Delta {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            all: ChangeRatePattern::new(client.clone(), "addr_count".to_string()),
            p2pk65: ChangeRatePattern::new(client.clone(), "p2pk65_addr_count".to_string()),
            p2pk33: ChangeRatePattern::new(client.clone(), "p2pk33_addr_count".to_string()),
            p2pkh: ChangeRatePattern::new(client.clone(), "p2pkh_addr_count".to_string()),
            p2sh: ChangeRatePattern::new(client.clone(), "p2sh_addr_count".to_string()),
            p2wpkh: ChangeRatePattern::new(client.clone(), "p2wpkh_addr_count".to_string()),
            p2wsh: ChangeRatePattern::new(client.clone(), "p2wsh_addr_count".to_string()),
            p2tr: ChangeRatePattern::new(client.clone(), "p2tr_addr_count".to_string()),
            p2a: ChangeRatePattern::new(client.clone(), "p2a_addr_count".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Supply {
    pub circulating: BtcCentsSatsUsdPattern,
    pub burned: MetricsTree_Supply_Burned,
    pub inflation_rate: BpsPercentRatioPattern,
    pub velocity: MetricsTree_Supply_Velocity,
    pub market_cap: MetricPattern1<Dollars>,
    pub market_cap_growth_rate: BpsPercentRatioPattern,
    pub realized_cap_growth_rate: BpsPercentRatioPattern,
    pub market_minus_realized_cap_growth_rate: MetricPattern1<BasisPointsSigned32>,
}

impl MetricsTree_Supply {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            circulating: BtcCentsSatsUsdPattern::new(client.clone(), "circulating_supply".to_string()),
            burned: MetricsTree_Supply_Burned::new(client.clone(), format!("{base_path}_burned")),
            inflation_rate: BpsPercentRatioPattern::new(client.clone(), "inflation_rate".to_string()),
            velocity: MetricsTree_Supply_Velocity::new(client.clone(), format!("{base_path}_velocity")),
            market_cap: MetricPattern1::new(client.clone(), "market_cap".to_string()),
            market_cap_growth_rate: BpsPercentRatioPattern::new(client.clone(), "market_cap_growth_rate".to_string()),
            realized_cap_growth_rate: BpsPercentRatioPattern::new(client.clone(), "realized_cap_growth_rate".to_string()),
            market_minus_realized_cap_growth_rate: MetricPattern1::new(client.clone(), "market_minus_realized_cap_growth_rate".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Supply_Burned {
    pub opreturn: BaseCumulativeSumPattern,
    pub unspendable: BaseCumulativeSumPattern,
}

impl MetricsTree_Supply_Burned {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            opreturn: BaseCumulativeSumPattern::new(client.clone(), "opreturn_supply".to_string()),
            unspendable: BaseCumulativeSumPattern::new(client.clone(), "unspendable_supply".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Supply_Velocity {
    pub btc: MetricPattern1<StoredF64>,
    pub usd: MetricPattern1<StoredF64>,
}

impl MetricsTree_Supply_Velocity {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            btc: MetricPattern1::new(client.clone(), "velocity_btc".to_string()),
            usd: MetricPattern1::new(client.clone(), "velocity_usd".to_string()),
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
    pub const VERSION: &'static str = "v0.1.9";

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

    /// Create a dynamic date-based metric endpoint builder.
    ///
    /// Returns `Err` if the index is not date-based.
    pub fn date_metric(&self, metric: impl Into<Metric>, index: Index) -> Result<DateMetricEndpointBuilder<serde_json::Value>> {
        if !index.is_date_based() {
            return Err(BrkError { message: format!("{} is not a date-based index", index.name()) });
        }
        Ok(DateMetricEndpointBuilder::new(
            self.base.clone(),
            Arc::from(metric.into().as_str()),
            index,
        ))
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

    /// Live BTC/USD price
    ///
    /// Returns the current BTC/USD price in dollars, derived from on-chain round-dollar output patterns in the last 12 blocks plus mempool.
    ///
    /// Endpoint: `GET /api/mempool/price`
    pub fn get_live_price(&self) -> Result<Dollars> {
        self.base.get_json(&format!("/api/mempool/price"))
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
    /// Returns the list of indexes supported by the specified metric. For example, `realized_price` might be available on day1, week1, and month1.
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
        let path = format!("/api/metric/{metric}/{}{}", index.name(), query_str);
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
