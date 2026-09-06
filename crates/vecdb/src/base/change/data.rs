use crate::Stamp;

/// Parsed change data returned by parse_change_data, consumed by apply_rollback.
#[derive(Debug)]
pub struct ChangeData<T> {
    pub prev_stamp: Stamp,
    pub prev_stored_len: usize,
    pub truncated_start: usize,
    pub truncated_values: Vec<T>,
    pub prev_pushed: Vec<T>,
}
