# Struct Iterable

`Struct Iterable` is a Rust library that provides a proc macro to make a struct iterable. This allows you to iterate over the fields of your struct in a generic way, with each iteration returning a tuple containing the name of the field as a static string and a reference to the field's value as a `dyn Any`.

## How to Use

First, add `Struct Iterable` to your `Cargo.toml`:

```toml
[dependencies]
struct_iterable = "0.1.1"
```

Next, include the library at the top of your Rust file:

```rust
use struct_iterable::Iterable;
```

Finally, add the `#[derive(Iterable)]` attribute to your struct:

```rust
#[derive(Iterable)]
struct MyStruct {
    field1: u32,
    field2: String,
    // etc.
}
```

Now, you can iterate over the fields of an instance of your struct:
    
```rust
let my_instance = MyStruct {
    field1: 42,
    field2: "Hello, world!".to_string(),
};

for (field_name, field_value) in my_instance.iter() {
    println!("{}: {:?}", field_name, field_value);
}
```

## Limitations
- Only structs with named fields are supported.
- Only structs are supported, not enums or unions.

## Implementation

Here is the implementation of the proc macro:

```rust
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};
use iterable_structs::Iterable;

#[proc_macro_derive(Iterable)]
pub fn derive_iterable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = input.ident;
    let fields = match input.data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(fields_named) => fields_named.named,
            _ => panic!("Only structs with named fields are supported"),
        },
        _ => panic!("Only structs are supported"),
    };

    let fields_iter = fields.iter().map(|field| {
        let field_ident = &field.ident;
        let field_name = field_ident.as_ref().unwrap().to_string();
        quote! {
            (#field_name, &(self.#field_ident) as &dyn std::any::Any)
        }
    });

    let expanded = quote! {
        impl Iterable for #struct_name {
            fn iter<'a>(&'a self) -> std::vec::IntoIter<(&'static str, &'a dyn std::any::Any)> {
                vec![
                    #(#fields_iter),*
                ].into_iter()
            }
        }
    };

    TokenStream::from(expanded)
}
```

The macro takes in the TokenStream of a struct and expands it into an implementation of the Iterable trait for that struct. This trait provides an iter method that returns an iterator over tuples of field names and values.

## Contributing and License

`Struct Iterable` is an open-source project, and contributions are warmly welcomed. Whether you're fixing bugs, improving the documentation, or proposing new features, your efforts are highly appreciated!

If you're interested in contributing, please feel free to submit a pull request. For major changes, please open an issue first to discuss what you would like to change.

Please note that this project is released with a Contributor Code of Conduct. By participating in this project, you agree to abide by its terms.

`Struct Iterable` is distributed under the terms of the MIT license. As such, you're free to use, modify, distribute, and privately use it in any way you see fit, in accordance with the terms of the license.
