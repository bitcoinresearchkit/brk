use bitview_traversable::Traversable;

/// A Bitcoin amount and its fiat value, with independently chosen representations.
#[derive(Clone, Traversable)]
pub struct Value<S, C, B, U> {
    /// Reported in BTC; one BTC equals 100,000,000 satoshis.
    pub btc: B,
    /// Reported in satoshis.
    pub sats: S,
    /// Reported in US dollars.
    pub usd: U,
    /// Reported in US cents; 100 cents equal one US dollar.
    pub cents: C,
}
