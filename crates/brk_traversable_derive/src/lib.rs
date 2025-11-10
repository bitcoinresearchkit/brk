use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Type, parse_macro_input};

#[proc_macro_derive(Traversable, attributes(traversable))]
pub fn derive_traversable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, _) = generics.split_for_impl();

    let Data::Struct(data) = &input.data else {
        return syn::Error::new_spanned(
            &input.ident,
            "Traversable can only be derived for structs",
        )
        .to_compile_error()
        .into();
    };

    // Handle single-field tuple struct delegation
    if let Fields::Unnamed(fields) = &data.fields
        && fields.unnamed.len() == 1
    {
        let where_clause = build_where_clause(generics, &[]);
        return TokenStream::from(quote! {
            impl #impl_generics Traversable for #name #ty_generics #where_clause {
                fn to_tree_node(&self) -> brk_traversable::TreeNode {
                    self.0.to_tree_node()
                }

                fn iter_any_writable(&self) -> impl Iterator<Item = &dyn vecdb::AnyWritableVec> {
                    self.0.iter_any_writable()
                }
            }
        });
    }

    // Handle named fields
    let Fields::Named(named_fields) = &data.fields else {
        return TokenStream::from(quote! {
            impl #impl_generics Traversable for #name #ty_generics {
                fn to_tree_node(&self) -> brk_traversable::TreeNode {
                    brk_traversable::TreeNode::Branch(std::collections::BTreeMap::new())
                }

                fn iter_any_writable(&self) -> impl Iterator<Item = &dyn vecdb::AnyWritableVec> {
                    std::iter::empty()
                }
            }
        });
    };

    let generic_params: Vec<_> = generics.type_params().map(|p| &p.ident).collect();

    let (field_infos, generics_needing_traversable) = analyze_fields(named_fields, &generic_params);

    let field_traversals = generate_field_traversals(&field_infos);
    let iterator_impl = generate_iterator_impl(&field_infos);
    let where_clause = build_where_clause(generics, &generics_needing_traversable);

    TokenStream::from(quote! {
        impl #impl_generics Traversable for #name #ty_generics #where_clause {
            fn to_tree_node(&self) -> brk_traversable::TreeNode {
                #field_traversals
            }

            #iterator_impl
        }
    })
}

enum FieldAttr {
    Normal,
    Flatten,
}

struct FieldInfo<'a> {
    name: &'a syn::Ident,
    is_option: bool,
    attr: FieldAttr,
}

fn analyze_fields<'a>(
    fields: &'a syn::FieldsNamed,
    generic_params: &[&'a syn::Ident],
) -> (Vec<FieldInfo<'a>>, Vec<&'a syn::Ident>) {
    let mut field_infos = Vec::new();
    let mut generics_set = std::collections::BTreeSet::new();

    for field in &fields.named {
        let field_attr = get_field_attr(field);

        // Skip attribute means don't process at all
        if field_attr.is_none() {
            continue;
        }

        if !matches!(field.vis, syn::Visibility::Public(_)) {
            continue;
        }

        let Some(field_name) = &field.ident else {
            continue;
        };

        if let Type::Path(type_path) = &field.ty
            && type_path.path.segments.len() == 1
            && let Some(seg) = type_path.path.segments.first()
            && seg.arguments.is_empty()
            && let Some(&param) = generic_params.iter().find(|&&g| g == &seg.ident)
        {
            generics_set.insert(param);
        }

        field_infos.push(FieldInfo {
            name: field_name,
            is_option: is_option_type(&field.ty),
            attr: field_attr.unwrap(),
        });
    }

    (field_infos, generics_set.into_iter().collect())
}

/// Returns None for skip, Some(attr) for normal/flatten
fn get_field_attr(field: &syn::Field) -> Option<FieldAttr> {
    for attr in &field.attrs {
        if attr.path().is_ident("traversable")
            && let Ok(ident) = attr.parse_args::<syn::Ident>()
        {
            return match ident.to_string().as_str() {
                "skip" => None,
                "flatten" => Some(FieldAttr::Flatten),
                _ => Some(FieldAttr::Normal),
            };
        }
    }
    Some(FieldAttr::Normal)
}

fn is_option_type(ty: &Type) -> bool {
    matches!(
        ty,
        Type::Path(type_path)
        if type_path.path.segments.last()
            .is_some_and(|seg| seg.ident == "Option")
    )
}

