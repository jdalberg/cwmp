use std::io::Write;

#[cfg(test)]
use quickcheck::{Arbitrary, Gen};
use xml::writer::XmlEvent;

use super::{cwmp_prefix, parse_to_int, write_simple, AllQueuedTransfers, GenerateError};

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct GetAllQueuedTransfersResponse {
    pub transfer_list: Vec<AllQueuedTransfers>,
}

impl GetAllQueuedTransfersResponse {
    pub fn new(transfer_list: Vec<AllQueuedTransfers>) -> Self {
        GetAllQueuedTransfersResponse {
            transfer_list,
        }
    }
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        writer.write(XmlEvent::start_element(
            &cwmp_prefix(has_cwmp, "GetAllQueuedTransfersResponse")[..],
        ))?;

        let ss = format!(
            "cwmp::AllQueuedTransferStruct[{}]",
            self.transfer_list.len()
        );

        writer
            .write(XmlEvent::start_element("TransferList").attr("SOAP-ENC:arrayType", &ss[..]))?;

        for t in self.transfer_list.iter() {
            writer.write(XmlEvent::start_element("AllQueuedTransferStruct"))?;
            write_simple(writer, "CommandKey", &t.command_key)?;
            write_simple(writer, "State", &t.state)?;
            write_simple(writer, "IsDownload", &t.is_download.to_string())?;
            write_simple(writer, "FileType", &t.file_type)?;
            write_simple(writer, "FileSize", &t.file_size.to_string())?;
            write_simple(writer, "TargetFileName", &t.target_filename)?;
            writer.write(XmlEvent::end_element())?;
        }

        writer.write(XmlEvent::end_element())?;
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
    pub fn start_handler(
        &mut self,
        path: &[&str],
        _name: &xml::name::OwnedName,
        _attributes: &Vec<xml::attribute::OwnedAttribute>,
    ) {
        let path_pattern: Vec<&str> = path.iter().map(AsRef::as_ref).collect();
        if let ["GetAllQueuedTransfersResponse", "TransferList", "AllQueuedTransferStruct"] = &path_pattern[..] {
            self.transfer_list.push(AllQueuedTransfers::new(
                String::from(""),
                String::from(""),
                0,
                String::from(""),
                0,
                String::from(""),
            ))
        }
    }
    pub fn characters(&mut self, path: &[&str], characters: &String) {
        if let ["GetAllQueuedTransfersResponse", "TransferList", "AllQueuedTransferStruct", key] = *path {
            if let Some(last) = self.transfer_list.last_mut() {
                match key {
                    "CommandKey" => last.command_key = characters.to_string(),
                    "State" => last.state = characters.to_string(),
                    "IsDownload" => last.is_download = parse_to_int(characters, 0),
                    "FileType" => last.file_type = characters.to_string(),
                    "FileSize" => last.file_size = parse_to_int(characters, 0),
                    "TargetFileName" => last.target_filename = characters.to_string(),
                    _ => {}
                }
            }
        }
    }
}

#[cfg(test)]
impl Arbitrary for GetAllQueuedTransfersResponse {
    fn arbitrary(g: &mut Gen) -> Self {
        GetAllQueuedTransfersResponse::new(Vec::<AllQueuedTransfers>::arbitrary(g))
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            self.transfer_list
                .clone()
                .shrink()
                .map(|tl| GetAllQueuedTransfersResponse { transfer_list: tl }),
        )
    }
}
