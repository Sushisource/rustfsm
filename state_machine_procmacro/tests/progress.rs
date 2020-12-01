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
    // TODO: B for both should work here, doesn't.
    Two --(C, baz)--> One
}

pub struct One {}
impl One {
    fn foo(
        &mut self,
        _: String,
    ) -> TransitionResult<Infallible, SimpleMachine, SimpleMachineCommand> {
        TransitionResult::Ok {
            commands: vec![],
            new_state: Two {}.into(),
        }
    }
}

#[derive(Default)]
pub struct Two {}
impl Two {
    fn baz(&mut self) -> TransitionResult<Infallible, SimpleMachine, SimpleMachineCommand> {
        TransitionResult::Ok {
            commands: vec![],
            new_state: One {}.into(),
        }
    }
}
enum SimpleMachineCommand {}
