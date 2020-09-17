pub fn field_to_keyword(field_name: String) -> String {
    let mut keyword = field_name
        .to_lowercase()
        .replace("___", "/")
        .replace("__", ".")
        .replace("_", "-");
    keyword.insert(0, ':');
    keyword
}

#[test]
fn test_field_to_keyword_lowercase() {
    assert_eq!(field_to_keyword("name".to_string()), ":name");
    assert_eq!(field_to_keyword("crux__db___id".to_string()), ":crux.db/id");
    assert_eq!(
        field_to_keyword("account___amount".to_string()),
        ":account/amount"
    );
    assert_eq!(field_to_keyword("tx___tx_time".to_string()), ":tx/tx-time");
}

#[test]
fn test_field_to_keyword_mixedcase() {
    assert_eq!(field_to_keyword("Name".to_string()), ":name");
    assert_eq!(field_to_keyword("Crux__dB___id".to_string()), ":crux.db/id");
    assert_eq!(
        field_to_keyword("acCount___amouNt".to_string()),
        ":account/amount"
    );
    assert_eq!(field_to_keyword("tX___tx_timE".to_string()), ":tx/tx-time");
}

#[test]
fn test_field_to_keyword_uppercase() {
    assert_eq!(field_to_keyword("NAME".to_string()), ":name");
    assert_eq!(field_to_keyword("CRUX__DB___ID".to_string()), ":crux.db/id");
    assert_eq!(
        field_to_keyword("ACCOUNT___AMOUNT".to_string()),
        ":account/amount"
    );
    assert_eq!(field_to_keyword("TX___TX_TIME".to_string()), ":tx/tx-time");
}
