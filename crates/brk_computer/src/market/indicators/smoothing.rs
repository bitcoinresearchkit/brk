use std::collections::VecDeque;

pub(super) fn compute_rma(source: &[f32], period: usize) -> Vec<f32> {
    let mut result = Vec::with_capacity(source.len());
    let k = 1.0 / period as f32;
    let mut sum = 0.0f32;

    for (i, &val) in source.iter().enumerate() {
        if i < period {
            sum += val;
            result.push(sum / (i + 1) as f32);
        } else {
            let prev = result[i - 1];
            result.push(val * k + prev * (1.0 - k));
        }
    }

    result
}

pub(super) fn compute_ema(source: &[f32], period: usize) -> Vec<f32> {
    let mut result = Vec::with_capacity(source.len());
    let k = 2.0 / (period as f32 + 1.0);
    let mut sum = 0.0f32;

    for (i, &val) in source.iter().enumerate() {
        if i < period {
            sum += val;
            if i == period - 1 {
                result.push(sum / period as f32);
            } else {
                result.push(val);
            }
        } else {
            let prev = result[i - 1];
            result.push(val * k + prev * (1.0 - k));
        }
    }

    result
}

pub(super) fn compute_sma(source: &[f32], window: usize) -> Vec<f32> {
    let mut result = Vec::with_capacity(source.len());
    let mut sum = 0.0f32;

    for (i, &val) in source.iter().enumerate() {
        sum += val;
        if i >= window {
            sum -= source[i - window];
        }
        let count = (i + 1).min(window);
        result.push(sum / count as f32);
    }

    result
}

pub(super) fn compute_rolling_min(source: &[f32], window: usize) -> Vec<f32> {
    let mut result = Vec::with_capacity(source.len());
    let mut deque = VecDeque::new();

    for (i, &val) in source.iter().enumerate() {
        while deque.back().is_some_and(|&(_, v): &(usize, f32)| v >= val) {
            deque.pop_back();
        }
        deque.push_back((i, val));
        if deque.front().unwrap().0 + window <= i {
            deque.pop_front();
        }
        result.push(deque.front().unwrap().1);
    }

    result
}

pub(super) fn compute_rolling_max(source: &[f32], window: usize) -> Vec<f32> {
    let mut result = Vec::with_capacity(source.len());
    let mut deque = VecDeque::new();

    for (i, &val) in source.iter().enumerate() {
        while deque.back().is_some_and(|&(_, v): &(usize, f32)| v <= val) {
            deque.pop_back();
        }
        deque.push_back((i, val));
        if deque.front().unwrap().0 + window <= i {
            deque.pop_front();
        }
        result.push(deque.front().unwrap().1);
    }

    result
}
