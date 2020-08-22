use edn_derive::Deserialize;
use edn_rs::EdnError;

// The `Debug` and `PartialEq` are only necessary because of `assert_eq`, you don't need them
#[derive(Deserialize, Debug, PartialEq)]
pub struct Person {
    name: String,
    age: usize,
}

fn main() -> Result<(), EdnError> {
    let edn_person = "{ :name \"joana\", :age 290000, }";

    let person: Person = edn_rs::from_str(edn_person)?;

    assert_eq!(
        person,
        Person {
            name: "joana".to_string(),
            age: 290000,
        }
    );

    Ok(())
}
