use crate::enums::{generate_variant_deserialization, get_enum_variants};
use crate::structs;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    punctuated::Punctuated, token::Comma, Data, DataEnum, DataStruct, Error, Field, Fields, Ident,
};

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
    match data_struct.fields {
        Fields::Named(ref fields) => expand_named_struct(struct_name, &fields.named),
        Fields::Unnamed(ref fields) => expand_unnamed_struct(struct_name, &fields.unnamed),
        Fields::Unit => expand_unit_struct(struct_name),
    }
}

fn expand_named_struct(struct_name: &Ident, fields: &Punctuated<Field, Comma>) -> TokenStream2 {
    let deserialized_fields = structs::named_field_deserialization(&fields);
    quote! {
        impl edn_rs::Deserialize for #struct_name {
            fn deserialize(edn: &edn_rs::Edn) -> std::result::Result<Self, edn_rs::EdnError> {
                std::result::Result::Ok(Self {
                    #deserialized_fields
                })
            }
        }
    }
}

fn expand_unnamed_struct(struct_name: &Ident, fields: &Punctuated<Field, Comma>) -> TokenStream2 {
    let deserialized_fields = structs::unnamed_field_deserialization(&fields);
    quote! {
        impl edn_rs::Deserialize for #struct_name {
            fn deserialize(edn: &edn_rs::Edn) -> std::result::Result<Self, edn_rs::EdnError> {
                std::result::Result::Ok(Self {
                    #deserialized_fields
                })
            }
        }
    }
}

fn expand_unit_struct(struct_name: &Ident) -> TokenStream2 {
    quote! {
        impl edn_rs::Deserialize for #struct_name {
            fn deserialize(edn: &edn_rs::Edn) -> std::result::Result<Self, edn_rs::EdnError> {
                match edn {
                    edn_rs::Edn::Nil => std::result::Result::Ok(Self),
                    _ => std::result::Result::Err(edn_rs::EdnError::Deserialize(format!(
                                "couldn't convert {} into an unit struct",
                                edn
                    )))
                }
            }
        }
    }
}

fn expand_enum(enum_name: &Ident, data_enum: &DataEnum) -> TokenStream2 {
    let enum_variants = get_enum_variants(data_enum);

    let deserialized_variants = generate_variant_deserialization(enum_name, &enum_variants);

    quote! {
        impl edn_rs::Deserialize for #enum_name {
            fn deserialize(edn: &edn_rs::Edn) -> std::result::Result<Self, edn_rs::EdnError> {
                match edn {
                    edn_rs::Edn::Key(k) => match &k[..] {
                        #deserialized_variants
                        _ => std::result::Result::Err(edn_rs::EdnError::Deserialize(format!(
                                "couldn't convert {} keyword into enum",
                                k
                        )))
                    },
                    edn_rs::Edn::Str(s) => match &s[..] {
                        #deserialized_variants
                        _ => std::result::Result::Err(edn_rs::EdnError::Deserialize(format!(
                                "couldn't convert {} string into enum",
                                s
                        ))),
                    },
                    _ => std::result::Result::Err(edn_rs::EdnError::Deserialize(format!(
                                "couldn't convert {} into enum",
                                edn
                    ))),
                }
            }
        }
    }
}
