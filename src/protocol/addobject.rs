use std::io::Write;

#[cfg(test)]
use quickcheck::{Arbitrary, Gen};
use xml::writer::XmlEvent;

use super::{cwmp_prefix, write_simple, GenerateError};

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct AddObject {
    pub object_name: String,
    pub parameter_key: String,
}

impl AddObject {
    #[must_use] pub fn new(object_name: String, parameter_key: String) -> Self {
        AddObject {
            object_name,
            parameter_key,
        }
    }
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        writer.write(XmlEvent::start_element(
            &cwmp_prefix(has_cwmp, "AddObject")[..],
        ))?;
        write_simple(writer, "ObjectName", &self.object_name)?;
        write_simple(writer, "ParameterKey", &self.parameter_key)?;
        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
    pub fn characters(&mut self, path: &[&str], characters: &String) {
        match *path {
            ["AddObject", "ObjectName"] => {
                self.object_name = characters.to_string();
            }
            ["AddObject", "ParameterKey"] => {
                self.parameter_key = characters.to_string();
            }
            _ => {}
        }
    }
}

#[cfg(test)]
impl Arbitrary for AddObject {
    fn arbitrary(g: &mut Gen) -> Self {
        AddObject::new(String::arbitrary(g), String::arbitrary(g))
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
