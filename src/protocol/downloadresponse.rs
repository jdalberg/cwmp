use std::io::Write;

use chrono::{DateTime, Utc};
#[cfg(test)]
use quickcheck::{Arbitrary, Gen};
use xml::writer::XmlEvent;

#[cfg(test)]
use super::gen_utc_date;
use super::{cwmp_prefix, write_simple, GenerateError};

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct DownloadResponse {
    pub status: String,
    pub start_time: Option<DateTime<Utc>>,
    pub complete_time: Option<DateTime<Utc>>,
}

impl DownloadResponse {
    #[must_use] pub fn new(status: String, start_time: DateTime<Utc>, complete_time: DateTime<Utc>) -> Self {
        DownloadResponse {
            status,
            start_time: Some(start_time),
            complete_time: Some(complete_time),
        }
    }
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        writer.write(XmlEvent::start_element(
            &cwmp_prefix(has_cwmp, "DownloadResponse")[..],
        ))?;
        write_simple(writer, "Status", &self.status)?;
        match self.start_time {
            None => {}
            Some(dt) => write_simple(writer, "StartTime", &dt.to_rfc3339())?,
        }
        match self.complete_time {
            None => {}
            Some(dt) => write_simple(writer, "CompleteTime", &dt.to_rfc3339())?,
        }
        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
    pub fn characters(&mut self, path: &[&str], characters: &String) {
        match *path {
            ["DownloadResponse", "Status"] => {
                self.status = characters.to_string();
            }
            ["DownloadResponse", "StartTime"] => if let Ok(dt) = characters.parse::<DateTime<Utc>>() { self.start_time = Some(dt) },
            ["DownloadResponse", "CompleteTime"] => if let Ok(dt) = characters.parse::<DateTime<Utc>>() { self.complete_time = Some(dt) },
            _ => {}
        }
    }
}

#[cfg(test)]
impl Arbitrary for DownloadResponse {
    fn arbitrary(g: &mut Gen) -> Self {
        let bogus_st = gen_utc_date(2014, 11, 28, 12, 0, 9);
        let bogus_ct = gen_utc_date(2014, 11, 28, 12, 0, 9);

        DownloadResponse::new(String::arbitrary(g), bogus_st, bogus_ct)
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(self.status.clone().shrink().map(|s| DownloadResponse {
            status: s,
            start_time: Some(gen_utc_date(2014, 11, 28, 12, 0, 9)),
            complete_time: Some(gen_utc_date(2014, 11, 29, 12, 0, 9)),
        }))
    }
}
