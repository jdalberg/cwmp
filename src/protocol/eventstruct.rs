#[cfg(test)]
use quickcheck::{Arbitrary, Gen};

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct EventStruct {
    pub event_code: String,
    pub command_key: String,
}

impl EventStruct {
    #[must_use]
    pub fn new(event_code: String, command_key: String) -> Self {
        EventStruct {
            event_code,
            command_key,
        }
    }
}

#[cfg(test)]
impl Arbitrary for EventStruct {
    fn arbitrary(g: &mut Gen) -> Self {
        EventStruct::new(String::arbitrary(g), String::arbitrary(g))
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
