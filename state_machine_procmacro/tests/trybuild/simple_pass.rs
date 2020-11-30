use state_machine_procmacro::fsm;
use std::convert::Infallible;

fsm! {
    SimpleMachine, SimpleMachineCommand, Infallible

    One --(A)--> Two
}

pub struct One {}
pub struct Two {}

pub enum SimpleMachineCommand {}

fn main() {
    // main enum exists with both states
    let _ = SimpleMachine::One(One {});
    let _ = SimpleMachine::Two(Two {});
    // Event enum exists
    let _ = SimpleMachineEvents::A;
}
