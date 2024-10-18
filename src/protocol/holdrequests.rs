use std::io::Write;

#[cfg(test)]
use quickcheck::{Arbitrary, Gen};
use xml::writer::XmlEvent;

use super::{bool2str, cwmp_prefix, GenerateError};

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct HoldRequests {
    pub must_understand: bool,
    pub hold: bool,
}

impl HoldRequests {
    #[must_use]
    pub fn new(must_understand: bool, hold: bool) -> Self {
        HoldRequests {
            must_understand,
            hold,
        }
    }

    /// Generate XML for `HoldRequests`
    ///     
    /// # Errors
    ///     Any errors encountered while writing to `writer` will be returned.
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        writer.write(
            XmlEvent::start_element(&cwmp_prefix(has_cwmp, "HoldRequests")[..])
                .attr("mustUnderstand", bool2str(self.must_understand)),
        )?;

        writer.write(bool2str(self.hold))?;
        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
}

#[cfg(test)]
impl Arbitrary for HoldRequests {
    fn arbitrary(g: &mut Gen) -> Self {
        HoldRequests::new(bool::arbitrary(g), bool::arbitrary(g))
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (self.must_understand.clone(), self.hold.clone())
                .shrink()
                .map(|(m, h)| HoldRequests {
                    must_understand: m,
                    hold: h,
                }),
        )
    }
}
