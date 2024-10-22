use std::io::Write;

#[cfg(test)]
use quickcheck::{Arbitrary, Gen};
use xml::writer::XmlEvent;

use super::{cwmp_prefix, write_simple, GenerateError, XmlSafeString};
#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct Kicked {
    pub command: XmlSafeString,
    pub referer: XmlSafeString,
    pub arg: XmlSafeString,
    pub next: XmlSafeString,
}

impl Kicked {
    #[must_use]
    pub fn new(command: &str, referer: &str, arg: &str, next: &str) -> Self {
        Kicked {
            command: command.into(),
            referer: referer.into(),
            arg: arg.into(),
            next: next.into(),
        }
    }

    /// Generate XML for `Kicked`
    ///     
    /// # Errors
    ///
    /// Any errors encountered while writing to "writer" will be returned.
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        writer.write(XmlEvent::start_element(
            &cwmp_prefix(has_cwmp, "Kicked")[..],
        ))?;
        write_simple(writer, "Command", self.command.0.as_ref())?;
        write_simple(writer, "Referer", self.referer.0.as_ref())?;
        write_simple(writer, "Arg", self.arg.0.as_ref())?;
        write_simple(writer, "Next", self.next.0.as_ref())?;
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
    pub fn characters(&mut self, path: &[&str], characters: &str) {
        match *path {
            ["Kicked", "Command"] => {
                self.command = characters.into();
            }
            ["Kicked", "Referer"] => {
                self.referer = characters.into();
            }
            ["Kicked", "Arg"] => {
                self.arg = characters.into();
            }
            ["Kicked", "Next"] => {
                self.next = characters.into();
            }
            _ => {}
        }
    }
}

#[cfg(test)]
impl Arbitrary for Kicked {
    fn arbitrary(g: &mut Gen) -> Self {
        Self {
            command: XmlSafeString::arbitrary(g),
            referer: XmlSafeString::arbitrary(g),
            arg: XmlSafeString::arbitrary(g),
            next: XmlSafeString::arbitrary(g),
        }
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
