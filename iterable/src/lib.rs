/// The `Iterable` proc macro.
///
/// This macro provides a convenient way to make a struct iterable.
/// The struct fields' names are returned as static strings and their values as `dyn Any`.
/// This allows to iterate over the struct fields in a generic way.
///
/// Note that only structs with named fields are supported.
///
/// # Example
///
/// ```
/// use struct_iterable::Iterable;
///
/// #[derive(Iterable)]
/// struct MyStruct {
///     field1: i32,
///     field2: String,
///     // etc.
/// }
///
/// let my_instance = MyStruct {
///     field1: 42,
///     field2: "Hello, world!".to_string(),
/// };
///
/// for (field_name, field_value) in my_instance.iter() {
///     println!("{}: {:?}", field_name, field_value);
/// }
/// ```
pub use struct_iterable_derive::Iterable;

/// The `Iterable` trait.
///
/// This trait is implemented by the struct once the `Iterable` proc macro is derived.
/// It provides an `iter` method that returns an iterator over tuples of field names and values.
///
/// # Example
///
/// ```
/// use struct_iterable::Iterable;
///
/// #[derive(Iterable)]
/// struct MyStruct {
///     field1: i32,
///     field2: String,
///     // etc.
/// }
///
/// let my_instance = MyStruct {
///     field1: 42,
///     field2: "Hello, world!".to_string(),
/// };
///
/// for (field_name, field_value) in my_instance.iter() {
///     println!("{}: {:?}", field_name, field_value);
/// }
/// ```
pub use struct_iterable_internal::Iterable;

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Iterable)]
    struct MyStruct {
        field1: i32,
        field2: String,
        // etc.
    }

    #[test]
    fn it_works() {
        let mut my_instance = MyStruct {
            field1: 42,
            field2: "Hello, world!".to_string(),
        };

        for (field_name, field_value) in my_instance.iter() {
            dbg!("{}: {:?}", field_name, field_value);
        }

        for (field_name, field_value) in my_instance.iter_mut() {
            dbg!("{}: {:?}", field_name, &field_value);
            if let Some(i32) = field_value.downcast_mut::<i32>() {
                *i32 += 1;
                dbg!(i32);
            }
            dbg!("{}: {:?}", field_name, &field_value);
        }
    }
}
