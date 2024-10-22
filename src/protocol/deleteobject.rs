use std::io::Write;

#[cfg(test)]
use quickcheck::{Arbitrary, Gen};
use xml::writer::XmlEvent;

use super::{cwmp_prefix, write_simple, GenerateError, XmlSafeString};

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct DeleteObject {
    pub object_name: XmlSafeString,
    pub parameter_key: XmlSafeString,
}

impl DeleteObject {
    #[must_use]
    pub fn new(object_name: &str, parameter_key: &str) -> Self {
        Self {
            object_name: object_name.into(),
            parameter_key: parameter_key.into(),
        }
    }
    /// Generate XML for `DeleteObject`
    ///     
    /// # Errors
    ///
    /// Any errors encountered while writing to "writer" will be returned.
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        writer.write(XmlEvent::start_element(
            &cwmp_prefix(has_cwmp, "DeleteObject")[..],
        ))?;
        write_simple(writer, "ObjectName", self.object_name.0.as_ref())?;
        write_simple(writer, "ParameterKey", self.parameter_key.0.as_ref())?;
        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
    pub fn characters(&mut self, path: &[&str], characters: &str) {
        match *path {
            ["DeleteObject", "ObjectName"] => {
                self.object_name = characters.into();
            }
            ["DeleteObject", "ParameterKey"] => {
                self.parameter_key = characters.into();
            }
            _ => {}
        }
    }
}

#[cfg(test)]
impl Arbitrary for DeleteObject {
    fn arbitrary(g: &mut Gen) -> Self {
        Self {
            object_name: XmlSafeString::arbitrary(g),
            parameter_key: XmlSafeString::arbitrary(g),
        }
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (self.object_name.clone(), self.parameter_key.clone())
                .shrink()
                .map(|(o, p)| DeleteObject {
                    object_name: o,
                    parameter_key: p,
                }),
        )
    }
}
