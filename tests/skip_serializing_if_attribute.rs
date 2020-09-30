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
    #[edn(skip_serializing_if = "Option::is_none")]
    age: Option<usize>,
    kind: Kind,
}

fn main() {
    let person = Person {
        name: "joana".to_string(),
        age: Some(290000),
        kind: Kind::Chill,
    };
    assert_eq!(
        edn_rs::to_string(person),
        "{ :name \"joana\", :age 290000, :kind :kind/chill, }"
    );

    let person = Person {
        name: "joana".to_string(),
        age: None,
        kind: Kind::Chill,
    };
    assert_eq!(
        edn_rs::to_string(person),
        "{ :name \"joana\", :kind :kind/chill, }"
    );
}
