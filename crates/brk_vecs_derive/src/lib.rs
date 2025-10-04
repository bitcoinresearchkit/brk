use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Type, parse_macro_input};

#[proc_macro_derive(IVecs, attributes(vecs))]
pub fn derive_ivecs(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let traverse_impl = match &input.data {
        Data::Struct(data) => {
            let field_traversals = generate_field_traversals(&data.fields);
            let iterator_impl = generate_iterator_impl(&data.fields);

            quote! {
                impl IVecs for #name {
                    fn to_tree_node(&self) -> TreeNode {
                        let mut children = std::collections::HashMap::new();
                        #field_traversals
                        TreeNode::Branch(children)
                    }

                    #iterator_impl
                }
            }
        }
        _ => panic!("IVecs can only be derived for structs"),
    };

    TokenStream::from(traverse_impl)
}

// This catches EagerVec, RawVec, CompressedVec, StoredVec, and any future *Vec types
fn is_vec_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty
        && let Some(segment) = type_path.path.segments.last()
    {
        let ident = segment.ident.to_string();
        // Heuristic: if it ends with "Vec", it's likely a direct vec type
        // This is more maintainable than hardcoding all vec types
        return ident.ends_with("Vec");
    }
    false
}

fn generate_field_traversals(fields: &Fields) -> proc_macro2::TokenStream {
    match fields {
        Fields::Named(fields) => {
            let traversals = fields.named.iter().filter_map(|f| {
                let field_name = f.ident.as_ref()?;
                let field_name_str = field_name.to_string();

                if has_skip_attribute(f) {
                    return None;
                }

                if !matches!(f.vis, syn::Visibility::Public(_)) {
                    return None;
                }

                Some(quote! {
                    children.insert(
                        String::from(#field_name_str),
                        self.#field_name.to_tree_node()
                    );
                })
            });

            quote! { #(#traversals)* }
        }
        _ => quote! {},
    }
}

fn has_skip_attribute(field: &syn::Field) -> bool {
    field.attrs.iter().any(|attr| {
        attr.path().is_ident("vec_tree")
            && attr
                .parse_args::<syn::Ident>()
                .map(|ident| ident == "skip")
                .unwrap_or(false)
    })
}

fn generate_iterator_impl(fields: &Fields) -> proc_macro2::TokenStream {
    match fields {
        Fields::Named(fields) => {
            let mut direct_vecs = Vec::new();
            let mut option_vecs = Vec::new();
            let mut option_nested = Vec::new();
            let mut nested_fields = Vec::new();

            for field in fields.named.iter() {
                if let Some(field_name) = &field.ident {
                    if !matches!(field.vis, syn::Visibility::Public(_)) {
                        continue;
                    }

                    if let Some(inner_ty) = get_option_inner_type(&field.ty) {
                        if is_vec_type(inner_ty) {
                            option_vecs.push(field_name);
                        } else {
                            option_nested.push(field_name);
                        }
                    } else if is_vec_type(&field.ty) {
                        direct_vecs.push(field_name);
                    } else {
                        nested_fields.push(field_name);
                    }
                }
            }

            let base = if !direct_vecs.is_empty() {
                quote! {
                    let mut iter: Box<dyn Iterator<Item = &dyn AnyCollectableVec> + '_> = Box::new(
                        [#(&self.#direct_vecs as &dyn AnyCollectableVec,)*].into_iter()
                    );
                }
            } else {
                quote! {
                    let mut iter: Box<dyn Iterator<Item = &dyn AnyCollectableVec> + '_> =
                        Box::new(std::iter::empty());
                }
            };

            let option_vec_chains = option_vecs.iter().map(|f| {
                quote! { iter = Box::new(iter.chain(self.#f.iter())); }
            });

            let option_nested_chains = option_nested.iter().map(|f| {
                quote! { iter = Box::new(iter.chain(self.#f.iter().flat_map(|v| v.iter_any_collectable()))); }
            });

            let nested_chains = nested_fields.iter().map(|f| {
                quote! { iter = Box::new(iter.chain(self.#f.iter_any_collectable())); }
            });

            quote! {
                fn iter_any_collectable<'a>(&'a self) -> Box<dyn Iterator<Item = &'a dyn AnyCollectableVec> + 'a> {
                    #base
                    #(#option_vec_chains)*
                    #(#option_nested_chains)*
                    #(#nested_chains)*
                    iter
                }
            }
        }
        _ => quote! {
            fn iter_any_collectable<'a>(&'a self) -> Box<dyn Iterator<Item = &'a dyn AnyCollectableVec> + 'a> {
                Box::new(std::iter::empty())
            }
        },
    }
}

fn get_option_inner_type(ty: &Type) -> Option<&Type> {
    if let Type::Path(type_path) = ty
        && let Some(segment) = type_path.path.segments.last()
        && segment.ident == "Option"
        && let syn::PathArguments::AngleBracketed(args) = &segment.arguments
        && let Some(syn::GenericArgument::Type(inner_ty)) = args.args.first()
    {
        return Some(inner_ty);
    }
    None
}
