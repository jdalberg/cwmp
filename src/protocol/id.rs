use std::io::Write;

#[cfg(test)]
use quickcheck::{Arbitrary, Gen};
use xml::writer::XmlEvent;

use super::{bool2str, cwmp_prefix, GenerateError, XmlSafeString};

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct ID {
    pub must_understand: bool,
    pub id: XmlSafeString,
}

impl ID {
    #[must_use]
    pub fn new(must_understand: bool, id: &str) -> Self {
        ID {
            must_understand,
            id: id.into(),
        }
    }

    /// Generate XML for `ID`
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
            XmlEvent::start_element(&cwmp_prefix(has_cwmp, "ID")[..])
                .attr("mustUnderstand", bool2str(self.must_understand)),
        )?;
        writer.write(self.id.0.as_ref())?;
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
}

#[cfg(test)]
impl Arbitrary for ID {
    fn arbitrary(g: &mut Gen) -> Self {
        Self {
            must_understand: bool::arbitrary(g),
            id: XmlSafeString::arbitrary(g),
        }
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (self.must_understand, self.id.clone())
                .shrink()
                .map(|(m, i)| ID {
                    must_understand: m,
                    id: i,
                }),
        )
    }
}
