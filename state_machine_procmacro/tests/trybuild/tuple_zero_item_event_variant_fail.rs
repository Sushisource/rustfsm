use state_machine_procmacro::fsm;

fsm! {
    Simple

    One --(A(), on_a)--> Two
}

fn main() {}
