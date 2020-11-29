use std::error::Error;

/// This trait defines a state machine (more formally, a [finite state
/// transducer](https://en.wikipedia.org/wiki/Finite-state_transducer)) which accepts events (the
/// input alphabet), uses them to mutate itself, and (may) output some commands (the output
/// alphabet) as a result.
pub trait StateMachine<State, Event, Command> {
    type Error: Error;

    /// Handle an incoming event
    fn on_event(&mut self, event: Event) -> Result<Vec<Command>, Self::Error>;

    /// Returns the current state of the machine
    fn state(&self) -> &State;
}
