use crate::edn::to_edn_keyword;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{punctuated::Punctuated, token::Comma, DataStruct, Field, Fields};

pub fn get_struct_fields(data_struct: &DataStruct) -> &Punctuated<Field, Comma> {
    match data_struct.fields {
        Fields::Named(ref fields) => &fields.named,
        _ => unimplemented!(),
    }
}

pub fn generate_field_deserialization(fields: &Punctuated<Field, Comma>) -> TokenStream2 {
    fields
        .iter()
        .map(|f| {
            let name = &f.ident;
            let keyword = to_edn_keyword(format!("{}", quote! {#name}));

            quote! {
                #name: edn_rs::from_edn(&edn[#keyword])?,
            }
        })
        .collect()
}
