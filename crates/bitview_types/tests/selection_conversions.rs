use bitview_types::{
    DataRangeFormat, SeriesList, SeriesName, SeriesNameWithIndex, SeriesSelection,
};
use brk_types::Index;

#[test]
fn single_series_conversion_preserves_all_range_fields() {
    for json in [
        "{}",
        r#"{"start":"-10","end":"42","limit":7,"format":"csv"}"#,
    ] {
        let scalar = SeriesSelection::from((
            Index::Height,
            SeriesName::from("price_close"),
            serde_json::from_str::<DataRangeFormat>(json).unwrap(),
        ));
        let list = SeriesSelection::from((
            Index::Height,
            SeriesList::from(SeriesName::from("price_close")),
            serde_json::from_str::<DataRangeFormat>(json).unwrap(),
        ));
        assert_eq!(scalar.index, list.index);
        assert_eq!(scalar.series.to_string(), list.series.to_string());
        assert_eq!(
            scalar.start().map(|index| format!("{index:?}")),
            list.start().map(|index| format!("{index:?}"))
        );
        assert_eq!(
            scalar.end().map(|index| format!("{index:?}")),
            list.end().map(|index| format!("{index:?}"))
        );
        assert_eq!(
            scalar.limit().map(|limit| *limit),
            list.limit().map(|limit| *limit)
        );
        assert_eq!(scalar.format(), list.format());
    }
}

#[test]
fn indexed_name_conversions_use_the_same_wire_shape() {
    let expected = SeriesNameWithIndex::new("price_close", Index::Height);
    for converted in [
        SeriesNameWithIndex::from(("price_close", Index::Height)),
        SeriesNameWithIndex::from((SeriesName::from("price_close"), Index::Height)),
    ] {
        assert_eq!(
            serde_json::to_value(converted).unwrap(),
            serde_json::to_value(&expected).unwrap(),
        );
    }
}
