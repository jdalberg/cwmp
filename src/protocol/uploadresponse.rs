use std::io::Write;

use chrono::{DateTime, Utc};
use xml::writer::XmlEvent;

#[cfg(test)]
use super::gen_utc_date;
use super::{cwmp_prefix, parse_to_int, write_simple, GenerateError};
#[cfg(test)]
use quickcheck::{Arbitrary, Gen};

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct UploadResponse {
    pub status: u8,
    pub start_time: Option<DateTime<Utc>>,
    pub complete_time: Option<DateTime<Utc>>,
}

impl UploadResponse {
    #[must_use]
    pub fn new(
        status: u8,
        start_time: Option<DateTime<Utc>>,
        complete_time: Option<DateTime<Utc>>,
    ) -> Self {
        UploadResponse {
            status,
            start_time,
            complete_time,
        }
    }

    /// Generate XML for `UploadResponse`
    ///     
    /// # Errors
    ///
    /// Any "errors" encountered while writing to "writer" will be returned.
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        writer.write(XmlEvent::start_element(
            &cwmp_prefix(has_cwmp, "UploadResponse")[..],
        ))?;
        write_simple(writer, "Status", &self.status.to_string())?;
        if let Some(dt) = self.start_time {
            write_simple(writer, "StartTime", &dt.to_rfc3339())?;
        }
        if let Some(dt) = self.complete_time {
            write_simple(writer, "CompleteTime", &dt.to_rfc3339())?;
        }
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
    pub fn characters(&mut self, path: &[&str], characters: &str) {
        match *path {
            ["UploadResponse", "Status"] => self.status = parse_to_int(characters, 0),
            ["UploadResponse", "StartTime"] => {
                if let Ok(dt) = characters.parse::<DateTime<Utc>>() {
                    self.start_time = Some(dt);
                }
            }
            ["UploadResponse", "CompleteTime"] => {
                if let Ok(dt) = characters.parse::<DateTime<Utc>>() {
                    self.complete_time = Some(dt);
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
impl Arbitrary for UploadResponse {
    fn arbitrary(g: &mut Gen) -> Self {
        UploadResponse::new(
            u8::arbitrary(g),
            Some(gen_utc_date(2014, 11, 28, 12, 0, 9)),
            Some(gen_utc_date(2014, 11, 29, 12, 0, 9)),
        )
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(self.status.clone().shrink().map(|s| UploadResponse {
            status: s,
            start_time: Some(gen_utc_date(2014, 11, 28, 12, 0, 9)),
            complete_time: Some(gen_utc_date(2014, 11, 29, 12, 0, 9)),
        }))
    }
}
