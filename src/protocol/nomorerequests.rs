use std::io::Write;

#[cfg(test)]
use quickcheck::{Arbitrary, Gen};
use xml::writer::XmlEvent;

use super::{bool2str, cwmp_prefix, GenerateError};

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct NoMoreRequests {
    pub must_understand: bool,
    pub value: u8,
}

impl NoMoreRequests {
    pub fn new(must_understand: bool, value: u8) -> Self {
        NoMoreRequests {
            must_understand,
            value,
        }
    }
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        writer.write(
            XmlEvent::start_element(&cwmp_prefix(has_cwmp, "NoMoreRequests")[..])
                .attr("mustUnderstand", bool2str(self.must_understand)),
        )?;
        writer.write(&self.value.to_string()[..])?;
        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
}

#[cfg(test)]
impl Arbitrary for NoMoreRequests {
    fn arbitrary(g: &mut Gen) -> Self {
        NoMoreRequests::new(bool::arbitrary(g), u8::arbitrary(g))
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (self.must_understand.clone(), self.value.clone())
                .shrink()
                .map(|(m, v)| NoMoreRequests {
                    must_understand: m,
                    value: v,
                }),
        )
    }
}
