use std::io::Write;

#[cfg(test)]
use quickcheck::{Arbitrary, Gen};
use xml::writer::XmlEvent;

use super::{bool2str, cwmp_prefix, GenerateError, XmlSafeString};

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct SupportedCWMPVersions {
    pub must_understand: bool,
    pub value: XmlSafeString,
}

impl SupportedCWMPVersions {
    #[must_use]
    pub fn new(must_understand: bool, value: &str) -> Self {
        SupportedCWMPVersions {
            must_understand,
            value: value.into(),
        }
    }

    /// Generate XML for `TransferComplete`
    ///     
    /// # Errors
    ///     Any errors encountered while writing to `writer` will be returned.
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        writer.write(
            XmlEvent::start_element(&cwmp_prefix(has_cwmp, "SupportedCWMPVersions")[..])
                .attr("mustUnderstand", bool2str(self.must_understand)),
        )?;
        writer.write(self.value.0.as_ref())?;
        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
}

#[cfg(test)]
impl Arbitrary for SupportedCWMPVersions {
    fn arbitrary(g: &mut Gen) -> Self {
        Self {
            must_understand: bool::arbitrary(g),
            value: XmlSafeString::arbitrary(g),
        }
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (self.must_understand, self.value.clone())
                .shrink()
                .map(|(m, v)| SupportedCWMPVersions {
                    must_understand: m,
                    value: v,
                }),
        )
    }
}
