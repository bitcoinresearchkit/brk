use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Type, parse_macro_input};

/// Struct-level attributes for Traversable derive
#[derive(Default)]
struct StructAttr {
    /// If true, call .merge_branches().unwrap() on the final result
    merge: bool,
    /// If true, delegate to the single field (transparent newtype pattern)
    transparent: bool,
    /// If set, wrap the result in Branch { key: inner }
    wrap: Option<String>,
}

fn get_struct_attr(attrs: &[syn::Attribute]) -> StructAttr {
    let mut result = StructAttr::default();
    for attr in attrs {
        if !attr.path().is_ident("traversable") {
            continue;
        }

        // Try parsing as single ident (merge, transparent)
        if let Ok(ident) = attr.parse_args::<syn::Ident>() {
            match ident.to_string().as_str() {
                "merge" => result.merge = true,
                "transparent" => result.transparent = true,
                _ => {}
            }
            continue;
        }

        // Try parsing as name-value (wrap = "...")
        if let Ok(meta) = attr.parse_args::<syn::MetaNameValue>()
            && meta.path.is_ident("wrap")
            && let syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Str(lit_str),
                ..
            }) = &meta.value
        {
            result.wrap = Some(lit_str.value());
        }
    }
    result
}

#[proc_macro_derive(Traversable, attributes(traversable))]
pub fn derive_traversable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, _) = generics.split_for_impl();

    let struct_attr = get_struct_attr(&input.attrs);

    let Data::Struct(data) = &input.data else {
        return syn::Error::new_spanned(
            &input.ident,
            "Traversable can only be derived for structs",
        )
        .to_compile_error()
        .into();
    };

    // Handle single-field tuple struct delegation (automatic transparent)
    if let Fields::Unnamed(fields) = &data.fields
        && fields.unnamed.len() == 1
    {
        let where_clause = build_where_clause(generics, &[]);
        let to_tree_node_body = if let Some(wrap_key) = &struct_attr.wrap {
            quote! {
                brk_traversable::TreeNode::wrap(#wrap_key, self.0.to_tree_node())
            }
        } else {
            quote! {
                self.0.to_tree_node()
            }
        };
        return TokenStream::from(quote! {
            impl #impl_generics Traversable for #name #ty_generics #where_clause {
                fn to_tree_node(&self) -> brk_traversable::TreeNode {
                    #to_tree_node_body
                }

                fn iter_any_exportable(&self) -> impl Iterator<Item = &dyn vecdb::AnyExportableVec> {
                    self.0.iter_any_exportable()
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

                fn iter_any_exportable(&self) -> impl Iterator<Item = &dyn vecdb::AnyExportableVec> {
                    std::iter::empty()
                }
            }
        });
    };

    // Handle transparent delegation for named structs (delegates to first field)
    if struct_attr.transparent {
        let first_field = named_fields
            .named
            .first()
            .expect("transparent requires at least one field");
        let field_name = first_field
            .ident
            .as_ref()
            .expect("named field must have ident");
        let where_clause = build_where_clause(generics, &[]);
        return TokenStream::from(quote! {
            impl #impl_generics Traversable for #name #ty_generics #where_clause {
                fn to_tree_node(&self) -> brk_traversable::TreeNode {
                    self.#field_name.to_tree_node()
                }

                fn iter_any_exportable(&self) -> impl Iterator<Item = &dyn vecdb::AnyExportableVec> {
                    self.#field_name.iter_any_exportable()
                }
            }
        });
    }

    let generic_params: Vec<_> = generics.type_params().map(|p| &p.ident).collect();

    let (field_infos, generics_needing_traversable) = analyze_fields(named_fields, &generic_params);

    let field_traversals = generate_field_traversals(&field_infos, struct_attr.merge);
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
    rename: Option<String>,
    wrap: Option<String>,
}

fn analyze_fields<'a>(
    fields: &'a syn::FieldsNamed,
    generic_params: &[&'a syn::Ident],
) -> (Vec<FieldInfo<'a>>, Vec<&'a syn::Ident>) {
    let mut field_infos = Vec::new();
    let mut generics_set = std::collections::BTreeSet::new();

    for field in &fields.named {
        let Some((attr, rename, wrap)) = get_field_attr(field) else {
            // Skip attribute means don't process at all
            continue;
        };

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
            attr,
            rename,
            wrap,
        });
    }

    (field_infos, generics_set.into_iter().collect())
}

