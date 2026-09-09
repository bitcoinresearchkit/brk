use bitview_bindgen::{
    GenericSyntax, escape_python_keyword, extract_inner_type, find_common_prefix,
    find_common_suffix, to_camel_case, to_pascal_case, to_snake_case,
};

#[test]
fn names_respect_word_boundaries_and_unicode() {
    for (names, prefix, suffix) in [
        (&[][..], None, None),
        (&[""][..], None, None),
        (&["price", "price_close"][..], Some("price_"), None),
        (&["foo_bar_x", "foo_bar_y"][..], Some("foo_bar_"), None),
        (&["é_price", "é_supply"][..], Some("é_"), None),
        (&["sth_供給", "lth_供給"][..], None, Some("_供給")),
        (&["é", "ê"][..], None, None),
        (&["price", "surprise"][..], None, None),
    ] {
        assert_eq!(find_common_prefix(names).as_deref(), prefix);
        assert_eq!(find_common_suffix(names).as_deref(), suffix);
    }
}

#[test]
fn casing_preserves_unicode_expansion_and_identifier_rules() {
    for (input, pascal, camel, snake) in [
        ("", "", "", ""),
        ("price-close", "PriceClose", "priceClose", "price_close"),
        ("7_blocks", "7Blocks", "_7Blocks", "_7_blocks"),
        ("ß_id", "SSId", "sSId", "ß_id"),
        ("İ_value", "İValue", "i\u{0307}Value", "i\u{0307}_value"),
        ("ΟΣ", "ΟΣ", "οΣ", "ος"),
    ] {
        assert_eq!(to_pascal_case(input), pascal);
        assert_eq!(to_camel_case(input), camel);
        assert_eq!(to_snake_case(input), snake);
    }
    for (input, expected) in [
        ("class", "class_"),
        ("None", "None_"),
        ("txId[]", "txId"),
        ("[7]", "_7"),
        ("[class]", "class_"),
        ("é", "é"),
        ("", ""),
    ] {
        assert_eq!(escape_python_keyword(input), expected);
    }
}

#[test]
fn inner_type_and_language_arrays_keep_owned_public_results() {
    for (input, expected) in [
        ("Cents", "Cents"),
        ("Close<Cents>", "Cents"),
        ("Foo<Bar<Cents>>", "Bar<Cents>"),
        ("Cents>>", "Cents"),
        ("Foo<", "Foo<"),
        ("<>", ""),
        (">Foo<", ">Foo<"),
    ] {
        assert_eq!(extract_inner_type(input), expected);
    }
    for (input, js, python) in [
        ("Foo<Bar<Cents>>", "Cents", "Cents"),
        ("[[Cents; 2]; 3]", "Cents[][]", "List[List[Cents]]"),
        ("[Cents; 0]", "Cents[]", "List[Cents]"),
        ("[Cents; x]", "[Cents; x]", "[Cents; x]"),
    ] {
        assert_eq!(GenericSyntax::JAVASCRIPT.convert(input), js);
        assert_eq!(GenericSyntax::PYTHON.convert(input), python);
    }
}

#[test]
fn template_backends_keep_their_distinct_identity_and_discriminator_rules() {
    use bitview_bindgen::{JavaScriptSyntax, LanguageSyntax, PythonSyntax};

    for (template, js, python) in [
        ("", "_m(baseName, disc)", "_m(base_name, '')"),
        ("{disc}", "_m(baseName, disc)", "_m(base_name, disc)"),
        ("raw", "_m(baseName, 'raw')", "_m(base_name, 'raw')"),
        (
            "ratio_{disc}",
            "_m(_m(baseName, 'ratio'), disc)",
            "_m(_m(base_name, 'ratio'), disc)",
        ),
        (
            "_{disc}",
            "_m(_m(baseName, ''), disc)",
            "_m(_m(base_name, ''), disc)",
        ),
        (
            "{disc}{disc}",
            "_m(_m(baseName, '{disc}'), disc)",
            "_m(_m(base_name, '{disc}'), disc)",
        ),
    ] {
        assert_eq!(JavaScriptSyntax.template_expr("base_name", template), js);
        assert_eq!(PythonSyntax.template_expr("base_name", template), python);
    }
}
