#[derive(Clone, Copy)]
pub struct RequestBody {
    pub value_name: &'static str,
    pub required: bool,
    pub content_type: &'static str,
}
