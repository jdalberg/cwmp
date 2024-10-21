use super::{write_simple, GenerateError};
use super::{QueuedTransferStruct, XmlSafeString};
use std::io::Write;

#[cfg(test)]
use quickcheck::{Arbitrary, Gen};
use xml::writer::XmlEvent;

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct GetQueuedTransfersResponse {
    pub transfer_list: Vec<QueuedTransferStruct>,
}

impl GetQueuedTransfersResponse {
    #[must_use]
    pub fn new(transfer_list: Vec<QueuedTransferStruct>) -> Self {
        GetQueuedTransfersResponse { transfer_list }
    }

    /// Generate XML for `GetQueuedTransfersResponse`
    ///     
    /// # Errors
    ///     Any errors encountered while writing to `writer` will be returned.
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        let ss = if has_cwmp {
            writer.write(XmlEvent::start_element("cwmp:GetQueuedTransfersResponse"))?;
            format!("cwmp:QueuedTransferStruct[{}]", self.transfer_list.len())
        } else {
            writer.write(XmlEvent::start_element("GetQueuedTransfersResponse"))?;
            format!("QueuedTransferStruct[{}]", self.transfer_list.len())
        };

        writer
            .write(XmlEvent::start_element("TransferList").attr("SOAP-ENC:arrayType", &ss[..]))?;

        for p in &self.transfer_list {
            writer.write(XmlEvent::start_element("QueuedTransferStruct"))?;
            if let Some(ck) = &p.command_key {
                write_simple(writer, "CommandKey", ck.0.as_ref())?;
            }
            if let Some(s) = &p.state {
                write_simple(writer, "State", s.0.as_ref())?;
            }
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
        match &path_pattern[..] {
            ["GetQueuedTransfersResponse", "TransferList", "QueuedTransferStruct"] => {
                self.transfer_list.push(QueuedTransferStruct::default());
            }
            ["GetQueuedTransfersResponse", "TransferList", "QueuedTransferStruct", "CommandKey"] => {
                if let Some(l) = self.transfer_list.last_mut() {
                    l.command_key = Some(XmlSafeString::new());
                }
            }
            ["GetQueuedTransfersResponse", "TransferList", "QueuedTransferStruct", "State"] => {
                if let Some(l) = self.transfer_list.last_mut() {
                    l.state = Some(XmlSafeString::new());
                }
            }
            _ => {}
        }
    }
    pub fn characters(&mut self, path: &[&str], characters: &str) {
        if let ["GetQueuedTransfersResponse", "TransferList", "QueuedTransferStruct", key] = *path {
            if let Some(e) = self.transfer_list.last_mut() {
                match key {
                    "CommandKey" => e.command_key = Some(characters.into()),
                    "State" => e.state = Some(characters.into()),
                    _ => {}
                }
            }
        }
    }
}

#[cfg(test)]
impl Arbitrary for GetQueuedTransfersResponse {
    fn arbitrary(g: &mut Gen) -> Self {
        GetQueuedTransfersResponse::new(Vec::<QueuedTransferStruct>::arbitrary(g))
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            self.transfer_list
                .clone()
                .shrink()
                .map(|t| GetQueuedTransfersResponse { transfer_list: t }),
        )
    }
}
