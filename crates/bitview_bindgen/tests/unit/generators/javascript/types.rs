use super::jsdoc_normalize;

#[test]
fn openapi_integer_types_reach_get_and_post_jsdoc_as_numbers() {
    use crate::{
        generators::javascript::api::generate_api_methods,
        openapi::{extract_endpoints, parse_openapi_json},
    };

    let spec = parse_openapi_json(r#"{
        "openapi":"3.1.0", "info":{"title":"Test","version":"1"},
        "paths":{"/api/value/{height}":{
            "get":{
                "operationId":"get_value",
                "parameters":[
                    {"name":"height","in":"path","required":true,"schema":{"type":"integer"}},
                    {"name":"count","in":"query","required":false,"schema":{"type":"integer"}}
                ],
                "responses":{"200":{"description":"Success","content":{"application/json":{"schema":{"type":"integer"}}}}}
            },
            "post":{
                "operationId":"post_value",
                "parameters":[{"name":"height","in":"path","required":true,"schema":{"type":"integer"}}],
                "requestBody":{"required":true,"content":{"application/json":{"schema":{"type":"array","items":{"type":"integer"}}}}},
                "responses":{"200":{"description":"Success","content":{"application/json":{"schema":{"type":"integer"}}}}}
            }
        }}
    }"#).unwrap();
    let mut output = String::new();
    generate_api_methods(&mut output, &extract_endpoints(&spec));
    assert_eq!(
        output.matches("@param {number} height").count(),
        2,
        "{output}"
    );
    assert!(output.contains("@param {number=} [count]"), "{output}");
    assert!(output.contains("@param {number[]} body"), "{output}");
    assert_eq!(
        output.matches("@returns {Promise<number>}").count(),
        2,
        "{output}"
    );
    assert!(!output.contains("integer"), "{output}");
}

fn original(ty: &str) -> String {
    let mut out = ty.to_string();
    let mut prev = String::new();
    while prev != out {
        prev = out.clone();
        out = out.replace("integer[]", "number[]");
        out = out.replace("<integer>", "<number>");
        out = out.replace("(integer)", "(number)");
        out = out.replace("integer | ", "number | ");
        out = out.replace(" | integer", " | number");
    }
    if out == "integer" {
        "number".to_string()
    } else {
        out
    }
}

#[test]
fn normalization_matches_original_for_nested_and_adjacent_fragments() {
    let fragments = [
        "",
        "integer",
        "integer[]",
        "<integer>",
        "(integer)",
        "integer | ",
        " | integer",
        "Foo",
        "number",
        "[]",
        "<",
        ">",
        "(",
        ")",
        " | ",
        "é",
    ];
    for a in fragments {
        for b in fragments {
            for c in fragments {
                let input = format!("{a}{b}{c}");
                assert_eq!(jsdoc_normalize(&input), original(&input), "{input}");
            }
        }
    }
    for (input, expected) in [
        ("integer", "number"),
        ("Foo<integer>[] | integer", "Foo<number>[] | number"),
        ("BigInteger", "BigInteger"),
        ("integerish", "integerish"),
    ] {
        assert_eq!(jsdoc_normalize(input), expected);
    }
}
