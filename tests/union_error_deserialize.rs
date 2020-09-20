use edn_derive::Deserialize;

#[derive(Deserialize)]
union BadType {
    value: usize,
}

fn main() {}
