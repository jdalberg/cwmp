use super::{cwmp_prefix, parse_to_int, write_simple, GenerateError};
use std::io::Write;

#[cfg(test)]
use quickcheck::{Arbitrary, Gen};
use xml::writer::XmlEvent;

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct GetParameterNames {
    pub parameter_path: String,
    pub next_level: u32,
}
impl GetParameterNames {
    #[must_use]
    pub fn new(parameter_path: String, next_level: u32) -> Self {
        GetParameterNames {
            parameter_path,
            next_level,
        }
    }
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        writer.write(XmlEvent::start_element(
            &cwmp_prefix(has_cwmp, "GetParameterNames")[..],
        ))?;
        write_simple(writer, "ParameterPath", &self.parameter_path)?;
        write_simple(writer, "NextLevel", &self.next_level.to_string())?;
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
    pub fn characters(&mut self, path: &[&str], characters: &String) {
        match *path {
            ["GetParameterNames", "ParameterPath"] => self.parameter_path = characters.to_string(),
            ["GetParameterNames", "NextLevel"] => self.next_level = parse_to_int(characters, 0),
            _ => {}
        }
    }
}

#[cfg(test)]
impl Arbitrary for GetParameterNames {
    fn arbitrary(g: &mut Gen) -> Self {
        GetParameterNames::new(String::arbitrary(g), u32::arbitrary(g))
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (self.parameter_path.clone(), self.next_level.clone())
                .shrink()
                .map(|(pp, nl)| GetParameterNames {
                    parameter_path: pp,
                    next_level: nl,
                }),
        )
    }
}