/// Returns None for skip, Some((attr, rename, wrap)) for normal/flatten
fn get_field_attr(field: &syn::Field) -> Option<(FieldAttr, Option<String>, Option<String>)> {
    let mut attr_type = FieldAttr::Normal;
    let mut rename = None;
    let mut wrap = None;

    for attr in &field.attrs {
        if !attr.path().is_ident("traversable") {
            continue;
        }

        // Try parsing as a single ident (skip, flatten)
        if let Ok(ident) = attr.parse_args::<syn::Ident>() {
            match ident.to_string().as_str() {
                "skip" => return None,
                "flatten" => attr_type = FieldAttr::Flatten,
                _ => {}
            }
            continue;
        }

        // Try parsing as comma-separated name-value pairs (rename = "...", wrap = "...")
        if let Ok(metas) = attr.parse_args_with(
            syn::punctuated::Punctuated::<syn::MetaNameValue, syn::Token![,]>::parse_terminated,
        ) {
            for meta in metas {
                if let syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Str(lit_str),
                    ..
                }) = &meta.value
                {
                    if meta.path.is_ident("rename") {
                        rename = Some(lit_str.value());
                    } else if meta.path.is_ident("wrap") {
                        wrap = Some(lit_str.value());
                    }
                }
            }
        }
    }

    Some((attr_type, rename, wrap))
}

fn is_option_type(ty: &Type) -> bool {
    matches!(
        ty,
        Type::Path(type_path)
        if type_path.path.segments.last()
            .is_some_and(|seg| seg.ident == "Option")
    )
}

