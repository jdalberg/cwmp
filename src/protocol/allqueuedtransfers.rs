#[cfg(test)]
use quickcheck::{Arbitrary, Gen};

use super::XmlSafeString;

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct AllQueuedTransfers {
    pub command_key: XmlSafeString,
    pub state: XmlSafeString,
    pub is_download: u8,
    pub file_type: XmlSafeString,
    pub file_size: u32,
    pub target_filename: XmlSafeString,
}

impl AllQueuedTransfers {
    #[must_use]
    pub fn new(
        command_key: &str,
        state: &str,
        is_download: u8,
        file_type: &str,
        file_size: u32,
        target_filename: &str,
    ) -> Self {
        AllQueuedTransfers {
            command_key: command_key.into(),
            state: state.into(),
            is_download,
            file_type: file_type.into(),
            file_size,
            target_filename: target_filename.into(),
        }
    }
}

#[cfg(test)]
impl Arbitrary for AllQueuedTransfers {
    fn arbitrary(g: &mut Gen) -> Self {
        Self {
            command_key: XmlSafeString::arbitrary(g),
            state: XmlSafeString::arbitrary(g),
            is_download: u8::arbitrary(g),
            file_type: XmlSafeString::arbitrary(g),
            file_size: u32::arbitrary(g),
            target_filename: XmlSafeString::arbitrary(g),
        }
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (
                self.command_key.clone(),
                self.state.clone(),
                self.is_download,
                self.file_type.clone(),
                self.file_size,
                self.target_filename.clone(),
            )
                .shrink()
                .map(|(c, s, id, ft, fs, tf)| AllQueuedTransfers {
                    command_key: c,
                    state: s,
                    is_download: id,
                    file_type: ft,
                    file_size: fs,
                    target_filename: tf,
                }),
        )
    }
}
