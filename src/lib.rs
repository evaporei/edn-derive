use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, punctuated::Punctuated, token::Comma, Data, DeriveInput, Field, Fields,
};

#[proc_macro_derive(Serialize)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = input.ident;

    let struct_fields = get_struct_fields(&input.data);

    let it = struct_fields.iter().map(|field| {
        let name = &field.ident;
        let keyword = to_edn_keyword(format!("{}", quote! {#name}));
        quote! {
            format!("{} {} ", #keyword, self.#name.serialize())
        }
    });

    let expanded = quote! {
        impl edn_rs::Serialize for #struct_name {
            fn serialize(self) -> String {
                let mut s = String::new();
                s.push_str("{");
                #(s.push_str(&#it);)*
                s.push_str("}");
                s
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
