use std::io::Write;

#[cfg(test)]
use quickcheck::{Arbitrary, Gen};
use xml::writer::XmlEvent;

use super::{cwmp_prefix, write_simple, GenerateError, XmlSafeString};

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct CancelTransfer {
    pub command_key: XmlSafeString,
}

impl CancelTransfer {
    #[must_use]
    pub fn new(command_key: &str) -> Self {
        CancelTransfer {
            command_key: command_key.into(),
        }
    }

    /// Generate XML for `CancelTransfer`
    ///     
    /// # Errors
    ///     Any errors encountered while writing to `writer` will be returned.
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        writer.write(XmlEvent::start_element(
            &cwmp_prefix(has_cwmp, "CancelTransfer")[..],
        ))?;
        write_simple(writer, "CommandKey", self.command_key.0.as_ref())?;
        writer.write(XmlEvent::end_element())?;

        Ok(())
    }

    pub fn characters(&mut self, path: &[&str], characters: &str) {
        if let ["CancelTransfer", "CommandKey"] = *path {
            self.command_key = characters.into();
        }
    }
}

#[cfg(test)]
impl Arbitrary for CancelTransfer {
    fn arbitrary(g: &mut Gen) -> Self {
        Self {
            command_key: XmlSafeString::arbitrary(g),
        }
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            self.command_key
                .clone()
                .shrink()
                .map(|c| CancelTransfer { command_key: c }),
        )
    }
}
