use crate::enums::{generate_variant_deserialization, get_enum_variants};
use crate::structs::{generate_field_deserialization, get_struct_fields};
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Data, Ident};

pub fn expand(type_name: &Ident, data: &Data) -> TokenStream2 {
    match data {
        Data::Struct(ref data_struct) => {
            let struct_fields = get_struct_fields(data_struct);

            let deserialized_fields = generate_field_deserialization(&struct_fields);

            quote! {
                impl edn_rs::Deserialize for #type_name {
                    fn deserialize(edn: &edn_rs::Edn) -> Result<Self, edn_rs::EdnError> {
                        Ok(Self {
                            #deserialized_fields
                        })
                    }
                }
            }
        }
        Data::Enum(ref data_enum) => {
            let enum_variants = get_enum_variants(data_enum);

            let deserialized_variants =
                generate_variant_deserialization(&type_name, &enum_variants);

            quote! {
                impl edn_rs::Deserialize for #type_name {
                    fn deserialize(edn: &edn_rs::Edn) -> Result<Self, edn_rs::EdnError> {
                        match edn {
                            edn_rs::Edn::Key(k) => match &k[..] {
                                #deserialized_variants
                                _ => Err(edn_rs::EdnError::Deserialize(format!(
                                        "couldn't convert {} keyword into enum",
                                        k
                                ))),
                            },
                            edn_rs::Edn::Str(s) => match &s[..] {
                                #deserialized_variants
                                _ => Err(edn_rs::EdnError::Deserialize(format!(
                                        "couldn't convert {} string into enum",
                                        s
                                ))),
                            },
                            _ => Err(edn_rs::EdnError::Deserialize(format!(
                                        "couldn't convert {} into enum",
                                        edn
                            ))),
                        }
                    }
                }
            }
        }
        _ => unimplemented!(),
    }
}
