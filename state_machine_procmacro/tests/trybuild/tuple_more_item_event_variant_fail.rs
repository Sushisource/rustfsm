use state_machine_procmacro::fsm;

fsm! {
    Simple

    One --(A(Foo, Bar), on_a)--> Two
}

fn main() {}
