use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod deserialize;
mod edn;
mod enums;
mod serialize;
mod structs;

#[proc_macro_derive(Serialize, attributes(edn))]
pub fn derive_serialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let type_name = input.ident;

    serialize::expand(&type_name, &input.data)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

#[proc_macro_derive(Deserialize)]
pub fn derive_deserialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let type_name = input.ident;

    deserialize::expand(&type_name, &input.data)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}
