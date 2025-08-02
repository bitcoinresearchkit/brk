use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Fields, parse_macro_input};

#[proc_macro_derive(StoredCompressed)]
pub fn derive_stored_compressed(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let inner_type = match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Unnamed(fields),
            ..
        }) if fields.unnamed.len() == 1 => &fields.unnamed[0].ty,
        _ => {
            return syn::Error::new_spanned(
                &input.ident,
                "StoredCompressed can only be derived for single-field tuple structs",
            )
            .to_compile_error()
            .into();
        }
    };

    // Check if we have generic parameters
    let has_generics = !generics.params.is_empty();

    let expanded = if has_generics {
        // Generic case: add where clause constraint
        let where_clause = if where_clause.is_some() {
            quote! { #where_clause #inner_type: StoredCompressed, }
        } else {
            quote! { where #inner_type: StoredCompressed, }
        };

        quote! {
            impl #impl_generics ::brk_vecs::TransparentStoredCompressed<<#inner_type as StoredCompressed>::NumberType> for #struct_name #ty_generics #where_clause {}

            impl #impl_generics StoredCompressed for #struct_name #ty_generics #where_clause {
                type NumberType = <#inner_type as StoredCompressed>::NumberType;
            }
        }
    } else {
        // Non-generic case: use the original approach
        quote! {
            impl ::brk_vecs::TransparentStoredCompressed<<#inner_type as StoredCompressed>::NumberType> for #struct_name {}

            impl StoredCompressed for #struct_name {
                type NumberType = <#inner_type as StoredCompressed>::NumberType;
            }
        }
    };

    TokenStream::from(expanded)
}
