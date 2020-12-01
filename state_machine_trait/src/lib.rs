use std::error::Error;

/// This trait defines a state machine (more formally, a [finite state
/// transducer](https://en.wikipedia.org/wiki/Finite-state_transducer)) which accepts events (the
/// input alphabet), uses them to mutate itself, and (may) output some commands (the output
/// alphabet) as a result.
pub trait StateMachine<State, Event, Command> {
    /// The error type produced by this state machine when handling events
    type Error: Error;

    /// Handle an incoming event
    fn on_event(&mut self, event: Event) -> TransitionResult<State, Self::Error, Command>;

    /// Returns the current state of the machine
    fn state(&self) -> &State;
}

pub enum TransitionResult<StateMachine, StateMachineError, StateMachineCommand> {
    /// This state does not define a transition for this event
    InvalidTransition,
    /// The transition was successful
    Ok {
        commands: Vec<StateMachineCommand>,
        new_state: StateMachine,
    },
    /// There an error performing the transition
    Err(StateMachineError),
}

impl<S, E, C> TransitionResult<S, E, C> {
    pub fn ok<CI, IS>(commands: CI, new_state: IS) -> Self
    where
        CI: IntoIterator<Item = C>,
        IS: Into<S>,
    {
        Self::Ok {
            commands: commands.into_iter().collect(),
            new_state: new_state.into(),
        }
    }
    pub fn unwrap_commands(self) -> Vec<C> {
        match self {
            Self::Ok { commands, .. } => commands,
            _ => panic!("Transition was not successful!"),
        }
    }
}
