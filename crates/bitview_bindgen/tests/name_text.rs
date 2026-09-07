use bitview_bindgen::{
    GenericSyntax, escape_python_keyword, extract_inner_type, find_common_prefix,
    find_common_suffix, infer_accumulated_name, to_camel_case, to_pascal_case, to_snake_case,
};

fn prefix_reference(names: &[&str]) -> Option<String> {
    let first = *names.first()?;
    let end = first
        .char_indices()
        .map(|(index, ch)| index + ch.len_utf8())
        .rfind(|&end| names.iter().all(|name| name.starts_with(&first[..end])))?;
    let raw = &first[..end];
    if raw.ends_with('_') {
        Some(raw.into())
    } else if names.contains(&raw) {
        Some(format!("{raw}_"))
    } else {
        raw.rfind('_').map(|index| raw[..=index].into())
    }
}

fn suffix_reference(names: &[&str]) -> Option<String> {
    let first = *names.first()?;
    let start = first
        .char_indices()
        .map(|(index, _)| index)
        .find(|&start| names.iter().all(|name| name.ends_with(&first[start..])))?;
    let raw = &first[start..];
    if raw.starts_with('_') {
        Some(raw.into())
    } else if names.iter().all(|name| {
        *name == raw
            || name
                .strip_suffix(raw)
                .is_some_and(|prefix| prefix.ends_with('_'))
    }) {
        Some(format!("_{raw}"))
    } else {
        raw.find('_').map(|index| raw[index..].into())
    }
}

#[test]
fn prefix_suffix_match_word_boundary_oracles_for_ascii_and_unicode() {
    let names = [
        "",
        "a",
        "_",
        "a_b",
        "a_bc",
        "b_a",
        "é",
        "ê",
        "é_a",
        "供給",
        "a_供給",
        "_é",
        "Σ",
        "a_Σ",
        "🙂",
        "🙂_",
        "foo_bar_x",
        "foo_bar_y",
    ];
    for a in names {
        assert_eq!(find_common_prefix(&[a]), prefix_reference(&[a]));
        assert_eq!(find_common_suffix(&[a]), suffix_reference(&[a]));
        for b in names {
            for c in names {
                let input = [a, b, c];
                assert_eq!(
                    find_common_prefix(&input),
                    prefix_reference(&input),
                    "prefix {input:?}"
                );
                assert_eq!(
                    find_common_suffix(&input),
                    suffix_reference(&input),
                    "suffix {input:?}"
                );
            }
        }
    }
    assert_eq!(find_common_prefix(&[]), None);
    assert_eq!(find_common_suffix(&[]), None);
    assert_eq!(
        find_common_prefix(&["é_price", "é_supply"]),
        Some("é_".into())
    );
    assert_eq!(
        find_common_suffix(&["sth_供給", "lth_供給"]),
        Some("_供給".into())
    );
    assert_eq!(find_common_prefix(&["é", "ê"]), None);
    assert_eq!(find_common_suffix(&["é", "ê"]), None);
}

fn pascal_reference(value: &str) -> String {
    value
        .replace('-', "_")
        .split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect()
}

fn camel_reference(value: &str) -> String {
    let pascal = pascal_reference(value);
    let mut chars = pascal.chars();
    let result = match chars.next() {
        None => String::new(),
        Some(first) => first.to_lowercase().collect::<String>() + chars.as_str(),
    };
    if result.chars().next().is_some_and(|c| c.is_ascii_digit()) {
        format!("_{result}")
    } else {
        result
    }
}

#[test]
fn casing_preserves_unicode_expansion_context_and_digit_rules() {
    let fragments = [
        "", "_", "-", "a", "AB", "7", "ß", "İ", "Σ", "ΟΣ", "é", "ﬃ", "[", "]", "🙂", "class",
    ];
    for a in fragments {
        for b in fragments {
            for c in fragments {
                let input = format!("{a}{b}{c}");
                assert_eq!(to_pascal_case(&input), pascal_reference(&input), "{input}");
                assert_eq!(to_camel_case(&input), camel_reference(&input), "{input}");
                let mut expected_snake = input.to_lowercase().replace('-', "_");
                if expected_snake.starts_with(|c: char| c.is_ascii_digit()) {
                    expected_snake = format!("_{expected_snake}");
                }
                assert_eq!(to_snake_case(&input), expected_snake, "{input}");
            }
        }
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
fn accumulated_names_keep_existing_prefix_and_fallback_rules() {
    for parent in ["", "parent", "é"] {
        for field in ["", "price", "供給"] {
            for descendant in [
                "",
                "price",
                "price_close",
                "foo_price",
                "é_price",
                "供給_close",
            ] {
                let expected = if descendant.starts_with(field) || parent.is_empty() {
                    field.to_string()
                } else {
                    format!("{parent}_{field}")
                };
                assert_eq!(infer_accumulated_name(parent, field, descendant), expected);
            }
        }
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
    for (input, rust, js, python) in [
        ("Foo<Bar<Cents>>", "Cents", "Cents", "Cents"),
        (
            "[[Cents; 2]; 3]",
            "[[Cents; 2]; 3]",
            "Cents[][]",
            "List[List[Cents]]",
        ),
        ("[Cents; 0]", "[Cents; 0]", "Cents[]", "List[Cents]"),
        ("[Cents; x]", "[Cents; x]", "[Cents; x]", "[Cents; x]"),
    ] {
        assert_eq!(GenericSyntax::RUST.convert(input), rust);
        assert_eq!(GenericSyntax::JAVASCRIPT.convert(input), js);
        assert_eq!(GenericSyntax::PYTHON.convert(input), python);
    }
}

#[test]
fn template_backends_keep_their_distinct_identity_and_discriminator_rules() {
    use bitview_bindgen::{JavaScriptSyntax, LanguageSyntax, PythonSyntax, RustSyntax};

    for (template, js, python, rust) in [
        ("", "_m(baseName, disc)", "_m(base_name, '')", "base_name"),
        (
            "{disc}",
            "_m(baseName, disc)",
            "_m(base_name, disc)",
            "_m(&base_name, &disc)",
        ),
        (
            "raw",
            "_m(baseName, 'raw')",
            "_m(base_name, 'raw')",
            "_m(&base_name, \"raw\")",
        ),
        (
            "ratio_{disc}",
            "_m(_m(baseName, 'ratio'), disc)",
            "_m(_m(base_name, 'ratio'), disc)",
            "_m(&_m(&base_name, \"ratio\"), &disc)",
        ),
        (
            "_{disc}",
            "_m(_m(baseName, ''), disc)",
            "_m(_m(base_name, ''), disc)",
            "_m(&_m(&base_name, \"\"), &disc)",
        ),
        (
            "{disc}{disc}",
            "_m(_m(baseName, '{disc}'), disc)",
            "_m(_m(base_name, '{disc}'), disc)",
            "_m(&_m(&base_name, \"{disc}\"), &disc)",
        ),
    ] {
        assert_eq!(JavaScriptSyntax.template_expr("base_name", template), js);
        assert_eq!(PythonSyntax.template_expr("base_name", template), python);
        assert_eq!(RustSyntax.template_expr("base_name", template), rust);
    }
}
