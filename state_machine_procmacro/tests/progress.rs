#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/01-simple.rs");
    t.compile_fail("tests/02-forgot-to-name.rs");
}
