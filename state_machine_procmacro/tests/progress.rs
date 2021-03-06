use state_machine_trait::TransitionResult;
use std::convert::Infallible;

#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/trybuild/*_pass.rs");
    t.compile_fail("tests/trybuild/*_fail.rs");
}

//Kept here to inspect manual expansion
state_machine_procmacro::fsm! {
    SimpleMachine, SimpleMachineCommand, Infallible

    One --(A(String), foo)--> Two;
    One --(B)--> Two;
    Two --(B)--> One;
    Two --(C, baz)--> One
}

#[derive(Default)]
pub struct One {}
impl One {
    fn foo(self, _: String) -> SimpleMachineTransition {
        TransitionResult::default::<Two>()
    }
}

#[derive(Default)]
pub struct Two {}
impl Two {
    fn baz(self) -> SimpleMachineTransition {
        TransitionResult::default::<One>()
    }
}
enum SimpleMachineCommand {}
