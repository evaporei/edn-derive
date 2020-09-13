use crate::edn::to_edn_keyword;
use crate::enums::get_enum_variants;
use crate::structs::get_struct_fields;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Data, Ident};

pub fn expand(type_name: &Ident, data: &Data) -> TokenStream2 {
    match data {
        Data::Struct(ref data_struct) => {
            let struct_fields = get_struct_fields(data_struct);

            let it = struct_fields.iter().map(|field| {
                let name = &field.ident;
                let keyword = to_edn_keyword(format!("{}", quote! {#name}));
                quote! {
                    format!("{} {}, ", #keyword, self.#name.serialize())
                }
            });

            quote! {
                impl edn_rs::Serialize for #type_name {
                    fn serialize(self) -> String {
                        let mut s = String::new();
                        s.push_str("{ ");
                        #(s.push_str(&#it);)*
                        s.push_str("}");
                        s
                    }
                }
            }
        }
        Data::Enum(ref data_enum) => {
            let enum_variants = get_enum_variants(data_enum);

            let it = enum_variants.iter().map(|variant| {
                let name = &variant.ident;
                let keyword =
                    to_edn_keyword(format!("{}/{}", quote! {#type_name}, quote! {#name}));
                quote! {
                    Self::#name => #keyword.to_string(),
                }
            });

            quote! {
                impl edn_rs::Serialize for #type_name {
                    fn serialize(self) -> String {
                        match self {
                            #(#it)*
                        }
                    }
                }
            }
        }
        _ => unimplemented!(),
    }
}
