use std::io::Write;

#[cfg(test)]
use quickcheck::{Arbitrary, Gen};
use xml::writer::XmlEvent;

use super::{cwmp_prefix, GenerateError};

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct AddObjectResponse {
    pub instance_number: u32,
    pub status: String,
}

impl AddObjectResponse {
    #[must_use]
    pub fn new(instance_number: u32, status: String) -> Self {
        AddObjectResponse {
            instance_number,
            status,
        }
    }

    /// Generate XML for `AddObjectResponse`
    ///     
    /// # Errors
    ///     Any errors encountered while writing to `writer` will be returned.
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
        writer.write(&self.status[..])?;
        writer.write(XmlEvent::end_element())?;

        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
    pub fn characters(&mut self, path: &[&str], characters: &String) {
        match *path {
            ["AddObjectResponse", "InstanceNumber"] => {
                if let Ok(instance) = characters.parse() {
                    self.instance_number = instance;
                }
            }
            ["AddObjectResponse", "Status"] => {
                self.status = characters.to_string();
            }
            _ => {}
        }
    }
}

#[cfg(test)]
impl Arbitrary for AddObjectResponse {
    fn arbitrary(g: &mut Gen) -> Self {
        AddObjectResponse::new(u32::arbitrary(g), String::arbitrary(g))
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (self.instance_number.clone(), self.status.clone())
                .shrink()
                .map(|(i, s)| AddObjectResponse {
                    instance_number: i,
                    status: s,
                }),
        )
    }
}
