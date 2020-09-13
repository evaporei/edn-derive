use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    parse_macro_input, punctuated::Punctuated, token::Comma, Data, DataStruct,
    DeriveInput, Field, Fields,
};

#[proc_macro_derive(Serialize)]
pub fn derive_serialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let type_name = input.ident;

    let expanded = match input.data {
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
        _ => unimplemented!(),
    };

    expanded.into()
}

#[proc_macro_derive(Deserialize)]
pub fn derive_deserialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = input.ident;

    let expanded = match input.data {
        Data::Struct(ref data_struct) => {
            let struct_fields = get_struct_fields(data_struct);

            let deserialized_fields = generate_field_deserialization(&struct_fields);

            quote! {
                impl edn_rs::Deserialize for #struct_name {
                    fn deserialize(edn: &edn_rs::Edn) -> Result<Self, edn_rs::EdnError> {
                        Ok(Self {
                            #deserialized_fields
                        })
                    }
                }
            }
        }
        _ => unimplemented!(),
    };

    expanded.into()
}

fn get_struct_fields(data_struct: &DataStruct) -> &Punctuated<Field, Comma> {
    match data_struct.fields {
        Fields::Named(ref fields) => &fields.named,
        _ => unimplemented!(),
    }
}

fn to_edn_keyword(field_name: String) -> String {
    let mut keyword = field_name
        .replace("___", "/")
        .replace("__", ".")
        .replace("_", "-");
    keyword.insert(0, ':');
    keyword
}

#[test]
fn test_to_edn_keyword() {
    assert_eq!(to_edn_keyword("name".to_string()), ":name");
    assert_eq!(to_edn_keyword("crux__db___id".to_string()), ":crux.db/id");
    assert_eq!(
        to_edn_keyword("account___amount".to_string()),
        ":account/amount"
    );
    assert_eq!(to_edn_keyword("tx___tx_time".to_string()), ":tx/tx-time");
}

fn generate_field_deserialization(fields: &Punctuated<Field, Comma>) -> TokenStream2 {
    fields
        .iter()
        .map(|f| {
            let name = &f.ident;
            let keyword = to_edn_keyword(format!("{}", quote! {#name}));

            quote! {
                #name: edn_rs::Deserialize::deserialize(&edn[#keyword])?,
            }
        })
        .collect()
}
