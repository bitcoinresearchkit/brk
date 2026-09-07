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

impl<T> ChangeData<T> {
    /// Re-queues truncated values where disk stops agreeing with the rolled-back
    /// state. Query persisted length only when there are truncated values.
    pub fn into_rollback(self, real_stored_len: impl FnOnce() -> usize) -> (Stamp, usize, Vec<T>) {
        let (stored_len, pushed) = if self.truncated_values.is_empty() {
            (self.prev_stored_len, self.prev_pushed)
        } else {
            let agree_at = self.truncated_start.min(real_stored_len());
            let mut pushed = self.truncated_values;
            pushed.extend(self.prev_pushed);
            (agree_at, pushed)
        };
        (self.prev_stamp, stored_len, pushed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn append_only_rollback_does_not_query_persisted_length() {
        let change = ChangeData {
            prev_stamp: Stamp::new(7),
            prev_stored_len: 5,
            truncated_start: 5,
            truncated_values: Vec::<String>::new(),
            prev_pushed: vec!["pending".into()],
        };
        let (stamp, len, pushed) = change.into_rollback(|| panic!("unneeded length query"));
        assert_eq!(stamp, Stamp::new(7));
        assert_eq!(len, 5);
        assert_eq!(pushed, ["pending"]);
    }

    #[test]
    fn truncated_rollback_clamps_to_disk_and_preserves_value_order() {
        for persisted_len in [0, 2, 5, 10] {
            let change = ChangeData {
                prev_stamp: Stamp::new(7),
                prev_stored_len: 5,
                truncated_start: 3,
                truncated_values: vec!["first".to_owned(), "second".into()],
                prev_pushed: vec!["pending".into()],
            };
            let mut queries = 0;
            let (stamp, len, pushed) = change.into_rollback(|| {
                queries += 1;
                persisted_len
            });
            assert_eq!(queries, 1);
            assert_eq!(stamp, Stamp::new(7));
            assert_eq!(len, 3.min(persisted_len));
            assert_eq!(pushed, ["first", "second", "pending"]);
        }
    }
}
