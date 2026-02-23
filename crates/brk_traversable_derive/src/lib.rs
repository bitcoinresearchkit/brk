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

    let mut output = gen_traversable(&input);
    output.extend(gen_read_only_clone(&input));
    TokenStream::from(output)
}

fn gen_traversable(input: &DeriveInput) -> proc_macro2::TokenStream {
    let name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, _) = generics.split_for_impl();

    let struct_attr = get_struct_attr(&input.attrs);

    let Data::Struct(data) = &input.data else {
        return syn::Error::new_spanned(
            &input.ident,
            "Traversable can only be derived for structs",
        )
        .to_compile_error();
    };

    // Handle single-field tuple struct delegation (automatic transparent)
    if let Fields::Unnamed(fields) = &data.fields
        && fields.unnamed.len() == 1
    {
        let field_ty = &fields.unnamed.first().unwrap().ty;
        let where_clause = build_where_clause(generics, &[], &[field_ty]);
        let to_tree_node_body = if let Some(wrap_key) = &struct_attr.wrap {
            quote! {
                brk_traversable::TreeNode::wrap(#wrap_key, self.0.to_tree_node())
            }
        } else {
            quote! {
                self.0.to_tree_node()
            }
        };
        return quote! {
            impl #impl_generics Traversable for #name #ty_generics #where_clause {
                fn to_tree_node(&self) -> brk_traversable::TreeNode {
                    #to_tree_node_body
                }

                fn iter_any_exportable(&self) -> impl Iterator<Item = &dyn vecdb::AnyExportableVec> {
                    self.0.iter_any_exportable()
                }
            }
        };
    }

    // Handle named fields
    let Fields::Named(named_fields) = &data.fields else {
        return quote! {
            impl #impl_generics Traversable for #name #ty_generics {
                fn to_tree_node(&self) -> brk_traversable::TreeNode {
                    brk_traversable::TreeNode::Branch(brk_traversable::IndexMap::new())
                }

                fn iter_any_exportable(&self) -> impl Iterator<Item = &dyn vecdb::AnyExportableVec> {
                    std::iter::empty()
                }
            }
        };
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
        let field_ty = &first_field.ty;
        let where_clause = build_where_clause(generics, &[], &[field_ty]);
        return quote! {
            impl #impl_generics Traversable for #name #ty_generics #where_clause {
                fn to_tree_node(&self) -> brk_traversable::TreeNode {
                    self.#field_name.to_tree_node()
                }

                fn iter_any_exportable(&self) -> impl Iterator<Item = &dyn vecdb::AnyExportableVec> {
                    self.#field_name.iter_any_exportable()
                }
            }
        };
    }

    let generic_params: Vec<_> = generics.type_params().map(|p| &p.ident).collect();

    let (field_infos, generics_needing_traversable, field_traversable_types) =
        analyze_fields(named_fields, &generic_params);

    let field_traversals = generate_field_traversals(&field_infos, struct_attr.merge);
    let iterator_impl = generate_iterator_impl(&field_infos);
    let where_clause = build_where_clause(
        generics,
        &generics_needing_traversable,
        &field_traversable_types,
    );

    quote! {
        impl #impl_generics Traversable for #name #ty_generics #where_clause {
            fn to_tree_node(&self) -> brk_traversable::TreeNode {
                #field_traversals
            }

            #iterator_impl
        }
    }
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
) -> (Vec<FieldInfo<'a>>, Vec<&'a syn::Ident>, Vec<&'a syn::Type>) {
    let mut field_infos = Vec::new();
    let mut generics_set = std::collections::BTreeSet::new();
    let mut field_traversable_types = Vec::new();

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

        let is_option = is_option_type(&field.ty);

        if let Type::Path(type_path) = &field.ty
            && type_path.path.segments.len() == 1
            && let Some(seg) = type_path.path.segments.first()
            && seg.arguments.is_empty()
            && let Some(&param) = generic_params.iter().find(|&&g| g == &seg.ident)
        {
            generics_set.insert(param);
        } else {
            // For non-bare-generic field types, add a Traversable bound.
            // For Option<T> fields, unwrap to get the inner T.
            let ty = if is_option {
                extract_option_inner(&field.ty).unwrap_or(&field.ty)
            } else {
                &field.ty
            };
            field_traversable_types.push(ty);
        }

        field_infos.push(FieldInfo {
            name: field_name,
            is_option,
            attr,
            rename,
            wrap,
        });
    }

    (
        field_infos,
        generics_set.into_iter().collect(),
        field_traversable_types,
    )
}