fn generate_field_traversals(infos: &[FieldInfo], merge: bool) -> proc_macro2::TokenStream {
    let has_flatten = infos.iter().any(|i| matches!(i.attr, FieldAttr::Flatten));

    // Generate normal field entries
    let normal_entries: Vec<_> = infos
        .iter()
        .filter(|i| matches!(i.attr, FieldAttr::Normal))
        .map(|info| {
            let field_name = info.name;
            let field_name_str = {
                let s = field_name.to_string();
                s.strip_prefix('_').map(String::from).unwrap_or(s)
            };

            // Determine outer key and inner wrap key based on which attrs are present
            // When both wrap and rename are present: wrap is outer container, rename is inner key
            // When only wrap: wrap is outer container, field_name is inner key
            // When only rename: rename is outer, no inner wrapping
            let (outer_key, inner_wrap): (&str, Option<&str>) =
                match (info.wrap.as_deref(), info.rename.as_deref()) {
                    (Some(wrap), Some(rename)) => (wrap, Some(rename)),
                    (Some(wrap), None) => (wrap, Some(&field_name_str)),
                    (None, Some(rename)) => (rename, None),
                    (None, None) => (&field_name_str, None),
                };

            // Generate tree node expression, optionally wrapped
            let node_expr = if let Some(inner_key) = inner_wrap {
                quote! { brk_traversable::TreeNode::wrap(#inner_key, nested.to_tree_node()) }
            } else {
                quote! { nested.to_tree_node() }
            };

            if info.is_option {
                quote! {
                    self.#field_name.as_ref().map(|nested| (String::from(#outer_key), #node_expr))
                }
            } else {
                let node_expr_self = if let Some(inner_key) = inner_wrap {
                    quote! { brk_traversable::TreeNode::wrap(#inner_key, self.#field_name.to_tree_node()) }
                } else {
                    quote! { self.#field_name.to_tree_node() }
                };
                quote! {
                    Some((String::from(#outer_key), #node_expr_self))
                }
            }
        })
        .collect();

    // Generate flatten field entries
    let flatten_entries: Vec<_> = infos
        .iter()
        .filter(|i| matches!(i.attr, FieldAttr::Flatten))
        .map(|info| {
            let field_name = info.name;

            if info.is_option {
                quote! {
                    if let Some(ref nested) = self.#field_name {
                        match nested.to_tree_node() {
                            brk_traversable::TreeNode::Branch(map) => {
                                for (key, node) in map {
                                    brk_traversable::TreeNode::merge_node(&mut collected, key, node)
                                        .expect("Conflicting values for same key during flatten");
                                }
                            }
                            leaf @ brk_traversable::TreeNode::Leaf(_) => {
                                // Collapsed leaf from child - insert with field name as key
                                brk_traversable::TreeNode::merge_node(&mut collected, String::from(stringify!(#field_name)), leaf)
                                    .expect("Conflicting values for same key during flatten");
                            }
                        }
                    }
                }
            } else {
                quote! {
                    match self.#field_name.to_tree_node() {
                        brk_traversable::TreeNode::Branch(map) => {
                            for (key, node) in map {
                                brk_traversable::TreeNode::merge_node(&mut collected, key, node)
                                    .expect("Conflicting values for same key during flatten");
                            }
                        }
                        leaf @ brk_traversable::TreeNode::Leaf(_) => {
                            // Collapsed leaf from child - insert with field name as key
                            brk_traversable::TreeNode::merge_node(&mut collected, String::from(stringify!(#field_name)), leaf)
                                .expect("Conflicting values for same key during flatten");
                        }
                    }
                }
            }
        })
        .collect();

    let final_expr = if merge {
        quote! { brk_traversable::TreeNode::Branch(collected).merge_branches().unwrap() }
    } else {
        quote! { brk_traversable::TreeNode::Branch(collected) }
    };

    // Build collected map initialization based on what we have
    // Use merge_entry to handle duplicate keys (e.g., multiple fields renamed to same key)
    let (init_collected, extend_flatten) = if !has_flatten {
        // No flatten fields - use merge_entry for each to handle duplicates
        (
            quote! {
                let mut collected: std::collections::BTreeMap<String, brk_traversable::TreeNode> =
                    std::collections::BTreeMap::new();
                for entry in [#(#normal_entries,)*].into_iter().flatten() {
                    brk_traversable::TreeNode::merge_node(&mut collected, entry.0, entry.1)
                        .expect("Conflicting values for same key");
                }
            },
            quote! {},
        )
    } else if normal_entries.is_empty() {
        // Only flatten fields - explicit type annotation needed
        (
            quote! {
                let mut collected: std::collections::BTreeMap<String, brk_traversable::TreeNode> =
                    std::collections::BTreeMap::new();
            },
            quote! { #(#flatten_entries)* },
        )
    } else {
        // Both normal and flatten fields - use merge_entry for normal fields
        (
            quote! {
                let mut collected: std::collections::BTreeMap<String, brk_traversable::TreeNode> =
                    std::collections::BTreeMap::new();
                for entry in [#(#normal_entries,)*].into_iter().flatten() {
                    brk_traversable::TreeNode::merge_node(&mut collected, entry.0, entry.1)
                        .expect("Conflicting values for same key");
                }
            },
            quote! { #(#flatten_entries)* },
        )
    };

    quote! {
        #init_collected
        #extend_flatten
        #final_expr
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
            fn iter_any_exportable(&self) -> impl Iterator<Item = &dyn vecdb::AnyExportableVec> {
                std::iter::empty()
            }
        };
    }

    let (init_part, chain_part) = if let Some((&first, rest)) = regular_fields.split_first() {
        (
            quote! {
                let mut regular_iter: Box<dyn Iterator<Item = &dyn vecdb::AnyExportableVec>> =
                    Box::new(self.#first.iter_any_exportable());
            },
            quote! {
                #(regular_iter = Box::new(regular_iter.chain(self.#rest.iter_any_exportable()));)*
            },
        )
    } else {
        (
            quote! {
                let mut regular_iter: Box<dyn Iterator<Item = &dyn vecdb::AnyExportableVec>> =
                    Box::new(std::iter::empty());
            },
            quote! {},
        )
    };

    let option_part = if !option_fields.is_empty() {
        let chains = option_fields.iter().map(|f| {
            quote! {
                if let Some(ref x) = self.#f {
                    regular_iter = Box::new(regular_iter.chain(x.iter_any_exportable()));
                }
            }
        });
        quote! { #(#chains)* }
    } else {
        quote! {}
    };

    quote! {
        fn iter_any_exportable(&self) -> impl Iterator<Item = &dyn vecdb::AnyExportableVec> {
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
