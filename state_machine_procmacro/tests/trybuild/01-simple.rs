use state_machine_procmacro::fsm;

fsm! {
    Simple

    One --(A)--> Two
}

pub struct One {}
pub struct Two {}

fn main() {
    // main enum exists with both states
    let _ = Simple::One(One {});
    let _ = Simple::Two(Two {});
}
