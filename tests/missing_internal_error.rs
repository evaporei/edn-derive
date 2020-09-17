use edn_derive::Serialize;

#[derive(Serialize)]
struct A {
    b: B,
}

struct B {
    name: String,
}

fn main() {}
