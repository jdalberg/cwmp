use std::io::Write;

#[cfg(test)]
use quickcheck::{Arbitrary, Gen};
use xml::writer::XmlEvent;

use super::{bool2str, cwmp_prefix, GenerateError};

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct UseCWMPVersion {
    pub must_understand: bool,
    pub value: String,
}

impl UseCWMPVersion {
    #[must_use]
    pub fn new(must_understand: bool, value: String) -> Self {
        UseCWMPVersion {
            must_understand,
            value,
        }
    }

    /// Generate XML for the `UseCWMPVersion` message type
    ///     
    /// # Arguments
    ///
    /// # Example
    ///
    /// # Errors
    ///
    /// Any problem encountered while writing to writer will be returned in a `GenerateError`.
    ///
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        writer.write(
            XmlEvent::start_element(&cwmp_prefix(has_cwmp, "UseCWMPVersion")[..])
                .attr("mustUnderstand", bool2str(self.must_understand)),
        )?;
        writer.write(&self.value.to_string()[..])?;
        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
}

#[cfg(test)]
impl Arbitrary for UseCWMPVersion {
    fn arbitrary(g: &mut Gen) -> Self {
        UseCWMPVersion::new(bool::arbitrary(g), String::arbitrary(g))
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (self.must_understand, self.value.clone())
                .shrink()
                .map(|(m, v)| UseCWMPVersion {
                    must_understand: m,
                    value: v,
                }),
        )
    }
}
