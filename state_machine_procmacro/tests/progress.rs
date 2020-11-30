#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/trybuild/*_pass.rs");
    t.compile_fail("tests/trybuild/*_fail.rs");
}

// Kept here to inspect manual expansion
state_machine_procmacro::fsm! {
    SimpleMachine, SimpleMachineCommand, Infallible

    One --(A(String), foo)--> Two;
    Two --(B, bar)--> One
}
