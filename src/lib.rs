use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    parse_macro_input, punctuated::Punctuated, token::Comma, Data, DeriveInput, Field, Fields,
    Ident, Type,
};

#[proc_macro_derive(Serialize)]
pub fn derive_serialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = input.ident;

    let struct_fields = get_struct_fields(&input.data);

    let it = struct_fields.iter().map(|field| {
        let name = &field.ident;
        let keyword = to_edn_keyword(format!("{}", quote! {#name}));
        quote! {
            format!("{} {}, ", #keyword, self.#name.serialize())
        }
    });

    let expanded = quote! {
        impl edn_rs::Serialize for #struct_name {
            fn serialize(self) -> String {
                let mut s = String::new();
                s.push_str("{ ");
                #(s.push_str(&#it);)*
                s.push_str("}");
                s
            }
        }
    };

    expanded.into()
}

#[proc_macro_derive(Deserialize)]
pub fn derive_deserialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = input.ident;

    let struct_fields = get_struct_fields(&input.data);

    let deserialized_fields = generate_field_deserialization(&struct_fields);

    let expanded = quote! {
        impl edn_rs::Deserialize for #struct_name {
            fn deserialize(edn: edn_rs::Edn) -> Result<Self, edn_rs::EdnError> {
                Ok(Self {
                    #deserialized_fields
                })
            }
        }
    };

    expanded.into()
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

fn deserialize_for_type(type_ident: &Ident, keyword: &str) -> TokenStream2 {
    match &type_ident.to_string()[..] {
        "String" => quote! { to_string() },
        "usize" | "u8" | "u16" | "u32" | "u64" => {
            quote! { to_uint().ok_or_else(|| edn_rs::EdnError::Deserialize(format!("couldn't convert `{}` into `uint`", #keyword)))? }
        }
        "isize" | "i8" | "i16" | "i32" | "i64" => {
            quote! { to_int().ok_or_else(|| edn_rs::EdnError::Deserialize(format!("couldn't convert `{}` into `int`", #keyword)))? }
        }
        "f32" | "f64" => {
            quote! { to_float().ok_or_else(|| edn_rs::EdnError::Deserialize(format!("couldn't convert `{}` into `float`", #keyword)))? }
        }
        "bool" => {
            quote! { to_bool().ok_or_else(|| edn_rs::EdnError::Deserialize(format!("couldn't convert `{}` into `bool`", #keyword)))? }
        }
        "char" => {
            quote! { to_char().ok_or_else(|| edn_rs::EdnError::Deserialize(format!("couldn't convert `{}` into `char`", #keyword)))? }
        }
        _ => unimplemented!(),
    }
}

fn generate_field_deserialization(fields: &Punctuated<Field, Comma>) -> TokenStream2 {
    fields
        .iter()
        .map(|f| {
            let name = &f.ident;
            let ty = &f.ty;
            let keyword = to_edn_keyword(format!("{}", quote! {#name}));

            let type_ident = get_type_ident(ty).unwrap();

            let deserialization = deserialize_for_type(type_ident, &keyword);

            quote! {
                #name: edn[#keyword].#deserialization,
            }
        })
        .collect()
}

fn get_type_ident(ty: &Type) -> Option<&Ident> {
    if let Type::Path(typepath) = ty {
        if let Some(segment) = typepath.path.segments.last() {
            return Some(&segment.ident);
        }
    }

    None
}
