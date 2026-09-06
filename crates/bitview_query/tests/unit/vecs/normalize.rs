use super::normalize;

#[test]
fn preserves_decimal_numbers_and_comparison_symbols() {
    assert_eq!(normalize("Realized price."), "realized price");
    assert_eq!(normalize("Price... then 0.1 BTC."), "price then 0.1 btc");
    assert_eq!(
        normalize("STH_realized-price/ratio"),
        "sth realized price ratio"
    );
    assert_eq!(normalize(">=10% supply"), ">=10% supply");
}
