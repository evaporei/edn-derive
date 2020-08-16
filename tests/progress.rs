#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/simple.rs");
    t.pass("tests/to_string.rs");
}
