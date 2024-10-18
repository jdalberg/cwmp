use std::io::Write;

#[cfg(test)]
use quickcheck::{Arbitrary, Gen};
use xml::writer::XmlEvent;

use super::{cwmp_prefix, write_simple, GenerateError};

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct KickedResponse {
    pub next_url: String,
}

impl KickedResponse {
    #[must_use]
    pub fn new(next_url: String) -> Self {
        KickedResponse { next_url }
    }

    /// Generate XML for `NoMoreRequests`
    ///     
    /// # Errors
    ///     Any errors encountered while writing to `writer` will be returned.
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        writer.write(XmlEvent::start_element(
            &cwmp_prefix(has_cwmp, "KickedResponse")[..],
        ))?;
        write_simple(writer, "NextURL", &self.next_url)?;
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
    pub fn characters(&mut self, path: &[&str], characters: &String) {
        if let ["KickedResponse", "NextURL"] = *path {
            self.next_url = characters.to_string();
        }
    }
}

#[cfg(test)]
impl Arbitrary for KickedResponse {
    fn arbitrary(g: &mut Gen) -> Self {
        KickedResponse::new(String::arbitrary(g))
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            self.next_url
                .clone()
                .shrink()
                .map(|n| KickedResponse { next_url: n }),
        )
    }
}
