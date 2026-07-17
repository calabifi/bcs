use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, DeriveInput, GenericParam, Generics, Lifetime};

/// Adds a trait bound to every type parameter.
fn add_type_bound(mut generics: Generics, bound: proc_macro2::TokenStream) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(type_param) = param {
            type_param.bounds.push(syn::parse2(bound.clone()).unwrap());
        }
    }
    generics
}

/// Builds impl generics for `TBCSDeserialize<'de>`:
/// - inserts `'de` first
/// - for each existing lifetime `'a` on the type, adds `'de: 'a`
///   (same relationship serde requires for borrowed data)
fn deserialize_generics(mut generics: Generics) -> Generics {
    let existing_lifetimes: Vec<Lifetime> = generics
        .params
        .iter()
        .filter_map(|param| match param {
            GenericParam::Lifetime(lifetime) => Some(lifetime.lifetime.clone()),
            _ => None,
        })
        .collect();

    generics.params.insert(0, syn::parse_quote!('de));

    if !existing_lifetimes.is_empty() {
        let where_clause = generics.make_where_clause();
        for lifetime in existing_lifetimes {
            where_clause
                .predicates
                .push(syn::parse_quote!('de: #lifetime));
        }
    }

    generics
}

#[proc_macro_derive(BcsSerialize)]
pub fn derive_bcs_serialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let generics = add_type_bound(input.generics, quote!(calabi_bcs::TBCSSerialize));
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        impl #impl_generics calabi_bcs::TBCSSerialize for #name #ty_generics #where_clause {}
    }
    .into()
}

#[proc_macro_derive(BcsDeserialize)]
pub fn derive_bcs_deserialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    // Keep type generics from the original type (without `'de`); `'de` belongs
    // only on the impl / trait side, matching how `Bcs` expands deserialize.
    let (_, ty_generics, _) = input.generics.split_for_impl();
    let ty_generics = ty_generics.to_token_stream();

    let de_generics = add_type_bound(input.generics, quote!(calabi_bcs::TBCSDeserialize<'de>));
    let de_generics = deserialize_generics(de_generics);
    let (impl_generics, _, where_clause) = de_generics.split_for_impl();

    quote! {
        impl #impl_generics calabi_bcs::TBCSDeserialize<'de> for #name #ty_generics #where_clause {}
    }
    .into()
}

#[proc_macro_derive(Bcs)]
pub fn derive_bcs(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let ser_generics = add_type_bound(input.generics.clone(), quote!(calabi_bcs::TBCSSerialize));
    let (ser_impl_g, ty_g, ser_where) = ser_generics.split_for_impl();

    let de_generics = add_type_bound(input.generics, quote!(calabi_bcs::TBCSDeserialize<'de>));
    let de_generics = deserialize_generics(de_generics);
    let (de_impl_g, _, de_where) = de_generics.split_for_impl();

    quote! {
        impl #ser_impl_g calabi_bcs::TBCSSerialize for #name #ty_g #ser_where {}
        impl #de_impl_g calabi_bcs::TBCSDeserialize<'de> for #name #ty_g #de_where {}
    }
    .into()
}