fn generate_field_traversals(infos: &[FieldInfo]) -> proc_macro2::TokenStream {
    let has_flatten = infos.iter().any(|i| matches!(i.attr, FieldAttr::Flatten));
    let has_normal = infos.iter().any(|i| matches!(i.attr, FieldAttr::Normal));

    if !has_flatten {
        // Fast path: no flatten, simple collection
        let entries = infos.iter().map(|info| {
            let field_name = info.name;
            let field_name_str = field_name.to_string();

            if info.is_option {
                quote! {
                    self.#field_name.as_ref().map(|nested| (String::from(#field_name_str), nested.to_tree_node()))
                }
            } else {
                quote! {
                    Some((String::from(#field_name_str), self.#field_name.to_tree_node()))
                }
            }
        });

        return quote! {
            let collected: std::collections::BTreeMap<_, _> = [#(#entries,)*]
                .into_iter()
                .flatten()
                .collect();

            brk_traversable::TreeNode::Branch(collected)
        };
    }

    // Has flatten fields
    if !has_normal {
        // Only flatten fields, no normal fields - need explicit type annotation
        let flatten_entries = infos.iter()
            .filter(|i| matches!(i.attr, FieldAttr::Flatten))
            .map(|info| {
                let field_name = info.name;

                if info.is_option {
                    quote! {
                        if let Some(ref nested) = self.#field_name {
                            if let brk_traversable::TreeNode::Branch(map) = nested.to_tree_node() {
                                collected.extend(map);
                            }
                        }
                    }
                } else {
                    quote! {
                        if let brk_traversable::TreeNode::Branch(map) = self.#field_name.to_tree_node() {
                            collected.extend(map);
                        }
                    }
                }
            });

        return quote! {
            let mut collected: std::collections::BTreeMap<String, brk_traversable::TreeNode> =
                std::collections::BTreeMap::new();

            #(#flatten_entries)*

            brk_traversable::TreeNode::Branch(collected)
        };
    }

    // Has both normal and flatten fields
    let normal_entries = infos.iter()
        .filter(|i| matches!(i.attr, FieldAttr::Normal))
        .map(|info| {
            let field_name = info.name;
            let field_name_str = field_name.to_string();

            if info.is_option {
                quote! {
                    self.#field_name.as_ref().map(|nested| (String::from(#field_name_str), nested.to_tree_node()))
                }
            } else {
                quote! {
                    Some((String::from(#field_name_str), self.#field_name.to_tree_node()))
                }
            }
        });

    let flatten_entries = infos.iter()
        .filter(|i| matches!(i.attr, FieldAttr::Flatten))
        .map(|info| {
            let field_name = info.name;

            if info.is_option {
                quote! {
                    if let Some(ref nested) = self.#field_name {
                        if let brk_traversable::TreeNode::Branch(map) = nested.to_tree_node() {
                            collected.extend(map);
                        }
                    }
                }
            } else {
                quote! {
                    if let brk_traversable::TreeNode::Branch(map) = self.#field_name.to_tree_node() {
                        collected.extend(map);
                    }
                }
            }
        });

    quote! {
        let mut collected: std::collections::BTreeMap<_, _> = [#(#normal_entries,)*]
            .into_iter()
            .flatten()
            .collect();

        #(#flatten_entries)*

        brk_traversable::TreeNode::Branch(collected)
    }
}

fn generate_iterator_impl(infos: &[FieldInfo]) -> proc_macro2::TokenStream {
    let regular_fields: Vec<_> = infos
        .iter()
        .filter(|i| !i.is_option)
        .map(|i| i.name)
        .collect();

    let option_fields: Vec<_> = infos
        .iter()
        .filter(|i| i.is_option)
        .map(|i| i.name)
        .collect();

    if regular_fields.is_empty() && option_fields.is_empty() {
        return quote! {
            fn iter_any_writable(&self) -> impl Iterator<Item = &dyn vecdb::AnyWritableVec> {
                std::iter::empty()
            }
        };
    }

    let (init_part, chain_part) = if let Some((&first, rest)) = regular_fields.split_first() {
        (
            quote! {
                let mut regular_iter: Box<dyn Iterator<Item = &dyn vecdb::AnyWritableVec>> =
                    Box::new(self.#first.iter_any_writable());
            },
            quote! {
                #(regular_iter = Box::new(regular_iter.chain(self.#rest.iter_any_writable()));)*
            },
        )
    } else {
        (
            quote! {
                let mut regular_iter: Box<dyn Iterator<Item = &dyn vecdb::AnyWritableVec>> =
                    Box::new(std::iter::empty());
            },
            quote! {},
        )
    };

    let option_part = if !option_fields.is_empty() {
        let chains = option_fields.iter().map(|f| {
            quote! {
                if let Some(ref x) = self.#f {
                    regular_iter = Box::new(regular_iter.chain(x.iter_any_writable()));
                }
            }
        });
        quote! { #(#chains)* }
    } else {
        quote! {}
    };

    quote! {
        fn iter_any_writable(&self) -> impl Iterator<Item = &dyn vecdb::AnyWritableVec> {
            #init_part
            #chain_part
            #option_part
            regular_iter
        }
    }
}

fn build_where_clause(
    generics: &syn::Generics,
    generics_needing_traversable: &[&syn::Ident],
) -> proc_macro2::TokenStream {
    let generic_params: Vec<_> = generics.type_params().map(|p| &p.ident).collect();
    let original_predicates = generics.where_clause.as_ref().map(|w| &w.predicates);

    if generics_needing_traversable.is_empty()
        && generic_params.is_empty()
        && original_predicates.is_none()
    {
        return quote! {};
    }

    quote! {
        where
            #(#generics_needing_traversable: brk_traversable::Traversable,)*
            #(#generic_params: Send + Sync,)*
            #original_predicates
    }
}
