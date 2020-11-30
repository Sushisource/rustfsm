#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/trybuild/01-simple.rs");
    t.compile_fail("tests/trybuild/02-forgot-to-name.rs");
    t.pass("tests/trybuild/03-handler.rs");
}
