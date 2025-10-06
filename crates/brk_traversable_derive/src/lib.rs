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
                    let generic_params: Vec<_> = generics.type_params().map(|p| &p.ident).collect();
                    let original_predicates =
                        &generics.where_clause.as_ref().map(|w| &w.predicates);

                    let where_clause =
                        if original_predicates.is_some() || !generic_params.is_empty() {
                            quote! {
                                where
                                    #(#generic_params: Send + Sync,)*
                                    #original_predicates
                            }
                        } else {
                            quote! {}
                        };

                    return TokenStream::from(quote! {
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
                    });
                }
                _ => {
                    // Normal struct with named fields
                    let field_traversals = generate_field_traversals(&data.fields);
                    let iterator_impl = generate_iterator_impl(&data.fields);

                    let generic_params: Vec<_> = generics.type_params().map(|p| &p.ident).collect();

                    let generics_needing_traversable =
                        if let Fields::Named(named_fields) = &data.fields {
                            let mut used = std::collections::BTreeSet::new();

                            for field in named_fields.named.iter() {
                                if !should_process_field(field) {
                                    continue;
                                }

                                if let Type::Path(type_path) = &field.ty
                                    && type_path.path.segments.len() == 1
                                    && let Some(seg) = type_path.path.segments.first()
                                    && seg.arguments.is_empty()
                                    && let Some(pos) =
                                        generic_params.iter().position(|g| g == &&seg.ident)
                                {
                                    used.insert(generic_params[pos]);
                                }
                            }
                            used.into_iter().collect::<Vec<_>>()
                        } else {
                            Vec::new()
                        };

                    let original_predicates =
                        &generics.where_clause.as_ref().map(|w| &w.predicates);

                    let where_clause = if !generics_needing_traversable.is_empty()
                        || original_predicates.is_some()
                        || !generic_params.is_empty()
                    {
                        quote! {
                            where
                                #(#generics_needing_traversable: brk_traversable::Traversable,)*
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
        _ => {
            return syn::Error::new_spanned(
                &input.ident,
                "Traversable can only be derived for structs",
            )
            .to_compile_error()
            .into();
        }
    };

    TokenStream::from(traverse_impl)
}

fn should_process_field(field: &syn::Field) -> bool {
    matches!(field.vis, syn::Visibility::Public(_)) && !has_skip_attribute(field)
}

fn generate_field_traversals(fields: &Fields) -> proc_macro2::TokenStream {
    match fields {
        Fields::Named(fields) => {
            let entries = fields.named.iter().filter_map(|f| {
                let field_name = f.ident.as_ref()?;
                let field_name_str = field_name.to_string();

                if !should_process_field(f) {
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
                let collected: std::collections::BTreeMap<_, _> = [#(#entries,)*]
                    .into_iter()
                    .flatten()
                    .collect();

                return if collected.len() == 1 {
                    collected.into_values().next().unwrap()
                } else {
                    brk_traversable::TreeNode::Branch(collected)
                };
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
                    if !should_process_field(field) {
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
                let (init_part, chain_part) = if !regular_fields.is_empty() {
                    let first = regular_fields.first().unwrap();
                    let rest = &regular_fields[1..];
                    (
                        quote! {
                            let mut regular_iter: Box<dyn Iterator<Item = &dyn vecdb::AnyCollectableVec>> =
                                Box::new(self.#first.iter_any_collectable());
                        },
                        quote! {
                            #(regular_iter = Box::new(regular_iter.chain(self.#rest.iter_any_collectable()));)*
                        },
                    )
                } else {
                    (
                        quote! {
                            let mut regular_iter: Box<dyn Iterator<Item = &dyn vecdb::AnyCollectableVec>> =
                                Box::new(std::iter::empty());
                        },
                        quote! {},
                    )
                };

                let option_part = if !option_fields.is_empty() {
                    let chains = option_fields.iter().map(|f| {
                        quote! {
                            if let Some(ref x) = self.#f {
                                regular_iter = Box::new(regular_iter.chain(x.iter_any_collectable()));
                            }
                        }
                    });
                    quote! { #(#chains)* }
                } else {
                    quote! {}
                };

                quote! {
                    fn iter_any_collectable(&self) -> impl Iterator<Item = &dyn vecdb::AnyCollectableVec> {
                        #init_part
                        #chain_part
                        #option_part
                        regular_iter
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
