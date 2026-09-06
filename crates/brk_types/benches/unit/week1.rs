use std::{hint::black_box, time::Instant};

use jiff::Span;

use super::Week1;
use crate::{Date, Day1, Index};

// ISO week 2009-W01 begins on Monday, three days before INDEX_ZERO.
fn direct(date: Date) -> Option<Week1> {
    Week1::try_from(date).ok()
}

fn old(date: Date) -> Week1 {
    let date = date.into_jiff().iso_week_date();
    let mut week = 0_u16;
    for year in 2009..date.year() {
        week += Date::new(year as u16, 6, 6)
            .into_jiff()
            .iso_week_date()
            .weeks_in_year() as u16;
    }
    week += date.week() as u16;
    Week1::from(week - 1)
}

fn reference(date: Date) -> u32 {
    let iso = date.into_jiff().iso_week_date();
    let mut weeks = iso.week() as u32 - 1;
    for year in 2009..iso.year() {
        weeks += Date::new(year as u16, 6, 6)
            .into_jiff()
            .iso_week_date()
            .weeks_in_year() as u32;
    }
    weeks
}

#[test]
fn direct_weeks_match_iso_boundaries() {
    for year in 2009..=3264 {
        // Include ISO year transitions and leap-year boundaries.
        for (month, day) in [(1, 1), (1, 4), (1, 5), (2, 28), (3, 1), (12, 28), (12, 31)] {
            let date = Date::new(year, month, day);
            assert_eq!(
                direct(date).map(u16::from).map(u32::from),
                Some(reference(date)),
                "{date}"
            );
        }
    }
    for days in 0..11_323 {
        let date = Date::from(
            Date::INDEX_ZERO_
                .checked_add(Span::new().days(days))
                .unwrap(),
        );
        assert_eq!(direct(date), Some(old(date)), "{date}");
        assert_eq!(
            direct(date),
            Some(Week1::from(Day1::from(days as usize))),
            "{date}"
        );
    }
    for days in [-4, -3, 458_748, 458_749] {
        let date = Date::from(
            Date::INDEX_ZERO_
                .checked_add(Span::new().days(days))
                .unwrap(),
        );
        let expected = match days {
            -3 => Some(Week1::from(0_u16)),
            458_748 => Some(Week1::from(u16::MAX)),
            _ => None,
        };
        assert_eq!(direct(date), expected, "{date}");
        assert_eq!(
            Index::Week1.date_to_index(date),
            if days < 0 {
                None
            } else {
                expected.map(usize::from)
            }
        );
    }
}

#[test]
#[ignore = "weekly conversion comparison; no storage, server, or RPC"]
fn benchmark_week_conversion() {
    const BATCH: u32 = 10_000;
    for year in [2009, 2026, 2106, 3000] {
        let date = Date::new(year, 12, 31);
        let expected = old(date);
        assert_eq!(direct(date), Some(expected));
        let mut samples = [Vec::new(), Vec::new()];
        for round in 0..12 {
            for offset in 0..2 {
                let variant = (round + offset) % 2;
                let start = Instant::now();
                for _ in 0..BATCH {
                    let date = black_box(date);
                    black_box(if variant == 0 {
                        Some(old(date))
                    } else {
                        direct(date)
                    });
                }
                if round >= 2 {
                    samples[variant].push(start.elapsed() / BATCH);
                }
            }
        }
        for sample in &mut samples {
            sample.sort_unstable();
        }
        eprintln!(
            "year={year} old={:?} direct={:?}",
            samples[0][5], samples[1][5]
        );
    }
}
