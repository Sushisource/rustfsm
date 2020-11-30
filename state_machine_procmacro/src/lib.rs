extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use std::collections::HashSet;
use syn::{
    parenthesized,
    parse::{Parse, ParseStream, Result},
    parse_macro_input,
    punctuated::Punctuated,
    Error, Ident, Token,
};

/// Parses a DSL for defining finite state machines, and produces code implementing the
/// [StateMachine](trait.StateMachine.html) trait.
///
/// An example state machine definition of a card reader for unlocking a door:
/// ```
/// use state_machine_procmacro::fsm;
///
/// fsm! {
///     CardMachine
///
///     Locked      --(CardReadable, on_card_readable)--> ReadingCard;
///     ReadingCard --(CardAccepted, on_card_accepted)--> Unlocked;
///     ReadingCard --(CardRejected, on_card_rejected)--> Locked;
///     Unlocked    --(DoorClosed)-->                     Locked;
/// }
/// struct Locked {}
/// struct ReadingCard {}
/// struct Unlocked {}
/// ```
///
/// In the above example each line represents a transition, where the first word is the initial
/// state, the tuple inside the arrow is (EventType\[, event handler\]), and the word after the
/// arrow is the destination state.
///
/// The first line can be interpreted as "If the machine is in the locked state, when a
/// `CardReadable` event is seen, call `on_card_readable` and transition to the `ReadingCard` state.
#[proc_macro]
pub fn fsm(input: TokenStream) -> TokenStream {
    let def: StateMachineDefinition = parse_macro_input!(input as StateMachineDefinition);
    def.codegen()
}

struct StateMachineDefinition {
    name: Ident,
    transitions: HashSet<Transition>,
}

impl Parse for StateMachineDefinition {
    // TODO: Pub keyword
    fn parse(input: ParseStream) -> Result<Self> {
        // First parse the state machine name
        let name: Ident = input.parse()?;
        // Then the state machine definition is simply a sequence of transitions separated by
        // semicolons
        let transitions: Punctuated<Transition, Token![;]> =
            input.parse_terminated(Transition::parse)?;
        let transitions = transitions.into_iter().collect();
        Ok(Self { name, transitions })
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Transition {
    from: Ident,
    to: Ident,
    event: Ident,
    handler: Option<Ident>,
}

impl Parse for Transition {
    fn parse(input: ParseStream) -> Result<Self> {
        // Parse the initial state name
        let from: Ident = input.parse().map_err(|mut e| {
            e.combine(Error::new(e.span(),
                "I should have seen two identifiers at this point, the state machine name, and the \
                name of the initial state for the first transition. Did you forget the state \
                machine name?"
            ));
            e
        })?;
        // Parse at least one dash
        input.parse::<Token![-]>()?;
        while input.peek(Token![-]) {
            input.parse::<Token![-]>()?;
        }
        // Parse transition information inside parens
        let transition_info;
        parenthesized!(transition_info in input);
        // Get the event name
        let event: Ident = transition_info.parse()?;
        // Check if there is an event handler
        let handler = if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
            Some(input.parse()?)
        } else {
            None
        };
        // Parse at least one dash followed by the "arrow"
        input.parse::<Token![-]>()?;
        while input.peek(Token![-]) {
            input.parse::<Token![-]>()?;
        }
        input.parse::<Token![>]>()?;
        // Parse the destination state
        let to: Ident = input.parse()?;

        Ok(Self {
            from,
            event,
            handler,
            to,
        })
    }
}

impl StateMachineDefinition {
    fn codegen(&self) -> TokenStream {
        // First extract all of the states into a set, and build the enum's insides
        let states: HashSet<_> = self
            .transitions
            .iter()
            .flat_map(|t| vec![t.from.clone(), t.to.clone()])
            .collect();
        let states = states.into_iter().map(|s| {
            quote! {
                #s(#s)
            }
        });
        let name = &self.name;
        let main_enum = quote! {
            pub enum #name {
                #(#states),*
            }
        };

        for _transition in &self.transitions {}
        let output = quote! {
            #main_enum
        };

        output.into()
    }
}
