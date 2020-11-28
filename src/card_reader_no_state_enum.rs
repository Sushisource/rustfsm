//! We'll imagine a (idealized) card reader which unlocks a door / blinks a light when it's open

use crate::card_reader_no_state_enum::CardReaderError::EventNotApplicable;

pub enum CardReader {
    Locked(Locked),
    ReadingCard(ReadingCard),
    Unlocked(DoorOpen),
}

#[derive(thiserror::Error, Debug)]
pub enum CardReaderError {
    #[error("Event is not applicable to current state")]
    EventNotApplicable,
}

impl CardReader {
    /// Reader starts locked
    pub fn new() -> Self {
        CardReader::Locked(Locked {})
    }

    // TODO: Non exhaustive event match?
    pub fn on_event(&mut self, event: Events) -> Result<Vec<Commands>, CardReaderError> {
        let mut commands = vec![];
        let new_state = match self {
            CardReader::Locked(ls) => match event {
                Events::CardReadable(data) => {
                    commands.push(Commands::StartBlinkingLight);
                    commands.push(Commands::ProcessData(data.clone()));
                    Self::ReadingCard(ls.on_card_readable(data))
                }
                _ => return Err(EventNotApplicable),
            },
            CardReader::ReadingCard(rc) => match event {
                Events::CardAccepted => {
                    commands.push(Commands::StopBlinkingLight);
                    Self::Unlocked(rc.on_card_accepted())
                }
                Events::CardRejected => {
                    commands.push(Commands::StopBlinkingLight);
                    Self::Locked(rc.on_card_rejected())
                }
                _ => return Err(EventNotApplicable),
            },
            CardReader::Unlocked(_) => match event {
                Events::DoorClosed => Self::Locked(Locked {}),
                _ => return Err(EventNotApplicable),
            },
        };
        *self = new_state;
        Ok(commands)
    }
}

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

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Events {
    /// Someone's presented a card for reading
    CardReadable(CardData),
    /// The door latch is disconnected
    DoorOpened,
    /// Door latch connected
    DoorClosed,
    CardAccepted,
    CardRejected,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Commands {
    StartBlinkingLight,
    StopBlinkingLight,
    ProcessData(CardData),
}

type CardData = String;

impl Locked {
    fn on_card_readable(&self, data: CardData) -> ReadingCard {
        ReadingCard { card_data: data }
    }
}

impl ReadingCard {
    fn on_card_accepted(&self) -> DoorOpen {
        DoorOpen {}
    }
    fn on_card_rejected(&self) -> Locked {
        Locked {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_a_card_reader() {
        let mut cr = CardReader::new();
        let mut cmds = cr
            .on_event(Events::CardReadable("badguy".to_string()))
            .unwrap();
        assert!(matches!(cmds.pop().unwrap(), Commands::ProcessData(_)));
        assert!(matches!(cmds.pop().unwrap(), Commands::StartBlinkingLight));

        let mut cmds = cr.on_event(Events::CardRejected).unwrap();
        assert!(matches!(cmds.pop().unwrap(), Commands::StopBlinkingLight));

        let mut cmds = cr
            .on_event(Events::CardReadable("goodguy".to_string()))
            .unwrap();
        assert!(matches!(cmds.pop().unwrap(), Commands::ProcessData(_)));
        assert!(matches!(cmds.pop().unwrap(), Commands::StartBlinkingLight));

        let mut cmds = cr.on_event(Events::CardAccepted).unwrap();
        assert!(matches!(cmds.pop().unwrap(), Commands::StopBlinkingLight));
    }
}
