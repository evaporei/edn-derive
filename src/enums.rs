use crate::edn;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{punctuated::Punctuated, token::Comma, DataEnum, Ident, Variant};

pub const fn get_enum_variants(data_enum: &DataEnum) -> &Punctuated<Variant, Comma> {
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
            let keyword = edn::enum_to_keyword(
                &quote! {#enum_name}.to_string(),
                &quote! {#name}.to_string(),
            );

            quote! {
                #keyword => std::result::Result::Ok(Self::#name),
            }
        })
        .collect()
}
