use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Type, parse_macro_input};

#[proc_macro_derive(Traversable, attributes(vecs))]
pub fn derive_traversable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, _) = generics.split_for_impl();

    let traverse_impl = match &input.data {
        Data::Struct(data) => {
            match &data.fields {
                Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                    // Special case for single-field tuple structs - just delegate
                    let generic_params = generics.type_params().map(|p| &p.ident);
                    let original_predicates =
                        &generics.where_clause.as_ref().map(|w| &w.predicates);

                    let where_clause =
                        if original_predicates.is_some() || generics.type_params().count() > 0 {
                            quote! {
                                where
                                    #(#generic_params: Send + Sync,)*
                                    #original_predicates
                            }
                        } else {
                            quote! {}
                        };

                    quote! {
                        impl #impl_generics Traversable for #name #ty_generics
                        #where_clause
                        {
                            fn to_tree_node(&self) -> brk_traversable::TreeNode {
                                self.0.to_tree_node()
                            }

                            fn iter_any_collectable(&self) -> impl Iterator<Item = &dyn vecdb::AnyCollectableVec> {
                                self.0.iter_any_collectable()
                            }
                        }
                    }
                }
                _ => {
                    // Normal struct with named fields
                    let field_traversals = generate_field_traversals(&data.fields);
                    let iterator_impl = generate_iterator_impl(&data.fields);

                    // Collect field types that need to implement Traversable
                    let field_types = if let Fields::Named(named_fields) = &data.fields {
                        named_fields
                            .named
                            .iter()
                            .filter(|f| matches!(f.vis, syn::Visibility::Public(_)))
                            .filter(|f| !has_skip_attribute(f))
                            .map(|f| &f.ty)
                            .collect::<Vec<_>>()
                    } else {
                        Vec::new()
                    };

                    let generic_params = generics.type_params().map(|p| &p.ident);
                    let original_predicates =
                        &generics.where_clause.as_ref().map(|w| &w.predicates);

                    let where_clause = if !field_types.is_empty()
                        || original_predicates.is_some()
                        || generics.type_params().count() > 0
                    {
                        quote! {
                            where
                                #(#field_types: brk_traversable::Traversable,)*
                                #(#generic_params: Send + Sync,)*
                                #original_predicates
                        }
                    } else {
                        quote! {}
                    };

                    quote! {
                        impl #impl_generics Traversable for #name #ty_generics
                        #where_clause
                        {
                            fn to_tree_node(&self) -> brk_traversable::TreeNode {
                                #field_traversals
                            }

                            #iterator_impl
                        }
                    }
                }
            }
        }
        _ => panic!("Traversable can only be derived for structs"),
    };

    TokenStream::from(traverse_impl)
}

fn generate_field_traversals(fields: &Fields) -> proc_macro2::TokenStream {
    match fields {
        Fields::Named(fields) => {
            let entries = fields.named.iter().filter_map(|f| {
                let field_name = f.ident.as_ref()?;
                let field_name_str = field_name.to_string();

                if has_skip_attribute(f) || !matches!(f.vis, syn::Visibility::Public(_)) {
                    return None;
                }

                if get_option_inner_type(&f.ty).is_some() {
                    Some(quote! {
                        self.#field_name.as_ref().map(|nested| (String::from(#field_name_str), nested.to_tree_node()))
                    })
                } else {
                    Some(quote! {
                        Some((String::from(#field_name_str), self.#field_name.to_tree_node()))
                    })
                }
            });

            quote! {
                return brk_traversable::TreeNode::Branch(
                    [#(#entries,)*]
                        .into_iter()
                        .flatten()
                        .collect()
                );
            }
        }
        _ => quote! {},
    }
}

fn has_skip_attribute(field: &syn::Field) -> bool {
    field.attrs.iter().any(|attr| {
        attr.path().is_ident("vecs")
            && attr
                .parse_args::<syn::Ident>()
                .map(|ident| ident == "skip")
                .unwrap_or(false)
    })
}

fn generate_iterator_impl(fields: &Fields) -> proc_macro2::TokenStream {
    match fields {
        Fields::Named(fields) => {
            let mut regular_fields = Vec::new();
            let mut option_fields = Vec::new();

            for field in fields.named.iter() {
                if let Some(field_name) = &field.ident {
                    if !matches!(field.vis, syn::Visibility::Public(_)) {
                        continue;
                    }

                    if has_skip_attribute(field) {
                        continue;
                    }

                    if get_option_inner_type(&field.ty).is_some() {
                        option_fields.push(field_name);
                    } else {
                        regular_fields.push(field_name);
                    }
                }
            }

            if regular_fields.is_empty() && option_fields.is_empty() {
                quote! {
                    fn iter_any_collectable(&self) -> impl Iterator<Item = &dyn vecdb::AnyCollectableVec> {
                        std::iter::empty()
                    }
                }
            } else {
                let regular_part = if !regular_fields.is_empty() {
                    let first = regular_fields.first().unwrap();
                    let rest = &regular_fields[1..];

                    quote! {
                        let mut regular_iter: Box<dyn Iterator<Item = &dyn vecdb::AnyCollectableVec>> =
                            Box::new(self.#first.iter_any_collectable());
                        #(regular_iter = Box::new(regular_iter.chain(self.#rest.iter_any_collectable()));)*
                    }
                } else {
                    quote! {
                        let regular_iter = std::iter::empty();
                    }
                };

                let option_part = if !option_fields.is_empty() {
                    quote! {
                        let option_iter = [
                            #(self.#option_fields.as_ref().map(|x| Box::new(x.iter_any_collectable()) as Box<dyn Iterator<Item = &dyn vecdb::AnyCollectableVec>>),)*
                        ]
                        .into_iter()
                        .flatten()
                        .flatten();
                    }
                } else {
                    quote! {
                        let option_iter = std::iter::empty();
                    }
                };

                quote! {
                    fn iter_any_collectable(&self) -> impl Iterator<Item = &dyn vecdb::AnyCollectableVec> {
                        #regular_part
                        #option_part
                        regular_iter.chain(option_iter)
                    }
                }
            }
        }
        _ => quote! {
            fn iter_any_collectable(&self) -> impl Iterator<Item = &dyn vecdb::AnyCollectableVec> {
                std::iter::empty()
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
