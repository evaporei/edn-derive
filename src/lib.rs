use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

use quote::quote;
use syn::{
    parse_macro_input, punctuated::Punctuated, token::Comma, Data, DeriveInput, Field, Fields,
};
type Variants = syn::punctuated::Punctuated<syn::Field, syn::Token![,]>;

#[proc_macro_derive(Serialize)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = input.ident;

    let struct_fields = get_struct_fields(&input.data);

    let impl_ser = impl_ser(&struct_name, struct_fields);
    let impl_display = impl_display(&struct_name);
    let impl_to_edn = impl_to_edn(&struct_name);

    let expanded = quote! {
            #impl_ser
            #impl_display
            #impl_to_edn
    };

    expanded.into()
}

fn impl_ser(ident: &syn::Ident, struct_fields: &Variants) -> TokenStream2 {
    let it = struct_fields.iter().map(|field| {
        let name = &field.ident;
        let keyword = to_edn_keyword(format!("{}", quote! {#name}));
        quote! {
            format!("{} {}, ", #keyword, self.#name.serialize())
        }
    });

    quote! {
        impl edn_rs::Serialize for #ident {
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

fn impl_display(ident: &syn::Ident) -> TokenStream2 {
    quote! {
        use std::fmt;
        impl fmt::Display for #ident {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.clone().serialize())
            }
         }
    }
}

fn impl_to_edn(ident: &syn::Ident) -> TokenStream2 {
    quote! {
        impl #ident {
            fn to_edn(&self) -> String {
                self.clone().serialize()
            }
         }
    }
}

fn get_struct_fields(data: &Data) -> &Punctuated<Field, Comma> {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => &fields.named,
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    }
}

fn to_edn_keyword(field_name: String) -> String {
    let mut keyword = field_name
        .replace("_", "-")
        .replace("__", ".")
        .replace("___", "/");
    keyword.insert(0, ':');
    keyword
}
