use state_machine_procmacro::fsm;

fsm! {
    SimpleMachine

    One --(A)--> Two
}

pub struct One {}
pub struct Two {}

fn main() {
    // main enum exists with both states
    let _ = SimpleMachine::One(One {});
    let _ = SimpleMachine::Two(Two {});
    // Event enum exists
    let _ = SimpleMachineEvents::A;
}
