use std::io::Write;

#[cfg(test)]
use quickcheck::{Arbitrary, Gen};
use xml::writer::XmlEvent;

use super::{bool2str, cwmp_prefix, GenerateError};

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct SessionTimeout {
    pub must_understand: bool,
    pub timeout: u32,
}

impl SessionTimeout {
    #[must_use]
    pub fn new(must_understand: bool, timeout: u32) -> Self {
        SessionTimeout {
            must_understand,
            timeout,
        }
    }

    /// Generate XML for `SessionTimeout`
    ///     
    /// # Errors
    ///
    /// Any errors encountered while writing to "writer" will be returned.
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        writer.write(
            XmlEvent::start_element(&cwmp_prefix(has_cwmp, "SessionTimeout")[..])
                .attr("mustUnderstand", bool2str(self.must_understand)),
        )?;
        writer.write(&self.timeout.to_string()[..])?;
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
}

#[cfg(test)]
impl Arbitrary for SessionTimeout {
    fn arbitrary(g: &mut Gen) -> Self {
        SessionTimeout::new(bool::arbitrary(g), u32::arbitrary(g))
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (self.must_understand, self.timeout)
                .shrink()
                .map(|(m, t)| SessionTimeout {
                    must_understand: m,
                    timeout: t,
                }),
        )
    }
}
