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
    #[must_use]
    pub fn new(transfer_list: Vec<AllQueuedTransfers>) -> Self {
        GetAllQueuedTransfersResponse { transfer_list }
    }
    /// Generate XML for `GetAllQueuedTransfersResponse`
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
            &cwmp_prefix(has_cwmp, "GetAllQueuedTransfersResponse")[..],
        ))?;

        let ss = format!(
            "cwmp::AllQueuedTransferStruct[{}]",
            self.transfer_list.len()
        );

        writer
            .write(XmlEvent::start_element("TransferList").attr("SOAP-ENC:arrayType", &ss[..]))?;

        for t in &self.transfer_list {
            writer.write(XmlEvent::start_element("AllQueuedTransferStruct"))?;
            write_simple(writer, "CommandKey", t.command_key.0.as_ref())?;
            write_simple(writer, "State", t.state.0.as_ref())?;
            write_simple(writer, "IsDownload", &t.is_download.to_string())?;
            write_simple(writer, "FileType", t.file_type.0.as_ref())?;
            write_simple(writer, "FileSize", &t.file_size.to_string())?;
            write_simple(writer, "TargetFileName", t.target_filename.0.as_ref())?;
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
        _attributes: &[xml::attribute::OwnedAttribute],
    ) {
        let path_pattern: Vec<&str> = path.iter().map(AsRef::as_ref).collect();
        if let ["GetAllQueuedTransfersResponse", "TransferList", "AllQueuedTransferStruct"] =
            &path_pattern[..]
        {
            self.transfer_list
                .push(AllQueuedTransfers::new("", "", 0, "", 0, ""));
        }
    }
    pub fn characters(&mut self, path: &[&str], characters: &str) {
        if let ["GetAllQueuedTransfersResponse", "TransferList", "AllQueuedTransferStruct", key] =
            *path
        {
            if let Some(last) = self.transfer_list.last_mut() {
                match key {
                    "CommandKey" => last.command_key = characters.into(),
                    "State" => last.state = characters.into(),
                    "IsDownload" => last.is_download = parse_to_int(characters, 0),
                    "FileType" => last.file_type = characters.into(),
                    "FileSize" => last.file_size = parse_to_int(characters, 0),
                    "TargetFileName" => last.target_filename = characters.into(),
                    _ => {}
                }
            }
        }
    }
}

#[cfg(test)]
impl Arbitrary for GetAllQueuedTransfersResponse {
    fn arbitrary(g: &mut Gen) -> Self {
        Self {
            transfer_list: Vec::<AllQueuedTransfers>::arbitrary(g),
        }
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
