#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/serialize.rs");
    t.pass("tests/deserialize.rs");
    t.pass("tests/both_ways.rs");
    t.pass("tests/complex.rs");
}
