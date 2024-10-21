#[cfg(test)]
use quickcheck::{Arbitrary, Gen};

use super::XmlSafeString;

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct EventStruct {
    pub event_code: XmlSafeString,
    pub command_key: XmlSafeString,
}

impl EventStruct {
    #[must_use]
    pub fn new(event_code: &str, command_key: &str) -> Self {
        EventStruct {
            event_code: event_code.into(),
            command_key: command_key.into(),
        }
    }
}

#[cfg(test)]
impl Arbitrary for EventStruct {
    fn arbitrary(g: &mut Gen) -> Self {
        Self {
            event_code: XmlSafeString::arbitrary(g),
            command_key: XmlSafeString::arbitrary(g),
        }
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (self.event_code.clone(), self.command_key.clone())
                .shrink()
                .map(|(e, c)| EventStruct {
                    event_code: e,
                    command_key: c,
                }),
        )
    }
}
