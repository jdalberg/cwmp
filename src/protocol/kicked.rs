use std::io::Write;

#[cfg(test)]
use quickcheck::{Arbitrary, Gen};
use xml::writer::XmlEvent;

use super::{cwmp_prefix, write_simple, GenerateError};
#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct Kicked {
    pub command: String,
    pub referer: String,
    pub arg: String,
    pub next: String,
}

impl Kicked {
    #[must_use]
    pub fn new(command: String, referer: String, arg: String, next: String) -> Self {
        Kicked {
            command,
            referer,
            arg,
            next,
        }
    }

    /// Generate XML for `Kicked`
    ///     
    /// # Errors
    ///     Any errors encountered while writing to `writer` will be returned.
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        writer.write(XmlEvent::start_element(
            &cwmp_prefix(has_cwmp, "Kicked")[..],
        ))?;
        write_simple(writer, "Command", &self.command)?;
        write_simple(writer, "Referer", &self.referer)?;
        write_simple(writer, "Arg", &self.arg)?;
        write_simple(writer, "Next", &self.next)?;
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
    pub fn characters(&mut self, path: &[&str], characters: &String) {
        match *path {
            ["Kicked", "Command"] => {
                self.command = characters.to_string();
            }
            ["Kicked", "Referer"] => {
                self.referer = characters.to_string();
            }
            ["Kicked", "Arg"] => {
                self.arg = characters.to_string();
            }
            ["Kicked", "Next"] => {
                self.next = characters.to_string();
            }
            _ => {}
        }
    }
}

#[cfg(test)]
impl Arbitrary for Kicked {
    fn arbitrary(g: &mut Gen) -> Self {
        Kicked::new(
            String::arbitrary(g),
            String::arbitrary(g),
            String::arbitrary(g),
            String::arbitrary(g),
        )
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (
                self.command.clone(),
                self.referer.clone(),
                self.arg.clone(),
                self.next.clone(),
            )
                .shrink()
                .map(|(c, r, a, n)| Kicked {
                    command: c,
                    referer: r,
                    arg: a,
                    next: n,
                }),
        )
    }
}
