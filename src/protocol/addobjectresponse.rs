use std::io::Write;

#[cfg(test)]
use quickcheck::{Arbitrary, Gen};
use xml::writer::XmlEvent;

use super::{cwmp_prefix, GenerateError, XmlSafeString};

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct AddObjectResponse {
    pub instance_number: u32,
    pub status: XmlSafeString,
}

impl AddObjectResponse {
    #[must_use]
    pub fn new(instance_number: u32, status: &str) -> Self {
        AddObjectResponse {
            instance_number,
            status: status.into(),
        }
    }

    /// Generate XML for `AddObjectResponse`
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
            &cwmp_prefix(has_cwmp, "AddObjectResponse")[..],
        ))?;

        writer.write(XmlEvent::start_element("InstanceNumber"))?;
        writer.write(&self.instance_number.to_string()[..])?;
        writer.write(XmlEvent::end_element())?;
        writer.write(XmlEvent::start_element("Status"))?;
        writer.write(self.status.0.as_ref())?;
        writer.write(XmlEvent::end_element())?;

        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
    pub fn characters(&mut self, path: &[&str], characters: &str) {
        match *path {
            ["AddObjectResponse", "InstanceNumber"] => {
                if let Ok(instance) = characters.parse() {
                    self.instance_number = instance;
                }
            }
            ["AddObjectResponse", "Status"] => {
                self.status = characters.into();
            }
            _ => {}
        }
    }
}

#[cfg(test)]
impl Arbitrary for AddObjectResponse {
    fn arbitrary(g: &mut Gen) -> Self {
        Self {
            instance_number: u32::arbitrary(g),
            status: XmlSafeString::arbitrary(g),
        }
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (self.instance_number, self.status.clone())
                .shrink()
                .map(|(i, s)| AddObjectResponse {
                    instance_number: i,
                    status: s,
                }),
        )
    }
}
