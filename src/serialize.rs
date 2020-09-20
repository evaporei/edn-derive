use crate::edn;
use crate::enums::get_enum_variants;
use crate::structs::get_struct_fields;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Data, DataEnum, DataStruct, Error, Ident};

pub fn expand(type_name: &Ident, data: &Data) -> Result<TokenStream2, Error> {
    match data {
        Data::Struct(ref data_struct) => Ok(expand_struct(type_name, data_struct)),
        Data::Enum(ref data_enum) => Ok(expand_enum(type_name, data_enum)),
        Data::Union(ref data_union) => Err(Error::new(
            data_union.union_token.span,
            "edn-derive does not support derive for unions",
        )),
    }
}

fn expand_struct(struct_name: &Ident, data_struct: &DataStruct) -> TokenStream2 {
    let struct_fields = get_struct_fields(data_struct);

    let it = struct_fields.iter().map(|field| {
        let name = &field.ident;
        let keyword = edn::field_to_keyword(&quote! {#name}.to_string());
        quote! {
            format!("{} {}, ", #keyword, self.#name.serialize())
        }
    });

    quote! {
        impl edn_rs::Serialize for #struct_name {
            fn serialize(self) -> std::string::String {
                let mut s = std::string::String::new();
                s.push_str("{ ");
                #(s.push_str(&#it);)*
                s.push_str("}");
                s
            }
        }
    }
}

fn expand_enum(enum_name: &Ident, data_enum: &DataEnum) -> TokenStream2 {
    let enum_variants = get_enum_variants(data_enum);

    let it = enum_variants.iter().map(|variant| {
        let name = &variant.ident;
        let keyword = edn::enum_to_keyword(
            &quote! {#enum_name}.to_string(),
            &quote! {#name}.to_string(),
        );
        quote! {
            Self::#name => #keyword.to_string(),
        }
    });

    quote! {
        impl edn_rs::Serialize for #enum_name {
            fn serialize(self) -> std::string::String {
                match self {
                    #(#it)*
                }
            }
        }
    }
}
