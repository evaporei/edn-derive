use crate::edn;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{punctuated::Punctuated, token::Comma, DataStruct, Field, Fields};

pub fn get_struct_fields(data_struct: &DataStruct) -> Option<&Punctuated<Field, Comma>> {
    match data_struct.fields {
        Fields::Named(ref fields) => Some(&fields.named),
        Fields::Unnamed(ref fields) => Some(&fields.unnamed),
        Fields::Unit => None,
    }
}

pub fn generate_field_deserialization(fields: &Punctuated<Field, Comma>) -> TokenStream2 {
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
