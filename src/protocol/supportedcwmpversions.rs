use std::io::Write;

#[cfg(test)]
use quickcheck::{Arbitrary, Gen};
use xml::writer::XmlEvent;

use super::{bool2str, cwmp_prefix, GenerateError};

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct SupportedCWMPVersions {
    pub must_understand: bool,
    pub value: String,
}

impl SupportedCWMPVersions {
    pub fn new(must_understand: bool, value: String) -> Self {
        SupportedCWMPVersions {
            must_understand: must_understand,
            value: value,
        }
    }
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        writer.write(
            XmlEvent::start_element(&cwmp_prefix(has_cwmp, "SupportedCWMPVersions")[..])
                .attr("mustUnderstand", bool2str(self.must_understand)),
        )?;
        writer.write(&self.value.to_string()[..])?;
        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
}

#[cfg(test)]
impl Arbitrary for SupportedCWMPVersions {
    fn arbitrary(g: &mut Gen) -> Self {
        SupportedCWMPVersions::new(bool::arbitrary(g), String::arbitrary(g))
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (self.must_understand.clone(), self.value.clone())
                .shrink()
                .map(|(m, v)| SupportedCWMPVersions {
                    must_understand: m,
                    value: v,
                }),
        )
    }
}
