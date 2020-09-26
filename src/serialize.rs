use crate::edn;
use crate::enums::get_enum_variants;
use crate::structs::get_struct_fields;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Data, DataEnum, DataStruct, Error, Fields, Ident};

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
    let maybe_fields = get_struct_fields(data_struct);

    match maybe_fields {
        Some(fields) => {
            let it = fields.iter().enumerate().map(|(i, field)| {
                let maybe_name = &field.ident;
                match maybe_name {
                    Some(name) => {
                        let keyword = edn::field_to_keyword(&quote! {#name}.to_string());

                        quote! {
                            format!("{} {}, ", #keyword, self.#name.serialize())
                        }
                    }
                    None => {
                        let i = syn::Index::from(i); // Eg: `0usize` to `0`
                        quote! {
                            format!("{} {}, ", #i, self.#i.serialize())
                        }
                    }
                }
            });

            match data_struct.fields {
                Fields::Named(_) => quote! {
                    impl edn_rs::Serialize for #struct_name {
                        fn serialize(self) -> std::string::String {
                            let mut s = std::string::String::new();
                            s.push_str("{ ");
                            #(s.push_str(&#it);)*
                            s.push_str("}");
                            s
                        }
                    }
                },
                Fields::Unnamed(_) => {
                    let str_struct_name = edn::camel_to_kebab(&quote! {#struct_name}.to_string());

                    quote! {
                        impl edn_rs::Serialize for #struct_name {
                            fn serialize(self) -> std::string::String {
                                let mut s = std::string::String::from(':');
                                s.push_str(#str_struct_name);
                                s.push_str("{ ");
                                #(s.push_str(&#it);)*
                                s.push_str("}");
                                s
                            }
                        }
                    }
                }
                _ => unreachable!(),
            }
        }
        None => quote! {
            impl edn_rs::Serialize for #struct_name {
                fn serialize(self) -> std::string::String {
                    String::from("nil")
                }
            }
        },
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
