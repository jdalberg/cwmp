use std::io::Write;

#[cfg(test)]
use quickcheck::{Arbitrary, Gen};
use xml::writer::XmlEvent;

use super::{cwmp_prefix, write_simple, GenerateError};

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct Reboot {
    pub command_key: String,
}

impl Reboot {
    #[must_use]
    pub fn new(command_key: String) -> Self {
        Reboot { command_key }
    }
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        writer.write(XmlEvent::start_element(
            &cwmp_prefix(has_cwmp, "Reboot")[..],
        ))?;
        write_simple(writer, "CommandKey", &self.command_key)?;
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
    pub fn characters(&mut self, path: &[&str], characters: &String) {
        if let ["Reboot", "CommandKey"] = *path {
            self.command_key = characters.to_string();
        }
    }
}

#[cfg(test)]
impl Arbitrary for Reboot {
    fn arbitrary(g: &mut Gen) -> Self {
        Reboot::new(String::arbitrary(g))
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            self.command_key
                .clone()
                .shrink()
                .map(|c| Reboot { command_key: c }),
        )
    }
}
