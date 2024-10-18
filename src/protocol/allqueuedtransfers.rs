#[cfg(test)]
use quickcheck::{Arbitrary, Gen};

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct AllQueuedTransfers {
    pub command_key: String,
    pub state: String,
    pub is_download: u8,
    pub file_type: String,
    pub file_size: u32,
    pub target_filename: String,
}

impl AllQueuedTransfers {
    #[must_use] pub fn new(
        command_key: String,
        state: String,
        is_download: u8,
        file_type: String,
        file_size: u32,
        target_filename: String,
    ) -> Self {
        AllQueuedTransfers {
            command_key,
            state,
            is_download,
            file_type,
            file_size,
            target_filename,
        }
    }
}

#[cfg(test)]
impl Arbitrary for AllQueuedTransfers {
    fn arbitrary(g: &mut Gen) -> Self {
        AllQueuedTransfers::new(
            String::arbitrary(g),
            String::arbitrary(g),
            u8::arbitrary(g),
            String::arbitrary(g),
            u32::arbitrary(g),
            String::arbitrary(g),
        )
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (
                self.command_key.clone(),
                self.state.clone(),
                self.is_download.clone(),
                self.file_type.clone(),
                self.file_size.clone(),
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
