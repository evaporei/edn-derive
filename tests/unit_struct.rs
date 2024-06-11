use edn_derive::{Serialize, Deserialize};
use edn_rs::EdnError;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Nothing;

fn main() -> Result<(), EdnError> {
    let nothing = Nothing;

    assert_eq!(
        edn_rs::to_string(nothing),
        "nil"
    );

    let nothing: Nothing = edn_rs::from_str("nil")?;

    assert_eq!(
        nothing,
        Nothing
    );

    let nothing_err: Result<Nothing, EdnError> = edn_rs::from_str(":a-key");

    assert_eq!(
        nothing_err,
        Err(EdnError::Deserialize("couldn't convert :a-key into an unit struct".to_string()))
    );

    Ok(())
}
