use state_machine_procmacro::fsm;

fsm! {
    Simple, SimpleCmd, Infallible

    One --(A(), on_a)--> Two
}

fn main() {}
