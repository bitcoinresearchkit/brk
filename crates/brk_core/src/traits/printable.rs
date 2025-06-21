pub trait Printable {
    fn to_string() -> &'static str;
    fn to_possible_strings() -> &'static [&'static str];
}
