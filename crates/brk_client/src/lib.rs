// Auto-generated BRK Rust client
// Do not edit manually

#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(clippy::useless_format)]

use std::sync::Arc;
use serde::de::DeserializeOwned;
pub use brk_grouper::*;
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
        let url = format!("{}{}", self.base_url, path);
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


/// A metric node that can fetch data for different indexes.
pub struct MetricNode<T> {
    client: Arc<BrkClientBase>,
    path: String,
    _marker: std::marker::PhantomData<T>,
}

impl<T: DeserializeOwned> MetricNode<T> {
    pub fn new(client: Arc<BrkClientBase>, path: String) -> Self {
        Self {
            client,
            path,
            _marker: std::marker::PhantomData,
        }
    }

    /// Fetch all data points for this metric.
    pub fn get(&self) -> Result<Vec<T>> {
        self.client.get(&self.path)
    }

    /// Fetch data points within a range.
    pub fn get_range(&self, from: Option<&str>, to: Option<&str>) -> Result<Vec<T>> {
        let mut params = Vec::new();
        if let Some(f) = from { params.push(format!("from={}", f)); }
        if let Some(t) = to { params.push(format!("to={}", t)); }
        let path = if params.is_empty() {
            self.path.clone()
        } else {
            format!("{}?{}", self.path, params.join("&"))
        };
        self.client.get(&path)
    }
}


// Index accessor structs

/// Index accessor for metrics with 9 indexes.
pub struct Indexes3<T> {
    pub by_dateindex: MetricNode<T>,
    pub by_decadeindex: MetricNode<T>,
    pub by_difficultyepoch: MetricNode<T>,
    pub by_height: MetricNode<T>,
    pub by_monthindex: MetricNode<T>,
    pub by_quarterindex: MetricNode<T>,
    pub by_semesterindex: MetricNode<T>,
    pub by_weekindex: MetricNode<T>,
    pub by_yearindex: MetricNode<T>,
}

impl<T: DeserializeOwned> Indexes3<T> {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            by_dateindex: MetricNode::new(client.clone(), format!("{base_path}/dateindex")),
            by_decadeindex: MetricNode::new(client.clone(), format!("{base_path}/decadeindex")),
            by_difficultyepoch: MetricNode::new(client.clone(), format!("{base_path}/difficultyepoch")),
            by_height: MetricNode::new(client.clone(), format!("{base_path}/height")),
            by_monthindex: MetricNode::new(client.clone(), format!("{base_path}/monthindex")),
            by_quarterindex: MetricNode::new(client.clone(), format!("{base_path}/quarterindex")),
            by_semesterindex: MetricNode::new(client.clone(), format!("{base_path}/semesterindex")),
            by_weekindex: MetricNode::new(client.clone(), format!("{base_path}/weekindex")),
            by_yearindex: MetricNode::new(client.clone(), format!("{base_path}/yearindex")),
        }
    }
}

/// Index accessor for metrics with 8 indexes.
pub struct Indexes4<T> {
    pub by_dateindex: MetricNode<T>,
    pub by_decadeindex: MetricNode<T>,
    pub by_difficultyepoch: MetricNode<T>,
    pub by_monthindex: MetricNode<T>,
    pub by_quarterindex: MetricNode<T>,
    pub by_semesterindex: MetricNode<T>,
    pub by_weekindex: MetricNode<T>,
    pub by_yearindex: MetricNode<T>,
}

impl<T: DeserializeOwned> Indexes4<T> {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            by_dateindex: MetricNode::new(client.clone(), format!("{base_path}/dateindex")),
            by_decadeindex: MetricNode::new(client.clone(), format!("{base_path}/decadeindex")),
            by_difficultyepoch: MetricNode::new(client.clone(), format!("{base_path}/difficultyepoch")),
            by_monthindex: MetricNode::new(client.clone(), format!("{base_path}/monthindex")),
            by_quarterindex: MetricNode::new(client.clone(), format!("{base_path}/quarterindex")),
            by_semesterindex: MetricNode::new(client.clone(), format!("{base_path}/semesterindex")),
            by_weekindex: MetricNode::new(client.clone(), format!("{base_path}/weekindex")),
            by_yearindex: MetricNode::new(client.clone(), format!("{base_path}/yearindex")),
        }
    }
}

/// Index accessor for metrics with 8 indexes.
pub struct Indexes26<T> {
    pub by_dateindex: MetricNode<T>,
    pub by_decadeindex: MetricNode<T>,
    pub by_height: MetricNode<T>,
    pub by_monthindex: MetricNode<T>,
    pub by_quarterindex: MetricNode<T>,
    pub by_semesterindex: MetricNode<T>,
    pub by_weekindex: MetricNode<T>,
    pub by_yearindex: MetricNode<T>,
}

impl<T: DeserializeOwned> Indexes26<T> {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            by_dateindex: MetricNode::new(client.clone(), format!("{base_path}/dateindex")),
            by_decadeindex: MetricNode::new(client.clone(), format!("{base_path}/decadeindex")),
            by_height: MetricNode::new(client.clone(), format!("{base_path}/height")),
            by_monthindex: MetricNode::new(client.clone(), format!("{base_path}/monthindex")),
            by_quarterindex: MetricNode::new(client.clone(), format!("{base_path}/quarterindex")),
            by_semesterindex: MetricNode::new(client.clone(), format!("{base_path}/semesterindex")),
            by_weekindex: MetricNode::new(client.clone(), format!("{base_path}/weekindex")),
            by_yearindex: MetricNode::new(client.clone(), format!("{base_path}/yearindex")),
        }
    }
}

/// Index accessor for metrics with 7 indexes.
pub struct Indexes<T> {
    pub by_dateindex: MetricNode<T>,
    pub by_decadeindex: MetricNode<T>,
    pub by_monthindex: MetricNode<T>,
    pub by_quarterindex: MetricNode<T>,
    pub by_semesterindex: MetricNode<T>,
    pub by_weekindex: MetricNode<T>,
    pub by_yearindex: MetricNode<T>,
}

impl<T: DeserializeOwned> Indexes<T> {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            by_dateindex: MetricNode::new(client.clone(), format!("{base_path}/dateindex")),
            by_decadeindex: MetricNode::new(client.clone(), format!("{base_path}/decadeindex")),
            by_monthindex: MetricNode::new(client.clone(), format!("{base_path}/monthindex")),
            by_quarterindex: MetricNode::new(client.clone(), format!("{base_path}/quarterindex")),
            by_semesterindex: MetricNode::new(client.clone(), format!("{base_path}/semesterindex")),
            by_weekindex: MetricNode::new(client.clone(), format!("{base_path}/weekindex")),
            by_yearindex: MetricNode::new(client.clone(), format!("{base_path}/yearindex")),
        }
    }
}

/// Index accessor for metrics with 7 indexes.
pub struct Indexes27<T> {
    pub by_decadeindex: MetricNode<T>,
    pub by_height: MetricNode<T>,
    pub by_monthindex: MetricNode<T>,
    pub by_quarterindex: MetricNode<T>,
    pub by_semesterindex: MetricNode<T>,
    pub by_weekindex: MetricNode<T>,
    pub by_yearindex: MetricNode<T>,
}

impl<T: DeserializeOwned> Indexes27<T> {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            by_decadeindex: MetricNode::new(client.clone(), format!("{base_path}/decadeindex")),
            by_height: MetricNode::new(client.clone(), format!("{base_path}/height")),
            by_monthindex: MetricNode::new(client.clone(), format!("{base_path}/monthindex")),
            by_quarterindex: MetricNode::new(client.clone(), format!("{base_path}/quarterindex")),
            by_semesterindex: MetricNode::new(client.clone(), format!("{base_path}/semesterindex")),
            by_weekindex: MetricNode::new(client.clone(), format!("{base_path}/weekindex")),
            by_yearindex: MetricNode::new(client.clone(), format!("{base_path}/yearindex")),
        }
    }
}

/// Index accessor for metrics with 6 indexes.
pub struct Indexes28<T> {
    pub by_decadeindex: MetricNode<T>,
    pub by_monthindex: MetricNode<T>,
    pub by_quarterindex: MetricNode<T>,
    pub by_semesterindex: MetricNode<T>,
    pub by_weekindex: MetricNode<T>,
    pub by_yearindex: MetricNode<T>,
}

impl<T: DeserializeOwned> Indexes28<T> {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            by_decadeindex: MetricNode::new(client.clone(), format!("{base_path}/decadeindex")),
            by_monthindex: MetricNode::new(client.clone(), format!("{base_path}/monthindex")),
            by_quarterindex: MetricNode::new(client.clone(), format!("{base_path}/quarterindex")),
            by_semesterindex: MetricNode::new(client.clone(), format!("{base_path}/semesterindex")),
            by_weekindex: MetricNode::new(client.clone(), format!("{base_path}/weekindex")),
            by_yearindex: MetricNode::new(client.clone(), format!("{base_path}/yearindex")),
        }
    }
}

/// Index accessor for metrics with 3 indexes.
pub struct Indexes15<T> {
    pub by_quarterindex: MetricNode<T>,
    pub by_semesterindex: MetricNode<T>,
    pub by_yearindex: MetricNode<T>,
}

impl<T: DeserializeOwned> Indexes15<T> {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            by_quarterindex: MetricNode::new(client.clone(), format!("{base_path}/quarterindex")),
            by_semesterindex: MetricNode::new(client.clone(), format!("{base_path}/semesterindex")),
            by_yearindex: MetricNode::new(client.clone(), format!("{base_path}/yearindex")),
        }
    }
}

/// Index accessor for metrics with 2 indexes.
pub struct Indexes13<T> {
    pub by_dateindex: MetricNode<T>,
    pub by_height: MetricNode<T>,
}

impl<T: DeserializeOwned> Indexes13<T> {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            by_dateindex: MetricNode::new(client.clone(), format!("{base_path}/dateindex")),
            by_height: MetricNode::new(client.clone(), format!("{base_path}/height")),
        }
    }
}

/// Index accessor for metrics with 2 indexes.
pub struct Indexes14<T> {
    pub by_monthindex: MetricNode<T>,
    pub by_weekindex: MetricNode<T>,
}

impl<T: DeserializeOwned> Indexes14<T> {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            by_monthindex: MetricNode::new(client.clone(), format!("{base_path}/monthindex")),
            by_weekindex: MetricNode::new(client.clone(), format!("{base_path}/weekindex")),
        }
    }
}

/// Index accessor for metrics with 1 indexes.
pub struct Indexes2<T> {
    pub by_height: MetricNode<T>,
}

impl<T: DeserializeOwned> Indexes2<T> {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            by_height: MetricNode::new(client.clone(), format!("{base_path}/height")),
        }
    }
}

/// Index accessor for metrics with 1 indexes.
pub struct Indexes5<T> {
    pub by_dateindex: MetricNode<T>,
}

impl<T: DeserializeOwned> Indexes5<T> {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            by_dateindex: MetricNode::new(client.clone(), format!("{base_path}/dateindex")),
        }
    }
}

/// Index accessor for metrics with 1 indexes.
pub struct Indexes6<T> {
    pub by_txindex: MetricNode<T>,
}

impl<T: DeserializeOwned> Indexes6<T> {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            by_txindex: MetricNode::new(client.clone(), format!("{base_path}/txindex")),
        }
    }
}

/// Index accessor for metrics with 1 indexes.
pub struct Indexes7<T> {
    pub by_decadeindex: MetricNode<T>,
}

impl<T: DeserializeOwned> Indexes7<T> {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            by_decadeindex: MetricNode::new(client.clone(), format!("{base_path}/decadeindex")),
        }
    }
}

/// Index accessor for metrics with 1 indexes.
pub struct Indexes8<T> {
    pub by_monthindex: MetricNode<T>,
}

impl<T: DeserializeOwned> Indexes8<T> {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            by_monthindex: MetricNode::new(client.clone(), format!("{base_path}/monthindex")),
        }
    }
}

/// Index accessor for metrics with 1 indexes.
pub struct Indexes9<T> {
    pub by_quarterindex: MetricNode<T>,
}

impl<T: DeserializeOwned> Indexes9<T> {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            by_quarterindex: MetricNode::new(client.clone(), format!("{base_path}/quarterindex")),
        }
    }
}

/// Index accessor for metrics with 1 indexes.
pub struct Indexes10<T> {
    pub by_semesterindex: MetricNode<T>,
}

impl<T: DeserializeOwned> Indexes10<T> {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            by_semesterindex: MetricNode::new(client.clone(), format!("{base_path}/semesterindex")),
        }
    }
}

/// Index accessor for metrics with 1 indexes.
pub struct Indexes11<T> {
    pub by_weekindex: MetricNode<T>,
}

impl<T: DeserializeOwned> Indexes11<T> {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            by_weekindex: MetricNode::new(client.clone(), format!("{base_path}/weekindex")),
        }
    }
}

/// Index accessor for metrics with 1 indexes.
pub struct Indexes12<T> {
    pub by_yearindex: MetricNode<T>,
}

impl<T: DeserializeOwned> Indexes12<T> {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            by_yearindex: MetricNode::new(client.clone(), format!("{base_path}/yearindex")),
        }
    }
}

/// Index accessor for metrics with 1 indexes.
pub struct Indexes16<T> {
    pub by_p2aaddressindex: MetricNode<T>,
}

impl<T: DeserializeOwned> Indexes16<T> {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            by_p2aaddressindex: MetricNode::new(client.clone(), format!("{base_path}/p2aaddressindex")),
        }
    }
}

/// Index accessor for metrics with 1 indexes.
pub struct Indexes17<T> {
    pub by_p2pk33addressindex: MetricNode<T>,
}

impl<T: DeserializeOwned> Indexes17<T> {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            by_p2pk33addressindex: MetricNode::new(client.clone(), format!("{base_path}/p2pk33addressindex")),
        }
    }
}

/// Index accessor for metrics with 1 indexes.
pub struct Indexes18<T> {
    pub by_p2pk65addressindex: MetricNode<T>,
}

impl<T: DeserializeOwned> Indexes18<T> {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            by_p2pk65addressindex: MetricNode::new(client.clone(), format!("{base_path}/p2pk65addressindex")),
        }
    }
}

/// Index accessor for metrics with 1 indexes.
pub struct Indexes19<T> {
    pub by_p2pkhaddressindex: MetricNode<T>,
}

impl<T: DeserializeOwned> Indexes19<T> {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            by_p2pkhaddressindex: MetricNode::new(client.clone(), format!("{base_path}/p2pkhaddressindex")),
        }
    }
}

/// Index accessor for metrics with 1 indexes.
pub struct Indexes20<T> {
    pub by_p2shaddressindex: MetricNode<T>,
}

impl<T: DeserializeOwned> Indexes20<T> {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            by_p2shaddressindex: MetricNode::new(client.clone(), format!("{base_path}/p2shaddressindex")),
        }
    }
}

/// Index accessor for metrics with 1 indexes.
pub struct Indexes21<T> {
    pub by_p2traddressindex: MetricNode<T>,
}

impl<T: DeserializeOwned> Indexes21<T> {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            by_p2traddressindex: MetricNode::new(client.clone(), format!("{base_path}/p2traddressindex")),
        }
    }
}

/// Index accessor for metrics with 1 indexes.
pub struct Indexes22<T> {
    pub by_p2wpkhaddressindex: MetricNode<T>,
}

impl<T: DeserializeOwned> Indexes22<T> {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            by_p2wpkhaddressindex: MetricNode::new(client.clone(), format!("{base_path}/p2wpkhaddressindex")),
        }
    }
}

/// Index accessor for metrics with 1 indexes.
pub struct Indexes23<T> {
    pub by_p2wshaddressindex: MetricNode<T>,
}

impl<T: DeserializeOwned> Indexes23<T> {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            by_p2wshaddressindex: MetricNode::new(client.clone(), format!("{base_path}/p2wshaddressindex")),
        }
    }
}

/// Index accessor for metrics with 1 indexes.
pub struct Indexes24<T> {
    pub by_txinindex: MetricNode<T>,
}

impl<T: DeserializeOwned> Indexes24<T> {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            by_txinindex: MetricNode::new(client.clone(), format!("{base_path}/txinindex")),
        }
    }
}

/// Index accessor for metrics with 1 indexes.
pub struct Indexes25<T> {
    pub by_txoutindex: MetricNode<T>,
}

impl<T: DeserializeOwned> Indexes25<T> {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            by_txoutindex: MetricNode::new(client.clone(), format!("{base_path}/txoutindex")),
        }
    }
}

/// Index accessor for metrics with 1 indexes.
pub struct Indexes29<T> {
    pub by_emptyaddressindex: MetricNode<T>,
}

impl<T: DeserializeOwned> Indexes29<T> {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            by_emptyaddressindex: MetricNode::new(client.clone(), format!("{base_path}/emptyaddressindex")),
        }
    }
}

/// Index accessor for metrics with 1 indexes.
pub struct Indexes30<T> {
    pub by_loadedaddressindex: MetricNode<T>,
}

impl<T: DeserializeOwned> Indexes30<T> {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            by_loadedaddressindex: MetricNode::new(client.clone(), format!("{base_path}/loadedaddressindex")),
        }
    }
}

// Reusable pattern structs

/// Pattern struct for repeated tree structure.
pub struct RealizedPattern3 {
    pub adjusted_sopr: Indexes5<StoredF64>,
    pub adjusted_sopr_30d_ema: Indexes5<StoredF64>,
    pub adjusted_sopr_7d_ema: Indexes5<StoredF64>,
    pub adjusted_value_created: Indexes3<Dollars>,
    pub adjusted_value_destroyed: Indexes3<Dollars>,
    pub neg_realized_loss: BlockCountPattern<Dollars>,
    pub net_realized_pnl: BlockCountPattern<Dollars>,
    pub net_realized_pnl_cumulative_30d_delta: Indexes<Dollars>,
    pub net_realized_pnl_cumulative_30d_delta_rel_to_market_cap: Indexes<StoredF32>,
    pub net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap: Indexes<StoredF32>,
    pub net_realized_pnl_rel_to_realized_cap: Indexes2<StoredF32>,
    pub realized_cap: Indexes3<Dollars>,
    pub realized_cap_30d_delta: Indexes<Dollars>,
    pub realized_cap_rel_to_own_market_cap: Indexes3<StoredF32>,
    pub realized_loss: BlockCountPattern<Dollars>,
    pub realized_loss_rel_to_realized_cap: Indexes2<StoredF32>,
    pub realized_price: Indexes3<Dollars>,
    pub realized_price_extra: ActivePriceRatioPattern,
    pub realized_profit: BlockCountPattern<Dollars>,
    pub realized_profit_rel_to_realized_cap: Indexes2<StoredF32>,
    pub realized_profit_to_loss_ratio: Indexes5<StoredF64>,
    pub realized_value: Indexes3<Dollars>,
    pub sell_side_risk_ratio: Indexes5<StoredF32>,
    pub sell_side_risk_ratio_30d_ema: Indexes5<StoredF32>,
    pub sell_side_risk_ratio_7d_ema: Indexes5<StoredF32>,
    pub sopr: Indexes5<StoredF64>,
    pub sopr_30d_ema: Indexes5<StoredF64>,
    pub sopr_7d_ema: Indexes5<StoredF64>,
    pub total_realized_pnl: BitcoinPattern2<Dollars>,
    pub value_created: Indexes3<Dollars>,
    pub value_destroyed: Indexes3<Dollars>,
}

