extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use std::collections::{HashMap, HashSet};
use syn::{
    parenthesized,
    parse::{Parse, ParseStream, Result},
    parse_macro_input,
    punctuated::Punctuated,
    spanned::Spanned,
    Error, Fields, Ident, Token, Variant,
};

/// Parses a DSL for defining finite state machines, and produces code implementing the
/// [StateMachine](trait.StateMachine.html) trait.
///
/// An example state machine definition of a card reader for unlocking a door:
/// ```
/// use state_machine_procmacro::fsm;
/// use std::convert::Infallible;
/// use state_machine_trait::TransitionResult;
///
/// fsm! {
///     CardMachine, CardMachineCommands, Infallible
///
///     Locked      --(CardReadable(CardData), on_card_readable)--> ReadingCard;
///     ReadingCard --(CardAccepted, on_card_accepted)--> Unlocked;
///     ReadingCard --(CardRejected, on_card_rejected)--> Locked;
///     Unlocked    --(DoorClosed)-->                     Locked;
/// }
///
/// #[derive(Default)]
/// struct Locked {}
/// impl Locked {
///     fn on_card_readable(self, data: CardData)
///         -> TransitionResult<CardMachine, Infallible, CardMachineCommands> {
///         TransitionResult::ok(vec![], ReadingCard {})
///     }
/// }
///
/// struct ReadingCard {}
/// impl ReadingCard {
///     fn on_card_accepted(self)
///         -> TransitionResult<CardMachine, Infallible, CardMachineCommands> {
///         TransitionResult::ok(vec![], Unlocked {})
///     }
///     fn on_card_rejected(self)
///         -> TransitionResult<CardMachine, Infallible, CardMachineCommands> {
///         TransitionResult::ok(vec![], Locked {})
///     }
/// }
/// struct Unlocked {}
///
/// enum CardMachineCommands {}
///
/// type CardData = &'static str;
///
/// ```
///
/// In the above example the first word is the name of the state machine, then after the comma the
/// type (which you must define separately) of commands produced by the machine.
///
/// then each line represents a transition, where the first word is the initial state, the tuple
/// inside the arrow is `(eventtype[, event handler])`, and the word after the arrow is the
/// destination state. here `eventtype` is an enum variant , and `event_handler` is a function you
/// must define outside the enum whose form depends on the event variant. the only variant types
/// allowed are unit and one-item tuple variants. For unit variants, the function takes no
/// parameters. For the tuple variants, the function takes the variant data as its parameter. In
/// either case the function is expected to return a `TransitionResult` to the appropriate state.
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
    command_type: Ident,
    error_type: Ident,
    transitions: HashSet<Transition>,
}

impl Parse for StateMachineDefinition {
    // TODO: Pub keyword
    fn parse(input: ParseStream) -> Result<Self> {
        // First parse the state machine name, command type, and error type
        let (name, command_type, error_type) = parse_first_line(&input).map_err(|mut e| {
            e.combine(Error::new(
                e.span(),
                "The first line of the fsm definition should be `MachineName, CommandType, ErrorType`",
            ));
            e
        })?;
        // Then the state machine definition is simply a sequence of transitions separated by
        // semicolons
        let transitions: Punctuated<Transition, Token![;]> =
            input.parse_terminated(Transition::parse)?;
        let transitions = transitions.into_iter().collect();
        Ok(Self {
            name,
            transitions,
            command_type,
            error_type,
        })
    }
}

fn parse_first_line(input: &ParseStream) -> Result<(Ident, Ident, Ident)> {
    let name: Ident = input.parse()?;
    input.parse::<Token![,]>()?;
    let command_type: Ident = input.parse()?;
    input.parse::<Token![,]>()?;
    let error_type: Ident = input.parse()?;
    Ok((name, command_type, error_type))
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
        let from: Ident = input.parse()?;
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
            #[derive(::derive_more::From)]
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

        // Construct the trait implementation
        let cmd_type = &self.command_type;
        let err_type = &self.error_type;
        let mut statemap: HashMap<Ident, Vec<Transition>> = HashMap::new();
        for t in &self.transitions {
            statemap
                .entry(t.from.clone())
                .and_modify(|v| v.push(t.clone()))
                .or_insert(vec![t.clone()]);
        }
        let state_branches = statemap.iter().map(|(from, transitions)| {
            let event_branches = transitions
                .iter()
                .map(|ts| {
                    let ev_variant = &ts.event.ident;
                    if let Some(ts_fn) = ts.handler.clone() {
                        let span = ts_fn.span();
                        match ts.event.fields {
                            Fields::Unnamed(_) => quote_spanned! {span=>
                                #events_enum_name::#ev_variant(val) => {
                                    state_data.#ts_fn(val)
                                }
                            },
                            Fields::Unit => quote_spanned! {span=>
                                #events_enum_name::#ev_variant => {
                                    state_data.#ts_fn()
                                }
                            },
                            Fields::Named(_) => unreachable!(),
                        }
                    } else {
                        // TODO: What should events with no handler do? How do we construct the next
                        //    state?
                        let new_state = ts.to.clone();
                        let span = new_state.span();
                        let default_trans = quote_spanned! {span=>
                            TransitionResult::ok(vec![], #new_state::default())
                        };
                        let span = ts.event.span();
                        match ts.event.fields {
                            Fields::Unnamed(_) => quote_spanned! {span=>
                                #events_enum_name::#ev_variant(_val) => {
                                    #default_trans
                                }
                            },
                            Fields::Unit => quote_spanned! {span=>
                                #events_enum_name::#ev_variant => {
                                    #default_trans
                                }
                            },
                            Fields::Named(_) => unreachable!(),
                        }
                    }
                })
                // Since most states won't handle every possible event, return an error to that effect
                .chain(std::iter::once(
                    quote! { _ => { return TransitionResult::InvalidTransition } },
                ));
            quote! {
                #name::#from(state_data) => match event {
                    #(#event_branches),*
                }
            }
        });

        // TODO: Make a transition result type alias so user doesn't have to type generics
        let trait_impl = quote! {
            impl ::state_machine_trait::StateMachine<#name, #events_enum_name, #cmd_type> for #name {
                type Error = #err_type;

                fn on_event(self, event: #events_enum_name)
                  -> TransitionResult<#name, Self::Error, #cmd_type> {
                    match self {
                        #(#state_branches),*
                    }
                }

                fn state(&self) -> &Self {
                    &self
                }
            }
        };

        let output = quote! {
            #main_enum

            #events_enum

            #trait_impl
        };

        output.into()
    }
}
