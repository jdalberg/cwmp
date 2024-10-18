use std::io::Write;

#[cfg(test)]
use quickcheck::{Arbitrary, Gen};
use xml::writer::XmlEvent;

use super::{bool2str, cwmp_prefix, GenerateError};

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct ID {
    pub must_understand: bool,
    pub id: String,
}

impl ID {
    pub fn new(must_understand: bool, id: String) -> Self {
        ID {
            must_understand: must_understand,
            id: id,
        }
    }
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        writer.write(
            XmlEvent::start_element(&cwmp_prefix(has_cwmp, "ID")[..])
                .attr("mustUnderstand", bool2str(self.must_understand)),
        )?;
        writer.write(&self.id[..])?;
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
}

#[cfg(test)]
impl Arbitrary for ID {
    fn arbitrary(g: &mut Gen) -> Self {
        ID::new(bool::arbitrary(g), String::arbitrary(g))
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (self.must_understand.clone(), self.id.clone())
                .shrink()
                .map(|(m, i)| ID {
                    must_understand: m,
                    id: i,
                }),
        )
    }
}
