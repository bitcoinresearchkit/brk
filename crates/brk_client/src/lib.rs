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
const _I1: &[Index] = &[Index::Minute1, Index::Minute5, Index::Minute10, Index::Minute30, Index::Hour1, Index::Hour4, Index::Hour12, Index::Day1, Index::Day3, Index::Week1, Index::Month1, Index::Month3, Index::Month6, Index::Year1, Index::Year10, Index::HalvingEpoch, Index::DifficultyEpoch, Index::Height];
const _I2: &[Index] = &[Index::Minute1, Index::Minute5, Index::Minute10, Index::Minute30, Index::Hour1, Index::Hour4, Index::Hour12, Index::Day1, Index::Day3, Index::Week1, Index::Month1, Index::Month3, Index::Month6, Index::Year1, Index::Year10, Index::HalvingEpoch, Index::DifficultyEpoch];
const _I3: &[Index] = &[Index::Minute1];
const _I4: &[Index] = &[Index::Minute5];
const _I5: &[Index] = &[Index::Minute10];
const _I6: &[Index] = &[Index::Minute30];
const _I7: &[Index] = &[Index::Hour1];
const _I8: &[Index] = &[Index::Hour4];
const _I9: &[Index] = &[Index::Hour12];
const _I10: &[Index] = &[Index::Day1];
const _I11: &[Index] = &[Index::Day3];
const _I12: &[Index] = &[Index::Week1];
const _I13: &[Index] = &[Index::Month1];
const _I14: &[Index] = &[Index::Month3];
const _I15: &[Index] = &[Index::Month6];
const _I16: &[Index] = &[Index::Year1];
const _I17: &[Index] = &[Index::Year10];
const _I18: &[Index] = &[Index::HalvingEpoch];
const _I19: &[Index] = &[Index::DifficultyEpoch];
const _I20: &[Index] = &[Index::Height];
const _I21: &[Index] = &[Index::TxIndex];
const _I22: &[Index] = &[Index::TxInIndex];
const _I23: &[Index] = &[Index::TxOutIndex];
const _I24: &[Index] = &[Index::EmptyOutputIndex];
const _I25: &[Index] = &[Index::OpReturnIndex];
const _I26: &[Index] = &[Index::P2AAddressIndex];
const _I27: &[Index] = &[Index::P2MSOutputIndex];
const _I28: &[Index] = &[Index::P2PK33AddressIndex];
const _I29: &[Index] = &[Index::P2PK65AddressIndex];
const _I30: &[Index] = &[Index::P2PKHAddressIndex];
const _I31: &[Index] = &[Index::P2SHAddressIndex];
const _I32: &[Index] = &[Index::P2TRAddressIndex];
const _I33: &[Index] = &[Index::P2WPKHAddressIndex];
const _I34: &[Index] = &[Index::P2WSHAddressIndex];
const _I35: &[Index] = &[Index::UnknownOutputIndex];
const _I36: &[Index] = &[Index::FundedAddressIndex];
const _I37: &[Index] = &[Index::EmptyAddressIndex];

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
    pub fn minute1(&self) -> DateMetricEndpointBuilder<T> { _dep(&self.client, &self.name, Index::Minute1) }
    pub fn minute5(&self) -> DateMetricEndpointBuilder<T> { _dep(&self.client, &self.name, Index::Minute5) }
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
    pub fn minute1(&self) -> DateMetricEndpointBuilder<T> { _dep(&self.client, &self.name, Index::Minute1) }
    pub fn minute5(&self) -> DateMetricEndpointBuilder<T> { _dep(&self.client, &self.name, Index::Minute5) }
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
    pub fn minute1(&self) -> DateMetricEndpointBuilder<T> { _dep(&self.client, &self.name, Index::Minute1) }
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
    pub fn minute5(&self) -> DateMetricEndpointBuilder<T> { _dep(&self.client, &self.name, Index::Minute5) }
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
    pub fn minute10(&self) -> DateMetricEndpointBuilder<T> { _dep(&self.client, &self.name, Index::Minute10) }
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
    pub fn minute30(&self) -> DateMetricEndpointBuilder<T> { _dep(&self.client, &self.name, Index::Minute30) }
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
    pub fn hour1(&self) -> DateMetricEndpointBuilder<T> { _dep(&self.client, &self.name, Index::Hour1) }
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
    pub fn hour4(&self) -> DateMetricEndpointBuilder<T> { _dep(&self.client, &self.name, Index::Hour4) }
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
    pub fn hour12(&self) -> DateMetricEndpointBuilder<T> { _dep(&self.client, &self.name, Index::Hour12) }
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
    pub fn day1(&self) -> DateMetricEndpointBuilder<T> { _dep(&self.client, &self.name, Index::Day1) }
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
    pub fn day3(&self) -> DateMetricEndpointBuilder<T> { _dep(&self.client, &self.name, Index::Day3) }
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
    pub fn week1(&self) -> DateMetricEndpointBuilder<T> { _dep(&self.client, &self.name, Index::Week1) }
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
    pub fn month1(&self) -> DateMetricEndpointBuilder<T> { _dep(&self.client, &self.name, Index::Month1) }
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
    pub fn month3(&self) -> DateMetricEndpointBuilder<T> { _dep(&self.client, &self.name, Index::Month3) }
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
    pub fn month6(&self) -> DateMetricEndpointBuilder<T> { _dep(&self.client, &self.name, Index::Month6) }
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
    pub fn year1(&self) -> DateMetricEndpointBuilder<T> { _dep(&self.client, &self.name, Index::Year1) }
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
    pub fn year10(&self) -> DateMetricEndpointBuilder<T> { _dep(&self.client, &self.name, Index::Year10) }
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
    pub fn halvingepoch(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::HalvingEpoch) }
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
    pub fn difficultyepoch(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::DifficultyEpoch) }
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
    pub fn height(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::Height) }
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
    pub fn txindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::TxIndex) }
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
    pub fn txinindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::TxInIndex) }
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
    pub fn txoutindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::TxOutIndex) }
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
    pub fn emptyoutputindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::EmptyOutputIndex) }
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
    pub fn opreturnindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::OpReturnIndex) }
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
    pub fn p2aaddressindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::P2AAddressIndex) }
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
    pub fn p2msoutputindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::P2MSOutputIndex) }
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
    pub fn p2pk33addressindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::P2PK33AddressIndex) }
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
    pub fn p2pk65addressindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::P2PK65AddressIndex) }
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
    pub fn p2pkhaddressindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::P2PKHAddressIndex) }
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
    pub fn p2shaddressindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::P2SHAddressIndex) }
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
    pub fn p2traddressindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::P2TRAddressIndex) }
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
    pub fn p2wpkhaddressindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::P2WPKHAddressIndex) }
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
    pub fn p2wshaddressindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::P2WSHAddressIndex) }
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
    pub fn unknownoutputindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::UnknownOutputIndex) }
}

pub struct MetricPattern35<T> { name: Arc<str>, pub by: MetricPattern35By<T> }
impl<T: DeserializeOwned> MetricPattern35<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self { let name: Arc<str> = name.into(); Self { name: name.clone(), by: MetricPattern35By { client, name, _marker: std::marker::PhantomData } } }
    pub fn name(&self) -> &str { &self.name }
}

impl<T> AnyMetricPattern for MetricPattern35<T> { fn name(&self) -> &str { &self.name } fn indexes(&self) -> &'static [Index] { _I35 } }
impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern35<T> { fn get(&self, index: Index) -> Option<MetricEndpointBuilder<T>> { _I35.contains(&index).then(|| _ep(&self.by.client, &self.by.name, index)) } }

pub struct MetricPattern36By<T> { client: Arc<BrkClientBase>, name: Arc<str>, _marker: std::marker::PhantomData<T> }
impl<T: DeserializeOwned> MetricPattern36By<T> {
    pub fn fundedaddressindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::FundedAddressIndex) }
}

pub struct MetricPattern36<T> { name: Arc<str>, pub by: MetricPattern36By<T> }
impl<T: DeserializeOwned> MetricPattern36<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self { let name: Arc<str> = name.into(); Self { name: name.clone(), by: MetricPattern36By { client, name, _marker: std::marker::PhantomData } } }
    pub fn name(&self) -> &str { &self.name }
}

