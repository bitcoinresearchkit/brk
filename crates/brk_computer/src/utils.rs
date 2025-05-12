use std::ops::{Add, Div};

pub fn get_percentile<T>(sorted: &[T], percentile: f64) -> T
where
    T: Clone + Div<usize, Output = T> + Add<T, Output = T>,
{
    let len = sorted.len();

    if len == 0 {
        panic!();
    } else if len == 1 {
        sorted[0].clone()
    } else {
        let index = (len - 1) as f64 * percentile;

        let fract = index.fract();

        if fract != 0.0 {
            let left = sorted.get(index as usize).unwrap().clone();
            let right = sorted.get(index.ceil() as usize).unwrap().clone();
            left / 2 + right / 2
        } else {
            // dbg!(sorted.len(), index);
            sorted.get(index as usize).unwrap().clone()
        }
    }
}
