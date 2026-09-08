use super::*;

fn intersect_dates(left: Vec<Date>, right: Vec<Date>) -> Vec<Date> {
    left.into_iter()
        .filter(|date| right.contains(date))
        .collect()
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