/// Extract the inner type from `Option<T>`, returning `Some(&T)`.
fn extract_option_inner(ty: &Type) -> Option<&Type> {
    if let Type::Path(type_path) = ty
        && let Some(seg) = type_path.path.segments.last()
        && seg.ident == "Option"
        && let syn::PathArguments::AngleBracketed(args) = &seg.arguments
        && let Some(syn::GenericArgument::Type(inner)) = args.args.first()
    {
        Some(inner)
    } else {
        None
    }
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

fn is_box_type(ty: &Type) -> bool {
    matches!(
        ty,
        Type::Path(type_path)
        if type_path.path.segments.last()
            .is_some_and(|seg| seg.ident == "Box")
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
                let mut collected: brk_traversable::IndexMap<String, brk_traversable::TreeNode> =
                    brk_traversable::IndexMap::new();
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
                let mut collected: brk_traversable::IndexMap<String, brk_traversable::TreeNode> =
                    brk_traversable::IndexMap::new();
            },
            quote! { #(#flatten_entries)* },
        )
    } else {
        // Both normal and flatten fields - use merge_entry for normal fields
        (
            quote! {
                let mut collected: brk_traversable::IndexMap<String, brk_traversable::TreeNode> =
                    brk_traversable::IndexMap::new();
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
    extra_traversable_types: &[&syn::Type],
) -> proc_macro2::TokenStream {
    let generic_params: Vec<_> = generics.type_params().map(|p| &p.ident).collect();
    let original_predicates = generics.where_clause.as_ref().map(|w| &w.predicates);

    if generics_needing_traversable.is_empty()
        && extra_traversable_types.is_empty()
        && generic_params.is_empty()
        && original_predicates.is_none()
    {
        return quote! {};
    }

    quote! {
        where
            #(#generics_needing_traversable: brk_traversable::Traversable,)*
            #(#extra_traversable_types: brk_traversable::Traversable,)*
            #(#generic_params: Send + Sync,)*
            #original_predicates
    }
}

// ---------------------------------------------------------------------------
// ReadOnlyClone + Clone generation
// ---------------------------------------------------------------------------

/// Find the generic type parameter bounded by `StorageMode`, if any.
fn find_storage_mode_param(generics: &syn::Generics) -> Option<&syn::Ident> {
    generics.type_params().find_map(|p| {
        p.bounds
            .iter()
            .any(|b| {
                matches!(b, syn::TypeParamBound::Trait(t)
                    if t.path.segments.last().is_some_and(|s| s.ident == "StorageMode"))
            })
            .then_some(&p.ident)
    })
}

/// Check if a type AST references the given identifier anywhere.
fn type_contains_ident(ty: &Type, ident: &syn::Ident) -> bool {
    match ty {
        Type::Path(type_path) => {
            // Check qualified self (e.g. <M as StorageMode>::Stored<V>)
            if let Some(qself) = &type_path.qself
                && type_contains_ident(&qself.ty, ident)
            {
                return true;
            }
            type_path.path.segments.iter().any(|seg| {
                if seg.ident == *ident {
                    return true;
                }
                match &seg.arguments {
                    syn::PathArguments::AngleBracketed(args) => args.args.iter().any(|arg| {
                        matches!(arg, syn::GenericArgument::Type(inner) if type_contains_ident(inner, ident))
                    }),
                    syn::PathArguments::Parenthesized(args) => {
                        args.inputs.iter().any(|inner| type_contains_ident(inner, ident))
                            || matches!(&args.output, syn::ReturnType::Type(_, inner) if type_contains_ident(inner, ident))
                    }
                    syn::PathArguments::None => false,
                }
            })
        }
        Type::Reference(r) => type_contains_ident(&r.elem, ident),
        Type::Tuple(t) => t.elems.iter().any(|e| type_contains_ident(e, ident)),
        Type::Array(a) => type_contains_ident(&a.elem, ident),
        Type::Slice(s) => type_contains_ident(&s.elem, ident),
        Type::Paren(p) => type_contains_ident(&p.elem, ident),
        _ => false,
    }
}

/// Generate `ReadOnlyClone` for Traversable-derived types.
///
/// - Types with `M: StorageMode` → maps `Self<Rw>` → `Self<Ro>`.
/// - Types with other generic type params (no M) → propagates `ReadOnlyClone` through each param.
/// - Types with no generic type params → nothing generated (they should `#[derive(Clone)]`).
fn gen_read_only_clone(input: &DeriveInput) -> proc_macro2::TokenStream {
    let generics = &input.generics;
    let name = &input.ident;

    let Data::Struct(data) = &input.data else {
        return quote! {};
    };

    if let Some(mode_param) = find_storage_mode_param(generics) {
        return gen_read_only_clone_for_m(name, generics, data, mode_param);
    }

    // Collect generic type params that have NO trait bounds.
    // Container types (ByDcaClass<T>, Price<U>) have unbounded params.
    // Leaf types (LazyPercentiles<I: VecIndex, T: ComputedVecValue, ...>) have bounded params.
    // Only generate ReadOnlyClone for container-like types (all params unbounded).
    let type_params: Vec<&syn::TypeParam> = generics
        .params
        .iter()
        .filter_map(|p| match p {
            syn::GenericParam::Type(tp) => Some(tp),
            _ => None,
        })
        .collect();

    if type_params.is_empty() {
        return quote! {};
    }

    // If any type param has bounds (inline or in where clause), skip —
    // this is a leaf/computation type, not a container.
    if type_params.iter().any(|tp| !tp.bounds.is_empty()) {
        return quote! {};
    }
    // Also check where clause for bounds on any type param.
    if let Some(where_clause) = &generics.where_clause {
        let param_names: Vec<&syn::Ident> = type_params.iter().map(|tp| &tp.ident).collect();
        let has_where_bounds = where_clause.predicates.iter().any(|pred| {
            if let syn::WherePredicate::Type(pt) = pred
                && let Type::Path(tp) = &pt.bounded_ty
                && let Some(seg) = tp.path.segments.first()
            {
                return param_names.iter().any(|p| seg.ident == **p);
            }
            false
        });
        if has_where_bounds {
            return quote! {};
        }
    }

    let param_idents: Vec<&syn::Ident> = type_params.iter().map(|tp| &tp.ident).collect();

    gen_read_only_clone_for_generics(name, generics, data, &param_idents)
}

/// Generate `ReadOnlyClone` for types with `M: StorageMode`.
fn gen_read_only_clone_for_m(
    name: &syn::Ident,
    generics: &syn::Generics,
    data: &syn::DataStruct,
    mode_param: &syn::Ident,
) -> proc_macro2::TokenStream {
    // Impl generics: all params except M, with bounds but without defaults.
    let impl_params: Vec<proc_macro2::TokenStream> = generics
        .params
        .iter()
        .filter_map(|p| match p {
            syn::GenericParam::Type(tp) if tp.ident == *mode_param => None,
            syn::GenericParam::Type(tp) => {
                let ident = &tp.ident;
                let bounds = &tp.bounds;
                if bounds.is_empty() {
                    Some(quote! { #ident })
                } else {
                    Some(quote! { #ident: #bounds })
                }
            }
            syn::GenericParam::Lifetime(lt) => Some(quote! { #lt }),
            syn::GenericParam::Const(c) => {
                let ident = &c.ident;
                let ty = &c.ty;
                Some(quote! { const #ident: #ty })
            }
        })
        .collect();

    // Type args with M replaced by Rw / Ro.
    let make_ty_args = |replacement: proc_macro2::TokenStream| -> Vec<proc_macro2::TokenStream> {
        generics
            .params
            .iter()
            .map(|p| match p {
                syn::GenericParam::Type(tp) if tp.ident == *mode_param => replacement.clone(),
                syn::GenericParam::Type(tp) => {
                    let id = &tp.ident;
                    quote! { #id }
                }
                syn::GenericParam::Lifetime(lt) => {
                    let lt = &lt.lifetime;
                    quote! { #lt }
                }
                syn::GenericParam::Const(c) => {
                    let id = &c.ident;
                    quote! { #id }
                }
            })
            .collect()
    };

    let ty_args_rw = make_ty_args(quote! { vecdb::Rw });
    let ty_args_ro = make_ty_args(quote! { vecdb::Ro });

    let where_clause = &generics.where_clause;

    let body = match &data.fields {
        Fields::Named(named) => {
            let field_conversions: Vec<_> = named
                .named
                .iter()
                .map(|f| {
                    let field_name = f.ident.as_ref().unwrap();
                    if is_field_skipped(f) && is_option_type(&f.ty) {
                        quote! { #field_name: None }
                    } else if type_contains_ident(&f.ty, mode_param) {
                        if is_box_type(&f.ty) {
                            quote! { #field_name: Box::new(vecdb::ReadOnlyClone::read_only_clone(&*self.#field_name)) }
                        } else {
                            quote! { #field_name: vecdb::ReadOnlyClone::read_only_clone(&self.#field_name) }
                        }
                    } else {
                        quote! { #field_name: self.#field_name.clone() }
                    }
                })
                .collect();
            quote! { #name { #(#field_conversions,)* } }
        }
        Fields::Unnamed(unnamed) => {
            let field_conversions: Vec<_> = unnamed
                .unnamed
                .iter()
                .enumerate()
                .map(|(i, f)| {
                    let idx = syn::Index::from(i);
                    if is_field_skipped(f) && is_option_type(&f.ty) {
                        quote! { None }
                    } else if type_contains_ident(&f.ty, mode_param) {
                        if is_box_type(&f.ty) {
                            quote! { Box::new(vecdb::ReadOnlyClone::read_only_clone(&*self.#idx)) }
                        } else {
                            quote! { vecdb::ReadOnlyClone::read_only_clone(&self.#idx) }
                        }
                    } else {
                        quote! { self.#idx.clone() }
                    }
                })
                .collect();
            quote! { #name(#(#field_conversions,)*) }
        }
        Fields::Unit => quote! { #name },
    };

    let impl_generics = if impl_params.is_empty() {
        quote! {}
    } else {
        quote! { <#(#impl_params),*> }
    };

    quote! {
        impl #impl_generics vecdb::ReadOnlyClone for #name<#(#ty_args_rw),*> #where_clause {
            type ReadOnly = #name<#(#ty_args_ro),*>;

            fn read_only_clone(&self) -> Self::ReadOnly {
                #body
            }
        }
    }
}

/// Check if a field has `#[traversable(skip)]`.
fn is_field_skipped(field: &syn::Field) -> bool {
    field.attrs.iter().any(|attr| {
        attr.path().is_ident("traversable")
            && attr.parse_args::<syn::Ident>().is_ok_and(|id| id == "skip")
    })
}

/// Generate `ReadOnlyClone` for types with generic type params but no `M: StorageMode`.
///
/// Each generic type param T gets a `ReadOnlyClone` bound.
/// `type ReadOnly = Self<T::ReadOnly, ...>` for each type param.
/// Fields containing any type param use `.read_only_clone()`, others use `.clone()`.
fn gen_read_only_clone_for_generics(
    name: &syn::Ident,
    generics: &syn::Generics,
    data: &syn::DataStruct,
    type_params: &[&syn::Ident],
) -> proc_macro2::TokenStream {
    // Check if any field actually references a type param (otherwise skip).
    let has_generic_field = match &data.fields {
        Fields::Named(named) => named
            .named
            .iter()
            .any(|f| type_params.iter().any(|tp| type_contains_ident(&f.ty, tp))),
        Fields::Unnamed(unnamed) => unnamed
            .unnamed
            .iter()
            .any(|f| type_params.iter().any(|tp| type_contains_ident(&f.ty, tp))),
        Fields::Unit => false,
    };

    if !has_generic_field {
        return quote! {};
    }

    // Impl generics: add ReadOnlyClone bound to type params.
    let impl_params: Vec<proc_macro2::TokenStream> = generics
        .params
        .iter()
        .map(|p| match p {
            syn::GenericParam::Type(tp) => {
                let ident = &tp.ident;
                let bounds = &tp.bounds;
                if bounds.is_empty() {
                    quote! { #ident: vecdb::ReadOnlyClone }
                } else {
                    quote! { #ident: #bounds + vecdb::ReadOnlyClone }
                }
            }
            syn::GenericParam::Lifetime(lt) => quote! { #lt },
            syn::GenericParam::Const(c) => {
                let ident = &c.ident;
                let ty = &c.ty;
                quote! { const #ident: #ty }
            }
        })
        .collect();

    // Self type args (just the param names).
    let self_ty_args: Vec<proc_macro2::TokenStream> = generics
        .params
        .iter()
        .map(|p| match p {
            syn::GenericParam::Type(tp) => {
                let id = &tp.ident;
                quote! { #id }
            }
            syn::GenericParam::Lifetime(lt) => {
                let lt = &lt.lifetime;
                quote! { #lt }
            }
            syn::GenericParam::Const(c) => {
                let id = &c.ident;
                quote! { #id }
            }
        })
        .collect();

    // ReadOnly type args: replace each type param T with <T as ReadOnlyClone>::ReadOnly.
    let ro_ty_args: Vec<proc_macro2::TokenStream> = generics
        .params
        .iter()
        .map(|p| match p {
            syn::GenericParam::Type(tp) => {
                let id = &tp.ident;
                quote! { <#id as vecdb::ReadOnlyClone>::ReadOnly }
            }
            syn::GenericParam::Lifetime(lt) => {
                let lt = &lt.lifetime;
                quote! { #lt }
            }
            syn::GenericParam::Const(c) => {
                let id = &c.ident;
                quote! { #id }
            }
        })
        .collect();

    let where_clause = &generics.where_clause;

    // Field-level: if field type contains any type param → read_only_clone, else → clone.
    let field_contains_any_param =
        |ty: &Type| type_params.iter().any(|tp| type_contains_ident(ty, tp));

    let body = match &data.fields {
        Fields::Named(named) => {
            let field_conversions: Vec<_> = named
                .named
                .iter()
                .map(|f| {
                    let field_name = f.ident.as_ref().unwrap();
                    if is_field_skipped(f) {
                        quote! { #field_name: Default::default() }
                    } else if field_contains_any_param(&f.ty) {
                        if is_box_type(&f.ty) {
                            quote! { #field_name: Box::new(vecdb::ReadOnlyClone::read_only_clone(&*self.#field_name)) }
                        } else {
                            quote! { #field_name: vecdb::ReadOnlyClone::read_only_clone(&self.#field_name) }
                        }
                    } else {
                        quote! { #field_name: self.#field_name.clone() }
                    }
                })
                .collect();
            quote! { #name { #(#field_conversions,)* } }
        }
        Fields::Unnamed(unnamed) => {
            let field_conversions: Vec<_> = unnamed
                .unnamed
                .iter()
                .enumerate()
                .map(|(i, f)| {
                    let idx = syn::Index::from(i);
                    if is_field_skipped(f) {
                        quote! { Default::default() }
                    } else if field_contains_any_param(&f.ty) {
                        if is_box_type(&f.ty) {
                            quote! { Box::new(vecdb::ReadOnlyClone::read_only_clone(&*self.#idx)) }
                        } else {
                            quote! { vecdb::ReadOnlyClone::read_only_clone(&self.#idx) }
                        }
                    } else {
                        quote! { self.#idx.clone() }
                    }
                })
                .collect();
            quote! { #name(#(#field_conversions,)*) }
        }
        Fields::Unit => quote! { #name },
    };

    quote! {
        impl<#(#impl_params),*> vecdb::ReadOnlyClone for #name<#(#self_ty_args),*> #where_clause {
            type ReadOnly = #name<#(#ro_ty_args),*>;

            fn read_only_clone(&self) -> Self::ReadOnly {
                #body
            }
        }
    }
}
