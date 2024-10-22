use std::io::Write;

#[cfg(test)]
use quickcheck::{Arbitrary, Gen};
use xml::writer::XmlEvent;

use super::{cwmp_prefix, parse_to_int, write_simple, GenerateError};

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct InformResponse {
    pub max_envelopes: u16,
}

impl InformResponse {
    #[must_use]
    pub fn new(max_envelopes: u16) -> Self {
        InformResponse { max_envelopes }
    }

    /// Generate XML for `InformResponse`
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
            &cwmp_prefix(has_cwmp, "InformResponse")[..],
        ))?;
        write_simple(writer, "MaxEnvelopes", &self.max_envelopes.to_string())?;
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
    pub fn characters(&mut self, path: &[&str], characters: &str) {
        if let ["InformResponse", "MaxEnvelopes"] = *path {
            self.max_envelopes = parse_to_int(characters, 1);
        }
    }
}

#[cfg(test)]
impl Arbitrary for InformResponse {
    fn arbitrary(g: &mut Gen) -> Self {
        InformResponse::new(u16::arbitrary(g))
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            self.max_envelopes
                .clone()
                .shrink()
                .map(|me| InformResponse { max_envelopes: me }),
        )
    }
}
