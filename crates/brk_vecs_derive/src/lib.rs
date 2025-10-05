use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Type, parse_macro_input};

#[proc_macro_derive(IVecs, attributes(vecs))]
pub fn derive_ivecs(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, _) = generics.split_for_impl();

    // Build extended where clause with Send + Sync bounds
    let generic_params = generics.type_params().map(|p| &p.ident);
    let original_predicates = &generics.where_clause.as_ref().map(|w| &w.predicates);

    let where_clause = if original_predicates.is_some() || generics.type_params().count() > 0 {
        quote! {
            where
                #(#generic_params: Send + Sync + std::fmt::Debug,)*
                #original_predicates
        }
    } else {
        quote! {}
    };

    let traverse_impl = match &input.data {
        Data::Struct(data) => {
            match &data.fields {
                Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                    // Special case for single-field tuple structs - just delegate
                    quote! {
                        impl #impl_generics IVecs for #name #ty_generics
                        #where_clause
                        {
                            fn to_tree_node(&self) -> brk_vecs::TreeNode {
                                self.0.to_tree_node()
                            }

                            fn iter(&self) -> impl Iterator<Item = &dyn vecdb::AnyCollectableVec> {
                                IVecs::iter(&self.0)
                            }
                        }
                    }
                }
                _ => {
                    // Normal struct with named fields
                    let field_traversals = generate_field_traversals(&data.fields);
                    let iterator_impl = generate_iterator_impl(&data.fields);

                    quote! {
                        impl #impl_generics IVecs for #name #ty_generics
                        #where_clause
                        {
                            fn to_tree_node(&self) -> brk_vecs::TreeNode {
                                let mut children = std::collections::HashMap::new();
                                #field_traversals
                                brk_vecs::TreeNode::Branch(children)
                            }

                            #iterator_impl
                        }
                    }
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
        return ident.ends_with("Vec") || ident.starts_with("LazyVecFrom");
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

                // Check for Option<inner> types
                if let Some(inner_ty) = get_option_inner_type(&f.ty) {
                    if is_vec_like(inner_ty) {
                        // Option<Vec> or Option<Box<Vec>>
                        let vec_access = if get_box_inner_type(inner_ty).is_some() {
                            quote! { vec.as_ref() }
                        } else {
                            quote! { vec }
                        };

                        return Some(quote! {
                            if let Some(ref vec) = self.#field_name {
                                children.insert(
                                    String::from(#field_name_str),
                                    brk_vecs::TreeNode::Leaf(vecdb::AnyVec::name(#vec_access).to_string())
                                );
                            }
                        });
                    } else {
                        // Option<nested_struct>
                        return Some(quote! {
                            if let Some(ref nested) = self.#field_name {
                                children.insert(
                                    String::from(#field_name_str),
                                    nested.to_tree_node()
                                );
                            }
                        });
                    }
                }

                // Check for direct vec or Box<vec>
                if is_vec_like(&f.ty) {
                    let vec_access = if get_box_inner_type(&f.ty).is_some() {
                        quote! { self.#field_name.as_ref() }
                    } else {
                        quote! { &self.#field_name }
                    };

                    Some(quote! {
                        children.insert(
                            String::from(#field_name_str),
                            brk_vecs::TreeNode::Leaf(vecdb::AnyVec::name(#vec_access).to_string())
                        );
                    })
                } else {
                    // Direct nested struct
                    Some(quote! {
                        children.insert(
                            String::from(#field_name_str),
                            self.#field_name.to_tree_node()
                        );
                    })
                }
            });

            quote! { #(#traversals)* }
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

fn is_vec_like(ty: &Type) -> bool {
    if is_vec_type(ty) {
        return true;
    }
    if let Some(inner) = get_box_inner_type(ty) {
        return is_vec_type(inner);
    }
    false
}

fn generate_iterator_impl(fields: &Fields) -> proc_macro2::TokenStream {
    match fields {
        Fields::Named(fields) => {
            let mut direct_vecs = Vec::new();
            let mut option_vecs = Vec::new();
            let mut option_box_vecs = Vec::new();

            for field in fields.named.iter() {
                if let Some(field_name) = &field.ident {
                    if !matches!(field.vis, syn::Visibility::Public(_)) {
                        continue;
                    }

                    if let Some(inner_ty) = get_option_inner_type(&field.ty) {
                        if is_vec_type(inner_ty) {
                            // Option<Vec> - use as_ref()
                            option_vecs.push(field_name);
                        } else if let Some(box_inner) = get_box_inner_type(inner_ty)
                            && is_vec_type(box_inner)
                        {
                            // Option<Box<Vec>> - use as_deref()
                            option_box_vecs.push(field_name);
                        }
                    } else if is_vec_like(&field.ty) {
                        // Direct Vec or Box<Vec>
                        direct_vecs.push(field_name);
                    }
                }
            }

            if direct_vecs.is_empty() && option_vecs.is_empty() && option_box_vecs.is_empty() {
                quote! {
                    fn iter(&self) -> impl Iterator<Item = &dyn vecdb::AnyCollectableVec> {
                        std::iter::empty()
                    }
                }
            } else {
                let direct_part = if !direct_vecs.is_empty() {
                    quote! {
                        let direct_iter = [
                            #(&self.#direct_vecs as &dyn vecdb::AnyCollectableVec,)*
                        ].into_iter();
                    }
                } else {
                    quote! {
                        let direct_iter = std::iter::empty();
                    }
                };

                let option_part = if !option_vecs.is_empty() {
                    quote! {
                        let option_iter = [
                            #((&self.#option_vecs).as_ref().map(|x| x as &dyn vecdb::AnyCollectableVec),)*
                        ]
                        .into_iter()
                        .flatten();
                    }
                } else {
                    quote! {
                        let option_iter = std::iter::empty();
                    }
                };

                let option_box_part = if !option_box_vecs.is_empty() {
                    quote! {
                        let option_box_iter = [
                            #(self.#option_box_vecs.as_deref(),)*
                        ]
                        .into_iter()
                        .flatten()
                        .map(|x| x as &dyn vecdb::AnyCollectableVec);
                    }
                } else {
                    quote! {
                        let option_box_iter = std::iter::empty();
                    }
                };

                quote! {
                    fn iter(&self) -> impl Iterator<Item = &dyn vecdb::AnyCollectableVec> {
                        #direct_part
                        #option_part
                        #option_box_part
                        direct_iter.chain(option_iter).chain(option_box_iter)
                    }
                }
            }
        }
        _ => quote! {
            fn iter(&self) -> impl Iterator<Item = &dyn vecdb::AnyCollectableVec> {
                std::iter::empty()
            }
        },
    }
}

fn get_box_inner_type(ty: &Type) -> Option<&Type> {
    if let Type::Path(type_path) = ty
        && let Some(segment) = type_path.path.segments.last()
        && segment.ident == "Box"
        && let syn::PathArguments::AngleBracketed(args) = &segment.arguments
        && let Some(syn::GenericArgument::Type(inner_ty)) = args.args.first()
    {
        return Some(inner_ty);
    }
    None
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
