use super::*;
use std::{hint::black_box, time::Instant};

fn intersect_dates(left: Vec<Date>, right: Vec<Date>) -> Vec<Date> {
    let mut left = left.into_iter().peekable();
    let mut right = right.into_iter().peekable();
    let mut intersection = Vec::new();

    while let (Some(&a), Some(&b)) = (left.peek(), right.peek()) {
        match a.cmp(&b) {
            Ordering::Less => {
                left.next();
            }
            Ordering::Greater => {
                right.next();
            }
            Ordering::Equal => {
                intersection.push(a);
                left.next();
                right.next();
            }
        }
    }

    intersection
}

fn collected_intersection(left: Vec<Date>, right: Vec<Date>) -> Vec<Date> {
    let mut dates = Vec::new();
    visit_intersection(left, right, |date| dates.push(date));
    dates
}

fn last_intersection(left: Vec<Date>, right: Vec<Date>) -> Option<Date> {
    let mut last = None;
    visit_intersection(left, right, |date| last = Some(date));
    last
}

#[test]
#[ignore = "sorted URPD date intersection comparison; excludes directory scanning"]
fn benchmark_date_intersection() {
    for count in [0, 1, 32, 512, 4096] {
        let left: Vec<_> = (0..count)
            .map(|n| {
                Date::new(
                    2009 + (n / 336) as u16,
                    (n / 28 % 12 + 1) as u8,
                    (n % 28 + 1) as u8,
                )
            })
            .collect();
        for (name, right) in [
            ("all", left.clone()),
            ("half", left.iter().step_by(2).copied().collect()),
            ("none", Vec::new()),
        ] {
            assert_eq!(
                intersect_dates(left.clone(), right.clone()),
                collected_intersection(left.clone(), right.clone())
            );
            assert_eq!(
                intersect_dates(left.clone(), right.clone()).last().copied(),
                last_intersection(left.clone(), right.clone())
            );
            let mut samples = [Vec::new(), Vec::new(), Vec::new(), Vec::new()];
            for round in 0..12 {
                for offset in 0..4 {
                    let variant = (round + offset) % 4;
                    let start = Instant::now();
                    for _ in 0..1000 {
                        let left = black_box(&left).clone();
                        let right = black_box(&right).clone();
                        match variant {
                            0 => {
                                black_box(intersect_dates(left, right));
                            }
                            1 => {
                                black_box(collected_intersection(left, right));
                            }
                            2 => {
                                black_box(intersect_dates(left, right).last().copied());
                            }
                            3 => {
                                black_box(last_intersection(left, right));
                            }
                            _ => unreachable!(),
                        }
                    }
                    if round >= 2 {
                        samples[variant].push(start.elapsed() / 1000);
                    }
                }
            }
            for sample in &mut samples {
                sample.sort_unstable();
            }
            eprintln!(
                "{count}/{name}: list {:?}/{:?}, latest {:?}/{:?}",
                samples[0][5], samples[1][5], samples[2][5], samples[3][5]
            );
        }
    }
}

#[test]
fn date_intersection_stays_sorted() {
    let a = Date::new(2026, 8, 1);
    let b = Date::new(2026, 8, 2);
    let c = Date::new(2026, 8, 3);

    for left in [vec![], vec![a], vec![a, b, c]] {
        for right in [vec![], vec![a], vec![b], vec![c], vec![a, c], vec![a, b, c]] {
            let expected = intersect_dates(left.clone(), right.clone());
            assert_eq!(
                collected_intersection(left.clone(), right.clone()),
                expected
            );
            assert_eq!(
                last_intersection(left.clone(), right),
                expected.last().copied()
            );
        }
    }
}
