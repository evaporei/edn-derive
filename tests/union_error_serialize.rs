use edn_derive::Serialize;

#[derive(Serialize)]
union BadType {
    value: usize,
}

fn main() {}
