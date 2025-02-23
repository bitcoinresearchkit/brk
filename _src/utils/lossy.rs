use ordered_float::OrderedFloat;

pub trait LossyFrom<T> {
    fn lossy_from(x: T) -> Self;
}

// ---
// u32
// ---

impl LossyFrom<u32> for u32 {
    #[inline(always)]
    fn lossy_from(x: u32) -> Self {
        x
    }
}

impl LossyFrom<u64> for u32 {
    #[inline(always)]
    fn lossy_from(x: u64) -> Self {
        x as u32
    }
}

impl LossyFrom<usize> for u32 {
    #[inline(always)]
    fn lossy_from(x: usize) -> Self {
        x as u32
    }
}

impl LossyFrom<f32> for u32 {
    #[inline(always)]
    fn lossy_from(x: f32) -> Self {
        x as u32
    }
}

impl LossyFrom<OrderedFloat<f32>> for u32 {
    #[inline(always)]
    fn lossy_from(x: OrderedFloat<f32>) -> Self {
        x.0 as u32
    }
}

// ---
// u64
// ---

impl LossyFrom<u32> for u64 {
    #[inline(always)]
    fn lossy_from(x: u32) -> Self {
        x as u64
    }
}

impl LossyFrom<u64> for u64 {
    #[inline(always)]
    fn lossy_from(x: u64) -> Self {
        x
    }
}

impl LossyFrom<usize> for u64 {
    #[inline(always)]
    fn lossy_from(x: usize) -> Self {
        x as u64
    }
}

impl LossyFrom<f32> for u64 {
    #[inline(always)]
    fn lossy_from(x: f32) -> Self {
        x as u64
    }
}

impl LossyFrom<OrderedFloat<f32>> for u64 {
    #[inline(always)]
    fn lossy_from(x: OrderedFloat<f32>) -> Self {
        x.0 as u64
    }
}

// ---
// usize
// ---

impl LossyFrom<usize> for usize {
    #[inline(always)]
    fn lossy_from(x: usize) -> Self {
        x
    }
}

impl LossyFrom<f32> for usize {
    #[inline(always)]
    fn lossy_from(x: f32) -> Self {
        x.round() as usize
    }
}

// ---
// f32
// ---

impl LossyFrom<u32> for f32 {
    #[inline(always)]
    fn lossy_from(x: u32) -> Self {
        x as f32
    }
}

impl LossyFrom<u64> for f32 {
    #[inline(always)]
    fn lossy_from(x: u64) -> Self {
        x as f32
    }
}

impl LossyFrom<usize> for f32 {
    #[inline(always)]
    fn lossy_from(x: usize) -> Self {
        x as f32
    }
}

impl LossyFrom<f32> for f32 {
    #[inline(always)]
    fn lossy_from(x: f32) -> Self {
        x
    }
}

impl LossyFrom<OrderedFloat<f32>> for f32 {
    #[inline(always)]
    fn lossy_from(x: OrderedFloat<f32>) -> Self {
        x.0
    }
}

impl LossyFrom<f64> for f32 {
    #[inline(always)]
    fn lossy_from(x: f64) -> Self {
        x as f32
    }
}

impl LossyFrom<OrderedFloat<f64>> for f32 {
    #[inline(always)]
    fn lossy_from(x: OrderedFloat<f64>) -> Self {
        x.0 as f32
    }
}

// ---
// OrderedFloat<f32>
// ---

impl LossyFrom<u32> for OrderedFloat<f32> {
    #[inline(always)]
    fn lossy_from(x: u32) -> Self {
        OrderedFloat(x as f32)
    }
}

impl LossyFrom<u64> for OrderedFloat<f32> {
    #[inline(always)]
    fn lossy_from(x: u64) -> Self {
        OrderedFloat(x as f32)
    }
}

impl LossyFrom<usize> for OrderedFloat<f32> {
    #[inline(always)]
    fn lossy_from(x: usize) -> Self {
        OrderedFloat(x as f32)
    }
}

impl LossyFrom<f32> for OrderedFloat<f32> {
    #[inline(always)]
    fn lossy_from(x: f32) -> Self {
        OrderedFloat(x)
    }
}

impl LossyFrom<OrderedFloat<f32>> for OrderedFloat<f32> {
    #[inline(always)]
    fn lossy_from(x: OrderedFloat<f32>) -> Self {
        x
    }
}

impl LossyFrom<f64> for OrderedFloat<f32> {
    #[inline(always)]
    fn lossy_from(x: f64) -> Self {
        OrderedFloat(x as f32)
    }
}

impl LossyFrom<OrderedFloat<f64>> for OrderedFloat<f32> {
    #[inline(always)]
    fn lossy_from(x: OrderedFloat<f64>) -> Self {
        OrderedFloat(x.0 as f32)
    }
}

// ---
// f64
// ---

impl LossyFrom<u64> for f64 {
    #[inline(always)]
    fn lossy_from(x: u64) -> Self {
        x as f64
    }
}

impl LossyFrom<usize> for f64 {
    #[inline(always)]
    fn lossy_from(x: usize) -> Self {
        x as f64
    }
}

impl LossyFrom<f32> for f64 {
    #[inline(always)]
    fn lossy_from(x: f32) -> Self {
        x as f64
    }
}

impl LossyFrom<OrderedFloat<f32>> for f64 {
    #[inline(always)]
    fn lossy_from(x: OrderedFloat<f32>) -> Self {
        x.0 as f64
    }
}

impl LossyFrom<f64> for f64 {
    #[inline(always)]
    fn lossy_from(x: f64) -> Self {
        x
    }
}

impl LossyFrom<OrderedFloat<f64>> for f64 {
    #[inline(always)]
    fn lossy_from(x: OrderedFloat<f64>) -> Self {
        x.0
    }
}

// ---
// OrderedFloat<f64>
// ---

impl LossyFrom<u64> for OrderedFloat<f64> {
    #[inline(always)]
    fn lossy_from(x: u64) -> Self {
        OrderedFloat(x as f64)
    }
}

impl LossyFrom<usize> for OrderedFloat<f64> {
    #[inline(always)]
    fn lossy_from(x: usize) -> Self {
        OrderedFloat(x as f64)
    }
}

impl LossyFrom<f32> for OrderedFloat<f64> {
    #[inline(always)]
    fn lossy_from(x: f32) -> Self {
        OrderedFloat(x as f64)
    }
}

impl LossyFrom<OrderedFloat<f32>> for OrderedFloat<f64> {
    #[inline(always)]
    fn lossy_from(x: OrderedFloat<f32>) -> Self {
        OrderedFloat(x.0 as f64)
    }
}

impl LossyFrom<f64> for OrderedFloat<f64> {
    #[inline(always)]
    fn lossy_from(x: f64) -> Self {
        OrderedFloat(x)
    }
}

impl LossyFrom<OrderedFloat<f64>> for OrderedFloat<f64> {
    #[inline(always)]
    fn lossy_from(x: OrderedFloat<f64>) -> Self {
        x
    }
}
