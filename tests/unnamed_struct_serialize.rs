use edn_derive::Serialize;

#[derive(Serialize)]
enum Kind {
    Cool,
    Chill,
    Pirate,
}

#[derive(Serialize)]
pub struct Person(String, usize, Kind);

fn main() {
    let person = Person("joana".to_string(), 290000, Kind::Chill);

    assert_eq!(
        edn_rs::to_string(person),
        ":person{ 0 \"joana\", 1 290000, 2 :kind/chill, }"
    );
}

