pub trait Printable {
    fn to_string() -> &'static str;
    fn to_possible_strings() -> &'static [&'static str];
}

impl Printable for usize {
    fn to_string() -> &'static str {
        "usize"
    }

    fn to_possible_strings() -> &'static [&'static str] {
        &["usize"]
    }
}
