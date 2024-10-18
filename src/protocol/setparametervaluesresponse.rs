use std::io::Write;

use xml::writer::XmlEvent;

use super::{cwmp_prefix, parse_to_int, write_simple, GenerateError};
#[cfg(test)]
use quickcheck::{Arbitrary, Gen};

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct SetParameterValuesResponse {
    pub status: u32,
}

impl SetParameterValuesResponse {
    #[must_use]
    pub fn new(status: u32) -> Self {
        SetParameterValuesResponse { status }
    }

    /// Generate XML for `SetParameterValuesResponse`
    ///     
    /// # Errors
    ///     Any errors encountered while writing to `writer` will be returned.
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        writer.write(XmlEvent::start_element(
            &cwmp_prefix(has_cwmp, "SetParameterValuesResponse")[..],
        ))?;
        write_simple(writer, "Status", &self.status.to_string())?;
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
    pub fn characters(&mut self, path: &[&str], characters: &str) {
        if let ["SetParameterValuesResponse", "Status"] = *path {
            self.status = parse_to_int(characters, 0);
        }
    }
}

#[cfg(test)]
impl Arbitrary for SetParameterValuesResponse {
    fn arbitrary(g: &mut Gen) -> Self {
        SetParameterValuesResponse::new(u32::arbitrary(g))
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            self.status
                .clone()
                .shrink()
                .map(|s| SetParameterValuesResponse { status: s }),
        )
    }
}
