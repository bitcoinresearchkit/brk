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

#[test]
fn collapses_separators_without_leading_or_trailing_spaces() {
    for (input, expected) in [
        ("", ""),
        (" _/—\t\n", ""),
        ("  STH___price / ratio  ", "sth price ratio"),
        ("Price€USD", "price usd"),
        (".1 1. 0.01 1..2", "1 1 0.01 1 2"),
        (" <10% + >=20% ", "<10% + >=20%"),
    ] {
        assert_eq!(normalize(input), expected, "{input:?}");
    }
}
