//! Macro version of the card reader

use state_machine_procmacro::fsm;
use state_machine_trait::TransitionResult;
use std::convert::Infallible;

fsm! {
    CardReader, Commands, Infallible

    Locked --(CardReadable(CardData), on_card_readable) --> ReadingCard;
    ReadingCard --(CardAccepted, on_card_accepted) --> DoorOpen;
    ReadingCard --(CardRejected, on_card_rejected) --> Locked;
    DoorOpen --(DoorClosed, on_door_closed) --> Locked;
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Commands {
    StartBlinkingLight,
    StopBlinkingLight,
    ProcessData(CardData),
}

type CardData = String;

/// Door is locked / idle / we are ready to read
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Locked {}

/// Actively reading the card
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ReadingCard {
    card_data: CardData,
}

/// The door is open, we shouldn't be accepting cards and should be blinking the light
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct DoorOpen {}
impl DoorOpen {
    fn on_door_closed(&self) -> CardReaderTransition {
        TransitionResult::ok(vec![], Locked {})
    }
}

impl Locked {
    fn on_card_readable(&self, data: CardData) -> CardReaderTransition {
        TransitionResult::ok(
            vec![
                Commands::ProcessData(data.clone()),
                Commands::StartBlinkingLight,
            ],
            ReadingCard { card_data: data },
        )
    }
}

impl ReadingCard {
    fn on_card_accepted(&self) -> CardReaderTransition {
        TransitionResult::ok(vec![Commands::StopBlinkingLight], DoorOpen {})
    }
    fn on_card_rejected(&self) -> CardReaderTransition {
        TransitionResult::ok(vec![Commands::StopBlinkingLight], Locked {})
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use state_machine_trait::StateMachine;

    // Should be kept the same as by-hand test
    #[test]
    fn run_a_card_reader() {
        let cr = CardReader::Locked(Locked {});
        let (cr, cmds) = cr
            .on_event(CardReaderEvents::CardReadable("badguy".to_string()))
            .unwrap();
        assert!(matches!(cmds[0], Commands::ProcessData(_)));
        assert!(matches!(cmds[1], Commands::StartBlinkingLight));

        let (cr, cmds) = cr.on_event(CardReaderEvents::CardRejected).unwrap();
        assert!(matches!(cmds[0], Commands::StopBlinkingLight));

        let (cr, cmds) = cr
            .on_event(CardReaderEvents::CardReadable("goodguy".to_string()))
            .unwrap();
        assert!(matches!(cmds[0], Commands::ProcessData(_)));
        assert!(matches!(cmds[1], Commands::StartBlinkingLight));

        let (_, cmds) = cr.on_event(CardReaderEvents::CardAccepted).unwrap();
        assert!(matches!(cmds[0], Commands::StopBlinkingLight));
    }
}
