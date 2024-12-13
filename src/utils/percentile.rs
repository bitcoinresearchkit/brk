use std::ops::Add;

use super::LossyFrom;

pub fn get_percentile<T>(sorted: &[T], percentile: f32) -> T
where
    T: Clone + Copy + LossyFrom<f32> + Add<Output = T>,
    f32: LossyFrom<T>,
{
    let len = sorted.len();

    if len < 2 {
        T::lossy_from(f32::NAN)
    } else {
        let index = (len - 1) as f32 * percentile;

        let fract = index.fract();

        if fract != 0.0 {
            let left = *sorted.get(index as usize).unwrap();
            let right = *sorted.get(index.ceil() as usize).unwrap();

            T::lossy_from(f32::lossy_from(left + right) / 2.0)
        } else {
            *sorted.get(index as usize).unwrap()
        }
    }
}
