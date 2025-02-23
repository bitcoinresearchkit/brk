use std::cell::UnsafeCell;

#[derive(Copy, Clone)]
pub struct UnsafeSlice<'a, T>(&'a [UnsafeCell<T>]);
unsafe impl<T: Send + Sync> Send for UnsafeSlice<'_, T> {}
unsafe impl<T: Send + Sync> Sync for UnsafeSlice<'_, T> {}

impl<'a, T> UnsafeSlice<'a, T> {
    pub fn new(slice: &'a mut [T]) -> Self {
        let ptr = slice as *mut [T] as *const [UnsafeCell<T>];
        Self(unsafe { &*ptr })
    }

    /// SAFETY: It is UB if two threads write to the same index without
    /// synchronization.
    pub fn write(&self, i: usize, value: T) {
        unsafe {
            *self.0[i].get() = value;
        }
    }

    pub fn copy_slice(&self, start: usize, slice: &[T])
    where
        T: Copy,
    {
        slice.iter().enumerate().for_each(|(i, v)| {
            self.write(start + i, *v);
        });
    }
}
