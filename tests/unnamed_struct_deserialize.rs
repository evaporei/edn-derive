#![allow(dead_code)]

use edn_derive::Deserialize;
use edn_rs::EdnError;

// The `Debug` and `PartialEq` are only necessary because of `assert_eq`, you don't need them
#[derive(Deserialize, Debug, PartialEq)]
enum Kind {
    Cool,
    Chill,
    Pirate,
}

// The `Debug` and `PartialEq` are only necessary because of `assert_eq`, you don't need them
#[derive(Deserialize, Debug, PartialEq)]
pub struct Person(String, usize, Kind);

fn main() -> Result<(), EdnError> {
    let edn_person = "{ 0 \"joana\", 1 290000, 2 :kind/pirate, }";

    let person: Person = edn_rs::from_str(edn_person)?;

    assert_eq!(person, Person("joana".to_string(), 290000, Kind::Pirate));

    Ok(())
}
