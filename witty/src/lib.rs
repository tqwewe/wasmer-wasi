use witgen::witgen;

#[witgen]
pub enum Error {
    Command(String),
    DeserializeState,
    DeserializeEvent,
    DeserializeCommand,
}

/// Takes state, event
/// Returns new state
#[witgen]
pub fn apply(state: Vec<u8>, event: Vec<u8>) -> Result<Vec<u8>, Error> {
    todo!()
}

/// Takes state, command
/// Returns list of events
#[witgen]
pub fn handle_command(state: Vec<u8>, command: Vec<u8>) -> Result<Vec<Vec<u8>>, Error> {
    todo!()
}
