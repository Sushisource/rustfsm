use state_machine_procmacro::fsm;

fsm! {
    One --(A)--> Two
}

pub struct One {}
pub struct Two {}

fn main() {}
