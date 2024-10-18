use std::io::Write;

#[cfg(test)]
use quickcheck::{Arbitrary, Gen};
use xml::writer::XmlEvent;

use super::{cwmp_prefix, write_simple, GenerateError};

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct DeleteObjectResponse {
    status: String,
}

impl DeleteObjectResponse {
    pub fn new(status: String) -> Self {
        DeleteObjectResponse { status: status }
    }
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        writer.write(XmlEvent::start_element(
            &cwmp_prefix(has_cwmp, "DeleteObjectResponse")[..],
        ))?;
        write_simple(writer, "Status", &self.status)?;
        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
    pub fn characters(&mut self, path: &[&str], characters: &String) {
        match *path {
            ["DeleteObjectResponse", "Status"] => self.status = characters.to_string(),
            _ => {}
        }
    }
}

#[cfg(test)]
impl Arbitrary for DeleteObjectResponse {
    fn arbitrary(g: &mut Gen) -> Self {
        DeleteObjectResponse::new(String::arbitrary(g))
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
