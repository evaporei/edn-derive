use crate::edn;
use crate::enums::get_enum_variants;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{
    punctuated::Punctuated, token::Comma, Data, DataEnum, DataStruct, Error, Field, Fields, Ident,
    Lit, Meta, NestedMeta, Path,
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
    let it = fields.iter().map(|field| {
        let name = &field.ident;
        let keyword = edn::field_to_keyword(&quote! {#name}.to_string());

        for attr in &field.attrs {
            let meta = attr.parse_meta();
            match meta {
                Ok(Meta::List(list)) => {
                    let path = list.path;
                    if path.get_ident() == Some(&Ident::new("edn", Span::call_site())) {
                        for nested in list.nested {
                            match nested {
                                NestedMeta::Meta(m) => match m {
                                    Meta::NameValue(namevalue) => {
                                        if namevalue.path.get_ident() == Some(&Ident::new("skip_serializing_if", Span::call_site())){
                                            match namevalue.lit {
                                                Lit::Str(litstr) => {
                                                    let parsed: Path = litstr.parse().unwrap();

                                                    return quote!{if #parsed(&self.#name) {
                                                        None
                                                    } else {
                                                        Some(format!("{} {}, ", #keyword, self.#name.serialize()))
                                                    }};
                                                },
                                                _ => {}
                                            }

                                        }
                                    }
                                    _ => {}
                                }
                                _ => {},
                            }
                        }
                    }
                },
                _ => {},
            }
        }

        quote! {
            Some(format!("{} {}, ", #keyword, self.#name.serialize()))
        }
    });

    quote! {
        impl edn_rs::Serialize for #struct_name {
            fn serialize(self) -> std::string::String {
                let mut s = std::string::String::new();
                s.push_str("{ ");
                #(match #it {
                    Some(serialized) => s.push_str(&serialized),
                    None => {}
                };)*
                s.push_str("}");
                s
            }
        }
    }
}

fn expand_unnamed_struct(struct_name: &Ident, fields: &Punctuated<Field, Comma>) -> TokenStream2 {
    let it = fields.iter().enumerate().map(|(i, _)| {
        let i = syn::Index::from(i); // Eg: `0usize` to `0`
        quote! {
            format!("{} {}, ", #i, self.#i.serialize())
        }
    });

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

fn expand_unit_struct(struct_name: &Ident) -> TokenStream2 {
    quote! {
        impl edn_rs::Serialize for #struct_name {
            fn serialize(self) -> std::string::String {
                String::from("nil")
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
