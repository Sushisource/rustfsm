use state_machine_procmacro::fsm;
use std::convert::Infallible;

fsm! {
    Simple, SimpleCommand, Infallible

    One --(A(String), on_a)--> Two
}

pub struct One {}
pub struct Two {}

pub enum SimpleCommand {}

fn main() {
    // main enum exists with both states
    let _ = Simple::One(One {});
    let _ = Simple::Two(Two {});
}
