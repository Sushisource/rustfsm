use state_machine_procmacro::fsm;

fsm! {
    Simple, SimpleCmd, Infallible

    One --(A(Foo, Bar), on_a)--> Two
}

fn main() {}
