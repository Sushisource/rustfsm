extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use std::collections::HashSet;
use syn::spanned::Spanned;
use syn::{
    parenthesized,
    parse::{Parse, ParseStream, Result},
    parse_macro_input,
    punctuated::Punctuated,
    Error, Fields, Ident, Token, Variant,
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
///     Locked      --(CardReadable(CardData), on_card_readable)--> ReadingCard;
///     ReadingCard --(CardAccepted, on_card_accepted)--> Unlocked;
///     ReadingCard --(CardRejected, on_card_rejected)--> Locked;
///     Unlocked    --(DoorClosed)-->                     Locked;
/// }
///
/// struct Locked {}
/// struct ReadingCard {}
/// struct Unlocked {}
///
/// type CardData = &'static str;
///
/// fn on_card_readable(data: CardData) {}
/// fn on_card_accepted() {}
/// fn on_card_rejected() {}
/// ```
///
/// In the above example each line represents a transition, where the first word is the initial
/// state, the tuple inside the arrow is `(EventType[, event handler])`, and the word after the
/// arrow is the destination state. Here `EventType` is an enum variant , and `event_handler` is
/// a function you must define outside the enum whose form depends on the event variant. The only
/// variant types allowed are unit and one-item tuple variants. For unit variants, the function
/// takes no parameters and returnsa list of commands. For the tuple variants, the function takes
/// the variant data as its parameter.
///
/// The first transition can be interpreted as "If the machine is in the locked state, when a
/// `CardReadable` event is seen, call `on_card_readable` (pasing in `CardData`) and transition to
/// the `ReadingCard` state.
///
/// The macro will generate a few things:
/// * An enum with a variant for each state, named with the provided name. In this case:
///   ```ignore
///   enum CardMachine {
///       Locked(Locked),
///       ReadingCard(ReadingCard),
///       Unlocked(Unlocked),
///   }
///   ```
///
///   You are expected to define a type for each state, to contain that state's data. If there is
///   no data, you can simply: `type StateName = ()`
/// * An enum with a variant for each event. You are expected to define the type (if any) contained
///   in the event variant. In this case:
///   ```ignore
///   enum CardMachineEvents {
///     CardReadable(CardData)
///   }
///   ```
/// * An implementation of the [StateMachine](trait.StateMachine.html) trait for the generated state
///   machine enum (in this case, `CardMachine`)
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
    event: Variant,
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
        // Get the event variant definition
        let event: Variant = transition_info.parse()?;
        // Reject non-unit or single-item-tuple variants
        match &event.fields {
            Fields::Named(_) => {
                return Err(Error::new(
                    event.span(),
                    "Struct variants are not supported for events",
                ))
            }
            Fields::Unnamed(uf) => {
                if uf.unnamed.len() != 1 {
                    return Err(Error::new(
                        event.span(),
                        "Only tuple variants with exactly one item are supported for events",
                    ));
                }
            }
            Fields::Unit => {}
        }
        // Check if there is an event handler, and parse it
        let handler = if transition_info.peek(Token![,]) {
            transition_info.parse::<Token![,]>()?;
            Some(transition_info.parse()?)
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

        // Build the events enum
        let events: Vec<Variant> = self.transitions.iter().map(|t| t.event.clone()).collect();
        let events_enum_name = Ident::new(&format!("{}Events", name), name.span());
        let events_enum = quote! {
            pub enum #events_enum_name {
                #(#events),*
            }
        };

        let output = quote! {
            #main_enum

            #events_enum
        };

        output.into()
    }
}
