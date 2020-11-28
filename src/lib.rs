mod state_machine;

/// We'll imagine a (idealized) card reader which unlocks a door / blinks a light when it's open
enum States {
    /// Door is locked / idle / we are ready to read
    Locked,
    /// Actively reading the card
    ReadingCard,
    /// The door is open, we shouldn't be accepting cards and should be blinking the light
    DoorOpen
}

enum Events {
    /// Someone's presented a card for reading
    CardReadable(CardData)
}

enum Commands {
    StartBlinkingLight,
    StopBlinkingLight
}

type CardData = String;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
