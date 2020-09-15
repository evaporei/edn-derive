use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod deserialize;
mod edn;
mod enums;
mod serialize;
mod structs;

#[proc_macro_derive(Serialize)]
pub fn derive_serialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let type_name = input.ident;

    let expanded = serialize::expand(&type_name, &input.data);

    expanded.into()
}

#[proc_macro_derive(Deserialize)]
pub fn derive_deserialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let type_name = input.ident;

    let expanded = deserialize::expand(&type_name, &input.data);

    expanded.into()
}
