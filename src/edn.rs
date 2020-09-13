pub fn to_edn_keyword(field_name: String) -> String {
    let mut keyword = field_name
        .to_lowercase()
        .replace("___", "/")
        .replace("__", ".")
        .replace("_", "-");
    keyword.insert(0, ':');
    keyword
}

#[test]
fn test_to_edn_keyword_lowercase() {
    assert_eq!(to_edn_keyword("name".to_string()), ":name");
    assert_eq!(to_edn_keyword("crux__db___id".to_string()), ":crux.db/id");
    assert_eq!(
        to_edn_keyword("account___amount".to_string()),
        ":account/amount"
    );
    assert_eq!(to_edn_keyword("tx___tx_time".to_string()), ":tx/tx-time");
}

#[test]
fn test_to_edn_keyword_mixedcase() {
    assert_eq!(to_edn_keyword("Name".to_string()), ":name");
    assert_eq!(to_edn_keyword("Crux__dB___id".to_string()), ":crux.db/id");
    assert_eq!(
        to_edn_keyword("acCount___amouNt".to_string()),
        ":account/amount"
    );
    assert_eq!(to_edn_keyword("tX___tx_timE".to_string()), ":tx/tx-time");
}

#[test]
fn test_to_edn_keyword_uppercase() {
    assert_eq!(to_edn_keyword("NAME".to_string()), ":name");
    assert_eq!(to_edn_keyword("CRUX__DB___ID".to_string()), ":crux.db/id");
    assert_eq!(
        to_edn_keyword("ACCOUNT___AMOUNT".to_string()),
        ":account/amount"
    );
    assert_eq!(to_edn_keyword("TX___TX_TIME".to_string()), ":tx/tx-time");
}
