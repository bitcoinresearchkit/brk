use super::*;

#[test]
fn brace_escaping_matches_the_original_replacements() {
    let fragments = [
        "", "{", "}", "{{", "}}", "{disc}", "{{disc}}", "disc", "{other}", "é", "🙂", "_",
    ];
    for a in fragments {
        for b in fragments {
            for c in fragments {
                let input = format!("{a}{b}{c}");
                let expected = input
                    .replace('{', "{{")
                    .replace('}', "}}")
                    .replace("{{disc}}", "{disc}");
                assert_eq!(escape_rust_format(&input), expected, "{input}");
            }
        }
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
