use std::io::Write;

use chrono::{DateTime, Utc};
#[cfg(test)]
use quickcheck::{Arbitrary, Gen};
use xml::writer::XmlEvent;

#[cfg(test)]
use super::gen_utc_date;
use super::{
    cwmp_prefix, parse_to_int, write_fault_struct, write_simple, FaultStruct, GenerateError,
};

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct AutonomousTransferComplete {
    announce_url: String,
    transfer_url: String,
    is_download: u8,
    file_type: String,
    file_size: u32,
    target_filename: String,
    fault: FaultStruct,
    start_time: Option<DateTime<Utc>>,
    complete_time: Option<DateTime<Utc>>,
}

impl AutonomousTransferComplete {
    pub fn new(
        announce_url: String,
        transfer_url: String,
        is_download: u8,
        file_type: String,
        file_size: u32,
        target_filename: String,
        fault: FaultStruct,
        start_time: DateTime<Utc>,
        complete_time: DateTime<Utc>,
    ) -> Self {
        AutonomousTransferComplete {
            announce_url: announce_url,
            transfer_url: transfer_url,
            is_download: is_download,
            file_type: file_type,
            file_size: file_size,
            target_filename: target_filename,
            fault: fault,
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
            &cwmp_prefix(has_cwmp, "AutonomousTransferComplete")[..],
        ))?;
        write_simple(writer, "AnnounceURL", &self.announce_url)?;
        write_simple(writer, "TransferURL", &self.transfer_url)?;
        write_simple(writer, "IsDownload", &self.is_download.to_string())?;
        write_simple(writer, "FileType", &self.file_type)?;
        write_simple(writer, "FileSize", &self.file_size.to_string())?;
        write_simple(writer, "TargetFileName", &self.target_filename)?;
        write_fault_struct(writer, &self.fault)?;
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
            ["AutonomousTransferComplete", "AnnounceURL"] => {
                self.announce_url = characters.to_string()
            }
            ["AutonomousTransferComplete", "TransferURL"] => {
                self.transfer_url = characters.to_string()
            }
            ["AutonomousTransferComplete", "IsDownload"] => {
                self.is_download = parse_to_int(characters, 0)
            }
            ["AutonomousTransferComplete", "TargetFileName"] => {
                self.target_filename = characters.to_string()
            }
            ["AutonomousTransferComplete", "FileType"] => self.file_type = characters.to_string(),
            ["AutonomousTransferComplete", "FileSize"] => {
                self.file_size = parse_to_int(characters, 0)
            }
            ["AutonomousTransferComplete", "StartTime"] => {
                match characters.parse::<DateTime<Utc>>() {
                    Ok(dt) => self.start_time = Some(dt),
                    _ => {}
                }
            }
            ["AutonomousTransferComplete", "CompleteTime"] => {
                match characters.parse::<DateTime<Utc>>() {
                    Ok(dt) => self.complete_time = Some(dt),
                    _ => {}
                }
            }
            ["AutonomousTransferComplete", "FaultStruct", "FaultCode"] => {
                self.fault.set_code(parse_to_int(characters, 0))
            }
            ["AutonomousTransferComplete", "FaultStruct", "FaultString"] => {
                self.fault.set_string(characters)
            }
            _ => {}
        }
    }
}

#[cfg(test)]
impl Arbitrary for AutonomousTransferComplete {
    fn arbitrary(g: &mut Gen) -> Self {
        // times are not arbitrary due to qc
        // tuple (used in shrink) limitations
        AutonomousTransferComplete::new(
            String::arbitrary(g),
            String::arbitrary(g),
            u8::arbitrary(g),
            String::arbitrary(g),
            u32::arbitrary(g),
            String::arbitrary(g),
            FaultStruct::arbitrary(g),
            gen_utc_date(2014, 11, 28, 12, 0, 9),
            gen_utc_date(2014, 11, 29, 12, 0, 9),
        )
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (
                self.announce_url.clone(),
                self.transfer_url.clone(),
                self.is_download.clone(),
                self.file_type.clone(),
                self.file_size.clone(),
                self.target_filename.clone(),
                self.fault.clone(),
            )
                .shrink()
                .map(|(a, t, i, ft, fs, tf, f)| AutonomousTransferComplete {
                    announce_url: a,
                    transfer_url: t,
                    is_download: i,
                    file_type: ft,
                    file_size: fs,
                    target_filename: tf,
                    fault: f,
                    start_time: Some(gen_utc_date(2014, 11, 28, 12, 0, 9)),
                    complete_time: Some(gen_utc_date(2014, 11, 29, 12, 0, 9)),
                }),
        )
    }
}
