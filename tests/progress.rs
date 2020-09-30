#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/serialize.rs");
    t.pass("tests/deserialize.rs");
    t.pass("tests/both_ways.rs");
    t.pass("tests/complex.rs");
    t.compile_fail("tests/union_error_serialize.rs");
    t.compile_fail("tests/union_error_deserialize.rs");
    t.pass("tests/unit_struct.rs");
    t.pass("tests/unnamed_struct_serialize.rs");
    t.pass("tests/unnamed_struct_deserialize.rs");
    t.pass("tests/skip_serializing_if_attribute.rs");
}
