use std::io::Write;

#[cfg(test)]
use quickcheck::{Arbitrary, Gen};
use xml::writer::XmlEvent;

use super::{cwmp_prefix, write_simple, GenerateError, XmlSafeString};

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct AddObject {
    pub object_name: XmlSafeString,
    pub parameter_key: XmlSafeString,
}

impl AddObject {
    #[must_use]
    pub fn new(object_name: &str, parameter_key: &str) -> Self {
        AddObject {
            object_name: object_name.into(),
            parameter_key: parameter_key.into(),
        }
    }

    /// Generate XML for `AddObject`
    ///     
    /// # Errors
    ///     Any errors encountered while writing to `writer` will be returned.
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        writer.write(XmlEvent::start_element(
            &cwmp_prefix(has_cwmp, "AddObject")[..],
        ))?;
        write_simple(writer, "ObjectName", self.object_name.0.as_ref())?;
        write_simple(writer, "ParameterKey", self.parameter_key.0.as_ref())?;
        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
    pub fn characters(&mut self, path: &[&str], characters: &str) {
        match *path {
            ["AddObject", "ObjectName"] => {
                self.object_name = characters.into();
            }
            ["AddObject", "ParameterKey"] => {
                self.parameter_key = characters.into();
            }
            _ => {}
        }
    }
}

#[cfg(test)]
impl Arbitrary for AddObject {
    fn arbitrary(g: &mut Gen) -> Self {
        AddObject::new(
            XmlSafeString::arbitrary(g).0.as_ref(),
            XmlSafeString::arbitrary(g).0.as_ref(),
        )
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (self.object_name.clone(), self.parameter_key.clone())
                .shrink()
                .map(|(o, p)| AddObject {
                    object_name: o,
                    parameter_key: p,
                }),
        )
    }
}
