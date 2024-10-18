#[cfg(test)]
use quickcheck::{Arbitrary, Gen};

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct QueuedTransferStruct {
    pub command_key: Option<String>,
    pub state: Option<String>,
}

impl QueuedTransferStruct {
    #[must_use]
    pub fn new(command_key: Option<String>, state: Option<String>) -> Self {
        QueuedTransferStruct { command_key, state }
    }
}

#[cfg(test)]
impl Arbitrary for QueuedTransferStruct {
    fn arbitrary(g: &mut Gen) -> Self {
        QueuedTransferStruct::new(
            Option::<String>::arbitrary(g),
            Option::<String>::arbitrary(g),
        )
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (self.command_key.clone(), self.state.clone())
                .shrink()
                .map(|(c, s)| QueuedTransferStruct {
                    command_key: c,
                    state: s,
                }),
        )
    }
}
