use pco::data_types::Number;

use super::StoredRaw;

pub trait TransparentStoredCompressed<T> {}

pub trait StoredCompressed
where
    Self: StoredRaw + Copy + 'static + TransparentStoredCompressed<Self::NumberType>,
{
    type NumberType: pco::data_types::Number;
}

pub trait AsInnerSlice<T>
where
    T: Number,
{
    fn as_inner_slice(&self) -> &[T];
}

impl<T> AsInnerSlice<T::NumberType> for [T]
where
    T: StoredCompressed,
{
    fn as_inner_slice(&self) -> &[T::NumberType] {
        assert_eq!(
            std::mem::size_of::<T>(),
            std::mem::size_of::<T::NumberType>()
        );
        assert_eq!(
            std::mem::align_of::<T>(),
            std::mem::align_of::<T::NumberType>()
        );
        unsafe { std::slice::from_raw_parts(self.as_ptr() as *const T::NumberType, self.len()) }
    }
}

pub trait FromInnerSlice<T> {
    fn from_inner_slice(slice: Vec<T>) -> Vec<Self>
    where
        Self: Sized;
}

impl<T> FromInnerSlice<T::NumberType> for T
where
    T: StoredCompressed,
{
    fn from_inner_slice(vec: Vec<T::NumberType>) -> Vec<T> {
        assert_eq!(
            std::mem::size_of::<T>(),
            std::mem::size_of::<T::NumberType>()
        );
        assert_eq!(
            std::mem::align_of::<T>(),
            std::mem::align_of::<T::NumberType>()
        );

        let mut vec = std::mem::ManuallyDrop::new(vec);
        unsafe { Vec::from_raw_parts(vec.as_mut_ptr() as *mut T, vec.len(), vec.capacity()) }
    }
}

impl TransparentStoredCompressed<u16> for u16 {}
impl StoredCompressed for u16 {
    type NumberType = u16;
}
impl TransparentStoredCompressed<u32> for u32 {}
impl StoredCompressed for u32 {
    type NumberType = u32;
}
impl TransparentStoredCompressed<u64> for u64 {}
impl StoredCompressed for u64 {
    type NumberType = u64;
}
impl TransparentStoredCompressed<i16> for i16 {}
impl StoredCompressed for i16 {
    type NumberType = i16;
}
impl TransparentStoredCompressed<i32> for i32 {}
impl StoredCompressed for i32 {
    type NumberType = i32;
}
impl TransparentStoredCompressed<i64> for i64 {}
impl StoredCompressed for i64 {
    type NumberType = i64;
}
impl TransparentStoredCompressed<f32> for f32 {}
impl StoredCompressed for f32 {
    type NumberType = f32;
}
impl TransparentStoredCompressed<f64> for f64 {}
impl StoredCompressed for f64 {
    type NumberType = f64;
}
impl TransparentStoredCompressed<u16> for () {}
impl StoredCompressed for () {
    type NumberType = u16;
}
