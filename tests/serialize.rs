#![allow(dead_code)]

use edn_derive::Serialize;

#[derive(Serialize)]
enum Kind {
    Cool,
    Chill,
    Pirate,
}

#[derive(Serialize)]
pub struct Person {
    name: String,
    age: usize,
    kind: Kind,
}

fn main() {
    let person = Person {
        name: "joana".to_string(),
        age: 290000,
        kind: Kind::Chill,
    };
    assert_eq!(
        edn_rs::to_string(person),
        "{ :name \"joana\", :age 290000, :kind :kind/chill, }"
    );
}
