#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/serialize.rs");
    t.pass("tests/deserialize.rs");
    t.pass("tests/both_ways.rs");
    t.pass("tests/complex.rs");
    t.compile_fail("tests/union_error_serialize.rs");
}
