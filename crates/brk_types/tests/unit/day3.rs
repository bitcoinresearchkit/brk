use jiff::Span;

use super::*;
use crate::Index;

#[test]
fn timestamp_boundaries_do_not_underflow() {
    let epoch = INDEX_EPOCH - 86_400;
    for (seconds, expected) in [
        (0, 0),
        (epoch - 1, 0),
        (epoch, 0),
        (INDEX_EPOCH - 1, 0),
        (epoch + DAY3_INTERVAL - 1, 0),
        (epoch + DAY3_INTERVAL, 1),
        (u32::MAX, (u32::MAX - epoch) / DAY3_INTERVAL),
    ] {
        let timestamp = Timestamp::new(seconds);
        assert_eq!(
            usize::from(Day3::from_timestamp(timestamp)),
            expected as usize
        );
        assert_eq!(
            Index::Day3.timestamp_to_index(timestamp),
            Some(expected as usize)
        );
    }
}

#[test]
fn dates_preserve_bucket_boundaries_without_timestamp_narrowing() {
    for days in [-2_i64, -1, 0, 1, 2, 65_536, 196_604, 196_606, 196_607] {
        let date = Date::from(
            Date::INDEX_ZERO_
                .checked_add(Span::new().days(days))
                .unwrap(),
        );
        let expected = if (-1..196_607).contains(&days) {
            Some(((days + 1) / 3) as usize)
        } else {
            None
        };
        assert_eq!(Day3::try_from(date).ok().map(usize::from), expected);
        assert_eq!(
            Index::Day3.date_to_index(date),
            if days < 0 { None } else { expected }
        );
    }
}
