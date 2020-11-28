use std::{collections::HashMap, hash::Hash, marker::PhantomData, mem::Discriminant};

#[derive(Debug)]
pub struct StateMachine<State, Event, Command>
where
    Event: Eq + Hash,
{
    transition_handlers: HashMap<Event, Transition<State>>,
    _cmd_marker: PhantomData<Command>,
}

// TODO: Should maybe be trait?
impl<S, E, C> StateMachine<S, E, C>
where
    E: Eq + Hash,
{
    pub fn new() -> Self {
        Self {
            transition_handlers: Default::default(),
            _cmd_marker: PhantomData::default(),
        }
    }
    /// When `event` is seen, trigger a transition from `from` to `to`
    pub fn on_event(&mut self, event: E, from: S, to: S) -> &mut Self {
        // let transition = Transition::new(from, to);
        // self.transition_handlers.insert(event, transition);
        self
    }

    /// When `event` is seen, trigger a transition from `from` to `to`, also executing the provided
    /// function. TODO: That
    pub fn on_event_do(&mut self, event: E, from: S, to: S) {}
}

pub struct StateMachineDefinition<State, Event> {
    transitions: HashMap<Discriminant<Event>, Transition<State>>,
}
impl<S, E> StateMachineDefinition<S, E>
where
    E: Eq + Hash,
{
    pub fn new() -> Self {
        Self {
            transitions: Default::default(),
        }
    }

    pub fn transition(
        &mut self,
        on: Discriminant<E>,
        from: Discriminant<S>,
        to: Discriminant<S>,
    ) -> &mut Self {
        self.transitions.insert(on, Transition::new(from, to));
        self
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, derive_more::Constructor)]
pub struct Transition<State> {
    from: Discriminant<State>,
    to: Discriminant<State>,
}
