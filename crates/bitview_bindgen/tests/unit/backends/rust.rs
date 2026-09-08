use super::*;

#[test]
fn format_escaping_keeps_only_the_discriminator_live() {
    for (input, expected) in [
        ("", ""),
        ("{disc}", "{disc}"),
        ("{other}", "{{other}}"),
        ("{{disc}}", "{{{disc}}}"),
        ("é{disc}🙂", "é{disc}🙂"),
    ] {
        assert_eq!(escape_rust_format(input), expected);
    }
    assert_eq!(
        RustSyntax.disc_arg_expr("ratio_{disc}_ppm"),
        "format!(\"ratio_{disc}_ppm\")"
    );
    assert_eq!(
        RustSyntax.template_expr("acc", "ratio_{disc}_ppm"),
        "_m(&acc, &format!(\"ratio_{disc}_ppm\", disc=disc))"
    );
}
