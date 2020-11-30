use state_machine_procmacro::fsm;

fsm! {
    Simple

    One --(A{foo: String}, on_a)--> Two
}

pub struct One {}
pub struct Two {}

fn main() {}
