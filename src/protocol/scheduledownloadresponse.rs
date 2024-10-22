use std::io::Write;

use super::{cwmp_prefix, write_empty_tag, GenerateError};

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct ScheduleDownloadResponse {}

impl ScheduleDownloadResponse {
    /// Generate XML for `ScheduleDownloadResponse`
    ///     
    /// # Errors
    ///
    /// Any errors encountered while writing to "writer" will be returned.
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        write_empty_tag(
            writer,
            &cwmp_prefix(has_cwmp, "ScheduleDownloadResponse")[..],
        )?;
        Ok(())
    }
}
