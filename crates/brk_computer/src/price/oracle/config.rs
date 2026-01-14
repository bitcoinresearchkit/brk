//! Era-based configuration for the UTXOracle algorithm.
//! Different time periods require different price bounds and aggregation windows
//! Due to varying transaction volumes and price levels.

/// Configuration for a specific era
#[derive(Debug, Clone, Copy)]
pub struct OracleConfig {
    /// Minimum expected price in cents (e.g., 10 = $0.10)
    pub min_price_cents: u64,
    /// Maximum expected price in cents (e.g., 100_000_000 = $1,000,000)
    pub max_price_cents: u64,
    /// Number of blocks to aggregate for sufficient sample size
    pub blocks_per_window: u32,
    /// Minimum qualifying transactions needed for a valid estimate
    pub min_tx_count: u32,
}

impl OracleConfig {
    /// Get configuration for a given year
    pub fn for_year(year: u16) -> Self {
        match year {
            // 2009-2010: Very early Bitcoin, extremely low volume and prices
            // Price: $0 - ~$0.10, very few transactions
            2009..=2010 => Self {
                min_price_cents: 1,      // $0.01
                max_price_cents: 100,    // $1.00
                blocks_per_window: 2016, // ~2 weeks
                min_tx_count: 50,
            },
            // 2011: First major price movements ($0.30 - $30)
            2011 => Self {
                min_price_cents: 10,     // $0.10
                max_price_cents: 10_000, // $100
                blocks_per_window: 1008, // ~1 week
                min_tx_count: 100,
            },
            // 2012-2013: Growing adoption ($5 - $1,200)
            2012..=2013 => Self {
                min_price_cents: 100,     // $1
                max_price_cents: 200_000, // $2,000
                blocks_per_window: 288,   // ~2 days
                min_tx_count: 500,
            },
            // 2014-2016: Post-bubble consolidation ($200 - $1,000)
            2014..=2016 => Self {
                min_price_cents: 10_000,    // $100
                max_price_cents: 2_000_000, // $20,000
                blocks_per_window: 144,     // ~1 day
                min_tx_count: 1000,
            },
            // 2017+: Modern era ($10,000 - $500,000)
            // Matches Python's slide range of -141 to 201
            _ => Self {
                min_price_cents: 1_000_000,  // $10,000 (gives max_slide = 200)
                max_price_cents: 50_000_000, // $500,000 (gives min_slide ≈ -140)
                blocks_per_window: 144,      // ~1 day
                min_tx_count: 2000,
            },
        }
    }

    /// Convert price bounds to histogram slide range
    /// Returns (min_slide, max_slide) for stencil positioning
    ///
    /// The stencil center (bin 600) corresponds to 0.001 BTC.
    /// At $100,000/BTC, 0.001 BTC = $100, so position 0 = $100,000/BTC.
    ///
    /// For a given price P (in cents/BTC):
    /// - $100 USD = 10000/P BTC
    /// - The histogram bin for $100 shifts based on price
    /// - slide = (7 - log10(P)) * 200
    ///
    /// Higher prices → lower (negative) slides
    /// Lower prices → higher (positive) slides
    pub fn slide_range(&self) -> (i32, i32) {
        let min_log = (self.min_price_cents as f64).log10();
        let max_log = (self.max_price_cents as f64).log10();

        // min_slide corresponds to max_price (higher price = more negative slide)
        // max_slide corresponds to min_price (lower price = more positive slide)
        let min_slide = ((7.0 - max_log) * 200.0) as i32;
        let max_slide = ((7.0 - min_log) * 200.0) as i32;

        (min_slide, max_slide)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_for_year() {
        // 2017+ config matches Python: $10,000 to $500,000
        let c2020 = OracleConfig::for_year(2020);
        assert_eq!(c2020.min_price_cents, 1_000_000);
        assert_eq!(c2020.max_price_cents, 50_000_000);

        let c2015 = OracleConfig::for_year(2015);
        assert_eq!(c2015.min_price_cents, 10_000);
        assert_eq!(c2015.max_price_cents, 2_000_000);
    }

    #[test]
    fn test_slide_range() {
        // 2024 config: $10,000 to $500,000 (matches Python's -141 to 201)
        let config = OracleConfig::for_year(2024);
        let (min, max) = config.slide_range();
        // $500,000 = 5*10^7 cents → slide = (7-7.699)*200 ≈ -140
        // $10,000 = 10^6 cents → slide = (7-6)*200 = 200
        assert!((-141..=-139).contains(&min)); // ~-140
        assert_eq!(max, 200);

        // 2015 config: $100 to $20,000
        let config = OracleConfig::for_year(2015);
        let (min, max) = config.slide_range();
        // $20,000 = 2*10^6 cents → slide = (7-6.3)*200 ≈ 140
        // $100 = 10^4 cents → slide = (7-4)*200 = 600
        assert!(min > 100 && min < 200); // ~140
        assert_eq!(max, 600);
    }
}
