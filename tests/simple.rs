use edn_derive::Serialize as SerializeEdn;
use edn_rs::Serialize;

#[derive(SerializeEdn)]
pub struct Person {
    name: String,
    age: usize,
}

fn main() {
    let person = Person {
        name: "joana".to_string(),
        age: 290000,
    };
    assert_eq!(person.serialize(), "{ :name \"joana\", :age 290000, }");
}
