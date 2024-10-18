use std::io::Write;

use xml::writer::XmlEvent;

use super::{cwmp_prefix, parse_to_int, write_simple, GenerateError};
#[cfg(test)]
use quickcheck::{Arbitrary, Gen};

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct ScheduleInform {
    pub delay_seconds: u32,
    pub command_key: String,
}

impl ScheduleInform {
    #[must_use]
    pub fn new(delay_seconds: u32, command_key: String) -> Self {
        ScheduleInform {
            delay_seconds,
            command_key,
        }
    }

    /// Generate XML for `ScheduleInform`
    ///     
    /// # Errors
    ///     Any errors encountered while writing to `writer` will be returned.
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        writer.write(XmlEvent::start_element(
            &cwmp_prefix(has_cwmp, "ScheduleInform")[..],
        ))?;
        write_simple(writer, "DelaySeconds", &self.delay_seconds.to_string())?;
        write_simple(writer, "CommandKey", &self.command_key)?;
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
    pub fn characters(&mut self, path: &[&str], characters: &String) {
        match *path {
            ["ScheduleInform", "DelaySeconds"] => {
                self.delay_seconds = parse_to_int(characters, 0);
            }
            ["ScheduleInform", "CommandKey"] => {
                self.command_key = characters.to_string();
            }
            _ => {}
        }
    }
}

#[cfg(test)]
impl Arbitrary for ScheduleInform {
    fn arbitrary(g: &mut Gen) -> Self {
        ScheduleInform::new(u32::arbitrary(g), String::arbitrary(g))
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (self.delay_seconds.clone(), self.command_key.clone())
                .shrink()
                .map(|(d, c)| ScheduleInform {
                    delay_seconds: d,
                    command_key: c,
                }),
        )
    }
}
