use crate::edn::to_edn_keyword;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{punctuated::Punctuated, token::Comma, DataEnum, Ident, Variant};

pub fn get_enum_variants(data_enum: &DataEnum) -> &Punctuated<Variant, Comma> {
    &data_enum.variants
}

pub fn generate_variant_deserialization(
    enum_name: &Ident,
    variants: &Punctuated<Variant, Comma>,
) -> TokenStream2 {
    variants
        .iter()
        .map(|v| {
            let name = &v.ident;
            let keyword = to_edn_keyword(format!("{}/{}", quote! {#enum_name}, quote! {#name}));

            quote! {
                #keyword => Ok(Self::#name),
            }
        })
        .collect()
}
