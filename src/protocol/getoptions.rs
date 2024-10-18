use std::io::Write;

#[cfg(test)]
use quickcheck::{Arbitrary, Gen};
use xml::writer::XmlEvent;

use super::{cwmp_prefix, write_simple, GenerateError};

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct GetOptions {
    pub option_name: String,
}

impl GetOptions {
    #[must_use]
    pub fn new(option_name: String) -> Self {
        GetOptions { option_name }
    }
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        writer.write(XmlEvent::start_element(
            &cwmp_prefix(has_cwmp, "GetOptions")[..],
        ))?;
        write_simple(writer, "OptionName", &self.option_name)?;
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
    pub fn characters(&mut self, path: &[&str], characters: &String) {
        if let ["GetOptions", "OptionName"] = *path {
            self.option_name = characters.to_string();
        }
    }
}

#[cfg(test)]
impl Arbitrary for GetOptions {
    fn arbitrary(g: &mut Gen) -> Self {
        GetOptions::new(String::arbitrary(g))
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            self.option_name
                .clone()
                .shrink()
                .map(|on| GetOptions { option_name: on }),
        )
    }
}
