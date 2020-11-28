/// We'll imagine a (idealized) card reader which unlocks a door / blinks a light when it's open
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum States {
    /// Door is locked / idle / we are ready to read
    Locked,
    /// Actively reading the card
    ReadingCard,
    /// The door is open, we shouldn't be accepting cards and should be blinking the light
    DoorOpen,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Events {
    /// Someone's presented a card for reading
    CardReadable(CardData),
    /// The door latch is disconnected
    DoorOpened,
    /// Door latch connected
    DoorClosed,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Commands {
    StartBlinkingLight,
    StopBlinkingLight,
    SaveData(CardData),
    UnlockDoor,
    LockDoor,
}

type CardData = String;

struct CardReader<State> {
    state: State,
}

#[cfg(test)]
mod tests {}
