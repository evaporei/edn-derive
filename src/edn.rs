pub fn field_to_keyword(field_name: &str) -> String {
    let mut keyword = String::from(':');
    let inner = field_name
        .replace("___", "/")
        .replace("__", ".")
        .replace("_", "-");
    keyword.push_str(&inner);
    keyword.to_lowercase()
}

fn camel_to_kebab(s: &str) -> String {
    s.chars()
        .enumerate()
        .fold(String::new(), |mut kebab, (i, c)| {
            if c.is_uppercase() {
                if i != 0 {
                    kebab.push('-');
                }
                kebab.push_str(&c.to_lowercase().collect::<String>());
            } else {
                kebab.push(c);
            }

            kebab
        })
}

#[test]
fn test_field_to_keyword_lowercase() {
    assert_eq!(field_to_keyword("name"), ":name");
    assert_eq!(field_to_keyword("crux__db___id"), ":crux.db/id");
    assert_eq!(field_to_keyword("account___amount"), ":account/amount");
    assert_eq!(field_to_keyword("tx___tx_time"), ":tx/tx-time");
}

#[test]
fn test_field_to_keyword_mixedcase() {
    assert_eq!(field_to_keyword("Name"), ":name");
    assert_eq!(field_to_keyword("Crux__dB___id"), ":crux.db/id");
    assert_eq!(field_to_keyword("acCount___amouNt"), ":account/amount");
    assert_eq!(field_to_keyword("tX___tx_timE"), ":tx/tx-time");
}

#[test]
fn test_field_to_keyword_uppercase() {
    assert_eq!(field_to_keyword("NAME"), ":name");
    assert_eq!(field_to_keyword("CRUX__DB___ID"), ":crux.db/id");
    assert_eq!(field_to_keyword("ACCOUNT___AMOUNT"), ":account/amount");
    assert_eq!(field_to_keyword("TX___TX_TIME"), ":tx/tx-time");
}

#[test]
fn test_camel_to_kebab() {
    assert_eq!(camel_to_kebab("CoolText"), "cool-text");
    assert_eq!(camel_to_kebab("Nice"), "nice");
}

pub fn enum_to_keyword(enum_name: &str, variant_name: &str) -> String {
    let mut keyword = String::from(':');
    keyword.push_str(&camel_to_kebab(enum_name));
    keyword.push('/');
    keyword.push_str(&camel_to_kebab(variant_name));
    keyword
}

#[test]
fn test_enum_to_keyword() {
    assert_eq!(
        enum_to_keyword("EnumName", "EnumVariant"),
        ":enum-name/enum-variant"
    );
}
