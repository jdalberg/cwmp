#[cfg(test)]
use quickcheck::{Arbitrary, Gen};

use super::XmlSafeString;

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct QueuedTransferStruct {
    pub command_key: Option<XmlSafeString>,
    pub state: Option<XmlSafeString>,
}

impl QueuedTransferStruct {
    #[must_use]
    pub fn new(command_key: Option<&str>, state: Option<&str>) -> Self {
        QueuedTransferStruct {
            command_key: command_key.map(Into::into),
            state: state.map(Into::into),
        }
    }
}

#[cfg(test)]
impl Arbitrary for QueuedTransferStruct {
    fn arbitrary(g: &mut Gen) -> Self {
        Self {
            command_key: Option::<XmlSafeString>::arbitrary(g),
            state: Option::<XmlSafeString>::arbitrary(g),
        }
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