impl RealizedPattern3 {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            adjusted_sopr: Indexes5::new(client.clone(), &format!("{base_path}/adjusted_sopr")),
            adjusted_sopr_30d_ema: Indexes5::new(client.clone(), &format!("{base_path}/adjusted_sopr_30d_ema")),
            adjusted_sopr_7d_ema: Indexes5::new(client.clone(), &format!("{base_path}/adjusted_sopr_7d_ema")),
            adjusted_value_created: Indexes3::new(client.clone(), &format!("{base_path}/adjusted_value_created")),
            adjusted_value_destroyed: Indexes3::new(client.clone(), &format!("{base_path}/adjusted_value_destroyed")),
            neg_realized_loss: BlockCountPattern::new(client.clone(), &format!("{base_path}/neg_realized_loss")),
            net_realized_pnl: BlockCountPattern::new(client.clone(), &format!("{base_path}/net_realized_pnl")),
            net_realized_pnl_cumulative_30d_delta: Indexes::new(client.clone(), &format!("{base_path}/net_realized_pnl_cumulative_30d_delta")),
            net_realized_pnl_cumulative_30d_delta_rel_to_market_cap: Indexes::new(client.clone(), &format!("{base_path}/net_realized_pnl_cumulative_30d_delta_rel_to_market_cap")),
            net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap: Indexes::new(client.clone(), &format!("{base_path}/net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap")),
            net_realized_pnl_rel_to_realized_cap: Indexes2::new(client.clone(), &format!("{base_path}/net_realized_pnl_rel_to_realized_cap")),
            realized_cap: Indexes3::new(client.clone(), &format!("{base_path}/realized_cap")),
            realized_cap_30d_delta: Indexes::new(client.clone(), &format!("{base_path}/realized_cap_30d_delta")),
            realized_cap_rel_to_own_market_cap: Indexes3::new(client.clone(), &format!("{base_path}/realized_cap_rel_to_own_market_cap")),
            realized_loss: BlockCountPattern::new(client.clone(), &format!("{base_path}/realized_loss")),
            realized_loss_rel_to_realized_cap: Indexes2::new(client.clone(), &format!("{base_path}/realized_loss_rel_to_realized_cap")),
            realized_price: Indexes3::new(client.clone(), &format!("{base_path}/realized_price")),
            realized_price_extra: ActivePriceRatioPattern::new(client.clone(), &format!("{base_path}/realized_price_extra")),
            realized_profit: BlockCountPattern::new(client.clone(), &format!("{base_path}/realized_profit")),
            realized_profit_rel_to_realized_cap: Indexes2::new(client.clone(), &format!("{base_path}/realized_profit_rel_to_realized_cap")),
            realized_profit_to_loss_ratio: Indexes5::new(client.clone(), &format!("{base_path}/realized_profit_to_loss_ratio")),
            realized_value: Indexes3::new(client.clone(), &format!("{base_path}/realized_value")),
            sell_side_risk_ratio: Indexes5::new(client.clone(), &format!("{base_path}/sell_side_risk_ratio")),
            sell_side_risk_ratio_30d_ema: Indexes5::new(client.clone(), &format!("{base_path}/sell_side_risk_ratio_30d_ema")),
            sell_side_risk_ratio_7d_ema: Indexes5::new(client.clone(), &format!("{base_path}/sell_side_risk_ratio_7d_ema")),
            sopr: Indexes5::new(client.clone(), &format!("{base_path}/sopr")),
            sopr_30d_ema: Indexes5::new(client.clone(), &format!("{base_path}/sopr_30d_ema")),
            sopr_7d_ema: Indexes5::new(client.clone(), &format!("{base_path}/sopr_7d_ema")),
            total_realized_pnl: BitcoinPattern2::new(client.clone(), &format!("{base_path}/total_realized_pnl")),
            value_created: Indexes3::new(client.clone(), &format!("{base_path}/value_created")),
            value_destroyed: Indexes3::new(client.clone(), &format!("{base_path}/value_destroyed")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct Ratio1ySdPattern {
    pub _0sd_usd: Indexes<Dollars>,
    pub m0_5sd: Indexes<StoredF32>,
    pub m0_5sd_usd: Indexes<Dollars>,
    pub m1_5sd: Indexes<StoredF32>,
    pub m1_5sd_usd: Indexes<Dollars>,
    pub m1sd: Indexes<StoredF32>,
    pub m1sd_usd: Indexes<Dollars>,
    pub m2_5sd: Indexes<StoredF32>,
    pub m2_5sd_usd: Indexes<Dollars>,
    pub m2sd: Indexes<StoredF32>,
    pub m2sd_usd: Indexes<Dollars>,
    pub m3sd: Indexes<StoredF32>,
    pub m3sd_usd: Indexes<Dollars>,
    pub p0_5sd: Indexes<StoredF32>,
    pub p0_5sd_usd: Indexes<Dollars>,
    pub p1_5sd: Indexes<StoredF32>,
    pub p1_5sd_usd: Indexes<Dollars>,
    pub p1sd: Indexes<StoredF32>,
    pub p1sd_usd: Indexes<Dollars>,
    pub p2_5sd: Indexes<StoredF32>,
    pub p2_5sd_usd: Indexes<Dollars>,
    pub p2sd: Indexes<StoredF32>,
    pub p2sd_usd: Indexes<Dollars>,
    pub p3sd: Indexes<StoredF32>,
    pub p3sd_usd: Indexes<Dollars>,
    pub sd: Indexes<StoredF32>,
    pub sma: Indexes<StoredF32>,
    pub zscore: Indexes<StoredF32>,
}

impl Ratio1ySdPattern {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            _0sd_usd: Indexes::new(client.clone(), &format!("{base_path}/_0sd_usd")),
            m0_5sd: Indexes::new(client.clone(), &format!("{base_path}/m0_5sd")),
            m0_5sd_usd: Indexes::new(client.clone(), &format!("{base_path}/m0_5sd_usd")),
            m1_5sd: Indexes::new(client.clone(), &format!("{base_path}/m1_5sd")),
            m1_5sd_usd: Indexes::new(client.clone(), &format!("{base_path}/m1_5sd_usd")),
            m1sd: Indexes::new(client.clone(), &format!("{base_path}/m1sd")),
            m1sd_usd: Indexes::new(client.clone(), &format!("{base_path}/m1sd_usd")),
            m2_5sd: Indexes::new(client.clone(), &format!("{base_path}/m2_5sd")),
            m2_5sd_usd: Indexes::new(client.clone(), &format!("{base_path}/m2_5sd_usd")),
            m2sd: Indexes::new(client.clone(), &format!("{base_path}/m2sd")),
            m2sd_usd: Indexes::new(client.clone(), &format!("{base_path}/m2sd_usd")),
            m3sd: Indexes::new(client.clone(), &format!("{base_path}/m3sd")),
            m3sd_usd: Indexes::new(client.clone(), &format!("{base_path}/m3sd_usd")),
            p0_5sd: Indexes::new(client.clone(), &format!("{base_path}/p0_5sd")),
            p0_5sd_usd: Indexes::new(client.clone(), &format!("{base_path}/p0_5sd_usd")),
            p1_5sd: Indexes::new(client.clone(), &format!("{base_path}/p1_5sd")),
            p1_5sd_usd: Indexes::new(client.clone(), &format!("{base_path}/p1_5sd_usd")),
            p1sd: Indexes::new(client.clone(), &format!("{base_path}/p1sd")),
            p1sd_usd: Indexes::new(client.clone(), &format!("{base_path}/p1sd_usd")),
            p2_5sd: Indexes::new(client.clone(), &format!("{base_path}/p2_5sd")),
            p2_5sd_usd: Indexes::new(client.clone(), &format!("{base_path}/p2_5sd_usd")),
            p2sd: Indexes::new(client.clone(), &format!("{base_path}/p2sd")),
            p2sd_usd: Indexes::new(client.clone(), &format!("{base_path}/p2sd_usd")),
            p3sd: Indexes::new(client.clone(), &format!("{base_path}/p3sd")),
            p3sd_usd: Indexes::new(client.clone(), &format!("{base_path}/p3sd_usd")),
            sd: Indexes::new(client.clone(), &format!("{base_path}/sd")),
            sma: Indexes::new(client.clone(), &format!("{base_path}/sma")),
            zscore: Indexes::new(client.clone(), &format!("{base_path}/zscore")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct RealizedPattern2 {
    pub neg_realized_loss: BlockCountPattern<Dollars>,
    pub net_realized_pnl: BlockCountPattern<Dollars>,
    pub net_realized_pnl_cumulative_30d_delta: Indexes<Dollars>,
    pub net_realized_pnl_cumulative_30d_delta_rel_to_market_cap: Indexes<StoredF32>,
    pub net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap: Indexes<StoredF32>,
    pub net_realized_pnl_rel_to_realized_cap: Indexes2<StoredF32>,
    pub realized_cap: Indexes3<Dollars>,
    pub realized_cap_30d_delta: Indexes<Dollars>,
    pub realized_cap_rel_to_own_market_cap: Indexes3<StoredF32>,
    pub realized_loss: BlockCountPattern<Dollars>,
    pub realized_loss_rel_to_realized_cap: Indexes2<StoredF32>,
    pub realized_price: Indexes3<Dollars>,
    pub realized_price_extra: ActivePriceRatioPattern,
    pub realized_profit: BlockCountPattern<Dollars>,
    pub realized_profit_rel_to_realized_cap: Indexes2<StoredF32>,
    pub realized_profit_to_loss_ratio: Indexes5<StoredF64>,
    pub realized_value: Indexes3<Dollars>,
    pub sell_side_risk_ratio: Indexes5<StoredF32>,
    pub sell_side_risk_ratio_30d_ema: Indexes5<StoredF32>,
    pub sell_side_risk_ratio_7d_ema: Indexes5<StoredF32>,
    pub sopr: Indexes5<StoredF64>,
    pub sopr_30d_ema: Indexes5<StoredF64>,
    pub sopr_7d_ema: Indexes5<StoredF64>,
    pub total_realized_pnl: BitcoinPattern2<Dollars>,
    pub value_created: Indexes3<Dollars>,
    pub value_destroyed: Indexes3<Dollars>,
}

impl RealizedPattern2 {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            neg_realized_loss: BlockCountPattern::new(client.clone(), &format!("{base_path}/neg_realized_loss")),
            net_realized_pnl: BlockCountPattern::new(client.clone(), &format!("{base_path}/net_realized_pnl")),
            net_realized_pnl_cumulative_30d_delta: Indexes::new(client.clone(), &format!("{base_path}/net_realized_pnl_cumulative_30d_delta")),
            net_realized_pnl_cumulative_30d_delta_rel_to_market_cap: Indexes::new(client.clone(), &format!("{base_path}/net_realized_pnl_cumulative_30d_delta_rel_to_market_cap")),
            net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap: Indexes::new(client.clone(), &format!("{base_path}/net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap")),
            net_realized_pnl_rel_to_realized_cap: Indexes2::new(client.clone(), &format!("{base_path}/net_realized_pnl_rel_to_realized_cap")),
            realized_cap: Indexes3::new(client.clone(), &format!("{base_path}/realized_cap")),
            realized_cap_30d_delta: Indexes::new(client.clone(), &format!("{base_path}/realized_cap_30d_delta")),
            realized_cap_rel_to_own_market_cap: Indexes3::new(client.clone(), &format!("{base_path}/realized_cap_rel_to_own_market_cap")),
            realized_loss: BlockCountPattern::new(client.clone(), &format!("{base_path}/realized_loss")),
            realized_loss_rel_to_realized_cap: Indexes2::new(client.clone(), &format!("{base_path}/realized_loss_rel_to_realized_cap")),
            realized_price: Indexes3::new(client.clone(), &format!("{base_path}/realized_price")),
            realized_price_extra: ActivePriceRatioPattern::new(client.clone(), &format!("{base_path}/realized_price_extra")),
            realized_profit: BlockCountPattern::new(client.clone(), &format!("{base_path}/realized_profit")),
            realized_profit_rel_to_realized_cap: Indexes2::new(client.clone(), &format!("{base_path}/realized_profit_rel_to_realized_cap")),
            realized_profit_to_loss_ratio: Indexes5::new(client.clone(), &format!("{base_path}/realized_profit_to_loss_ratio")),
            realized_value: Indexes3::new(client.clone(), &format!("{base_path}/realized_value")),
            sell_side_risk_ratio: Indexes5::new(client.clone(), &format!("{base_path}/sell_side_risk_ratio")),
            sell_side_risk_ratio_30d_ema: Indexes5::new(client.clone(), &format!("{base_path}/sell_side_risk_ratio_30d_ema")),
            sell_side_risk_ratio_7d_ema: Indexes5::new(client.clone(), &format!("{base_path}/sell_side_risk_ratio_7d_ema")),
            sopr: Indexes5::new(client.clone(), &format!("{base_path}/sopr")),
            sopr_30d_ema: Indexes5::new(client.clone(), &format!("{base_path}/sopr_30d_ema")),
            sopr_7d_ema: Indexes5::new(client.clone(), &format!("{base_path}/sopr_7d_ema")),
            total_realized_pnl: BitcoinPattern2::new(client.clone(), &format!("{base_path}/total_realized_pnl")),
            value_created: Indexes3::new(client.clone(), &format!("{base_path}/value_created")),
            value_destroyed: Indexes3::new(client.clone(), &format!("{base_path}/value_destroyed")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct RealizedPattern {
    pub neg_realized_loss: BlockCountPattern<Dollars>,
    pub net_realized_pnl: BlockCountPattern<Dollars>,
    pub net_realized_pnl_cumulative_30d_delta: Indexes<Dollars>,
    pub net_realized_pnl_cumulative_30d_delta_rel_to_market_cap: Indexes<StoredF32>,
    pub net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap: Indexes<StoredF32>,
    pub net_realized_pnl_rel_to_realized_cap: Indexes2<StoredF32>,
    pub realized_cap: Indexes3<Dollars>,
    pub realized_cap_30d_delta: Indexes<Dollars>,
    pub realized_loss: BlockCountPattern<Dollars>,
    pub realized_loss_rel_to_realized_cap: Indexes2<StoredF32>,
    pub realized_price: Indexes3<Dollars>,
    pub realized_price_extra: RealizedPriceExtraPattern,
    pub realized_profit: BlockCountPattern<Dollars>,
    pub realized_profit_rel_to_realized_cap: Indexes2<StoredF32>,
    pub realized_value: Indexes3<Dollars>,
    pub sell_side_risk_ratio: Indexes5<StoredF32>,
    pub sell_side_risk_ratio_30d_ema: Indexes5<StoredF32>,
    pub sell_side_risk_ratio_7d_ema: Indexes5<StoredF32>,
    pub sopr: Indexes5<StoredF64>,
    pub sopr_30d_ema: Indexes5<StoredF64>,
    pub sopr_7d_ema: Indexes5<StoredF64>,
    pub total_realized_pnl: BitcoinPattern2<Dollars>,
    pub value_created: Indexes3<Dollars>,
    pub value_destroyed: Indexes3<Dollars>,
}

impl RealizedPattern {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            neg_realized_loss: BlockCountPattern::new(client.clone(), &format!("{base_path}/neg_realized_loss")),
            net_realized_pnl: BlockCountPattern::new(client.clone(), &format!("{base_path}/net_realized_pnl")),
            net_realized_pnl_cumulative_30d_delta: Indexes::new(client.clone(), &format!("{base_path}/net_realized_pnl_cumulative_30d_delta")),
            net_realized_pnl_cumulative_30d_delta_rel_to_market_cap: Indexes::new(client.clone(), &format!("{base_path}/net_realized_pnl_cumulative_30d_delta_rel_to_market_cap")),
            net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap: Indexes::new(client.clone(), &format!("{base_path}/net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap")),
            net_realized_pnl_rel_to_realized_cap: Indexes2::new(client.clone(), &format!("{base_path}/net_realized_pnl_rel_to_realized_cap")),
            realized_cap: Indexes3::new(client.clone(), &format!("{base_path}/realized_cap")),
            realized_cap_30d_delta: Indexes::new(client.clone(), &format!("{base_path}/realized_cap_30d_delta")),
            realized_loss: BlockCountPattern::new(client.clone(), &format!("{base_path}/realized_loss")),
            realized_loss_rel_to_realized_cap: Indexes2::new(client.clone(), &format!("{base_path}/realized_loss_rel_to_realized_cap")),
            realized_price: Indexes3::new(client.clone(), &format!("{base_path}/realized_price")),
            realized_price_extra: RealizedPriceExtraPattern::new(client.clone(), &format!("{base_path}/realized_price_extra")),
            realized_profit: BlockCountPattern::new(client.clone(), &format!("{base_path}/realized_profit")),
            realized_profit_rel_to_realized_cap: Indexes2::new(client.clone(), &format!("{base_path}/realized_profit_rel_to_realized_cap")),
            realized_value: Indexes3::new(client.clone(), &format!("{base_path}/realized_value")),
            sell_side_risk_ratio: Indexes5::new(client.clone(), &format!("{base_path}/sell_side_risk_ratio")),
            sell_side_risk_ratio_30d_ema: Indexes5::new(client.clone(), &format!("{base_path}/sell_side_risk_ratio_30d_ema")),
            sell_side_risk_ratio_7d_ema: Indexes5::new(client.clone(), &format!("{base_path}/sell_side_risk_ratio_7d_ema")),
            sopr: Indexes5::new(client.clone(), &format!("{base_path}/sopr")),
            sopr_30d_ema: Indexes5::new(client.clone(), &format!("{base_path}/sopr_30d_ema")),
            sopr_7d_ema: Indexes5::new(client.clone(), &format!("{base_path}/sopr_7d_ema")),
            total_realized_pnl: BitcoinPattern2::new(client.clone(), &format!("{base_path}/total_realized_pnl")),
            value_created: Indexes3::new(client.clone(), &format!("{base_path}/value_created")),
            value_destroyed: Indexes3::new(client.clone(), &format!("{base_path}/value_destroyed")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct Price13dEmaPattern {
    pub price: Indexes<Dollars>,
    pub ratio: Indexes<StoredF32>,
    pub ratio_1m_sma: Indexes<StoredF32>,
    pub ratio_1w_sma: Indexes<StoredF32>,
    pub ratio_1y_sd: Ratio1ySdPattern,
    pub ratio_2y_sd: Ratio1ySdPattern,
    pub ratio_4y_sd: Ratio1ySdPattern,
    pub ratio_pct1: Indexes<StoredF32>,
    pub ratio_pct1_usd: Indexes<Dollars>,
    pub ratio_pct2: Indexes<StoredF32>,
    pub ratio_pct2_usd: Indexes<Dollars>,
    pub ratio_pct5: Indexes<StoredF32>,
    pub ratio_pct5_usd: Indexes<Dollars>,
    pub ratio_pct95: Indexes<StoredF32>,
    pub ratio_pct95_usd: Indexes<Dollars>,
    pub ratio_pct98: Indexes<StoredF32>,
    pub ratio_pct98_usd: Indexes<Dollars>,
    pub ratio_pct99: Indexes<StoredF32>,
    pub ratio_pct99_usd: Indexes<Dollars>,
    pub ratio_sd: Ratio1ySdPattern,
}

impl Price13dEmaPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: &str) -> Self {
        Self {
            price: Indexes::new(client.clone(), &format!("/{acc}")),
            ratio: Indexes::new(client.clone(), &format!("/{acc}_ratio")),
            ratio_1m_sma: Indexes::new(client.clone(), &format!("/{acc}_ratio_1m_sma")),
            ratio_1w_sma: Indexes::new(client.clone(), &format!("/{acc}_ratio_1w_sma")),
            ratio_1y_sd: Ratio1ySdPattern::new(client.clone(), &format!("{acc}_ratio_1y_sd")),
            ratio_2y_sd: Ratio1ySdPattern::new(client.clone(), &format!("{acc}_ratio_2y_sd")),
            ratio_4y_sd: Ratio1ySdPattern::new(client.clone(), &format!("{acc}_ratio_4y_sd")),
            ratio_pct1: Indexes::new(client.clone(), &format!("/{acc}_ratio_pct1")),
            ratio_pct1_usd: Indexes::new(client.clone(), &format!("/{acc}_ratio_pct1_usd")),
            ratio_pct2: Indexes::new(client.clone(), &format!("/{acc}_ratio_pct2")),
            ratio_pct2_usd: Indexes::new(client.clone(), &format!("/{acc}_ratio_pct2_usd")),
            ratio_pct5: Indexes::new(client.clone(), &format!("/{acc}_ratio_pct5")),
            ratio_pct5_usd: Indexes::new(client.clone(), &format!("/{acc}_ratio_pct5_usd")),
            ratio_pct95: Indexes::new(client.clone(), &format!("/{acc}_ratio_pct95")),
            ratio_pct95_usd: Indexes::new(client.clone(), &format!("/{acc}_ratio_pct95_usd")),
            ratio_pct98: Indexes::new(client.clone(), &format!("/{acc}_ratio_pct98")),
            ratio_pct98_usd: Indexes::new(client.clone(), &format!("/{acc}_ratio_pct98_usd")),
            ratio_pct99: Indexes::new(client.clone(), &format!("/{acc}_ratio_pct99")),
            ratio_pct99_usd: Indexes::new(client.clone(), &format!("/{acc}_ratio_pct99_usd")),
            ratio_sd: Ratio1ySdPattern::new(client.clone(), &format!("{acc}_ratio_sd")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct PricePercentilesPattern {
    pub pct05: Indexes<Dollars>,
    pub pct10: Indexes<Dollars>,
    pub pct15: Indexes<Dollars>,
    pub pct20: Indexes<Dollars>,
    pub pct25: Indexes<Dollars>,
    pub pct30: Indexes<Dollars>,
    pub pct35: Indexes<Dollars>,
    pub pct40: Indexes<Dollars>,
    pub pct45: Indexes<Dollars>,
    pub pct50: Indexes<Dollars>,
    pub pct55: Indexes<Dollars>,
    pub pct60: Indexes<Dollars>,
    pub pct65: Indexes<Dollars>,
    pub pct70: Indexes<Dollars>,
    pub pct75: Indexes<Dollars>,
    pub pct80: Indexes<Dollars>,
    pub pct85: Indexes<Dollars>,
    pub pct90: Indexes<Dollars>,
    pub pct95: Indexes<Dollars>,
}

impl PricePercentilesPattern {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            pct05: Indexes::new(client.clone(), &format!("{base_path}/pct05")),
            pct10: Indexes::new(client.clone(), &format!("{base_path}/pct10")),
            pct15: Indexes::new(client.clone(), &format!("{base_path}/pct15")),
            pct20: Indexes::new(client.clone(), &format!("{base_path}/pct20")),
            pct25: Indexes::new(client.clone(), &format!("{base_path}/pct25")),
            pct30: Indexes::new(client.clone(), &format!("{base_path}/pct30")),
            pct35: Indexes::new(client.clone(), &format!("{base_path}/pct35")),
            pct40: Indexes::new(client.clone(), &format!("{base_path}/pct40")),
            pct45: Indexes::new(client.clone(), &format!("{base_path}/pct45")),
            pct50: Indexes::new(client.clone(), &format!("{base_path}/pct50")),
            pct55: Indexes::new(client.clone(), &format!("{base_path}/pct55")),
            pct60: Indexes::new(client.clone(), &format!("{base_path}/pct60")),
            pct65: Indexes::new(client.clone(), &format!("{base_path}/pct65")),
            pct70: Indexes::new(client.clone(), &format!("{base_path}/pct70")),
            pct75: Indexes::new(client.clone(), &format!("{base_path}/pct75")),
            pct80: Indexes::new(client.clone(), &format!("{base_path}/pct80")),
            pct85: Indexes::new(client.clone(), &format!("{base_path}/pct85")),
            pct90: Indexes::new(client.clone(), &format!("{base_path}/pct90")),
            pct95: Indexes::new(client.clone(), &format!("{base_path}/pct95")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct ActivePriceRatioPattern {
    pub ratio: Indexes<StoredF32>,
    pub ratio_1m_sma: Indexes<StoredF32>,
    pub ratio_1w_sma: Indexes<StoredF32>,
    pub ratio_1y_sd: Ratio1ySdPattern,
    pub ratio_2y_sd: Ratio1ySdPattern,
    pub ratio_4y_sd: Ratio1ySdPattern,
    pub ratio_pct1: Indexes<StoredF32>,
    pub ratio_pct1_usd: Indexes<Dollars>,
    pub ratio_pct2: Indexes<StoredF32>,
    pub ratio_pct2_usd: Indexes<Dollars>,
    pub ratio_pct5: Indexes<StoredF32>,
    pub ratio_pct5_usd: Indexes<Dollars>,
    pub ratio_pct95: Indexes<StoredF32>,
    pub ratio_pct95_usd: Indexes<Dollars>,
    pub ratio_pct98: Indexes<StoredF32>,
    pub ratio_pct98_usd: Indexes<Dollars>,
    pub ratio_pct99: Indexes<StoredF32>,
    pub ratio_pct99_usd: Indexes<Dollars>,
    pub ratio_sd: Ratio1ySdPattern,
}

impl ActivePriceRatioPattern {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            ratio: Indexes::new(client.clone(), &format!("{base_path}/ratio")),
            ratio_1m_sma: Indexes::new(client.clone(), &format!("{base_path}/ratio_1m_sma")),
            ratio_1w_sma: Indexes::new(client.clone(), &format!("{base_path}/ratio_1w_sma")),
            ratio_1y_sd: Ratio1ySdPattern::new(client.clone(), &format!("{base_path}/ratio_1y_sd")),
            ratio_2y_sd: Ratio1ySdPattern::new(client.clone(), &format!("{base_path}/ratio_2y_sd")),
            ratio_4y_sd: Ratio1ySdPattern::new(client.clone(), &format!("{base_path}/ratio_4y_sd")),
            ratio_pct1: Indexes::new(client.clone(), &format!("{base_path}/ratio_pct1")),
            ratio_pct1_usd: Indexes::new(client.clone(), &format!("{base_path}/ratio_pct1_usd")),
            ratio_pct2: Indexes::new(client.clone(), &format!("{base_path}/ratio_pct2")),
            ratio_pct2_usd: Indexes::new(client.clone(), &format!("{base_path}/ratio_pct2_usd")),
            ratio_pct5: Indexes::new(client.clone(), &format!("{base_path}/ratio_pct5")),
            ratio_pct5_usd: Indexes::new(client.clone(), &format!("{base_path}/ratio_pct5_usd")),
            ratio_pct95: Indexes::new(client.clone(), &format!("{base_path}/ratio_pct95")),
            ratio_pct95_usd: Indexes::new(client.clone(), &format!("{base_path}/ratio_pct95_usd")),
            ratio_pct98: Indexes::new(client.clone(), &format!("{base_path}/ratio_pct98")),
            ratio_pct98_usd: Indexes::new(client.clone(), &format!("{base_path}/ratio_pct98_usd")),
            ratio_pct99: Indexes::new(client.clone(), &format!("{base_path}/ratio_pct99")),
            ratio_pct99_usd: Indexes::new(client.clone(), &format!("{base_path}/ratio_pct99_usd")),
            ratio_sd: Ratio1ySdPattern::new(client.clone(), &format!("{base_path}/ratio_sd")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct RelativePattern2 {
    pub neg_unrealized_loss_rel_to_market_cap: Indexes27<StoredF32>,
    pub neg_unrealized_loss_rel_to_own_market_cap: Indexes27<StoredF32>,
    pub neg_unrealized_loss_rel_to_own_total_unrealized_pnl: Indexes27<StoredF32>,
    pub net_unrealized_pnl_rel_to_market_cap: Indexes26<StoredF32>,
    pub net_unrealized_pnl_rel_to_own_market_cap: Indexes26<StoredF32>,
    pub net_unrealized_pnl_rel_to_own_total_unrealized_pnl: Indexes26<StoredF32>,
    pub supply_in_loss_rel_to_circulating_supply: Indexes27<StoredF64>,
    pub supply_in_loss_rel_to_own_supply: Indexes27<StoredF64>,
    pub supply_in_profit_rel_to_circulating_supply: Indexes27<StoredF64>,
    pub supply_in_profit_rel_to_own_supply: Indexes27<StoredF64>,
    pub supply_rel_to_circulating_supply: Indexes<StoredF64>,
    pub unrealized_loss_rel_to_market_cap: Indexes27<StoredF32>,
    pub unrealized_loss_rel_to_own_market_cap: Indexes27<StoredF32>,
    pub unrealized_loss_rel_to_own_total_unrealized_pnl: Indexes27<StoredF32>,
    pub unrealized_profit_rel_to_market_cap: Indexes27<StoredF32>,
    pub unrealized_profit_rel_to_own_market_cap: Indexes27<StoredF32>,
    pub unrealized_profit_rel_to_own_total_unrealized_pnl: Indexes27<StoredF32>,
}

impl RelativePattern2 {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            neg_unrealized_loss_rel_to_market_cap: Indexes27::new(client.clone(), &format!("{base_path}/neg_unrealized_loss_rel_to_market_cap")),
            neg_unrealized_loss_rel_to_own_market_cap: Indexes27::new(client.clone(), &format!("{base_path}/neg_unrealized_loss_rel_to_own_market_cap")),
            neg_unrealized_loss_rel_to_own_total_unrealized_pnl: Indexes27::new(client.clone(), &format!("{base_path}/neg_unrealized_loss_rel_to_own_total_unrealized_pnl")),
            net_unrealized_pnl_rel_to_market_cap: Indexes26::new(client.clone(), &format!("{base_path}/net_unrealized_pnl_rel_to_market_cap")),
            net_unrealized_pnl_rel_to_own_market_cap: Indexes26::new(client.clone(), &format!("{base_path}/net_unrealized_pnl_rel_to_own_market_cap")),
            net_unrealized_pnl_rel_to_own_total_unrealized_pnl: Indexes26::new(client.clone(), &format!("{base_path}/net_unrealized_pnl_rel_to_own_total_unrealized_pnl")),
            supply_in_loss_rel_to_circulating_supply: Indexes27::new(client.clone(), &format!("{base_path}/supply_in_loss_rel_to_circulating_supply")),
            supply_in_loss_rel_to_own_supply: Indexes27::new(client.clone(), &format!("{base_path}/supply_in_loss_rel_to_own_supply")),
            supply_in_profit_rel_to_circulating_supply: Indexes27::new(client.clone(), &format!("{base_path}/supply_in_profit_rel_to_circulating_supply")),
            supply_in_profit_rel_to_own_supply: Indexes27::new(client.clone(), &format!("{base_path}/supply_in_profit_rel_to_own_supply")),
            supply_rel_to_circulating_supply: Indexes::new(client.clone(), &format!("{base_path}/supply_rel_to_circulating_supply")),
            unrealized_loss_rel_to_market_cap: Indexes27::new(client.clone(), &format!("{base_path}/unrealized_loss_rel_to_market_cap")),
            unrealized_loss_rel_to_own_market_cap: Indexes27::new(client.clone(), &format!("{base_path}/unrealized_loss_rel_to_own_market_cap")),
            unrealized_loss_rel_to_own_total_unrealized_pnl: Indexes27::new(client.clone(), &format!("{base_path}/unrealized_loss_rel_to_own_total_unrealized_pnl")),
            unrealized_profit_rel_to_market_cap: Indexes27::new(client.clone(), &format!("{base_path}/unrealized_profit_rel_to_market_cap")),
            unrealized_profit_rel_to_own_market_cap: Indexes27::new(client.clone(), &format!("{base_path}/unrealized_profit_rel_to_own_market_cap")),
            unrealized_profit_rel_to_own_total_unrealized_pnl: Indexes27::new(client.clone(), &format!("{base_path}/unrealized_profit_rel_to_own_total_unrealized_pnl")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct AXbtPattern {
    pub _1d_dominance: BlockCountPattern<StoredF32>,
    pub _1m_blocks_mined: Indexes<StoredU32>,
    pub _1m_dominance: Indexes<StoredF32>,
    pub _1w_blocks_mined: Indexes<StoredU32>,
    pub _1w_dominance: Indexes<StoredF32>,
    pub _1y_blocks_mined: Indexes<StoredU32>,
    pub _1y_dominance: Indexes<StoredF32>,
    pub blocks_mined: BlockCountPattern<StoredU32>,
    pub coinbase: UnclaimedRewardsPattern,
    pub days_since_block: Indexes<StoredU16>,
    pub dominance: BlockCountPattern<StoredF32>,
    pub fee: FeePattern2,
    pub subsidy: FeePattern2,
}

impl AXbtPattern {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            _1d_dominance: BlockCountPattern::new(client.clone(), &format!("{base_path}/1d_dominance")),
            _1m_blocks_mined: Indexes::new(client.clone(), &format!("{base_path}/1m_blocks_mined")),
            _1m_dominance: Indexes::new(client.clone(), &format!("{base_path}/1m_dominance")),
            _1w_blocks_mined: Indexes::new(client.clone(), &format!("{base_path}/1w_blocks_mined")),
            _1w_dominance: Indexes::new(client.clone(), &format!("{base_path}/1w_dominance")),
            _1y_blocks_mined: Indexes::new(client.clone(), &format!("{base_path}/1y_blocks_mined")),
            _1y_dominance: Indexes::new(client.clone(), &format!("{base_path}/1y_dominance")),
            blocks_mined: BlockCountPattern::new(client.clone(), &format!("{base_path}/blocks_mined")),
            coinbase: UnclaimedRewardsPattern::new(client.clone(), &format!("{base_path}/coinbase")),
            days_since_block: Indexes::new(client.clone(), &format!("{base_path}/days_since_block")),
            dominance: BlockCountPattern::new(client.clone(), &format!("{base_path}/dominance")),
            fee: FeePattern2::new(client.clone(), &format!("{base_path}/fee")),
            subsidy: FeePattern2::new(client.clone(), &format!("{base_path}/subsidy")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct BitcoinPattern<T> {
    pub average: Indexes4<T>,
    pub base: Indexes2<T>,
    pub cumulative: Indexes3<T>,
    pub max: Indexes4<T>,
    pub median: Indexes5<T>,
    pub min: Indexes4<T>,
    pub pct10: Indexes5<T>,
    pub pct25: Indexes5<T>,
    pub pct75: Indexes5<T>,
    pub pct90: Indexes5<T>,
    pub sum: Indexes4<T>,
}

impl<T: DeserializeOwned> BitcoinPattern<T> {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            average: Indexes4::new(client.clone(), &format!("{base_path}/average")),
            base: Indexes2::new(client.clone(), &format!("{base_path}/base")),
            cumulative: Indexes3::new(client.clone(), &format!("{base_path}/cumulative")),
            max: Indexes4::new(client.clone(), &format!("{base_path}/max")),
            median: Indexes5::new(client.clone(), &format!("{base_path}/median")),
            min: Indexes4::new(client.clone(), &format!("{base_path}/min")),
            pct10: Indexes5::new(client.clone(), &format!("{base_path}/pct10")),
            pct25: Indexes5::new(client.clone(), &format!("{base_path}/pct25")),
            pct75: Indexes5::new(client.clone(), &format!("{base_path}/pct75")),
            pct90: Indexes5::new(client.clone(), &format!("{base_path}/pct90")),
            sum: Indexes4::new(client.clone(), &format!("{base_path}/sum")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct BlockSizePattern<T> {
    pub average: Indexes3<T>,
    pub cumulative: Indexes3<T>,
    pub max: Indexes3<T>,
    pub median: Indexes2<T>,
    pub min: Indexes3<T>,
    pub pct10: Indexes2<T>,
    pub pct25: Indexes2<T>,
    pub pct75: Indexes2<T>,
    pub pct90: Indexes2<T>,
    pub sum: Indexes3<T>,
}

impl<T: DeserializeOwned> BlockSizePattern<T> {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            average: Indexes3::new(client.clone(), &format!("{base_path}/average")),
            cumulative: Indexes3::new(client.clone(), &format!("{base_path}/cumulative")),
            max: Indexes3::new(client.clone(), &format!("{base_path}/max")),
            median: Indexes2::new(client.clone(), &format!("{base_path}/median")),
            min: Indexes3::new(client.clone(), &format!("{base_path}/min")),
            pct10: Indexes2::new(client.clone(), &format!("{base_path}/pct10")),
            pct25: Indexes2::new(client.clone(), &format!("{base_path}/pct25")),
            pct75: Indexes2::new(client.clone(), &format!("{base_path}/pct75")),
            pct90: Indexes2::new(client.clone(), &format!("{base_path}/pct90")),
            sum: Indexes3::new(client.clone(), &format!("{base_path}/sum")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct RelativePattern {
    pub neg_unrealized_loss_rel_to_market_cap: Indexes27<StoredF32>,
    pub net_unrealized_pnl_rel_to_market_cap: Indexes26<StoredF32>,
    pub supply_in_loss_rel_to_circulating_supply: Indexes27<StoredF64>,
    pub supply_in_loss_rel_to_own_supply: Indexes27<StoredF64>,
    pub supply_in_profit_rel_to_circulating_supply: Indexes27<StoredF64>,
    pub supply_in_profit_rel_to_own_supply: Indexes27<StoredF64>,
    pub supply_rel_to_circulating_supply: Indexes<StoredF64>,
    pub unrealized_loss_rel_to_market_cap: Indexes27<StoredF32>,
    pub unrealized_profit_rel_to_market_cap: Indexes27<StoredF32>,
}

impl RelativePattern {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            neg_unrealized_loss_rel_to_market_cap: Indexes27::new(client.clone(), &format!("{base_path}/neg_unrealized_loss_rel_to_market_cap")),
            net_unrealized_pnl_rel_to_market_cap: Indexes26::new(client.clone(), &format!("{base_path}/net_unrealized_pnl_rel_to_market_cap")),
            supply_in_loss_rel_to_circulating_supply: Indexes27::new(client.clone(), &format!("{base_path}/supply_in_loss_rel_to_circulating_supply")),
            supply_in_loss_rel_to_own_supply: Indexes27::new(client.clone(), &format!("{base_path}/supply_in_loss_rel_to_own_supply")),
            supply_in_profit_rel_to_circulating_supply: Indexes27::new(client.clone(), &format!("{base_path}/supply_in_profit_rel_to_circulating_supply")),
            supply_in_profit_rel_to_own_supply: Indexes27::new(client.clone(), &format!("{base_path}/supply_in_profit_rel_to_own_supply")),
            supply_rel_to_circulating_supply: Indexes::new(client.clone(), &format!("{base_path}/supply_rel_to_circulating_supply")),
            unrealized_loss_rel_to_market_cap: Indexes27::new(client.clone(), &format!("{base_path}/unrealized_loss_rel_to_market_cap")),
            unrealized_profit_rel_to_market_cap: Indexes27::new(client.clone(), &format!("{base_path}/unrealized_profit_rel_to_market_cap")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct UnrealizedPattern {
    pub neg_unrealized_loss: Indexes26<Dollars>,
    pub net_unrealized_pnl: Indexes26<Dollars>,
    pub supply_in_loss: SupplyPattern,
    pub supply_in_loss_value: SupplyValuePattern,
    pub supply_in_profit: SupplyPattern,
    pub supply_in_profit_value: SupplyValuePattern,
    pub total_unrealized_pnl: Indexes26<Dollars>,
    pub unrealized_loss: Indexes26<Dollars>,
    pub unrealized_profit: Indexes26<Dollars>,
}

impl UnrealizedPattern {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            neg_unrealized_loss: Indexes26::new(client.clone(), &format!("{base_path}/neg_unrealized_loss")),
            net_unrealized_pnl: Indexes26::new(client.clone(), &format!("{base_path}/net_unrealized_pnl")),
            supply_in_loss: SupplyPattern::new(client.clone(), &format!("{base_path}/supply_in_loss")),
            supply_in_loss_value: SupplyValuePattern::new(client.clone(), &format!("{base_path}/supply_in_loss_value")),
            supply_in_profit: SupplyPattern::new(client.clone(), &format!("{base_path}/supply_in_profit")),
            supply_in_profit_value: SupplyValuePattern::new(client.clone(), &format!("{base_path}/supply_in_profit_value")),
            total_unrealized_pnl: Indexes26::new(client.clone(), &format!("{base_path}/total_unrealized_pnl")),
            unrealized_loss: Indexes26::new(client.clone(), &format!("{base_path}/unrealized_loss")),
            unrealized_profit: Indexes26::new(client.clone(), &format!("{base_path}/unrealized_profit")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct BlockIntervalPattern<T> {
    pub average: Indexes3<T>,
    pub max: Indexes3<T>,
    pub median: Indexes2<T>,
    pub min: Indexes3<T>,
    pub pct10: Indexes2<T>,
    pub pct25: Indexes2<T>,
    pub pct75: Indexes2<T>,
    pub pct90: Indexes2<T>,
}

impl<T: DeserializeOwned> BlockIntervalPattern<T> {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: &str) -> Self {
        Self {
            average: Indexes3::new(client.clone(), &format!("/{acc}_avg")),
            max: Indexes3::new(client.clone(), &format!("/{acc}_max")),
            median: Indexes2::new(client.clone(), &format!("/{acc}_median")),
            min: Indexes3::new(client.clone(), &format!("/{acc}_min")),
            pct10: Indexes2::new(client.clone(), &format!("/{acc}_pct10")),
            pct25: Indexes2::new(client.clone(), &format!("/{acc}_pct25")),
            pct75: Indexes2::new(client.clone(), &format!("/{acc}_pct75")),
            pct90: Indexes2::new(client.clone(), &format!("/{acc}_pct90")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct Constant0Pattern<T> {
    pub dateindex: Indexes5<T>,
    pub decadeindex: Indexes7<T>,
    pub height: Indexes2<T>,
    pub monthindex: Indexes8<T>,
    pub quarterindex: Indexes9<T>,
    pub semesterindex: Indexes10<T>,
    pub weekindex: Indexes11<T>,
    pub yearindex: Indexes12<T>,
}

impl<T: DeserializeOwned> Constant0Pattern<T> {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: &str) -> Self {
        Self {
            dateindex: Indexes5::new(client.clone(), &format!("/{acc}")),
            decadeindex: Indexes7::new(client.clone(), &format!("/{acc}")),
            height: Indexes2::new(client.clone(), &format!("/{acc}")),
            monthindex: Indexes8::new(client.clone(), &format!("/{acc}")),
            quarterindex: Indexes9::new(client.clone(), &format!("/{acc}")),
            semesterindex: Indexes10::new(client.clone(), &format!("/{acc}")),
            weekindex: Indexes11::new(client.clone(), &format!("/{acc}")),
            yearindex: Indexes12::new(client.clone(), &format!("/{acc}")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct AddresstypeToHeightToAddrCountPattern<T> {
    pub p2a: Indexes2<T>,
    pub p2pk33: Indexes2<T>,
    pub p2pk65: Indexes2<T>,
    pub p2pkh: Indexes2<T>,
    pub p2sh: Indexes2<T>,
    pub p2tr: Indexes2<T>,
    pub p2wpkh: Indexes2<T>,
    pub p2wsh: Indexes2<T>,
}

impl<T: DeserializeOwned> AddresstypeToHeightToAddrCountPattern<T> {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            p2a: Indexes2::new(client.clone(), &format!("{base_path}/p2a")),
            p2pk33: Indexes2::new(client.clone(), &format!("{base_path}/p2pk33")),
            p2pk65: Indexes2::new(client.clone(), &format!("{base_path}/p2pk65")),
            p2pkh: Indexes2::new(client.clone(), &format!("{base_path}/p2pkh")),
            p2sh: Indexes2::new(client.clone(), &format!("{base_path}/p2sh")),
            p2tr: Indexes2::new(client.clone(), &format!("{base_path}/p2tr")),
            p2wpkh: Indexes2::new(client.clone(), &format!("{base_path}/p2wpkh")),
            p2wsh: Indexes2::new(client.clone(), &format!("{base_path}/p2wsh")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct _0satsPattern {
    pub activity: ActivityPattern,
    pub addr_count: Indexes3<StoredU64>,
    pub price_paid: PricePaidPattern,
    pub realized: RealizedPattern,
    pub relative: RelativePattern,
    pub supply: SupplyPattern2,
    pub unrealized: UnrealizedPattern,
}

impl _0satsPattern {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            activity: ActivityPattern::new(client.clone(), &format!("{base_path}/activity")),
            addr_count: Indexes3::new(client.clone(), &format!("{base_path}/addr_count")),
            price_paid: PricePaidPattern::new(client.clone(), &format!("{base_path}/price_paid")),
            realized: RealizedPattern::new(client.clone(), &format!("{base_path}/realized")),
            relative: RelativePattern::new(client.clone(), &format!("{base_path}/relative")),
            supply: SupplyPattern2::new(client.clone(), &format!("{base_path}/supply")),
            unrealized: UnrealizedPattern::new(client.clone(), &format!("{base_path}/unrealized")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct _10yTo12yPattern {
    pub activity: ActivityPattern,
    pub price_paid: PricePaidPattern2,
    pub realized: RealizedPattern2,
    pub relative: RelativePattern2,
    pub supply: SupplyPattern2,
    pub unrealized: UnrealizedPattern,
}

impl _10yTo12yPattern {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            activity: ActivityPattern::new(client.clone(), &format!("{base_path}/activity")),
            price_paid: PricePaidPattern2::new(client.clone(), &format!("{base_path}/price_paid")),
            realized: RealizedPattern2::new(client.clone(), &format!("{base_path}/realized")),
            relative: RelativePattern2::new(client.clone(), &format!("{base_path}/relative")),
            supply: SupplyPattern2::new(client.clone(), &format!("{base_path}/supply")),
            unrealized: UnrealizedPattern::new(client.clone(), &format!("{base_path}/unrealized")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct _0satsPattern2 {
    pub activity: ActivityPattern,
    pub price_paid: PricePaidPattern,
    pub realized: RealizedPattern,
    pub relative: RelativePattern,
    pub supply: SupplyPattern2,
    pub unrealized: UnrealizedPattern,
}

impl _0satsPattern2 {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            activity: ActivityPattern::new(client.clone(), &format!("{base_path}/activity")),
            price_paid: PricePaidPattern::new(client.clone(), &format!("{base_path}/price_paid")),
            realized: RealizedPattern::new(client.clone(), &format!("{base_path}/realized")),
            relative: RelativePattern::new(client.clone(), &format!("{base_path}/relative")),
            supply: SupplyPattern2::new(client.clone(), &format!("{base_path}/supply")),
            unrealized: UnrealizedPattern::new(client.clone(), &format!("{base_path}/unrealized")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct UpTo1dPattern {
    pub activity: ActivityPattern,
    pub price_paid: PricePaidPattern2,
    pub realized: RealizedPattern3,
    pub relative: RelativePattern2,
    pub supply: SupplyPattern2,
    pub unrealized: UnrealizedPattern,
}

impl UpTo1dPattern {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            activity: ActivityPattern::new(client.clone(), &format!("{base_path}/activity")),
            price_paid: PricePaidPattern2::new(client.clone(), &format!("{base_path}/price_paid")),
            realized: RealizedPattern3::new(client.clone(), &format!("{base_path}/realized")),
            relative: RelativePattern2::new(client.clone(), &format!("{base_path}/relative")),
            supply: SupplyPattern2::new(client.clone(), &format!("{base_path}/supply")),
            unrealized: UnrealizedPattern::new(client.clone(), &format!("{base_path}/unrealized")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct SupplyPattern2 {
    pub supply: SupplyPattern,
    pub supply_half: ActiveSupplyPattern,
    pub supply_half_value: ActiveSupplyPattern,
    pub supply_value: SupplyValuePattern,
    pub utxo_count: Indexes3<StoredU64>,
}

impl SupplyPattern2 {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            supply: SupplyPattern::new(client.clone(), &format!("{base_path}/supply")),
            supply_half: ActiveSupplyPattern::new(client.clone(), &format!("{base_path}/supply_half")),
            supply_half_value: ActiveSupplyPattern::new(client.clone(), &format!("{base_path}/supply_half_value")),
            supply_value: SupplyValuePattern::new(client.clone(), &format!("{base_path}/supply_value")),
            utxo_count: Indexes3::new(client.clone(), &format!("{base_path}/utxo_count")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct ActivityPattern {
    pub coinblocks_destroyed: BlockCountPattern<StoredF64>,
    pub coindays_destroyed: BlockCountPattern<StoredF64>,
    pub satblocks_destroyed: Indexes2<Sats>,
    pub satdays_destroyed: Indexes2<Sats>,
    pub sent: FeePattern2,
}

impl ActivityPattern {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            coinblocks_destroyed: BlockCountPattern::new(client.clone(), &format!("{base_path}/coinblocks_destroyed")),
            coindays_destroyed: BlockCountPattern::new(client.clone(), &format!("{base_path}/coindays_destroyed")),
            satblocks_destroyed: Indexes2::new(client.clone(), &format!("{base_path}/satblocks_destroyed")),
            satdays_destroyed: Indexes2::new(client.clone(), &format!("{base_path}/satdays_destroyed")),
            sent: FeePattern2::new(client.clone(), &format!("{base_path}/sent")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct SupplyPattern {
    pub base: Indexes2<Sats>,
    pub bitcoin: Indexes<Bitcoin>,
    pub dollars: Indexes<Dollars>,
    pub sats: Indexes<Sats>,
}

impl SupplyPattern {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            base: Indexes2::new(client.clone(), &format!("{base_path}/base")),
            bitcoin: Indexes::new(client.clone(), &format!("{base_path}/bitcoin")),
            dollars: Indexes::new(client.clone(), &format!("{base_path}/dollars")),
            sats: Indexes::new(client.clone(), &format!("{base_path}/sats")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct FeePattern2 {
    pub base: Indexes2<Sats>,
    pub bitcoin: BlockCountPattern<Bitcoin>,
    pub dollars: BlockCountPattern<Dollars>,
    pub sats: SatsPattern,
}

impl FeePattern2 {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            base: Indexes2::new(client.clone(), &format!("{base_path}/base")),
            bitcoin: BlockCountPattern::new(client.clone(), &format!("{base_path}/bitcoin")),
            dollars: BlockCountPattern::new(client.clone(), &format!("{base_path}/dollars")),
            sats: SatsPattern::new(client.clone(), &format!("{base_path}/sats")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct ActiveSupplyPattern {
    pub bitcoin: Indexes3<Bitcoin>,
    pub dollars: Indexes3<Dollars>,
    pub sats: Indexes3<Sats>,
}

impl ActiveSupplyPattern {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            bitcoin: Indexes3::new(client.clone(), &format!("{base_path}/bitcoin")),
            dollars: Indexes3::new(client.clone(), &format!("{base_path}/dollars")),
            sats: Indexes3::new(client.clone(), &format!("{base_path}/sats")),
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
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            bitcoin: BlockCountPattern::new(client.clone(), &format!("{base_path}/bitcoin")),
            dollars: BlockCountPattern::new(client.clone(), &format!("{base_path}/dollars")),
            sats: BlockCountPattern::new(client.clone(), &format!("{base_path}/sats")),
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
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            bitcoin: BitcoinPattern::new(client.clone(), &format!("{base_path}/bitcoin")),
            dollars: BitcoinPattern::new(client.clone(), &format!("{base_path}/dollars")),
            sats: BitcoinPattern::new(client.clone(), &format!("{base_path}/sats")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct PricePaidPattern2 {
    pub max_price_paid: Indexes3<Dollars>,
    pub min_price_paid: Indexes3<Dollars>,
    pub price_percentiles: PricePercentilesPattern,
}

impl PricePaidPattern2 {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            max_price_paid: Indexes3::new(client.clone(), &format!("{base_path}/max_price_paid")),
            min_price_paid: Indexes3::new(client.clone(), &format!("{base_path}/min_price_paid")),
            price_percentiles: PricePercentilesPattern::new(client.clone(), &format!("{base_path}/price_percentiles")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct BlockCountPattern<T> {
    pub base: Indexes2<T>,
    pub cumulative: Indexes3<T>,
    pub sum: Indexes4<T>,
}

impl<T: DeserializeOwned> BlockCountPattern<T> {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            base: Indexes2::new(client.clone(), &format!("{base_path}/base")),
            cumulative: Indexes3::new(client.clone(), &format!("{base_path}/cumulative")),
            sum: Indexes4::new(client.clone(), &format!("{base_path}/sum")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct SatsPattern {
    pub cumulative: Indexes3<Sats>,
    pub sum: Indexes4<Sats>,
}

impl SatsPattern {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            cumulative: Indexes3::new(client.clone(), &format!("{base_path}/cumulative")),
            sum: Indexes4::new(client.clone(), &format!("{base_path}/sum")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct SupplyValuePattern {
    pub bitcoin: Indexes2<Bitcoin>,
    pub dollars: Indexes2<Dollars>,
}

impl SupplyValuePattern {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            bitcoin: Indexes2::new(client.clone(), &format!("{base_path}/bitcoin")),
            dollars: Indexes2::new(client.clone(), &format!("{base_path}/dollars")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct _1dReturns1mSdPattern {
    pub sd: Indexes<StoredF32>,
    pub sma: Indexes<StoredF32>,
}

impl _1dReturns1mSdPattern {
    /// Create a new pattern node with accumulated metric name.
    pub fn new(client: Arc<BrkClientBase>, acc: &str) -> Self {
        Self {
            sd: Indexes::new(client.clone(), &format!("/{acc}_sd")),
            sma: Indexes::new(client.clone(), &format!("/{acc}_sma")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct PricePaidPattern {
    pub max_price_paid: Indexes3<Dollars>,
    pub min_price_paid: Indexes3<Dollars>,
}

impl PricePaidPattern {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            max_price_paid: Indexes3::new(client.clone(), &format!("{base_path}/max_price_paid")),
            min_price_paid: Indexes3::new(client.clone(), &format!("{base_path}/min_price_paid")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct BitcoinPattern2<T> {
    pub base: Indexes2<T>,
    pub sum: Indexes4<T>,
}

impl<T: DeserializeOwned> BitcoinPattern2<T> {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            base: Indexes2::new(client.clone(), &format!("{base_path}/base")),
            sum: Indexes4::new(client.clone(), &format!("{base_path}/sum")),
        }
    }
}

/// Pattern struct for repeated tree structure.
pub struct RealizedPriceExtraPattern {
    pub ratio: Indexes<StoredF32>,
}

impl RealizedPriceExtraPattern {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            ratio: Indexes::new(client.clone(), &format!("{base_path}/ratio")),
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
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            computed: CatalogTree_Computed::new(client.clone(), &format!("{base_path}/computed")),
            indexed: CatalogTree_Indexed::new(client.clone(), &format!("{base_path}/indexed")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed {
    pub blks: CatalogTree_Computed_Blks,
    pub chain: CatalogTree_Computed_Chain,
    pub cointime: CatalogTree_Computed_Cointime,
    pub constants: CatalogTree_Computed_Constants,
    pub fetched: CatalogTree_Computed_Fetched,
    pub indexes: CatalogTree_Computed_Indexes,
    pub market: CatalogTree_Computed_Market,
    pub pools: CatalogTree_Computed_Pools,
    pub price: CatalogTree_Computed_Price,
    pub stateful: CatalogTree_Computed_Stateful,
    pub txins: CatalogTree_Computed_Txins,
    pub txouts: CatalogTree_Computed_Txouts,
}

impl CatalogTree_Computed {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            blks: CatalogTree_Computed_Blks::new(client.clone(), &format!("{base_path}/blks")),
            chain: CatalogTree_Computed_Chain::new(client.clone(), &format!("{base_path}/chain")),
            cointime: CatalogTree_Computed_Cointime::new(client.clone(), &format!("{base_path}/cointime")),
            constants: CatalogTree_Computed_Constants::new(client.clone(), &format!("{base_path}/constants")),
            fetched: CatalogTree_Computed_Fetched::new(client.clone(), &format!("{base_path}/fetched")),
            indexes: CatalogTree_Computed_Indexes::new(client.clone(), &format!("{base_path}/indexes")),
            market: CatalogTree_Computed_Market::new(client.clone(), &format!("{base_path}/market")),
            pools: CatalogTree_Computed_Pools::new(client.clone(), &format!("{base_path}/pools")),
            price: CatalogTree_Computed_Price::new(client.clone(), &format!("{base_path}/price")),
            stateful: CatalogTree_Computed_Stateful::new(client.clone(), &format!("{base_path}/stateful")),
            txins: CatalogTree_Computed_Txins::new(client.clone(), &format!("{base_path}/txins")),
            txouts: CatalogTree_Computed_Txouts::new(client.clone(), &format!("{base_path}/txouts")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Blks {
    pub position: MetricNode<BlkPosition>,
}

impl CatalogTree_Computed_Blks {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            position: MetricNode::new(client.clone(), format!("{base_path}/position")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Chain {
    pub _1m_block_count: Indexes<StoredU32>,
    pub _1w_block_count: Indexes<StoredU32>,
    pub _1y_block_count: Indexes<StoredU32>,
    pub _24h_block_count: Indexes2<StoredU32>,
    pub _24h_coinbase_sum: Indexes2<Sats>,
    pub _24h_coinbase_usd_sum: Indexes2<Dollars>,
    pub annualized_volume: Indexes<Sats>,
    pub annualized_volume_btc: Indexes<Bitcoin>,
    pub annualized_volume_usd: Indexes<Dollars>,
    pub block_count: BlockCountPattern<StoredU32>,
    pub block_count_target: Indexes<StoredU64>,
    pub block_interval: BlockIntervalPattern<Timestamp>,
    pub block_size: BlockSizePattern<StoredU64>,
    pub block_vbytes: BlockSizePattern<StoredU64>,
    pub block_weight: BlockSizePattern<Weight>,
    pub blocks_before_next_difficulty_adjustment: Indexes3<StoredU32>,
    pub blocks_before_next_halving: Indexes3<StoredU32>,
    pub coinbase: CoinbasePattern,
    pub days_before_next_difficulty_adjustment: Indexes3<StoredF32>,
    pub days_before_next_halving: Indexes3<StoredF32>,
    pub difficulty: Indexes4<StoredF64>,
    pub difficulty_adjustment: Indexes3<StoredF32>,
    pub difficulty_as_hash: Indexes3<StoredF32>,
    pub difficultyepoch: Indexes<DifficultyEpoch>,
    pub emptyoutput_count: BitcoinPattern<StoredU64>,
    pub exact_utxo_count: Indexes3<StoredU64>,
    pub fee: CatalogTree_Computed_Chain_Fee,
    pub fee_dominance: Indexes5<StoredF32>,
    pub fee_rate: CatalogTree_Computed_Chain_FeeRate,
    pub halvingepoch: Indexes<HalvingEpoch>,
    pub hash_price_phs: Indexes3<StoredF32>,
    pub hash_price_phs_min: Indexes3<StoredF32>,
    pub hash_price_rebound: Indexes3<StoredF32>,
    pub hash_price_ths: Indexes3<StoredF32>,
    pub hash_price_ths_min: Indexes3<StoredF32>,
    pub hash_rate: Indexes3<StoredF64>,
    pub hash_rate_1m_sma: Indexes<StoredF32>,
    pub hash_rate_1w_sma: Indexes<StoredF64>,
    pub hash_rate_1y_sma: Indexes<StoredF32>,
    pub hash_rate_2m_sma: Indexes<StoredF32>,
    pub hash_value_phs: Indexes3<StoredF32>,
    pub hash_value_phs_min: Indexes3<StoredF32>,
    pub hash_value_rebound: Indexes3<StoredF32>,
    pub hash_value_ths: Indexes3<StoredF32>,
    pub hash_value_ths_min: Indexes3<StoredF32>,
    pub inflation_rate: Indexes<StoredF32>,
    pub input_count: BlockSizePattern<StoredU64>,
    pub input_value: Indexes6<Sats>,
    pub inputs_per_sec: Indexes<StoredF32>,
    pub interval: Indexes2<Timestamp>,
    pub is_coinbase: Indexes6<StoredBool>,
    pub opreturn_count: BitcoinPattern<StoredU64>,
    pub output_count: BlockSizePattern<StoredU64>,
    pub output_value: Indexes6<Sats>,
    pub outputs_per_sec: Indexes<StoredF32>,
    pub p2a_count: BitcoinPattern<StoredU64>,
    pub p2ms_count: BitcoinPattern<StoredU64>,
    pub p2pk33_count: BitcoinPattern<StoredU64>,
    pub p2pk65_count: BitcoinPattern<StoredU64>,
    pub p2pkh_count: BitcoinPattern<StoredU64>,
    pub p2sh_count: BitcoinPattern<StoredU64>,
    pub p2tr_count: BitcoinPattern<StoredU64>,
    pub p2wpkh_count: BitcoinPattern<StoredU64>,
    pub p2wsh_count: BitcoinPattern<StoredU64>,
    pub puell_multiple: Indexes<StoredF32>,
    pub sent_sum: CatalogTree_Computed_Chain_SentSum,
    pub subsidy: CoinbasePattern,
    pub subsidy_dominance: Indexes5<StoredF32>,
    pub subsidy_usd_1y_sma: Indexes<Dollars>,
    pub timestamp: MetricNode<Timestamp>,
    pub tx_btc_velocity: Indexes<StoredF64>,
    pub tx_count: BitcoinPattern<StoredU64>,
    pub tx_per_sec: Indexes<StoredF32>,
    pub tx_usd_velocity: Indexes<StoredF64>,
    pub tx_v1: BlockCountPattern<StoredU64>,
    pub tx_v2: BlockCountPattern<StoredU64>,
    pub tx_v3: BlockCountPattern<StoredU64>,
    pub tx_vsize: BlockIntervalPattern<VSize>,
    pub tx_weight: BlockIntervalPattern<Weight>,
    pub unclaimed_rewards: UnclaimedRewardsPattern,
    pub unknownoutput_count: BitcoinPattern<StoredU64>,
    pub vbytes: Indexes2<StoredU64>,
    pub vsize: Indexes6<VSize>,
    pub weight: Indexes6<Weight>,
}

impl CatalogTree_Computed_Chain {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            _1m_block_count: Indexes::new(client.clone(), &format!("{base_path}/1m_block_count")),
            _1w_block_count: Indexes::new(client.clone(), &format!("{base_path}/1w_block_count")),
            _1y_block_count: Indexes::new(client.clone(), &format!("{base_path}/1y_block_count")),
            _24h_block_count: Indexes2::new(client.clone(), &format!("{base_path}/24h_block_count")),
            _24h_coinbase_sum: Indexes2::new(client.clone(), &format!("{base_path}/24h_coinbase_sum")),
            _24h_coinbase_usd_sum: Indexes2::new(client.clone(), &format!("{base_path}/24h_coinbase_usd_sum")),
            annualized_volume: Indexes::new(client.clone(), &format!("{base_path}/annualized_volume")),
            annualized_volume_btc: Indexes::new(client.clone(), &format!("{base_path}/annualized_volume_btc")),
            annualized_volume_usd: Indexes::new(client.clone(), &format!("{base_path}/annualized_volume_usd")),
            block_count: BlockCountPattern::new(client.clone(), &format!("{base_path}/block_count")),
            block_count_target: Indexes::new(client.clone(), &format!("{base_path}/block_count_target")),
            block_interval: BlockIntervalPattern::new(client.clone(), "block_interval"),
            block_size: BlockSizePattern::new(client.clone(), &format!("{base_path}/block_size")),
            block_vbytes: BlockSizePattern::new(client.clone(), &format!("{base_path}/block_vbytes")),
            block_weight: BlockSizePattern::new(client.clone(), &format!("{base_path}/block_weight")),
            blocks_before_next_difficulty_adjustment: Indexes3::new(client.clone(), &format!("{base_path}/blocks_before_next_difficulty_adjustment")),
            blocks_before_next_halving: Indexes3::new(client.clone(), &format!("{base_path}/blocks_before_next_halving")),
            coinbase: CoinbasePattern::new(client.clone(), &format!("{base_path}/coinbase")),
            days_before_next_difficulty_adjustment: Indexes3::new(client.clone(), &format!("{base_path}/days_before_next_difficulty_adjustment")),
            days_before_next_halving: Indexes3::new(client.clone(), &format!("{base_path}/days_before_next_halving")),
            difficulty: Indexes4::new(client.clone(), &format!("{base_path}/difficulty")),
            difficulty_adjustment: Indexes3::new(client.clone(), &format!("{base_path}/difficulty_adjustment")),
            difficulty_as_hash: Indexes3::new(client.clone(), &format!("{base_path}/difficulty_as_hash")),
            difficultyepoch: Indexes::new(client.clone(), &format!("{base_path}/difficultyepoch")),
            emptyoutput_count: BitcoinPattern::new(client.clone(), &format!("{base_path}/emptyoutput_count")),
            exact_utxo_count: Indexes3::new(client.clone(), &format!("{base_path}/exact_utxo_count")),
            fee: CatalogTree_Computed_Chain_Fee::new(client.clone(), &format!("{base_path}/fee")),
            fee_dominance: Indexes5::new(client.clone(), &format!("{base_path}/fee_dominance")),
            fee_rate: CatalogTree_Computed_Chain_FeeRate::new(client.clone(), &format!("{base_path}/fee_rate")),
            halvingepoch: Indexes::new(client.clone(), &format!("{base_path}/halvingepoch")),
            hash_price_phs: Indexes3::new(client.clone(), &format!("{base_path}/hash_price_phs")),
            hash_price_phs_min: Indexes3::new(client.clone(), &format!("{base_path}/hash_price_phs_min")),
            hash_price_rebound: Indexes3::new(client.clone(), &format!("{base_path}/hash_price_rebound")),
            hash_price_ths: Indexes3::new(client.clone(), &format!("{base_path}/hash_price_ths")),
            hash_price_ths_min: Indexes3::new(client.clone(), &format!("{base_path}/hash_price_ths_min")),
            hash_rate: Indexes3::new(client.clone(), &format!("{base_path}/hash_rate")),
            hash_rate_1m_sma: Indexes::new(client.clone(), &format!("{base_path}/hash_rate_1m_sma")),
            hash_rate_1w_sma: Indexes::new(client.clone(), &format!("{base_path}/hash_rate_1w_sma")),
            hash_rate_1y_sma: Indexes::new(client.clone(), &format!("{base_path}/hash_rate_1y_sma")),
            hash_rate_2m_sma: Indexes::new(client.clone(), &format!("{base_path}/hash_rate_2m_sma")),
            hash_value_phs: Indexes3::new(client.clone(), &format!("{base_path}/hash_value_phs")),
            hash_value_phs_min: Indexes3::new(client.clone(), &format!("{base_path}/hash_value_phs_min")),
            hash_value_rebound: Indexes3::new(client.clone(), &format!("{base_path}/hash_value_rebound")),
            hash_value_ths: Indexes3::new(client.clone(), &format!("{base_path}/hash_value_ths")),
            hash_value_ths_min: Indexes3::new(client.clone(), &format!("{base_path}/hash_value_ths_min")),
            inflation_rate: Indexes::new(client.clone(), &format!("{base_path}/inflation_rate")),
            input_count: BlockSizePattern::new(client.clone(), &format!("{base_path}/input_count")),
            input_value: Indexes6::new(client.clone(), &format!("{base_path}/input_value")),
            inputs_per_sec: Indexes::new(client.clone(), &format!("{base_path}/inputs_per_sec")),
            interval: Indexes2::new(client.clone(), &format!("{base_path}/interval")),
            is_coinbase: Indexes6::new(client.clone(), &format!("{base_path}/is_coinbase")),
            opreturn_count: BitcoinPattern::new(client.clone(), &format!("{base_path}/opreturn_count")),
            output_count: BlockSizePattern::new(client.clone(), &format!("{base_path}/output_count")),
            output_value: Indexes6::new(client.clone(), &format!("{base_path}/output_value")),
            outputs_per_sec: Indexes::new(client.clone(), &format!("{base_path}/outputs_per_sec")),
            p2a_count: BitcoinPattern::new(client.clone(), &format!("{base_path}/p2a_count")),
            p2ms_count: BitcoinPattern::new(client.clone(), &format!("{base_path}/p2ms_count")),
            p2pk33_count: BitcoinPattern::new(client.clone(), &format!("{base_path}/p2pk33_count")),
            p2pk65_count: BitcoinPattern::new(client.clone(), &format!("{base_path}/p2pk65_count")),
            p2pkh_count: BitcoinPattern::new(client.clone(), &format!("{base_path}/p2pkh_count")),
            p2sh_count: BitcoinPattern::new(client.clone(), &format!("{base_path}/p2sh_count")),
            p2tr_count: BitcoinPattern::new(client.clone(), &format!("{base_path}/p2tr_count")),
            p2wpkh_count: BitcoinPattern::new(client.clone(), &format!("{base_path}/p2wpkh_count")),
            p2wsh_count: BitcoinPattern::new(client.clone(), &format!("{base_path}/p2wsh_count")),
            puell_multiple: Indexes::new(client.clone(), &format!("{base_path}/puell_multiple")),
            sent_sum: CatalogTree_Computed_Chain_SentSum::new(client.clone(), &format!("{base_path}/sent_sum")),
            subsidy: CoinbasePattern::new(client.clone(), &format!("{base_path}/subsidy")),
            subsidy_dominance: Indexes5::new(client.clone(), &format!("{base_path}/subsidy_dominance")),
            subsidy_usd_1y_sma: Indexes::new(client.clone(), &format!("{base_path}/subsidy_usd_1y_sma")),
            timestamp: MetricNode::new(client.clone(), format!("{base_path}/timestamp")),
            tx_btc_velocity: Indexes::new(client.clone(), &format!("{base_path}/tx_btc_velocity")),
            tx_count: BitcoinPattern::new(client.clone(), &format!("{base_path}/tx_count")),
            tx_per_sec: Indexes::new(client.clone(), &format!("{base_path}/tx_per_sec")),
            tx_usd_velocity: Indexes::new(client.clone(), &format!("{base_path}/tx_usd_velocity")),
            tx_v1: BlockCountPattern::new(client.clone(), &format!("{base_path}/tx_v1")),
            tx_v2: BlockCountPattern::new(client.clone(), &format!("{base_path}/tx_v2")),
            tx_v3: BlockCountPattern::new(client.clone(), &format!("{base_path}/tx_v3")),
            tx_vsize: BlockIntervalPattern::new(client.clone(), "tx_vsize"),
            tx_weight: BlockIntervalPattern::new(client.clone(), "tx_weight"),
            unclaimed_rewards: UnclaimedRewardsPattern::new(client.clone(), &format!("{base_path}/unclaimed_rewards")),
            unknownoutput_count: BitcoinPattern::new(client.clone(), &format!("{base_path}/unknownoutput_count")),
            vbytes: Indexes2::new(client.clone(), &format!("{base_path}/vbytes")),
            vsize: Indexes6::new(client.clone(), &format!("{base_path}/vsize")),
            weight: Indexes6::new(client.clone(), &format!("{base_path}/weight")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Chain_Fee {
    pub base: Indexes6<Sats>,
    pub bitcoin: BlockSizePattern<Bitcoin>,
    pub bitcoin_txindex: Indexes6<Bitcoin>,
    pub dollars: BlockSizePattern<Dollars>,
    pub dollars_txindex: Indexes6<Dollars>,
    pub sats: BlockSizePattern<Sats>,
}

impl CatalogTree_Computed_Chain_Fee {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            base: Indexes6::new(client.clone(), &format!("{base_path}/base")),
            bitcoin: BlockSizePattern::new(client.clone(), &format!("{base_path}/bitcoin")),
            bitcoin_txindex: Indexes6::new(client.clone(), &format!("{base_path}/bitcoin_txindex")),
            dollars: BlockSizePattern::new(client.clone(), &format!("{base_path}/dollars")),
            dollars_txindex: Indexes6::new(client.clone(), &format!("{base_path}/dollars_txindex")),
            sats: BlockSizePattern::new(client.clone(), &format!("{base_path}/sats")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Chain_FeeRate {
    pub average: Indexes3<FeeRate>,
    pub base: Indexes6<FeeRate>,
    pub max: Indexes3<FeeRate>,
    pub median: Indexes2<FeeRate>,
    pub min: Indexes3<FeeRate>,
    pub pct10: Indexes2<FeeRate>,
    pub pct25: Indexes2<FeeRate>,
    pub pct75: Indexes2<FeeRate>,
    pub pct90: Indexes2<FeeRate>,
}

impl CatalogTree_Computed_Chain_FeeRate {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            average: Indexes3::new(client.clone(), &format!("{base_path}/average")),
            base: Indexes6::new(client.clone(), &format!("{base_path}/base")),
            max: Indexes3::new(client.clone(), &format!("{base_path}/max")),
            median: Indexes2::new(client.clone(), &format!("{base_path}/median")),
            min: Indexes3::new(client.clone(), &format!("{base_path}/min")),
            pct10: Indexes2::new(client.clone(), &format!("{base_path}/pct10")),
            pct25: Indexes2::new(client.clone(), &format!("{base_path}/pct25")),
            pct75: Indexes2::new(client.clone(), &format!("{base_path}/pct75")),
            pct90: Indexes2::new(client.clone(), &format!("{base_path}/pct90")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Chain_SentSum {
    pub bitcoin: BitcoinPattern2<Bitcoin>,
    pub dollars: Indexes3<Dollars>,
    pub sats: Indexes3<Sats>,
}

impl CatalogTree_Computed_Chain_SentSum {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            bitcoin: BitcoinPattern2::new(client.clone(), &format!("{base_path}/bitcoin")),
            dollars: Indexes3::new(client.clone(), &format!("{base_path}/dollars")),
            sats: Indexes3::new(client.clone(), &format!("{base_path}/sats")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Cointime {
    pub active_cap: Indexes3<Dollars>,
    pub active_price: Indexes3<Dollars>,
    pub active_price_ratio: ActivePriceRatioPattern,
    pub active_supply: ActiveSupplyPattern,
    pub activity_to_vaultedness_ratio: Indexes3<StoredF64>,
    pub coinblocks_created: BlockCountPattern<StoredF64>,
    pub coinblocks_stored: BlockCountPattern<StoredF64>,
    pub cointime_adj_inflation_rate: Indexes<StoredF32>,
    pub cointime_adj_tx_btc_velocity: Indexes<StoredF64>,
    pub cointime_adj_tx_usd_velocity: Indexes<StoredF64>,
    pub cointime_cap: Indexes3<Dollars>,
    pub cointime_price: Indexes3<Dollars>,
    pub cointime_price_ratio: ActivePriceRatioPattern,
    pub cointime_value_created: BlockCountPattern<StoredF64>,
    pub cointime_value_destroyed: BlockCountPattern<StoredF64>,
    pub cointime_value_stored: BlockCountPattern<StoredF64>,
    pub investor_cap: Indexes3<Dollars>,
    pub liveliness: Indexes3<StoredF64>,
    pub thermo_cap: Indexes3<Dollars>,
    pub true_market_mean: Indexes3<Dollars>,
    pub true_market_mean_ratio: ActivePriceRatioPattern,
    pub vaulted_cap: Indexes3<Dollars>,
    pub vaulted_price: Indexes3<Dollars>,
    pub vaulted_price_ratio: ActivePriceRatioPattern,
    pub vaulted_supply: ActiveSupplyPattern,
    pub vaultedness: Indexes3<StoredF64>,
}

impl CatalogTree_Computed_Cointime {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            active_cap: Indexes3::new(client.clone(), &format!("{base_path}/active_cap")),
            active_price: Indexes3::new(client.clone(), &format!("{base_path}/active_price")),
            active_price_ratio: ActivePriceRatioPattern::new(client.clone(), &format!("{base_path}/active_price_ratio")),
            active_supply: ActiveSupplyPattern::new(client.clone(), &format!("{base_path}/active_supply")),
            activity_to_vaultedness_ratio: Indexes3::new(client.clone(), &format!("{base_path}/activity_to_vaultedness_ratio")),
            coinblocks_created: BlockCountPattern::new(client.clone(), &format!("{base_path}/coinblocks_created")),
            coinblocks_stored: BlockCountPattern::new(client.clone(), &format!("{base_path}/coinblocks_stored")),
            cointime_adj_inflation_rate: Indexes::new(client.clone(), &format!("{base_path}/cointime_adj_inflation_rate")),
            cointime_adj_tx_btc_velocity: Indexes::new(client.clone(), &format!("{base_path}/cointime_adj_tx_btc_velocity")),
            cointime_adj_tx_usd_velocity: Indexes::new(client.clone(), &format!("{base_path}/cointime_adj_tx_usd_velocity")),
            cointime_cap: Indexes3::new(client.clone(), &format!("{base_path}/cointime_cap")),
            cointime_price: Indexes3::new(client.clone(), &format!("{base_path}/cointime_price")),
            cointime_price_ratio: ActivePriceRatioPattern::new(client.clone(), &format!("{base_path}/cointime_price_ratio")),
            cointime_value_created: BlockCountPattern::new(client.clone(), &format!("{base_path}/cointime_value_created")),
            cointime_value_destroyed: BlockCountPattern::new(client.clone(), &format!("{base_path}/cointime_value_destroyed")),
            cointime_value_stored: BlockCountPattern::new(client.clone(), &format!("{base_path}/cointime_value_stored")),
            investor_cap: Indexes3::new(client.clone(), &format!("{base_path}/investor_cap")),
            liveliness: Indexes3::new(client.clone(), &format!("{base_path}/liveliness")),
            thermo_cap: Indexes3::new(client.clone(), &format!("{base_path}/thermo_cap")),
            true_market_mean: Indexes3::new(client.clone(), &format!("{base_path}/true_market_mean")),
            true_market_mean_ratio: ActivePriceRatioPattern::new(client.clone(), &format!("{base_path}/true_market_mean_ratio")),
            vaulted_cap: Indexes3::new(client.clone(), &format!("{base_path}/vaulted_cap")),
            vaulted_price: Indexes3::new(client.clone(), &format!("{base_path}/vaulted_price")),
            vaulted_price_ratio: ActivePriceRatioPattern::new(client.clone(), &format!("{base_path}/vaulted_price_ratio")),
            vaulted_supply: ActiveSupplyPattern::new(client.clone(), &format!("{base_path}/vaulted_supply")),
            vaultedness: Indexes3::new(client.clone(), &format!("{base_path}/vaultedness")),
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
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            constant_0: Constant0Pattern::new(client.clone(), "constant_0"),
            constant_1: Constant0Pattern::new(client.clone(), "constant_1"),
            constant_100: Constant0Pattern::new(client.clone(), "constant_100"),
            constant_2: Constant0Pattern::new(client.clone(), "constant_2"),
            constant_3: Constant0Pattern::new(client.clone(), "constant_3"),
            constant_38_2: Constant0Pattern::new(client.clone(), "constant_38_2"),
            constant_4: Constant0Pattern::new(client.clone(), "constant_4"),
            constant_50: Constant0Pattern::new(client.clone(), "constant_50"),
            constant_600: Constant0Pattern::new(client.clone(), "constant_600"),
            constant_61_8: Constant0Pattern::new(client.clone(), "constant_61_8"),
            constant_minus_1: Constant0Pattern::new(client.clone(), "constant_minus_1"),
            constant_minus_2: Constant0Pattern::new(client.clone(), "constant_minus_2"),
            constant_minus_3: Constant0Pattern::new(client.clone(), "constant_minus_3"),
            constant_minus_4: Constant0Pattern::new(client.clone(), "constant_minus_4"),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Fetched {
    pub price_ohlc_in_cents: Indexes13<OHLCCents>,
}

impl CatalogTree_Computed_Fetched {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            price_ohlc_in_cents: Indexes13::new(client.clone(), &format!("{base_path}/price_ohlc_in_cents")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Indexes {
    pub date: Indexes13<Date>,
    pub date_fixed: Indexes2<Date>,
    pub dateindex: Indexes13<DateIndex>,
    pub dateindex_count: Indexes14<StoredU64>,
    pub decadeindex: MetricNode<DecadeIndex>,
    pub difficultyepoch: MetricNode<DifficultyEpoch>,
    pub emptyoutputindex: MetricNode<EmptyOutputIndex>,
    pub first_dateindex: Indexes14<DateIndex>,
    pub first_height: MetricNode<Height>,
    pub first_monthindex: Indexes15<MonthIndex>,
    pub first_yearindex: Indexes7<YearIndex>,
    pub halvingepoch: MetricNode<HalvingEpoch>,
    pub height: Indexes2<Height>,
    pub height_count: MetricNode<StoredU64>,
    pub input_count: Indexes6<StoredU64>,
    pub monthindex: MetricNode<MonthIndex>,
    pub monthindex_count: Indexes15<StoredU64>,
    pub opreturnindex: MetricNode<OpReturnIndex>,
    pub output_count: Indexes6<StoredU64>,
    pub p2aaddressindex: Indexes16<P2AAddressIndex>,
    pub p2msoutputindex: MetricNode<P2MSOutputIndex>,
    pub p2pk33addressindex: Indexes17<P2PK33AddressIndex>,
    pub p2pk65addressindex: Indexes18<P2PK65AddressIndex>,
    pub p2pkhaddressindex: Indexes19<P2PKHAddressIndex>,
    pub p2shaddressindex: Indexes20<P2SHAddressIndex>,
    pub p2traddressindex: Indexes21<P2TRAddressIndex>,
    pub p2wpkhaddressindex: Indexes22<P2WPKHAddressIndex>,
    pub p2wshaddressindex: Indexes23<P2WSHAddressIndex>,
    pub quarterindex: MetricNode<QuarterIndex>,
    pub semesterindex: MetricNode<SemesterIndex>,
    pub timestamp_fixed: Indexes2<Timestamp>,
    pub txindex: Indexes6<TxIndex>,
    pub txindex_count: Indexes2<StoredU64>,
    pub txinindex: Indexes24<TxInIndex>,
    pub txoutindex: Indexes25<TxOutIndex>,
    pub unknownoutputindex: MetricNode<UnknownOutputIndex>,
    pub weekindex: MetricNode<WeekIndex>,
    pub yearindex: MetricNode<YearIndex>,
    pub yearindex_count: Indexes7<StoredU64>,
}

impl CatalogTree_Computed_Indexes {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            date: Indexes13::new(client.clone(), &format!("{base_path}/date")),
            date_fixed: Indexes2::new(client.clone(), &format!("{base_path}/date_fixed")),
            dateindex: Indexes13::new(client.clone(), &format!("{base_path}/dateindex")),
            dateindex_count: Indexes14::new(client.clone(), &format!("{base_path}/dateindex_count")),
            decadeindex: MetricNode::new(client.clone(), format!("{base_path}/decadeindex")),
            difficultyepoch: MetricNode::new(client.clone(), format!("{base_path}/difficultyepoch")),
            emptyoutputindex: MetricNode::new(client.clone(), format!("{base_path}/emptyoutputindex")),
            first_dateindex: Indexes14::new(client.clone(), &format!("{base_path}/first_dateindex")),
            first_height: MetricNode::new(client.clone(), format!("{base_path}/first_height")),
            first_monthindex: Indexes15::new(client.clone(), &format!("{base_path}/first_monthindex")),
            first_yearindex: Indexes7::new(client.clone(), &format!("{base_path}/first_yearindex")),
            halvingepoch: MetricNode::new(client.clone(), format!("{base_path}/halvingepoch")),
            height: Indexes2::new(client.clone(), &format!("{base_path}/height")),
            height_count: MetricNode::new(client.clone(), format!("{base_path}/height_count")),
            input_count: Indexes6::new(client.clone(), &format!("{base_path}/input_count")),
            monthindex: MetricNode::new(client.clone(), format!("{base_path}/monthindex")),
            monthindex_count: Indexes15::new(client.clone(), &format!("{base_path}/monthindex_count")),
            opreturnindex: MetricNode::new(client.clone(), format!("{base_path}/opreturnindex")),
            output_count: Indexes6::new(client.clone(), &format!("{base_path}/output_count")),
            p2aaddressindex: Indexes16::new(client.clone(), &format!("{base_path}/p2aaddressindex")),
            p2msoutputindex: MetricNode::new(client.clone(), format!("{base_path}/p2msoutputindex")),
            p2pk33addressindex: Indexes17::new(client.clone(), &format!("{base_path}/p2pk33addressindex")),
            p2pk65addressindex: Indexes18::new(client.clone(), &format!("{base_path}/p2pk65addressindex")),
            p2pkhaddressindex: Indexes19::new(client.clone(), &format!("{base_path}/p2pkhaddressindex")),
            p2shaddressindex: Indexes20::new(client.clone(), &format!("{base_path}/p2shaddressindex")),
            p2traddressindex: Indexes21::new(client.clone(), &format!("{base_path}/p2traddressindex")),
            p2wpkhaddressindex: Indexes22::new(client.clone(), &format!("{base_path}/p2wpkhaddressindex")),
            p2wshaddressindex: Indexes23::new(client.clone(), &format!("{base_path}/p2wshaddressindex")),
            quarterindex: MetricNode::new(client.clone(), format!("{base_path}/quarterindex")),
            semesterindex: MetricNode::new(client.clone(), format!("{base_path}/semesterindex")),
            timestamp_fixed: Indexes2::new(client.clone(), &format!("{base_path}/timestamp_fixed")),
            txindex: Indexes6::new(client.clone(), &format!("{base_path}/txindex")),
            txindex_count: Indexes2::new(client.clone(), &format!("{base_path}/txindex_count")),
            txinindex: Indexes24::new(client.clone(), &format!("{base_path}/txinindex")),
            txoutindex: Indexes25::new(client.clone(), &format!("{base_path}/txoutindex")),
            unknownoutputindex: MetricNode::new(client.clone(), format!("{base_path}/unknownoutputindex")),
            weekindex: MetricNode::new(client.clone(), format!("{base_path}/weekindex")),
            yearindex: MetricNode::new(client.clone(), format!("{base_path}/yearindex")),
            yearindex_count: Indexes7::new(client.clone(), &format!("{base_path}/yearindex_count")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Market {
    pub _1d_returns_1m_sd: _1dReturns1mSdPattern,
    pub _1d_returns_1w_sd: _1dReturns1mSdPattern,
    pub _1d_returns_1y_sd: _1dReturns1mSdPattern,
    pub _10y_cagr: Indexes<StoredF32>,
    pub _10y_dca_avg_price: Indexes<Dollars>,
    pub _10y_dca_cagr: Indexes<StoredF32>,
    pub _10y_dca_returns: Indexes<StoredF32>,
    pub _10y_dca_stack: Indexes<Sats>,
    pub _10y_price_returns: Indexes<StoredF32>,
    pub _1d_price_returns: Indexes<StoredF32>,
    pub _1m_dca_avg_price: Indexes<Dollars>,
    pub _1m_dca_returns: Indexes<StoredF32>,
    pub _1m_dca_stack: Indexes<Sats>,
    pub _1m_price_returns: Indexes<StoredF32>,
    pub _1w_dca_avg_price: Indexes<Dollars>,
    pub _1w_dca_returns: Indexes<StoredF32>,
    pub _1w_dca_stack: Indexes<Sats>,
    pub _1w_price_returns: Indexes<StoredF32>,
    pub _1y_dca_avg_price: Indexes<Dollars>,
    pub _1y_dca_returns: Indexes<StoredF32>,
    pub _1y_dca_stack: Indexes<Sats>,
    pub _1y_price_returns: Indexes<StoredF32>,
    pub _2y_cagr: Indexes<StoredF32>,
    pub _2y_dca_avg_price: Indexes<Dollars>,
    pub _2y_dca_cagr: Indexes<StoredF32>,
    pub _2y_dca_returns: Indexes<StoredF32>,
    pub _2y_dca_stack: Indexes<Sats>,
    pub _2y_price_returns: Indexes<StoredF32>,
    pub _3m_dca_avg_price: Indexes<Dollars>,
    pub _3m_dca_returns: Indexes<StoredF32>,
    pub _3m_dca_stack: Indexes<Sats>,
    pub _3m_price_returns: Indexes<StoredF32>,
    pub _3y_cagr: Indexes<StoredF32>,
    pub _3y_dca_avg_price: Indexes<Dollars>,
    pub _3y_dca_cagr: Indexes<StoredF32>,
    pub _3y_dca_returns: Indexes<StoredF32>,
    pub _3y_dca_stack: Indexes<Sats>,
    pub _3y_price_returns: Indexes<StoredF32>,
    pub _4y_cagr: Indexes<StoredF32>,
    pub _4y_dca_avg_price: Indexes<Dollars>,
    pub _4y_dca_cagr: Indexes<StoredF32>,
    pub _4y_dca_returns: Indexes<StoredF32>,
    pub _4y_dca_stack: Indexes<Sats>,
    pub _4y_price_returns: Indexes<StoredF32>,
    pub _5y_cagr: Indexes<StoredF32>,
    pub _5y_dca_avg_price: Indexes<Dollars>,
    pub _5y_dca_cagr: Indexes<StoredF32>,
    pub _5y_dca_returns: Indexes<StoredF32>,
    pub _5y_dca_stack: Indexes<Sats>,
    pub _5y_price_returns: Indexes<StoredF32>,
    pub _6m_dca_avg_price: Indexes<Dollars>,
    pub _6m_dca_returns: Indexes<StoredF32>,
    pub _6m_dca_stack: Indexes<Sats>,
    pub _6m_price_returns: Indexes<StoredF32>,
    pub _6y_cagr: Indexes<StoredF32>,
    pub _6y_dca_avg_price: Indexes<Dollars>,
    pub _6y_dca_cagr: Indexes<StoredF32>,
    pub _6y_dca_returns: Indexes<StoredF32>,
    pub _6y_dca_stack: Indexes<Sats>,
    pub _6y_price_returns: Indexes<StoredF32>,
    pub _8y_cagr: Indexes<StoredF32>,
    pub _8y_dca_avg_price: Indexes<Dollars>,
    pub _8y_dca_cagr: Indexes<StoredF32>,
    pub _8y_dca_returns: Indexes<StoredF32>,
    pub _8y_dca_stack: Indexes<Sats>,
    pub _8y_price_returns: Indexes<StoredF32>,
    pub days_since_price_ath: Indexes<StoredU16>,
    pub dca_class_2015_avg_price: Indexes<Dollars>,
    pub dca_class_2015_returns: Indexes<StoredF32>,
    pub dca_class_2015_stack: Indexes<Sats>,
    pub dca_class_2016_avg_price: Indexes<Dollars>,
    pub dca_class_2016_returns: Indexes<StoredF32>,
    pub dca_class_2016_stack: Indexes<Sats>,
    pub dca_class_2017_avg_price: Indexes<Dollars>,
    pub dca_class_2017_returns: Indexes<StoredF32>,
    pub dca_class_2017_stack: Indexes<Sats>,
    pub dca_class_2018_avg_price: Indexes<Dollars>,
    pub dca_class_2018_returns: Indexes<StoredF32>,
    pub dca_class_2018_stack: Indexes<Sats>,
    pub dca_class_2019_avg_price: Indexes<Dollars>,
    pub dca_class_2019_returns: Indexes<StoredF32>,
    pub dca_class_2019_stack: Indexes<Sats>,
    pub dca_class_2020_avg_price: Indexes<Dollars>,
    pub dca_class_2020_returns: Indexes<StoredF32>,
    pub dca_class_2020_stack: Indexes<Sats>,
    pub dca_class_2021_avg_price: Indexes<Dollars>,
    pub dca_class_2021_returns: Indexes<StoredF32>,
    pub dca_class_2021_stack: Indexes<Sats>,
    pub dca_class_2022_avg_price: Indexes<Dollars>,
    pub dca_class_2022_returns: Indexes<StoredF32>,
    pub dca_class_2022_stack: Indexes<Sats>,
    pub dca_class_2023_avg_price: Indexes<Dollars>,
    pub dca_class_2023_returns: Indexes<StoredF32>,
    pub dca_class_2023_stack: Indexes<Sats>,
    pub dca_class_2024_avg_price: Indexes<Dollars>,
    pub dca_class_2024_returns: Indexes<StoredF32>,
    pub dca_class_2024_stack: Indexes<Sats>,
    pub dca_class_2025_avg_price: Indexes<Dollars>,
    pub dca_class_2025_returns: Indexes<StoredF32>,
    pub dca_class_2025_stack: Indexes<Sats>,
    pub max_days_between_price_aths: Indexes<StoredU16>,
    pub max_years_between_price_aths: Indexes<StoredF32>,
    pub price_10y_ago: Indexes<Dollars>,
    pub price_13d_ema: Price13dEmaPattern,
    pub price_13d_sma: Price13dEmaPattern,
    pub price_144d_ema: Price13dEmaPattern,
    pub price_144d_sma: Price13dEmaPattern,
    pub price_1d_ago: Indexes<Dollars>,
    pub price_1m_ago: Indexes<Dollars>,
    pub price_1m_ema: Price13dEmaPattern,
    pub price_1m_max: Indexes<Dollars>,
    pub price_1m_min: Indexes<Dollars>,
    pub price_1m_sma: Price13dEmaPattern,
    pub price_1m_volatility: Indexes<StoredF32>,
    pub price_1w_ago: Indexes<Dollars>,
    pub price_1w_ema: Price13dEmaPattern,
    pub price_1w_max: Indexes<Dollars>,
    pub price_1w_min: Indexes<Dollars>,
    pub price_1w_sma: Price13dEmaPattern,
    pub price_1w_volatility: Indexes<StoredF32>,
    pub price_1y_ago: Indexes<Dollars>,
    pub price_1y_ema: Price13dEmaPattern,
    pub price_1y_max: Indexes<Dollars>,
    pub price_1y_min: Indexes<Dollars>,
    pub price_1y_sma: Price13dEmaPattern,
    pub price_1y_volatility: Indexes<StoredF32>,
    pub price_200d_ema: Price13dEmaPattern,
    pub price_200d_sma: Price13dEmaPattern,
    pub price_200d_sma_x0_8: Indexes<Dollars>,
    pub price_200d_sma_x2_4: Indexes<Dollars>,
    pub price_200w_ema: Price13dEmaPattern,
    pub price_200w_sma: Price13dEmaPattern,
    pub price_21d_ema: Price13dEmaPattern,
    pub price_21d_sma: Price13dEmaPattern,
    pub price_2w_choppiness_index: Indexes<StoredF32>,
    pub price_2w_max: Indexes<Dollars>,
    pub price_2w_min: Indexes<Dollars>,
    pub price_2y_ago: Indexes<Dollars>,
    pub price_2y_ema: Price13dEmaPattern,
    pub price_2y_sma: Price13dEmaPattern,
    pub price_34d_ema: Price13dEmaPattern,
    pub price_34d_sma: Price13dEmaPattern,
    pub price_3m_ago: Indexes<Dollars>,
    pub price_3y_ago: Indexes<Dollars>,
    pub price_4y_ago: Indexes<Dollars>,
    pub price_4y_ema: Price13dEmaPattern,
    pub price_4y_sma: Price13dEmaPattern,
    pub price_55d_ema: Price13dEmaPattern,
    pub price_55d_sma: Price13dEmaPattern,
    pub price_5y_ago: Indexes<Dollars>,
    pub price_6m_ago: Indexes<Dollars>,
    pub price_6y_ago: Indexes<Dollars>,
    pub price_89d_ema: Price13dEmaPattern,
    pub price_89d_sma: Price13dEmaPattern,
    pub price_8d_ema: Price13dEmaPattern,
    pub price_8d_sma: Price13dEmaPattern,
    pub price_8y_ago: Indexes<Dollars>,
    pub price_ath: Indexes26<Dollars>,
    pub price_drawdown: Indexes26<StoredF32>,
    pub price_true_range: Indexes5<StoredF32>,
    pub price_true_range_2w_sum: Indexes5<StoredF32>,
}

impl CatalogTree_Computed_Market {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            _1d_returns_1m_sd: _1dReturns1mSdPattern::new(client.clone(), "1d_returns_1m_sd"),
            _1d_returns_1w_sd: _1dReturns1mSdPattern::new(client.clone(), "1d_returns_1w_sd"),
            _1d_returns_1y_sd: _1dReturns1mSdPattern::new(client.clone(), "1d_returns_1y_sd"),
            _10y_cagr: Indexes::new(client.clone(), &format!("{base_path}/_10y_cagr")),
            _10y_dca_avg_price: Indexes::new(client.clone(), &format!("{base_path}/_10y_dca_avg_price")),
            _10y_dca_cagr: Indexes::new(client.clone(), &format!("{base_path}/_10y_dca_cagr")),
            _10y_dca_returns: Indexes::new(client.clone(), &format!("{base_path}/_10y_dca_returns")),
            _10y_dca_stack: Indexes::new(client.clone(), &format!("{base_path}/_10y_dca_stack")),
            _10y_price_returns: Indexes::new(client.clone(), &format!("{base_path}/_10y_price_returns")),
            _1d_price_returns: Indexes::new(client.clone(), &format!("{base_path}/_1d_price_returns")),
            _1m_dca_avg_price: Indexes::new(client.clone(), &format!("{base_path}/_1m_dca_avg_price")),
            _1m_dca_returns: Indexes::new(client.clone(), &format!("{base_path}/_1m_dca_returns")),
            _1m_dca_stack: Indexes::new(client.clone(), &format!("{base_path}/_1m_dca_stack")),
            _1m_price_returns: Indexes::new(client.clone(), &format!("{base_path}/_1m_price_returns")),
            _1w_dca_avg_price: Indexes::new(client.clone(), &format!("{base_path}/_1w_dca_avg_price")),
            _1w_dca_returns: Indexes::new(client.clone(), &format!("{base_path}/_1w_dca_returns")),
            _1w_dca_stack: Indexes::new(client.clone(), &format!("{base_path}/_1w_dca_stack")),
            _1w_price_returns: Indexes::new(client.clone(), &format!("{base_path}/_1w_price_returns")),
            _1y_dca_avg_price: Indexes::new(client.clone(), &format!("{base_path}/_1y_dca_avg_price")),
            _1y_dca_returns: Indexes::new(client.clone(), &format!("{base_path}/_1y_dca_returns")),
            _1y_dca_stack: Indexes::new(client.clone(), &format!("{base_path}/_1y_dca_stack")),
            _1y_price_returns: Indexes::new(client.clone(), &format!("{base_path}/_1y_price_returns")),
            _2y_cagr: Indexes::new(client.clone(), &format!("{base_path}/_2y_cagr")),
            _2y_dca_avg_price: Indexes::new(client.clone(), &format!("{base_path}/_2y_dca_avg_price")),
            _2y_dca_cagr: Indexes::new(client.clone(), &format!("{base_path}/_2y_dca_cagr")),
            _2y_dca_returns: Indexes::new(client.clone(), &format!("{base_path}/_2y_dca_returns")),
            _2y_dca_stack: Indexes::new(client.clone(), &format!("{base_path}/_2y_dca_stack")),
            _2y_price_returns: Indexes::new(client.clone(), &format!("{base_path}/_2y_price_returns")),
            _3m_dca_avg_price: Indexes::new(client.clone(), &format!("{base_path}/_3m_dca_avg_price")),
            _3m_dca_returns: Indexes::new(client.clone(), &format!("{base_path}/_3m_dca_returns")),
            _3m_dca_stack: Indexes::new(client.clone(), &format!("{base_path}/_3m_dca_stack")),
            _3m_price_returns: Indexes::new(client.clone(), &format!("{base_path}/_3m_price_returns")),
            _3y_cagr: Indexes::new(client.clone(), &format!("{base_path}/_3y_cagr")),
            _3y_dca_avg_price: Indexes::new(client.clone(), &format!("{base_path}/_3y_dca_avg_price")),
            _3y_dca_cagr: Indexes::new(client.clone(), &format!("{base_path}/_3y_dca_cagr")),
            _3y_dca_returns: Indexes::new(client.clone(), &format!("{base_path}/_3y_dca_returns")),
            _3y_dca_stack: Indexes::new(client.clone(), &format!("{base_path}/_3y_dca_stack")),
            _3y_price_returns: Indexes::new(client.clone(), &format!("{base_path}/_3y_price_returns")),
            _4y_cagr: Indexes::new(client.clone(), &format!("{base_path}/_4y_cagr")),
            _4y_dca_avg_price: Indexes::new(client.clone(), &format!("{base_path}/_4y_dca_avg_price")),
            _4y_dca_cagr: Indexes::new(client.clone(), &format!("{base_path}/_4y_dca_cagr")),
            _4y_dca_returns: Indexes::new(client.clone(), &format!("{base_path}/_4y_dca_returns")),
            _4y_dca_stack: Indexes::new(client.clone(), &format!("{base_path}/_4y_dca_stack")),
            _4y_price_returns: Indexes::new(client.clone(), &format!("{base_path}/_4y_price_returns")),
            _5y_cagr: Indexes::new(client.clone(), &format!("{base_path}/_5y_cagr")),
            _5y_dca_avg_price: Indexes::new(client.clone(), &format!("{base_path}/_5y_dca_avg_price")),
            _5y_dca_cagr: Indexes::new(client.clone(), &format!("{base_path}/_5y_dca_cagr")),
            _5y_dca_returns: Indexes::new(client.clone(), &format!("{base_path}/_5y_dca_returns")),
            _5y_dca_stack: Indexes::new(client.clone(), &format!("{base_path}/_5y_dca_stack")),
            _5y_price_returns: Indexes::new(client.clone(), &format!("{base_path}/_5y_price_returns")),
            _6m_dca_avg_price: Indexes::new(client.clone(), &format!("{base_path}/_6m_dca_avg_price")),
            _6m_dca_returns: Indexes::new(client.clone(), &format!("{base_path}/_6m_dca_returns")),
            _6m_dca_stack: Indexes::new(client.clone(), &format!("{base_path}/_6m_dca_stack")),
            _6m_price_returns: Indexes::new(client.clone(), &format!("{base_path}/_6m_price_returns")),
            _6y_cagr: Indexes::new(client.clone(), &format!("{base_path}/_6y_cagr")),
            _6y_dca_avg_price: Indexes::new(client.clone(), &format!("{base_path}/_6y_dca_avg_price")),
            _6y_dca_cagr: Indexes::new(client.clone(), &format!("{base_path}/_6y_dca_cagr")),
            _6y_dca_returns: Indexes::new(client.clone(), &format!("{base_path}/_6y_dca_returns")),
            _6y_dca_stack: Indexes::new(client.clone(), &format!("{base_path}/_6y_dca_stack")),
            _6y_price_returns: Indexes::new(client.clone(), &format!("{base_path}/_6y_price_returns")),
            _8y_cagr: Indexes::new(client.clone(), &format!("{base_path}/_8y_cagr")),
            _8y_dca_avg_price: Indexes::new(client.clone(), &format!("{base_path}/_8y_dca_avg_price")),
            _8y_dca_cagr: Indexes::new(client.clone(), &format!("{base_path}/_8y_dca_cagr")),
            _8y_dca_returns: Indexes::new(client.clone(), &format!("{base_path}/_8y_dca_returns")),
            _8y_dca_stack: Indexes::new(client.clone(), &format!("{base_path}/_8y_dca_stack")),
            _8y_price_returns: Indexes::new(client.clone(), &format!("{base_path}/_8y_price_returns")),
            days_since_price_ath: Indexes::new(client.clone(), &format!("{base_path}/days_since_price_ath")),
            dca_class_2015_avg_price: Indexes::new(client.clone(), &format!("{base_path}/dca_class_2015_avg_price")),
            dca_class_2015_returns: Indexes::new(client.clone(), &format!("{base_path}/dca_class_2015_returns")),
            dca_class_2015_stack: Indexes::new(client.clone(), &format!("{base_path}/dca_class_2015_stack")),
            dca_class_2016_avg_price: Indexes::new(client.clone(), &format!("{base_path}/dca_class_2016_avg_price")),
            dca_class_2016_returns: Indexes::new(client.clone(), &format!("{base_path}/dca_class_2016_returns")),
            dca_class_2016_stack: Indexes::new(client.clone(), &format!("{base_path}/dca_class_2016_stack")),
            dca_class_2017_avg_price: Indexes::new(client.clone(), &format!("{base_path}/dca_class_2017_avg_price")),
            dca_class_2017_returns: Indexes::new(client.clone(), &format!("{base_path}/dca_class_2017_returns")),
            dca_class_2017_stack: Indexes::new(client.clone(), &format!("{base_path}/dca_class_2017_stack")),
            dca_class_2018_avg_price: Indexes::new(client.clone(), &format!("{base_path}/dca_class_2018_avg_price")),
            dca_class_2018_returns: Indexes::new(client.clone(), &format!("{base_path}/dca_class_2018_returns")),
            dca_class_2018_stack: Indexes::new(client.clone(), &format!("{base_path}/dca_class_2018_stack")),
            dca_class_2019_avg_price: Indexes::new(client.clone(), &format!("{base_path}/dca_class_2019_avg_price")),
            dca_class_2019_returns: Indexes::new(client.clone(), &format!("{base_path}/dca_class_2019_returns")),
            dca_class_2019_stack: Indexes::new(client.clone(), &format!("{base_path}/dca_class_2019_stack")),
            dca_class_2020_avg_price: Indexes::new(client.clone(), &format!("{base_path}/dca_class_2020_avg_price")),
            dca_class_2020_returns: Indexes::new(client.clone(), &format!("{base_path}/dca_class_2020_returns")),
            dca_class_2020_stack: Indexes::new(client.clone(), &format!("{base_path}/dca_class_2020_stack")),
            dca_class_2021_avg_price: Indexes::new(client.clone(), &format!("{base_path}/dca_class_2021_avg_price")),
            dca_class_2021_returns: Indexes::new(client.clone(), &format!("{base_path}/dca_class_2021_returns")),
            dca_class_2021_stack: Indexes::new(client.clone(), &format!("{base_path}/dca_class_2021_stack")),
            dca_class_2022_avg_price: Indexes::new(client.clone(), &format!("{base_path}/dca_class_2022_avg_price")),
            dca_class_2022_returns: Indexes::new(client.clone(), &format!("{base_path}/dca_class_2022_returns")),
            dca_class_2022_stack: Indexes::new(client.clone(), &format!("{base_path}/dca_class_2022_stack")),
            dca_class_2023_avg_price: Indexes::new(client.clone(), &format!("{base_path}/dca_class_2023_avg_price")),
            dca_class_2023_returns: Indexes::new(client.clone(), &format!("{base_path}/dca_class_2023_returns")),
            dca_class_2023_stack: Indexes::new(client.clone(), &format!("{base_path}/dca_class_2023_stack")),
            dca_class_2024_avg_price: Indexes::new(client.clone(), &format!("{base_path}/dca_class_2024_avg_price")),
            dca_class_2024_returns: Indexes::new(client.clone(), &format!("{base_path}/dca_class_2024_returns")),
            dca_class_2024_stack: Indexes::new(client.clone(), &format!("{base_path}/dca_class_2024_stack")),
            dca_class_2025_avg_price: Indexes::new(client.clone(), &format!("{base_path}/dca_class_2025_avg_price")),
            dca_class_2025_returns: Indexes::new(client.clone(), &format!("{base_path}/dca_class_2025_returns")),
            dca_class_2025_stack: Indexes::new(client.clone(), &format!("{base_path}/dca_class_2025_stack")),
            max_days_between_price_aths: Indexes::new(client.clone(), &format!("{base_path}/max_days_between_price_aths")),
            max_years_between_price_aths: Indexes::new(client.clone(), &format!("{base_path}/max_years_between_price_aths")),
            price_10y_ago: Indexes::new(client.clone(), &format!("{base_path}/price_10y_ago")),
            price_13d_ema: Price13dEmaPattern::new(client.clone(), "price_13d_ema"),
            price_13d_sma: Price13dEmaPattern::new(client.clone(), "price_13d_sma"),
            price_144d_ema: Price13dEmaPattern::new(client.clone(), "price_144d_ema"),
            price_144d_sma: Price13dEmaPattern::new(client.clone(), "price_144d_sma"),
            price_1d_ago: Indexes::new(client.clone(), &format!("{base_path}/price_1d_ago")),
            price_1m_ago: Indexes::new(client.clone(), &format!("{base_path}/price_1m_ago")),
            price_1m_ema: Price13dEmaPattern::new(client.clone(), "price_1m_ema"),
            price_1m_max: Indexes::new(client.clone(), &format!("{base_path}/price_1m_max")),
            price_1m_min: Indexes::new(client.clone(), &format!("{base_path}/price_1m_min")),
            price_1m_sma: Price13dEmaPattern::new(client.clone(), "price_1m_sma"),
            price_1m_volatility: Indexes::new(client.clone(), &format!("{base_path}/price_1m_volatility")),
            price_1w_ago: Indexes::new(client.clone(), &format!("{base_path}/price_1w_ago")),
            price_1w_ema: Price13dEmaPattern::new(client.clone(), "price_1w_ema"),
            price_1w_max: Indexes::new(client.clone(), &format!("{base_path}/price_1w_max")),
            price_1w_min: Indexes::new(client.clone(), &format!("{base_path}/price_1w_min")),
            price_1w_sma: Price13dEmaPattern::new(client.clone(), "price_1w_sma"),
            price_1w_volatility: Indexes::new(client.clone(), &format!("{base_path}/price_1w_volatility")),
            price_1y_ago: Indexes::new(client.clone(), &format!("{base_path}/price_1y_ago")),
            price_1y_ema: Price13dEmaPattern::new(client.clone(), "price_1y_ema"),
            price_1y_max: Indexes::new(client.clone(), &format!("{base_path}/price_1y_max")),
            price_1y_min: Indexes::new(client.clone(), &format!("{base_path}/price_1y_min")),
            price_1y_sma: Price13dEmaPattern::new(client.clone(), "price_1y_sma"),
            price_1y_volatility: Indexes::new(client.clone(), &format!("{base_path}/price_1y_volatility")),
            price_200d_ema: Price13dEmaPattern::new(client.clone(), "price_200d_ema"),
            price_200d_sma: Price13dEmaPattern::new(client.clone(), "price_200d_sma"),
            price_200d_sma_x0_8: Indexes::new(client.clone(), &format!("{base_path}/price_200d_sma_x0_8")),
            price_200d_sma_x2_4: Indexes::new(client.clone(), &format!("{base_path}/price_200d_sma_x2_4")),
            price_200w_ema: Price13dEmaPattern::new(client.clone(), "price_200w_ema"),
            price_200w_sma: Price13dEmaPattern::new(client.clone(), "price_200w_sma"),
            price_21d_ema: Price13dEmaPattern::new(client.clone(), "price_21d_ema"),
            price_21d_sma: Price13dEmaPattern::new(client.clone(), "price_21d_sma"),
            price_2w_choppiness_index: Indexes::new(client.clone(), &format!("{base_path}/price_2w_choppiness_index")),
            price_2w_max: Indexes::new(client.clone(), &format!("{base_path}/price_2w_max")),
            price_2w_min: Indexes::new(client.clone(), &format!("{base_path}/price_2w_min")),
            price_2y_ago: Indexes::new(client.clone(), &format!("{base_path}/price_2y_ago")),
            price_2y_ema: Price13dEmaPattern::new(client.clone(), "price_2y_ema"),
            price_2y_sma: Price13dEmaPattern::new(client.clone(), "price_2y_sma"),
            price_34d_ema: Price13dEmaPattern::new(client.clone(), "price_34d_ema"),
            price_34d_sma: Price13dEmaPattern::new(client.clone(), "price_34d_sma"),
            price_3m_ago: Indexes::new(client.clone(), &format!("{base_path}/price_3m_ago")),
            price_3y_ago: Indexes::new(client.clone(), &format!("{base_path}/price_3y_ago")),
            price_4y_ago: Indexes::new(client.clone(), &format!("{base_path}/price_4y_ago")),
            price_4y_ema: Price13dEmaPattern::new(client.clone(), "price_4y_ema"),
            price_4y_sma: Price13dEmaPattern::new(client.clone(), "price_4y_sma"),
            price_55d_ema: Price13dEmaPattern::new(client.clone(), "price_55d_ema"),
            price_55d_sma: Price13dEmaPattern::new(client.clone(), "price_55d_sma"),
            price_5y_ago: Indexes::new(client.clone(), &format!("{base_path}/price_5y_ago")),
            price_6m_ago: Indexes::new(client.clone(), &format!("{base_path}/price_6m_ago")),
            price_6y_ago: Indexes::new(client.clone(), &format!("{base_path}/price_6y_ago")),
            price_89d_ema: Price13dEmaPattern::new(client.clone(), "price_89d_ema"),
            price_89d_sma: Price13dEmaPattern::new(client.clone(), "price_89d_sma"),
            price_8d_ema: Price13dEmaPattern::new(client.clone(), "price_8d_ema"),
            price_8d_sma: Price13dEmaPattern::new(client.clone(), "price_8d_sma"),
            price_8y_ago: Indexes::new(client.clone(), &format!("{base_path}/price_8y_ago")),
            price_ath: Indexes26::new(client.clone(), &format!("{base_path}/price_ath")),
            price_drawdown: Indexes26::new(client.clone(), &format!("{base_path}/price_drawdown")),
            price_true_range: Indexes5::new(client.clone(), &format!("{base_path}/price_true_range")),
            price_true_range_2w_sum: Indexes5::new(client.clone(), &format!("{base_path}/price_true_range_2w_sum")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Pools {
    pub pool: Indexes2<PoolSlug>,
    pub vecs: CatalogTree_Computed_Pools_Vecs,
}

impl CatalogTree_Computed_Pools {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            pool: Indexes2::new(client.clone(), &format!("{base_path}/pool")),
            vecs: CatalogTree_Computed_Pools_Vecs::new(client.clone(), &format!("{base_path}/vecs")),
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
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            axbt: AXbtPattern::new(client.clone(), &format!("{base_path}/AXbt")),
            aaopool: AXbtPattern::new(client.clone(), &format!("{base_path}/AaoPool")),
            antpool: AXbtPattern::new(client.clone(), &format!("{base_path}/AntPool")),
            arkpool: AXbtPattern::new(client.clone(), &format!("{base_path}/ArkPool")),
            asicminer: AXbtPattern::new(client.clone(), &format!("{base_path}/AsicMiner")),
            batpool: AXbtPattern::new(client.clone(), &format!("{base_path}/BatPool")),
            bcmonster: AXbtPattern::new(client.clone(), &format!("{base_path}/BcMonster")),
            bcpoolio: AXbtPattern::new(client.clone(), &format!("{base_path}/BcpoolIo")),
            binancepool: AXbtPattern::new(client.clone(), &format!("{base_path}/BinancePool")),
            bitclub: AXbtPattern::new(client.clone(), &format!("{base_path}/BitClub")),
            bitfufupool: AXbtPattern::new(client.clone(), &format!("{base_path}/BitFuFuPool")),
            bitfury: AXbtPattern::new(client.clone(), &format!("{base_path}/BitFury")),
            bitminter: AXbtPattern::new(client.clone(), &format!("{base_path}/BitMinter")),
            bitalo: AXbtPattern::new(client.clone(), &format!("{base_path}/Bitalo")),
            bitcoinaffiliatenetwork: AXbtPattern::new(client.clone(), &format!("{base_path}/BitcoinAffiliateNetwork")),
            bitcoincom: AXbtPattern::new(client.clone(), &format!("{base_path}/BitcoinCom")),
            bitcoinindia: AXbtPattern::new(client.clone(), &format!("{base_path}/BitcoinIndia")),
            bitcoinrussia: AXbtPattern::new(client.clone(), &format!("{base_path}/BitcoinRussia")),
            bitcoinukraine: AXbtPattern::new(client.clone(), &format!("{base_path}/BitcoinUkraine")),
            bitfarms: AXbtPattern::new(client.clone(), &format!("{base_path}/Bitfarms")),
            bitparking: AXbtPattern::new(client.clone(), &format!("{base_path}/Bitparking")),
            bitsolo: AXbtPattern::new(client.clone(), &format!("{base_path}/Bitsolo")),
            bixin: AXbtPattern::new(client.clone(), &format!("{base_path}/Bixin")),
            blockfills: AXbtPattern::new(client.clone(), &format!("{base_path}/BlockFills")),
            braiinspool: AXbtPattern::new(client.clone(), &format!("{base_path}/BraiinsPool")),
            bravomining: AXbtPattern::new(client.clone(), &format!("{base_path}/BravoMining")),
            btpool: AXbtPattern::new(client.clone(), &format!("{base_path}/BtPool")),
            btccom: AXbtPattern::new(client.clone(), &format!("{base_path}/BtcCom")),
            btcdig: AXbtPattern::new(client.clone(), &format!("{base_path}/BtcDig")),
            btcguild: AXbtPattern::new(client.clone(), &format!("{base_path}/BtcGuild")),
            btclab: AXbtPattern::new(client.clone(), &format!("{base_path}/BtcLab")),
            btcmp: AXbtPattern::new(client.clone(), &format!("{base_path}/BtcMp")),
            btcnuggets: AXbtPattern::new(client.clone(), &format!("{base_path}/BtcNuggets")),
            btcpoolparty: AXbtPattern::new(client.clone(), &format!("{base_path}/BtcPoolParty")),
            btcserv: AXbtPattern::new(client.clone(), &format!("{base_path}/BtcServ")),
            btctop: AXbtPattern::new(client.clone(), &format!("{base_path}/BtcTop")),
            btcc: AXbtPattern::new(client.clone(), &format!("{base_path}/Btcc")),
            bwpool: AXbtPattern::new(client.clone(), &format!("{base_path}/BwPool")),
            bytepool: AXbtPattern::new(client.clone(), &format!("{base_path}/BytePool")),
            canoe: AXbtPattern::new(client.clone(), &format!("{base_path}/Canoe")),
            canoepool: AXbtPattern::new(client.clone(), &format!("{base_path}/CanoePool")),
            carbonnegative: AXbtPattern::new(client.clone(), &format!("{base_path}/CarbonNegative")),
            ckpool: AXbtPattern::new(client.clone(), &format!("{base_path}/CkPool")),
            cloudhashing: AXbtPattern::new(client.clone(), &format!("{base_path}/CloudHashing")),
            coinlab: AXbtPattern::new(client.clone(), &format!("{base_path}/CoinLab")),
            cointerra: AXbtPattern::new(client.clone(), &format!("{base_path}/Cointerra")),
            connectbtc: AXbtPattern::new(client.clone(), &format!("{base_path}/ConnectBtc")),
            dpool: AXbtPattern::new(client.clone(), &format!("{base_path}/DPool")),
            dcexploration: AXbtPattern::new(client.clone(), &format!("{base_path}/DcExploration")),
            dcex: AXbtPattern::new(client.clone(), &format!("{base_path}/Dcex")),
            digitalbtc: AXbtPattern::new(client.clone(), &format!("{base_path}/DigitalBtc")),
            digitalxmintsy: AXbtPattern::new(client.clone(), &format!("{base_path}/DigitalXMintsy")),
            eclipsemc: AXbtPattern::new(client.clone(), &format!("{base_path}/EclipseMc")),
            eightbaochi: AXbtPattern::new(client.clone(), &format!("{base_path}/EightBaochi")),
            ekanembtc: AXbtPattern::new(client.clone(), &format!("{base_path}/EkanemBtc")),
            eligius: AXbtPattern::new(client.clone(), &format!("{base_path}/Eligius")),
            emcdpool: AXbtPattern::new(client.clone(), &format!("{base_path}/EmcdPool")),
            entrustcharitypool: AXbtPattern::new(client.clone(), &format!("{base_path}/EntrustCharityPool")),
            eobot: AXbtPattern::new(client.clone(), &format!("{base_path}/Eobot")),
            exxbw: AXbtPattern::new(client.clone(), &format!("{base_path}/ExxBw")),
            f2pool: AXbtPattern::new(client.clone(), &format!("{base_path}/F2Pool")),
            fiftyeightcoin: AXbtPattern::new(client.clone(), &format!("{base_path}/FiftyEightCoin")),
            foundryusa: AXbtPattern::new(client.clone(), &format!("{base_path}/FoundryUsa")),
            futurebitapollosolo: AXbtPattern::new(client.clone(), &format!("{base_path}/FutureBitApolloSolo")),
            gbminers: AXbtPattern::new(client.clone(), &format!("{base_path}/GbMiners")),
            ghashio: AXbtPattern::new(client.clone(), &format!("{base_path}/GhashIo")),
            givemecoins: AXbtPattern::new(client.clone(), &format!("{base_path}/GiveMeCoins")),
            gogreenlight: AXbtPattern::new(client.clone(), &format!("{base_path}/GoGreenLight")),
            haozhuzhu: AXbtPattern::new(client.clone(), &format!("{base_path}/HaoZhuZhu")),
            haominer: AXbtPattern::new(client.clone(), &format!("{base_path}/Haominer")),
            hashbx: AXbtPattern::new(client.clone(), &format!("{base_path}/HashBx")),
            hashpool: AXbtPattern::new(client.clone(), &format!("{base_path}/HashPool")),
            helix: AXbtPattern::new(client.clone(), &format!("{base_path}/Helix")),
            hhtt: AXbtPattern::new(client.clone(), &format!("{base_path}/Hhtt")),
            hotpool: AXbtPattern::new(client.clone(), &format!("{base_path}/HotPool")),
            hummerpool: AXbtPattern::new(client.clone(), &format!("{base_path}/Hummerpool")),
            huobipool: AXbtPattern::new(client.clone(), &format!("{base_path}/HuobiPool")),
            innopolistech: AXbtPattern::new(client.clone(), &format!("{base_path}/InnopolisTech")),
            kanopool: AXbtPattern::new(client.clone(), &format!("{base_path}/KanoPool")),
            kncminer: AXbtPattern::new(client.clone(), &format!("{base_path}/KncMiner")),
            kucoinpool: AXbtPattern::new(client.clone(), &format!("{base_path}/KuCoinPool")),
            lubiancom: AXbtPattern::new(client.clone(), &format!("{base_path}/LubianCom")),
            luckypool: AXbtPattern::new(client.clone(), &format!("{base_path}/LuckyPool")),
            luxor: AXbtPattern::new(client.clone(), &format!("{base_path}/Luxor")),
            marapool: AXbtPattern::new(client.clone(), &format!("{base_path}/MaraPool")),
            maxbtc: AXbtPattern::new(client.clone(), &format!("{base_path}/MaxBtc")),
            maxipool: AXbtPattern::new(client.clone(), &format!("{base_path}/MaxiPool")),
            megabigpower: AXbtPattern::new(client.clone(), &format!("{base_path}/MegaBigPower")),
            minerium: AXbtPattern::new(client.clone(), &format!("{base_path}/Minerium")),
            miningcity: AXbtPattern::new(client.clone(), &format!("{base_path}/MiningCity")),
            miningdutch: AXbtPattern::new(client.clone(), &format!("{base_path}/MiningDutch")),
            miningkings: AXbtPattern::new(client.clone(), &format!("{base_path}/MiningKings")),
            miningsquared: AXbtPattern::new(client.clone(), &format!("{base_path}/MiningSquared")),
            mmpool: AXbtPattern::new(client.clone(), &format!("{base_path}/Mmpool")),
            mtred: AXbtPattern::new(client.clone(), &format!("{base_path}/MtRed")),
            multicoinco: AXbtPattern::new(client.clone(), &format!("{base_path}/MultiCoinCo")),
            multipool: AXbtPattern::new(client.clone(), &format!("{base_path}/Multipool")),
            mybtccoinpool: AXbtPattern::new(client.clone(), &format!("{base_path}/MyBtcCoinPool")),
            neopool: AXbtPattern::new(client.clone(), &format!("{base_path}/Neopool")),
            nexious: AXbtPattern::new(client.clone(), &format!("{base_path}/Nexious")),
            nicehash: AXbtPattern::new(client.clone(), &format!("{base_path}/NiceHash")),
            nmcbit: AXbtPattern::new(client.clone(), &format!("{base_path}/NmcBit")),
            novablock: AXbtPattern::new(client.clone(), &format!("{base_path}/NovaBlock")),
            ocean: AXbtPattern::new(client.clone(), &format!("{base_path}/Ocean")),
            okexpool: AXbtPattern::new(client.clone(), &format!("{base_path}/OkExPool")),
            okminer: AXbtPattern::new(client.clone(), &format!("{base_path}/OkMiner")),
            okkong: AXbtPattern::new(client.clone(), &format!("{base_path}/Okkong")),
            okpooltop: AXbtPattern::new(client.clone(), &format!("{base_path}/OkpoolTop")),
            onehash: AXbtPattern::new(client.clone(), &format!("{base_path}/OneHash")),
            onem1x: AXbtPattern::new(client.clone(), &format!("{base_path}/OneM1x")),
            onethash: AXbtPattern::new(client.clone(), &format!("{base_path}/OneThash")),
            ozcoin: AXbtPattern::new(client.clone(), &format!("{base_path}/OzCoin")),
            phashio: AXbtPattern::new(client.clone(), &format!("{base_path}/PHashIo")),
            parasite: AXbtPattern::new(client.clone(), &format!("{base_path}/Parasite")),
            patels: AXbtPattern::new(client.clone(), &format!("{base_path}/Patels")),
            pegapool: AXbtPattern::new(client.clone(), &format!("{base_path}/PegaPool")),
            phoenix: AXbtPattern::new(client.clone(), &format!("{base_path}/Phoenix")),
            polmine: AXbtPattern::new(client.clone(), &format!("{base_path}/Polmine")),
            pool175btc: AXbtPattern::new(client.clone(), &format!("{base_path}/Pool175btc")),
            pool50btc: AXbtPattern::new(client.clone(), &format!("{base_path}/Pool50btc")),
            poolin: AXbtPattern::new(client.clone(), &format!("{base_path}/Poolin")),
            portlandhodl: AXbtPattern::new(client.clone(), &format!("{base_path}/PortlandHodl")),
            publicpool: AXbtPattern::new(client.clone(), &format!("{base_path}/PublicPool")),
            purebtccom: AXbtPattern::new(client.clone(), &format!("{base_path}/PureBtcCom")),
            rawpool: AXbtPattern::new(client.clone(), &format!("{base_path}/Rawpool")),
            rigpool: AXbtPattern::new(client.clone(), &format!("{base_path}/RigPool")),
            sbicrypto: AXbtPattern::new(client.clone(), &format!("{base_path}/SbiCrypto")),
            secpool: AXbtPattern::new(client.clone(), &format!("{base_path}/SecPool")),
            secretsuperstar: AXbtPattern::new(client.clone(), &format!("{base_path}/SecretSuperstar")),
            sevenpool: AXbtPattern::new(client.clone(), &format!("{base_path}/SevenPool")),
            shawnp0wers: AXbtPattern::new(client.clone(), &format!("{base_path}/ShawnP0wers")),
            sigmapoolcom: AXbtPattern::new(client.clone(), &format!("{base_path}/SigmapoolCom")),
            simplecoinus: AXbtPattern::new(client.clone(), &format!("{base_path}/SimplecoinUs")),
            solock: AXbtPattern::new(client.clone(), &format!("{base_path}/SoloCk")),
            spiderpool: AXbtPattern::new(client.clone(), &format!("{base_path}/SpiderPool")),
            stminingcorp: AXbtPattern::new(client.clone(), &format!("{base_path}/StMiningCorp")),
            tangpool: AXbtPattern::new(client.clone(), &format!("{base_path}/Tangpool")),
            tatmaspool: AXbtPattern::new(client.clone(), &format!("{base_path}/TatmasPool")),
            tbdice: AXbtPattern::new(client.clone(), &format!("{base_path}/TbDice")),
            telco214: AXbtPattern::new(client.clone(), &format!("{base_path}/Telco214")),
            terrapool: AXbtPattern::new(client.clone(), &format!("{base_path}/TerraPool")),
            tiger: AXbtPattern::new(client.clone(), &format!("{base_path}/Tiger")),
            tigerpoolnet: AXbtPattern::new(client.clone(), &format!("{base_path}/TigerpoolNet")),
            titan: AXbtPattern::new(client.clone(), &format!("{base_path}/Titan")),
            transactioncoinmining: AXbtPattern::new(client.clone(), &format!("{base_path}/TransactionCoinMining")),
            trickysbtcpool: AXbtPattern::new(client.clone(), &format!("{base_path}/TrickysBtcPool")),
            triplemining: AXbtPattern::new(client.clone(), &format!("{base_path}/TripleMining")),
            twentyoneinc: AXbtPattern::new(client.clone(), &format!("{base_path}/TwentyOneInc")),
            ultimuspool: AXbtPattern::new(client.clone(), &format!("{base_path}/UltimusPool")),
            unknown: AXbtPattern::new(client.clone(), &format!("{base_path}/Unknown")),
            unomp: AXbtPattern::new(client.clone(), &format!("{base_path}/Unomp")),
            viabtc: AXbtPattern::new(client.clone(), &format!("{base_path}/ViaBtc")),
            waterhole: AXbtPattern::new(client.clone(), &format!("{base_path}/Waterhole")),
            wayicn: AXbtPattern::new(client.clone(), &format!("{base_path}/WayiCn")),
            whitepool: AXbtPattern::new(client.clone(), &format!("{base_path}/WhitePool")),
            wk057: AXbtPattern::new(client.clone(), &format!("{base_path}/Wk057")),
            yourbtcnet: AXbtPattern::new(client.clone(), &format!("{base_path}/YourbtcNet")),
            zulupool: AXbtPattern::new(client.clone(), &format!("{base_path}/Zulupool")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Price {
    pub price_close: Indexes3<Dollars>,
    pub price_close_in_cents: Indexes13<Cents>,
    pub price_close_in_sats: Indexes3<Sats>,
    pub price_high: Indexes3<Dollars>,
    pub price_high_in_cents: Indexes13<Cents>,
    pub price_high_in_sats: Indexes3<Sats>,
    pub price_low: Indexes3<Dollars>,
    pub price_low_in_cents: Indexes13<Cents>,
    pub price_low_in_sats: Indexes3<Sats>,
    pub price_ohlc: Indexes3<OHLCDollars>,
    pub price_ohlc_in_sats: Indexes3<OHLCSats>,
    pub price_open: Indexes3<Dollars>,
    pub price_open_in_cents: Indexes13<Cents>,
    pub price_open_in_sats: Indexes3<Sats>,
}

impl CatalogTree_Computed_Price {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            price_close: Indexes3::new(client.clone(), &format!("{base_path}/price_close")),
            price_close_in_cents: Indexes13::new(client.clone(), &format!("{base_path}/price_close_in_cents")),
            price_close_in_sats: Indexes3::new(client.clone(), &format!("{base_path}/price_close_in_sats")),
            price_high: Indexes3::new(client.clone(), &format!("{base_path}/price_high")),
            price_high_in_cents: Indexes13::new(client.clone(), &format!("{base_path}/price_high_in_cents")),
            price_high_in_sats: Indexes3::new(client.clone(), &format!("{base_path}/price_high_in_sats")),
            price_low: Indexes3::new(client.clone(), &format!("{base_path}/price_low")),
            price_low_in_cents: Indexes13::new(client.clone(), &format!("{base_path}/price_low_in_cents")),
            price_low_in_sats: Indexes3::new(client.clone(), &format!("{base_path}/price_low_in_sats")),
            price_ohlc: Indexes3::new(client.clone(), &format!("{base_path}/price_ohlc")),
            price_ohlc_in_sats: Indexes3::new(client.clone(), &format!("{base_path}/price_ohlc_in_sats")),
            price_open: Indexes3::new(client.clone(), &format!("{base_path}/price_open")),
            price_open_in_cents: Indexes13::new(client.clone(), &format!("{base_path}/price_open_in_cents")),
            price_open_in_sats: Indexes3::new(client.clone(), &format!("{base_path}/price_open_in_sats")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Stateful {
    pub addr_count: Indexes3<StoredU64>,
    pub address_cohorts: CatalogTree_Computed_Stateful_AddressCohorts,
    pub addresses_data: CatalogTree_Computed_Stateful_AddressesData,
    pub addresstype_to_height_to_addr_count: AddresstypeToHeightToAddrCountPattern<StoredU64>,
    pub addresstype_to_height_to_empty_addr_count: AddresstypeToHeightToAddrCountPattern<StoredU64>,
    pub addresstype_to_indexes_to_addr_count: AddresstypeToHeightToAddrCountPattern<StoredU64>,
    pub addresstype_to_indexes_to_empty_addr_count: AddresstypeToHeightToAddrCountPattern<StoredU64>,
    pub any_address_indexes: AddresstypeToHeightToAddrCountPattern<AnyAddressIndex>,
    pub chain_state: Indexes2<SupplyState>,
    pub empty_addr_count: Indexes3<StoredU64>,
    pub emptyaddressindex: Indexes29<EmptyAddressIndex>,
    pub loadedaddressindex: Indexes30<LoadedAddressIndex>,
    pub market_cap: Indexes26<Dollars>,
    pub opreturn_supply: SupplyPattern,
    pub unspendable_supply: SupplyPattern,
    pub utxo_cohorts: CatalogTree_Computed_Stateful_UtxoCohorts,
}

impl CatalogTree_Computed_Stateful {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            addr_count: Indexes3::new(client.clone(), &format!("{base_path}/addr_count")),
            address_cohorts: CatalogTree_Computed_Stateful_AddressCohorts::new(client.clone(), &format!("{base_path}/address_cohorts")),
            addresses_data: CatalogTree_Computed_Stateful_AddressesData::new(client.clone(), &format!("{base_path}/addresses_data")),
            addresstype_to_height_to_addr_count: AddresstypeToHeightToAddrCountPattern::new(client.clone(), &format!("{base_path}/addresstype_to_height_to_addr_count")),
            addresstype_to_height_to_empty_addr_count: AddresstypeToHeightToAddrCountPattern::new(client.clone(), &format!("{base_path}/addresstype_to_height_to_empty_addr_count")),
            addresstype_to_indexes_to_addr_count: AddresstypeToHeightToAddrCountPattern::new(client.clone(), &format!("{base_path}/addresstype_to_indexes_to_addr_count")),
            addresstype_to_indexes_to_empty_addr_count: AddresstypeToHeightToAddrCountPattern::new(client.clone(), &format!("{base_path}/addresstype_to_indexes_to_empty_addr_count")),
            any_address_indexes: AddresstypeToHeightToAddrCountPattern::new(client.clone(), &format!("{base_path}/any_address_indexes")),
            chain_state: Indexes2::new(client.clone(), &format!("{base_path}/chain_state")),
            empty_addr_count: Indexes3::new(client.clone(), &format!("{base_path}/empty_addr_count")),
            emptyaddressindex: Indexes29::new(client.clone(), &format!("{base_path}/emptyaddressindex")),
            loadedaddressindex: Indexes30::new(client.clone(), &format!("{base_path}/loadedaddressindex")),
            market_cap: Indexes26::new(client.clone(), &format!("{base_path}/market_cap")),
            opreturn_supply: SupplyPattern::new(client.clone(), &format!("{base_path}/opreturn_supply")),
            unspendable_supply: SupplyPattern::new(client.clone(), &format!("{base_path}/unspendable_supply")),
            utxo_cohorts: CatalogTree_Computed_Stateful_UtxoCohorts::new(client.clone(), &format!("{base_path}/utxo_cohorts")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Stateful_AddressCohorts {
    pub amount_range: CatalogTree_Computed_Stateful_AddressCohorts_AmountRange,
    pub ge_amount: CatalogTree_Computed_Stateful_AddressCohorts_GeAmount,
    pub lt_amount: CatalogTree_Computed_Stateful_AddressCohorts_LtAmount,
}

impl CatalogTree_Computed_Stateful_AddressCohorts {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            amount_range: CatalogTree_Computed_Stateful_AddressCohorts_AmountRange::new(client.clone(), &format!("{base_path}/amount_range")),
            ge_amount: CatalogTree_Computed_Stateful_AddressCohorts_GeAmount::new(client.clone(), &format!("{base_path}/ge_amount")),
            lt_amount: CatalogTree_Computed_Stateful_AddressCohorts_LtAmount::new(client.clone(), &format!("{base_path}/lt_amount")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Stateful_AddressCohorts_AmountRange {
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

impl CatalogTree_Computed_Stateful_AddressCohorts_AmountRange {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            _0sats: _0satsPattern::new(client.clone(), &format!("{base_path}/_0sats")),
            _100btc_to_1k_btc: _0satsPattern::new(client.clone(), &format!("{base_path}/_100btc_to_1k_btc")),
            _100k_btc_or_more: _0satsPattern::new(client.clone(), &format!("{base_path}/_100k_btc_or_more")),
            _100k_sats_to_1m_sats: _0satsPattern::new(client.clone(), &format!("{base_path}/_100k_sats_to_1m_sats")),
            _100sats_to_1k_sats: _0satsPattern::new(client.clone(), &format!("{base_path}/_100sats_to_1k_sats")),
            _10btc_to_100btc: _0satsPattern::new(client.clone(), &format!("{base_path}/_10btc_to_100btc")),
            _10k_btc_to_100k_btc: _0satsPattern::new(client.clone(), &format!("{base_path}/_10k_btc_to_100k_btc")),
            _10k_sats_to_100k_sats: _0satsPattern::new(client.clone(), &format!("{base_path}/_10k_sats_to_100k_sats")),
            _10m_sats_to_1btc: _0satsPattern::new(client.clone(), &format!("{base_path}/_10m_sats_to_1btc")),
            _10sats_to_100sats: _0satsPattern::new(client.clone(), &format!("{base_path}/_10sats_to_100sats")),
            _1btc_to_10btc: _0satsPattern::new(client.clone(), &format!("{base_path}/_1btc_to_10btc")),
            _1k_btc_to_10k_btc: _0satsPattern::new(client.clone(), &format!("{base_path}/_1k_btc_to_10k_btc")),
            _1k_sats_to_10k_sats: _0satsPattern::new(client.clone(), &format!("{base_path}/_1k_sats_to_10k_sats")),
            _1m_sats_to_10m_sats: _0satsPattern::new(client.clone(), &format!("{base_path}/_1m_sats_to_10m_sats")),
            _1sat_to_10sats: _0satsPattern::new(client.clone(), &format!("{base_path}/_1sat_to_10sats")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Stateful_AddressCohorts_GeAmount {
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

impl CatalogTree_Computed_Stateful_AddressCohorts_GeAmount {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            _100btc: _0satsPattern::new(client.clone(), &format!("{base_path}/_100btc")),
            _100k_sats: _0satsPattern::new(client.clone(), &format!("{base_path}/_100k_sats")),
            _100sats: _0satsPattern::new(client.clone(), &format!("{base_path}/_100sats")),
            _10btc: _0satsPattern::new(client.clone(), &format!("{base_path}/_10btc")),
            _10k_btc: _0satsPattern::new(client.clone(), &format!("{base_path}/_10k_btc")),
            _10k_sats: _0satsPattern::new(client.clone(), &format!("{base_path}/_10k_sats")),
            _10m_sats: _0satsPattern::new(client.clone(), &format!("{base_path}/_10m_sats")),
            _10sats: _0satsPattern::new(client.clone(), &format!("{base_path}/_10sats")),
            _1btc: _0satsPattern::new(client.clone(), &format!("{base_path}/_1btc")),
            _1k_btc: _0satsPattern::new(client.clone(), &format!("{base_path}/_1k_btc")),
            _1k_sats: _0satsPattern::new(client.clone(), &format!("{base_path}/_1k_sats")),
            _1m_sats: _0satsPattern::new(client.clone(), &format!("{base_path}/_1m_sats")),
            _1sat: _0satsPattern::new(client.clone(), &format!("{base_path}/_1sat")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Stateful_AddressCohorts_LtAmount {
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

impl CatalogTree_Computed_Stateful_AddressCohorts_LtAmount {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            _100btc: _0satsPattern::new(client.clone(), &format!("{base_path}/_100btc")),
            _100k_btc: _0satsPattern::new(client.clone(), &format!("{base_path}/_100k_btc")),
            _100k_sats: _0satsPattern::new(client.clone(), &format!("{base_path}/_100k_sats")),
            _100sats: _0satsPattern::new(client.clone(), &format!("{base_path}/_100sats")),
            _10btc: _0satsPattern::new(client.clone(), &format!("{base_path}/_10btc")),
            _10k_btc: _0satsPattern::new(client.clone(), &format!("{base_path}/_10k_btc")),
            _10k_sats: _0satsPattern::new(client.clone(), &format!("{base_path}/_10k_sats")),
            _10m_sats: _0satsPattern::new(client.clone(), &format!("{base_path}/_10m_sats")),
            _10sats: _0satsPattern::new(client.clone(), &format!("{base_path}/_10sats")),
            _1btc: _0satsPattern::new(client.clone(), &format!("{base_path}/_1btc")),
            _1k_btc: _0satsPattern::new(client.clone(), &format!("{base_path}/_1k_btc")),
            _1k_sats: _0satsPattern::new(client.clone(), &format!("{base_path}/_1k_sats")),
            _1m_sats: _0satsPattern::new(client.clone(), &format!("{base_path}/_1m_sats")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Stateful_AddressesData {
    pub empty: Indexes29<EmptyAddressData>,
    pub loaded: Indexes30<LoadedAddressData>,
}

impl CatalogTree_Computed_Stateful_AddressesData {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            empty: Indexes29::new(client.clone(), &format!("{base_path}/empty")),
            loaded: Indexes30::new(client.clone(), &format!("{base_path}/loaded")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Stateful_UtxoCohorts {
    pub age_range: CatalogTree_Computed_Stateful_UtxoCohorts_AgeRange,
    pub all: CatalogTree_Computed_Stateful_UtxoCohorts_All,
    pub amount_range: CatalogTree_Computed_Stateful_UtxoCohorts_AmountRange,
    pub epoch: CatalogTree_Computed_Stateful_UtxoCohorts_Epoch,
    pub ge_amount: CatalogTree_Computed_Stateful_UtxoCohorts_GeAmount,
    pub lt_amount: CatalogTree_Computed_Stateful_UtxoCohorts_LtAmount,
    pub max_age: CatalogTree_Computed_Stateful_UtxoCohorts_MaxAge,
    pub min_age: CatalogTree_Computed_Stateful_UtxoCohorts_MinAge,
    pub term: CatalogTree_Computed_Stateful_UtxoCohorts_Term,
    pub type_: CatalogTree_Computed_Stateful_UtxoCohorts_Type,
    pub year: CatalogTree_Computed_Stateful_UtxoCohorts_Year,
}

impl CatalogTree_Computed_Stateful_UtxoCohorts {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            age_range: CatalogTree_Computed_Stateful_UtxoCohorts_AgeRange::new(client.clone(), &format!("{base_path}/age_range")),
            all: CatalogTree_Computed_Stateful_UtxoCohorts_All::new(client.clone(), &format!("{base_path}/all")),
            amount_range: CatalogTree_Computed_Stateful_UtxoCohorts_AmountRange::new(client.clone(), &format!("{base_path}/amount_range")),
            epoch: CatalogTree_Computed_Stateful_UtxoCohorts_Epoch::new(client.clone(), &format!("{base_path}/epoch")),
            ge_amount: CatalogTree_Computed_Stateful_UtxoCohorts_GeAmount::new(client.clone(), &format!("{base_path}/ge_amount")),
            lt_amount: CatalogTree_Computed_Stateful_UtxoCohorts_LtAmount::new(client.clone(), &format!("{base_path}/lt_amount")),
            max_age: CatalogTree_Computed_Stateful_UtxoCohorts_MaxAge::new(client.clone(), &format!("{base_path}/max_age")),
            min_age: CatalogTree_Computed_Stateful_UtxoCohorts_MinAge::new(client.clone(), &format!("{base_path}/min_age")),
            term: CatalogTree_Computed_Stateful_UtxoCohorts_Term::new(client.clone(), &format!("{base_path}/term")),
            type_: CatalogTree_Computed_Stateful_UtxoCohorts_Type::new(client.clone(), &format!("{base_path}/type_")),
            year: CatalogTree_Computed_Stateful_UtxoCohorts_Year::new(client.clone(), &format!("{base_path}/year")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Stateful_UtxoCohorts_AgeRange {
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

impl CatalogTree_Computed_Stateful_UtxoCohorts_AgeRange {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            _10y_to_12y: _10yTo12yPattern::new(client.clone(), &format!("{base_path}/_10y_to_12y")),
            _12y_to_15y: _10yTo12yPattern::new(client.clone(), &format!("{base_path}/_12y_to_15y")),
            _1d_to_1w: _10yTo12yPattern::new(client.clone(), &format!("{base_path}/_1d_to_1w")),
            _1m_to_2m: _10yTo12yPattern::new(client.clone(), &format!("{base_path}/_1m_to_2m")),
            _1w_to_1m: _10yTo12yPattern::new(client.clone(), &format!("{base_path}/_1w_to_1m")),
            _1y_to_2y: _10yTo12yPattern::new(client.clone(), &format!("{base_path}/_1y_to_2y")),
            _2m_to_3m: _10yTo12yPattern::new(client.clone(), &format!("{base_path}/_2m_to_3m")),
            _2y_to_3y: _10yTo12yPattern::new(client.clone(), &format!("{base_path}/_2y_to_3y")),
            _3m_to_4m: _10yTo12yPattern::new(client.clone(), &format!("{base_path}/_3m_to_4m")),
            _3y_to_4y: _10yTo12yPattern::new(client.clone(), &format!("{base_path}/_3y_to_4y")),
            _4m_to_5m: _10yTo12yPattern::new(client.clone(), &format!("{base_path}/_4m_to_5m")),
            _4y_to_5y: _10yTo12yPattern::new(client.clone(), &format!("{base_path}/_4y_to_5y")),
            _5m_to_6m: _10yTo12yPattern::new(client.clone(), &format!("{base_path}/_5m_to_6m")),
            _5y_to_6y: _10yTo12yPattern::new(client.clone(), &format!("{base_path}/_5y_to_6y")),
            _6m_to_1y: _10yTo12yPattern::new(client.clone(), &format!("{base_path}/_6m_to_1y")),
            _6y_to_7y: _10yTo12yPattern::new(client.clone(), &format!("{base_path}/_6y_to_7y")),
            _7y_to_8y: _10yTo12yPattern::new(client.clone(), &format!("{base_path}/_7y_to_8y")),
            _8y_to_10y: _10yTo12yPattern::new(client.clone(), &format!("{base_path}/_8y_to_10y")),
            from_15y: _10yTo12yPattern::new(client.clone(), &format!("{base_path}/from_15y")),
            up_to_1d: UpTo1dPattern::new(client.clone(), &format!("{base_path}/up_to_1d")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Stateful_UtxoCohorts_All {
    pub activity: ActivityPattern,
    pub price_paid: PricePaidPattern2,
    pub realized: RealizedPattern3,
    pub relative: CatalogTree_Computed_Stateful_UtxoCohorts_All_Relative,
    pub supply: SupplyPattern2,
    pub unrealized: UnrealizedPattern,
}

impl CatalogTree_Computed_Stateful_UtxoCohorts_All {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            activity: ActivityPattern::new(client.clone(), &format!("{base_path}/activity")),
            price_paid: PricePaidPattern2::new(client.clone(), &format!("{base_path}/price_paid")),
            realized: RealizedPattern3::new(client.clone(), &format!("{base_path}/realized")),
            relative: CatalogTree_Computed_Stateful_UtxoCohorts_All_Relative::new(client.clone(), &format!("{base_path}/relative")),
            supply: SupplyPattern2::new(client.clone(), &format!("{base_path}/supply")),
            unrealized: UnrealizedPattern::new(client.clone(), &format!("{base_path}/unrealized")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Stateful_UtxoCohorts_All_Relative {
    pub neg_unrealized_loss_rel_to_own_total_unrealized_pnl: Indexes27<StoredF32>,
    pub net_unrealized_pnl_rel_to_own_total_unrealized_pnl: Indexes26<StoredF32>,
    pub supply_in_loss_rel_to_own_supply: Indexes27<StoredF64>,
    pub supply_in_profit_rel_to_own_supply: Indexes27<StoredF64>,
    pub unrealized_loss_rel_to_own_total_unrealized_pnl: Indexes27<StoredF32>,
    pub unrealized_profit_rel_to_own_total_unrealized_pnl: Indexes27<StoredF32>,
}

impl CatalogTree_Computed_Stateful_UtxoCohorts_All_Relative {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            neg_unrealized_loss_rel_to_own_total_unrealized_pnl: Indexes27::new(client.clone(), &format!("{base_path}/neg_unrealized_loss_rel_to_own_total_unrealized_pnl")),
            net_unrealized_pnl_rel_to_own_total_unrealized_pnl: Indexes26::new(client.clone(), &format!("{base_path}/net_unrealized_pnl_rel_to_own_total_unrealized_pnl")),
            supply_in_loss_rel_to_own_supply: Indexes27::new(client.clone(), &format!("{base_path}/supply_in_loss_rel_to_own_supply")),
            supply_in_profit_rel_to_own_supply: Indexes27::new(client.clone(), &format!("{base_path}/supply_in_profit_rel_to_own_supply")),
            unrealized_loss_rel_to_own_total_unrealized_pnl: Indexes27::new(client.clone(), &format!("{base_path}/unrealized_loss_rel_to_own_total_unrealized_pnl")),
            unrealized_profit_rel_to_own_total_unrealized_pnl: Indexes27::new(client.clone(), &format!("{base_path}/unrealized_profit_rel_to_own_total_unrealized_pnl")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Stateful_UtxoCohorts_AmountRange {
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

impl CatalogTree_Computed_Stateful_UtxoCohorts_AmountRange {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            _0sats: _0satsPattern2::new(client.clone(), &format!("{base_path}/_0sats")),
            _100btc_to_1k_btc: _0satsPattern2::new(client.clone(), &format!("{base_path}/_100btc_to_1k_btc")),
            _100k_btc_or_more: _0satsPattern2::new(client.clone(), &format!("{base_path}/_100k_btc_or_more")),
            _100k_sats_to_1m_sats: _0satsPattern2::new(client.clone(), &format!("{base_path}/_100k_sats_to_1m_sats")),
            _100sats_to_1k_sats: _0satsPattern2::new(client.clone(), &format!("{base_path}/_100sats_to_1k_sats")),
            _10btc_to_100btc: _0satsPattern2::new(client.clone(), &format!("{base_path}/_10btc_to_100btc")),
            _10k_btc_to_100k_btc: _0satsPattern2::new(client.clone(), &format!("{base_path}/_10k_btc_to_100k_btc")),
            _10k_sats_to_100k_sats: _0satsPattern2::new(client.clone(), &format!("{base_path}/_10k_sats_to_100k_sats")),
            _10m_sats_to_1btc: _0satsPattern2::new(client.clone(), &format!("{base_path}/_10m_sats_to_1btc")),
            _10sats_to_100sats: _0satsPattern2::new(client.clone(), &format!("{base_path}/_10sats_to_100sats")),
            _1btc_to_10btc: _0satsPattern2::new(client.clone(), &format!("{base_path}/_1btc_to_10btc")),
            _1k_btc_to_10k_btc: _0satsPattern2::new(client.clone(), &format!("{base_path}/_1k_btc_to_10k_btc")),
            _1k_sats_to_10k_sats: _0satsPattern2::new(client.clone(), &format!("{base_path}/_1k_sats_to_10k_sats")),
            _1m_sats_to_10m_sats: _0satsPattern2::new(client.clone(), &format!("{base_path}/_1m_sats_to_10m_sats")),
            _1sat_to_10sats: _0satsPattern2::new(client.clone(), &format!("{base_path}/_1sat_to_10sats")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Stateful_UtxoCohorts_Epoch {
    pub _0: _10yTo12yPattern,
    pub _1: _10yTo12yPattern,
    pub _2: _10yTo12yPattern,
    pub _3: _10yTo12yPattern,
    pub _4: _10yTo12yPattern,
}

impl CatalogTree_Computed_Stateful_UtxoCohorts_Epoch {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            _0: _10yTo12yPattern::new(client.clone(), &format!("{base_path}/_0")),
            _1: _10yTo12yPattern::new(client.clone(), &format!("{base_path}/_1")),
            _2: _10yTo12yPattern::new(client.clone(), &format!("{base_path}/_2")),
            _3: _10yTo12yPattern::new(client.clone(), &format!("{base_path}/_3")),
            _4: _10yTo12yPattern::new(client.clone(), &format!("{base_path}/_4")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Stateful_UtxoCohorts_GeAmount {
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

impl CatalogTree_Computed_Stateful_UtxoCohorts_GeAmount {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            _100btc: _0satsPattern2::new(client.clone(), &format!("{base_path}/_100btc")),
            _100k_sats: _0satsPattern2::new(client.clone(), &format!("{base_path}/_100k_sats")),
            _100sats: _0satsPattern2::new(client.clone(), &format!("{base_path}/_100sats")),
            _10btc: _0satsPattern2::new(client.clone(), &format!("{base_path}/_10btc")),
            _10k_btc: _0satsPattern2::new(client.clone(), &format!("{base_path}/_10k_btc")),
            _10k_sats: _0satsPattern2::new(client.clone(), &format!("{base_path}/_10k_sats")),
            _10m_sats: _0satsPattern2::new(client.clone(), &format!("{base_path}/_10m_sats")),
            _10sats: _0satsPattern2::new(client.clone(), &format!("{base_path}/_10sats")),
            _1btc: _0satsPattern2::new(client.clone(), &format!("{base_path}/_1btc")),
            _1k_btc: _0satsPattern2::new(client.clone(), &format!("{base_path}/_1k_btc")),
            _1k_sats: _0satsPattern2::new(client.clone(), &format!("{base_path}/_1k_sats")),
            _1m_sats: _0satsPattern2::new(client.clone(), &format!("{base_path}/_1m_sats")),
            _1sat: _0satsPattern2::new(client.clone(), &format!("{base_path}/_1sat")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Stateful_UtxoCohorts_LtAmount {
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

impl CatalogTree_Computed_Stateful_UtxoCohorts_LtAmount {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            _100btc: _0satsPattern2::new(client.clone(), &format!("{base_path}/_100btc")),
            _100k_btc: _0satsPattern2::new(client.clone(), &format!("{base_path}/_100k_btc")),
            _100k_sats: _0satsPattern2::new(client.clone(), &format!("{base_path}/_100k_sats")),
            _100sats: _0satsPattern2::new(client.clone(), &format!("{base_path}/_100sats")),
            _10btc: _0satsPattern2::new(client.clone(), &format!("{base_path}/_10btc")),
            _10k_btc: _0satsPattern2::new(client.clone(), &format!("{base_path}/_10k_btc")),
            _10k_sats: _0satsPattern2::new(client.clone(), &format!("{base_path}/_10k_sats")),
            _10m_sats: _0satsPattern2::new(client.clone(), &format!("{base_path}/_10m_sats")),
            _10sats: _0satsPattern2::new(client.clone(), &format!("{base_path}/_10sats")),
            _1btc: _0satsPattern2::new(client.clone(), &format!("{base_path}/_1btc")),
            _1k_btc: _0satsPattern2::new(client.clone(), &format!("{base_path}/_1k_btc")),
            _1k_sats: _0satsPattern2::new(client.clone(), &format!("{base_path}/_1k_sats")),
            _1m_sats: _0satsPattern2::new(client.clone(), &format!("{base_path}/_1m_sats")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Stateful_UtxoCohorts_MaxAge {
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

impl CatalogTree_Computed_Stateful_UtxoCohorts_MaxAge {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            _10y: UpTo1dPattern::new(client.clone(), &format!("{base_path}/_10y")),
            _12y: UpTo1dPattern::new(client.clone(), &format!("{base_path}/_12y")),
            _15y: UpTo1dPattern::new(client.clone(), &format!("{base_path}/_15y")),
            _1m: UpTo1dPattern::new(client.clone(), &format!("{base_path}/_1m")),
            _1w: UpTo1dPattern::new(client.clone(), &format!("{base_path}/_1w")),
            _1y: UpTo1dPattern::new(client.clone(), &format!("{base_path}/_1y")),
            _2m: UpTo1dPattern::new(client.clone(), &format!("{base_path}/_2m")),
            _2y: UpTo1dPattern::new(client.clone(), &format!("{base_path}/_2y")),
            _3m: UpTo1dPattern::new(client.clone(), &format!("{base_path}/_3m")),
            _3y: UpTo1dPattern::new(client.clone(), &format!("{base_path}/_3y")),
            _4m: UpTo1dPattern::new(client.clone(), &format!("{base_path}/_4m")),
            _4y: UpTo1dPattern::new(client.clone(), &format!("{base_path}/_4y")),
            _5m: UpTo1dPattern::new(client.clone(), &format!("{base_path}/_5m")),
            _5y: UpTo1dPattern::new(client.clone(), &format!("{base_path}/_5y")),
            _6m: UpTo1dPattern::new(client.clone(), &format!("{base_path}/_6m")),
            _6y: UpTo1dPattern::new(client.clone(), &format!("{base_path}/_6y")),
            _7y: UpTo1dPattern::new(client.clone(), &format!("{base_path}/_7y")),
            _8y: UpTo1dPattern::new(client.clone(), &format!("{base_path}/_8y")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Stateful_UtxoCohorts_MinAge {
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

impl CatalogTree_Computed_Stateful_UtxoCohorts_MinAge {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            _10y: _10yTo12yPattern::new(client.clone(), &format!("{base_path}/_10y")),
            _12y: _10yTo12yPattern::new(client.clone(), &format!("{base_path}/_12y")),
            _1d: _10yTo12yPattern::new(client.clone(), &format!("{base_path}/_1d")),
            _1m: _10yTo12yPattern::new(client.clone(), &format!("{base_path}/_1m")),
            _1w: _10yTo12yPattern::new(client.clone(), &format!("{base_path}/_1w")),
            _1y: _10yTo12yPattern::new(client.clone(), &format!("{base_path}/_1y")),
            _2m: _10yTo12yPattern::new(client.clone(), &format!("{base_path}/_2m")),
            _2y: _10yTo12yPattern::new(client.clone(), &format!("{base_path}/_2y")),
            _3m: _10yTo12yPattern::new(client.clone(), &format!("{base_path}/_3m")),
            _3y: _10yTo12yPattern::new(client.clone(), &format!("{base_path}/_3y")),
            _4m: _10yTo12yPattern::new(client.clone(), &format!("{base_path}/_4m")),
            _4y: _10yTo12yPattern::new(client.clone(), &format!("{base_path}/_4y")),
            _5m: _10yTo12yPattern::new(client.clone(), &format!("{base_path}/_5m")),
            _5y: _10yTo12yPattern::new(client.clone(), &format!("{base_path}/_5y")),
            _6m: _10yTo12yPattern::new(client.clone(), &format!("{base_path}/_6m")),
            _6y: _10yTo12yPattern::new(client.clone(), &format!("{base_path}/_6y")),
            _7y: _10yTo12yPattern::new(client.clone(), &format!("{base_path}/_7y")),
            _8y: _10yTo12yPattern::new(client.clone(), &format!("{base_path}/_8y")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Stateful_UtxoCohorts_Term {
    pub long: UpTo1dPattern,
    pub short: UpTo1dPattern,
}

impl CatalogTree_Computed_Stateful_UtxoCohorts_Term {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            long: UpTo1dPattern::new(client.clone(), &format!("{base_path}/long")),
            short: UpTo1dPattern::new(client.clone(), &format!("{base_path}/short")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Stateful_UtxoCohorts_Type {
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

impl CatalogTree_Computed_Stateful_UtxoCohorts_Type {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            empty: _0satsPattern2::new(client.clone(), &format!("{base_path}/empty")),
            p2a: _0satsPattern2::new(client.clone(), &format!("{base_path}/p2a")),
            p2ms: _0satsPattern2::new(client.clone(), &format!("{base_path}/p2ms")),
            p2pk33: _0satsPattern2::new(client.clone(), &format!("{base_path}/p2pk33")),
            p2pk65: _0satsPattern2::new(client.clone(), &format!("{base_path}/p2pk65")),
            p2pkh: _0satsPattern2::new(client.clone(), &format!("{base_path}/p2pkh")),
            p2sh: _0satsPattern2::new(client.clone(), &format!("{base_path}/p2sh")),
            p2tr: _0satsPattern2::new(client.clone(), &format!("{base_path}/p2tr")),
            p2wpkh: _0satsPattern2::new(client.clone(), &format!("{base_path}/p2wpkh")),
            p2wsh: _0satsPattern2::new(client.clone(), &format!("{base_path}/p2wsh")),
            unknown: _0satsPattern2::new(client.clone(), &format!("{base_path}/unknown")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Stateful_UtxoCohorts_Year {
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

impl CatalogTree_Computed_Stateful_UtxoCohorts_Year {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            _2009: _10yTo12yPattern::new(client.clone(), &format!("{base_path}/_2009")),
            _2010: _10yTo12yPattern::new(client.clone(), &format!("{base_path}/_2010")),
            _2011: _10yTo12yPattern::new(client.clone(), &format!("{base_path}/_2011")),
            _2012: _10yTo12yPattern::new(client.clone(), &format!("{base_path}/_2012")),
            _2013: _10yTo12yPattern::new(client.clone(), &format!("{base_path}/_2013")),
            _2014: _10yTo12yPattern::new(client.clone(), &format!("{base_path}/_2014")),
            _2015: _10yTo12yPattern::new(client.clone(), &format!("{base_path}/_2015")),
            _2016: _10yTo12yPattern::new(client.clone(), &format!("{base_path}/_2016")),
            _2017: _10yTo12yPattern::new(client.clone(), &format!("{base_path}/_2017")),
            _2018: _10yTo12yPattern::new(client.clone(), &format!("{base_path}/_2018")),
            _2019: _10yTo12yPattern::new(client.clone(), &format!("{base_path}/_2019")),
            _2020: _10yTo12yPattern::new(client.clone(), &format!("{base_path}/_2020")),
            _2021: _10yTo12yPattern::new(client.clone(), &format!("{base_path}/_2021")),
            _2022: _10yTo12yPattern::new(client.clone(), &format!("{base_path}/_2022")),
            _2023: _10yTo12yPattern::new(client.clone(), &format!("{base_path}/_2023")),
            _2024: _10yTo12yPattern::new(client.clone(), &format!("{base_path}/_2024")),
            _2025: _10yTo12yPattern::new(client.clone(), &format!("{base_path}/_2025")),
            _2026: _10yTo12yPattern::new(client.clone(), &format!("{base_path}/_2026")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Txins {
    pub txoutindex: Indexes24<TxOutIndex>,
    pub value: Indexes24<Sats>,
}

impl CatalogTree_Computed_Txins {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            txoutindex: Indexes24::new(client.clone(), &format!("{base_path}/txoutindex")),
            value: Indexes24::new(client.clone(), &format!("{base_path}/value")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Computed_Txouts {
    pub txinindex: Indexes25<TxInIndex>,
}

impl CatalogTree_Computed_Txouts {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            txinindex: Indexes25::new(client.clone(), &format!("{base_path}/txinindex")),
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
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            address: CatalogTree_Indexed_Address::new(client.clone(), &format!("{base_path}/address")),
            block: CatalogTree_Indexed_Block::new(client.clone(), &format!("{base_path}/block")),
            output: CatalogTree_Indexed_Output::new(client.clone(), &format!("{base_path}/output")),
            tx: CatalogTree_Indexed_Tx::new(client.clone(), &format!("{base_path}/tx")),
            txin: CatalogTree_Indexed_Txin::new(client.clone(), &format!("{base_path}/txin")),
            txout: CatalogTree_Indexed_Txout::new(client.clone(), &format!("{base_path}/txout")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Indexed_Address {
    pub first_p2aaddressindex: Indexes2<P2AAddressIndex>,
    pub first_p2pk33addressindex: Indexes2<P2PK33AddressIndex>,
    pub first_p2pk65addressindex: Indexes2<P2PK65AddressIndex>,
    pub first_p2pkhaddressindex: Indexes2<P2PKHAddressIndex>,
    pub first_p2shaddressindex: Indexes2<P2SHAddressIndex>,
    pub first_p2traddressindex: Indexes2<P2TRAddressIndex>,
    pub first_p2wpkhaddressindex: Indexes2<P2WPKHAddressIndex>,
    pub first_p2wshaddressindex: Indexes2<P2WSHAddressIndex>,
    pub p2abytes: Indexes16<P2ABytes>,
    pub p2pk33bytes: Indexes17<P2PK33Bytes>,
    pub p2pk65bytes: Indexes18<P2PK65Bytes>,
    pub p2pkhbytes: Indexes19<P2PKHBytes>,
    pub p2shbytes: Indexes20<P2SHBytes>,
    pub p2trbytes: Indexes21<P2TRBytes>,
    pub p2wpkhbytes: Indexes22<P2WPKHBytes>,
    pub p2wshbytes: Indexes23<P2WSHBytes>,
}

impl CatalogTree_Indexed_Address {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            first_p2aaddressindex: Indexes2::new(client.clone(), &format!("{base_path}/first_p2aaddressindex")),
            first_p2pk33addressindex: Indexes2::new(client.clone(), &format!("{base_path}/first_p2pk33addressindex")),
            first_p2pk65addressindex: Indexes2::new(client.clone(), &format!("{base_path}/first_p2pk65addressindex")),
            first_p2pkhaddressindex: Indexes2::new(client.clone(), &format!("{base_path}/first_p2pkhaddressindex")),
            first_p2shaddressindex: Indexes2::new(client.clone(), &format!("{base_path}/first_p2shaddressindex")),
            first_p2traddressindex: Indexes2::new(client.clone(), &format!("{base_path}/first_p2traddressindex")),
            first_p2wpkhaddressindex: Indexes2::new(client.clone(), &format!("{base_path}/first_p2wpkhaddressindex")),
            first_p2wshaddressindex: Indexes2::new(client.clone(), &format!("{base_path}/first_p2wshaddressindex")),
            p2abytes: Indexes16::new(client.clone(), &format!("{base_path}/p2abytes")),
            p2pk33bytes: Indexes17::new(client.clone(), &format!("{base_path}/p2pk33bytes")),
            p2pk65bytes: Indexes18::new(client.clone(), &format!("{base_path}/p2pk65bytes")),
            p2pkhbytes: Indexes19::new(client.clone(), &format!("{base_path}/p2pkhbytes")),
            p2shbytes: Indexes20::new(client.clone(), &format!("{base_path}/p2shbytes")),
            p2trbytes: Indexes21::new(client.clone(), &format!("{base_path}/p2trbytes")),
            p2wpkhbytes: Indexes22::new(client.clone(), &format!("{base_path}/p2wpkhbytes")),
            p2wshbytes: Indexes23::new(client.clone(), &format!("{base_path}/p2wshbytes")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Indexed_Block {
    pub blockhash: Indexes2<BlockHash>,
    pub difficulty: Indexes2<StoredF64>,
    pub timestamp: Indexes2<Timestamp>,
    pub total_size: Indexes2<StoredU64>,
    pub weight: Indexes2<Weight>,
}

impl CatalogTree_Indexed_Block {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            blockhash: Indexes2::new(client.clone(), &format!("{base_path}/blockhash")),
            difficulty: Indexes2::new(client.clone(), &format!("{base_path}/difficulty")),
            timestamp: Indexes2::new(client.clone(), &format!("{base_path}/timestamp")),
            total_size: Indexes2::new(client.clone(), &format!("{base_path}/total_size")),
            weight: Indexes2::new(client.clone(), &format!("{base_path}/weight")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Indexed_Output {
    pub first_emptyoutputindex: Indexes2<EmptyOutputIndex>,
    pub first_opreturnindex: Indexes2<OpReturnIndex>,
    pub first_p2msoutputindex: Indexes2<P2MSOutputIndex>,
    pub first_unknownoutputindex: Indexes2<UnknownOutputIndex>,
    pub txindex: MetricNode<TxIndex>,
}

impl CatalogTree_Indexed_Output {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            first_emptyoutputindex: Indexes2::new(client.clone(), &format!("{base_path}/first_emptyoutputindex")),
            first_opreturnindex: Indexes2::new(client.clone(), &format!("{base_path}/first_opreturnindex")),
            first_p2msoutputindex: Indexes2::new(client.clone(), &format!("{base_path}/first_p2msoutputindex")),
            first_unknownoutputindex: Indexes2::new(client.clone(), &format!("{base_path}/first_unknownoutputindex")),
            txindex: MetricNode::new(client.clone(), format!("{base_path}/txindex")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Indexed_Tx {
    pub base_size: Indexes6<StoredU32>,
    pub first_txindex: Indexes2<TxIndex>,
    pub first_txinindex: Indexes6<TxInIndex>,
    pub first_txoutindex: Indexes6<TxOutIndex>,
    pub height: Indexes6<Height>,
    pub is_explicitly_rbf: Indexes6<StoredBool>,
    pub rawlocktime: Indexes6<RawLockTime>,
    pub total_size: Indexes6<StoredU32>,
    pub txid: Indexes6<Txid>,
    pub txversion: Indexes6<TxVersion>,
}

impl CatalogTree_Indexed_Tx {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            base_size: Indexes6::new(client.clone(), &format!("{base_path}/base_size")),
            first_txindex: Indexes2::new(client.clone(), &format!("{base_path}/first_txindex")),
            first_txinindex: Indexes6::new(client.clone(), &format!("{base_path}/first_txinindex")),
            first_txoutindex: Indexes6::new(client.clone(), &format!("{base_path}/first_txoutindex")),
            height: Indexes6::new(client.clone(), &format!("{base_path}/height")),
            is_explicitly_rbf: Indexes6::new(client.clone(), &format!("{base_path}/is_explicitly_rbf")),
            rawlocktime: Indexes6::new(client.clone(), &format!("{base_path}/rawlocktime")),
            total_size: Indexes6::new(client.clone(), &format!("{base_path}/total_size")),
            txid: Indexes6::new(client.clone(), &format!("{base_path}/txid")),
            txversion: Indexes6::new(client.clone(), &format!("{base_path}/txversion")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Indexed_Txin {
    pub first_txinindex: Indexes2<TxInIndex>,
    pub outpoint: Indexes24<OutPoint>,
    pub outputtype: Indexes24<OutputType>,
    pub txindex: Indexes24<TxIndex>,
    pub typeindex: Indexes24<TypeIndex>,
}

impl CatalogTree_Indexed_Txin {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            first_txinindex: Indexes2::new(client.clone(), &format!("{base_path}/first_txinindex")),
            outpoint: Indexes24::new(client.clone(), &format!("{base_path}/outpoint")),
            outputtype: Indexes24::new(client.clone(), &format!("{base_path}/outputtype")),
            txindex: Indexes24::new(client.clone(), &format!("{base_path}/txindex")),
            typeindex: Indexes24::new(client.clone(), &format!("{base_path}/typeindex")),
        }
    }
}

/// Catalog tree node.
pub struct CatalogTree_Indexed_Txout {
    pub first_txoutindex: Indexes2<TxOutIndex>,
    pub outputtype: Indexes25<OutputType>,
    pub txindex: Indexes25<TxIndex>,
    pub typeindex: Indexes25<TypeIndex>,
    pub value: Indexes25<Sats>,
}

impl CatalogTree_Indexed_Txout {
    pub fn new(client: Arc<BrkClientBase>, base_path: &str) -> Self {
        Self {
            first_txoutindex: Indexes2::new(client.clone(), &format!("{base_path}/first_txoutindex")),
            outputtype: Indexes25::new(client.clone(), &format!("{base_path}/outputtype")),
            txindex: Indexes25::new(client.clone(), &format!("{base_path}/txindex")),
            typeindex: Indexes25::new(client.clone(), &format!("{base_path}/typeindex")),
            value: Indexes25::new(client.clone(), &format!("{base_path}/value")),
        }
    }
}

/// Main BRK client with catalog tree and API methods.
pub struct BrkClient {
    base: Arc<BrkClientBase>,
    tree: CatalogTree,
}

impl BrkClient {
    /// Create a new client with the given base URL.
    pub fn new(base_url: impl Into<String>) -> Self {
        let base = Arc::new(BrkClientBase::new(base_url));
        let tree = CatalogTree::new(base.clone(), "");
        Self { base, tree }
    }

    /// Create a new client with options.
    pub fn with_options(options: BrkClientOptions) -> Self {
        let base = Arc::new(BrkClientBase::with_options(options));
        let tree = CatalogTree::new(base.clone(), "");
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