impl<T> AnyMetricPattern for MetricPattern36<T> { fn name(&self) -> &str { &self.name } fn indexes(&self) -> &'static [Index] { _I36 } }
impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern36<T> { fn get(&self, index: Index) -> Option<MetricEndpointBuilder<T>> { _I36.contains(&index).then(|| _ep(&self.by.client, &self.by.name, index)) } }

pub struct MetricPattern37By<T> { client: Arc<BrkClientBase>, name: Arc<str>, _marker: std::marker::PhantomData<T> }
impl<T: DeserializeOwned> MetricPattern37By<T> {
    pub fn emptyaddressindex(&self) -> MetricEndpointBuilder<T> { _ep(&self.client, &self.name, Index::EmptyAddressIndex) }
}

pub struct MetricPattern37<T> { name: Arc<str>, pub by: MetricPattern37By<T> }
impl<T: DeserializeOwned> MetricPattern37<T> {
    pub fn new(client: Arc<BrkClientBase>, name: String) -> Self { let name: Arc<str> = name.into(); Self { name: name.clone(), by: MetricPattern37By { client, name, _marker: std::marker::PhantomData } } }
    pub fn name(&self) -> &str { &self.name }
}

impl<T> AnyMetricPattern for MetricPattern37<T> { fn name(&self) -> &str { &self.name } fn indexes(&self) -> &'static [Index] { _I37 } }
impl<T: DeserializeOwned> MetricPattern<T> for MetricPattern37<T> { fn get(&self, index: Index) -> Option<MetricEndpointBuilder<T>> { _I37.contains(&index).then(|| _ep(&self.by.client, &self.by.name, index)) } }

// Reusable pattern structs

/// Pattern struct for repeated tree structure.
pub struct AdjustedCapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern {
    pub adjusted_sopr: MetricPattern1<StoredF64>,
    pub adjusted_sopr_1y: MetricPattern1<StoredF64>,
    pub adjusted_sopr_24h: MetricPattern1<StoredF64>,
    pub adjusted_sopr_24h_30d_ema: MetricPattern1<StoredF64>,
    pub adjusted_sopr_24h_7d_ema: MetricPattern1<StoredF64>,
    pub adjusted_sopr_30d: MetricPattern1<StoredF64>,
    pub adjusted_sopr_30d_ema: MetricPattern1<StoredF64>,
    pub adjusted_sopr_7d: MetricPattern1<StoredF64>,
    pub adjusted_sopr_7d_ema: MetricPattern1<StoredF64>,
    pub adjusted_value_created: MetricPattern1<Dollars>,
    pub adjusted_value_created_1y: MetricPattern1<Dollars>,
    pub adjusted_value_created_24h: MetricPattern1<Dollars>,
    pub adjusted_value_created_30d: MetricPattern1<Dollars>,
    pub adjusted_value_created_7d: MetricPattern1<Dollars>,
    pub adjusted_value_destroyed: MetricPattern1<Dollars>,
    pub adjusted_value_destroyed_1y: MetricPattern1<Dollars>,
    pub adjusted_value_destroyed_24h: MetricPattern1<Dollars>,
    pub adjusted_value_destroyed_30d: MetricPattern1<Dollars>,
    pub adjusted_value_destroyed_7d: MetricPattern1<Dollars>,
    pub cap_raw: MetricPattern20<CentsSats>,
    pub capitulation_flow: MetricPattern1<Dollars>,
    pub investor_cap_raw: MetricPattern20<CentsSquaredSats>,
    pub investor_price: SatsUsdPattern,
    pub investor_price_cents: MetricPattern1<Cents>,
    pub investor_price_extra: RatioPattern2,
    pub investor_price_ratio_ext: RatioPattern3,
    pub loss_value_created: MetricPattern1<Dollars>,
    pub loss_value_destroyed: MetricPattern1<Dollars>,
    pub lower_price_band: SatsUsdPattern,
    pub mvrv: MetricPattern1<StoredF32>,
    pub neg_realized_loss: MetricPattern1<Dollars>,
    pub net_realized_pnl: CumulativeHeightPattern,
    pub net_realized_pnl_7d_ema: MetricPattern1<Dollars>,
    pub net_realized_pnl_cumulative_30d_delta: MetricPattern1<Dollars>,
    pub net_realized_pnl_cumulative_30d_delta_rel_to_market_cap: MetricPattern1<StoredF32>,
    pub net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap: MetricPattern1<StoredF32>,
    pub net_realized_pnl_rel_to_realized_cap: MetricPattern1<StoredF32>,
    pub peak_regret: CumulativeHeightPattern,
    pub peak_regret_rel_to_realized_cap: MetricPattern1<StoredF32>,
    pub profit_flow: MetricPattern1<Dollars>,
    pub profit_value_created: MetricPattern1<Dollars>,
    pub profit_value_destroyed: MetricPattern1<Dollars>,
    pub realized_cap: MetricPattern1<Dollars>,
    pub realized_cap_30d_delta: MetricPattern1<Dollars>,
    pub realized_cap_cents: MetricPattern1<Cents>,
    pub realized_cap_rel_to_own_market_cap: MetricPattern1<StoredF32>,
    pub realized_loss: CumulativeHeightPattern,
    pub realized_loss_1y: MetricPattern1<Dollars>,
    pub realized_loss_24h: MetricPattern1<Dollars>,
    pub realized_loss_30d: MetricPattern1<Dollars>,
    pub realized_loss_7d: MetricPattern1<Dollars>,
    pub realized_loss_7d_ema: MetricPattern1<Dollars>,
    pub realized_loss_rel_to_realized_cap: MetricPattern1<StoredF32>,
    pub realized_price: SatsUsdPattern,
    pub realized_price_extra: RatioPattern2,
    pub realized_price_ratio_ext: RatioPattern3,
    pub realized_profit: CumulativeHeightPattern,
    pub realized_profit_1y: MetricPattern1<Dollars>,
    pub realized_profit_24h: MetricPattern1<Dollars>,
    pub realized_profit_30d: MetricPattern1<Dollars>,
    pub realized_profit_7d: MetricPattern1<Dollars>,
    pub realized_profit_7d_ema: MetricPattern1<Dollars>,
    pub realized_profit_rel_to_realized_cap: MetricPattern1<StoredF32>,
    pub realized_profit_to_loss_ratio_1y: MetricPattern1<StoredF64>,
    pub realized_profit_to_loss_ratio_24h: MetricPattern1<StoredF64>,
    pub realized_profit_to_loss_ratio_30d: MetricPattern1<StoredF64>,
    pub realized_profit_to_loss_ratio_7d: MetricPattern1<StoredF64>,
    pub realized_value: MetricPattern1<Dollars>,
    pub realized_value_1y: MetricPattern1<Dollars>,
    pub realized_value_24h: MetricPattern1<Dollars>,
    pub realized_value_30d: MetricPattern1<Dollars>,
    pub realized_value_7d: MetricPattern1<Dollars>,
    pub sell_side_risk_ratio: MetricPattern1<StoredF32>,
    pub sell_side_risk_ratio_1y: MetricPattern1<StoredF32>,
    pub sell_side_risk_ratio_24h: MetricPattern1<StoredF32>,
    pub sell_side_risk_ratio_24h_30d_ema: MetricPattern1<StoredF32>,
    pub sell_side_risk_ratio_24h_7d_ema: MetricPattern1<StoredF32>,
    pub sell_side_risk_ratio_30d: MetricPattern1<StoredF32>,
    pub sell_side_risk_ratio_30d_ema: MetricPattern1<StoredF32>,
    pub sell_side_risk_ratio_7d: MetricPattern1<StoredF32>,
    pub sell_side_risk_ratio_7d_ema: MetricPattern1<StoredF32>,
    pub sent_in_loss: BaseCumulativePattern,
    pub sent_in_loss_14d_ema: BtcSatsUsdPattern,
    pub sent_in_profit: BaseCumulativePattern,
    pub sent_in_profit_14d_ema: BtcSatsUsdPattern,
    pub sopr: MetricPattern1<StoredF64>,
    pub sopr_1y: MetricPattern1<StoredF64>,
    pub sopr_24h: MetricPattern1<StoredF64>,
    pub sopr_24h_30d_ema: MetricPattern1<StoredF64>,
    pub sopr_24h_7d_ema: MetricPattern1<StoredF64>,
    pub sopr_30d: MetricPattern1<StoredF64>,
    pub sopr_30d_ema: MetricPattern1<StoredF64>,
    pub sopr_7d: MetricPattern1<StoredF64>,
    pub sopr_7d_ema: MetricPattern1<StoredF64>,
    pub total_realized_pnl: MetricPattern1<Dollars>,
    pub upper_price_band: SatsUsdPattern,
    pub value_created: MetricPattern1<Dollars>,
    pub value_created_1y: MetricPattern1<Dollars>,
    pub value_created_24h: MetricPattern1<Dollars>,
    pub value_created_30d: MetricPattern1<Dollars>,
    pub value_created_7d: MetricPattern1<Dollars>,
    pub value_destroyed: MetricPattern1<Dollars>,
    pub value_destroyed_1y: MetricPattern1<Dollars>,
    pub value_destroyed_24h: MetricPattern1<Dollars>,
    pub value_destroyed_30d: MetricPattern1<Dollars>,
    pub value_destroyed_7d: MetricPattern1<Dollars>,
}

impl AdjustedCapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            adjusted_sopr: MetricPattern1::new(client.clone(), _m(&acc, "adjusted_sopr")),
            adjusted_sopr_1y: MetricPattern1::new(client.clone(), _m(&acc, "adjusted_sopr_1y")),
            adjusted_sopr_24h: MetricPattern1::new(client.clone(), _m(&acc, "adjusted_sopr_24h")),
            adjusted_sopr_24h_30d_ema: MetricPattern1::new(client.clone(), _m(&acc, "adjusted_sopr_24h_30d_ema")),
            adjusted_sopr_24h_7d_ema: MetricPattern1::new(client.clone(), _m(&acc, "adjusted_sopr_24h_7d_ema")),
            adjusted_sopr_30d: MetricPattern1::new(client.clone(), _m(&acc, "adjusted_sopr_30d")),
            adjusted_sopr_30d_ema: MetricPattern1::new(client.clone(), _m(&acc, "adjusted_sopr_30d_ema")),
            adjusted_sopr_7d: MetricPattern1::new(client.clone(), _m(&acc, "adjusted_sopr_7d")),
            adjusted_sopr_7d_ema: MetricPattern1::new(client.clone(), _m(&acc, "adjusted_sopr_7d_ema")),
            adjusted_value_created: MetricPattern1::new(client.clone(), _m(&acc, "adjusted_value_created")),
            adjusted_value_created_1y: MetricPattern1::new(client.clone(), _m(&acc, "adjusted_value_created_1y")),
            adjusted_value_created_24h: MetricPattern1::new(client.clone(), _m(&acc, "adjusted_value_created_24h")),
            adjusted_value_created_30d: MetricPattern1::new(client.clone(), _m(&acc, "adjusted_value_created_30d")),
            adjusted_value_created_7d: MetricPattern1::new(client.clone(), _m(&acc, "adjusted_value_created_7d")),
            adjusted_value_destroyed: MetricPattern1::new(client.clone(), _m(&acc, "adjusted_value_destroyed")),
            adjusted_value_destroyed_1y: MetricPattern1::new(client.clone(), _m(&acc, "adjusted_value_destroyed_1y")),
            adjusted_value_destroyed_24h: MetricPattern1::new(client.clone(), _m(&acc, "adjusted_value_destroyed_24h")),
            adjusted_value_destroyed_30d: MetricPattern1::new(client.clone(), _m(&acc, "adjusted_value_destroyed_30d")),
            adjusted_value_destroyed_7d: MetricPattern1::new(client.clone(), _m(&acc, "adjusted_value_destroyed_7d")),
            cap_raw: MetricPattern20::new(client.clone(), _m(&acc, "cap_raw")),
            capitulation_flow: MetricPattern1::new(client.clone(), _m(&acc, "capitulation_flow")),
            investor_cap_raw: MetricPattern20::new(client.clone(), _m(&acc, "investor_cap_raw")),
            investor_price: SatsUsdPattern::new(client.clone(), _m(&acc, "investor_price")),
            investor_price_cents: MetricPattern1::new(client.clone(), _m(&acc, "investor_price_cents")),
            investor_price_extra: RatioPattern2::new(client.clone(), _m(&acc, "investor_price_ratio")),
            investor_price_ratio_ext: RatioPattern3::new(client.clone(), _m(&acc, "investor_price_ratio")),
            loss_value_created: MetricPattern1::new(client.clone(), _m(&acc, "loss_value_created")),
            loss_value_destroyed: MetricPattern1::new(client.clone(), _m(&acc, "loss_value_destroyed")),
            lower_price_band: SatsUsdPattern::new(client.clone(), _m(&acc, "lower_price_band")),
            mvrv: MetricPattern1::new(client.clone(), _m(&acc, "mvrv")),
            neg_realized_loss: MetricPattern1::new(client.clone(), _m(&acc, "neg_realized_loss")),
            net_realized_pnl: CumulativeHeightPattern::new(client.clone(), _m(&acc, "net_realized_pnl")),
            net_realized_pnl_7d_ema: MetricPattern1::new(client.clone(), _m(&acc, "net_realized_pnl_7d_ema")),
            net_realized_pnl_cumulative_30d_delta: MetricPattern1::new(client.clone(), _m(&acc, "net_realized_pnl_cumulative_30d_delta")),
            net_realized_pnl_cumulative_30d_delta_rel_to_market_cap: MetricPattern1::new(client.clone(), _m(&acc, "net_realized_pnl_cumulative_30d_delta_rel_to_market_cap")),
            net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap: MetricPattern1::new(client.clone(), _m(&acc, "net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap")),
            net_realized_pnl_rel_to_realized_cap: MetricPattern1::new(client.clone(), _m(&acc, "net_realized_pnl_rel_to_realized_cap")),
            peak_regret: CumulativeHeightPattern::new(client.clone(), _m(&acc, "realized_peak_regret")),
            peak_regret_rel_to_realized_cap: MetricPattern1::new(client.clone(), _m(&acc, "peak_regret_rel_to_realized_cap")),
            profit_flow: MetricPattern1::new(client.clone(), _m(&acc, "profit_flow")),
            profit_value_created: MetricPattern1::new(client.clone(), _m(&acc, "profit_value_created")),
            profit_value_destroyed: MetricPattern1::new(client.clone(), _m(&acc, "profit_value_destroyed")),
            realized_cap: MetricPattern1::new(client.clone(), _m(&acc, "realized_cap")),
            realized_cap_30d_delta: MetricPattern1::new(client.clone(), _m(&acc, "realized_cap_30d_delta")),
            realized_cap_cents: MetricPattern1::new(client.clone(), _m(&acc, "realized_cap_cents")),
            realized_cap_rel_to_own_market_cap: MetricPattern1::new(client.clone(), _m(&acc, "realized_cap_rel_to_own_market_cap")),
            realized_loss: CumulativeHeightPattern::new(client.clone(), _m(&acc, "realized_loss")),
            realized_loss_1y: MetricPattern1::new(client.clone(), _m(&acc, "realized_loss_1y")),
            realized_loss_24h: MetricPattern1::new(client.clone(), _m(&acc, "realized_loss_24h")),
            realized_loss_30d: MetricPattern1::new(client.clone(), _m(&acc, "realized_loss_30d")),
            realized_loss_7d: MetricPattern1::new(client.clone(), _m(&acc, "realized_loss_7d")),
            realized_loss_7d_ema: MetricPattern1::new(client.clone(), _m(&acc, "realized_loss_7d_ema")),
            realized_loss_rel_to_realized_cap: MetricPattern1::new(client.clone(), _m(&acc, "realized_loss_rel_to_realized_cap")),
            realized_price: SatsUsdPattern::new(client.clone(), _m(&acc, "realized_price")),
            realized_price_extra: RatioPattern2::new(client.clone(), _m(&acc, "realized_price_ratio")),
            realized_price_ratio_ext: RatioPattern3::new(client.clone(), _m(&acc, "realized_price_ratio")),
            realized_profit: CumulativeHeightPattern::new(client.clone(), _m(&acc, "realized_profit")),
            realized_profit_1y: MetricPattern1::new(client.clone(), _m(&acc, "realized_profit_1y")),
            realized_profit_24h: MetricPattern1::new(client.clone(), _m(&acc, "realized_profit_24h")),
            realized_profit_30d: MetricPattern1::new(client.clone(), _m(&acc, "realized_profit_30d")),
            realized_profit_7d: MetricPattern1::new(client.clone(), _m(&acc, "realized_profit_7d")),
            realized_profit_7d_ema: MetricPattern1::new(client.clone(), _m(&acc, "realized_profit_7d_ema")),
            realized_profit_rel_to_realized_cap: MetricPattern1::new(client.clone(), _m(&acc, "realized_profit_rel_to_realized_cap")),
            realized_profit_to_loss_ratio_1y: MetricPattern1::new(client.clone(), _m(&acc, "realized_profit_to_loss_ratio_1y")),
            realized_profit_to_loss_ratio_24h: MetricPattern1::new(client.clone(), _m(&acc, "realized_profit_to_loss_ratio_24h")),
            realized_profit_to_loss_ratio_30d: MetricPattern1::new(client.clone(), _m(&acc, "realized_profit_to_loss_ratio_30d")),
            realized_profit_to_loss_ratio_7d: MetricPattern1::new(client.clone(), _m(&acc, "realized_profit_to_loss_ratio_7d")),
            realized_value: MetricPattern1::new(client.clone(), _m(&acc, "realized_value")),
            realized_value_1y: MetricPattern1::new(client.clone(), _m(&acc, "realized_value_1y")),
            realized_value_24h: MetricPattern1::new(client.clone(), _m(&acc, "realized_value_24h")),
            realized_value_30d: MetricPattern1::new(client.clone(), _m(&acc, "realized_value_30d")),
            realized_value_7d: MetricPattern1::new(client.clone(), _m(&acc, "realized_value_7d")),
            sell_side_risk_ratio: MetricPattern1::new(client.clone(), _m(&acc, "sell_side_risk_ratio")),
            sell_side_risk_ratio_1y: MetricPattern1::new(client.clone(), _m(&acc, "sell_side_risk_ratio_1y")),
            sell_side_risk_ratio_24h: MetricPattern1::new(client.clone(), _m(&acc, "sell_side_risk_ratio_24h")),
            sell_side_risk_ratio_24h_30d_ema: MetricPattern1::new(client.clone(), _m(&acc, "sell_side_risk_ratio_24h_30d_ema")),
            sell_side_risk_ratio_24h_7d_ema: MetricPattern1::new(client.clone(), _m(&acc, "sell_side_risk_ratio_24h_7d_ema")),
            sell_side_risk_ratio_30d: MetricPattern1::new(client.clone(), _m(&acc, "sell_side_risk_ratio_30d")),
            sell_side_risk_ratio_30d_ema: MetricPattern1::new(client.clone(), _m(&acc, "sell_side_risk_ratio_30d_ema")),
            sell_side_risk_ratio_7d: MetricPattern1::new(client.clone(), _m(&acc, "sell_side_risk_ratio_7d")),
            sell_side_risk_ratio_7d_ema: MetricPattern1::new(client.clone(), _m(&acc, "sell_side_risk_ratio_7d_ema")),
            sent_in_loss: BaseCumulativePattern::new(client.clone(), _m(&acc, "sent_in_loss")),
            sent_in_loss_14d_ema: BtcSatsUsdPattern::new(client.clone(), _m(&acc, "sent_in_loss_14d_ema")),
            sent_in_profit: BaseCumulativePattern::new(client.clone(), _m(&acc, "sent_in_profit")),
            sent_in_profit_14d_ema: BtcSatsUsdPattern::new(client.clone(), _m(&acc, "sent_in_profit_14d_ema")),
            sopr: MetricPattern1::new(client.clone(), _m(&acc, "sopr")),
            sopr_1y: MetricPattern1::new(client.clone(), _m(&acc, "sopr_1y")),
            sopr_24h: MetricPattern1::new(client.clone(), _m(&acc, "sopr_24h")),
            sopr_24h_30d_ema: MetricPattern1::new(client.clone(), _m(&acc, "sopr_24h_30d_ema")),
            sopr_24h_7d_ema: MetricPattern1::new(client.clone(), _m(&acc, "sopr_24h_7d_ema")),
            sopr_30d: MetricPattern1::new(client.clone(), _m(&acc, "sopr_30d")),
            sopr_30d_ema: MetricPattern1::new(client.clone(), _m(&acc, "sopr_30d_ema")),
            sopr_7d: MetricPattern1::new(client.clone(), _m(&acc, "sopr_7d")),
            sopr_7d_ema: MetricPattern1::new(client.clone(), _m(&acc, "sopr_7d_ema")),
            total_realized_pnl: MetricPattern1::new(client.clone(), _m(&acc, "total_realized_pnl")),
            upper_price_band: SatsUsdPattern::new(client.clone(), _m(&acc, "upper_price_band")),
            value_created: MetricPattern1::new(client.clone(), _m(&acc, "value_created")),
            value_created_1y: MetricPattern1::new(client.clone(), _m(&acc, "value_created_1y")),
            value_created_24h: MetricPattern1::new(client.clone(), _m(&acc, "value_created_24h")),
            value_created_30d: MetricPattern1::new(client.clone(), _m(&acc, "value_created_30d")),
            value_created_7d: MetricPattern1::new(client.clone(), _m(&acc, "value_created_7d")),
            value_destroyed: MetricPattern1::new(client.clone(), _m(&acc, "value_destroyed")),
            value_destroyed_1y: MetricPattern1::new(client.clone(), _m(&acc, "value_destroyed_1y")),
            value_destroyed_24h: MetricPattern1::new(client.clone(), _m(&acc, "value_destroyed_24h")),
            value_destroyed_30d: MetricPattern1::new(client.clone(), _m(&acc, "value_destroyed_30d")),
            value_destroyed_7d: MetricPattern1::new(client.clone(), _m(&acc, "value_destroyed_7d")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct AdjustedCapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern2 {
    pub adjusted_sopr: MetricPattern1<StoredF64>,
    pub adjusted_sopr_1y: MetricPattern1<StoredF64>,
    pub adjusted_sopr_24h: MetricPattern1<StoredF64>,
    pub adjusted_sopr_24h_30d_ema: MetricPattern1<StoredF64>,
    pub adjusted_sopr_24h_7d_ema: MetricPattern1<StoredF64>,
    pub adjusted_sopr_30d: MetricPattern1<StoredF64>,
    pub adjusted_sopr_30d_ema: MetricPattern1<StoredF64>,
    pub adjusted_sopr_7d: MetricPattern1<StoredF64>,
    pub adjusted_sopr_7d_ema: MetricPattern1<StoredF64>,
    pub adjusted_value_created: MetricPattern1<Dollars>,
    pub adjusted_value_created_1y: MetricPattern1<Dollars>,
    pub adjusted_value_created_24h: MetricPattern1<Dollars>,
    pub adjusted_value_created_30d: MetricPattern1<Dollars>,
    pub adjusted_value_created_7d: MetricPattern1<Dollars>,
    pub adjusted_value_destroyed: MetricPattern1<Dollars>,
    pub adjusted_value_destroyed_1y: MetricPattern1<Dollars>,
    pub adjusted_value_destroyed_24h: MetricPattern1<Dollars>,
    pub adjusted_value_destroyed_30d: MetricPattern1<Dollars>,
    pub adjusted_value_destroyed_7d: MetricPattern1<Dollars>,
    pub cap_raw: MetricPattern20<CentsSats>,
    pub capitulation_flow: MetricPattern1<Dollars>,
    pub investor_cap_raw: MetricPattern20<CentsSquaredSats>,
    pub investor_price: SatsUsdPattern,
    pub investor_price_cents: MetricPattern1<Cents>,
    pub investor_price_extra: RatioPattern2,
    pub loss_value_created: MetricPattern1<Dollars>,
    pub loss_value_destroyed: MetricPattern1<Dollars>,
    pub lower_price_band: SatsUsdPattern,
    pub mvrv: MetricPattern1<StoredF32>,
    pub neg_realized_loss: MetricPattern1<Dollars>,
    pub net_realized_pnl: CumulativeHeightPattern,
    pub net_realized_pnl_7d_ema: MetricPattern1<Dollars>,
    pub net_realized_pnl_cumulative_30d_delta: MetricPattern1<Dollars>,
    pub net_realized_pnl_cumulative_30d_delta_rel_to_market_cap: MetricPattern1<StoredF32>,
    pub net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap: MetricPattern1<StoredF32>,
    pub net_realized_pnl_rel_to_realized_cap: MetricPattern1<StoredF32>,
    pub peak_regret: CumulativeHeightPattern,
    pub peak_regret_rel_to_realized_cap: MetricPattern1<StoredF32>,
    pub profit_flow: MetricPattern1<Dollars>,
    pub profit_value_created: MetricPattern1<Dollars>,
    pub profit_value_destroyed: MetricPattern1<Dollars>,
    pub realized_cap: MetricPattern1<Dollars>,
    pub realized_cap_30d_delta: MetricPattern1<Dollars>,
    pub realized_cap_cents: MetricPattern1<Cents>,
    pub realized_loss: CumulativeHeightPattern,
    pub realized_loss_7d_ema: MetricPattern1<Dollars>,
    pub realized_loss_rel_to_realized_cap: MetricPattern1<StoredF32>,
    pub realized_price: SatsUsdPattern,
    pub realized_price_extra: RatioPattern2,
    pub realized_profit: CumulativeHeightPattern,
    pub realized_profit_7d_ema: MetricPattern1<Dollars>,
    pub realized_profit_rel_to_realized_cap: MetricPattern1<StoredF32>,
    pub realized_value: MetricPattern1<Dollars>,
    pub realized_value_1y: MetricPattern1<Dollars>,
    pub realized_value_24h: MetricPattern1<Dollars>,
    pub realized_value_30d: MetricPattern1<Dollars>,
    pub realized_value_7d: MetricPattern1<Dollars>,
    pub sell_side_risk_ratio: MetricPattern1<StoredF32>,
    pub sell_side_risk_ratio_1y: MetricPattern1<StoredF32>,
    pub sell_side_risk_ratio_24h: MetricPattern1<StoredF32>,
    pub sell_side_risk_ratio_24h_30d_ema: MetricPattern1<StoredF32>,
    pub sell_side_risk_ratio_24h_7d_ema: MetricPattern1<StoredF32>,
    pub sell_side_risk_ratio_30d: MetricPattern1<StoredF32>,
    pub sell_side_risk_ratio_30d_ema: MetricPattern1<StoredF32>,
    pub sell_side_risk_ratio_7d: MetricPattern1<StoredF32>,
    pub sell_side_risk_ratio_7d_ema: MetricPattern1<StoredF32>,
    pub sent_in_loss: BaseCumulativePattern,
    pub sent_in_loss_14d_ema: BtcSatsUsdPattern,
    pub sent_in_profit: BaseCumulativePattern,
    pub sent_in_profit_14d_ema: BtcSatsUsdPattern,
    pub sopr: MetricPattern1<StoredF64>,
    pub sopr_1y: MetricPattern1<StoredF64>,
    pub sopr_24h: MetricPattern1<StoredF64>,
    pub sopr_24h_30d_ema: MetricPattern1<StoredF64>,
    pub sopr_24h_7d_ema: MetricPattern1<StoredF64>,
    pub sopr_30d: MetricPattern1<StoredF64>,
    pub sopr_30d_ema: MetricPattern1<StoredF64>,
    pub sopr_7d: MetricPattern1<StoredF64>,
    pub sopr_7d_ema: MetricPattern1<StoredF64>,
    pub total_realized_pnl: MetricPattern1<Dollars>,
    pub upper_price_band: SatsUsdPattern,
    pub value_created: MetricPattern1<Dollars>,
    pub value_created_1y: MetricPattern1<Dollars>,
    pub value_created_24h: MetricPattern1<Dollars>,
    pub value_created_30d: MetricPattern1<Dollars>,
    pub value_created_7d: MetricPattern1<Dollars>,
    pub value_destroyed: MetricPattern1<Dollars>,
    pub value_destroyed_1y: MetricPattern1<Dollars>,
    pub value_destroyed_24h: MetricPattern1<Dollars>,
    pub value_destroyed_30d: MetricPattern1<Dollars>,
    pub value_destroyed_7d: MetricPattern1<Dollars>,
}

impl AdjustedCapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern2 {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            adjusted_sopr: MetricPattern1::new(client.clone(), _m(&acc, "adjusted_sopr")),
            adjusted_sopr_1y: MetricPattern1::new(client.clone(), _m(&acc, "adjusted_sopr_1y")),
            adjusted_sopr_24h: MetricPattern1::new(client.clone(), _m(&acc, "adjusted_sopr_24h")),
            adjusted_sopr_24h_30d_ema: MetricPattern1::new(client.clone(), _m(&acc, "adjusted_sopr_24h_30d_ema")),
            adjusted_sopr_24h_7d_ema: MetricPattern1::new(client.clone(), _m(&acc, "adjusted_sopr_24h_7d_ema")),
            adjusted_sopr_30d: MetricPattern1::new(client.clone(), _m(&acc, "adjusted_sopr_30d")),
            adjusted_sopr_30d_ema: MetricPattern1::new(client.clone(), _m(&acc, "adjusted_sopr_30d_ema")),
            adjusted_sopr_7d: MetricPattern1::new(client.clone(), _m(&acc, "adjusted_sopr_7d")),
            adjusted_sopr_7d_ema: MetricPattern1::new(client.clone(), _m(&acc, "adjusted_sopr_7d_ema")),
            adjusted_value_created: MetricPattern1::new(client.clone(), _m(&acc, "adjusted_value_created")),
            adjusted_value_created_1y: MetricPattern1::new(client.clone(), _m(&acc, "adjusted_value_created_1y")),
            adjusted_value_created_24h: MetricPattern1::new(client.clone(), _m(&acc, "adjusted_value_created_24h")),
            adjusted_value_created_30d: MetricPattern1::new(client.clone(), _m(&acc, "adjusted_value_created_30d")),
            adjusted_value_created_7d: MetricPattern1::new(client.clone(), _m(&acc, "adjusted_value_created_7d")),
            adjusted_value_destroyed: MetricPattern1::new(client.clone(), _m(&acc, "adjusted_value_destroyed")),
            adjusted_value_destroyed_1y: MetricPattern1::new(client.clone(), _m(&acc, "adjusted_value_destroyed_1y")),
            adjusted_value_destroyed_24h: MetricPattern1::new(client.clone(), _m(&acc, "adjusted_value_destroyed_24h")),
            adjusted_value_destroyed_30d: MetricPattern1::new(client.clone(), _m(&acc, "adjusted_value_destroyed_30d")),
            adjusted_value_destroyed_7d: MetricPattern1::new(client.clone(), _m(&acc, "adjusted_value_destroyed_7d")),
            cap_raw: MetricPattern20::new(client.clone(), _m(&acc, "cap_raw")),
            capitulation_flow: MetricPattern1::new(client.clone(), _m(&acc, "capitulation_flow")),
            investor_cap_raw: MetricPattern20::new(client.clone(), _m(&acc, "investor_cap_raw")),
            investor_price: SatsUsdPattern::new(client.clone(), _m(&acc, "investor_price")),
            investor_price_cents: MetricPattern1::new(client.clone(), _m(&acc, "investor_price_cents")),
            investor_price_extra: RatioPattern2::new(client.clone(), _m(&acc, "investor_price_ratio")),
            loss_value_created: MetricPattern1::new(client.clone(), _m(&acc, "loss_value_created")),
            loss_value_destroyed: MetricPattern1::new(client.clone(), _m(&acc, "loss_value_destroyed")),
            lower_price_band: SatsUsdPattern::new(client.clone(), _m(&acc, "lower_price_band")),
            mvrv: MetricPattern1::new(client.clone(), _m(&acc, "mvrv")),
            neg_realized_loss: MetricPattern1::new(client.clone(), _m(&acc, "neg_realized_loss")),
            net_realized_pnl: CumulativeHeightPattern::new(client.clone(), _m(&acc, "net_realized_pnl")),
            net_realized_pnl_7d_ema: MetricPattern1::new(client.clone(), _m(&acc, "net_realized_pnl_7d_ema")),
            net_realized_pnl_cumulative_30d_delta: MetricPattern1::new(client.clone(), _m(&acc, "net_realized_pnl_cumulative_30d_delta")),
            net_realized_pnl_cumulative_30d_delta_rel_to_market_cap: MetricPattern1::new(client.clone(), _m(&acc, "net_realized_pnl_cumulative_30d_delta_rel_to_market_cap")),
            net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap: MetricPattern1::new(client.clone(), _m(&acc, "net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap")),
            net_realized_pnl_rel_to_realized_cap: MetricPattern1::new(client.clone(), _m(&acc, "net_realized_pnl_rel_to_realized_cap")),
            peak_regret: CumulativeHeightPattern::new(client.clone(), _m(&acc, "realized_peak_regret")),
            peak_regret_rel_to_realized_cap: MetricPattern1::new(client.clone(), _m(&acc, "peak_regret_rel_to_realized_cap")),
            profit_flow: MetricPattern1::new(client.clone(), _m(&acc, "profit_flow")),
            profit_value_created: MetricPattern1::new(client.clone(), _m(&acc, "profit_value_created")),
            profit_value_destroyed: MetricPattern1::new(client.clone(), _m(&acc, "profit_value_destroyed")),
            realized_cap: MetricPattern1::new(client.clone(), _m(&acc, "realized_cap")),
            realized_cap_30d_delta: MetricPattern1::new(client.clone(), _m(&acc, "realized_cap_30d_delta")),
            realized_cap_cents: MetricPattern1::new(client.clone(), _m(&acc, "realized_cap_cents")),
            realized_loss: CumulativeHeightPattern::new(client.clone(), _m(&acc, "realized_loss")),
            realized_loss_7d_ema: MetricPattern1::new(client.clone(), _m(&acc, "realized_loss_7d_ema")),
            realized_loss_rel_to_realized_cap: MetricPattern1::new(client.clone(), _m(&acc, "realized_loss_rel_to_realized_cap")),
            realized_price: SatsUsdPattern::new(client.clone(), _m(&acc, "realized_price")),
            realized_price_extra: RatioPattern2::new(client.clone(), _m(&acc, "realized_price_ratio")),
            realized_profit: CumulativeHeightPattern::new(client.clone(), _m(&acc, "realized_profit")),
            realized_profit_7d_ema: MetricPattern1::new(client.clone(), _m(&acc, "realized_profit_7d_ema")),
            realized_profit_rel_to_realized_cap: MetricPattern1::new(client.clone(), _m(&acc, "realized_profit_rel_to_realized_cap")),
            realized_value: MetricPattern1::new(client.clone(), _m(&acc, "realized_value")),
            realized_value_1y: MetricPattern1::new(client.clone(), _m(&acc, "realized_value_1y")),
            realized_value_24h: MetricPattern1::new(client.clone(), _m(&acc, "realized_value_24h")),
            realized_value_30d: MetricPattern1::new(client.clone(), _m(&acc, "realized_value_30d")),
            realized_value_7d: MetricPattern1::new(client.clone(), _m(&acc, "realized_value_7d")),
            sell_side_risk_ratio: MetricPattern1::new(client.clone(), _m(&acc, "sell_side_risk_ratio")),
            sell_side_risk_ratio_1y: MetricPattern1::new(client.clone(), _m(&acc, "sell_side_risk_ratio_1y")),
            sell_side_risk_ratio_24h: MetricPattern1::new(client.clone(), _m(&acc, "sell_side_risk_ratio_24h")),
            sell_side_risk_ratio_24h_30d_ema: MetricPattern1::new(client.clone(), _m(&acc, "sell_side_risk_ratio_24h_30d_ema")),
            sell_side_risk_ratio_24h_7d_ema: MetricPattern1::new(client.clone(), _m(&acc, "sell_side_risk_ratio_24h_7d_ema")),
            sell_side_risk_ratio_30d: MetricPattern1::new(client.clone(), _m(&acc, "sell_side_risk_ratio_30d")),
            sell_side_risk_ratio_30d_ema: MetricPattern1::new(client.clone(), _m(&acc, "sell_side_risk_ratio_30d_ema")),
            sell_side_risk_ratio_7d: MetricPattern1::new(client.clone(), _m(&acc, "sell_side_risk_ratio_7d")),
            sell_side_risk_ratio_7d_ema: MetricPattern1::new(client.clone(), _m(&acc, "sell_side_risk_ratio_7d_ema")),
            sent_in_loss: BaseCumulativePattern::new(client.clone(), _m(&acc, "sent_in_loss")),
            sent_in_loss_14d_ema: BtcSatsUsdPattern::new(client.clone(), _m(&acc, "sent_in_loss_14d_ema")),
            sent_in_profit: BaseCumulativePattern::new(client.clone(), _m(&acc, "sent_in_profit")),
            sent_in_profit_14d_ema: BtcSatsUsdPattern::new(client.clone(), _m(&acc, "sent_in_profit_14d_ema")),
            sopr: MetricPattern1::new(client.clone(), _m(&acc, "sopr")),
            sopr_1y: MetricPattern1::new(client.clone(), _m(&acc, "sopr_1y")),
            sopr_24h: MetricPattern1::new(client.clone(), _m(&acc, "sopr_24h")),
            sopr_24h_30d_ema: MetricPattern1::new(client.clone(), _m(&acc, "sopr_24h_30d_ema")),
            sopr_24h_7d_ema: MetricPattern1::new(client.clone(), _m(&acc, "sopr_24h_7d_ema")),
            sopr_30d: MetricPattern1::new(client.clone(), _m(&acc, "sopr_30d")),
            sopr_30d_ema: MetricPattern1::new(client.clone(), _m(&acc, "sopr_30d_ema")),
            sopr_7d: MetricPattern1::new(client.clone(), _m(&acc, "sopr_7d")),
            sopr_7d_ema: MetricPattern1::new(client.clone(), _m(&acc, "sopr_7d_ema")),
            total_realized_pnl: MetricPattern1::new(client.clone(), _m(&acc, "total_realized_pnl")),
            upper_price_band: SatsUsdPattern::new(client.clone(), _m(&acc, "upper_price_band")),
            value_created: MetricPattern1::new(client.clone(), _m(&acc, "value_created")),
            value_created_1y: MetricPattern1::new(client.clone(), _m(&acc, "value_created_1y")),
            value_created_24h: MetricPattern1::new(client.clone(), _m(&acc, "value_created_24h")),
            value_created_30d: MetricPattern1::new(client.clone(), _m(&acc, "value_created_30d")),
            value_created_7d: MetricPattern1::new(client.clone(), _m(&acc, "value_created_7d")),
            value_destroyed: MetricPattern1::new(client.clone(), _m(&acc, "value_destroyed")),
            value_destroyed_1y: MetricPattern1::new(client.clone(), _m(&acc, "value_destroyed_1y")),
            value_destroyed_24h: MetricPattern1::new(client.clone(), _m(&acc, "value_destroyed_24h")),
            value_destroyed_30d: MetricPattern1::new(client.clone(), _m(&acc, "value_destroyed_30d")),
            value_destroyed_7d: MetricPattern1::new(client.clone(), _m(&acc, "value_destroyed_7d")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct CapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern2 {
    pub cap_raw: MetricPattern20<CentsSats>,
    pub capitulation_flow: MetricPattern1<Dollars>,
    pub investor_cap_raw: MetricPattern20<CentsSquaredSats>,
    pub investor_price: SatsUsdPattern,
    pub investor_price_cents: MetricPattern1<Cents>,
    pub investor_price_extra: RatioPattern2,
    pub investor_price_ratio_ext: RatioPattern3,
    pub loss_value_created: MetricPattern1<Dollars>,
    pub loss_value_destroyed: MetricPattern1<Dollars>,
    pub lower_price_band: SatsUsdPattern,
    pub mvrv: MetricPattern1<StoredF32>,
    pub neg_realized_loss: MetricPattern1<Dollars>,
    pub net_realized_pnl: CumulativeHeightPattern,
    pub net_realized_pnl_7d_ema: MetricPattern1<Dollars>,
    pub net_realized_pnl_cumulative_30d_delta: MetricPattern1<Dollars>,
    pub net_realized_pnl_cumulative_30d_delta_rel_to_market_cap: MetricPattern1<StoredF32>,
    pub net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap: MetricPattern1<StoredF32>,
    pub net_realized_pnl_rel_to_realized_cap: MetricPattern1<StoredF32>,
    pub peak_regret: CumulativeHeightPattern,
    pub peak_regret_rel_to_realized_cap: MetricPattern1<StoredF32>,
    pub profit_flow: MetricPattern1<Dollars>,
    pub profit_value_created: MetricPattern1<Dollars>,
    pub profit_value_destroyed: MetricPattern1<Dollars>,
    pub realized_cap: MetricPattern1<Dollars>,
    pub realized_cap_30d_delta: MetricPattern1<Dollars>,
    pub realized_cap_cents: MetricPattern1<Cents>,
    pub realized_cap_rel_to_own_market_cap: MetricPattern1<StoredF32>,
    pub realized_loss: CumulativeHeightPattern,
    pub realized_loss_1y: MetricPattern1<Dollars>,
    pub realized_loss_24h: MetricPattern1<Dollars>,
    pub realized_loss_30d: MetricPattern1<Dollars>,
    pub realized_loss_7d: MetricPattern1<Dollars>,
    pub realized_loss_7d_ema: MetricPattern1<Dollars>,
    pub realized_loss_rel_to_realized_cap: MetricPattern1<StoredF32>,
    pub realized_price: SatsUsdPattern,
    pub realized_price_extra: RatioPattern2,
    pub realized_price_ratio_ext: RatioPattern3,
    pub realized_profit: CumulativeHeightPattern,
    pub realized_profit_1y: MetricPattern1<Dollars>,
    pub realized_profit_24h: MetricPattern1<Dollars>,
    pub realized_profit_30d: MetricPattern1<Dollars>,
    pub realized_profit_7d: MetricPattern1<Dollars>,
    pub realized_profit_7d_ema: MetricPattern1<Dollars>,
    pub realized_profit_rel_to_realized_cap: MetricPattern1<StoredF32>,
    pub realized_profit_to_loss_ratio_1y: MetricPattern1<StoredF64>,
    pub realized_profit_to_loss_ratio_24h: MetricPattern1<StoredF64>,
    pub realized_profit_to_loss_ratio_30d: MetricPattern1<StoredF64>,
    pub realized_profit_to_loss_ratio_7d: MetricPattern1<StoredF64>,
    pub realized_value: MetricPattern1<Dollars>,
    pub realized_value_1y: MetricPattern1<Dollars>,
    pub realized_value_24h: MetricPattern1<Dollars>,
    pub realized_value_30d: MetricPattern1<Dollars>,
    pub realized_value_7d: MetricPattern1<Dollars>,
    pub sell_side_risk_ratio: MetricPattern1<StoredF32>,
    pub sell_side_risk_ratio_1y: MetricPattern1<StoredF32>,
    pub sell_side_risk_ratio_24h: MetricPattern1<StoredF32>,
    pub sell_side_risk_ratio_24h_30d_ema: MetricPattern1<StoredF32>,
    pub sell_side_risk_ratio_24h_7d_ema: MetricPattern1<StoredF32>,
    pub sell_side_risk_ratio_30d: MetricPattern1<StoredF32>,
    pub sell_side_risk_ratio_30d_ema: MetricPattern1<StoredF32>,
    pub sell_side_risk_ratio_7d: MetricPattern1<StoredF32>,
    pub sell_side_risk_ratio_7d_ema: MetricPattern1<StoredF32>,
    pub sent_in_loss: BaseCumulativePattern,
    pub sent_in_loss_14d_ema: BtcSatsUsdPattern,
    pub sent_in_profit: BaseCumulativePattern,
    pub sent_in_profit_14d_ema: BtcSatsUsdPattern,
    pub sopr: MetricPattern1<StoredF64>,
    pub sopr_1y: MetricPattern1<StoredF64>,
    pub sopr_24h: MetricPattern1<StoredF64>,
    pub sopr_24h_30d_ema: MetricPattern1<StoredF64>,
    pub sopr_24h_7d_ema: MetricPattern1<StoredF64>,
    pub sopr_30d: MetricPattern1<StoredF64>,
    pub sopr_30d_ema: MetricPattern1<StoredF64>,
    pub sopr_7d: MetricPattern1<StoredF64>,
    pub sopr_7d_ema: MetricPattern1<StoredF64>,
    pub total_realized_pnl: MetricPattern1<Dollars>,
    pub upper_price_band: SatsUsdPattern,
    pub value_created: MetricPattern1<Dollars>,
    pub value_created_1y: MetricPattern1<Dollars>,
    pub value_created_24h: MetricPattern1<Dollars>,
    pub value_created_30d: MetricPattern1<Dollars>,
    pub value_created_7d: MetricPattern1<Dollars>,
    pub value_destroyed: MetricPattern1<Dollars>,
    pub value_destroyed_1y: MetricPattern1<Dollars>,
    pub value_destroyed_24h: MetricPattern1<Dollars>,
    pub value_destroyed_30d: MetricPattern1<Dollars>,
    pub value_destroyed_7d: MetricPattern1<Dollars>,
}

impl CapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern2 {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            cap_raw: MetricPattern20::new(client.clone(), _m(&acc, "cap_raw")),
            capitulation_flow: MetricPattern1::new(client.clone(), _m(&acc, "capitulation_flow")),
            investor_cap_raw: MetricPattern20::new(client.clone(), _m(&acc, "investor_cap_raw")),
            investor_price: SatsUsdPattern::new(client.clone(), _m(&acc, "investor_price")),
            investor_price_cents: MetricPattern1::new(client.clone(), _m(&acc, "investor_price_cents")),
            investor_price_extra: RatioPattern2::new(client.clone(), _m(&acc, "investor_price_ratio")),
            investor_price_ratio_ext: RatioPattern3::new(client.clone(), _m(&acc, "investor_price_ratio")),
            loss_value_created: MetricPattern1::new(client.clone(), _m(&acc, "loss_value_created")),
            loss_value_destroyed: MetricPattern1::new(client.clone(), _m(&acc, "loss_value_destroyed")),
            lower_price_band: SatsUsdPattern::new(client.clone(), _m(&acc, "lower_price_band")),
            mvrv: MetricPattern1::new(client.clone(), _m(&acc, "mvrv")),
            neg_realized_loss: MetricPattern1::new(client.clone(), _m(&acc, "neg_realized_loss")),
            net_realized_pnl: CumulativeHeightPattern::new(client.clone(), _m(&acc, "net_realized_pnl")),
            net_realized_pnl_7d_ema: MetricPattern1::new(client.clone(), _m(&acc, "net_realized_pnl_7d_ema")),
            net_realized_pnl_cumulative_30d_delta: MetricPattern1::new(client.clone(), _m(&acc, "net_realized_pnl_cumulative_30d_delta")),
            net_realized_pnl_cumulative_30d_delta_rel_to_market_cap: MetricPattern1::new(client.clone(), _m(&acc, "net_realized_pnl_cumulative_30d_delta_rel_to_market_cap")),
            net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap: MetricPattern1::new(client.clone(), _m(&acc, "net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap")),
            net_realized_pnl_rel_to_realized_cap: MetricPattern1::new(client.clone(), _m(&acc, "net_realized_pnl_rel_to_realized_cap")),
            peak_regret: CumulativeHeightPattern::new(client.clone(), _m(&acc, "realized_peak_regret")),
            peak_regret_rel_to_realized_cap: MetricPattern1::new(client.clone(), _m(&acc, "peak_regret_rel_to_realized_cap")),
            profit_flow: MetricPattern1::new(client.clone(), _m(&acc, "profit_flow")),
            profit_value_created: MetricPattern1::new(client.clone(), _m(&acc, "profit_value_created")),
            profit_value_destroyed: MetricPattern1::new(client.clone(), _m(&acc, "profit_value_destroyed")),
            realized_cap: MetricPattern1::new(client.clone(), _m(&acc, "realized_cap")),
            realized_cap_30d_delta: MetricPattern1::new(client.clone(), _m(&acc, "realized_cap_30d_delta")),
            realized_cap_cents: MetricPattern1::new(client.clone(), _m(&acc, "realized_cap_cents")),
            realized_cap_rel_to_own_market_cap: MetricPattern1::new(client.clone(), _m(&acc, "realized_cap_rel_to_own_market_cap")),
            realized_loss: CumulativeHeightPattern::new(client.clone(), _m(&acc, "realized_loss")),
            realized_loss_1y: MetricPattern1::new(client.clone(), _m(&acc, "realized_loss_1y")),
            realized_loss_24h: MetricPattern1::new(client.clone(), _m(&acc, "realized_loss_24h")),
            realized_loss_30d: MetricPattern1::new(client.clone(), _m(&acc, "realized_loss_30d")),
            realized_loss_7d: MetricPattern1::new(client.clone(), _m(&acc, "realized_loss_7d")),
            realized_loss_7d_ema: MetricPattern1::new(client.clone(), _m(&acc, "realized_loss_7d_ema")),
            realized_loss_rel_to_realized_cap: MetricPattern1::new(client.clone(), _m(&acc, "realized_loss_rel_to_realized_cap")),
            realized_price: SatsUsdPattern::new(client.clone(), _m(&acc, "realized_price")),
            realized_price_extra: RatioPattern2::new(client.clone(), _m(&acc, "realized_price_ratio")),
            realized_price_ratio_ext: RatioPattern3::new(client.clone(), _m(&acc, "realized_price_ratio")),
            realized_profit: CumulativeHeightPattern::new(client.clone(), _m(&acc, "realized_profit")),
            realized_profit_1y: MetricPattern1::new(client.clone(), _m(&acc, "realized_profit_1y")),
            realized_profit_24h: MetricPattern1::new(client.clone(), _m(&acc, "realized_profit_24h")),
            realized_profit_30d: MetricPattern1::new(client.clone(), _m(&acc, "realized_profit_30d")),
            realized_profit_7d: MetricPattern1::new(client.clone(), _m(&acc, "realized_profit_7d")),
            realized_profit_7d_ema: MetricPattern1::new(client.clone(), _m(&acc, "realized_profit_7d_ema")),
            realized_profit_rel_to_realized_cap: MetricPattern1::new(client.clone(), _m(&acc, "realized_profit_rel_to_realized_cap")),
            realized_profit_to_loss_ratio_1y: MetricPattern1::new(client.clone(), _m(&acc, "realized_profit_to_loss_ratio_1y")),
            realized_profit_to_loss_ratio_24h: MetricPattern1::new(client.clone(), _m(&acc, "realized_profit_to_loss_ratio_24h")),
            realized_profit_to_loss_ratio_30d: MetricPattern1::new(client.clone(), _m(&acc, "realized_profit_to_loss_ratio_30d")),
            realized_profit_to_loss_ratio_7d: MetricPattern1::new(client.clone(), _m(&acc, "realized_profit_to_loss_ratio_7d")),
            realized_value: MetricPattern1::new(client.clone(), _m(&acc, "realized_value")),
            realized_value_1y: MetricPattern1::new(client.clone(), _m(&acc, "realized_value_1y")),
            realized_value_24h: MetricPattern1::new(client.clone(), _m(&acc, "realized_value_24h")),
            realized_value_30d: MetricPattern1::new(client.clone(), _m(&acc, "realized_value_30d")),
            realized_value_7d: MetricPattern1::new(client.clone(), _m(&acc, "realized_value_7d")),
            sell_side_risk_ratio: MetricPattern1::new(client.clone(), _m(&acc, "sell_side_risk_ratio")),
            sell_side_risk_ratio_1y: MetricPattern1::new(client.clone(), _m(&acc, "sell_side_risk_ratio_1y")),
            sell_side_risk_ratio_24h: MetricPattern1::new(client.clone(), _m(&acc, "sell_side_risk_ratio_24h")),
            sell_side_risk_ratio_24h_30d_ema: MetricPattern1::new(client.clone(), _m(&acc, "sell_side_risk_ratio_24h_30d_ema")),
            sell_side_risk_ratio_24h_7d_ema: MetricPattern1::new(client.clone(), _m(&acc, "sell_side_risk_ratio_24h_7d_ema")),
            sell_side_risk_ratio_30d: MetricPattern1::new(client.clone(), _m(&acc, "sell_side_risk_ratio_30d")),
            sell_side_risk_ratio_30d_ema: MetricPattern1::new(client.clone(), _m(&acc, "sell_side_risk_ratio_30d_ema")),
            sell_side_risk_ratio_7d: MetricPattern1::new(client.clone(), _m(&acc, "sell_side_risk_ratio_7d")),
            sell_side_risk_ratio_7d_ema: MetricPattern1::new(client.clone(), _m(&acc, "sell_side_risk_ratio_7d_ema")),
            sent_in_loss: BaseCumulativePattern::new(client.clone(), _m(&acc, "sent_in_loss")),
            sent_in_loss_14d_ema: BtcSatsUsdPattern::new(client.clone(), _m(&acc, "sent_in_loss_14d_ema")),
            sent_in_profit: BaseCumulativePattern::new(client.clone(), _m(&acc, "sent_in_profit")),
            sent_in_profit_14d_ema: BtcSatsUsdPattern::new(client.clone(), _m(&acc, "sent_in_profit_14d_ema")),
            sopr: MetricPattern1::new(client.clone(), _m(&acc, "sopr")),
            sopr_1y: MetricPattern1::new(client.clone(), _m(&acc, "sopr_1y")),
            sopr_24h: MetricPattern1::new(client.clone(), _m(&acc, "sopr_24h")),
            sopr_24h_30d_ema: MetricPattern1::new(client.clone(), _m(&acc, "sopr_24h_30d_ema")),
            sopr_24h_7d_ema: MetricPattern1::new(client.clone(), _m(&acc, "sopr_24h_7d_ema")),
            sopr_30d: MetricPattern1::new(client.clone(), _m(&acc, "sopr_30d")),
            sopr_30d_ema: MetricPattern1::new(client.clone(), _m(&acc, "sopr_30d_ema")),
            sopr_7d: MetricPattern1::new(client.clone(), _m(&acc, "sopr_7d")),
            sopr_7d_ema: MetricPattern1::new(client.clone(), _m(&acc, "sopr_7d_ema")),
            total_realized_pnl: MetricPattern1::new(client.clone(), _m(&acc, "total_realized_pnl")),
            upper_price_band: SatsUsdPattern::new(client.clone(), _m(&acc, "upper_price_band")),
            value_created: MetricPattern1::new(client.clone(), _m(&acc, "value_created")),
            value_created_1y: MetricPattern1::new(client.clone(), _m(&acc, "value_created_1y")),
            value_created_24h: MetricPattern1::new(client.clone(), _m(&acc, "value_created_24h")),
            value_created_30d: MetricPattern1::new(client.clone(), _m(&acc, "value_created_30d")),
            value_created_7d: MetricPattern1::new(client.clone(), _m(&acc, "value_created_7d")),
            value_destroyed: MetricPattern1::new(client.clone(), _m(&acc, "value_destroyed")),
            value_destroyed_1y: MetricPattern1::new(client.clone(), _m(&acc, "value_destroyed_1y")),
            value_destroyed_24h: MetricPattern1::new(client.clone(), _m(&acc, "value_destroyed_24h")),
            value_destroyed_30d: MetricPattern1::new(client.clone(), _m(&acc, "value_destroyed_30d")),
            value_destroyed_7d: MetricPattern1::new(client.clone(), _m(&acc, "value_destroyed_7d")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct CapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern {
    pub cap_raw: MetricPattern20<CentsSats>,
    pub capitulation_flow: MetricPattern1<Dollars>,
    pub investor_cap_raw: MetricPattern20<CentsSquaredSats>,
    pub investor_price: SatsUsdPattern,
    pub investor_price_cents: MetricPattern1<Cents>,
    pub investor_price_extra: RatioPattern2,
    pub loss_value_created: MetricPattern1<Dollars>,
    pub loss_value_destroyed: MetricPattern1<Dollars>,
    pub lower_price_band: SatsUsdPattern,
    pub mvrv: MetricPattern1<StoredF32>,
    pub neg_realized_loss: MetricPattern1<Dollars>,
    pub net_realized_pnl: CumulativeHeightPattern,
    pub net_realized_pnl_7d_ema: MetricPattern1<Dollars>,
    pub net_realized_pnl_cumulative_30d_delta: MetricPattern1<Dollars>,
    pub net_realized_pnl_cumulative_30d_delta_rel_to_market_cap: MetricPattern1<StoredF32>,
    pub net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap: MetricPattern1<StoredF32>,
    pub net_realized_pnl_rel_to_realized_cap: MetricPattern1<StoredF32>,
    pub peak_regret: CumulativeHeightPattern,
    pub peak_regret_rel_to_realized_cap: MetricPattern1<StoredF32>,
    pub profit_flow: MetricPattern1<Dollars>,
    pub profit_value_created: MetricPattern1<Dollars>,
    pub profit_value_destroyed: MetricPattern1<Dollars>,
    pub realized_cap: MetricPattern1<Dollars>,
    pub realized_cap_30d_delta: MetricPattern1<Dollars>,
    pub realized_cap_cents: MetricPattern1<Cents>,
    pub realized_loss: CumulativeHeightPattern,
    pub realized_loss_7d_ema: MetricPattern1<Dollars>,
    pub realized_loss_rel_to_realized_cap: MetricPattern1<StoredF32>,
    pub realized_price: SatsUsdPattern,
    pub realized_price_extra: RatioPattern2,
    pub realized_profit: CumulativeHeightPattern,
    pub realized_profit_7d_ema: MetricPattern1<Dollars>,
    pub realized_profit_rel_to_realized_cap: MetricPattern1<StoredF32>,
    pub realized_value: MetricPattern1<Dollars>,
    pub realized_value_1y: MetricPattern1<Dollars>,
    pub realized_value_24h: MetricPattern1<Dollars>,
    pub realized_value_30d: MetricPattern1<Dollars>,
    pub realized_value_7d: MetricPattern1<Dollars>,
    pub sell_side_risk_ratio: MetricPattern1<StoredF32>,
    pub sell_side_risk_ratio_1y: MetricPattern1<StoredF32>,
    pub sell_side_risk_ratio_24h: MetricPattern1<StoredF32>,
    pub sell_side_risk_ratio_24h_30d_ema: MetricPattern1<StoredF32>,
    pub sell_side_risk_ratio_24h_7d_ema: MetricPattern1<StoredF32>,
    pub sell_side_risk_ratio_30d: MetricPattern1<StoredF32>,
    pub sell_side_risk_ratio_30d_ema: MetricPattern1<StoredF32>,
    pub sell_side_risk_ratio_7d: MetricPattern1<StoredF32>,
    pub sell_side_risk_ratio_7d_ema: MetricPattern1<StoredF32>,
    pub sent_in_loss: BaseCumulativePattern,
    pub sent_in_loss_14d_ema: BtcSatsUsdPattern,
    pub sent_in_profit: BaseCumulativePattern,
    pub sent_in_profit_14d_ema: BtcSatsUsdPattern,
    pub sopr: MetricPattern1<StoredF64>,
    pub sopr_1y: MetricPattern1<StoredF64>,
    pub sopr_24h: MetricPattern1<StoredF64>,
    pub sopr_24h_30d_ema: MetricPattern1<StoredF64>,
    pub sopr_24h_7d_ema: MetricPattern1<StoredF64>,
    pub sopr_30d: MetricPattern1<StoredF64>,
    pub sopr_30d_ema: MetricPattern1<StoredF64>,
    pub sopr_7d: MetricPattern1<StoredF64>,
    pub sopr_7d_ema: MetricPattern1<StoredF64>,
    pub total_realized_pnl: MetricPattern1<Dollars>,
    pub upper_price_band: SatsUsdPattern,
    pub value_created: MetricPattern1<Dollars>,
    pub value_created_1y: MetricPattern1<Dollars>,
    pub value_created_24h: MetricPattern1<Dollars>,
    pub value_created_30d: MetricPattern1<Dollars>,
    pub value_created_7d: MetricPattern1<Dollars>,
    pub value_destroyed: MetricPattern1<Dollars>,
    pub value_destroyed_1y: MetricPattern1<Dollars>,
    pub value_destroyed_24h: MetricPattern1<Dollars>,
    pub value_destroyed_30d: MetricPattern1<Dollars>,
    pub value_destroyed_7d: MetricPattern1<Dollars>,
}

impl CapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            cap_raw: MetricPattern20::new(client.clone(), _m(&acc, "cap_raw")),
            capitulation_flow: MetricPattern1::new(client.clone(), _m(&acc, "capitulation_flow")),
            investor_cap_raw: MetricPattern20::new(client.clone(), _m(&acc, "investor_cap_raw")),
            investor_price: SatsUsdPattern::new(client.clone(), _m(&acc, "investor_price")),
            investor_price_cents: MetricPattern1::new(client.clone(), _m(&acc, "investor_price_cents")),
            investor_price_extra: RatioPattern2::new(client.clone(), _m(&acc, "investor_price_ratio")),
            loss_value_created: MetricPattern1::new(client.clone(), _m(&acc, "loss_value_created")),
            loss_value_destroyed: MetricPattern1::new(client.clone(), _m(&acc, "loss_value_destroyed")),
            lower_price_band: SatsUsdPattern::new(client.clone(), _m(&acc, "lower_price_band")),
            mvrv: MetricPattern1::new(client.clone(), _m(&acc, "mvrv")),
            neg_realized_loss: MetricPattern1::new(client.clone(), _m(&acc, "neg_realized_loss")),
            net_realized_pnl: CumulativeHeightPattern::new(client.clone(), _m(&acc, "net_realized_pnl")),
            net_realized_pnl_7d_ema: MetricPattern1::new(client.clone(), _m(&acc, "net_realized_pnl_7d_ema")),
            net_realized_pnl_cumulative_30d_delta: MetricPattern1::new(client.clone(), _m(&acc, "net_realized_pnl_cumulative_30d_delta")),
            net_realized_pnl_cumulative_30d_delta_rel_to_market_cap: MetricPattern1::new(client.clone(), _m(&acc, "net_realized_pnl_cumulative_30d_delta_rel_to_market_cap")),
            net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap: MetricPattern1::new(client.clone(), _m(&acc, "net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap")),
            net_realized_pnl_rel_to_realized_cap: MetricPattern1::new(client.clone(), _m(&acc, "net_realized_pnl_rel_to_realized_cap")),
            peak_regret: CumulativeHeightPattern::new(client.clone(), _m(&acc, "realized_peak_regret")),
            peak_regret_rel_to_realized_cap: MetricPattern1::new(client.clone(), _m(&acc, "peak_regret_rel_to_realized_cap")),
            profit_flow: MetricPattern1::new(client.clone(), _m(&acc, "profit_flow")),
            profit_value_created: MetricPattern1::new(client.clone(), _m(&acc, "profit_value_created")),
            profit_value_destroyed: MetricPattern1::new(client.clone(), _m(&acc, "profit_value_destroyed")),
            realized_cap: MetricPattern1::new(client.clone(), _m(&acc, "realized_cap")),
            realized_cap_30d_delta: MetricPattern1::new(client.clone(), _m(&acc, "realized_cap_30d_delta")),
            realized_cap_cents: MetricPattern1::new(client.clone(), _m(&acc, "realized_cap_cents")),
            realized_loss: CumulativeHeightPattern::new(client.clone(), _m(&acc, "realized_loss")),
            realized_loss_7d_ema: MetricPattern1::new(client.clone(), _m(&acc, "realized_loss_7d_ema")),
            realized_loss_rel_to_realized_cap: MetricPattern1::new(client.clone(), _m(&acc, "realized_loss_rel_to_realized_cap")),
            realized_price: SatsUsdPattern::new(client.clone(), _m(&acc, "realized_price")),
            realized_price_extra: RatioPattern2::new(client.clone(), _m(&acc, "realized_price_ratio")),
            realized_profit: CumulativeHeightPattern::new(client.clone(), _m(&acc, "realized_profit")),
            realized_profit_7d_ema: MetricPattern1::new(client.clone(), _m(&acc, "realized_profit_7d_ema")),
            realized_profit_rel_to_realized_cap: MetricPattern1::new(client.clone(), _m(&acc, "realized_profit_rel_to_realized_cap")),
            realized_value: MetricPattern1::new(client.clone(), _m(&acc, "realized_value")),
            realized_value_1y: MetricPattern1::new(client.clone(), _m(&acc, "realized_value_1y")),
            realized_value_24h: MetricPattern1::new(client.clone(), _m(&acc, "realized_value_24h")),
            realized_value_30d: MetricPattern1::new(client.clone(), _m(&acc, "realized_value_30d")),
            realized_value_7d: MetricPattern1::new(client.clone(), _m(&acc, "realized_value_7d")),
            sell_side_risk_ratio: MetricPattern1::new(client.clone(), _m(&acc, "sell_side_risk_ratio")),
            sell_side_risk_ratio_1y: MetricPattern1::new(client.clone(), _m(&acc, "sell_side_risk_ratio_1y")),
            sell_side_risk_ratio_24h: MetricPattern1::new(client.clone(), _m(&acc, "sell_side_risk_ratio_24h")),
            sell_side_risk_ratio_24h_30d_ema: MetricPattern1::new(client.clone(), _m(&acc, "sell_side_risk_ratio_24h_30d_ema")),
            sell_side_risk_ratio_24h_7d_ema: MetricPattern1::new(client.clone(), _m(&acc, "sell_side_risk_ratio_24h_7d_ema")),
            sell_side_risk_ratio_30d: MetricPattern1::new(client.clone(), _m(&acc, "sell_side_risk_ratio_30d")),
            sell_side_risk_ratio_30d_ema: MetricPattern1::new(client.clone(), _m(&acc, "sell_side_risk_ratio_30d_ema")),
            sell_side_risk_ratio_7d: MetricPattern1::new(client.clone(), _m(&acc, "sell_side_risk_ratio_7d")),
            sell_side_risk_ratio_7d_ema: MetricPattern1::new(client.clone(), _m(&acc, "sell_side_risk_ratio_7d_ema")),
            sent_in_loss: BaseCumulativePattern::new(client.clone(), _m(&acc, "sent_in_loss")),
            sent_in_loss_14d_ema: BtcSatsUsdPattern::new(client.clone(), _m(&acc, "sent_in_loss_14d_ema")),
            sent_in_profit: BaseCumulativePattern::new(client.clone(), _m(&acc, "sent_in_profit")),
            sent_in_profit_14d_ema: BtcSatsUsdPattern::new(client.clone(), _m(&acc, "sent_in_profit_14d_ema")),
            sopr: MetricPattern1::new(client.clone(), _m(&acc, "sopr")),
            sopr_1y: MetricPattern1::new(client.clone(), _m(&acc, "sopr_1y")),
            sopr_24h: MetricPattern1::new(client.clone(), _m(&acc, "sopr_24h")),
            sopr_24h_30d_ema: MetricPattern1::new(client.clone(), _m(&acc, "sopr_24h_30d_ema")),
            sopr_24h_7d_ema: MetricPattern1::new(client.clone(), _m(&acc, "sopr_24h_7d_ema")),
            sopr_30d: MetricPattern1::new(client.clone(), _m(&acc, "sopr_30d")),
            sopr_30d_ema: MetricPattern1::new(client.clone(), _m(&acc, "sopr_30d_ema")),
            sopr_7d: MetricPattern1::new(client.clone(), _m(&acc, "sopr_7d")),
            sopr_7d_ema: MetricPattern1::new(client.clone(), _m(&acc, "sopr_7d_ema")),
            total_realized_pnl: MetricPattern1::new(client.clone(), _m(&acc, "total_realized_pnl")),
            upper_price_band: SatsUsdPattern::new(client.clone(), _m(&acc, "upper_price_band")),
            value_created: MetricPattern1::new(client.clone(), _m(&acc, "value_created")),
            value_created_1y: MetricPattern1::new(client.clone(), _m(&acc, "value_created_1y")),
            value_created_24h: MetricPattern1::new(client.clone(), _m(&acc, "value_created_24h")),
            value_created_30d: MetricPattern1::new(client.clone(), _m(&acc, "value_created_30d")),
            value_created_7d: MetricPattern1::new(client.clone(), _m(&acc, "value_created_7d")),
            value_destroyed: MetricPattern1::new(client.clone(), _m(&acc, "value_destroyed")),
            value_destroyed_1y: MetricPattern1::new(client.clone(), _m(&acc, "value_destroyed_1y")),
            value_destroyed_24h: MetricPattern1::new(client.clone(), _m(&acc, "value_destroyed_24h")),
            value_destroyed_30d: MetricPattern1::new(client.clone(), _m(&acc, "value_destroyed_30d")),
            value_destroyed_7d: MetricPattern1::new(client.clone(), _m(&acc, "value_destroyed_7d")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern {
    pub _0sd_usd: SatsUsdPattern,
    pub m0_5sd: MetricPattern1<StoredF32>,
    pub m0_5sd_usd: SatsUsdPattern,
    pub m1_5sd: MetricPattern1<StoredF32>,
    pub m1_5sd_usd: SatsUsdPattern,
    pub m1sd: MetricPattern1<StoredF32>,
    pub m1sd_usd: SatsUsdPattern,
    pub m2_5sd: MetricPattern1<StoredF32>,
    pub m2_5sd_usd: SatsUsdPattern,
    pub m2sd: MetricPattern1<StoredF32>,
    pub m2sd_usd: SatsUsdPattern,
    pub m3sd: MetricPattern1<StoredF32>,
    pub m3sd_usd: SatsUsdPattern,
    pub p0_5sd: MetricPattern1<StoredF32>,
    pub p0_5sd_usd: SatsUsdPattern,
    pub p1_5sd: MetricPattern1<StoredF32>,
    pub p1_5sd_usd: SatsUsdPattern,
    pub p1sd: MetricPattern1<StoredF32>,
    pub p1sd_usd: SatsUsdPattern,
    pub p2_5sd: MetricPattern1<StoredF32>,
    pub p2_5sd_usd: SatsUsdPattern,
    pub p2sd: MetricPattern1<StoredF32>,
    pub p2sd_usd: SatsUsdPattern,
    pub p3sd: MetricPattern1<StoredF32>,
    pub p3sd_usd: SatsUsdPattern,
    pub sd: MetricPattern1<StoredF32>,
    pub sma: MetricPattern1<StoredF32>,
    pub zscore: MetricPattern1<StoredF32>,
}

impl _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            _0sd_usd: SatsUsdPattern::new(client.clone(), _m(&acc, "0sd_usd")),
            m0_5sd: MetricPattern1::new(client.clone(), _m(&acc, "m0_5sd")),
            m0_5sd_usd: SatsUsdPattern::new(client.clone(), _m(&acc, "m0_5sd_usd")),
            m1_5sd: MetricPattern1::new(client.clone(), _m(&acc, "m1_5sd")),
            m1_5sd_usd: SatsUsdPattern::new(client.clone(), _m(&acc, "m1_5sd_usd")),
            m1sd: MetricPattern1::new(client.clone(), _m(&acc, "m1sd")),
            m1sd_usd: SatsUsdPattern::new(client.clone(), _m(&acc, "m1sd_usd")),
            m2_5sd: MetricPattern1::new(client.clone(), _m(&acc, "m2_5sd")),
            m2_5sd_usd: SatsUsdPattern::new(client.clone(), _m(&acc, "m2_5sd_usd")),
            m2sd: MetricPattern1::new(client.clone(), _m(&acc, "m2sd")),
            m2sd_usd: SatsUsdPattern::new(client.clone(), _m(&acc, "m2sd_usd")),
            m3sd: MetricPattern1::new(client.clone(), _m(&acc, "m3sd")),
            m3sd_usd: SatsUsdPattern::new(client.clone(), _m(&acc, "m3sd_usd")),
            p0_5sd: MetricPattern1::new(client.clone(), _m(&acc, "p0_5sd")),
            p0_5sd_usd: SatsUsdPattern::new(client.clone(), _m(&acc, "p0_5sd_usd")),
            p1_5sd: MetricPattern1::new(client.clone(), _m(&acc, "p1_5sd")),
            p1_5sd_usd: SatsUsdPattern::new(client.clone(), _m(&acc, "p1_5sd_usd")),
            p1sd: MetricPattern1::new(client.clone(), _m(&acc, "p1sd")),
            p1sd_usd: SatsUsdPattern::new(client.clone(), _m(&acc, "p1sd_usd")),
            p2_5sd: MetricPattern1::new(client.clone(), _m(&acc, "p2_5sd")),
            p2_5sd_usd: SatsUsdPattern::new(client.clone(), _m(&acc, "p2_5sd_usd")),
            p2sd: MetricPattern1::new(client.clone(), _m(&acc, "p2sd")),
            p2sd_usd: SatsUsdPattern::new(client.clone(), _m(&acc, "p2sd_usd")),
            p3sd: MetricPattern1::new(client.clone(), _m(&acc, "p3sd")),
            p3sd_usd: SatsUsdPattern::new(client.clone(), _m(&acc, "p3sd_usd")),
            sd: MetricPattern1::new(client.clone(), _m(&acc, "sd")),
            sma: MetricPattern1::new(client.clone(), _m(&acc, "sma")),
            zscore: MetricPattern1::new(client.clone(), _m(&acc, "zscore")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct InvestedNegNetNuplSupplyUnrealizedPattern2 {
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
    pub supply_rel_to_circulating_supply: MetricPattern1<StoredF64>,
    pub unrealized_loss_rel_to_market_cap: MetricPattern1<StoredF32>,
    pub unrealized_loss_rel_to_own_market_cap: MetricPattern1<StoredF32>,
    pub unrealized_loss_rel_to_own_total_unrealized_pnl: MetricPattern1<StoredF32>,
    pub unrealized_peak_regret_rel_to_market_cap: MetricPattern1<StoredF32>,
    pub unrealized_profit_rel_to_market_cap: MetricPattern1<StoredF32>,
    pub unrealized_profit_rel_to_own_market_cap: MetricPattern1<StoredF32>,
    pub unrealized_profit_rel_to_own_total_unrealized_pnl: MetricPattern1<StoredF32>,
}

impl InvestedNegNetNuplSupplyUnrealizedPattern2 {
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
            supply_rel_to_circulating_supply: MetricPattern1::new(client.clone(), _m(&acc, "supply_rel_to_circulating_supply")),
            unrealized_loss_rel_to_market_cap: MetricPattern1::new(client.clone(), _m(&acc, "unrealized_loss_rel_to_market_cap")),
            unrealized_loss_rel_to_own_market_cap: MetricPattern1::new(client.clone(), _m(&acc, "unrealized_loss_rel_to_own_market_cap")),
            unrealized_loss_rel_to_own_total_unrealized_pnl: MetricPattern1::new(client.clone(), _m(&acc, "unrealized_loss_rel_to_own_total_unrealized_pnl")),
            unrealized_peak_regret_rel_to_market_cap: MetricPattern1::new(client.clone(), _m(&acc, "unrealized_peak_regret_rel_to_market_cap")),
            unrealized_profit_rel_to_market_cap: MetricPattern1::new(client.clone(), _m(&acc, "unrealized_profit_rel_to_market_cap")),
            unrealized_profit_rel_to_own_market_cap: MetricPattern1::new(client.clone(), _m(&acc, "unrealized_profit_rel_to_own_market_cap")),
            unrealized_profit_rel_to_own_total_unrealized_pnl: MetricPattern1::new(client.clone(), _m(&acc, "unrealized_profit_rel_to_own_total_unrealized_pnl")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct PriceRatioPattern {
    pub price: SatsUsdPattern,
    pub ratio: MetricPattern1<StoredF32>,
    pub ratio_1m_sma: MetricPattern1<StoredF32>,
    pub ratio_1w_sma: MetricPattern1<StoredF32>,
    pub ratio_1y_sd: _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern,
    pub ratio_2y_sd: _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern,
    pub ratio_4y_sd: _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern,
    pub ratio_pct1: MetricPattern1<StoredF32>,
    pub ratio_pct1_usd: SatsUsdPattern,
    pub ratio_pct2: MetricPattern1<StoredF32>,
    pub ratio_pct2_usd: SatsUsdPattern,
    pub ratio_pct5: MetricPattern1<StoredF32>,
    pub ratio_pct5_usd: SatsUsdPattern,
    pub ratio_pct95: MetricPattern1<StoredF32>,
    pub ratio_pct95_usd: SatsUsdPattern,
    pub ratio_pct98: MetricPattern1<StoredF32>,
    pub ratio_pct98_usd: SatsUsdPattern,
    pub ratio_pct99: MetricPattern1<StoredF32>,
    pub ratio_pct99_usd: SatsUsdPattern,
    pub ratio_sd: _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern,
}

impl PriceRatioPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            price: SatsUsdPattern::new(client.clone(), acc.clone()),
            ratio: MetricPattern1::new(client.clone(), _m(&acc, "ratio")),
            ratio_1m_sma: MetricPattern1::new(client.clone(), _m(&acc, "ratio_1m_sma")),
            ratio_1w_sma: MetricPattern1::new(client.clone(), _m(&acc, "ratio_1w_sma")),
            ratio_1y_sd: _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern::new(client.clone(), _m(&acc, "ratio_1y")),
            ratio_2y_sd: _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern::new(client.clone(), _m(&acc, "ratio_2y")),
            ratio_4y_sd: _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern::new(client.clone(), _m(&acc, "ratio_4y")),
            ratio_pct1: MetricPattern1::new(client.clone(), _m(&acc, "ratio_pct1")),
            ratio_pct1_usd: SatsUsdPattern::new(client.clone(), _m(&acc, "ratio_pct1_usd")),
            ratio_pct2: MetricPattern1::new(client.clone(), _m(&acc, "ratio_pct2")),
            ratio_pct2_usd: SatsUsdPattern::new(client.clone(), _m(&acc, "ratio_pct2_usd")),
            ratio_pct5: MetricPattern1::new(client.clone(), _m(&acc, "ratio_pct5")),
            ratio_pct5_usd: SatsUsdPattern::new(client.clone(), _m(&acc, "ratio_pct5_usd")),
            ratio_pct95: MetricPattern1::new(client.clone(), _m(&acc, "ratio_pct95")),
            ratio_pct95_usd: SatsUsdPattern::new(client.clone(), _m(&acc, "ratio_pct95_usd")),
            ratio_pct98: MetricPattern1::new(client.clone(), _m(&acc, "ratio_pct98")),
            ratio_pct98_usd: SatsUsdPattern::new(client.clone(), _m(&acc, "ratio_pct98_usd")),
            ratio_pct99: MetricPattern1::new(client.clone(), _m(&acc, "ratio_pct99")),
            ratio_pct99_usd: SatsUsdPattern::new(client.clone(), _m(&acc, "ratio_pct99_usd")),
            ratio_sd: _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern::new(client.clone(), _m(&acc, "ratio")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct Pct05Pct10Pct15Pct20Pct25Pct30Pct35Pct40Pct45Pct50Pct55Pct60Pct65Pct70Pct75Pct80Pct85Pct90Pct95Pattern {
    pub pct05: SatsUsdPattern,
    pub pct10: SatsUsdPattern,
    pub pct15: SatsUsdPattern,
    pub pct20: SatsUsdPattern,
    pub pct25: SatsUsdPattern,
    pub pct30: SatsUsdPattern,
    pub pct35: SatsUsdPattern,
    pub pct40: SatsUsdPattern,
    pub pct45: SatsUsdPattern,
    pub pct50: SatsUsdPattern,
    pub pct55: SatsUsdPattern,
    pub pct60: SatsUsdPattern,
    pub pct65: SatsUsdPattern,
    pub pct70: SatsUsdPattern,
    pub pct75: SatsUsdPattern,
    pub pct80: SatsUsdPattern,
    pub pct85: SatsUsdPattern,
    pub pct90: SatsUsdPattern,
    pub pct95: SatsUsdPattern,
}

impl Pct05Pct10Pct15Pct20Pct25Pct30Pct35Pct40Pct45Pct50Pct55Pct60Pct65Pct70Pct75Pct80Pct85Pct90Pct95Pattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            pct05: SatsUsdPattern::new(client.clone(), _m(&acc, "pct05")),
            pct10: SatsUsdPattern::new(client.clone(), _m(&acc, "pct10")),
            pct15: SatsUsdPattern::new(client.clone(), _m(&acc, "pct15")),
            pct20: SatsUsdPattern::new(client.clone(), _m(&acc, "pct20")),
            pct25: SatsUsdPattern::new(client.clone(), _m(&acc, "pct25")),
            pct30: SatsUsdPattern::new(client.clone(), _m(&acc, "pct30")),
            pct35: SatsUsdPattern::new(client.clone(), _m(&acc, "pct35")),
            pct40: SatsUsdPattern::new(client.clone(), _m(&acc, "pct40")),
            pct45: SatsUsdPattern::new(client.clone(), _m(&acc, "pct45")),
            pct50: SatsUsdPattern::new(client.clone(), _m(&acc, "pct50")),
            pct55: SatsUsdPattern::new(client.clone(), _m(&acc, "pct55")),
            pct60: SatsUsdPattern::new(client.clone(), _m(&acc, "pct60")),
            pct65: SatsUsdPattern::new(client.clone(), _m(&acc, "pct65")),
            pct70: SatsUsdPattern::new(client.clone(), _m(&acc, "pct70")),
            pct75: SatsUsdPattern::new(client.clone(), _m(&acc, "pct75")),
            pct80: SatsUsdPattern::new(client.clone(), _m(&acc, "pct80")),
            pct85: SatsUsdPattern::new(client.clone(), _m(&acc, "pct85")),
            pct90: SatsUsdPattern::new(client.clone(), _m(&acc, "pct90")),
            pct95: SatsUsdPattern::new(client.clone(), _m(&acc, "pct95")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct RatioPattern {
    pub ratio: MetricPattern1<StoredF32>,
    pub ratio_1m_sma: MetricPattern1<StoredF32>,
    pub ratio_1w_sma: MetricPattern1<StoredF32>,
    pub ratio_1y_sd: _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern,
    pub ratio_2y_sd: _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern,
    pub ratio_4y_sd: _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern,
    pub ratio_pct1: MetricPattern1<StoredF32>,
    pub ratio_pct1_usd: SatsUsdPattern,
    pub ratio_pct2: MetricPattern1<StoredF32>,
    pub ratio_pct2_usd: SatsUsdPattern,
    pub ratio_pct5: MetricPattern1<StoredF32>,
    pub ratio_pct5_usd: SatsUsdPattern,
    pub ratio_pct95: MetricPattern1<StoredF32>,
    pub ratio_pct95_usd: SatsUsdPattern,
    pub ratio_pct98: MetricPattern1<StoredF32>,
    pub ratio_pct98_usd: SatsUsdPattern,
    pub ratio_pct99: MetricPattern1<StoredF32>,
    pub ratio_pct99_usd: SatsUsdPattern,
    pub ratio_sd: _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern,
}

impl RatioPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            ratio: MetricPattern1::new(client.clone(), acc.clone()),
            ratio_1m_sma: MetricPattern1::new(client.clone(), _m(&acc, "1m_sma")),
            ratio_1w_sma: MetricPattern1::new(client.clone(), _m(&acc, "1w_sma")),
            ratio_1y_sd: _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern::new(client.clone(), _m(&acc, "1y")),
            ratio_2y_sd: _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern::new(client.clone(), _m(&acc, "2y")),
            ratio_4y_sd: _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern::new(client.clone(), _m(&acc, "4y")),
            ratio_pct1: MetricPattern1::new(client.clone(), _m(&acc, "pct1")),
            ratio_pct1_usd: SatsUsdPattern::new(client.clone(), _m(&acc, "pct1_usd")),
            ratio_pct2: MetricPattern1::new(client.clone(), _m(&acc, "pct2")),
            ratio_pct2_usd: SatsUsdPattern::new(client.clone(), _m(&acc, "pct2_usd")),
            ratio_pct5: MetricPattern1::new(client.clone(), _m(&acc, "pct5")),
            ratio_pct5_usd: SatsUsdPattern::new(client.clone(), _m(&acc, "pct5_usd")),
            ratio_pct95: MetricPattern1::new(client.clone(), _m(&acc, "pct95")),
            ratio_pct95_usd: SatsUsdPattern::new(client.clone(), _m(&acc, "pct95_usd")),
            ratio_pct98: MetricPattern1::new(client.clone(), _m(&acc, "pct98")),
            ratio_pct98_usd: SatsUsdPattern::new(client.clone(), _m(&acc, "pct98_usd")),
            ratio_pct99: MetricPattern1::new(client.clone(), _m(&acc, "pct99")),
            ratio_pct99_usd: SatsUsdPattern::new(client.clone(), _m(&acc, "pct99_usd")),
            ratio_sd: _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern::new(client.clone(), acc.clone()),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct RatioPattern3 {
    pub ratio_1m_sma: MetricPattern1<StoredF32>,
    pub ratio_1w_sma: MetricPattern1<StoredF32>,
    pub ratio_1y_sd: _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern,
    pub ratio_2y_sd: _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern,
    pub ratio_4y_sd: _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern,
    pub ratio_pct1: MetricPattern1<StoredF32>,
    pub ratio_pct1_usd: SatsUsdPattern,
    pub ratio_pct2: MetricPattern1<StoredF32>,
    pub ratio_pct2_usd: SatsUsdPattern,
    pub ratio_pct5: MetricPattern1<StoredF32>,
    pub ratio_pct5_usd: SatsUsdPattern,
    pub ratio_pct95: MetricPattern1<StoredF32>,
    pub ratio_pct95_usd: SatsUsdPattern,
    pub ratio_pct98: MetricPattern1<StoredF32>,
    pub ratio_pct98_usd: SatsUsdPattern,
    pub ratio_pct99: MetricPattern1<StoredF32>,
    pub ratio_pct99_usd: SatsUsdPattern,
    pub ratio_sd: _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern,
}

impl RatioPattern3 {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            ratio_1m_sma: MetricPattern1::new(client.clone(), _m(&acc, "1m_sma")),
            ratio_1w_sma: MetricPattern1::new(client.clone(), _m(&acc, "1w_sma")),
            ratio_1y_sd: _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern::new(client.clone(), _m(&acc, "1y")),
            ratio_2y_sd: _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern::new(client.clone(), _m(&acc, "2y")),
            ratio_4y_sd: _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern::new(client.clone(), _m(&acc, "4y")),
            ratio_pct1: MetricPattern1::new(client.clone(), _m(&acc, "pct1")),
            ratio_pct1_usd: SatsUsdPattern::new(client.clone(), _m(&acc, "pct1_usd")),
            ratio_pct2: MetricPattern1::new(client.clone(), _m(&acc, "pct2")),
            ratio_pct2_usd: SatsUsdPattern::new(client.clone(), _m(&acc, "pct2_usd")),
            ratio_pct5: MetricPattern1::new(client.clone(), _m(&acc, "pct5")),
            ratio_pct5_usd: SatsUsdPattern::new(client.clone(), _m(&acc, "pct5_usd")),
            ratio_pct95: MetricPattern1::new(client.clone(), _m(&acc, "pct95")),
            ratio_pct95_usd: SatsUsdPattern::new(client.clone(), _m(&acc, "pct95_usd")),
            ratio_pct98: MetricPattern1::new(client.clone(), _m(&acc, "pct98")),
            ratio_pct98_usd: SatsUsdPattern::new(client.clone(), _m(&acc, "pct98_usd")),
            ratio_pct99: MetricPattern1::new(client.clone(), _m(&acc, "pct99")),
            ratio_pct99_usd: SatsUsdPattern::new(client.clone(), _m(&acc, "pct99_usd")),
            ratio_sd: _0sdM0M1M1sdM2M2sdM3sdP0P1P1sdP2P2sdP3sdSdSmaZscorePattern::new(client.clone(), acc.clone()),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct GreedInvestedInvestorNegNetPainPeakSupplyTotalUnrealizedPattern {
    pub greed_index: MetricPattern1<Dollars>,
    pub invested_capital_in_loss: MetricPattern1<Dollars>,
    pub invested_capital_in_loss_raw: MetricPattern20<CentsSats>,
    pub invested_capital_in_profit: MetricPattern1<Dollars>,
    pub invested_capital_in_profit_raw: MetricPattern20<CentsSats>,
    pub investor_cap_in_loss_raw: MetricPattern20<CentsSquaredSats>,
    pub investor_cap_in_profit_raw: MetricPattern20<CentsSquaredSats>,
    pub neg_unrealized_loss: MetricPattern1<Dollars>,
    pub net_sentiment: MetricPattern1<Dollars>,
    pub net_unrealized_pnl: MetricPattern1<Dollars>,
    pub pain_index: MetricPattern1<Dollars>,
    pub peak_regret: MetricPattern1<Dollars>,
    pub supply_in_loss: BtcSatsUsdPattern,
    pub supply_in_profit: BtcSatsUsdPattern,
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
            invested_capital_in_loss_raw: MetricPattern20::new(client.clone(), _m(&acc, "invested_capital_in_loss_raw")),
            invested_capital_in_profit: MetricPattern1::new(client.clone(), _m(&acc, "invested_capital_in_profit")),
            invested_capital_in_profit_raw: MetricPattern20::new(client.clone(), _m(&acc, "invested_capital_in_profit_raw")),
            investor_cap_in_loss_raw: MetricPattern20::new(client.clone(), _m(&acc, "investor_cap_in_loss_raw")),
            investor_cap_in_profit_raw: MetricPattern20::new(client.clone(), _m(&acc, "investor_cap_in_profit_raw")),
            neg_unrealized_loss: MetricPattern1::new(client.clone(), _m(&acc, "neg_unrealized_loss")),
            net_sentiment: MetricPattern1::new(client.clone(), _m(&acc, "net_sentiment")),
            net_unrealized_pnl: MetricPattern1::new(client.clone(), _m(&acc, "net_unrealized_pnl")),
            pain_index: MetricPattern1::new(client.clone(), _m(&acc, "pain_index")),
            peak_regret: MetricPattern1::new(client.clone(), _m(&acc, "unrealized_peak_regret")),
            supply_in_loss: BtcSatsUsdPattern::new(client.clone(), _m(&acc, "supply_in_loss")),
            supply_in_profit: BtcSatsUsdPattern::new(client.clone(), _m(&acc, "supply_in_profit")),
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
    pub invested_capital_in_loss_raw: MetricPattern20<CentsSats>,
    pub invested_capital_in_profit: MetricPattern1<Dollars>,
    pub invested_capital_in_profit_raw: MetricPattern20<CentsSats>,
    pub investor_cap_in_loss_raw: MetricPattern20<CentsSquaredSats>,
    pub investor_cap_in_profit_raw: MetricPattern20<CentsSquaredSats>,
    pub neg_unrealized_loss: MetricPattern1<Dollars>,
    pub net_sentiment: MetricPattern1<Dollars>,
    pub net_unrealized_pnl: MetricPattern1<Dollars>,
    pub pain_index: MetricPattern1<Dollars>,
    pub supply_in_loss: BtcSatsUsdPattern,
    pub supply_in_profit: BtcSatsUsdPattern,
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
            invested_capital_in_loss_raw: MetricPattern20::new(client.clone(), _m(&acc, "invested_capital_in_loss_raw")),
            invested_capital_in_profit: MetricPattern1::new(client.clone(), _m(&acc, "invested_capital_in_profit")),
            invested_capital_in_profit_raw: MetricPattern20::new(client.clone(), _m(&acc, "invested_capital_in_profit_raw")),
            investor_cap_in_loss_raw: MetricPattern20::new(client.clone(), _m(&acc, "investor_cap_in_loss_raw")),
            investor_cap_in_profit_raw: MetricPattern20::new(client.clone(), _m(&acc, "investor_cap_in_profit_raw")),
            neg_unrealized_loss: MetricPattern1::new(client.clone(), _m(&acc, "neg_unrealized_loss")),
            net_sentiment: MetricPattern1::new(client.clone(), _m(&acc, "net_sentiment")),
            net_unrealized_pnl: MetricPattern1::new(client.clone(), _m(&acc, "net_unrealized_pnl")),
            pain_index: MetricPattern1::new(client.clone(), _m(&acc, "pain_index")),
            supply_in_loss: BtcSatsUsdPattern::new(client.clone(), _m(&acc, "supply_in_loss")),
            supply_in_profit: BtcSatsUsdPattern::new(client.clone(), _m(&acc, "supply_in_profit")),
            total_unrealized_pnl: MetricPattern1::new(client.clone(), _m(&acc, "total_unrealized_pnl")),
            unrealized_loss: MetricPattern1::new(client.clone(), _m(&acc, "unrealized_loss")),
            unrealized_profit: MetricPattern1::new(client.clone(), _m(&acc, "unrealized_profit")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct BlocksCoinbaseDaysDominanceFeeSubsidyPattern {
    pub blocks_mined: CumulativeHeightSumPattern<StoredU32>,
    pub blocks_mined_1m_sum: MetricPattern1<StoredU32>,
    pub blocks_mined_1w_sum: MetricPattern1<StoredU32>,
    pub blocks_mined_1y_sum: MetricPattern1<StoredU32>,
    pub blocks_mined_24h_sum: MetricPattern1<StoredU32>,
    pub blocks_since_block: MetricPattern1<StoredU32>,
    pub coinbase: BaseCumulativeSumPattern,
    pub days_since_block: MetricPattern1<StoredU16>,
    pub dominance: MetricPattern1<StoredF32>,
    pub dominance_1m: MetricPattern1<StoredF32>,
    pub dominance_1w: MetricPattern1<StoredF32>,
    pub dominance_1y: MetricPattern1<StoredF32>,
    pub dominance_24h: MetricPattern1<StoredF32>,
    pub fee: BaseCumulativeSumPattern,
    pub subsidy: BaseCumulativeSumPattern,
}

impl BlocksCoinbaseDaysDominanceFeeSubsidyPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            blocks_mined: CumulativeHeightSumPattern::new(client.clone(), _m(&acc, "blocks_mined")),
            blocks_mined_1m_sum: MetricPattern1::new(client.clone(), _m(&acc, "blocks_mined_1m_sum")),
            blocks_mined_1w_sum: MetricPattern1::new(client.clone(), _m(&acc, "blocks_mined_1w_sum")),
            blocks_mined_1y_sum: MetricPattern1::new(client.clone(), _m(&acc, "blocks_mined_1y_sum")),
            blocks_mined_24h_sum: MetricPattern1::new(client.clone(), _m(&acc, "blocks_mined_24h_sum")),
            blocks_since_block: MetricPattern1::new(client.clone(), _m(&acc, "blocks_since_block")),
            coinbase: BaseCumulativeSumPattern::new(client.clone(), _m(&acc, "coinbase")),
            days_since_block: MetricPattern1::new(client.clone(), _m(&acc, "days_since_block")),
            dominance: MetricPattern1::new(client.clone(), _m(&acc, "dominance")),
            dominance_1m: MetricPattern1::new(client.clone(), _m(&acc, "dominance_1m")),
            dominance_1w: MetricPattern1::new(client.clone(), _m(&acc, "dominance_1w")),
            dominance_1y: MetricPattern1::new(client.clone(), _m(&acc, "dominance_1y")),
            dominance_24h: MetricPattern1::new(client.clone(), _m(&acc, "dominance_24h")),
            fee: BaseCumulativeSumPattern::new(client.clone(), _m(&acc, "fee")),
            subsidy: BaseCumulativeSumPattern::new(client.clone(), _m(&acc, "subsidy")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct InvestedNegNetNuplSupplyUnrealizedPattern4 {
    pub invested_capital_in_loss_pct: MetricPattern1<StoredF32>,
    pub invested_capital_in_profit_pct: MetricPattern1<StoredF32>,
    pub neg_unrealized_loss_rel_to_market_cap: MetricPattern1<StoredF32>,
    pub net_unrealized_pnl_rel_to_market_cap: MetricPattern1<StoredF32>,
    pub nupl: MetricPattern1<StoredF32>,
    pub supply_in_loss_rel_to_circulating_supply: MetricPattern1<StoredF64>,
    pub supply_in_loss_rel_to_own_supply: MetricPattern1<StoredF64>,
    pub supply_in_profit_rel_to_circulating_supply: MetricPattern1<StoredF64>,
    pub supply_in_profit_rel_to_own_supply: MetricPattern1<StoredF64>,
    pub supply_rel_to_circulating_supply: MetricPattern1<StoredF64>,
    pub unrealized_loss_rel_to_market_cap: MetricPattern1<StoredF32>,
    pub unrealized_peak_regret_rel_to_market_cap: MetricPattern1<StoredF32>,
    pub unrealized_profit_rel_to_market_cap: MetricPattern1<StoredF32>,
}

impl InvestedNegNetNuplSupplyUnrealizedPattern4 {
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
            supply_rel_to_circulating_supply: MetricPattern1::new(client.clone(), _m(&acc, "supply_rel_to_circulating_supply")),
            unrealized_loss_rel_to_market_cap: MetricPattern1::new(client.clone(), _m(&acc, "unrealized_loss_rel_to_market_cap")),
            unrealized_peak_regret_rel_to_market_cap: MetricPattern1::new(client.clone(), _m(&acc, "unrealized_peak_regret_rel_to_market_cap")),
            unrealized_profit_rel_to_market_cap: MetricPattern1::new(client.clone(), _m(&acc, "unrealized_profit_rel_to_market_cap")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct _10y1m1w1y2y3m3y4y5y6m6y8yPattern3 {
    pub _10y: BtcSatsUsdPattern,
    pub _1m: BtcSatsUsdPattern,
    pub _1w: BtcSatsUsdPattern,
    pub _1y: BtcSatsUsdPattern,
    pub _2y: BtcSatsUsdPattern,
    pub _3m: BtcSatsUsdPattern,
    pub _3y: BtcSatsUsdPattern,
    pub _4y: BtcSatsUsdPattern,
    pub _5y: BtcSatsUsdPattern,
    pub _6m: BtcSatsUsdPattern,
    pub _6y: BtcSatsUsdPattern,
    pub _8y: BtcSatsUsdPattern,
}

impl _10y1m1w1y2y3m3y4y5y6m6y8yPattern3 {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            _10y: BtcSatsUsdPattern::new(client.clone(), _p("10y", &acc)),
            _1m: BtcSatsUsdPattern::new(client.clone(), _p("1m", &acc)),
            _1w: BtcSatsUsdPattern::new(client.clone(), _p("1w", &acc)),
            _1y: BtcSatsUsdPattern::new(client.clone(), _p("1y", &acc)),
            _2y: BtcSatsUsdPattern::new(client.clone(), _p("2y", &acc)),
            _3m: BtcSatsUsdPattern::new(client.clone(), _p("3m", &acc)),
            _3y: BtcSatsUsdPattern::new(client.clone(), _p("3y", &acc)),
            _4y: BtcSatsUsdPattern::new(client.clone(), _p("4y", &acc)),
            _5y: BtcSatsUsdPattern::new(client.clone(), _p("5y", &acc)),
            _6m: BtcSatsUsdPattern::new(client.clone(), _p("6m", &acc)),
            _6y: BtcSatsUsdPattern::new(client.clone(), _p("6y", &acc)),
            _8y: BtcSatsUsdPattern::new(client.clone(), _p("8y", &acc)),
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
    pub supply_rel_to_circulating_supply: MetricPattern1<StoredF64>,
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
            supply_rel_to_circulating_supply: MetricPattern1::new(client.clone(), _m(&acc, "supply_rel_to_circulating_supply")),
            unrealized_loss_rel_to_market_cap: MetricPattern1::new(client.clone(), _m(&acc, "unrealized_loss_rel_to_market_cap")),
            unrealized_profit_rel_to_market_cap: MetricPattern1::new(client.clone(), _m(&acc, "unrealized_profit_rel_to_market_cap")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct _10y1m1w1y2y3m3y4y5y6m6y8yPattern2<T> {
    pub _10y: MetricPattern1<T>,
    pub _1m: MetricPattern1<T>,
    pub _1w: MetricPattern1<T>,
    pub _1y: MetricPattern1<T>,
    pub _2y: MetricPattern1<T>,
    pub _3m: MetricPattern1<T>,
    pub _3y: MetricPattern1<T>,
    pub _4y: MetricPattern1<T>,
    pub _5y: MetricPattern1<T>,
    pub _6m: MetricPattern1<T>,
    pub _6y: MetricPattern1<T>,
    pub _8y: MetricPattern1<T>,
}

impl<T: DeserializeOwned> _10y1m1w1y2y3m3y4y5y6m6y8yPattern2<T> {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            _10y: MetricPattern1::new(client.clone(), _p("10y", &acc)),
            _1m: MetricPattern1::new(client.clone(), _p("1m", &acc)),
            _1w: MetricPattern1::new(client.clone(), _p("1w", &acc)),
            _1y: MetricPattern1::new(client.clone(), _p("1y", &acc)),
            _2y: MetricPattern1::new(client.clone(), _p("2y", &acc)),
            _3m: MetricPattern1::new(client.clone(), _p("3m", &acc)),
            _3y: MetricPattern1::new(client.clone(), _p("3y", &acc)),
            _4y: MetricPattern1::new(client.clone(), _p("4y", &acc)),
            _5y: MetricPattern1::new(client.clone(), _p("5y", &acc)),
            _6m: MetricPattern1::new(client.clone(), _p("6m", &acc)),
            _6y: MetricPattern1::new(client.clone(), _p("6y", &acc)),
            _8y: MetricPattern1::new(client.clone(), _p("8y", &acc)),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct _201520162017201820192020202120222023202420252026Pattern2<T> {
    pub _2015: MetricPattern1<T>,
    pub _2016: MetricPattern1<T>,
    pub _2017: MetricPattern1<T>,
    pub _2018: MetricPattern1<T>,
    pub _2019: MetricPattern1<T>,
    pub _2020: MetricPattern1<T>,
    pub _2021: MetricPattern1<T>,
    pub _2022: MetricPattern1<T>,
    pub _2023: MetricPattern1<T>,
    pub _2024: MetricPattern1<T>,
    pub _2025: MetricPattern1<T>,
    pub _2026: MetricPattern1<T>,
}

impl<T: DeserializeOwned> _201520162017201820192020202120222023202420252026Pattern2<T> {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            _2015: MetricPattern1::new(client.clone(), _m(&acc, "2015_returns")),
            _2016: MetricPattern1::new(client.clone(), _m(&acc, "2016_returns")),
            _2017: MetricPattern1::new(client.clone(), _m(&acc, "2017_returns")),
            _2018: MetricPattern1::new(client.clone(), _m(&acc, "2018_returns")),
            _2019: MetricPattern1::new(client.clone(), _m(&acc, "2019_returns")),
            _2020: MetricPattern1::new(client.clone(), _m(&acc, "2020_returns")),
            _2021: MetricPattern1::new(client.clone(), _m(&acc, "2021_returns")),
            _2022: MetricPattern1::new(client.clone(), _m(&acc, "2022_returns")),
            _2023: MetricPattern1::new(client.clone(), _m(&acc, "2023_returns")),
            _2024: MetricPattern1::new(client.clone(), _m(&acc, "2024_returns")),
            _2025: MetricPattern1::new(client.clone(), _m(&acc, "2025_returns")),
            _2026: MetricPattern1::new(client.clone(), _m(&acc, "2026_returns")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct AverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90RollingSumPattern {
    pub average: MetricPattern20<StoredU64>,
    pub cumulative: MetricPattern20<StoredU64>,
    pub max: MetricPattern20<StoredU64>,
    pub median: MetricPattern20<StoredU64>,
    pub min: MetricPattern20<StoredU64>,
    pub pct10: MetricPattern20<StoredU64>,
    pub pct25: MetricPattern20<StoredU64>,
    pub pct75: MetricPattern20<StoredU64>,
    pub pct90: MetricPattern20<StoredU64>,
    pub rolling: AverageMaxMedianMinPct10Pct25Pct75Pct90SumPattern,
    pub sum: MetricPattern20<StoredU64>,
}

impl AverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90RollingSumPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            average: MetricPattern20::new(client.clone(), _m(&acc, "average")),
            cumulative: MetricPattern20::new(client.clone(), _m(&acc, "cumulative")),
            max: MetricPattern20::new(client.clone(), _m(&acc, "max")),
            median: MetricPattern20::new(client.clone(), _m(&acc, "median")),
            min: MetricPattern20::new(client.clone(), _m(&acc, "min")),
            pct10: MetricPattern20::new(client.clone(), _m(&acc, "pct10")),
            pct25: MetricPattern20::new(client.clone(), _m(&acc, "pct25")),
            pct75: MetricPattern20::new(client.clone(), _m(&acc, "pct75")),
            pct90: MetricPattern20::new(client.clone(), _m(&acc, "pct90")),
            rolling: AverageMaxMedianMinPct10Pct25Pct75Pct90SumPattern::new(client.clone(), acc.clone()),
            sum: MetricPattern20::new(client.clone(), _m(&acc, "sum")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct AverageCumulativeHeightMaxMedianMinPct10Pct25Pct75Pct90SumPattern {
    pub average: _1y24h30d7dPattern<StoredU64>,
    pub cumulative: MetricPattern1<StoredU64>,
    pub height: MetricPattern20<StoredU64>,
    pub max: _1y24h30d7dPattern<StoredU64>,
    pub median: _1y24h30d7dPattern<StoredU64>,
    pub min: _1y24h30d7dPattern<StoredU64>,
    pub pct10: _1y24h30d7dPattern<StoredU64>,
    pub pct25: _1y24h30d7dPattern<StoredU64>,
    pub pct75: _1y24h30d7dPattern<StoredU64>,
    pub pct90: _1y24h30d7dPattern<StoredU64>,
    pub sum: _1y24h30d7dPattern<StoredU64>,
}

impl AverageCumulativeHeightMaxMedianMinPct10Pct25Pct75Pct90SumPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            average: _1y24h30d7dPattern::new(client.clone(), _m(&acc, "average")),
            cumulative: MetricPattern1::new(client.clone(), _m(&acc, "cumulative")),
            height: MetricPattern20::new(client.clone(), acc.clone()),
            max: _1y24h30d7dPattern::new(client.clone(), _m(&acc, "max")),
            median: _1y24h30d7dPattern::new(client.clone(), _m(&acc, "median")),
            min: _1y24h30d7dPattern::new(client.clone(), _m(&acc, "min")),
            pct10: _1y24h30d7dPattern::new(client.clone(), _m(&acc, "p10")),
            pct25: _1y24h30d7dPattern::new(client.clone(), _m(&acc, "p25")),
            pct75: _1y24h30d7dPattern::new(client.clone(), _m(&acc, "p75")),
            pct90: _1y24h30d7dPattern::new(client.clone(), _m(&acc, "p90")),
            sum: _1y24h30d7dPattern::new(client.clone(), _m(&acc, "sum")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct AverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern {
    pub average: _1y24h30d7dPattern<StoredU64>,
    pub cumulative: MetricPattern1<StoredU64>,
    pub max: _1y24h30d7dPattern<StoredU64>,
    pub median: _1y24h30d7dPattern<StoredU64>,
    pub min: _1y24h30d7dPattern<StoredU64>,
    pub pct10: _1y24h30d7dPattern<StoredU64>,
    pub pct25: _1y24h30d7dPattern<StoredU64>,
    pub pct75: _1y24h30d7dPattern<StoredU64>,
    pub pct90: _1y24h30d7dPattern<StoredU64>,
    pub sum: _1y24h30d7dPattern<StoredU64>,
}

impl AverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            average: _1y24h30d7dPattern::new(client.clone(), _m(&acc, "average")),
            cumulative: MetricPattern1::new(client.clone(), _m(&acc, "cumulative")),
            max: _1y24h30d7dPattern::new(client.clone(), _m(&acc, "max")),
            median: _1y24h30d7dPattern::new(client.clone(), _m(&acc, "median")),
            min: _1y24h30d7dPattern::new(client.clone(), _m(&acc, "min")),
            pct10: _1y24h30d7dPattern::new(client.clone(), _m(&acc, "p10")),
            pct25: _1y24h30d7dPattern::new(client.clone(), _m(&acc, "p25")),
            pct75: _1y24h30d7dPattern::new(client.clone(), _m(&acc, "p75")),
            pct90: _1y24h30d7dPattern::new(client.clone(), _m(&acc, "p90")),
            sum: _1y24h30d7dPattern::new(client.clone(), _m(&acc, "sum")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct AverageGainsLossesRsiStochPattern {
    pub average_gain: MetricPattern1<StoredF32>,
    pub average_loss: MetricPattern1<StoredF32>,
    pub gains: MetricPattern1<StoredF32>,
    pub losses: MetricPattern1<StoredF32>,
    pub rsi: MetricPattern1<StoredF32>,
    pub rsi_max: MetricPattern1<StoredF32>,
    pub rsi_min: MetricPattern1<StoredF32>,
    pub stoch_rsi: MetricPattern1<StoredF32>,
    pub stoch_rsi_d: MetricPattern1<StoredF32>,
    pub stoch_rsi_k: MetricPattern1<StoredF32>,
}

impl AverageGainsLossesRsiStochPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            average_gain: MetricPattern1::new(client.clone(), _m(&acc, "avg_gain_1y")),
            average_loss: MetricPattern1::new(client.clone(), _m(&acc, "avg_loss_1y")),
            gains: MetricPattern1::new(client.clone(), _m(&acc, "gains_1y")),
            losses: MetricPattern1::new(client.clone(), _m(&acc, "losses_1y")),
            rsi: MetricPattern1::new(client.clone(), _m(&acc, "1y")),
            rsi_max: MetricPattern1::new(client.clone(), _m(&acc, "rsi_max_1y")),
            rsi_min: MetricPattern1::new(client.clone(), _m(&acc, "rsi_min_1y")),
            stoch_rsi: MetricPattern1::new(client.clone(), _m(&acc, "stoch_rsi_1y")),
            stoch_rsi_d: MetricPattern1::new(client.clone(), _m(&acc, "stoch_rsi_d_1y")),
            stoch_rsi_k: MetricPattern1::new(client.clone(), _m(&acc, "stoch_rsi_k_1y")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct ActivityAddrCostOutputsRealizedRelativeSupplyUnrealizedPattern {
    pub activity: CoinblocksCoindaysSatblocksSatdaysSentPattern,
    pub addr_count: MetricPattern1<StoredU64>,
    pub addr_count_30d_change: MetricPattern1<StoredF64>,
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
            addr_count_30d_change: MetricPattern1::new(client.clone(), _m(&acc, "addr_count_30d_change")),
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
pub struct AverageMaxMedianMinPct10Pct25Pct75Pct90SumPattern2 {
    pub average: BtcSatsUsdPattern,
    pub max: BtcSatsUsdPattern,
    pub median: BtcSatsUsdPattern,
    pub min: BtcSatsUsdPattern,
    pub pct10: BtcSatsUsdPattern,
    pub pct25: BtcSatsUsdPattern,
    pub pct75: BtcSatsUsdPattern,
    pub pct90: BtcSatsUsdPattern,
    pub sum: BtcSatsUsdPattern,
}

impl AverageMaxMedianMinPct10Pct25Pct75Pct90SumPattern2 {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            average: BtcSatsUsdPattern::new(client.clone(), _m(&acc, "average")),
            max: BtcSatsUsdPattern::new(client.clone(), _m(&acc, "max")),
            median: BtcSatsUsdPattern::new(client.clone(), _m(&acc, "median")),
            min: BtcSatsUsdPattern::new(client.clone(), _m(&acc, "min")),
            pct10: BtcSatsUsdPattern::new(client.clone(), _m(&acc, "p10")),
            pct25: BtcSatsUsdPattern::new(client.clone(), _m(&acc, "p25")),
            pct75: BtcSatsUsdPattern::new(client.clone(), _m(&acc, "p75")),
            pct90: BtcSatsUsdPattern::new(client.clone(), _m(&acc, "p90")),
            sum: BtcSatsUsdPattern::new(client.clone(), _m(&acc, "sum")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct AverageMaxMedianMinPct10Pct25Pct75Pct90SumPattern {
    pub average: _1y24h30d7dPattern<StoredU64>,
    pub max: _1y24h30d7dPattern<StoredU64>,
    pub median: _1y24h30d7dPattern<StoredU64>,
    pub min: _1y24h30d7dPattern<StoredU64>,
    pub pct10: _1y24h30d7dPattern<StoredU64>,
    pub pct25: _1y24h30d7dPattern<StoredU64>,
    pub pct75: _1y24h30d7dPattern<StoredU64>,
    pub pct90: _1y24h30d7dPattern<StoredU64>,
    pub sum: _1y24h30d7dPattern<StoredU64>,
}

impl AverageMaxMedianMinPct10Pct25Pct75Pct90SumPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            average: _1y24h30d7dPattern::new(client.clone(), _m(&acc, "average")),
            max: _1y24h30d7dPattern::new(client.clone(), _m(&acc, "max")),
            median: _1y24h30d7dPattern::new(client.clone(), _m(&acc, "median")),
            min: _1y24h30d7dPattern::new(client.clone(), _m(&acc, "min")),
            pct10: _1y24h30d7dPattern::new(client.clone(), _m(&acc, "p10")),
            pct25: _1y24h30d7dPattern::new(client.clone(), _m(&acc, "p25")),
            pct75: _1y24h30d7dPattern::new(client.clone(), _m(&acc, "p75")),
            pct90: _1y24h30d7dPattern::new(client.clone(), _m(&acc, "p90")),
            sum: _1y24h30d7dPattern::new(client.clone(), _m(&acc, "sum")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct AverageHeightMaxMedianMinPct10Pct25Pct75Pct90Pattern<T> {
    pub average: _1y24h30d7dPattern<T>,
    pub height: MetricPattern20<T>,
    pub max: _1y24h30d7dPattern<T>,
    pub median: _1y24h30d7dPattern<T>,
    pub min: _1y24h30d7dPattern<T>,
    pub pct10: _1y24h30d7dPattern<T>,
    pub pct25: _1y24h30d7dPattern<T>,
    pub pct75: _1y24h30d7dPattern<T>,
    pub pct90: _1y24h30d7dPattern<T>,
}

impl<T: DeserializeOwned> AverageHeightMaxMedianMinPct10Pct25Pct75Pct90Pattern<T> {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            average: _1y24h30d7dPattern::new(client.clone(), _m(&acc, "average")),
            height: MetricPattern20::new(client.clone(), acc.clone()),
            max: _1y24h30d7dPattern::new(client.clone(), _m(&acc, "max")),
            median: _1y24h30d7dPattern::new(client.clone(), _m(&acc, "median")),
            min: _1y24h30d7dPattern::new(client.clone(), _m(&acc, "min")),
            pct10: _1y24h30d7dPattern::new(client.clone(), _m(&acc, "p10")),
            pct25: _1y24h30d7dPattern::new(client.clone(), _m(&acc, "p25")),
            pct75: _1y24h30d7dPattern::new(client.clone(), _m(&acc, "p75")),
            pct90: _1y24h30d7dPattern::new(client.clone(), _m(&acc, "p90")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct AverageMaxMedianMinPct10Pct25Pct75Pct90Pattern<T> {
    pub average: MetricPattern20<T>,
    pub max: MetricPattern20<T>,
    pub median: MetricPattern20<T>,
    pub min: MetricPattern20<T>,
    pub pct10: MetricPattern20<T>,
    pub pct25: MetricPattern20<T>,
    pub pct75: MetricPattern20<T>,
    pub pct90: MetricPattern20<T>,
}

impl<T: DeserializeOwned> AverageMaxMedianMinPct10Pct25Pct75Pct90Pattern<T> {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            average: MetricPattern20::new(client.clone(), _m(&acc, "average")),
            max: MetricPattern20::new(client.clone(), _m(&acc, "max")),
            median: MetricPattern20::new(client.clone(), _m(&acc, "median")),
            min: MetricPattern20::new(client.clone(), _m(&acc, "min")),
            pct10: MetricPattern20::new(client.clone(), _m(&acc, "pct10")),
            pct25: MetricPattern20::new(client.clone(), _m(&acc, "pct25")),
            pct75: MetricPattern20::new(client.clone(), _m(&acc, "pct75")),
            pct90: MetricPattern20::new(client.clone(), _m(&acc, "pct90")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct _10y2y3y4y5y6y8yPattern {
    pub _10y: MetricPattern1<StoredF32>,
    pub _2y: MetricPattern1<StoredF32>,
    pub _3y: MetricPattern1<StoredF32>,
    pub _4y: MetricPattern1<StoredF32>,
    pub _5y: MetricPattern1<StoredF32>,
    pub _6y: MetricPattern1<StoredF32>,
    pub _8y: MetricPattern1<StoredF32>,
}

impl _10y2y3y4y5y6y8yPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            _10y: MetricPattern1::new(client.clone(), _p("10y", &acc)),
            _2y: MetricPattern1::new(client.clone(), _p("2y", &acc)),
            _3y: MetricPattern1::new(client.clone(), _p("3y", &acc)),
            _4y: MetricPattern1::new(client.clone(), _p("4y", &acc)),
            _5y: MetricPattern1::new(client.clone(), _p("5y", &acc)),
            _6y: MetricPattern1::new(client.clone(), _p("6y", &acc)),
            _8y: MetricPattern1::new(client.clone(), _p("8y", &acc)),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct _1y24h30d7dBtcSatsUsdPattern {
    pub _1y: BtcSatsUsdPattern,
    pub _24h: BtcSatsUsdPattern,
    pub _30d: BtcSatsUsdPattern,
    pub _7d: BtcSatsUsdPattern,
    pub btc: MetricPattern20<Bitcoin>,
    pub sats: MetricPattern20<Sats>,
    pub usd: MetricPattern20<Dollars>,
}

impl _1y24h30d7dBtcSatsUsdPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            _1y: BtcSatsUsdPattern::new(client.clone(), _m(&acc, "1y")),
            _24h: BtcSatsUsdPattern::new(client.clone(), _m(&acc, "24h")),
            _30d: BtcSatsUsdPattern::new(client.clone(), _m(&acc, "30d")),
            _7d: BtcSatsUsdPattern::new(client.clone(), _m(&acc, "7d")),
            btc: MetricPattern20::new(client.clone(), _m(&acc, "btc")),
            sats: MetricPattern20::new(client.clone(), acc.clone()),
            usd: MetricPattern20::new(client.clone(), _m(&acc, "usd")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern {
    pub activity: CoinblocksCoindaysSatblocksSatdaysSentPattern,
    pub cost_basis: InvestedMaxMinPercentilesSpotPattern,
    pub outputs: UtxoPattern,
    pub realized: CapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern2,
    pub relative: InvestedNegNetNuplSupplyUnrealizedPattern2,
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
            relative: InvestedNegNetNuplSupplyUnrealizedPattern2::new(client.clone(), acc.clone()),
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
    pub realized: AdjustedCapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern2,
    pub relative: InvestedNegNetNuplSupplyUnrealizedPattern4,
    pub supply: _30dHalvedTotalPattern,
    pub unrealized: GreedInvestedInvestorNegNetPainPeakSupplyTotalUnrealizedPattern,
}

impl ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4 {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            activity: CoinblocksCoindaysSatblocksSatdaysSentPattern::new(client.clone(), acc.clone()),
            cost_basis: MaxMinPattern::new(client.clone(), acc.clone()),
            outputs: UtxoPattern::new(client.clone(), _m(&acc, "utxo_count")),
            realized: AdjustedCapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern2::new(client.clone(), acc.clone()),
            relative: InvestedNegNetNuplSupplyUnrealizedPattern4::new(client.clone(), acc.clone()),
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
    pub relative: InvestedNegNetNuplSupplyUnrealizedPattern,
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
            relative: InvestedNegNetNuplSupplyUnrealizedPattern::new(client.clone(), acc.clone()),
            supply: _30dHalvedTotalPattern::new(client.clone(), acc.clone()),
            unrealized: GreedInvestedInvestorNegNetPainSupplyTotalUnrealizedPattern::new(client.clone(), acc.clone()),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5 {
    pub activity: CoinblocksCoindaysSatblocksSatdaysSentPattern,
    pub cost_basis: MaxMinPattern,
    pub outputs: UtxoPattern,
    pub realized: CapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern,
    pub relative: InvestedNegNetNuplSupplyUnrealizedPattern4,
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
            realized: CapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern::new(client.clone(), acc.clone()),
            relative: InvestedNegNetNuplSupplyUnrealizedPattern4::new(client.clone(), acc.clone()),
            supply: _30dHalvedTotalPattern::new(client.clone(), acc.clone()),
            unrealized: GreedInvestedInvestorNegNetPainPeakSupplyTotalUnrealizedPattern::new(client.clone(), acc.clone()),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct _1y24h30d7dBaseCumulativePattern {
    pub _1y: AverageMaxMedianMinPct10Pct25Pct75Pct90SumPattern2,
    pub _24h: AverageMaxMedianMinPct10Pct25Pct75Pct90SumPattern2,
    pub _30d: AverageMaxMedianMinPct10Pct25Pct75Pct90SumPattern2,
    pub _7d: AverageMaxMedianMinPct10Pct25Pct75Pct90SumPattern2,
    pub base: BtcSatsUsdPattern,
    pub cumulative: BtcSatsUsdPattern,
}

impl _1y24h30d7dBaseCumulativePattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            _1y: AverageMaxMedianMinPct10Pct25Pct75Pct90SumPattern2::new(client.clone(), _m(&acc, "1y")),
            _24h: AverageMaxMedianMinPct10Pct25Pct75Pct90SumPattern2::new(client.clone(), _m(&acc, "24h")),
            _30d: AverageMaxMedianMinPct10Pct25Pct75Pct90SumPattern2::new(client.clone(), _m(&acc, "30d")),
            _7d: AverageMaxMedianMinPct10Pct25Pct75Pct90SumPattern2::new(client.clone(), _m(&acc, "7d")),
            base: BtcSatsUsdPattern::new(client.clone(), acc.clone()),
            cumulative: BtcSatsUsdPattern::new(client.clone(), _m(&acc, "cumulative")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct BalanceBothReactivatedReceivingSendingPattern {
    pub balance_decreased: AverageHeightMaxMedianMinPct10Pct25Pct75Pct90Pattern<StoredU32>,
    pub balance_increased: AverageHeightMaxMedianMinPct10Pct25Pct75Pct90Pattern<StoredU32>,
    pub both: AverageHeightMaxMedianMinPct10Pct25Pct75Pct90Pattern<StoredU32>,
    pub reactivated: AverageHeightMaxMedianMinPct10Pct25Pct75Pct90Pattern<StoredU32>,
    pub receiving: AverageHeightMaxMedianMinPct10Pct25Pct75Pct90Pattern<StoredU32>,
    pub sending: AverageHeightMaxMedianMinPct10Pct25Pct75Pct90Pattern<StoredU32>,
}

impl BalanceBothReactivatedReceivingSendingPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            balance_decreased: AverageHeightMaxMedianMinPct10Pct25Pct75Pct90Pattern::new(client.clone(), _m(&acc, "balance_decreased")),
            balance_increased: AverageHeightMaxMedianMinPct10Pct25Pct75Pct90Pattern::new(client.clone(), _m(&acc, "balance_increased")),
            both: AverageHeightMaxMedianMinPct10Pct25Pct75Pct90Pattern::new(client.clone(), _m(&acc, "both")),
            reactivated: AverageHeightMaxMedianMinPct10Pct25Pct75Pct90Pattern::new(client.clone(), _m(&acc, "reactivated")),
            receiving: AverageHeightMaxMedianMinPct10Pct25Pct75Pct90Pattern::new(client.clone(), _m(&acc, "receiving")),
            sending: AverageHeightMaxMedianMinPct10Pct25Pct75Pct90Pattern::new(client.clone(), _m(&acc, "sending")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct CoinblocksCoindaysSatblocksSatdaysSentPattern {
    pub coinblocks_destroyed: CumulativeHeightSumPattern<StoredF64>,
    pub coindays_destroyed: CumulativeHeightSumPattern<StoredF64>,
    pub satblocks_destroyed: MetricPattern20<Sats>,
    pub satdays_destroyed: MetricPattern20<Sats>,
    pub sent: BaseCumulativePattern,
    pub sent_14d_ema: BtcSatsUsdPattern,
}

impl CoinblocksCoindaysSatblocksSatdaysSentPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            coinblocks_destroyed: CumulativeHeightSumPattern::new(client.clone(), _m(&acc, "coinblocks_destroyed")),
            coindays_destroyed: CumulativeHeightSumPattern::new(client.clone(), _m(&acc, "coindays_destroyed")),
            satblocks_destroyed: MetricPattern20::new(client.clone(), _m(&acc, "satblocks_destroyed")),
            satdays_destroyed: MetricPattern20::new(client.clone(), _m(&acc, "satdays_destroyed")),
            sent: BaseCumulativePattern::new(client.clone(), _m(&acc, "sent")),
            sent_14d_ema: BtcSatsUsdPattern::new(client.clone(), _m(&acc, "sent_14d_ema")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct InvestedMaxMinPercentilesSpotPattern {
    pub invested_capital: Pct05Pct10Pct15Pct20Pct25Pct30Pct35Pct40Pct45Pct50Pct55Pct60Pct65Pct70Pct75Pct80Pct85Pct90Pct95Pattern,
    pub max: SatsUsdPattern,
    pub min: SatsUsdPattern,
    pub percentiles: Pct05Pct10Pct15Pct20Pct25Pct30Pct35Pct40Pct45Pct50Pct55Pct60Pct65Pct70Pct75Pct80Pct85Pct90Pct95Pattern,
    pub spot_cost_basis_percentile: MetricPattern1<StoredF32>,
    pub spot_invested_capital_percentile: MetricPattern1<StoredF32>,
}

impl InvestedMaxMinPercentilesSpotPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            invested_capital: Pct05Pct10Pct15Pct20Pct25Pct30Pct35Pct40Pct45Pct50Pct55Pct60Pct65Pct70Pct75Pct80Pct85Pct90Pct95Pattern::new(client.clone(), _m(&acc, "invested_capital")),
            max: SatsUsdPattern::new(client.clone(), _m(&acc, "max_cost_basis")),
            min: SatsUsdPattern::new(client.clone(), _m(&acc, "min_cost_basis")),
            percentiles: Pct05Pct10Pct15Pct20Pct25Pct30Pct35Pct40Pct45Pct50Pct55Pct60Pct65Pct70Pct75Pct80Pct85Pct90Pct95Pattern::new(client.clone(), _m(&acc, "cost_basis")),
            spot_cost_basis_percentile: MetricPattern1::new(client.clone(), _m(&acc, "spot_cost_basis_percentile")),
            spot_invested_capital_percentile: MetricPattern1::new(client.clone(), _m(&acc, "spot_invested_capital_percentile")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct _1y24h30d7dPattern2 {
    pub _1y: BtcSatsUsdPattern,
    pub _24h: BtcSatsUsdPattern,
    pub _30d: BtcSatsUsdPattern,
    pub _7d: BtcSatsUsdPattern,
}

impl _1y24h30d7dPattern2 {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            _1y: BtcSatsUsdPattern::new(client.clone(), _m(&acc, "1y")),
            _24h: BtcSatsUsdPattern::new(client.clone(), _m(&acc, "24h")),
            _30d: BtcSatsUsdPattern::new(client.clone(), _m(&acc, "30d")),
            _7d: BtcSatsUsdPattern::new(client.clone(), _m(&acc, "7d")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct _1h24hBlockTxindexPattern<T> {
    pub _1h: AverageMaxMedianMinPct10Pct25Pct75Pct90Pattern<T>,
    pub _24h: AverageMaxMedianMinPct10Pct25Pct75Pct90Pattern<T>,
    pub block: AverageMaxMedianMinPct10Pct25Pct75Pct90Pattern<T>,
    pub txindex: MetricPattern21<T>,
}

impl<T: DeserializeOwned> _1h24hBlockTxindexPattern<T> {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            _1h: AverageMaxMedianMinPct10Pct25Pct75Pct90Pattern::new(client.clone(), _m(&acc, "1h")),
            _24h: AverageMaxMedianMinPct10Pct25Pct75Pct90Pattern::new(client.clone(), _m(&acc, "24h")),
            block: AverageMaxMedianMinPct10Pct25Pct75Pct90Pattern::new(client.clone(), acc.clone()),
            txindex: MetricPattern21::new(client.clone(), acc.clone()),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct _1y24h30d7dPattern<T> {
    pub _1y: MetricPattern1<T>,
    pub _24h: MetricPattern1<T>,
    pub _30d: MetricPattern1<T>,
    pub _7d: MetricPattern1<T>,
}

impl<T: DeserializeOwned> _1y24h30d7dPattern<T> {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            _1y: MetricPattern1::new(client.clone(), _m(&acc, "1y")),
            _24h: MetricPattern1::new(client.clone(), _m(&acc, "24h")),
            _30d: MetricPattern1::new(client.clone(), _m(&acc, "30d")),
            _7d: MetricPattern1::new(client.clone(), _m(&acc, "7d")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct _30dHalvedTotalPattern {
    pub _30d_change: BtcSatsUsdPattern,
    pub halved: BtcSatsUsdPattern,
    pub total: BtcSatsUsdPattern,
}

impl _30dHalvedTotalPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            _30d_change: BtcSatsUsdPattern::new(client.clone(), _m(&acc, "_30d_change")),
            halved: BtcSatsUsdPattern::new(client.clone(), _m(&acc, "supply_halved")),
            total: BtcSatsUsdPattern::new(client.clone(), _m(&acc, "supply")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct BaseCumulativeSumPattern {
    pub base: BtcSatsUsdPattern,
    pub cumulative: BtcSatsUsdPattern,
    pub sum: _1y24h30d7dPattern2,
}

impl BaseCumulativeSumPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            base: BtcSatsUsdPattern::new(client.clone(), acc.clone()),
            cumulative: BtcSatsUsdPattern::new(client.clone(), _m(&acc, "cumulative")),
            sum: _1y24h30d7dPattern2::new(client.clone(), _m(&acc, "sum")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct BtcSatsUsdPattern {
    pub btc: MetricPattern1<Bitcoin>,
    pub sats: MetricPattern1<Sats>,
    pub usd: MetricPattern1<Dollars>,
}

impl BtcSatsUsdPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            btc: MetricPattern1::new(client.clone(), _m(&acc, "btc")),
            sats: MetricPattern1::new(client.clone(), acc.clone()),
            usd: MetricPattern1::new(client.clone(), _m(&acc, "usd")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct CentsSatsUsdPattern {
    pub cents: MetricPattern2<Cents>,
    pub sats: MetricPattern2<Sats>,
    pub usd: MetricPattern2<Dollars>,
}

impl CentsSatsUsdPattern {
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
pub struct HistogramLineSignalPattern {
    pub histogram: MetricPattern1<StoredF32>,
    pub line: MetricPattern1<StoredF32>,
    pub signal: MetricPattern1<StoredF32>,
}

impl HistogramLineSignalPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            histogram: MetricPattern1::new(client.clone(), _m(&acc, "histogram_1y")),
            line: MetricPattern1::new(client.clone(), _m(&acc, "line_1y")),
            signal: MetricPattern1::new(client.clone(), _m(&acc, "signal_1y")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct CumulativeHeightSumPattern<T> {
    pub cumulative: MetricPattern1<T>,
    pub height: MetricPattern20<T>,
    pub sum: _1y24h30d7dPattern<T>,
}

impl<T: DeserializeOwned> CumulativeHeightSumPattern<T> {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            cumulative: MetricPattern1::new(client.clone(), _m(&acc, "cumulative")),
            height: MetricPattern20::new(client.clone(), acc.clone()),
            sum: _1y24h30d7dPattern::new(client.clone(), acc.clone()),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct _30dCountPattern {
    pub _30d_change: MetricPattern1<StoredF64>,
    pub count: MetricPattern1<StoredU64>,
}

impl _30dCountPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            _30d_change: MetricPattern1::new(client.clone(), _m(&acc, "30d_change")),
            count: MetricPattern1::new(client.clone(), acc.clone()),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct BaseCumulativePattern {
    pub base: BtcSatsUsdPattern,
    pub cumulative: BtcSatsUsdPattern,
}

impl BaseCumulativePattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            base: BtcSatsUsdPattern::new(client.clone(), acc.clone()),
            cumulative: BtcSatsUsdPattern::new(client.clone(), _m(&acc, "cumulative")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct BaseRestPattern {
    pub base: MetricPattern20<StoredU64>,
    pub rest: AverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern,
}

impl BaseRestPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            base: MetricPattern20::new(client.clone(), acc.clone()),
            rest: AverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern::new(client.clone(), acc.clone()),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct CumulativeHeightPattern {
    pub cumulative: MetricPattern1<Dollars>,
    pub height: MetricPattern20<Dollars>,
}

impl CumulativeHeightPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            cumulative: MetricPattern1::new(client.clone(), _m(&acc, "cumulative")),
            height: MetricPattern20::new(client.clone(), acc.clone()),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct MaxMinPattern {
    pub max: SatsUsdPattern,
    pub min: SatsUsdPattern,
}

impl MaxMinPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            max: SatsUsdPattern::new(client.clone(), _m(&acc, "max_cost_basis")),
            min: SatsUsdPattern::new(client.clone(), _m(&acc, "min_cost_basis")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct SatsUsdPattern {
    pub sats: MetricPattern1<SatsFract>,
    pub usd: MetricPattern1<Dollars>,
}

impl SatsUsdPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            sats: MetricPattern1::new(client.clone(), _m(&acc, "sats")),
            usd: MetricPattern1::new(client.clone(), acc.clone()),
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
            sd: MetricPattern1::new(client.clone(), _m(&acc, "sd")),
            sma: MetricPattern1::new(client.clone(), _m(&acc, "sma")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct UtxoPattern {
    pub utxo_count: MetricPattern1<StoredU64>,
    pub utxo_count_30d_change: MetricPattern1<StoredF64>,
}

impl UtxoPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            utxo_count: MetricPattern1::new(client.clone(), acc.clone()),
            utxo_count_30d_change: MetricPattern1::new(client.clone(), _m(&acc, "30d_change")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct RatioPattern2 {
    pub ratio: MetricPattern1<StoredF32>,
}

impl RatioPattern2 {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: String) -> Self {
        Self {
            ratio: MetricPattern1::new(client.clone(), acc.clone()),
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
    pub blockhash: MetricPattern20<BlockHash>,
    pub difficulty: MetricsTree_Blocks_Difficulty,
    pub time: MetricsTree_Blocks_Time,
    pub total_size: MetricPattern20<StoredU64>,
    pub weight: MetricsTree_Blocks_Weight,
    pub count: MetricsTree_Blocks_Count,
    pub interval: AverageHeightMaxMedianMinPct10Pct25Pct75Pct90Pattern<Timestamp>,
    pub halving: MetricsTree_Blocks_Halving,
    pub vbytes: AverageCumulativeHeightMaxMedianMinPct10Pct25Pct75Pct90SumPattern,
    pub size: AverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern,
    pub fullness: AverageHeightMaxMedianMinPct10Pct25Pct75Pct90Pattern<StoredF32>,
}

impl MetricsTree_Blocks {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            blockhash: MetricPattern20::new(client.clone(), "blockhash".to_string()),
            difficulty: MetricsTree_Blocks_Difficulty::new(client.clone(), format!("{base_path}_difficulty")),
            time: MetricsTree_Blocks_Time::new(client.clone(), format!("{base_path}_time")),
            total_size: MetricPattern20::new(client.clone(), "total_size".to_string()),
            weight: MetricsTree_Blocks_Weight::new(client.clone(), format!("{base_path}_weight")),
            count: MetricsTree_Blocks_Count::new(client.clone(), format!("{base_path}_count")),
            interval: AverageHeightMaxMedianMinPct10Pct25Pct75Pct90Pattern::new(client.clone(), "block_interval".to_string()),
            halving: MetricsTree_Blocks_Halving::new(client.clone(), format!("{base_path}_halving")),
            vbytes: AverageCumulativeHeightMaxMedianMinPct10Pct25Pct75Pct90SumPattern::new(client.clone(), "block_vbytes".to_string()),
            size: AverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90SumPattern::new(client.clone(), "block_size".to_string()),
            fullness: AverageHeightMaxMedianMinPct10Pct25Pct75Pct90Pattern::new(client.clone(), "block_fullness".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Blocks_Difficulty {
    pub raw: MetricPattern1<StoredF64>,
    pub as_hash: MetricPattern1<StoredF32>,
    pub adjustment: MetricPattern1<StoredF32>,
    pub epoch: MetricPattern1<DifficultyEpoch>,
    pub blocks_before_next_adjustment: MetricPattern1<StoredU32>,
    pub days_before_next_adjustment: MetricPattern1<StoredF32>,
}

impl MetricsTree_Blocks_Difficulty {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            raw: MetricPattern1::new(client.clone(), "difficulty".to_string()),
            as_hash: MetricPattern1::new(client.clone(), "difficulty_as_hash".to_string()),
            adjustment: MetricPattern1::new(client.clone(), "difficulty_adjustment".to_string()),
            epoch: MetricPattern1::new(client.clone(), "difficulty_epoch".to_string()),
            blocks_before_next_adjustment: MetricPattern1::new(client.clone(), "blocks_before_next_difficulty_adjustment".to_string()),
            days_before_next_adjustment: MetricPattern1::new(client.clone(), "days_before_next_difficulty_adjustment".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Blocks_Time {
    pub timestamp: MetricPattern1<Timestamp>,
    pub date: MetricPattern20<Date>,
    pub timestamp_monotonic: MetricPattern20<Timestamp>,
}

impl MetricsTree_Blocks_Time {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            timestamp: MetricPattern1::new(client.clone(), "timestamp".to_string()),
            date: MetricPattern20::new(client.clone(), "date".to_string()),
            timestamp_monotonic: MetricPattern20::new(client.clone(), "timestamp_monotonic".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Blocks_Weight {
    pub base: MetricPattern20<Weight>,
    pub cumulative: MetricPattern1<Weight>,
    pub sum: _1y24h30d7dPattern<Weight>,
    pub average: _1y24h30d7dPattern<Weight>,
    pub min: _1y24h30d7dPattern<Weight>,
    pub max: _1y24h30d7dPattern<Weight>,
    pub pct10: _1y24h30d7dPattern<Weight>,
    pub pct25: _1y24h30d7dPattern<Weight>,
    pub median: _1y24h30d7dPattern<Weight>,
    pub pct75: _1y24h30d7dPattern<Weight>,
    pub pct90: _1y24h30d7dPattern<Weight>,
}

impl MetricsTree_Blocks_Weight {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            base: MetricPattern20::new(client.clone(), "block_weight".to_string()),
            cumulative: MetricPattern1::new(client.clone(), "block_weight_cumulative".to_string()),
            sum: _1y24h30d7dPattern::new(client.clone(), "block_weight_sum".to_string()),
            average: _1y24h30d7dPattern::new(client.clone(), "block_weight_average".to_string()),
            min: _1y24h30d7dPattern::new(client.clone(), "block_weight_min".to_string()),
            max: _1y24h30d7dPattern::new(client.clone(), "block_weight_max".to_string()),
            pct10: _1y24h30d7dPattern::new(client.clone(), "block_weight_p10".to_string()),
            pct25: _1y24h30d7dPattern::new(client.clone(), "block_weight_p25".to_string()),
            median: _1y24h30d7dPattern::new(client.clone(), "block_weight_median".to_string()),
            pct75: _1y24h30d7dPattern::new(client.clone(), "block_weight_p75".to_string()),
            pct90: _1y24h30d7dPattern::new(client.clone(), "block_weight_p90".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Blocks_Count {
    pub block_count_target: MetricPattern1<StoredU64>,
    pub block_count: CumulativeHeightSumPattern<StoredU32>,
    pub block_count_sum: _1y24h30d7dPattern<StoredU32>,
    pub height_1h_ago: MetricPattern20<Height>,
    pub height_24h_ago: MetricPattern20<Height>,
    pub height_3d_ago: MetricPattern20<Height>,
    pub height_1w_ago: MetricPattern20<Height>,
    pub height_8d_ago: MetricPattern20<Height>,
    pub height_9d_ago: MetricPattern20<Height>,
    pub height_12d_ago: MetricPattern20<Height>,
    pub height_13d_ago: MetricPattern20<Height>,
    pub height_2w_ago: MetricPattern20<Height>,
    pub height_21d_ago: MetricPattern20<Height>,
    pub height_26d_ago: MetricPattern20<Height>,
    pub height_1m_ago: MetricPattern20<Height>,
    pub height_34d_ago: MetricPattern20<Height>,
    pub height_55d_ago: MetricPattern20<Height>,
    pub height_2m_ago: MetricPattern20<Height>,
    pub height_89d_ago: MetricPattern20<Height>,
    pub height_111d_ago: MetricPattern20<Height>,
    pub height_144d_ago: MetricPattern20<Height>,
    pub height_3m_ago: MetricPattern20<Height>,
    pub height_6m_ago: MetricPattern20<Height>,
    pub height_200d_ago: MetricPattern20<Height>,
    pub height_350d_ago: MetricPattern20<Height>,
    pub height_1y_ago: MetricPattern20<Height>,
    pub height_2y_ago: MetricPattern20<Height>,
    pub height_200w_ago: MetricPattern20<Height>,
    pub height_3y_ago: MetricPattern20<Height>,
    pub height_4y_ago: MetricPattern20<Height>,
    pub height_5y_ago: MetricPattern20<Height>,
    pub height_6y_ago: MetricPattern20<Height>,
    pub height_8y_ago: MetricPattern20<Height>,
    pub height_10y_ago: MetricPattern20<Height>,
}

impl MetricsTree_Blocks_Count {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            block_count_target: MetricPattern1::new(client.clone(), "block_count_target".to_string()),
            block_count: CumulativeHeightSumPattern::new(client.clone(), "block_count".to_string()),
            block_count_sum: _1y24h30d7dPattern::new(client.clone(), "block_count_sum".to_string()),
            height_1h_ago: MetricPattern20::new(client.clone(), "height_1h_ago".to_string()),
            height_24h_ago: MetricPattern20::new(client.clone(), "height_24h_ago".to_string()),
            height_3d_ago: MetricPattern20::new(client.clone(), "height_3d_ago".to_string()),
            height_1w_ago: MetricPattern20::new(client.clone(), "height_1w_ago".to_string()),
            height_8d_ago: MetricPattern20::new(client.clone(), "height_8d_ago".to_string()),
            height_9d_ago: MetricPattern20::new(client.clone(), "height_9d_ago".to_string()),
            height_12d_ago: MetricPattern20::new(client.clone(), "height_12d_ago".to_string()),
            height_13d_ago: MetricPattern20::new(client.clone(), "height_13d_ago".to_string()),
            height_2w_ago: MetricPattern20::new(client.clone(), "height_2w_ago".to_string()),
            height_21d_ago: MetricPattern20::new(client.clone(), "height_21d_ago".to_string()),
            height_26d_ago: MetricPattern20::new(client.clone(), "height_26d_ago".to_string()),
            height_1m_ago: MetricPattern20::new(client.clone(), "height_1m_ago".to_string()),
            height_34d_ago: MetricPattern20::new(client.clone(), "height_34d_ago".to_string()),
            height_55d_ago: MetricPattern20::new(client.clone(), "height_55d_ago".to_string()),
            height_2m_ago: MetricPattern20::new(client.clone(), "height_2m_ago".to_string()),
            height_89d_ago: MetricPattern20::new(client.clone(), "height_89d_ago".to_string()),
            height_111d_ago: MetricPattern20::new(client.clone(), "height_111d_ago".to_string()),
            height_144d_ago: MetricPattern20::new(client.clone(), "height_144d_ago".to_string()),
            height_3m_ago: MetricPattern20::new(client.clone(), "height_3m_ago".to_string()),
            height_6m_ago: MetricPattern20::new(client.clone(), "height_6m_ago".to_string()),
            height_200d_ago: MetricPattern20::new(client.clone(), "height_200d_ago".to_string()),
            height_350d_ago: MetricPattern20::new(client.clone(), "height_350d_ago".to_string()),
            height_1y_ago: MetricPattern20::new(client.clone(), "height_1y_ago".to_string()),
            height_2y_ago: MetricPattern20::new(client.clone(), "height_2y_ago".to_string()),
            height_200w_ago: MetricPattern20::new(client.clone(), "height_200w_ago".to_string()),
            height_3y_ago: MetricPattern20::new(client.clone(), "height_3y_ago".to_string()),
            height_4y_ago: MetricPattern20::new(client.clone(), "height_4y_ago".to_string()),
            height_5y_ago: MetricPattern20::new(client.clone(), "height_5y_ago".to_string()),
            height_6y_ago: MetricPattern20::new(client.clone(), "height_6y_ago".to_string()),
            height_8y_ago: MetricPattern20::new(client.clone(), "height_8y_ago".to_string()),
            height_10y_ago: MetricPattern20::new(client.clone(), "height_10y_ago".to_string()),
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
pub struct MetricsTree_Transactions {
    pub first_txindex: MetricPattern20<TxIndex>,
    pub height: MetricPattern21<Height>,
    pub txid: MetricPattern21<Txid>,
    pub txversion: MetricPattern21<TxVersion>,
    pub rawlocktime: MetricPattern21<RawLockTime>,
    pub base_size: MetricPattern21<StoredU32>,
    pub total_size: MetricPattern21<StoredU32>,
    pub is_explicitly_rbf: MetricPattern21<StoredBool>,
    pub first_txinindex: MetricPattern21<TxInIndex>,
    pub first_txoutindex: MetricPattern21<TxOutIndex>,
    pub count: MetricsTree_Transactions_Count,
    pub size: MetricsTree_Transactions_Size,
    pub fees: MetricsTree_Transactions_Fees,
    pub versions: MetricsTree_Transactions_Versions,
    pub volume: MetricsTree_Transactions_Volume,
}

impl MetricsTree_Transactions {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            first_txindex: MetricPattern20::new(client.clone(), "first_txindex".to_string()),
            height: MetricPattern21::new(client.clone(), "height".to_string()),
            txid: MetricPattern21::new(client.clone(), "txid".to_string()),
            txversion: MetricPattern21::new(client.clone(), "txversion".to_string()),
            rawlocktime: MetricPattern21::new(client.clone(), "rawlocktime".to_string()),
            base_size: MetricPattern21::new(client.clone(), "base_size".to_string()),
            total_size: MetricPattern21::new(client.clone(), "total_size".to_string()),
            is_explicitly_rbf: MetricPattern21::new(client.clone(), "is_explicitly_rbf".to_string()),
            first_txinindex: MetricPattern21::new(client.clone(), "first_txinindex".to_string()),
            first_txoutindex: MetricPattern21::new(client.clone(), "first_txoutindex".to_string()),
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
    pub is_coinbase: MetricPattern21<StoredBool>,
}

impl MetricsTree_Transactions_Count {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            tx_count: AverageCumulativeHeightMaxMedianMinPct10Pct25Pct75Pct90SumPattern::new(client.clone(), "tx_count".to_string()),
            is_coinbase: MetricPattern21::new(client.clone(), "is_coinbase".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Transactions_Size {
    pub vsize: _1h24hBlockTxindexPattern<VSize>,
    pub weight: _1h24hBlockTxindexPattern<Weight>,
}

impl MetricsTree_Transactions_Size {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            vsize: _1h24hBlockTxindexPattern::new(client.clone(), "tx_vsize".to_string()),
            weight: _1h24hBlockTxindexPattern::new(client.clone(), "tx_weight".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Transactions_Fees {
    pub input_value: MetricPattern21<Sats>,
    pub output_value: MetricPattern21<Sats>,
    pub fee: _1h24hBlockTxindexPattern<Sats>,
    pub fee_rate: _1h24hBlockTxindexPattern<FeeRate>,
}

impl MetricsTree_Transactions_Fees {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            input_value: MetricPattern21::new(client.clone(), "input_value".to_string()),
            output_value: MetricPattern21::new(client.clone(), "output_value".to_string()),
            fee: _1h24hBlockTxindexPattern::new(client.clone(), "fee".to_string()),
            fee_rate: _1h24hBlockTxindexPattern::new(client.clone(), "fee_rate".to_string()),
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
    pub sent_sum: _1y24h30d7dBtcSatsUsdPattern,
    pub received_sum: _1y24h30d7dBtcSatsUsdPattern,
    pub annualized_volume: BtcSatsUsdPattern,
    pub tx_per_sec: MetricPattern1<StoredF32>,
    pub outputs_per_sec: MetricPattern1<StoredF32>,
    pub inputs_per_sec: MetricPattern1<StoredF32>,
}

impl MetricsTree_Transactions_Volume {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            sent_sum: _1y24h30d7dBtcSatsUsdPattern::new(client.clone(), "sent_sum".to_string()),
            received_sum: _1y24h30d7dBtcSatsUsdPattern::new(client.clone(), "received_sum".to_string()),
            annualized_volume: BtcSatsUsdPattern::new(client.clone(), "annualized_volume".to_string()),
            tx_per_sec: MetricPattern1::new(client.clone(), "tx_per_sec".to_string()),
            outputs_per_sec: MetricPattern1::new(client.clone(), "outputs_per_sec".to_string()),
            inputs_per_sec: MetricPattern1::new(client.clone(), "inputs_per_sec".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Inputs {
    pub first_txinindex: MetricPattern20<TxInIndex>,
    pub outpoint: MetricPattern22<OutPoint>,
    pub txindex: MetricPattern22<TxIndex>,
    pub outputtype: MetricPattern22<OutputType>,
    pub typeindex: MetricPattern22<TypeIndex>,
    pub spent: MetricsTree_Inputs_Spent,
    pub count: AverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90RollingSumPattern,
}

impl MetricsTree_Inputs {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            first_txinindex: MetricPattern20::new(client.clone(), "first_txinindex".to_string()),
            outpoint: MetricPattern22::new(client.clone(), "outpoint".to_string()),
            txindex: MetricPattern22::new(client.clone(), "txindex".to_string()),
            outputtype: MetricPattern22::new(client.clone(), "outputtype".to_string()),
            typeindex: MetricPattern22::new(client.clone(), "typeindex".to_string()),
            spent: MetricsTree_Inputs_Spent::new(client.clone(), format!("{base_path}_spent")),
            count: AverageCumulativeMaxMedianMinPct10Pct25Pct75Pct90RollingSumPattern::new(client.clone(), "input_count".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Inputs_Spent {
    pub txoutindex: MetricPattern22<TxOutIndex>,
    pub value: MetricPattern22<Sats>,
}

impl MetricsTree_Inputs_Spent {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            txoutindex: MetricPattern22::new(client.clone(), "txoutindex".to_string()),
            value: MetricPattern22::new(client.clone(), "value".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Outputs {
    pub first_txoutindex: MetricPattern20<TxOutIndex>,
    pub value: MetricPattern23<Sats>,
    pub outputtype: MetricPattern23<OutputType>,
    pub typeindex: MetricPattern23<TypeIndex>,
    pub txindex: MetricPattern23<TxIndex>,
    pub spent: MetricsTree_Outputs_Spent,
    pub count: MetricsTree_Outputs_Count,
}

impl MetricsTree_Outputs {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            first_txoutindex: MetricPattern20::new(client.clone(), "first_txoutindex".to_string()),
            value: MetricPattern23::new(client.clone(), "value".to_string()),
            outputtype: MetricPattern23::new(client.clone(), "outputtype".to_string()),
            typeindex: MetricPattern23::new(client.clone(), "typeindex".to_string()),
            txindex: MetricPattern23::new(client.clone(), "txindex".to_string()),
            spent: MetricsTree_Outputs_Spent::new(client.clone(), format!("{base_path}_spent")),
            count: MetricsTree_Outputs_Count::new(client.clone(), format!("{base_path}_count")),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Outputs_Spent {
    pub txinindex: MetricPattern23<TxInIndex>,
}

impl MetricsTree_Outputs_Spent {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            txinindex: MetricPattern23::new(client.clone(), "txinindex".to_string()),
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
    pub first_p2pk65addressindex: MetricPattern20<P2PK65AddressIndex>,
    pub first_p2pk33addressindex: MetricPattern20<P2PK33AddressIndex>,
    pub first_p2pkhaddressindex: MetricPattern20<P2PKHAddressIndex>,
    pub first_p2shaddressindex: MetricPattern20<P2SHAddressIndex>,
    pub first_p2wpkhaddressindex: MetricPattern20<P2WPKHAddressIndex>,
    pub first_p2wshaddressindex: MetricPattern20<P2WSHAddressIndex>,
    pub first_p2traddressindex: MetricPattern20<P2TRAddressIndex>,
    pub first_p2aaddressindex: MetricPattern20<P2AAddressIndex>,
    pub p2pk65bytes: MetricPattern29<P2PK65Bytes>,
    pub p2pk33bytes: MetricPattern28<P2PK33Bytes>,
    pub p2pkhbytes: MetricPattern30<P2PKHBytes>,
    pub p2shbytes: MetricPattern31<P2SHBytes>,
    pub p2wpkhbytes: MetricPattern33<P2WPKHBytes>,
    pub p2wshbytes: MetricPattern34<P2WSHBytes>,
    pub p2trbytes: MetricPattern32<P2TRBytes>,
    pub p2abytes: MetricPattern26<P2ABytes>,
}

impl MetricsTree_Addresses {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            first_p2pk65addressindex: MetricPattern20::new(client.clone(), "first_p2pk65addressindex".to_string()),
            first_p2pk33addressindex: MetricPattern20::new(client.clone(), "first_p2pk33addressindex".to_string()),
            first_p2pkhaddressindex: MetricPattern20::new(client.clone(), "first_p2pkhaddressindex".to_string()),
            first_p2shaddressindex: MetricPattern20::new(client.clone(), "first_p2shaddressindex".to_string()),
            first_p2wpkhaddressindex: MetricPattern20::new(client.clone(), "first_p2wpkhaddressindex".to_string()),
            first_p2wshaddressindex: MetricPattern20::new(client.clone(), "first_p2wshaddressindex".to_string()),
            first_p2traddressindex: MetricPattern20::new(client.clone(), "first_p2traddressindex".to_string()),
            first_p2aaddressindex: MetricPattern20::new(client.clone(), "first_p2aaddressindex".to_string()),
            p2pk65bytes: MetricPattern29::new(client.clone(), "p2pk65bytes".to_string()),
            p2pk33bytes: MetricPattern28::new(client.clone(), "p2pk33bytes".to_string()),
            p2pkhbytes: MetricPattern30::new(client.clone(), "p2pkhbytes".to_string()),
            p2shbytes: MetricPattern31::new(client.clone(), "p2shbytes".to_string()),
            p2wpkhbytes: MetricPattern33::new(client.clone(), "p2wpkhbytes".to_string()),
            p2wshbytes: MetricPattern34::new(client.clone(), "p2wshbytes".to_string()),
            p2trbytes: MetricPattern32::new(client.clone(), "p2trbytes".to_string()),
            p2abytes: MetricPattern26::new(client.clone(), "p2abytes".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Scripts {
    pub first_emptyoutputindex: MetricPattern20<EmptyOutputIndex>,
    pub first_opreturnindex: MetricPattern20<OpReturnIndex>,
    pub first_p2msoutputindex: MetricPattern20<P2MSOutputIndex>,
    pub first_unknownoutputindex: MetricPattern20<UnknownOutputIndex>,
    pub empty_to_txindex: MetricPattern24<TxIndex>,
    pub opreturn_to_txindex: MetricPattern25<TxIndex>,
    pub p2ms_to_txindex: MetricPattern27<TxIndex>,
    pub unknown_to_txindex: MetricPattern35<TxIndex>,
    pub count: MetricsTree_Scripts_Count,
    pub value: MetricsTree_Scripts_Value,
    pub adoption: MetricsTree_Scripts_Adoption,
}

impl MetricsTree_Scripts {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            first_emptyoutputindex: MetricPattern20::new(client.clone(), "first_emptyoutputindex".to_string()),
            first_opreturnindex: MetricPattern20::new(client.clone(), "first_opreturnindex".to_string()),
            first_p2msoutputindex: MetricPattern20::new(client.clone(), "first_p2msoutputindex".to_string()),
            first_unknownoutputindex: MetricPattern20::new(client.clone(), "first_unknownoutputindex".to_string()),
            empty_to_txindex: MetricPattern24::new(client.clone(), "txindex".to_string()),
            opreturn_to_txindex: MetricPattern25::new(client.clone(), "txindex".to_string()),
            p2ms_to_txindex: MetricPattern27::new(client.clone(), "txindex".to_string()),
            unknown_to_txindex: MetricPattern35::new(client.clone(), "txindex".to_string()),
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
    pub opreturn: _1y24h30d7dBaseCumulativePattern,
}

impl MetricsTree_Scripts_Value {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            opreturn: _1y24h30d7dBaseCumulativePattern::new(client.clone(), "opreturn_value".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Scripts_Adoption {
    pub taproot: MetricPattern1<StoredF32>,
    pub segwit: MetricPattern1<StoredF32>,
}

impl MetricsTree_Scripts_Adoption {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            taproot: MetricPattern1::new(client.clone(), "taproot_adoption".to_string()),
            segwit: MetricPattern1::new(client.clone(), "segwit_adoption".to_string()),
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
    pub coinbase: _1y24h30d7dBaseCumulativePattern,
    pub subsidy: _1y24h30d7dBaseCumulativePattern,
    pub fees: _1y24h30d7dBaseCumulativePattern,
    pub unclaimed_rewards: BaseCumulativeSumPattern,
    pub fee_dominance: MetricPattern1<StoredF32>,
    pub fee_dominance_24h: MetricPattern1<StoredF32>,
    pub fee_dominance_7d: MetricPattern1<StoredF32>,
    pub fee_dominance_30d: MetricPattern1<StoredF32>,
    pub fee_dominance_1y: MetricPattern1<StoredF32>,
    pub subsidy_dominance: MetricPattern1<StoredF32>,
    pub subsidy_dominance_24h: MetricPattern1<StoredF32>,
    pub subsidy_dominance_7d: MetricPattern1<StoredF32>,
    pub subsidy_dominance_30d: MetricPattern1<StoredF32>,
    pub subsidy_dominance_1y: MetricPattern1<StoredF32>,
    pub subsidy_usd_1y_sma: MetricPattern1<Dollars>,
}

impl MetricsTree_Mining_Rewards {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            coinbase: _1y24h30d7dBaseCumulativePattern::new(client.clone(), "coinbase".to_string()),
            subsidy: _1y24h30d7dBaseCumulativePattern::new(client.clone(), "subsidy".to_string()),
            fees: _1y24h30d7dBaseCumulativePattern::new(client.clone(), "fees".to_string()),
            unclaimed_rewards: BaseCumulativeSumPattern::new(client.clone(), "unclaimed_rewards".to_string()),
            fee_dominance: MetricPattern1::new(client.clone(), "fee_dominance".to_string()),
            fee_dominance_24h: MetricPattern1::new(client.clone(), "fee_dominance_24h".to_string()),
            fee_dominance_7d: MetricPattern1::new(client.clone(), "fee_dominance_7d".to_string()),
            fee_dominance_30d: MetricPattern1::new(client.clone(), "fee_dominance_30d".to_string()),
            fee_dominance_1y: MetricPattern1::new(client.clone(), "fee_dominance_1y".to_string()),
            subsidy_dominance: MetricPattern1::new(client.clone(), "subsidy_dominance".to_string()),
            subsidy_dominance_24h: MetricPattern1::new(client.clone(), "subsidy_dominance_24h".to_string()),
            subsidy_dominance_7d: MetricPattern1::new(client.clone(), "subsidy_dominance_7d".to_string()),
            subsidy_dominance_30d: MetricPattern1::new(client.clone(), "subsidy_dominance_30d".to_string()),
            subsidy_dominance_1y: MetricPattern1::new(client.clone(), "subsidy_dominance_1y".to_string()),
            subsidy_usd_1y_sma: MetricPattern1::new(client.clone(), "subsidy_usd_1y_sma".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Mining_Hashrate {
    pub hash_rate: MetricPattern1<StoredF64>,
    pub hash_rate_1w_sma: MetricPattern1<StoredF64>,
    pub hash_rate_1m_sma: MetricPattern1<StoredF32>,
    pub hash_rate_2m_sma: MetricPattern1<StoredF32>,
    pub hash_rate_1y_sma: MetricPattern1<StoredF32>,
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

impl MetricsTree_Mining_Hashrate {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            hash_rate: MetricPattern1::new(client.clone(), "hash_rate".to_string()),
            hash_rate_1w_sma: MetricPattern1::new(client.clone(), "hash_rate_1w_sma".to_string()),
            hash_rate_1m_sma: MetricPattern1::new(client.clone(), "hash_rate_1m_sma".to_string()),
            hash_rate_2m_sma: MetricPattern1::new(client.clone(), "hash_rate_2m_sma".to_string()),
            hash_rate_1y_sma: MetricPattern1::new(client.clone(), "hash_rate_1y_sma".to_string()),
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
pub struct MetricsTree_Positions {
    pub block_position: MetricPattern20<BlkPosition>,
    pub tx_position: MetricPattern21<BlkPosition>,
}

impl MetricsTree_Positions {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            block_position: MetricPattern20::new(client.clone(), "position".to_string()),
            tx_position: MetricPattern21::new(client.clone(), "position".to_string()),
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
    pub vaulted_supply: BtcSatsUsdPattern,
    pub active_supply: BtcSatsUsdPattern,
}

impl MetricsTree_Cointime_Supply {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            vaulted_supply: BtcSatsUsdPattern::new(client.clone(), "vaulted_supply".to_string()),
            active_supply: BtcSatsUsdPattern::new(client.clone(), "active_supply".to_string()),
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
    pub vaulted_price: SatsUsdPattern,
    pub vaulted_price_ratio: RatioPattern,
    pub active_price: SatsUsdPattern,
    pub active_price_ratio: RatioPattern,
    pub true_market_mean: SatsUsdPattern,
    pub true_market_mean_ratio: RatioPattern,
    pub cointime_price: SatsUsdPattern,
    pub cointime_price_ratio: RatioPattern,
}

impl MetricsTree_Cointime_Pricing {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            vaulted_price: SatsUsdPattern::new(client.clone(), "vaulted_price".to_string()),
            vaulted_price_ratio: RatioPattern::new(client.clone(), "vaulted_price_ratio".to_string()),
            active_price: SatsUsdPattern::new(client.clone(), "active_price".to_string()),
            active_price_ratio: RatioPattern::new(client.clone(), "active_price_ratio".to_string()),
            true_market_mean: SatsUsdPattern::new(client.clone(), "true_market_mean".to_string()),
            true_market_mean_ratio: RatioPattern::new(client.clone(), "true_market_mean_ratio".to_string()),
            cointime_price: SatsUsdPattern::new(client.clone(), "cointime_price".to_string()),
            cointime_price_ratio: RatioPattern::new(client.clone(), "cointime_price_ratio".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Cointime_Adjusted {
    pub cointime_adj_inflation_rate: MetricPattern1<StoredF32>,
    pub cointime_adj_tx_btc_velocity: MetricPattern1<StoredF64>,
    pub cointime_adj_tx_usd_velocity: MetricPattern1<StoredF64>,
}

impl MetricsTree_Cointime_Adjusted {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            cointime_adj_inflation_rate: MetricPattern1::new(client.clone(), "cointime_adj_inflation_rate".to_string()),
            cointime_adj_tx_btc_velocity: MetricPattern1::new(client.clone(), "cointime_adj_tx_btc_velocity".to_string()),
            cointime_adj_tx_usd_velocity: MetricPattern1::new(client.clone(), "cointime_adj_tx_usd_velocity".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Cointime_ReserveRisk {
    pub vocdd_365d_median: MetricPattern20<StoredF64>,
    pub hodl_bank: MetricPattern20<StoredF64>,
    pub reserve_risk: MetricPattern1<StoredF64>,
}

impl MetricsTree_Cointime_ReserveRisk {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            vocdd_365d_median: MetricPattern20::new(client.clone(), "vocdd_365d_median".to_string()),
            hodl_bank: MetricPattern20::new(client.clone(), "hodl_bank".to_string()),
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
    pub minute1: MetricsTree_Indexes_Minute1,
    pub minute5: MetricsTree_Indexes_Minute5,
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
            minute1: MetricsTree_Indexes_Minute1::new(client.clone(), format!("{base_path}_minute1")),
            minute5: MetricsTree_Indexes_Minute5::new(client.clone(), format!("{base_path}_minute5")),
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
    pub identity: MetricPattern28<P2PK33AddressIndex>,
}

impl MetricsTree_Indexes_Address_P2pk33 {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern28::new(client.clone(), "p2pk33addressindex".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Address_P2pk65 {
    pub identity: MetricPattern29<P2PK65AddressIndex>,
}

impl MetricsTree_Indexes_Address_P2pk65 {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern29::new(client.clone(), "p2pk65addressindex".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Address_P2pkh {
    pub identity: MetricPattern30<P2PKHAddressIndex>,
}

impl MetricsTree_Indexes_Address_P2pkh {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern30::new(client.clone(), "p2pkhaddressindex".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Address_P2sh {
    pub identity: MetricPattern31<P2SHAddressIndex>,
}

impl MetricsTree_Indexes_Address_P2sh {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern31::new(client.clone(), "p2shaddressindex".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Address_P2tr {
    pub identity: MetricPattern32<P2TRAddressIndex>,
}

impl MetricsTree_Indexes_Address_P2tr {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern32::new(client.clone(), "p2traddressindex".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Address_P2wpkh {
    pub identity: MetricPattern33<P2WPKHAddressIndex>,
}

impl MetricsTree_Indexes_Address_P2wpkh {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern33::new(client.clone(), "p2wpkhaddressindex".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Address_P2wsh {
    pub identity: MetricPattern34<P2WSHAddressIndex>,
}

impl MetricsTree_Indexes_Address_P2wsh {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern34::new(client.clone(), "p2wshaddressindex".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Address_P2a {
    pub identity: MetricPattern26<P2AAddressIndex>,
}

impl MetricsTree_Indexes_Address_P2a {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern26::new(client.clone(), "p2aaddressindex".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Address_P2ms {
    pub identity: MetricPattern27<P2MSOutputIndex>,
}

impl MetricsTree_Indexes_Address_P2ms {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern27::new(client.clone(), "p2msoutputindex".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Address_Empty {
    pub identity: MetricPattern24<EmptyOutputIndex>,
}

impl MetricsTree_Indexes_Address_Empty {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern24::new(client.clone(), "emptyoutputindex".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Address_Unknown {
    pub identity: MetricPattern35<UnknownOutputIndex>,
}

impl MetricsTree_Indexes_Address_Unknown {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern35::new(client.clone(), "unknownoutputindex".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Address_Opreturn {
    pub identity: MetricPattern25<OpReturnIndex>,
}

impl MetricsTree_Indexes_Address_Opreturn {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern25::new(client.clone(), "opreturnindex".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Height {
    pub identity: MetricPattern20<Height>,
    pub minute1: MetricPattern20<Minute1>,
    pub minute5: MetricPattern20<Minute5>,
    pub minute10: MetricPattern20<Minute10>,
    pub minute30: MetricPattern20<Minute30>,
    pub hour1: MetricPattern20<Hour1>,
    pub hour4: MetricPattern20<Hour4>,
    pub hour12: MetricPattern20<Hour12>,
    pub day1: MetricPattern20<Day1>,
    pub day3: MetricPattern20<Day3>,
    pub difficultyepoch: MetricPattern20<DifficultyEpoch>,
    pub halvingepoch: MetricPattern20<HalvingEpoch>,
    pub week1: MetricPattern20<Week1>,
    pub month1: MetricPattern20<Month1>,
    pub month3: MetricPattern20<Month3>,
    pub month6: MetricPattern20<Month6>,
    pub year1: MetricPattern20<Year1>,
    pub year10: MetricPattern20<Year10>,
    pub txindex_count: MetricPattern20<StoredU64>,
}

impl MetricsTree_Indexes_Height {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern20::new(client.clone(), "height".to_string()),
            minute1: MetricPattern20::new(client.clone(), "minute1".to_string()),
            minute5: MetricPattern20::new(client.clone(), "minute5".to_string()),
            minute10: MetricPattern20::new(client.clone(), "minute10".to_string()),
            minute30: MetricPattern20::new(client.clone(), "minute30".to_string()),
            hour1: MetricPattern20::new(client.clone(), "hour1".to_string()),
            hour4: MetricPattern20::new(client.clone(), "hour4".to_string()),
            hour12: MetricPattern20::new(client.clone(), "hour12".to_string()),
            day1: MetricPattern20::new(client.clone(), "day1".to_string()),
            day3: MetricPattern20::new(client.clone(), "day3".to_string()),
            difficultyepoch: MetricPattern20::new(client.clone(), "difficultyepoch".to_string()),
            halvingepoch: MetricPattern20::new(client.clone(), "halvingepoch".to_string()),
            week1: MetricPattern20::new(client.clone(), "week1".to_string()),
            month1: MetricPattern20::new(client.clone(), "month1".to_string()),
            month3: MetricPattern20::new(client.clone(), "month3".to_string()),
            month6: MetricPattern20::new(client.clone(), "month6".to_string()),
            year1: MetricPattern20::new(client.clone(), "year1".to_string()),
            year10: MetricPattern20::new(client.clone(), "year10".to_string()),
            txindex_count: MetricPattern20::new(client.clone(), "txindex_count".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Difficultyepoch {
    pub identity: MetricPattern19<DifficultyEpoch>,
    pub first_height: MetricPattern19<Height>,
    pub height_count: MetricPattern19<StoredU64>,
}

impl MetricsTree_Indexes_Difficultyepoch {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern19::new(client.clone(), "difficultyepoch".to_string()),
            first_height: MetricPattern19::new(client.clone(), "first_height".to_string()),
            height_count: MetricPattern19::new(client.clone(), "height_count".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Halvingepoch {
    pub identity: MetricPattern18<HalvingEpoch>,
    pub first_height: MetricPattern18<Height>,
}

impl MetricsTree_Indexes_Halvingepoch {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern18::new(client.clone(), "halvingepoch".to_string()),
            first_height: MetricPattern18::new(client.clone(), "first_height".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Minute1 {
    pub identity: MetricPattern3<Minute1>,
    pub first_height: MetricPattern3<Height>,
}

impl MetricsTree_Indexes_Minute1 {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern3::new(client.clone(), "minute1".to_string()),
            first_height: MetricPattern3::new(client.clone(), "minute1_first_height".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Minute5 {
    pub identity: MetricPattern4<Minute5>,
    pub first_height: MetricPattern4<Height>,
}

impl MetricsTree_Indexes_Minute5 {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern4::new(client.clone(), "minute5".to_string()),
            first_height: MetricPattern4::new(client.clone(), "minute5_first_height".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Minute10 {
    pub identity: MetricPattern5<Minute10>,
    pub first_height: MetricPattern5<Height>,
}

impl MetricsTree_Indexes_Minute10 {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern5::new(client.clone(), "minute10".to_string()),
            first_height: MetricPattern5::new(client.clone(), "minute10_first_height".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Minute30 {
    pub identity: MetricPattern6<Minute30>,
    pub first_height: MetricPattern6<Height>,
}

impl MetricsTree_Indexes_Minute30 {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern6::new(client.clone(), "minute30".to_string()),
            first_height: MetricPattern6::new(client.clone(), "minute30_first_height".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Hour1 {
    pub identity: MetricPattern7<Hour1>,
    pub first_height: MetricPattern7<Height>,
}

impl MetricsTree_Indexes_Hour1 {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern7::new(client.clone(), "hour1".to_string()),
            first_height: MetricPattern7::new(client.clone(), "hour1_first_height".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Hour4 {
    pub identity: MetricPattern8<Hour4>,
    pub first_height: MetricPattern8<Height>,
}

impl MetricsTree_Indexes_Hour4 {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern8::new(client.clone(), "hour4".to_string()),
            first_height: MetricPattern8::new(client.clone(), "hour4_first_height".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Hour12 {
    pub identity: MetricPattern9<Hour12>,
    pub first_height: MetricPattern9<Height>,
}

impl MetricsTree_Indexes_Hour12 {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern9::new(client.clone(), "hour12".to_string()),
            first_height: MetricPattern9::new(client.clone(), "hour12_first_height".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Day1 {
    pub identity: MetricPattern10<Day1>,
    pub date: MetricPattern10<Date>,
    pub first_height: MetricPattern10<Height>,
    pub height_count: MetricPattern10<StoredU64>,
}

impl MetricsTree_Indexes_Day1 {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern10::new(client.clone(), "day1".to_string()),
            date: MetricPattern10::new(client.clone(), "date".to_string()),
            first_height: MetricPattern10::new(client.clone(), "first_height".to_string()),
            height_count: MetricPattern10::new(client.clone(), "height_count".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Day3 {
    pub identity: MetricPattern11<Day3>,
    pub first_height: MetricPattern11<Height>,
}

impl MetricsTree_Indexes_Day3 {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern11::new(client.clone(), "day3".to_string()),
            first_height: MetricPattern11::new(client.clone(), "day3_first_height".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Week1 {
    pub identity: MetricPattern12<Week1>,
    pub date: MetricPattern12<Date>,
    pub first_height: MetricPattern12<Height>,
}

impl MetricsTree_Indexes_Week1 {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern12::new(client.clone(), "week1".to_string()),
            date: MetricPattern12::new(client.clone(), "date".to_string()),
            first_height: MetricPattern12::new(client.clone(), "week1_first_height".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Month1 {
    pub identity: MetricPattern13<Month1>,
    pub date: MetricPattern13<Date>,
    pub first_height: MetricPattern13<Height>,
}

impl MetricsTree_Indexes_Month1 {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern13::new(client.clone(), "month1".to_string()),
            date: MetricPattern13::new(client.clone(), "date".to_string()),
            first_height: MetricPattern13::new(client.clone(), "month1_first_height".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Month3 {
    pub identity: MetricPattern14<Month3>,
    pub date: MetricPattern14<Date>,
    pub first_height: MetricPattern14<Height>,
}

impl MetricsTree_Indexes_Month3 {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern14::new(client.clone(), "month3".to_string()),
            date: MetricPattern14::new(client.clone(), "date".to_string()),
            first_height: MetricPattern14::new(client.clone(), "month3_first_height".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Month6 {
    pub identity: MetricPattern15<Month6>,
    pub date: MetricPattern15<Date>,
    pub first_height: MetricPattern15<Height>,
}

impl MetricsTree_Indexes_Month6 {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern15::new(client.clone(), "month6".to_string()),
            date: MetricPattern15::new(client.clone(), "date".to_string()),
            first_height: MetricPattern15::new(client.clone(), "month6_first_height".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Year1 {
    pub identity: MetricPattern16<Year1>,
    pub date: MetricPattern16<Date>,
    pub first_height: MetricPattern16<Height>,
}

impl MetricsTree_Indexes_Year1 {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern16::new(client.clone(), "year1".to_string()),
            date: MetricPattern16::new(client.clone(), "date".to_string()),
            first_height: MetricPattern16::new(client.clone(), "year1_first_height".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Year10 {
    pub identity: MetricPattern17<Year10>,
    pub date: MetricPattern17<Date>,
    pub first_height: MetricPattern17<Height>,
}

impl MetricsTree_Indexes_Year10 {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern17::new(client.clone(), "year10".to_string()),
            date: MetricPattern17::new(client.clone(), "date".to_string()),
            first_height: MetricPattern17::new(client.clone(), "year10_first_height".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Txindex {
    pub identity: MetricPattern21<TxIndex>,
    pub input_count: MetricPattern21<StoredU64>,
    pub output_count: MetricPattern21<StoredU64>,
}

impl MetricsTree_Indexes_Txindex {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern21::new(client.clone(), "txindex".to_string()),
            input_count: MetricPattern21::new(client.clone(), "input_count".to_string()),
            output_count: MetricPattern21::new(client.clone(), "output_count".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Txinindex {
    pub identity: MetricPattern22<TxInIndex>,
}

impl MetricsTree_Indexes_Txinindex {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern22::new(client.clone(), "txinindex".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Indexes_Txoutindex {
    pub identity: MetricPattern23<TxOutIndex>,
}

impl MetricsTree_Indexes_Txoutindex {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            identity: MetricPattern23::new(client.clone(), "txoutindex".to_string()),
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
    pub price_ath: SatsUsdPattern,
    pub price_drawdown: MetricPattern1<StoredF32>,
    pub days_since_price_ath: MetricPattern1<StoredU16>,
    pub years_since_price_ath: MetricPattern2<StoredF32>,
    pub max_days_between_price_aths: MetricPattern1<StoredU16>,
    pub max_years_between_price_aths: MetricPattern2<StoredF32>,
}

impl MetricsTree_Market_Ath {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            price_ath: SatsUsdPattern::new(client.clone(), "price_ath".to_string()),
            price_drawdown: MetricPattern1::new(client.clone(), "price_drawdown".to_string()),
            days_since_price_ath: MetricPattern1::new(client.clone(), "days_since_price_ath".to_string()),
            years_since_price_ath: MetricPattern2::new(client.clone(), "years_since_price_ath".to_string()),
            max_days_between_price_aths: MetricPattern1::new(client.clone(), "max_days_between_price_aths".to_string()),
            max_years_between_price_aths: MetricPattern2::new(client.clone(), "max_years_between_price_aths".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Market_Lookback {
    pub _24h: SatsUsdPattern,
    pub _1w: SatsUsdPattern,
    pub _1m: SatsUsdPattern,
    pub _3m: SatsUsdPattern,
    pub _6m: SatsUsdPattern,
    pub _1y: SatsUsdPattern,
    pub _2y: SatsUsdPattern,
    pub _3y: SatsUsdPattern,
    pub _4y: SatsUsdPattern,
    pub _5y: SatsUsdPattern,
    pub _6y: SatsUsdPattern,
    pub _8y: SatsUsdPattern,
    pub _10y: SatsUsdPattern,
}

impl MetricsTree_Market_Lookback {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _24h: SatsUsdPattern::new(client.clone(), "price_24h_ago".to_string()),
            _1w: SatsUsdPattern::new(client.clone(), "price_1w_ago".to_string()),
            _1m: SatsUsdPattern::new(client.clone(), "price_1m_ago".to_string()),
            _3m: SatsUsdPattern::new(client.clone(), "price_3m_ago".to_string()),
            _6m: SatsUsdPattern::new(client.clone(), "price_6m_ago".to_string()),
            _1y: SatsUsdPattern::new(client.clone(), "price_1y_ago".to_string()),
            _2y: SatsUsdPattern::new(client.clone(), "price_2y_ago".to_string()),
            _3y: SatsUsdPattern::new(client.clone(), "price_3y_ago".to_string()),
            _4y: SatsUsdPattern::new(client.clone(), "price_4y_ago".to_string()),
            _5y: SatsUsdPattern::new(client.clone(), "price_5y_ago".to_string()),
            _6y: SatsUsdPattern::new(client.clone(), "price_6y_ago".to_string()),
            _8y: SatsUsdPattern::new(client.clone(), "price_8y_ago".to_string()),
            _10y: SatsUsdPattern::new(client.clone(), "price_10y_ago".to_string()),
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
    pub downside_returns: MetricPattern20<StoredF32>,
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
            downside_returns: MetricPattern20::new(client.clone(), "downside_returns".to_string()),
            downside_1w_sd: SdSmaPattern::new(client.clone(), "downside_1w_sd".to_string()),
            downside_1m_sd: SdSmaPattern::new(client.clone(), "downside_1m_sd".to_string()),
            downside_1y_sd: SdSmaPattern::new(client.clone(), "downside_1y_sd".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Market_Returns_PriceReturns {
    pub _24h: MetricPattern1<StoredF32>,
    pub _1w: MetricPattern1<StoredF32>,
    pub _1m: MetricPattern1<StoredF32>,
    pub _3m: MetricPattern1<StoredF32>,
    pub _6m: MetricPattern1<StoredF32>,
    pub _1y: MetricPattern1<StoredF32>,
    pub _2y: MetricPattern1<StoredF32>,
    pub _3y: MetricPattern1<StoredF32>,
    pub _4y: MetricPattern1<StoredF32>,
    pub _5y: MetricPattern1<StoredF32>,
    pub _6y: MetricPattern1<StoredF32>,
    pub _8y: MetricPattern1<StoredF32>,
    pub _10y: MetricPattern1<StoredF32>,
}

impl MetricsTree_Market_Returns_PriceReturns {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _24h: MetricPattern1::new(client.clone(), "24h_price_returns".to_string()),
            _1w: MetricPattern1::new(client.clone(), "1w_price_returns".to_string()),
            _1m: MetricPattern1::new(client.clone(), "1m_price_returns".to_string()),
            _3m: MetricPattern1::new(client.clone(), "3m_price_returns".to_string()),
            _6m: MetricPattern1::new(client.clone(), "6m_price_returns".to_string()),
            _1y: MetricPattern1::new(client.clone(), "1y_price_returns".to_string()),
            _2y: MetricPattern1::new(client.clone(), "2y_price_returns".to_string()),
            _3y: MetricPattern1::new(client.clone(), "3y_price_returns".to_string()),
            _4y: MetricPattern1::new(client.clone(), "4y_price_returns".to_string()),
            _5y: MetricPattern1::new(client.clone(), "5y_price_returns".to_string()),
            _6y: MetricPattern1::new(client.clone(), "6y_price_returns".to_string()),
            _8y: MetricPattern1::new(client.clone(), "8y_price_returns".to_string()),
            _10y: MetricPattern1::new(client.clone(), "10y_price_returns".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Market_Volatility {
    pub price_1w_volatility: MetricPattern1<StoredF32>,
    pub price_1m_volatility: MetricPattern1<StoredF32>,
    pub price_1y_volatility: MetricPattern1<StoredF32>,
    pub sharpe_1w: MetricPattern1<StoredF32>,
    pub sharpe_1m: MetricPattern1<StoredF32>,
    pub sharpe_1y: MetricPattern1<StoredF32>,
    pub sortino_1w: MetricPattern1<StoredF32>,
    pub sortino_1m: MetricPattern1<StoredF32>,
    pub sortino_1y: MetricPattern1<StoredF32>,
}

impl MetricsTree_Market_Volatility {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            price_1w_volatility: MetricPattern1::new(client.clone(), "price_1w_volatility".to_string()),
            price_1m_volatility: MetricPattern1::new(client.clone(), "price_1m_volatility".to_string()),
            price_1y_volatility: MetricPattern1::new(client.clone(), "price_1y_volatility".to_string()),
            sharpe_1w: MetricPattern1::new(client.clone(), "sharpe_1w".to_string()),
            sharpe_1m: MetricPattern1::new(client.clone(), "sharpe_1m".to_string()),
            sharpe_1y: MetricPattern1::new(client.clone(), "sharpe_1y".to_string()),
            sortino_1w: MetricPattern1::new(client.clone(), "sortino_1w".to_string()),
            sortino_1m: MetricPattern1::new(client.clone(), "sortino_1m".to_string()),
            sortino_1y: MetricPattern1::new(client.clone(), "sortino_1y".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Market_Range {
    pub price_1w_min: SatsUsdPattern,
    pub price_1w_max: SatsUsdPattern,
    pub price_2w_min: SatsUsdPattern,
    pub price_2w_max: SatsUsdPattern,
    pub price_1m_min: SatsUsdPattern,
    pub price_1m_max: SatsUsdPattern,
    pub price_1y_min: SatsUsdPattern,
    pub price_1y_max: SatsUsdPattern,
    pub price_true_range: MetricPattern1<StoredF32>,
    pub price_true_range_2w_sum: MetricPattern1<StoredF32>,
    pub price_2w_choppiness_index: MetricPattern1<StoredF32>,
}

impl MetricsTree_Market_Range {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            price_1w_min: SatsUsdPattern::new(client.clone(), "price_1w_min".to_string()),
            price_1w_max: SatsUsdPattern::new(client.clone(), "price_1w_max".to_string()),
            price_2w_min: SatsUsdPattern::new(client.clone(), "price_2w_min".to_string()),
            price_2w_max: SatsUsdPattern::new(client.clone(), "price_2w_max".to_string()),
            price_1m_min: SatsUsdPattern::new(client.clone(), "price_1m_min".to_string()),
            price_1m_max: SatsUsdPattern::new(client.clone(), "price_1m_max".to_string()),
            price_1y_min: SatsUsdPattern::new(client.clone(), "price_1y_min".to_string()),
            price_1y_max: SatsUsdPattern::new(client.clone(), "price_1y_max".to_string()),
            price_true_range: MetricPattern1::new(client.clone(), "price_true_range".to_string()),
            price_true_range_2w_sum: MetricPattern1::new(client.clone(), "price_true_range_2w_sum".to_string()),
            price_2w_choppiness_index: MetricPattern1::new(client.clone(), "price_2w_choppiness_index".to_string()),
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
    pub price_200d_sma_x2_4: SatsUsdPattern,
    pub price_200d_sma_x0_8: SatsUsdPattern,
    pub price_350d_sma_x2: SatsUsdPattern,
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
            price_200d_sma_x2_4: SatsUsdPattern::new(client.clone(), "price_200d_sma_x2_4".to_string()),
            price_200d_sma_x0_8: SatsUsdPattern::new(client.clone(), "price_200d_sma_x0_8".to_string()),
            price_350d_sma_x2: SatsUsdPattern::new(client.clone(), "price_350d_sma_x2".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Market_Dca {
    pub dca_sats_per_day: MetricPattern20<Sats>,
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
            dca_sats_per_day: MetricPattern20::new(client.clone(), "dca_sats_per_day".to_string()),
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
    pub _1w: SatsUsdPattern,
    pub _1m: SatsUsdPattern,
    pub _3m: SatsUsdPattern,
    pub _6m: SatsUsdPattern,
    pub _1y: SatsUsdPattern,
    pub _2y: SatsUsdPattern,
    pub _3y: SatsUsdPattern,
    pub _4y: SatsUsdPattern,
    pub _5y: SatsUsdPattern,
    pub _6y: SatsUsdPattern,
    pub _8y: SatsUsdPattern,
    pub _10y: SatsUsdPattern,
}

impl MetricsTree_Market_Dca_PeriodAveragePrice {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _1w: SatsUsdPattern::new(client.clone(), "1w_dca_average_price".to_string()),
            _1m: SatsUsdPattern::new(client.clone(), "1m_dca_average_price".to_string()),
            _3m: SatsUsdPattern::new(client.clone(), "3m_dca_average_price".to_string()),
            _6m: SatsUsdPattern::new(client.clone(), "6m_dca_average_price".to_string()),
            _1y: SatsUsdPattern::new(client.clone(), "1y_dca_average_price".to_string()),
            _2y: SatsUsdPattern::new(client.clone(), "2y_dca_average_price".to_string()),
            _3y: SatsUsdPattern::new(client.clone(), "3y_dca_average_price".to_string()),
            _4y: SatsUsdPattern::new(client.clone(), "4y_dca_average_price".to_string()),
            _5y: SatsUsdPattern::new(client.clone(), "5y_dca_average_price".to_string()),
            _6y: SatsUsdPattern::new(client.clone(), "6y_dca_average_price".to_string()),
            _8y: SatsUsdPattern::new(client.clone(), "8y_dca_average_price".to_string()),
            _10y: SatsUsdPattern::new(client.clone(), "10y_dca_average_price".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Market_Dca_ClassStack {
    pub _2015: BtcSatsUsdPattern,
    pub _2016: BtcSatsUsdPattern,
    pub _2017: BtcSatsUsdPattern,
    pub _2018: BtcSatsUsdPattern,
    pub _2019: BtcSatsUsdPattern,
    pub _2020: BtcSatsUsdPattern,
    pub _2021: BtcSatsUsdPattern,
    pub _2022: BtcSatsUsdPattern,
    pub _2023: BtcSatsUsdPattern,
    pub _2024: BtcSatsUsdPattern,
    pub _2025: BtcSatsUsdPattern,
    pub _2026: BtcSatsUsdPattern,
}

impl MetricsTree_Market_Dca_ClassStack {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _2015: BtcSatsUsdPattern::new(client.clone(), "dca_class_2015_stack".to_string()),
            _2016: BtcSatsUsdPattern::new(client.clone(), "dca_class_2016_stack".to_string()),
            _2017: BtcSatsUsdPattern::new(client.clone(), "dca_class_2017_stack".to_string()),
            _2018: BtcSatsUsdPattern::new(client.clone(), "dca_class_2018_stack".to_string()),
            _2019: BtcSatsUsdPattern::new(client.clone(), "dca_class_2019_stack".to_string()),
            _2020: BtcSatsUsdPattern::new(client.clone(), "dca_class_2020_stack".to_string()),
            _2021: BtcSatsUsdPattern::new(client.clone(), "dca_class_2021_stack".to_string()),
            _2022: BtcSatsUsdPattern::new(client.clone(), "dca_class_2022_stack".to_string()),
            _2023: BtcSatsUsdPattern::new(client.clone(), "dca_class_2023_stack".to_string()),
            _2024: BtcSatsUsdPattern::new(client.clone(), "dca_class_2024_stack".to_string()),
            _2025: BtcSatsUsdPattern::new(client.clone(), "dca_class_2025_stack".to_string()),
            _2026: BtcSatsUsdPattern::new(client.clone(), "dca_class_2026_stack".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Market_Dca_ClassAveragePrice {
    pub _2015: SatsUsdPattern,
    pub _2016: SatsUsdPattern,
    pub _2017: SatsUsdPattern,
    pub _2018: SatsUsdPattern,
    pub _2019: SatsUsdPattern,
    pub _2020: SatsUsdPattern,
    pub _2021: SatsUsdPattern,
    pub _2022: SatsUsdPattern,
    pub _2023: SatsUsdPattern,
    pub _2024: SatsUsdPattern,
    pub _2025: SatsUsdPattern,
    pub _2026: SatsUsdPattern,
}

impl MetricsTree_Market_Dca_ClassAveragePrice {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _2015: SatsUsdPattern::new(client.clone(), "dca_class_2015_average_price".to_string()),
            _2016: SatsUsdPattern::new(client.clone(), "dca_class_2016_average_price".to_string()),
            _2017: SatsUsdPattern::new(client.clone(), "dca_class_2017_average_price".to_string()),
            _2018: SatsUsdPattern::new(client.clone(), "dca_class_2018_average_price".to_string()),
            _2019: SatsUsdPattern::new(client.clone(), "dca_class_2019_average_price".to_string()),
            _2020: SatsUsdPattern::new(client.clone(), "dca_class_2020_average_price".to_string()),
            _2021: SatsUsdPattern::new(client.clone(), "dca_class_2021_average_price".to_string()),
            _2022: SatsUsdPattern::new(client.clone(), "dca_class_2022_average_price".to_string()),
            _2023: SatsUsdPattern::new(client.clone(), "dca_class_2023_average_price".to_string()),
            _2024: SatsUsdPattern::new(client.clone(), "dca_class_2024_average_price".to_string()),
            _2025: SatsUsdPattern::new(client.clone(), "dca_class_2025_average_price".to_string()),
            _2026: SatsUsdPattern::new(client.clone(), "dca_class_2026_average_price".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Market_Dca_ClassDaysInProfit {
    pub _2015: MetricPattern1<StoredU32>,
    pub _2016: MetricPattern1<StoredU32>,
    pub _2017: MetricPattern1<StoredU32>,
    pub _2018: MetricPattern1<StoredU32>,
    pub _2019: MetricPattern1<StoredU32>,
    pub _2020: MetricPattern1<StoredU32>,
    pub _2021: MetricPattern1<StoredU32>,
    pub _2022: MetricPattern1<StoredU32>,
    pub _2023: MetricPattern1<StoredU32>,
    pub _2024: MetricPattern1<StoredU32>,
    pub _2025: MetricPattern1<StoredU32>,
    pub _2026: MetricPattern1<StoredU32>,
}

impl MetricsTree_Market_Dca_ClassDaysInProfit {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _2015: MetricPattern1::new(client.clone(), "dca_class_2015_days_in_profit".to_string()),
            _2016: MetricPattern1::new(client.clone(), "dca_class_2016_days_in_profit".to_string()),
            _2017: MetricPattern1::new(client.clone(), "dca_class_2017_days_in_profit".to_string()),
            _2018: MetricPattern1::new(client.clone(), "dca_class_2018_days_in_profit".to_string()),
            _2019: MetricPattern1::new(client.clone(), "dca_class_2019_days_in_profit".to_string()),
            _2020: MetricPattern1::new(client.clone(), "dca_class_2020_days_in_profit".to_string()),
            _2021: MetricPattern1::new(client.clone(), "dca_class_2021_days_in_profit".to_string()),
            _2022: MetricPattern1::new(client.clone(), "dca_class_2022_days_in_profit".to_string()),
            _2023: MetricPattern1::new(client.clone(), "dca_class_2023_days_in_profit".to_string()),
            _2024: MetricPattern1::new(client.clone(), "dca_class_2024_days_in_profit".to_string()),
            _2025: MetricPattern1::new(client.clone(), "dca_class_2025_days_in_profit".to_string()),
            _2026: MetricPattern1::new(client.clone(), "dca_class_2026_days_in_profit".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Market_Dca_ClassDaysInLoss {
    pub _2015: MetricPattern1<StoredU32>,
    pub _2016: MetricPattern1<StoredU32>,
    pub _2017: MetricPattern1<StoredU32>,
    pub _2018: MetricPattern1<StoredU32>,
    pub _2019: MetricPattern1<StoredU32>,
    pub _2020: MetricPattern1<StoredU32>,
    pub _2021: MetricPattern1<StoredU32>,
    pub _2022: MetricPattern1<StoredU32>,
    pub _2023: MetricPattern1<StoredU32>,
    pub _2024: MetricPattern1<StoredU32>,
    pub _2025: MetricPattern1<StoredU32>,
    pub _2026: MetricPattern1<StoredU32>,
}

impl MetricsTree_Market_Dca_ClassDaysInLoss {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _2015: MetricPattern1::new(client.clone(), "dca_class_2015_days_in_loss".to_string()),
            _2016: MetricPattern1::new(client.clone(), "dca_class_2016_days_in_loss".to_string()),
            _2017: MetricPattern1::new(client.clone(), "dca_class_2017_days_in_loss".to_string()),
            _2018: MetricPattern1::new(client.clone(), "dca_class_2018_days_in_loss".to_string()),
            _2019: MetricPattern1::new(client.clone(), "dca_class_2019_days_in_loss".to_string()),
            _2020: MetricPattern1::new(client.clone(), "dca_class_2020_days_in_loss".to_string()),
            _2021: MetricPattern1::new(client.clone(), "dca_class_2021_days_in_loss".to_string()),
            _2022: MetricPattern1::new(client.clone(), "dca_class_2022_days_in_loss".to_string()),
            _2023: MetricPattern1::new(client.clone(), "dca_class_2023_days_in_loss".to_string()),
            _2024: MetricPattern1::new(client.clone(), "dca_class_2024_days_in_loss".to_string()),
            _2025: MetricPattern1::new(client.clone(), "dca_class_2025_days_in_loss".to_string()),
            _2026: MetricPattern1::new(client.clone(), "dca_class_2026_days_in_loss".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Market_Dca_ClassMinReturn {
    pub _2015: MetricPattern1<StoredF32>,
    pub _2016: MetricPattern1<StoredF32>,
    pub _2017: MetricPattern1<StoredF32>,
    pub _2018: MetricPattern1<StoredF32>,
    pub _2019: MetricPattern1<StoredF32>,
    pub _2020: MetricPattern1<StoredF32>,
    pub _2021: MetricPattern1<StoredF32>,
    pub _2022: MetricPattern1<StoredF32>,
    pub _2023: MetricPattern1<StoredF32>,
    pub _2024: MetricPattern1<StoredF32>,
    pub _2025: MetricPattern1<StoredF32>,
    pub _2026: MetricPattern1<StoredF32>,
}

impl MetricsTree_Market_Dca_ClassMinReturn {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _2015: MetricPattern1::new(client.clone(), "dca_class_2015_min_return".to_string()),
            _2016: MetricPattern1::new(client.clone(), "dca_class_2016_min_return".to_string()),
            _2017: MetricPattern1::new(client.clone(), "dca_class_2017_min_return".to_string()),
            _2018: MetricPattern1::new(client.clone(), "dca_class_2018_min_return".to_string()),
            _2019: MetricPattern1::new(client.clone(), "dca_class_2019_min_return".to_string()),
            _2020: MetricPattern1::new(client.clone(), "dca_class_2020_min_return".to_string()),
            _2021: MetricPattern1::new(client.clone(), "dca_class_2021_min_return".to_string()),
            _2022: MetricPattern1::new(client.clone(), "dca_class_2022_min_return".to_string()),
            _2023: MetricPattern1::new(client.clone(), "dca_class_2023_min_return".to_string()),
            _2024: MetricPattern1::new(client.clone(), "dca_class_2024_min_return".to_string()),
            _2025: MetricPattern1::new(client.clone(), "dca_class_2025_min_return".to_string()),
            _2026: MetricPattern1::new(client.clone(), "dca_class_2026_min_return".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Market_Dca_ClassMaxReturn {
    pub _2015: MetricPattern1<StoredF32>,
    pub _2016: MetricPattern1<StoredF32>,
    pub _2017: MetricPattern1<StoredF32>,
    pub _2018: MetricPattern1<StoredF32>,
    pub _2019: MetricPattern1<StoredF32>,
    pub _2020: MetricPattern1<StoredF32>,
    pub _2021: MetricPattern1<StoredF32>,
    pub _2022: MetricPattern1<StoredF32>,
    pub _2023: MetricPattern1<StoredF32>,
    pub _2024: MetricPattern1<StoredF32>,
    pub _2025: MetricPattern1<StoredF32>,
    pub _2026: MetricPattern1<StoredF32>,
}

impl MetricsTree_Market_Dca_ClassMaxReturn {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _2015: MetricPattern1::new(client.clone(), "dca_class_2015_max_return".to_string()),
            _2016: MetricPattern1::new(client.clone(), "dca_class_2016_max_return".to_string()),
            _2017: MetricPattern1::new(client.clone(), "dca_class_2017_max_return".to_string()),
            _2018: MetricPattern1::new(client.clone(), "dca_class_2018_max_return".to_string()),
            _2019: MetricPattern1::new(client.clone(), "dca_class_2019_max_return".to_string()),
            _2020: MetricPattern1::new(client.clone(), "dca_class_2020_max_return".to_string()),
            _2021: MetricPattern1::new(client.clone(), "dca_class_2021_max_return".to_string()),
            _2022: MetricPattern1::new(client.clone(), "dca_class_2022_max_return".to_string()),
            _2023: MetricPattern1::new(client.clone(), "dca_class_2023_max_return".to_string()),
            _2024: MetricPattern1::new(client.clone(), "dca_class_2024_max_return".to_string()),
            _2025: MetricPattern1::new(client.clone(), "dca_class_2025_max_return".to_string()),
            _2026: MetricPattern1::new(client.clone(), "dca_class_2026_max_return".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Market_Indicators {
    pub puell_multiple: MetricPattern1<StoredF32>,
    pub nvt: MetricPattern1<StoredF32>,
    pub rsi: MetricsTree_Market_Indicators_Rsi,
    pub stoch_k: MetricPattern1<StoredF32>,
    pub stoch_d: MetricPattern1<StoredF32>,
    pub pi_cycle: MetricPattern1<StoredF32>,
    pub macd: MetricsTree_Market_Indicators_Macd,
    pub gini: MetricPattern1<StoredF32>,
}

impl MetricsTree_Market_Indicators {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            puell_multiple: MetricPattern1::new(client.clone(), "puell_multiple".to_string()),
            nvt: MetricPattern1::new(client.clone(), "nvt".to_string()),
            rsi: MetricsTree_Market_Indicators_Rsi::new(client.clone(), format!("{base_path}_rsi")),
            stoch_k: MetricPattern1::new(client.clone(), "stoch_k".to_string()),
            stoch_d: MetricPattern1::new(client.clone(), "stoch_d".to_string()),
            pi_cycle: MetricPattern1::new(client.clone(), "pi_cycle".to_string()),
            macd: MetricsTree_Market_Indicators_Macd::new(client.clone(), format!("{base_path}_macd")),
            gini: MetricPattern1::new(client.clone(), "gini".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Market_Indicators_Rsi {
    pub _1d: MetricsTree_Market_Indicators_Rsi_1d,
    pub _1w: MetricsTree_Market_Indicators_Rsi_1w,
    pub _1m: MetricsTree_Market_Indicators_Rsi_1m,
    pub _1y: AverageGainsLossesRsiStochPattern,
}

impl MetricsTree_Market_Indicators_Rsi {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _1d: MetricsTree_Market_Indicators_Rsi_1d::new(client.clone(), format!("{base_path}_1d")),
            _1w: MetricsTree_Market_Indicators_Rsi_1w::new(client.clone(), format!("{base_path}_1w")),
            _1m: MetricsTree_Market_Indicators_Rsi_1m::new(client.clone(), format!("{base_path}_1m")),
            _1y: AverageGainsLossesRsiStochPattern::new(client.clone(), "rsi".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Market_Indicators_Rsi_1d {
    pub gains: MetricPattern1<StoredF32>,
    pub losses: MetricPattern1<StoredF32>,
    pub average_gain: MetricPattern1<StoredF32>,
    pub average_loss: MetricPattern1<StoredF32>,
    pub rsi: MetricPattern1<StoredF32>,
    pub rsi_min: MetricPattern1<StoredF32>,
    pub rsi_max: MetricPattern1<StoredF32>,
    pub stoch_rsi: MetricPattern1<StoredF32>,
    pub stoch_rsi_k: MetricPattern1<StoredF32>,
    pub stoch_rsi_d: MetricPattern1<StoredF32>,
}

impl MetricsTree_Market_Indicators_Rsi_1d {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            gains: MetricPattern1::new(client.clone(), "rsi_gains_1d".to_string()),
            losses: MetricPattern1::new(client.clone(), "rsi_losses_1d".to_string()),
            average_gain: MetricPattern1::new(client.clone(), "rsi_avg_gain_1d".to_string()),
            average_loss: MetricPattern1::new(client.clone(), "rsi_avg_loss_1d".to_string()),
            rsi: MetricPattern1::new(client.clone(), "rsi_1d".to_string()),
            rsi_min: MetricPattern1::new(client.clone(), "rsi_rsi_min_1d".to_string()),
            rsi_max: MetricPattern1::new(client.clone(), "rsi_rsi_max_1d".to_string()),
            stoch_rsi: MetricPattern1::new(client.clone(), "rsi_stoch_rsi_1d".to_string()),
            stoch_rsi_k: MetricPattern1::new(client.clone(), "rsi_stoch_rsi_k_1d".to_string()),
            stoch_rsi_d: MetricPattern1::new(client.clone(), "rsi_stoch_rsi_d_1d".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Market_Indicators_Rsi_1w {
    pub gains: MetricPattern1<StoredF32>,
    pub losses: MetricPattern1<StoredF32>,
    pub average_gain: MetricPattern1<StoredF32>,
    pub average_loss: MetricPattern1<StoredF32>,
    pub rsi: MetricPattern1<StoredF32>,
    pub rsi_min: MetricPattern1<StoredF32>,
    pub rsi_max: MetricPattern1<StoredF32>,
    pub stoch_rsi: MetricPattern1<StoredF32>,
    pub stoch_rsi_k: MetricPattern1<StoredF32>,
    pub stoch_rsi_d: MetricPattern1<StoredF32>,
}

impl MetricsTree_Market_Indicators_Rsi_1w {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            gains: MetricPattern1::new(client.clone(), "rsi_gains_1w".to_string()),
            losses: MetricPattern1::new(client.clone(), "rsi_losses_1w".to_string()),
            average_gain: MetricPattern1::new(client.clone(), "rsi_avg_gain_1w".to_string()),
            average_loss: MetricPattern1::new(client.clone(), "rsi_avg_loss_1w".to_string()),
            rsi: MetricPattern1::new(client.clone(), "rsi_1w".to_string()),
            rsi_min: MetricPattern1::new(client.clone(), "rsi_rsi_min_1w".to_string()),
            rsi_max: MetricPattern1::new(client.clone(), "rsi_rsi_max_1w".to_string()),
            stoch_rsi: MetricPattern1::new(client.clone(), "rsi_stoch_rsi_1w".to_string()),
            stoch_rsi_k: MetricPattern1::new(client.clone(), "rsi_stoch_rsi_k_1w".to_string()),
            stoch_rsi_d: MetricPattern1::new(client.clone(), "rsi_stoch_rsi_d_1w".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Market_Indicators_Rsi_1m {
    pub gains: MetricPattern1<StoredF32>,
    pub losses: MetricPattern1<StoredF32>,
    pub average_gain: MetricPattern1<StoredF32>,
    pub average_loss: MetricPattern1<StoredF32>,
    pub rsi: MetricPattern1<StoredF32>,
    pub rsi_min: MetricPattern1<StoredF32>,
    pub rsi_max: MetricPattern1<StoredF32>,
    pub stoch_rsi: MetricPattern1<StoredF32>,
    pub stoch_rsi_k: MetricPattern1<StoredF32>,
    pub stoch_rsi_d: MetricPattern1<StoredF32>,
}

impl MetricsTree_Market_Indicators_Rsi_1m {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            gains: MetricPattern1::new(client.clone(), "rsi_gains_1m".to_string()),
            losses: MetricPattern1::new(client.clone(), "rsi_losses_1m".to_string()),
            average_gain: MetricPattern1::new(client.clone(), "rsi_avg_gain_1m".to_string()),
            average_loss: MetricPattern1::new(client.clone(), "rsi_avg_loss_1m".to_string()),
            rsi: MetricPattern1::new(client.clone(), "rsi_1m".to_string()),
            rsi_min: MetricPattern1::new(client.clone(), "rsi_rsi_min_1m".to_string()),
            rsi_max: MetricPattern1::new(client.clone(), "rsi_rsi_max_1m".to_string()),
            stoch_rsi: MetricPattern1::new(client.clone(), "rsi_stoch_rsi_1m".to_string()),
            stoch_rsi_k: MetricPattern1::new(client.clone(), "rsi_stoch_rsi_k_1m".to_string()),
            stoch_rsi_d: MetricPattern1::new(client.clone(), "rsi_stoch_rsi_d_1m".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Market_Indicators_Macd {
    pub _1d: MetricsTree_Market_Indicators_Macd_1d,
    pub _1w: MetricsTree_Market_Indicators_Macd_1w,
    pub _1m: MetricsTree_Market_Indicators_Macd_1m,
    pub _1y: HistogramLineSignalPattern,
}

impl MetricsTree_Market_Indicators_Macd {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _1d: MetricsTree_Market_Indicators_Macd_1d::new(client.clone(), format!("{base_path}_1d")),
            _1w: MetricsTree_Market_Indicators_Macd_1w::new(client.clone(), format!("{base_path}_1w")),
            _1m: MetricsTree_Market_Indicators_Macd_1m::new(client.clone(), format!("{base_path}_1m")),
            _1y: HistogramLineSignalPattern::new(client.clone(), "macd".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Market_Indicators_Macd_1d {
    pub line: MetricPattern1<StoredF32>,
    pub signal: MetricPattern1<StoredF32>,
    pub histogram: MetricPattern1<StoredF32>,
}

impl MetricsTree_Market_Indicators_Macd_1d {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            line: MetricPattern1::new(client.clone(), "macd_line_1d".to_string()),
            signal: MetricPattern1::new(client.clone(), "macd_signal_1d".to_string()),
            histogram: MetricPattern1::new(client.clone(), "macd_histogram_1d".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Market_Indicators_Macd_1w {
    pub line: MetricPattern1<StoredF32>,
    pub signal: MetricPattern1<StoredF32>,
    pub histogram: MetricPattern1<StoredF32>,
}

impl MetricsTree_Market_Indicators_Macd_1w {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            line: MetricPattern1::new(client.clone(), "macd_line_1w".to_string()),
            signal: MetricPattern1::new(client.clone(), "macd_signal_1w".to_string()),
            histogram: MetricPattern1::new(client.clone(), "macd_histogram_1w".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Market_Indicators_Macd_1m {
    pub line: MetricPattern1<StoredF32>,
    pub signal: MetricPattern1<StoredF32>,
    pub histogram: MetricPattern1<StoredF32>,
}

impl MetricsTree_Market_Indicators_Macd_1m {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            line: MetricPattern1::new(client.clone(), "macd_line_1m".to_string()),
            signal: MetricPattern1::new(client.clone(), "macd_signal_1m".to_string()),
            histogram: MetricPattern1::new(client.clone(), "macd_histogram_1m".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Pools {
    pub height_to_pool: MetricPattern20<PoolSlug>,
    pub vecs: MetricsTree_Pools_Vecs,
}

impl MetricsTree_Pools {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            height_to_pool: MetricPattern20::new(client.clone(), "pool".to_string()),
            vecs: MetricsTree_Pools_Vecs::new(client.clone(), format!("{base_path}_vecs")),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Pools_Vecs {
    pub unknown: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub blockfills: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub ultimuspool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub terrapool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub luxor: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub onethash: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub btccom: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub bitfarms: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub huobipool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub wayicn: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub canoepool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub btctop: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub bitcoincom: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub pool175btc: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub gbminers: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub axbt: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub asicminer: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub bitminter: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub bitcoinrussia: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub btcserv: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub simplecoinus: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub btcguild: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub eligius: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub ozcoin: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub eclipsemc: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub maxbtc: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub triplemining: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub coinlab: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub pool50btc: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub ghashio: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub stminingcorp: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub bitparking: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub mmpool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub polmine: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub kncminer: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub bitalo: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub f2pool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub hhtt: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub megabigpower: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub mtred: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub nmcbit: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub yourbtcnet: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub givemecoins: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub braiinspool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub antpool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub multicoinco: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub bcpoolio: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub cointerra: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub kanopool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub solock: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub ckpool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub nicehash: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub bitclub: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub bitcoinaffiliatenetwork: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub btcc: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub bwpool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub exxbw: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub bitsolo: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub bitfury: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub twentyoneinc: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub digitalbtc: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub eightbaochi: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub mybtccoinpool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub tbdice: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub hashpool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub nexious: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub bravomining: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub hotpool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub okexpool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub bcmonster: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub onehash: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub bixin: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub tatmaspool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub viabtc: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub connectbtc: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub batpool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub waterhole: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub dcexploration: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub dcex: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub btpool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub fiftyeightcoin: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub bitcoinindia: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub shawnp0wers: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub phashio: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub rigpool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub haozhuzhu: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub sevenpool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub miningkings: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub hashbx: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub dpool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub rawpool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub haominer: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub helix: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub bitcoinukraine: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub poolin: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub secretsuperstar: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub tigerpoolnet: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub sigmapoolcom: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub okpooltop: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub hummerpool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub tangpool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub bytepool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub spiderpool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub novablock: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub miningcity: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub binancepool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub minerium: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub lubiancom: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub okkong: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub aaopool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub emcdpool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub foundryusa: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub sbicrypto: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub arkpool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub purebtccom: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub marapool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub kucoinpool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub entrustcharitypool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub okminer: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub titan: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub pegapool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub btcnuggets: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub cloudhashing: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub digitalxmintsy: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub telco214: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub btcpoolparty: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub multipool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub transactioncoinmining: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub btcdig: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub trickysbtcpool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub btcmp: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub eobot: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub unomp: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub patels: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub gogreenlight: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub bitcoinindiapool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub ekanembtc: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub canoe: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub tiger: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub onem1x: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub zulupool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub secpool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub ocean: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub whitepool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub wiz: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub wk057: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub futurebitapollosolo: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub carbonnegative: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub portlandhodl: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub phoenix: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub neopool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub maxipool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub bitfufupool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub gdpool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub miningdutch: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub publicpool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub miningsquared: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub innopolistech: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub btclab: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub parasite: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub redrockpool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
    pub est3lar: BlocksCoinbaseDaysDominanceFeeSubsidyPattern,
}

impl MetricsTree_Pools_Vecs {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            unknown: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "unknown".to_string()),
            blockfills: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "blockfills".to_string()),
            ultimuspool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "ultimuspool".to_string()),
            terrapool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "terrapool".to_string()),
            luxor: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "luxor".to_string()),
            onethash: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "onethash".to_string()),
            btccom: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "btccom".to_string()),
            bitfarms: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "bitfarms".to_string()),
            huobipool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "huobipool".to_string()),
            wayicn: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "wayicn".to_string()),
            canoepool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "canoepool".to_string()),
            btctop: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "btctop".to_string()),
            bitcoincom: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "bitcoincom".to_string()),
            pool175btc: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "pool175btc".to_string()),
            gbminers: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "gbminers".to_string()),
            axbt: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "axbt".to_string()),
            asicminer: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "asicminer".to_string()),
            bitminter: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "bitminter".to_string()),
            bitcoinrussia: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "bitcoinrussia".to_string()),
            btcserv: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "btcserv".to_string()),
            simplecoinus: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "simplecoinus".to_string()),
            btcguild: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "btcguild".to_string()),
            eligius: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "eligius".to_string()),
            ozcoin: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "ozcoin".to_string()),
            eclipsemc: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "eclipsemc".to_string()),
            maxbtc: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "maxbtc".to_string()),
            triplemining: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "triplemining".to_string()),
            coinlab: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "coinlab".to_string()),
            pool50btc: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "pool50btc".to_string()),
            ghashio: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "ghashio".to_string()),
            stminingcorp: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "stminingcorp".to_string()),
            bitparking: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "bitparking".to_string()),
            mmpool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "mmpool".to_string()),
            polmine: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "polmine".to_string()),
            kncminer: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "kncminer".to_string()),
            bitalo: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "bitalo".to_string()),
            f2pool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "f2pool".to_string()),
            hhtt: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "hhtt".to_string()),
            megabigpower: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "megabigpower".to_string()),
            mtred: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "mtred".to_string()),
            nmcbit: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "nmcbit".to_string()),
            yourbtcnet: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "yourbtcnet".to_string()),
            givemecoins: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "givemecoins".to_string()),
            braiinspool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "braiinspool".to_string()),
            antpool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "antpool".to_string()),
            multicoinco: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "multicoinco".to_string()),
            bcpoolio: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "bcpoolio".to_string()),
            cointerra: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "cointerra".to_string()),
            kanopool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "kanopool".to_string()),
            solock: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "solock".to_string()),
            ckpool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "ckpool".to_string()),
            nicehash: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "nicehash".to_string()),
            bitclub: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "bitclub".to_string()),
            bitcoinaffiliatenetwork: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "bitcoinaffiliatenetwork".to_string()),
            btcc: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "btcc".to_string()),
            bwpool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "bwpool".to_string()),
            exxbw: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "exxbw".to_string()),
            bitsolo: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "bitsolo".to_string()),
            bitfury: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "bitfury".to_string()),
            twentyoneinc: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "twentyoneinc".to_string()),
            digitalbtc: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "digitalbtc".to_string()),
            eightbaochi: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "eightbaochi".to_string()),
            mybtccoinpool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "mybtccoinpool".to_string()),
            tbdice: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "tbdice".to_string()),
            hashpool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "hashpool".to_string()),
            nexious: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "nexious".to_string()),
            bravomining: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "bravomining".to_string()),
            hotpool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "hotpool".to_string()),
            okexpool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "okexpool".to_string()),
            bcmonster: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "bcmonster".to_string()),
            onehash: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "onehash".to_string()),
            bixin: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "bixin".to_string()),
            tatmaspool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "tatmaspool".to_string()),
            viabtc: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "viabtc".to_string()),
            connectbtc: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "connectbtc".to_string()),
            batpool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "batpool".to_string()),
            waterhole: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "waterhole".to_string()),
            dcexploration: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "dcexploration".to_string()),
            dcex: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "dcex".to_string()),
            btpool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "btpool".to_string()),
            fiftyeightcoin: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "fiftyeightcoin".to_string()),
            bitcoinindia: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "bitcoinindia".to_string()),
            shawnp0wers: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "shawnp0wers".to_string()),
            phashio: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "phashio".to_string()),
            rigpool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "rigpool".to_string()),
            haozhuzhu: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "haozhuzhu".to_string()),
            sevenpool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "sevenpool".to_string()),
            miningkings: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "miningkings".to_string()),
            hashbx: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "hashbx".to_string()),
            dpool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "dpool".to_string()),
            rawpool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "rawpool".to_string()),
            haominer: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "haominer".to_string()),
            helix: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "helix".to_string()),
            bitcoinukraine: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "bitcoinukraine".to_string()),
            poolin: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "poolin".to_string()),
            secretsuperstar: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "secretsuperstar".to_string()),
            tigerpoolnet: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "tigerpoolnet".to_string()),
            sigmapoolcom: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "sigmapoolcom".to_string()),
            okpooltop: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "okpooltop".to_string()),
            hummerpool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "hummerpool".to_string()),
            tangpool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "tangpool".to_string()),
            bytepool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "bytepool".to_string()),
            spiderpool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "spiderpool".to_string()),
            novablock: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "novablock".to_string()),
            miningcity: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "miningcity".to_string()),
            binancepool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "binancepool".to_string()),
            minerium: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "minerium".to_string()),
            lubiancom: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "lubiancom".to_string()),
            okkong: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "okkong".to_string()),
            aaopool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "aaopool".to_string()),
            emcdpool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "emcdpool".to_string()),
            foundryusa: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "foundryusa".to_string()),
            sbicrypto: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "sbicrypto".to_string()),
            arkpool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "arkpool".to_string()),
            purebtccom: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "purebtccom".to_string()),
            marapool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "marapool".to_string()),
            kucoinpool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "kucoinpool".to_string()),
            entrustcharitypool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "entrustcharitypool".to_string()),
            okminer: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "okminer".to_string()),
            titan: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "titan".to_string()),
            pegapool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "pegapool".to_string()),
            btcnuggets: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "btcnuggets".to_string()),
            cloudhashing: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "cloudhashing".to_string()),
            digitalxmintsy: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "digitalxmintsy".to_string()),
            telco214: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "telco214".to_string()),
            btcpoolparty: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "btcpoolparty".to_string()),
            multipool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "multipool".to_string()),
            transactioncoinmining: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "transactioncoinmining".to_string()),
            btcdig: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "btcdig".to_string()),
            trickysbtcpool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "trickysbtcpool".to_string()),
            btcmp: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "btcmp".to_string()),
            eobot: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "eobot".to_string()),
            unomp: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "unomp".to_string()),
            patels: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "patels".to_string()),
            gogreenlight: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "gogreenlight".to_string()),
            bitcoinindiapool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "bitcoinindiapool".to_string()),
            ekanembtc: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "ekanembtc".to_string()),
            canoe: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "canoe".to_string()),
            tiger: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "tiger".to_string()),
            onem1x: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "onem1x".to_string()),
            zulupool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "zulupool".to_string()),
            secpool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "secpool".to_string()),
            ocean: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "ocean".to_string()),
            whitepool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "whitepool".to_string()),
            wiz: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "wiz".to_string()),
            wk057: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "wk057".to_string()),
            futurebitapollosolo: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "futurebitapollosolo".to_string()),
            carbonnegative: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "carbonnegative".to_string()),
            portlandhodl: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "portlandhodl".to_string()),
            phoenix: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "phoenix".to_string()),
            neopool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "neopool".to_string()),
            maxipool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "maxipool".to_string()),
            bitfufupool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "bitfufupool".to_string()),
            gdpool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "gdpool".to_string()),
            miningdutch: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "miningdutch".to_string()),
            publicpool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "publicpool".to_string()),
            miningsquared: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "miningsquared".to_string()),
            innopolistech: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "innopolistech".to_string()),
            btclab: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "btclab".to_string()),
            parasite: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "parasite".to_string()),
            redrockpool: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "redrockpool".to_string()),
            est3lar: BlocksCoinbaseDaysDominanceFeeSubsidyPattern::new(client.clone(), "est3lar".to_string()),
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
    pub open: CentsSatsUsdPattern,
    pub high: CentsSatsUsdPattern,
    pub low: CentsSatsUsdPattern,
    pub close: MetricsTree_Prices_Split_Close,
}

impl MetricsTree_Prices_Split {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            open: CentsSatsUsdPattern::new(client.clone(), "price_open".to_string()),
            high: CentsSatsUsdPattern::new(client.clone(), "price_high".to_string()),
            low: CentsSatsUsdPattern::new(client.clone(), "price_low".to_string()),
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
    pub supply_state: MetricPattern20<SupplyState>,
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
    pub fundedaddressindex: MetricPattern36<FundedAddressIndex>,
    pub emptyaddressindex: MetricPattern37<EmptyAddressIndex>,
}

impl MetricsTree_Distribution {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            supply_state: MetricPattern20::new(client.clone(), "supply_state".to_string()),
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
            fundedaddressindex: MetricPattern36::new(client.clone(), "fundedaddressindex".to_string()),
            emptyaddressindex: MetricPattern37::new(client.clone(), "emptyaddressindex".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Distribution_AnyAddressIndexes {
    pub p2a: MetricPattern26<AnyAddressIndex>,
    pub p2pk33: MetricPattern28<AnyAddressIndex>,
    pub p2pk65: MetricPattern29<AnyAddressIndex>,
    pub p2pkh: MetricPattern30<AnyAddressIndex>,
    pub p2sh: MetricPattern31<AnyAddressIndex>,
    pub p2tr: MetricPattern32<AnyAddressIndex>,
    pub p2wpkh: MetricPattern33<AnyAddressIndex>,
    pub p2wsh: MetricPattern34<AnyAddressIndex>,
}

impl MetricsTree_Distribution_AnyAddressIndexes {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            p2a: MetricPattern26::new(client.clone(), "anyaddressindex".to_string()),
            p2pk33: MetricPattern28::new(client.clone(), "anyaddressindex".to_string()),
            p2pk65: MetricPattern29::new(client.clone(), "anyaddressindex".to_string()),
            p2pkh: MetricPattern30::new(client.clone(), "anyaddressindex".to_string()),
            p2sh: MetricPattern31::new(client.clone(), "anyaddressindex".to_string()),
            p2tr: MetricPattern32::new(client.clone(), "anyaddressindex".to_string()),
            p2wpkh: MetricPattern33::new(client.clone(), "anyaddressindex".to_string()),
            p2wsh: MetricPattern34::new(client.clone(), "anyaddressindex".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Distribution_AddressesData {
    pub funded: MetricPattern36<FundedAddressData>,
    pub empty: MetricPattern37<EmptyAddressData>,
}

impl MetricsTree_Distribution_AddressesData {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            funded: MetricPattern36::new(client.clone(), "fundedaddressdata".to_string()),
            empty: MetricPattern37::new(client.clone(), "emptyaddressdata".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Distribution_UtxoCohorts {
    pub all: MetricsTree_Distribution_UtxoCohorts_All,
    pub sth: MetricsTree_Distribution_UtxoCohorts_Sth,
    pub lth: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern,
    pub age_range: MetricsTree_Distribution_UtxoCohorts_AgeRange,
    pub max_age: MetricsTree_Distribution_UtxoCohorts_MaxAge,
    pub min_age: MetricsTree_Distribution_UtxoCohorts_MinAge,
    pub ge_amount: MetricsTree_Distribution_UtxoCohorts_GeAmount,
    pub amount_range: MetricsTree_Distribution_UtxoCohorts_AmountRange,
    pub lt_amount: MetricsTree_Distribution_UtxoCohorts_LtAmount,
    pub epoch: MetricsTree_Distribution_UtxoCohorts_Epoch,
    pub year: MetricsTree_Distribution_UtxoCohorts_Year,
    pub type_: MetricsTree_Distribution_UtxoCohorts_Type,
}

impl MetricsTree_Distribution_UtxoCohorts {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            all: MetricsTree_Distribution_UtxoCohorts_All::new(client.clone(), format!("{base_path}_all")),
            sth: MetricsTree_Distribution_UtxoCohorts_Sth::new(client.clone(), format!("{base_path}_sth")),
            lth: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern::new(client.clone(), "lth".to_string()),
            age_range: MetricsTree_Distribution_UtxoCohorts_AgeRange::new(client.clone(), format!("{base_path}_age_range")),
            max_age: MetricsTree_Distribution_UtxoCohorts_MaxAge::new(client.clone(), format!("{base_path}_max_age")),
            min_age: MetricsTree_Distribution_UtxoCohorts_MinAge::new(client.clone(), format!("{base_path}_min_age")),
            ge_amount: MetricsTree_Distribution_UtxoCohorts_GeAmount::new(client.clone(), format!("{base_path}_ge_amount")),
            amount_range: MetricsTree_Distribution_UtxoCohorts_AmountRange::new(client.clone(), format!("{base_path}_amount_range")),
            lt_amount: MetricsTree_Distribution_UtxoCohorts_LtAmount::new(client.clone(), format!("{base_path}_lt_amount")),
            epoch: MetricsTree_Distribution_UtxoCohorts_Epoch::new(client.clone(), format!("{base_path}_epoch")),
            year: MetricsTree_Distribution_UtxoCohorts_Year::new(client.clone(), format!("{base_path}_year")),
            type_: MetricsTree_Distribution_UtxoCohorts_Type::new(client.clone(), format!("{base_path}_type_")),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Distribution_UtxoCohorts_All {
    pub supply: _30dHalvedTotalPattern,
    pub outputs: UtxoPattern,
    pub activity: CoinblocksCoindaysSatblocksSatdaysSentPattern,
    pub realized: AdjustedCapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern,
    pub cost_basis: InvestedMaxMinPercentilesSpotPattern,
    pub unrealized: GreedInvestedInvestorNegNetPainPeakSupplyTotalUnrealizedPattern,
    pub relative: MetricsTree_Distribution_UtxoCohorts_All_Relative,
}

impl MetricsTree_Distribution_UtxoCohorts_All {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            supply: _30dHalvedTotalPattern::new(client.clone(), "".to_string()),
            outputs: UtxoPattern::new(client.clone(), "utxo_count".to_string()),
            activity: CoinblocksCoindaysSatblocksSatdaysSentPattern::new(client.clone(), "".to_string()),
            realized: AdjustedCapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern::new(client.clone(), "".to_string()),
            cost_basis: InvestedMaxMinPercentilesSpotPattern::new(client.clone(), "".to_string()),
            unrealized: GreedInvestedInvestorNegNetPainPeakSupplyTotalUnrealizedPattern::new(client.clone(), "".to_string()),
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
    pub invested_capital_in_profit_pct: MetricPattern1<StoredF32>,
    pub invested_capital_in_loss_pct: MetricPattern1<StoredF32>,
    pub unrealized_profit_rel_to_own_total_unrealized_pnl: MetricPattern1<StoredF32>,
    pub unrealized_loss_rel_to_own_total_unrealized_pnl: MetricPattern1<StoredF32>,
    pub neg_unrealized_loss_rel_to_own_total_unrealized_pnl: MetricPattern1<StoredF32>,
    pub net_unrealized_pnl_rel_to_own_total_unrealized_pnl: MetricPattern1<StoredF32>,
    pub unrealized_peak_regret_rel_to_market_cap: MetricPattern1<StoredF32>,
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
            invested_capital_in_profit_pct: MetricPattern1::new(client.clone(), "invested_capital_in_profit_pct".to_string()),
            invested_capital_in_loss_pct: MetricPattern1::new(client.clone(), "invested_capital_in_loss_pct".to_string()),
            unrealized_profit_rel_to_own_total_unrealized_pnl: MetricPattern1::new(client.clone(), "unrealized_profit_rel_to_own_total_unrealized_pnl".to_string()),
            unrealized_loss_rel_to_own_total_unrealized_pnl: MetricPattern1::new(client.clone(), "unrealized_loss_rel_to_own_total_unrealized_pnl".to_string()),
            neg_unrealized_loss_rel_to_own_total_unrealized_pnl: MetricPattern1::new(client.clone(), "neg_unrealized_loss_rel_to_own_total_unrealized_pnl".to_string()),
            net_unrealized_pnl_rel_to_own_total_unrealized_pnl: MetricPattern1::new(client.clone(), "net_unrealized_pnl_rel_to_own_total_unrealized_pnl".to_string()),
            unrealized_peak_regret_rel_to_market_cap: MetricPattern1::new(client.clone(), "unrealized_peak_regret_rel_to_market_cap".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Distribution_UtxoCohorts_Sth {
    pub supply: _30dHalvedTotalPattern,
    pub outputs: UtxoPattern,
    pub activity: CoinblocksCoindaysSatblocksSatdaysSentPattern,
    pub realized: AdjustedCapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern,
    pub cost_basis: InvestedMaxMinPercentilesSpotPattern,
    pub unrealized: GreedInvestedInvestorNegNetPainPeakSupplyTotalUnrealizedPattern,
    pub relative: InvestedNegNetNuplSupplyUnrealizedPattern2,
}

impl MetricsTree_Distribution_UtxoCohorts_Sth {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            supply: _30dHalvedTotalPattern::new(client.clone(), "sth".to_string()),
            outputs: UtxoPattern::new(client.clone(), "sth_utxo_count".to_string()),
            activity: CoinblocksCoindaysSatblocksSatdaysSentPattern::new(client.clone(), "sth".to_string()),
            realized: AdjustedCapCapitulationInvestorLossLowerMvrvNegNetPeakProfitRealizedSellSentSoprTotalUpperValuePattern::new(client.clone(), "sth".to_string()),
            cost_basis: InvestedMaxMinPercentilesSpotPattern::new(client.clone(), "sth".to_string()),
            unrealized: GreedInvestedInvestorNegNetPainPeakSupplyTotalUnrealizedPattern::new(client.clone(), "sth".to_string()),
            relative: InvestedNegNetNuplSupplyUnrealizedPattern2::new(client.clone(), "sth".to_string()),
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
    pub _1w: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4,
    pub _1m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4,
    pub _2m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4,
    pub _3m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4,
    pub _4m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4,
    pub _5m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4,
    pub _6m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4,
    pub _1y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4,
    pub _2y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4,
    pub _3y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4,
    pub _4y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4,
    pub _5y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4,
    pub _6y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4,
    pub _7y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4,
    pub _8y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4,
    pub _10y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4,
    pub _12y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4,
    pub _15y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4,
}

impl MetricsTree_Distribution_UtxoCohorts_MaxAge {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _1w: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4::new(client.clone(), "utxos_under_1w_old".to_string()),
            _1m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4::new(client.clone(), "utxos_under_1m_old".to_string()),
            _2m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4::new(client.clone(), "utxos_under_2m_old".to_string()),
            _3m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4::new(client.clone(), "utxos_under_3m_old".to_string()),
            _4m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4::new(client.clone(), "utxos_under_4m_old".to_string()),
            _5m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4::new(client.clone(), "utxos_under_5m_old".to_string()),
            _6m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4::new(client.clone(), "utxos_under_6m_old".to_string()),
            _1y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4::new(client.clone(), "utxos_under_1y_old".to_string()),
            _2y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4::new(client.clone(), "utxos_under_2y_old".to_string()),
            _3y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4::new(client.clone(), "utxos_under_3y_old".to_string()),
            _4y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4::new(client.clone(), "utxos_under_4y_old".to_string()),
            _5y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4::new(client.clone(), "utxos_under_5y_old".to_string()),
            _6y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4::new(client.clone(), "utxos_under_6y_old".to_string()),
            _7y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4::new(client.clone(), "utxos_under_7y_old".to_string()),
            _8y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4::new(client.clone(), "utxos_under_8y_old".to_string()),
            _10y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4::new(client.clone(), "utxos_under_10y_old".to_string()),
            _12y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4::new(client.clone(), "utxos_under_12y_old".to_string()),
            _15y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern4::new(client.clone(), "utxos_under_15y_old".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Distribution_UtxoCohorts_MinAge {
    pub _1d: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5,
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
}

impl MetricsTree_Distribution_UtxoCohorts_MinAge {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _1d: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5::new(client.clone(), "utxos_over_1d_old".to_string()),
            _1w: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5::new(client.clone(), "utxos_over_1w_old".to_string()),
            _1m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5::new(client.clone(), "utxos_over_1m_old".to_string()),
            _2m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5::new(client.clone(), "utxos_over_2m_old".to_string()),
            _3m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5::new(client.clone(), "utxos_over_3m_old".to_string()),
            _4m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5::new(client.clone(), "utxos_over_4m_old".to_string()),
            _5m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5::new(client.clone(), "utxos_over_5m_old".to_string()),
            _6m: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5::new(client.clone(), "utxos_over_6m_old".to_string()),
            _1y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5::new(client.clone(), "utxos_over_1y_old".to_string()),
            _2y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5::new(client.clone(), "utxos_over_2y_old".to_string()),
            _3y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5::new(client.clone(), "utxos_over_3y_old".to_string()),
            _4y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5::new(client.clone(), "utxos_over_4y_old".to_string()),
            _5y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5::new(client.clone(), "utxos_over_5y_old".to_string()),
            _6y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5::new(client.clone(), "utxos_over_6y_old".to_string()),
            _7y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5::new(client.clone(), "utxos_over_7y_old".to_string()),
            _8y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5::new(client.clone(), "utxos_over_8y_old".to_string()),
            _10y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5::new(client.clone(), "utxos_over_10y_old".to_string()),
            _12y: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern5::new(client.clone(), "utxos_over_12y_old".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Distribution_UtxoCohorts_GeAmount {
    pub _1sat: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
    pub _10sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
    pub _100sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
    pub _1k_sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
    pub _10k_sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
    pub _100k_sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
    pub _1m_sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
    pub _10m_sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
    pub _1btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
    pub _10btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
    pub _100btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
    pub _1k_btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
    pub _10k_btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
}

impl MetricsTree_Distribution_UtxoCohorts_GeAmount {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _1sat: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "utxos_over_1sat".to_string()),
            _10sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "utxos_over_10sats".to_string()),
            _100sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "utxos_over_100sats".to_string()),
            _1k_sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "utxos_over_1k_sats".to_string()),
            _10k_sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "utxos_over_10k_sats".to_string()),
            _100k_sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "utxos_over_100k_sats".to_string()),
            _1m_sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "utxos_over_1m_sats".to_string()),
            _10m_sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "utxos_over_10m_sats".to_string()),
            _1btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "utxos_over_1btc".to_string()),
            _10btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "utxos_over_10btc".to_string()),
            _100btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "utxos_over_100btc".to_string()),
            _1k_btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "utxos_over_1k_btc".to_string()),
            _10k_btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "utxos_over_10k_btc".to_string()),
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
pub struct MetricsTree_Distribution_UtxoCohorts_LtAmount {
    pub _10sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
    pub _100sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
    pub _1k_sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
    pub _10k_sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
    pub _100k_sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
    pub _1m_sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
    pub _10m_sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
    pub _1btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
    pub _10btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
    pub _100btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
    pub _1k_btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
    pub _10k_btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
    pub _100k_btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
}

impl MetricsTree_Distribution_UtxoCohorts_LtAmount {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            _10sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "utxos_under_10sats".to_string()),
            _100sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "utxos_under_100sats".to_string()),
            _1k_sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "utxos_under_1k_sats".to_string()),
            _10k_sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "utxos_under_10k_sats".to_string()),
            _100k_sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "utxos_under_100k_sats".to_string()),
            _1m_sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "utxos_under_1m_sats".to_string()),
            _10m_sats: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "utxos_under_10m_sats".to_string()),
            _1btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "utxos_under_1btc".to_string()),
            _10btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "utxos_under_10btc".to_string()),
            _100btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "utxos_under_100btc".to_string()),
            _1k_btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "utxos_under_1k_btc".to_string()),
            _10k_btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "utxos_under_10k_btc".to_string()),
            _100k_btc: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "utxos_under_100k_btc".to_string()),
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
pub struct MetricsTree_Distribution_UtxoCohorts_Type {
    pub p2pk65: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
    pub p2pk33: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
    pub p2pkh: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
    pub p2ms: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
    pub p2sh: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
    pub p2wpkh: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
    pub p2wsh: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
    pub p2tr: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
    pub p2a: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
    pub unknown: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
    pub empty: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3,
}

impl MetricsTree_Distribution_UtxoCohorts_Type {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            p2pk65: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "p2pk65".to_string()),
            p2pk33: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "p2pk33".to_string()),
            p2pkh: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "p2pkh".to_string()),
            p2ms: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "p2ms".to_string()),
            p2sh: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "p2sh".to_string()),
            p2wpkh: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "p2wpkh".to_string()),
            p2wsh: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "p2wsh".to_string()),
            p2tr: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "p2tr".to_string()),
            p2a: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "p2a".to_string()),
            unknown: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "unknown_outputs".to_string()),
            empty: ActivityCostOutputsRealizedRelativeSupplyUnrealizedPattern3::new(client.clone(), "empty_outputs".to_string()),
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
    pub all: BaseRestPattern,
    pub p2pk65: BaseRestPattern,
    pub p2pk33: BaseRestPattern,
    pub p2pkh: BaseRestPattern,
    pub p2sh: BaseRestPattern,
    pub p2wpkh: BaseRestPattern,
    pub p2wsh: BaseRestPattern,
    pub p2tr: BaseRestPattern,
    pub p2a: BaseRestPattern,
}

impl MetricsTree_Distribution_NewAddrCount {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            all: BaseRestPattern::new(client.clone(), "new_addr_count".to_string()),
            p2pk65: BaseRestPattern::new(client.clone(), "p2pk65_new_addr_count".to_string()),
            p2pk33: BaseRestPattern::new(client.clone(), "p2pk33_new_addr_count".to_string()),
            p2pkh: BaseRestPattern::new(client.clone(), "p2pkh_new_addr_count".to_string()),
            p2sh: BaseRestPattern::new(client.clone(), "p2sh_new_addr_count".to_string()),
            p2wpkh: BaseRestPattern::new(client.clone(), "p2wpkh_new_addr_count".to_string()),
            p2wsh: BaseRestPattern::new(client.clone(), "p2wsh_new_addr_count".to_string()),
            p2tr: BaseRestPattern::new(client.clone(), "p2tr_new_addr_count".to_string()),
            p2a: BaseRestPattern::new(client.clone(), "p2a_new_addr_count".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Distribution_GrowthRate {
    pub all: AverageHeightMaxMedianMinPct10Pct25Pct75Pct90Pattern<StoredF32>,
    pub p2pk65: AverageHeightMaxMedianMinPct10Pct25Pct75Pct90Pattern<StoredF32>,
    pub p2pk33: AverageHeightMaxMedianMinPct10Pct25Pct75Pct90Pattern<StoredF32>,
    pub p2pkh: AverageHeightMaxMedianMinPct10Pct25Pct75Pct90Pattern<StoredF32>,
    pub p2sh: AverageHeightMaxMedianMinPct10Pct25Pct75Pct90Pattern<StoredF32>,
    pub p2wpkh: AverageHeightMaxMedianMinPct10Pct25Pct75Pct90Pattern<StoredF32>,
    pub p2wsh: AverageHeightMaxMedianMinPct10Pct25Pct75Pct90Pattern<StoredF32>,
    pub p2tr: AverageHeightMaxMedianMinPct10Pct25Pct75Pct90Pattern<StoredF32>,
    pub p2a: AverageHeightMaxMedianMinPct10Pct25Pct75Pct90Pattern<StoredF32>,
}

impl MetricsTree_Distribution_GrowthRate {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            all: AverageHeightMaxMedianMinPct10Pct25Pct75Pct90Pattern::new(client.clone(), "growth_rate".to_string()),
            p2pk65: AverageHeightMaxMedianMinPct10Pct25Pct75Pct90Pattern::new(client.clone(), "p2pk65_growth_rate".to_string()),
            p2pk33: AverageHeightMaxMedianMinPct10Pct25Pct75Pct90Pattern::new(client.clone(), "p2pk33_growth_rate".to_string()),
            p2pkh: AverageHeightMaxMedianMinPct10Pct25Pct75Pct90Pattern::new(client.clone(), "p2pkh_growth_rate".to_string()),
            p2sh: AverageHeightMaxMedianMinPct10Pct25Pct75Pct90Pattern::new(client.clone(), "p2sh_growth_rate".to_string()),
            p2wpkh: AverageHeightMaxMedianMinPct10Pct25Pct75Pct90Pattern::new(client.clone(), "p2wpkh_growth_rate".to_string()),
            p2wsh: AverageHeightMaxMedianMinPct10Pct25Pct75Pct90Pattern::new(client.clone(), "p2wsh_growth_rate".to_string()),
            p2tr: AverageHeightMaxMedianMinPct10Pct25Pct75Pct90Pattern::new(client.clone(), "p2tr_growth_rate".to_string()),
            p2a: AverageHeightMaxMedianMinPct10Pct25Pct75Pct90Pattern::new(client.clone(), "p2a_growth_rate".to_string()),
        }
    }
}

/// Metrics tree node.
pub struct MetricsTree_Supply {
    pub circulating: BtcSatsUsdPattern,
    pub burned: MetricsTree_Supply_Burned,
    pub inflation: MetricPattern1<StoredF32>,
    pub velocity: MetricsTree_Supply_Velocity,
    pub market_cap: MetricPattern1<Dollars>,
    pub market_cap_growth_rate: MetricPattern1<StoredF32>,
    pub realized_cap_growth_rate: MetricPattern1<StoredF32>,
    pub cap_growth_rate_diff: MetricPattern1<StoredF32>,
}

impl MetricsTree_Supply {
    pub fn new(client: Arc<BrkClientBase>, base_path: String) -> Self {
        Self {
            circulating: BtcSatsUsdPattern::new(client.clone(), "circulating_supply".to_string()),
            burned: MetricsTree_Supply_Burned::new(client.clone(), format!("{base_path}_burned")),
            inflation: MetricPattern1::new(client.clone(), "inflation_rate".to_string()),
            velocity: MetricsTree_Supply_Velocity::new(client.clone(), format!("{base_path}_velocity")),
            market_cap: MetricPattern1::new(client.clone(), "market_cap".to_string()),
            market_cap_growth_rate: MetricPattern1::new(client.clone(), "market_cap_growth_rate".to_string()),
            realized_cap_growth_rate: MetricPattern1::new(client.clone(), "realized_cap_growth_rate".to_string()),
            cap_growth_rate_diff: MetricPattern1::new(client.clone(), "cap_growth_rate_diff".to_string()),
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
            btc: MetricPattern1::new(client.clone(), "btc_velocity".to_string()),
            usd: MetricPattern1::new(client.clone(), "usd_velocity".to_string()),
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
