use std::io::Write;

use chrono::{DateTime, Utc};
use xml::writer::XmlEvent;

#[cfg(test)]
use super::gen_utc_date;
use super::{
    cwmp_prefix, parse_to_int, write_fault_struct, write_simple, FaultStruct, GenerateError,
    XmlSafeString,
};
#[cfg(test)]
use quickcheck::{Arbitrary, Gen};

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct TransferComplete {
    pub command_key: XmlSafeString,
    pub fault: FaultStruct,
    pub start_time: Option<DateTime<Utc>>,
    pub complete_time: Option<DateTime<Utc>>,
}

impl TransferComplete {
    #[must_use]
    pub fn new(
        command_key: &str,
        fault: FaultStruct,
        start_time: Option<DateTime<Utc>>,
        complete_time: Option<DateTime<Utc>>,
    ) -> Self {
        TransferComplete {
            command_key: command_key.into(),
            fault,
            start_time,
            complete_time,
        }
    }

    /// Generate XML for `TransferComplete`
    ///     
    /// # Errors
    ///     Any errors encountered while writing to `writer` will be returned.
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        writer.write(XmlEvent::start_element(
            &cwmp_prefix(has_cwmp, "TransferComplete")[..],
        ))?;
        write_simple(writer, "CommandKey", self.command_key.0.as_ref())?;
        write_fault_struct(writer, &self.fault)?;
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
            ["TransferComplete", "CommandKey"] => self.command_key = characters.into(),
            ["TransferComplete", "StartTime"] => {
                if let Ok(dt) = characters.parse::<DateTime<Utc>>() {
                    self.start_time = Some(dt);
                }
            }
            ["TransferComplete", "CompleteTime"] => {
                if let Ok(dt) = characters.parse::<DateTime<Utc>>() {
                    self.complete_time = Some(dt);
                }
            }
            ["TransferComplete", "FaultStruct", "FaultCode"] => {
                self.fault.set_code(parse_to_int(characters, 0));
            }
            ["TransferComplete", "FaultStruct", "FaultString"] => self.fault.set_string(characters),

            _ => {}
        }
    }
}

#[cfg(test)]
impl Arbitrary for TransferComplete {
    fn arbitrary(g: &mut Gen) -> Self {
        TransferComplete::new(
            &String::arbitrary(g),
            FaultStruct::arbitrary(g),
            Some(gen_utc_date(2014, 11, 28, 12, 0, 9)),
            Some(gen_utc_date(2014, 11, 29, 12, 0, 9)),
        )
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (self.command_key.clone(), self.fault.clone())
                .shrink()
                .map(|(c, f)| TransferComplete {
                    command_key: c,
                    fault: f,
                    start_time: Some(gen_utc_date(2014, 11, 28, 12, 0, 9)),
                    complete_time: Some(gen_utc_date(2014, 11, 29, 12, 0, 9)),
                }),
        )
    }
}
