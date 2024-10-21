use std::io::Write;

#[cfg(test)]
use quickcheck::{Arbitrary, Gen};
use xml::writer::XmlEvent;

use super::{cwmp_prefix, write_simple, GenerateError, XmlSafeString};

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct DeleteObjectResponse {
    status: XmlSafeString,
}

impl DeleteObjectResponse {
    #[must_use]
    pub fn new(status: &str) -> Self {
        DeleteObjectResponse {
            status: status.into(),
        }
    }
    /// Generate XML for `DeleteObjectResponse`
    ///     
    /// # Errors
    ///     Any errors encountered while writing to `writer` will be returned.
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        writer.write(XmlEvent::start_element(
            &cwmp_prefix(has_cwmp, "DeleteObjectResponse")[..],
        ))?;
        write_simple(writer, "Status", self.status.0.as_ref())?;
        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
    pub fn characters(&mut self, path: &[&str], characters: &str) {
        if let ["DeleteObjectResponse", "Status"] = *path {
            self.status = characters.into();
        }
    }
}

#[cfg(test)]
impl Arbitrary for DeleteObjectResponse {
    fn arbitrary(g: &mut Gen) -> Self {
        Self {
            status: XmlSafeString::arbitrary(g).into(),
        }
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            self.status
                .clone()
                .shrink()
                .map(|s| DeleteObjectResponse { status: s }),
        )
    }
}
