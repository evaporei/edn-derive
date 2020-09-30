use crate::edn;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{punctuated::Punctuated, token::Comma, Field};

pub fn named_field_deserialization(fields: &Punctuated<Field, Comma>) -> TokenStream2 {
    fields
        .iter()
        .map(|f| {
            let name = &f.ident;
            let keyword = edn::field_to_keyword(&quote! {#name}.to_string());

            quote! {
                #name: edn_rs::from_edn(&edn[#keyword])?,
            }
        })
        .collect()
}

pub fn unnamed_field_deserialization(fields: &Punctuated<Field, Comma>) -> TokenStream2 {
    fields
        .iter()
        .enumerate()
        .map(|(i, _)| {
            let i = syn::Index::from(i); // Eg: `0usize` to `0`

            quote! {
                #i: edn_rs::from_edn(&edn[#i])?,
            }
        })
        .collect()
}
